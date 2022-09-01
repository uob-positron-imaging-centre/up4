//! This file provides functionalities to convert data to HDF5 file format
//!
//!
use crate::{check_signals, print_debug, setup_bar};
use csv;
use hdf5::File;
use indicatif::{ProgressBar, ProgressStyle};
use ndarray;
use ndarray_csv::Array2Reader;
use regex::Regex;
use std::path::Path;

mod convertertools;
mod vtktools;
// Maximum amount of failiures in a row available for a process
const MAX_FAILS: i64 = 5000;

/// Convert a vtk file into a HDF5 file
///
///
/// # Examples
///
/// Convert data from a sorted list of vtk files into Hdf5 (TData-format)
/// Filename in format: vtk_(Number).vtk, important for filtering the time for each file
/// whereas 'number' is the timestep of the simulation
///
/// see [regex](https://docs.rs/regex/1.5.4/regex/) for mor information about filtering
///'''
///vtk(
///     filenames: filename_list,
///     timestep: 1e-5,             //simulation timestep
///     outname: "output.hdf5",
///     filter: r"vtk_(\d+).vtk"    // regex filter to extract the timestep
///)
///'''
pub fn vtk(
    filenames: Vec<&str>,
    timestep: f64,
    outname: &str,
    filter: &str, // example r"vtk_(\d+).vtk"
) {
    if filenames.len() == 0 {
        panic!("No files to convert");
    }
    let re = Regex::new(filter).expect("Unable to create Regex filter.");
    let hdf5file = File::create(outname).expect("Unable to create HDF5 file.");
    // TODO Read this in the readers. check weather
    // the types are correct! 0x1 --> tdata 0x2 --> pdata
    hdf5file
        .new_attr::<u8>()
        .create("hdf5_up4_type")
        .unwrap()
        .write_scalar(&0x1_i32)
        .unwrap();

    let mut step = 0;
    let bar = ProgressBar::new(filenames.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% {per_sec} ({eta})")
        .with_key("eta", |state| format!("Time left: {:.1}s", state.eta().as_secs_f64()))
        .with_key("per_sec", |state| format!("{:.1} steps/s", state.per_sec()))
        .progress_chars("#>-"));
    // Attributes
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions.slice_mut(ndarray::s![0usize, ..]).fill(f64::MAX);
    dimensions.slice_mut(ndarray::s![1usize, ..]).fill(f64::MIN);
    let mut nparticles: u64 = 0;
    let timesteps: usize = filenames.len();
    let mut time: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(2);
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
    for filename in filenames.iter() {
        print_debug!("Creating a new group \"timestep {}\"", step);
        let group = hdf5file
            .create_group(&format!("timestep {}", step))
            .expect(&format!("Can not create group timestep {}", step));
        // Extracting data from filename
        let current_step: i64 = re
            .captures(filename)
            .expect(&format!(
                "Unable to match filename {} with filter {}",
                filename, filter
            ))
            .get(1)
            .expect(&format!(
                "Unable collect mfirst match  of filename {} with filter {}",
                filename, filter
            ))
            .as_str()
            .parse::<i64>()
            .expect(&format!(
                "Unable to parse string to i64 from match filename {} with filter {}",
                filename, filter
            ));
        let current_time = current_step as f64 * timestep;
        if step == 0 {
            time[0] = current_time;
        }
        if current_time < time[1] {
            panic!("Vtk files are not sorted into the correct order!");
        }
        time[1] = current_time;
        let builder = group
            .new_dataset::<f64>()
            .create("time")
            .unwrap()
            .write_scalar(&current_time)
            .unwrap();
        // VTK data reading
        print_debug!("Recieving data from VTKio and creating datasets");
        let particle_id = vtktools::get_field::<u64>(filename, "id");
        let max_particle = particle_id.iter().max().unwrap().clone();
        if max_particle > nparticles {
            nparticles = max_particle
        }
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_id)
            .create("id")
            .expect(&format!(
                "Unable to create dataset \"id\" in file {}",
                filename
            ));

        let particle_radius = vtktools::get_field::<f64>(filename, "radius");
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_radius)
            .create("radius")
            .expect(&format!(
                "Unable to create dataset \"radius\" in file {}",
                filename
            ));
        let ppclouds = ndarray::Array1::<u64>::ones(particle_radius.len());
        let builder = group.new_dataset_builder();
        builder
            .with_data(&ppclouds)
            .create("ppcloud")
            .expect(&format!(
                "Unable to create dataset \"radius\" in file {}",
                filename
            ));
        let particle_type = vtktools::get_field::<i64>(filename, "type");
        let builder = group.new_dataset_builder();
        builder
            .with_data(&particle_type)
            .create("particleid")
            .expect(&format!(
                "Unable to create dataset \"particleid\" in file {}",
                filename
            ));
        let particle_velocity = vtktools::get_field::<f64>(filename, "v");
        let particle_velocity =
            ndarray::Array::from_shape_vec((particle_velocity.len() / 3, 3), particle_velocity)
                .unwrap();
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
        let particle_positions = vtktools::get_positions::<f64>(filename);
        for fh in 0..4 {
            print_debug!(
                "Old: {:?},{:?},{:?}",
                particle_positions[fh * 3 + 0],
                particle_positions[fh * 3 + 1],
                particle_positions[fh * 3 + 2]
            );
        }
        let particle_positions =
            ndarray::Array::from_shape_vec((particle_positions.len() / 3, 3), particle_positions)
                .unwrap();
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
        bar.inc(1);
        check_signals!();
    } // end filename forloop
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

    bar.finish();
    print_debug!("Finished with conversion from vtk to HDF5 ");
} // end vtk function

/// Convert a folder with vtk files into a HDF5 file
///
///
/// # Examples
///
/// Convert data from a sorted list of vtk files into Hdf5 (TData-format)
/// Filename in format: vtk_(Number).vtk, important for filtering the time for each file
/// whereas 'number' is the timestep of the simulation
///
/// see [regex](https://docs.rs/regex/1.5.4/regex/) for mor information about filtering
///'''
///vtk(
///     filenames: "post/",
///     timestep: 1e-5,             //simulation timestep
///     outname: "output.hdf5",
///     filter: r"vtk_(\d+).vtk"    // regex filter to extract the timestep
///)
///'''
pub fn vtk_from_folder(
    mut folder: &str,
    timestep: f64,
    outname: &str,
    filter: &str, // example r"vtk_(\d+).vtk"
) {
    print_debug!("Starting conversion from vtk to HDF5 ");
    let system_foldername;
    let string;
    if !folder.ends_with(std::path::MAIN_SEPARATOR) {
        // add separator if not present
        string = format!("{}{}", folder, std::path::MAIN_SEPARATOR);
        system_foldername = string.as_str();
    } else {
        system_foldername = folder;
    }

    let filenames = std::fs::read_dir(system_foldername)
        .expect(&format!("Unable to read directory {}", system_foldername))
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect(&format!(
            "Unable to convert files in directory {}",
            system_foldername
        ));
    let mut out_vec: Vec<&str> = Vec::new();
    for filename_ in filenames.iter() {
        let filename = filename_.to_str().unwrap();
        if filename.ends_with(".vtk") && !filename.contains("boundingBox") {
            print_debug!("\t Found file: {}", filename);
        } else {
            print_debug!("\t Ignoring file: {}", filename);
            continue;
        }
        // append out vec
        out_vec.push(filename);
    }
    // sort the filenames
    out_vec.sort_unstable_by(|a, b| natord::compare(a, b));

    if out_vec.len() == 0 {
        panic!("No files to convert");
    }

    vtk(out_vec, timestep, outname, filter);
}

/// Convert a single trajectory csv file to Hdf5
pub fn csv_converter(
    filename: &str,
    outname: &str,
    mut columns: Vec<i64>,
    delimiter: &str,
    header: bool,
    comment: &str,
    vel: bool,
    interpolate: bool,
    radius: f64,
) {
    if !Path::new(&filename).exists() {
        panic!("CSV file {} does not exist.", &filename);
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
    let data: ndarray::Array2<f64> = {
        let mut data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> = rdr
            .deserialize_array2_dynamic()
            .expect("Unable to extract CSV data to ndarray! \nYour delimiter might be wrong.\n");
        if interpolate {
            data = convertertools::interpolate(data);
        }
        if vel {
            data = convertertools::velocity_polynom(data, 9, 2);
            if columns.len() >= 5 {
                panic!(
                    "Your columns are specified with more then 4 values and velocity \
                computation is activated. If you wish to ignore the velocity data \
                in your current data, only specify 4 columns indexing \
                time, x, y, z -position "
                )
            }
            columns.append(&mut vec![columns[1] + 3]);
            columns.append(&mut vec![columns[2] + 3]);
            columns.append(&mut vec![columns[3] + 3]);
        }
        data
    };
    // progress bar
    let data_length = data.column(0).len();

    let bar = setup_bar!("Converting", data_length);
    // Attributes
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions
        .slice_mut(ndarray::s![0 as usize, ..])
        .fill(f64::MAX);
    dimensions
        .slice_mut(ndarray::s![1 as usize, ..])
        .fill(f64::MIN);
    // arrays that will be saved:
    let mut time_array = ndarray::Array1::<f64>::zeros(data_length);
    let particle_id_array = ndarray::Array1::<f64>::ones(data_length);
    let particle_radius_array = ndarray::Array1::from_elem(data_length, radius);
    let ppclouds_array = ndarray::Array1::<f64>::ones(data_length);
    let particle_type_array = ndarray::Array1::<f64>::ones(data_length);
    let mut vel_array = ndarray::Array2::<f64>::zeros((data_length, 3));
    let mut pos_array = ndarray::Array2::<f64>::zeros((data_length, 3));
    // ######### arrays for Attributes:
    let timesteps: usize = data_length;
    let mut time: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(2);
    let mut sample_rate: f64 = 0.0;
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((3, 3));
    velocity
        .slice_mut(ndarray::s![.., 0 as usize])
        .fill(f64::MAX);
    velocity
        .slice_mut(ndarray::s![.., 2 as usize])
        .fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: ndarray::Array1<f64> = ndarray::Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;

    let mut mean_counter: usize = 0;
    let mut failcount = 0;
    let mut old_time = 0.0; // TIme of the last falid timestep
    let mut step = 0;
    print_debug!("Creating a new group \"particle {}\"", step);
    let group = hdf5file
        .create_group(&format!("particle {}", step))
        .expect(&format!("Can not create group particle {}", step));
    // Go through every line of the csv file

    for (line_id, line) in data.outer_iter().enumerate() {
        let current_time = line[columns[0] as usize];
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
        let pos_x = line[columns[1] as usize];
        let pos_y = line[columns[2] as usize];
        let pos_z = line[columns[3] as usize];
        let pos = ndarray::array![pos_x, pos_y, pos_z];
        pos_array[[line_id, 0]] = pos_x;
        pos_array[[line_id, 1]] = pos_y;
        pos_array[[line_id, 2]] = pos_z;

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
        step += 1;
        mean_counter += 1;
        sample_rate = current_time - old_time;
        if current_time > time[1] {
            time[1] = current_time;
        }
        old_time = current_time;
        bar.inc(1);
        check_signals!();
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
        .expect(&format!(
            "Unable to create dataset \"id\" in file {}",
            filename
        ));
    let builder = group.new_dataset_builder();
    builder
        .with_data(&particle_radius_array)
        .create("radius")
        .expect(&format!(
            "Unable to create dataset \"radius\" in file {}",
            filename
        ));
    let builder = group.new_dataset_builder();
    builder
        .with_data(&ppclouds_array)
        .create("ppcloud")
        .expect(&format!(
            "Unable to create dataset \"radius\" in file {}",
            filename
        ));
    let builder = group.new_dataset_builder();
    builder
        .with_data(&particle_type_array)
        .create("particleid")
        .expect(&format!(
            "Unable to create dataset \"particleid\" in file {}",
            filename
        ));

    let builder = group.new_dataset_builder();
    builder
        .with_data(&vel_array)
        .create("velocity")
        .expect(&format!(
            "Unable to create dataset \"velocity\" in file {}",
            filename
        ));
    let builder = group.new_dataset_builder();
    builder
        .with_data(&pos_array)
        .create("position")
        .expect(&format!(
            "Unable to create dataset \"position\" in file {}",
            filename
        ));
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

    bar.finish();
    print_debug!("Finished with conversion from vtk to HDF5 ");
}
