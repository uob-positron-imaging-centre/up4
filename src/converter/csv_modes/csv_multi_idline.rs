use crate::{
    check_signals,
    converter::convertertools::sort_by_column,
    print_debug, setup_bar,
};
use csv;
use hdf5::File;
// use indicatif::{ProgressBar, ProgressStyle};
use ndarray;
use ndarray_csv::Array2Reader;
use std::collections::HashMap;

use crate::converter::convertertools;
// Maximum amount of failures in a row available for a process
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
        .new_attr::<i32>()
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
            if cfg!(debug_assertions) {
                println!("Data after interpolation: {:?}", temp_data);
            }
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
                // currently turned of due to bug in parallel computation
                if true {
                    temp_data = convertertools::velocity_polynom(temp_data, 9, 2);
                } else {
                    temp_data = convertertools::velocity_parallel::velocity_polynom_parallel(
                        temp_data, 9, 2,
                    );
                }
            }
            if cfg!(debug_assertions) {
                println!("Data after velocity calc: {:?}", temp_data);
            }
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
    velocity.slice_mut(ndarray::s![.., 0_usize]).fill(f64::MAX);
    velocity.slice_mut(ndarray::s![.., 2_usize]).fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;
    // ######### arrays for Attributes:
    let mut timesteps: usize = 0;
    let mut time_min = f64::INFINITY;
    let mut time_max = f64::NEG_INFINITY;
    let mut dt_sum = 0.0;
    let mut dt_count = 0_usize;
    let mut global_time_values: Vec<f64> = Vec::new();
    bar.inc(10);
    for (p_id, data) in particle_data.iter().enumerate() {
        // progress bar
        if cfg!(debug_assertions) {
            println!("data.shape(): {:?}", data.shape());
        }
        // Layout must be [t, x, y, z, vx, vy, vz] after removing the id column.
        if data.ncols() != 7 {
            panic!(
                "Expected particle data to have 7 columns [t, x, y, z, vx, vy, vz] after removing \
                 the id column. Got {} columns. Provide velocity columns in the CSV (8 columns \
                 total) or enable velocity calculation (vel=true) when supplying only positions.",
                data.ncols()
            );
        }
        if data.nrows() == 0 {
            panic!("Particle {} does not contain any rows after processing", p_id);
        }
        if data[[0, 6]].is_nan() {
            panic!(
                "Velocity information required: column layout must be [t, x, y, z, vx, vy, vz]. \
                 Provide velocities in the CSV or enable velocity calculation (vel=true)."
            )
        }
        let data_length = data.column(0).len();
        // Attributes
        // arrays that will be saved:
        let mut particle_time_array = ndarray::Array1::<f64>::zeros(data_length);
        let particle_id_array = ndarray::Array1::<f64>::from_elem(data_length, p_id as f64);
        let particle_radius_array = ndarray::Array1::from_elem(data_length, radius);
        let ppclouds_array = ndarray::Array1::<f64>::ones(data_length);
        let particle_type_array = ndarray::Array1::<f64>::zeros(data_length);
        let mut vel_array = ndarray::Array2::<f64>::zeros((data_length, 3));
        let mut pos_array = ndarray::Array2::<f64>::zeros((data_length, 3));
        let mut failcount = 0;
        let mut previous_time: Option<f64> = None;
        let mut write_index = 0_usize; // number of valid rows written

        print_debug!("Creating a new group \"particle {}\"", p_id);
        let group = hdf5file
            .create_group(&format!("particle {}", p_id))
            .unwrap_or_else(|_| panic!("Can not create group particle {}", p_id));

        // Go through every line of the csv file
        for (line_id, line) in data.outer_iter().enumerate() {
            let current_time = line[0];
            if let Some(old_time) = previous_time {
                if current_time <= old_time {
                    // The particle went back in time
                    // This is not possible and must be ignored.
                    failcount += 1;
                    if failcount > MAX_FAILS {
                        panic!(
                            "Maximum amount of points that are behind the current \
                     time reached. Please Check whether your data contains multiple\
                     trajectories that are sorted in label.
                     "
                        )
                    }
                    continue;
                }
                dt_sum += current_time - old_time;
                dt_count += 1;
            }
            previous_time = Some(current_time);
            // resetfailcount. we only dont allow them do be in a row!
            failcount = 0;
            particle_time_array[write_index] = current_time;
            let pos_x = line[1];
            let pos_y = line[2];
            let pos_z = line[3];
            let pos = ndarray::array![pos_x, pos_y, pos_z];
            pos_array[[write_index, 0]] = pos_x;
            pos_array[[write_index, 1]] = pos_y;
            pos_array[[write_index, 2]] = pos_z;
            let v_x = line[4];
            let v_y = line[5];
            let v_z = line[6];
            vel_array[[write_index, 0]] = v_x;
            vel_array[[write_index, 1]] = v_y;
            vel_array[[write_index, 2]] = v_z;
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
            if current_time < time_min {
                time_min = current_time;
            }
            if current_time > time_max {
                time_max = current_time;
            }
            global_time_values.push(current_time);
            write_index += 1;
            if line_id % 1000 == 0 {
                check_signals!();
            }
        } // end filename forloop
        if write_index == 0 {
            panic!("No valid rows found for particle {}", p_id);
        }
        let particle_time_array = particle_time_array
            .slice(ndarray::s![..write_index])
            .to_owned();
        let particle_id_array = particle_id_array
            .slice(ndarray::s![..write_index])
            .to_owned();
        let particle_radius_array = particle_radius_array
            .slice(ndarray::s![..write_index])
            .to_owned();
        let ppclouds_array = ppclouds_array
            .slice(ndarray::s![..write_index])
            .to_owned();
        let particle_type_array = particle_type_array
            .slice(ndarray::s![..write_index])
            .to_owned();
        let vel_array = vel_array
            .slice(ndarray::s![..write_index, ..])
            .to_owned();
        let pos_array = pos_array
            .slice(ndarray::s![..write_index, ..])
            .to_owned();
        if write_index > timesteps {
            timesteps = write_index;
        }
        // write data into HDF5 file
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_time_array)
            .create("time")
            .expect("Unable to create dataset \"time\"");
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_id_array)
            .create("id")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"id\" in file {}", filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_radius_array)
            .create("radius")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&ppclouds_array)
            .create("ppcloud")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_type_array)
            .create("particletype")
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to create dataset \"particletype\" in file {}",
                    filename
                )
            });

        let builder = group.new_dataset_builder();
        builder
            .with_data(&vel_array)
            .create("velocity")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"velocity\" in file {}", filename)
            });
        let builder = group.new_dataset_builder();
        builder
            .with_data(&pos_array)
            .create("position")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"position\" in file {}", filename)
            });
    }
    bar.inc(20);
    if mean_counter > 0 {
        velocity_mag[1] /= mean_counter as f64;
        velocity[[0, 1]] /= mean_counter as f64;
        velocity[[1, 1]] /= mean_counter as f64;
        velocity[[2, 1]] /= mean_counter as f64;
    }
    // Use mean dt across all particles as sample rate.
    let sample_rate = if dt_count > 0 {
        dt_sum / dt_count as f64
    } else {
        0.0
    };
    global_time_values.retain(|v| !v.is_nan());
    global_time_values.sort_by(|a, b| {
        a.partial_cmp(b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    global_time_values.dedup();
    let time = if global_time_values.is_empty() {
        ndarray::array![0.0, 0.0]
    } else {
        ndarray::array![time_min, time_max]
    };
    let time_array = ndarray::Array1::from(global_time_values);
    if time_array.len() > timesteps {
        timesteps = time_array.len();
    }
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
        .write_scalar(&(particle_data.len() as u64))
        .unwrap();
    hdf5file
        .new_attr::<u64>()
        .create("timesteps")
        .unwrap()
        .write_scalar(&(timesteps as u64))
        .unwrap();
    hdf5file
        .new_attr::<f64>()
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
    let mut id_to_index: HashMap<usize, usize> = HashMap::new();
    let mut ids_and_steps: Vec<(usize, usize)> = Vec::new(); // (id, steps)
    let mut max_steps = 0_usize;
    let mut max_t = data[[0, 0]];
    for row in 0..data.shape()[0] {
        let id = data[[row, 1]] as usize;
        let t = data[[row, 0]];
        if t > max_t {
            max_t = t;
        }
        let entry_index = *id_to_index.entry(id).or_insert_with(|| {
            ids_and_steps.push((id, 0));
            ids_and_steps.len() - 1
        });
        ids_and_steps[entry_index].1 += 1;
        if ids_and_steps[entry_index].1 > max_steps {
            max_steps = ids_and_steps[entry_index].1;
        }
    }

    ids_and_steps.sort_by_key(|(id, _)| *id);

    let mut sorted_data = Vec::with_capacity(ids_and_steps.len());
    let mut id_to_sorted_index: HashMap<usize, usize> = HashMap::new();
    for (sorted_idx, (id, steps)) in ids_and_steps.iter().enumerate() {
        sorted_data.push(ndarray::Array2::zeros((*steps, 8)));
        let mut id_line = sorted_data[sorted_idx].slice_mut(ndarray::s![.., 1]);
        id_line.fill(*id as f64);
        id_to_sorted_index.insert(*id, sorted_idx);
    }

    let mut last_line_pushed = vec![0; sorted_data.len()];
    for row in 0..data.shape()[0] {
        let id = data[[row, 1]] as usize;
        let index = *id_to_sorted_index
            .get(&id)
            .expect("Particle id not found in index map");
        let line = last_line_pushed[index];
        let mut line = sorted_data[index].slice_mut(ndarray::s![line, ..]);
        last_line_pushed[index] += 1;
        line.assign(&data.slice(ndarray::s![row, ..]));
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

#[cfg(test)]
mod tests {
    use super::csv_multi_idline;
    use hdf5::File;
    use std::{
        fs,
        io::Write,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn csv_multi_idline_writes_multiple_particles_and_metadata() {
        let base_dir = std::path::PathBuf::from("target/test_output");
        fs::create_dir_all(&base_dir).unwrap();
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let pid = std::process::id();
        let csv_path = base_dir.join(format!("csv_multi_idline_{pid}_{suffix}.csv"));
        let hdf5_path = base_dir.join(format!("csv_multi_idline_{pid}_{suffix}.h5"));

        let mut csv_file = fs::File::create(&csv_path).unwrap();
        writeln!(csv_file, "t,id,x,y,z,vx,vy,vz").unwrap();
        for line in &[
            "1.0,0,1.0,0.0,0.0,1.5,0.1,0.0",
            "0.0,1,10.0,0.0,0.0,0.0,1.0,0.0",
            "0.0,0,0.0,0.0,0.0,1.0,0.0,0.0",
            "2.0,1,10.0,2.0,0.0,0.0,1.0,0.0",
            "2.0,0,2.0,0.0,0.0,1.0,0.0,0.0",
            "1.5,1,10.0,1.5,0.0,0.0,1.0,0.0",
        ] {
            writeln!(csv_file, "{line}").unwrap();
        }

        csv_multi_idline(
            csv_path.to_str().unwrap(),
            hdf5_path.to_str().unwrap(),
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            ",",
            true,
            "#",
            false,
            false,
            0.1,
        );

        let file = File::open(&hdf5_path).unwrap();
        let particle_number: u64 = file
            .attr("particle number")
            .unwrap()
            .read_scalar()
            .unwrap();
        assert_eq!(particle_number, 2);

        let root_time: Vec<f64> = file.attr("time").unwrap().read_raw().unwrap();
        assert_eq!(root_time, vec![0.0, 2.0]);

        let root_time_array = file
            .dataset("time array")
            .unwrap()
            .read_1d::<f64>()
            .unwrap();
        assert_eq!(root_time_array.to_vec(), vec![0.0, 1.0, 1.5, 2.0]);

        let particle0 = file.group("particle 0").unwrap();
        let p0_time = particle0.dataset("time").unwrap().read_1d::<f64>().unwrap();
        assert_eq!(p0_time.to_vec(), vec![0.0, 1.0, 2.0]);
        let p0_pos = particle0
            .dataset("position")
            .unwrap()
            .read_2d::<f64>()
            .unwrap();
        assert_eq!(p0_pos[[0, 0]], 0.0);
        assert_eq!(p0_pos[[2, 0]], 2.0);

        let particle1 = file.group("particle 1").unwrap();
        let p1_time = particle1.dataset("time").unwrap().read_1d::<f64>().unwrap();
        assert_eq!(p1_time.to_vec(), vec![0.0, 1.5, 2.0]);
        let p1_vel = particle1
            .dataset("velocity")
            .unwrap()
            .read_2d::<f64>()
            .unwrap();
        assert_eq!(p1_vel[[0, 1]], 1.0);
        assert_eq!(p1_vel[[2, 1]], 1.0);

        let _ = fs::remove_file(csv_path);
        let _ = fs::remove_file(hdf5_path);
    }
}
