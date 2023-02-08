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

pub fn csv_multi_file_time_sep(
    filenames: Vec<&str>,
    outname: &str,
    columns: Vec<i64>,
    time_: Vec<f64>,
    delimiter: &str,
    header: bool,
    comment: &str,
    vel: bool,
    interpolate: bool,
    radius: f64,
) {
    println!("WARNING Currently only works for barracuda raw data!");
    // check if the column stack is either 5 or 8 long
    let bar = setup_bar!("CSV converter", 100);
    if !(columns.len() == 4 || columns.len() == 7) {
        panic!("The column stack must be either 4 or 7 long, containing the columns: pid, x, y, z,  (vx, vy, vz)");
    }
    // TODO: CHeck if we can buffer that for big datafiles!
    let hdf5file = File::create(outname).expect("Unable to create HDF5 file.");
    // TODO Read this in the readers. check weather
    // the types are correct! 0x1 --> tdata, 0x2 --> pdata
    hdf5file
        .new_attr::<u8>()
        .create("hdf5_up4_type")
        .unwrap()
        .write_scalar(&0x1_i32)
        .unwrap();

    let mut step = 0;
    let bar = setup_bar!("Vtk Read Data", filenames.len() as u64);
    // Attributes
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions.slice_mut(ndarray::s![0usize, ..]).fill(f64::MAX);
    dimensions.slice_mut(ndarray::s![1usize, ..]).fill(f64::MIN);
    let mut nparticles: u64 = 0;
    let timesteps: usize = filenames.len();
    let mut time: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(2);
    let mut time_array = Vec::<f64>::new();
    let mut sample_rate: f64 = 0.0;
    let mut old_time = 0.0;
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((3, 3));
    velocity.slice_mut(ndarray::s![.., 0usize]).fill(f64::MAX);
    velocity.slice_mut(ndarray::s![.., 2usize]).fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;
    let mut mean_counter: usize = 0;
    for (id, filename) in filenames.iter().enumerate() {
        println!("Reading file: {}", filename);
        let group = hdf5file
            .create_group(&format!("timestep {}", id))
            .expect(&format!("Can not create group timestep {}", id));
        let current_time = time_[id];
        if current_time > time[1] {
            time[1] = current_time;
        }
        time_array.push(current_time);
        group
            .new_dataset::<f64>()
            .create("time")
            .unwrap()
            .write_scalar(&current_time)
            .unwrap();
        // Read csv data
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(header)
            .delimiter(delimiter.as_bytes()[0])
            .double_quote(false)
            //.escape(Some(b'\\'))
            .flexible(true)
            .comment(Some(comment.as_bytes()[0]))
            .from_path(filename)
            .expect("Unable to open CSV file.");
        // read in the data from the csv file
        let data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> = rdr
            .deserialize_array2_dynamic()
            .expect("Unable to extract CSV data to ndarray! \nYour delimiter might be wrong.\n");
        // make a temporary array so we can reorder the columns

        let mut temp_data = ndarray::Array2::<f64>::from_elem((data.shape()[0], 8), f64::NAN);
        for (i, column) in columns.iter().enumerate() {
            temp_data
                .slice_mut(ndarray::s![.., i + 1]) // +1 because the time array is added extra
                .assign(&data.slice(ndarray::s![.., *column as usize]));
        }
        // add time in first column
        println!("Time: {}", time);
        println!("ID: {}", id);
        let time = time_[id];
        temp_data
            .slice_mut(ndarray::s![.., 0])
            .assign(&ndarray::Array1::<f64>::from_elem(data.shape()[0], time));

        let timestep_data = temp_data;
        let particle_id = timestep_data.slice(ndarray::s![.., 1]).to_vec();
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_id)
            .create("id")
            .expect(&format!(
                "Unable to create dataset \"id\" in file {}",
                filename
            ));
        //TODO: add a dict to that to read in other columns!
        let builder = group.new_dataset_builder();
        let zero_array = vec![0.0; particle_id.len()];
        builder
            .with_data(&zero_array)
            .create("radius")
            .expect(&format!(
                "Unable to create dataset \"radius\" in file {}",
                filename
            ));

        let builder = group.new_dataset_builder();
        builder
            .with_data(&zero_array)
            .create("ppcloud")
            .expect(&format!(
                "Unable to create dataset \"radius\" in file {}",
                filename
            ));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&zero_array)
            .create("particletype")
            .expect(&format!(
                "Unable to create dataset \"particletype\" in file {}",
                filename
            ));

        let particle_velocity: ndarray::Array2<f64> = timestep_data
            .slice(ndarray::s![.., 5usize..8usize])
            .to_owned();

        let particle_positions: ndarray::Array2<f64> = timestep_data
            .slice(ndarray::s![.., 2usize..5usize])
            .to_owned();

        print_debug!("Extracting statistical velocity information");
        for vel in particle_velocity.axis_iter(ndarray::Axis(0)) {
            for i in 0..3 {
                print_debug!("  i: {}", i);
                if vel[i] < velocity[[i, 0]] {
                    velocity[[i, 0]] = vel[i];
                } else if vel[i] > velocity[[i, 2]] {
                    velocity[[i, 2]] = vel[i];
                }
                velocity[[i, 1]] += vel[i];
            }
            let vel_mag = vel.map(|v| v * v).sum().sqrt();
            if vel_mag < velocity_mag[0] {
                velocity_mag[0] = vel_mag;
            } else if vel_mag > velocity_mag[2] {
                velocity_mag[2] = vel_mag;
            }
            velocity_mag[1] += vel_mag;
        }
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_velocity)
            .create("velocity")
            .expect(&format!(
                "Unable to create dataset \"velocity\" in file {}",
                filename
            ));
        print_debug!("New: {:?}", particle_positions);
        for pos in particle_positions.axis_iter(ndarray::Axis(0)) {
            for i in 0..3 {
                if pos[i] < dimensions[[0, i]] {
                    dimensions[[0, i]] = pos[i];
                } else if pos[i] > dimensions[[1, i]] {
                    dimensions[[1, i]] = pos[i];
                }
            }
        }
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_positions)
            .create("position")
            .expect(&format!(
                "Unable to create dataset \"position\" in file {}",
                filename
            ));
        step += 1;
        mean_counter += particle_id.len();
        sample_rate = current_time - old_time;
        old_time = current_time;
        if id % 10 == 0 {
            bar.inc(10);
            check_signals!();
        }
    }
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
        .write_scalar(&nparticles)
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

    print_debug!("Finished with conversion from vtk to HDF5 ");
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
            .map(|(a, b)| (*a as usize, *b as usize))
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
