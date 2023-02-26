use crate::{
    check_signals,
    converter::convertertools::{make_sortlist, sort_by_array, sort_by_column},
    print_debug, print_warning, setup_bar,
};
use csv;
use hdf5::File;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use ndarray;
use ndarray_csv::Array2Reader;
use regex::Regex;
use std::{path::Path, process::id};

use crate::converter::convertertools;
// Maximum amount of failiures in a row available for a process
const MAX_FAILS: i64 = 500;

// 
#[allow(clippy::too_many_arguments)]
pub fn csv_multi_idline(
    filename: &str,
    outname: &str,
    columns: Vec<i64>,
    delimiter: &str,
    header: bool,
    comment: &str,
    vel: bool,
    interpolate: bool,
    radius: f64,
) {
    // check if the column stack is either 5 or 8 long
    let bar = setup_bar!("CSV converter", 100);
    if !(columns.len() == 5 || columns.len() == 8) {
        panic!("The column stack must be either 5 or 8 long, containing the columns: t, id, x, y, z,  (vx, vy, vz)");
    }
    // TODO: CHeck if we can buffer that for big datafiles!
    let hdf5file = File::create(outname).expect("Unable to create HDF5 file.");
    // TODO Read this in the readers. check weather
    // the types are correct! 0x1 --> tdata, 0x2 --> pdata
    hdf5file
        .new_attr::<u8>()
        .create("hdf5_up4_type")
        .unwrap()
        .write_scalar(&0x2_i32)
        .unwrap();

    // Read csv data
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(header)
        .delimiter(delimiter.as_bytes()[0])
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(comment.as_bytes()[0]))
        .from_path(filename)
        .expect("Unable to open CSV file.");
    print_debug!("{:?}", rdr);
    bar.inc(1);
    let particle_data: Vec<ndarray::Array2<f64>> = {
        // read in the data from the csv file
        let mut data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> = rdr
            .deserialize_array2_dynamic()
            .expect("Unable to extract CSV data to ndarray! \nYour delimiter might be wrong.\n");
        // make a temporary array so we can reorder the columns

        let mut temp_data = ndarray::Array2::<f64>::from_elem((data.shape()[0], 8), f64::NAN);
        for (i, column) in columns.iter().enumerate() {
            temp_data
                .slice_mut(ndarray::s![.., i])
                .assign(&data.slice(ndarray::s![.., *column as usize]));
        }
        data = temp_data;
        // at this point the array has the following shape:
        // [t, id, x, y, z, vx, vy, vz] where vx, vy, vz are maybe 0.0
        bar.inc(2);
        // sort the data by id and time
        let (data, max_t, max_steps) = sort_by_id(data);
        bar.inc(20);
        let mut particle_data: Vec<ndarray::Array2<f64>> = Vec::new();

        // here, data is a vec of all particles, each particle is an array of [t, id, x, y, z, vx, vy, vz]
        for arr in data {
            let particle_id = arr.slice(ndarray::s![.., 1]).to_vec();
            let mut temp_data = arr.clone();
            // sort by time
            temp_data = sort_by_column(temp_data, 0);
            // remove id column because it was not implemented
            temp_data = remove_columns(temp_data, vec![1]);
            if interpolate {
                temp_data = convertertools::interpolate(temp_data, max_t, max_steps);
            }
            println!("Data after interpolation: {:?}", temp_data);
            if vel {
                if columns.len() > 5 {
                    panic!(
                        "Your columns are specified with more then 5 values and velocity \
                        computation is activated. If you wish to ignore the velocity data \
                        in your current data, only specify 5 columns indexing \
                        time, id, x, y, z -position "
                    )
                }
                // if condition to check weather to use the parallel version of the velocity computation
                // currently turned of due to bug in parralel computation
                if true {
                    temp_data = convertertools::velocity_polynom(temp_data, 9, 2);
                } else {
                    temp_data = convertertools::velocity_paralell::velocity_polynom_parallel(
                        temp_data, 9, 2,
                    );
                }
            }
            println!("Data after velocity calc: {:?}", temp_data);
            // push the current particle data into the vector
            particle_data.push(temp_data);
        }
        // returns the vector of particle data to a variable called
        // particle_data
        particle_data
    };
    // next step is constructing the hdf5 file

    bar.inc(30);
    print_debug!("Constructing data arrays for attributes!");

    //let mut step;
    let mut mean_counter: usize = 0;
    let mut dimensions: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions
        .slice_mut(ndarray::s![0_usize, ..])
        .fill(f64::MAX);
    dimensions
        .slice_mut(ndarray::s![1_usize, ..])
        .fill(f64::MIN);
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((3, 3));
    velocity
        .slice_mut(ndarray::s![.., 0_usize])
        .fill(f64::MAX);
    velocity
        .slice_mut(ndarray::s![.., 2_usize])
        .fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;
    // ######### arrays for Attributes:
    let mut timesteps: usize = 0;
    let mut time: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(2);
    let mut sample_rate: f64 = 0.0;
    let mut time_array = ndarray::Array1::<f64>::zeros(100); // random value as it will be overwritten anyways
    bar.inc(10);
    for (p_id, data) in particle_data.iter().enumerate() {
        // progress bar
        println!("{:?}", data.shape());
        let data_length = data.column(0).len();
        if data_length > timesteps {
            timesteps = data_length;
        }
        // Attributes
        // arrays that will be saved:
        time_array = ndarray::Array1::<f64>::zeros(data_length);
        let mut particle_id_array = ndarray::Array1::<f64>::ones(data_length);
        particle_id_array.fill(p_id as f64);
        let particle_radius_array = ndarray::Array1::from_elem(data_length, radius);
        let ppclouds_array = ndarray::Array1::<f64>::ones(data_length);
        let particle_type_array = ndarray::Array1::<f64>::zeros(data_length);
        let mut vel_array = ndarray::Array2::<f64>::zeros((data_length, 3));
        let mut pos_array = ndarray::Array2::<f64>::zeros((data_length, 3));
        let mut failcount = 0;
        let mut old_time = 0.0; // TIme of the last falid timestep

        print_debug!("Creating a new group \"particle {}\"", p_id);
        let group = hdf5file
            .create_group(&format!("particle {}", p_id))
            .unwrap_or_else(|_| panic!("Can not create group particle {}", p_id));

        if data[[0, 6]].is_nan() {
            panic!("Velocity information required")
        }
        // Go through every line of the csv file
        for (line_id, line) in data.outer_iter().enumerate() {
            let current_time = line[0];
            if current_time <= old_time {
                // The particle went back in time
                // This is not possible and must be ignored.
                failcount += 1;
                if failcount > MAX_FAILS {
                    panic!(
                        "Maximum amount of points that are behind the current \
                 time reached. Please Check wether your data contains multiple\
                 trajectories that are sorted in label.
                 "
                    )
                }
                continue;
            }
            time_array[line_id] = current_time;
            // resetfailcount. we only dont allow them do be in a row!
            failcount = 0;
            old_time = current_time;
            let pos_x = line[1];
            let pos_y = line[2];
            let pos_z = line[3];
            let pos = ndarray::array![pos_x, pos_y, pos_z];
            pos_array[[line_id, 0]] = pos_x;
            pos_array[[line_id, 1]] = pos_y;
            pos_array[[line_id, 2]] = pos_z;
            println!("{:?}", line);
            let v_x = line[4];
            let v_y = line[5];
            let v_z = line[6];
            vel_array[[line_id, 0]] = v_x;
            vel_array[[line_id, 1]] = v_y;
            vel_array[[line_id, 2]] = v_z;
            let vel: Vec<f64> = vec![v_x, v_y, v_z];
            print_debug!("Extracting statistical velocity information");

            for i in 0..3 {
                print_debug!("  i: {}", i);
                if vel[i] < velocity[[i, 0]] {
                    velocity[[i, 0]] = vel[i];
                } else if vel[i] > velocity[[i, 2]] {
                    velocity[[i, 2]] = vel[i];
                }
                velocity[[i, 1]] += vel[i];
            }

            let vel_mag: f64 = vel.iter().map(|v| v * v).sum::<f64>().sqrt();
            // check if vel magnitude is bigger or smaller then the current biggest or smallest
            if vel_mag < velocity_mag[0] {
                velocity_mag[0] = vel_mag;
            } else if vel_mag > velocity_mag[2] {
                velocity_mag[2] = vel_mag;
            }
            velocity_mag[1] += vel_mag;

            if pos[0] < dimensions[[0, 0]] {
                dimensions[[0, 0]] = pos[0];
            } else if pos[0] > dimensions[[1, 0]] {
                dimensions[[1, 0]] = pos[0];
            }
            if pos[1] < dimensions[[0, 1]] {
                dimensions[[0, 1]] = pos[1];
            } else if pos[1] > dimensions[[1, 1]] {
                dimensions[[1, 1]] = pos[1];
            }
            if pos[2] < dimensions[[0, 2]] {
                dimensions[[0, 2]] = pos[2];
            } else if pos[2] > dimensions[[1, 2]] {
                dimensions[[1, 2]] = pos[2];
            }
            //step += 1;
            mean_counter += 1;
            sample_rate = current_time - old_time;
            if current_time > time[1] {
                time[1] = current_time;
            }
            old_time = current_time;
            if line_id % 1000 == 0 {
                check_signals!();
            }
        } // end filename forloop
          // write data into HDF5 file
        let builder = group.new_dataset_builder();
        builder
            .with_data(&time_array)
            .create("time")
            .expect("Unable to create dataset \"time\"");
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_id_array)
            .create("id")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"id\" in file {}",
                filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_radius_array)
            .create("radius")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}",
                filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&ppclouds_array)
            .create("ppcloud")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}",
                filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_type_array)
            .create("particletype")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"particletype\" in file {}",
                filename));

        let builder = group.new_dataset_builder();
        builder
            .with_data(&vel_array)
            .create("velocity")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"velocity\" in file {}",
                filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&pos_array)
            .create("position")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"position\" in file {}",
                filename));
    }
    bar.inc(20);
    velocity_mag[1] /= mean_counter as f64;
    velocity[[0, 1]] /= mean_counter as f64;
    velocity[[1, 1]] /= mean_counter as f64;
    velocity[[2, 1]] /= mean_counter as f64;
    print_debug!(
        "Mean Velocity: \nmagnitude:  {}\nx:  {}\ny:  {}\nz:  {}\n",
        velocity_mag[1],
        velocity[[0, 1]],
        velocity[[1, 1]],
        velocity[[2, 1]]
    );
    print_debug!("Dimensions: {:?}", dimensions);
    hdf5file
        .new_attr_builder()
        .with_data(&dimensions)
        .create("dimensions")
        .unwrap();
    hdf5file
        .new_attr::<u64>()
        .create("particle number")
        .unwrap()
        .write_scalar(&1_usize)
        .unwrap();
    hdf5file
        .new_attr::<u64>()
        .create("timesteps")
        .unwrap()
        .write_scalar(&timesteps)
        .unwrap();
    hdf5file
        .new_attr::<u64>()
        .create("sample rate")
        .unwrap()
        .write_scalar(&sample_rate)
        .unwrap();
    hdf5file
        .new_attr_builder()
        .with_data(&time)
        .create("time")
        .unwrap();
    hdf5file
        .new_attr_builder()
        .with_data(&velocity)
        .create("velocity")
        .unwrap();
    hdf5file
        .new_attr_builder()
        .with_data(&velocity_mag)
        .create("velocity magnitude")
        .unwrap();
    hdf5file
        .new_dataset_builder()
        .with_data(&time_array)
        .create("time array")
        .unwrap();
    bar.finish()
}

fn sort_by_id(data: ndarray::Array2<f64>) -> (Vec<ndarray::Array2<f64>>, f64, usize) {
    // data has shape (n, 8)
    // [t, id, x, y, z, vx, vy, vz]
    let mut sorted_data = Vec::new();
    // go through data, find nr of particles and the number of timesteps they have.
    let (ids, min_id, max_id, max_steps, max_t) = {
        let n_ids = 0;
        let n_timesteps = 0;
        let mut min_id = data[[0, 1]] as usize;
        let mut max_id = data[[0, 1]] as usize;
        let mut min_t = data[[0, 0]];
        let mut max_t = data[[0, 0]];
        let mut ids = Vec::new();
        let mut ids_timesteps = Vec::new();
        let mut max_steps = 0;
        for i in 0..data.shape()[0] {
            let id = data[[i, 1]] as usize;
            let t = data[[i, 0]];

            if id < min_id {
                min_id = id;
            } else if id > max_id {
                max_id = id;
            }
            if t < min_t {
                min_t = t;
            } else if t > max_t {
                max_t = t;
            }
            if !ids.contains(&id) {
                ids.push(id);
                ids_timesteps.push(1);
            } else {
                let index = ids.iter().position(|&x| x == id).unwrap();
                ids_timesteps[index] += 1;
                if ids_timesteps[index] > max_steps {
                    max_steps = ids_timesteps[index];
                }
            }
        }
        // sort ids and ids_timesteps by ids
        let mut ids_and_steps = ids
            .iter()
            .zip(ids_timesteps.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(usize, usize)>>();
        ids_and_steps.sort_by(|a, b| a.0.cmp(&b.0));

        (ids_and_steps, min_id, max_id, max_steps, max_t)
    };
    // make array
    for (id, steps) in ids.iter() {
        sorted_data.push(ndarray::Array2::zeros((*steps, 8)));
        let len = sorted_data.len();
        let mut id_line = sorted_data[len - 1].slice_mut(ndarray::s![.., 1]);
        id_line.fill(*id as f64);
    }
    // fill array
    let mut last_line_pushed = vec![0; ids.len()];
    for i in 0..data.shape()[0] {
        let id = data[[i, 1]] as usize;
        let t = data[[i, 0]];
        let index = ids.iter().position(|&x| x.0 == id).unwrap();
        let line = last_line_pushed[index];
        let mut line = sorted_data[index].slice_mut(ndarray::s![line, ..]);
        last_line_pushed[index] += 1;
        line.assign(&data.slice(ndarray::s![i, ..]));
    }
    (sorted_data, max_t, max_steps)
}

fn remove_columns(data: ndarray::Array2<f64>, columns: Vec<usize>) -> ndarray::Array2<f64> {
    let mut new_data = ndarray::Array2::zeros((data.shape()[0], data.shape()[1] - columns.len()));
    let mut new_data_index = 0;
    for i in 0..data.shape()[1] {
        if !columns.contains(&i) {
            let mut new_data_line = new_data.slice_mut(ndarray::s![.., new_data_index]);
            new_data_index += 1;
            let data_line = data.slice(ndarray::s![.., i]);
            new_data_line.assign(&data_line);
        }
    }
    new_data
}
