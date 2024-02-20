//! This file provides functionalities to convert data to HDF5 file format
//!
//!
#![allow(unused_imports)]
use crate::{
    check_signals,
    converter::convertertools::{make_dataset_builder, make_sortlist, sort_by_array},
    print_debug, print_warning, setup_bar,
};
use csv;
pub use csv_modes::csv_multi_file_time_step;
#[cfg(feature = "blosc")]
use hdf5::filters::{blosc_get_nthreads, blosc_set_nthreads};
use hdf5::File;
use indicatif::{ProgressBar, ProgressStyle};
use ndarray::{self, s, Array, Array1, Array2, ArrayBase, Axis, Dim, OwnedRepr};
use ndarray_csv::Array2Reader;
use numpy::array;
use regex::Regex;
use std::path::Path;
use vtkio::model::DataSet;

mod convertertools;
mod csv_modes;
mod vtktools;
mod vtutools;
// Maximum amount of failiures in a row available for a process
const MAX_FAILS: i64 = 500;
// Shuffle for blosc filter
const BLOSC_SHUFFLE: bool = true;
// Compression level for blosc filter
const BLOSC_COMPRESSION: u8 = 9;
// Chunk size for blosc filter

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
    if filenames.is_empty() {
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

    let bar = setup_bar!("Vtk Read Data", filenames.len() as u64);
    // Attributes
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: Array2<f64> = Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions.slice_mut(s![0usize, ..]).fill(f64::MAX);
    dimensions.slice_mut(s![1usize, ..]).fill(f64::MIN);
    let mut nparticles: u64 = 0;
    let timesteps: usize = filenames.len();
    let mut time: Array1<f64> = Array1::<f64>::zeros(2);
    let mut time_array = Vec::<f64>::new();
    let mut sample_rate: f64 = 0.0;
    let mut old_time = 0.0;
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: Array2<f64> = Array2::<f64>::zeros((3, 3));
    velocity.slice_mut(s![.., 0usize]).fill(f64::MAX);
    velocity.slice_mut(s![.., 2usize]).fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: Array1<f64> = Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;

    let mut mean_counter: usize = 0;
    for (id, filename) in filenames.iter().enumerate() {
        print_debug!("Creating a new group \"timestep {}\"", id);
        let group = hdf5file
            .create_group(&format!("timestep {}", id))
            .unwrap_or_else(|_| panic!("Can not create group timestep {}", id));
        // Extracting data from filename
        let current_step: i64 = re
            .captures(filename)
            .unwrap_or_else(|| {
                panic!(
                    "Unable to match filename {} with filter {}",
                    filename, filter
                )
            })
            .get(1)
            .unwrap_or_else(|| {
                panic!(
                    "Unable collect mfirst match  of filename {} with filter {}",
                    filename, filter
                )
            })
            .as_str()
            .parse::<i64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to parse string to i64 from match filename {} with filter {}",
                    filename, filter
                )
            });
        let current_time = current_step as f64 * timestep;
        if id == 0 {
            time[0] = current_time;
        }
        if current_time < time[1] {
            panic!("Vtk files are not sorted into the correct order!");
        }
        time[1] = current_time;
        time_array.push(current_time);
        let builder = group.new_dataset::<f64>();
        builder
            .create("time")
            .expect("Unable to create dataset time")
            .write_scalar(&current_time)
            .expect("Unable to write time to dataset");
        // VTK data reading
        print_debug!("Recieving data from VTKio and creating datasets");
        let particle_id = vtktools::get_field::<u64>(filename, "id");
        let sort_list = make_sortlist(&particle_id);
        print_debug!("  Made sortlist");
        let particle_id = sort_by_array(particle_id, &sort_list);
        print_debug!("  Sorted");
        let max_particle = *particle_id.iter().max().unwrap();
        print_debug!("  Made sortlist");
        if max_particle > nparticles {
            nparticles = max_particle
        }
        print_debug!("  Creating particle id dataset");
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_id)
            .create("id")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"id\" in file {}", filename));
        print_debug!("  Creating radius dataset");
        let particle_radius =
            sort_by_array(vtktools::get_field::<f64>(filename, "radius"), &sort_list);
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_radius)
            .create("radius")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
        print_debug!("  Creating ppclouds dataset");
        let ppclouds = Array1::<u64>::ones(particle_radius.len());
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&ppclouds)
            .create("ppcloud")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
        print_debug!("  Creating type dataset");
        let particle_type = sort_by_array(vtktools::get_field::<i64>(filename, "type"), &sort_list);
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_type)
            .create("particletype")
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to create dataset \"particletype\" in file {}",
                    filename
                )
            });
        print_debug!("  Creating velocity dataset");
        let particle_velocity =
            sort_by_array(vtktools::get_field::<f64>(filename, "v"), &sort_list);
        let particle_velocity =
            Array::from_shape_vec((particle_velocity.len() / 3, 3), particle_velocity).unwrap();
        print_debug!("Extracting statistical velocity information");
        for vel in particle_velocity.axis_iter(Axis(0)) {
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
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_velocity)
            .create("velocity")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"velocity\" in file {}", filename)
            });
        print_debug!("  Creating position dataset");
        let particle_positions =
            sort_by_array(vtktools::get_positions::<f64>(filename), &sort_list);

        let particle_positions =
            Array::from_shape_vec((particle_positions.len() / 3, 3), particle_positions).unwrap();
        print_debug!("New: {:?}", particle_positions);
        for pos in particle_positions.axis_iter(Axis(0)) {
            for i in 0..3 {
                if pos[i] < dimensions[[0, i]] {
                    dimensions[[0, i]] = pos[i];
                } else if pos[i] > dimensions[[1, i]] {
                    dimensions[[1, i]] = pos[i];
                }
            }
        }
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_positions)
            .create("position")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"position\" in file {}", filename)
            });
        mean_counter += particle_id.len();
        sample_rate = current_time - old_time;
        old_time = current_time;
        if id % 10 == 0 {
            bar.inc(10);
            check_signals!();
        }
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
    hdf5file
        .new_dataset_builder()
        .with_data(&time_array)
        .create("time array")
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
    folder: &str,
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
        .unwrap_or_else(|_| panic!("Unable to read directory {}", system_foldername))
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap_or_else(|_| panic!("Unable to convert files in directory {}", system_foldername));
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

    if out_vec.is_empty() {
        panic!("No files to convert");
    }

    vtk(out_vec, timestep, outname, filter);
}

pub fn vtu(
    filenames: Vec<&str>,
    timestep: f64,
    outname: &str,
    filter: &str, // example r"vtk_(\d+).pvtu"
) {
    if filenames.is_empty() {
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

    let bar = setup_bar!("Vtk Read Data", filenames.len() as u64);
    // Attributes
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: Array2<f64> = Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions.slice_mut(s![0usize, ..]).fill(f64::MAX);
    dimensions.slice_mut(s![1usize, ..]).fill(f64::MIN);
    let mut nparticles: u64 = 0;
    let timesteps: usize = filenames.len();
    let mut time: Array1<f64> = Array1::<f64>::zeros(2);
    let mut time_array = Vec::<f64>::new();
    let mut sample_rate: f64 = 0.0;
    let mut old_time = 0.0;
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: Array2<f64> = Array2::<f64>::zeros((3, 3));
    velocity.slice_mut(s![.., 0usize]).fill(f64::MAX);
    velocity.slice_mut(s![.., 2usize]).fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: Array1<f64> = Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;

    let mut mean_counter: usize = 0;
    for (id, filename) in filenames.iter().enumerate() {
        print_debug!("Creating a new group \"timestep {}\"", id);
        let group = hdf5file
            .create_group(&format!("timestep {}", id))
            .unwrap_or_else(|_| panic!("Can not create group timestep {}", id));
        // Extracting data from filename
        let current_step: i64 = re
            .captures(filename)
            .unwrap_or_else(|| {
                panic!(
                    "Unable to match filename {} with filter {}",
                    filename, filter
                )
            })
            .get(1)
            .unwrap_or_else(|| {
                panic!(
                    "Unable collect mfirst match  of filename {} with filter {}",
                    filename, filter
                )
            })
            .as_str()
            .parse::<i64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to parse string to i64 from match filename {} with filter {}",
                    filename, filter
                )
            });
        let current_time = current_step as f64 * timestep;
        if id == 0 {
            time[0] = current_time;
        }
        if current_time < time[1] {
            panic!("Vtk files are not sorted into the correct order!");
        }
        time[1] = current_time;
        time_array.push(current_time);
        let builder = group.new_dataset::<f64>();
        builder
            .create("time")
            .expect("Unable to create dataset time")
            .write_scalar(&current_time)
            .expect("Unable to write time to dataset");
        // VTK data reading
        print_debug!("Recieving data from VTKio and creating datasets");
        let particle_id = vtutools::get_field::<u64>(filename, "ID");
        let sort_list = make_sortlist(&particle_id);
        print_debug!("  Made sortlist");
        let particle_id = sort_by_array(particle_id, &sort_list);
        print_debug!("  Sorted");
        let max_particle = *particle_id.iter().max().unwrap();
        print_debug!("  Made sortlist");
        if max_particle > nparticles {
            nparticles = max_particle
        }
        print_debug!("  Creating particle id dataset");
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_id)
            .create("id")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"ID\" in file {}", filename));
        print_debug!("  Creating radius dataset");
        let particle_radius =
            sort_by_array(vtutools::get_field::<f64>(filename, "Diameter"), &sort_list)
                .iter()
                .map(|x| x * 0.5)
                .collect::<Vec<f64>>();
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_radius)
            .create("radius")
            .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
        print_debug!("  Creating ppclouds dataset");
        let ppclouds = Array1::<u64>::ones(particle_radius.len());
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&ppclouds)
            .create("ppcloud")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"ppcloud\" in file {}", filename)
            });
        print_debug!("  Creating type dataset");
        let particle_type = sort_by_array(vtutools::get_field::<i64>(filename, "Type"), &sort_list);
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_type)
            .create("particletype")
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to create dataset \"particletype\" in file {}",
                    filename
                )
            });
        print_debug!("  Creating velocity dataset");
        let particle_velocity =
            sort_by_array(vtutools::get_field::<f64>(filename, "Velocity"), &sort_list);
        let particle_velocity =
            Array::from_shape_vec((particle_velocity.len() / 3, 3), particle_velocity).unwrap();
        print_debug!("Extracting statistical velocity information");
        for vel in particle_velocity.axis_iter(Axis(0)) {
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
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_velocity)
            .create("velocity")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"velocity\" in file {}", filename)
            });
        print_debug!("  Creating position dataset");
        let particle_positions =
            sort_by_array(vtutools::get_positions::<f64>(filename), &sort_list);

        let particle_positions =
            Array::from_shape_vec((particle_positions.len() / 3, 3), particle_positions).unwrap();
        print_debug!("New: {:?}", particle_positions);
        for pos in particle_positions.axis_iter(Axis(0)) {
            for i in 0..3 {
                if pos[i] < dimensions[[0, i]] {
                    dimensions[[0, i]] = pos[i];
                } else if pos[i] > dimensions[[1, i]] {
                    dimensions[[1, i]] = pos[i];
                }
            }
        }
        let builder = make_dataset_builder!(group);
        builder
            .with_data(&particle_positions)
            .create("position")
            .unwrap_or_else(|_| {
                panic!("Unable to create dataset \"position\" in file {}", filename)
            });
        mean_counter += particle_id.len();
        sample_rate = current_time - old_time;
        old_time = current_time;
        if id % 10 == 0 {
            bar.inc(10);
            check_signals!();
        }
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
    hdf5file
        .new_dataset_builder()
        .with_data(&time_array)
        .create("time array")
        .unwrap();

    bar.finish();
    print_debug!("Finished with conversion from vtu to HDF5 ");
}

pub fn vtu_from_folder(
    folder: &str,
    timestep: f64,
    outname: &str,
    filter: &str, // example r"vtk_(\d+).vtk"
) {
    print_debug!("Starting conversion from vtu to HDF5 ");
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
        .unwrap_or_else(|_| panic!("Unable to read directory {}", system_foldername))
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap_or_else(|_| panic!("Unable to convert files in directory {}", system_foldername));
    let mut out_vec: Vec<&str> = Vec::new();
    for filename_ in filenames.iter() {
        let filename = filename_.to_str().unwrap();
        if filename.ends_with(".pvtu")
            && !filename.contains("postprocess")
            && !filename.contains("boundaries")
            && !filename.contains("solid")
        {
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

    if out_vec.is_empty() {
        panic!("No files to convert");
    }
    println!("files = {:?}", out_vec);
    vtu(out_vec, timestep, outname, filter);
}

/// Convert a single trajectory csv file to Hdf5
// The number of arguments is necessary for proper csv reading.
#[allow(clippy::too_many_arguments)]
pub fn csv_converter(
    filename: &str,
    outname: &str,
    columns: Vec<i64>,
    delimiter: &str,
    header: bool,
    comment: &str,
    vel: bool,
    interpolate: bool,
    radius: f64,
    sampling_steps: usize,
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
    #[cfg(feature = "blosc")]
    let threads = blosc_get_nthreads();
    #[cfg(feature = "blosc")]
    println!(
        "Using {} threads for blosc compression. (Not working currently",
        threads
    );
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(threads);
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
    let data: Array2<f64> = {
        let mut data: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> = rdr
            .deserialize_array2_dynamic()
            .expect("Unable to extract CSV data to ndarray! \nYour delimiter might be wrong.\n");
        // slice the read array to only get the colums requested
        if !columns.is_empty() {
            let mut temp_data = Array2::<f64>::from_elem((data.shape()[0], 7), f64::NAN);
            for (i, column) in columns.iter().enumerate() {
                temp_data
                    .slice_mut(s![.., i])
                    .assign(&data.slice(s![.., *column as usize]));
            }
            data = temp_data;
        } else {
            panic!("No columns selected to extract!");
        }
        // Check if time is sorted if not sort the time
        if convertertools::is_sorted(&data.slice(s![.., 0])) {
            print_debug!("Time is sorted");
        } else {
            print_debug!("Time is not sorted");
            print_warning!("Sorting data according to time");
            let new_data = data;
            data = convertertools::sort_by_column(new_data, 0);
        }
        if interpolate {
            let mut t = data.slice_mut(s![.., 0_usize]);
            //set first timestep to 0 by substracting the first timestep from all timesteps
            t -= t[0];
            let max_t = t[t.len() - 1];
            let steps = t.len();
            print_debug!("Data before interpolation: {}", data);
            data = convertertools::interpolate(data, max_t, steps);
            print_debug!("Data after interpolation: {}", data);
        }
        if vel {
            if columns.len() >= 5 {
                panic!(
                    "Your columns are specified with more then 4 values and velocity \
                computation is activated. If you wish to ignore the velocity data \
                in your current data, only specify 4 columns indexing \
                time, x, y, z -position "
                )
            }
            if true {
                //data.column(0).len() < 100000000000 {
                data = convertertools::velocity_polynom(data, sampling_steps, 2);
            } else {
                data = convertertools::velocity_paralell::velocity_polynom_parallel(
                    data,
                    sampling_steps,
                    2,
                );
            }
        }
        data
    };
    print_debug!("Data: {:?}", data);
    print_debug!("Data shape: {:?}", data.shape());

    // progress bar
    let data_length = data.column(0).len();

    let bar = setup_bar!("Converting", data_length);
    // Attributes
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: Array2<f64> = Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions.slice_mut(s![0_usize, ..]).fill(f64::MAX);
    dimensions.slice_mut(s![1_usize, ..]).fill(f64::MIN);
    // arrays that will be saved:
    let mut time_array = Array1::<f64>::zeros(data_length);
    let particle_id_array = Array1::<f64>::ones(data_length);
    let particle_radius_array = Array1::from_elem(data_length, radius);
    let ppclouds_array = Array1::<f64>::ones(data_length);
    let particle_type_array = Array1::<f64>::ones(data_length);
    let mut vel_array = Array2::<f64>::zeros((data_length, 3));
    let mut pos_array = Array2::<f64>::zeros((data_length, 3));
    // ######### arrays for Attributes:
    let timesteps: usize = data_length;
    let mut time: Array1<f64> = Array1::<f64>::zeros(2);
    let mut sample_rate: f64 = 0.0;
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: Array2<f64> = Array2::<f64>::zeros((3, 3));
    velocity.slice_mut(s![.., 0_usize]).fill(f64::MAX);
    velocity.slice_mut(s![.., 2_usize]).fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: Array1<f64> = Array1::<f64>::zeros(3);
    velocity_mag[0] = f64::MAX;
    velocity_mag[2] = f64::MIN;

    let mut mean_counter: usize = 0;
    let mut failcount = 0;
    let mut old_time = 0.0; // TIme of the last falid timestep
    let mut step = 0;
    print_debug!("Creating a new group \"particle {}\"", step);
    let group = hdf5file
        .create_group(&format!("particle {}", step))
        .unwrap_or_else(|_| panic!("Can not create group particle {}", step));

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
        let pos = array![pos_x, pos_y, pos_z];
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
        if line_id % 1000 == 0 {
            bar.inc(1000);
            check_signals!();
        }
    } // end filename forloop
      // write data into HDF5 file
    let builder = make_dataset_builder!(group);
    builder
        .with_data(&time_array)
        .create("time")
        .expect("Unable to create dataset \"time\"");
    let builder = make_dataset_builder!(group);
    builder
        .with_data(&particle_id_array)
        .create("id")
        .unwrap_or_else(|_| panic!("Unable to create dataset \"id\" in file {}", filename));
    let builder = make_dataset_builder!(group);
    builder
        .with_data(&particle_radius_array)
        .create("radius")
        .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
    let builder = make_dataset_builder!(group);
    builder
        .with_data(&ppclouds_array)
        .create("ppcloud")
        .unwrap_or_else(|_| panic!("Unable to create dataset \"radius\" in file {}", filename));
    let builder = make_dataset_builder!(group);
    builder
        .with_data(&particle_type_array)
        .create("particletype")
        .unwrap_or_else(|_| {
            panic!(
                "Unable to create dataset \"particletype\" in file {}",
                filename
            )
        });

    let builder = make_dataset_builder!(group);
    builder
        .with_data(&vel_array)
        .create("velocity")
        .unwrap_or_else(|_| panic!("Unable to create dataset \"velocity\" in file {}", filename));
    let builder = make_dataset_builder!(group);
    builder
        .with_data(&pos_array)
        .create("position")
        .unwrap_or_else(|_| panic!("Unable to create dataset \"position\" in file {}", filename));
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

    bar.finish();
    print_debug!("Finished with conversion from vtk to HDF5 ");
}

// This function is used to read old hdf5 files and convert it to newer format
//fn hdf5(

/// Read in A CSV file containing multiple particles and convert it to HDF5
#[allow(unreachable_code, unused_variables, clippy::too_many_arguments)]
pub fn csv_multi_converter(
    filename: &str,
    outname: &str,
    columns: Vec<i64>,
    delimiter: &str,
    header: bool,
    comment: &str,
    vel: bool,
    interpolate: bool,
    radius: f64,
    mode: &str,
) {
    //unimplemented!("CSV Multi Converter is not implemented yet");
    println!(
        "Warning! This function is highly experimental and may not work as \
 expected! In this stage the current \"chain\" method cuts data to \
 make sure all particles have the same number of timesteps and live \
 in the same time frame. Be careful interpreting the data!"
    );
    // continue with the "Chain" mode
    if !Path::new(&filename).exists() {
        panic!("CSV file {} does not exist.", &filename);
    }
    if mode == "chain" {
        csv_modes::csv_multi_chain(
            filename,
            outname,
            columns,
            delimiter,
            header,
            comment,
            vel,
            interpolate,
            radius,
        );
    } else if mode == "id_line" {
        csv_modes::csv_multi_idline(
            filename,
            outname,
            columns,
            delimiter,
            header,
            comment,
            vel,
            interpolate,
            radius,
        );
    } else {
        println!("Mode not recognized! Please use \"chain\" or \"id_line\".");
    };
}

mod legacytools;
mod xmltools;

#[derive(Default)]
pub struct VTKConverter {
    velocity_field_name: Option<String>,
    id_field_name: Option<String>,
    radius_field_name: Option<String>,
    type_field_name: Option<String>,
    file_type: VTKType,
    is_legacy: bool,
    diameter_field_name: Option<String>,
}

impl VTKConverter {
    pub fn new(vtk_type: VTKType) -> Self {
        VTKConverter {
            file_type: vtk_type,
            ..Default::default()
        }
    }

    pub fn add_velocity_field(self, name: &str) -> Self {
        VTKConverter {
            velocity_field_name: Some(name.to_string()),
            ..self
        }
    }

    pub fn add_id_field(self, name: &str) -> Self {
        VTKConverter {
            id_field_name: Some(name.to_string()),
            ..self
        }
    }

    pub fn add_radius_field(self, name: &str) -> Self {
        VTKConverter {
            radius_field_name: Some(name.to_string()),
            ..self
        }
    }

    pub fn add_type_field(self, name: &str) -> Self {
        VTKConverter {
            type_field_name: Some(name.to_string()),
            ..self
        }
    }

    pub fn add_diameter_field(self, name: &str) -> Self {
        VTKConverter {
            diameter_field_name: Some(name.to_string()),
            ..self
        }
    }

    pub fn is_legacy(self, is_legacy: bool) -> Self {
        VTKConverter { is_legacy, ..self }
    }

    pub fn write_hdf5(self, filenames: Vec<&str>, timestep: f64, outname: &str, filter: &str) {
        if filenames.is_empty() {
            panic!("No files to convert");
        }
        let re = Regex::new(filter).expect("Unable to create Regex filter.");
        let output_file = File::create(outname).expect("Unable to create HDF5 file.");
        // Place an identifier for if the data is tdata (0x1) or pdata (0x2)
        output_file
            .new_attr::<u8>()
            .create("hdf5_up4_type")
            .unwrap()
            .write_scalar(&0x1_i32)
            .unwrap();
        let progress_bar = setup_bar!("Vtk Read Data", filenames.len() as u64);
        // Find and set the attributes
        let mut dimensions: Array2<f64> = Array2::<f64>::zeros((2, 3)); // [min:[x,y,z],max:[x,y,z]]
        dimensions.slice_mut(s![0usize, ..]).fill(f64::MAX);
        dimensions.slice_mut(s![1usize, ..]).fill(f64::MIN);
        let mut nparticles: u64 = 0;
        let timesteps = filenames.len();
        let mut time: Array1<f64> = Array1::<f64>::zeros(2);
        let mut time_array = Vec::<f64>::new();
        let mut sample_rate: f64 = 0.0;
        let mut old_time = 0.0;
        //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
        let mut velocity: Array2<f64> = Array2::<f64>::zeros((3, 3));
        velocity.slice_mut(s![.., 0usize]).fill(f64::MAX);
        velocity.slice_mut(s![.., 2usize]).fill(f64::MIN);
        // vel mag = [min,mean,max]
        let mut velocity_mag: Array1<f64> = Array1::<f64>::zeros(3);
        velocity_mag[0] = f64::MAX;
        velocity_mag[2] = f64::MIN;
        let mut mean_counter: usize = 0;
        for (file_id, filename) in filenames.iter().enumerate() {
            print_debug!("Creating group: {timestep}");
            let group = output_file
                .create_group(&format!("timestep {}", file_id))
                .unwrap_or_else(|_| panic!("Can not create group timestep {}", file_id));
            // Try to get data from filename
            let current_step: i64 = re
                .captures(filename)
                .unwrap_or_else(|| panic!("Unable to match filename {filename} with filter {filter}"))
                .get(1)
                .unwrap_or_else(|| panic!("Unable to find first match of filename {filename} with filter {filter}"))
                .as_str()
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Unable to parse string to i64 from match filename {filename} with filter {filter}."));
            let current_time = current_step as f64 * timestep;
            if file_id == 0 {
                time[0] = current_time;
            }
            // TODO this can be done better
            if current_time < time[1] {
                panic!("Vtk files are not sorted into the correct order!");
            }
            time_array.push(current_time);
            let builder = group.new_dataset::<f64>();
            builder
                .create("time")
                .expect("Unable to create dataset time")
                .write_scalar(&current_time)
                .expect("Unable to write time to dataset");
            print_debug!("Parsing VTK data");
            if self.is_legacy {
            } else {
                print_debug!("Parsing particle IDs");
                let particle_ids = if let Some(ref id_field) = self.id_field_name {
                    xmltools::get_field::<u64>(filename, id_field, &self.file_type)
                } else {
                    panic!("No ID field specified");
                };
                let sort_list = make_sortlist(&particle_ids);
                let particle_ids = sort_by_array(particle_ids, &sort_list);
                let max_particle = *particle_ids.iter().max().unwrap();
                if max_particle > nparticles {
                    nparticles = max_particle
                }
                let builder = make_dataset_builder!(group);
                builder
                    .with_data(&particle_ids)
                    .create("id")
                    .unwrap_or_else(|_| {
                        panic!("Unable to create dataset \"ID\" in file {}", filename)
                    });
                print_debug!("Parsing particle radius");
                let particle_radius = if let Some(ref radius_field) = self.radius_field_name {
                    sort_by_array(
                        xmltools::get_field::<f64>(filename, radius_field, &self.file_type),
                        &sort_list,
                    )
                } else if let Some(ref diameter_field) = self.diameter_field_name {
                    sort_by_array(
                        xmltools::get_field::<f64>(filename, diameter_field, &self.file_type)
                            .iter()
                            .map(|x| x * 0.5)
                            .collect::<Vec<f64>>(),
                        &sort_list,
                    )
                } else {
                    panic!("No radius or diameter field specified");
                };
                let builder = make_dataset_builder!(group);
                builder
                    .with_data(&particle_radius)
                    .create("radius")
                    .unwrap_or_else(|_| {
                        panic!("Unable to create dataset \"radius\" in file {}", filename)
                    });
                print_debug!("Creating ppclouds dataset");
                let ppclouds = Array1::<u64>::ones(particle_radius.len());
                let builder = make_dataset_builder!(group);
                builder
                    .with_data(&ppclouds)
                    .create("ppcloud")
                    .unwrap_or_else(|_| {
                        panic!("Unable to create dataset \"ppcloud\" in file {}", filename)
                    });
                print_debug!("Parsing particle type");
                let particle_type = if let Some(ref type_field) = self.type_field_name {
                    sort_by_array(
                        xmltools::get_field::<u64>(filename, type_field, &self.file_type),
                        &sort_list,
                    )
                } else {
                    print_warning!("No type field specified, defaulting to 1");
                    Array1::<u64>::ones(particle_radius.len()).into_raw_vec()
                };
                let builder = make_dataset_builder!(group);
                builder
                    .with_data(&particle_type)
                    .create("particletype")
                    .unwrap_or_else(|_| {
                        panic!(
                            "Unable to create dataset \"particletype\" in file {}",
                            filename
                        )
                    });
                print_debug!("Parsing particle velocity");
                let particle_velocity = if let Some(ref velocity_field) = self.velocity_field_name {
                    sort_by_array(
                        xmltools::get_field::<f64>(filename, velocity_field, &self.file_type),
                        &sort_list,
                    )
                } else {
                    panic!("No velocity field specified");
                };
                let particle_velocity =
                    Array::from_shape_vec((particle_velocity.len() / 3, 3), particle_velocity)
                        .unwrap();
                print_debug!("Extracting statistical velocity information");
                for vel in particle_velocity.axis_iter(Axis(0)) {
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
                let builder = make_dataset_builder!(group);
                builder
                    .with_data(&particle_velocity)
                    .create("velocity")
                    .unwrap_or_else(|_| {
                        panic!("Unable to create dataset \"velocity\" in file {}", filename)
                    });
                print_debug!("  Creating position dataset");
                let particle_positions = sort_by_array(
                    xmltools::get_positions::<f64>(filename, &self.file_type),
                    &sort_list,
                );
                let particle_positions =
                    Array::from_shape_vec((particle_positions.len() / 3, 3), particle_positions)
                        .unwrap();
                print_debug!("New: {:?}", particle_positions);
                for pos in particle_positions.axis_iter(Axis(0)) {
                    for i in 0..3 {
                        if pos[i] < dimensions[[0, i]] {
                            dimensions[[0, i]] = pos[i];
                        } else if pos[i] > dimensions[[1, i]] {
                            dimensions[[1, i]] = pos[i];
                        }
                    }
                }
                let builder = make_dataset_builder!(group);
                builder
                    .with_data(&particle_positions)
                    .create("position")
                    .unwrap_or_else(|_| {
                        panic!("Unable to create dataset \"position\" in file {}", filename)
                    });
                mean_counter += particle_ids.len();
                sample_rate = current_time - old_time;
                old_time = current_time;
                if file_id % 10 == 0 {
                    progress_bar.inc(10);
                    check_signals!();
                }
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

            output_file
                .new_attr_builder()
                .with_data(&dimensions)
                .create("dimensions")
                .unwrap();
            output_file
                .new_attr::<u64>()
                .create("particle number")
                .unwrap()
                .write_scalar(&nparticles)
                .unwrap();
            output_file
                .new_attr::<u64>()
                .create("timesteps")
                .unwrap()
                .write_scalar(&timesteps)
                .unwrap();
            output_file
                .new_attr::<u64>()
                .create("sample rate")
                .unwrap()
                .write_scalar(&sample_rate)
                .unwrap();
            output_file
                .new_attr_builder()
                .with_data(&time)
                .create("time")
                .unwrap();
            output_file
                .new_attr_builder()
                .with_data(&velocity)
                .create("velocity")
                .unwrap();
            output_file
                .new_attr_builder()
                .with_data(&velocity_mag)
                .create("velocity magnitude")
                .unwrap();
            output_file
                .new_dataset_builder()
                .with_data(&time_array)
                .create("time array")
                .unwrap();

            progress_bar.finish();
            print_debug!("Finished with conversion from vtk to HDF5 ");
        }
    }
}

// I think this is all that can be sanely done for particles?
#[derive(Default)]
pub enum VTKType {
    #[default]
    PolyData,
    UnstructuredGrid,
}
