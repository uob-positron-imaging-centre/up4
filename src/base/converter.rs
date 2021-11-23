//! This file provides functionalities to convert data to HDF5 file format
//!
//!
use crate::{print_debug,print_warning,check_signals};
use regex::Regex;
use ndarray;
use hdf5::File;
use indicatif::{ProgressBar,ProgressStyle};
use ndarray_stats;

mod vtktools;

/// Convert a vtk file into a HDF5 file
pub fn vtk(
    mut filenames: Vec<&str>,
    timestep: f64,
    outname: &str,
    filter: &str, // example r"vtk_(\d+).vtk"
){
    print_debug!("Sorting vector of filenames");
    filenames.sort();
    let re = Regex::new(filter).unwrap();
    let hdf5file = File::create(outname).unwrap();
    let mut step = 0;
    let bar = ProgressBar::new(filenames.len() as u64 );
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% {per_sec} ({eta})")
        .with_key("eta", |state| format!("Time left: {:.1}s", state.eta().as_secs_f64()))
        .with_key("per_sec", |state| format!("{:.1} steps/s", state.per_sec()))
        .progress_chars("#>-"));
    // Attributes
    let mut dimensions: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2,3));
    let mut nparticles: u64 = 0;
    let mut timesteps: u64 = 0;
    let mut time: f64 = 0.0;
    let mut sample_rate:f64 = 0.0;
    let mut velocity: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2,3));
    let mut velocity_mag: ndarray::Array1<f64>= ndarray::Array1::<f64>::zeros((3));


    for filename in filenames.iter(){
        print_debug!("Creating a new group \"timestep {}\"",step);
        let group = hdf5file.create_group(&format!("timestep {}",step))
                            .expect(&format!("Can not create group timestep {}",step));
        // Extracting data from filename
        let current_step:i64=re.captures(filename)
                        .expect(
                                &format!(
                                    "Unable to match filename {} with filter {}",
                                     filename, filter
                                 ))
                        .get(1)
                        .expect(
                                &format!(
                                    "Unable collect mfirst match  of filename {} with filter {}",
                                     filename, filter
                                 ))
                        .as_str()
                        .parse::<i64>()
                        .expect(
                                &format!(
                                    "Unable to parse string to i64 from match filename {} with filter {}",
                                     filename, filter
                                 ));
        let current_time = current_step as f64 * timestep;
        let builder = group.new_dataset_builder();
        builder.with_data(&[current_time])
                .create("time")
                .expect(
                    &format!("Unable to create dataset \"time\" in file {}", filename)
                );
        // VTK data reading
        print_debug!("Recieving data from VTKio and creating datasets");
        let particle_id = vtktools::get_field::<i64>(filename,"id");
        let builder = group.new_dataset_builder();
        builder.with_data(&particle_id)
                .create("id")
                .expect(
                    &format!("Unable to create dataset \"id\" in file {}", filename)
                );

        let particle_radius = vtktools::get_field::<f64>(filename,"radius");
        let builder = group.new_dataset_builder();
        builder.with_data(&particle_radius)
                .create("radius")
                .expect(
                    &format!("Unable to create dataset \"radius\" in file {}", filename)
                );
        let particle_type = vtktools::get_field::<i64>(filename,"type");
        let builder = group.new_dataset_builder();
        builder.with_data(&particle_type)
                .create("particleid")
                .expect(
                    &format!("Unable to create dataset \"particleid\" in file {}", filename)
                );
        let particle_velocity = vtktools::get_field::<f64>(filename,"v");
        let particle_velocity = ndarray::Array::from_shape_vec(
            (particle_velocity.len()/3,3),
            particle_velocity
        ).unwrap();
        let builder = group.new_dataset_builder();
        builder.with_data(&particle_velocity)
                .create("velocity")
                .expect(
                    &format!("Unable to create dataset \"velocity\" in file {}", filename)
                );
        let particle_positions = vtktools::get_positions::<f64>(filename);
        let particle_positions = ndarray::Array::from_shape_vec(
            (particle_positions.len()/3,3),
            particle_positions
        ).unwrap();
        let builder = group.new_dataset_builder();
        builder.with_data(&particle_positions)
                .create("position")
                .expect(
                    &format!("Unable to create dataset \"position\" in file {}", filename)
                );
        step+=1;
        bar.inc(1);
        check_signals!();
    }// end filename forloop
    bar.finish();
}// end vtk function



/*

*/
