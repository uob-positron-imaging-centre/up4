//! This file provides functionalities to convert data to HDF5 file format
//!
//!
use crate::{print_debug,check_signals};
use regex::Regex;
use ndarray;
use hdf5::File;
use indicatif::{ProgressBar,ProgressStyle};

mod vtktools;

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
){
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
    print_debug!("Constructing data arrays for attributes!");
    let mut dimensions: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((2,3)); // [min:[x,y,z],max:[x,y,z]]
    dimensions.slice_mut(ndarray::s![0,..]).fill(f64::MAX);
    dimensions.slice_mut(ndarray::s![1,..]).fill(f64::MIN);
    let mut nparticles: u64 = 0;
    let timesteps: usize = filenames.len();
    let mut time:  ndarray::Array1<f64>= ndarray::Array1::<f64>::zeros(2);
    let mut sample_rate:f64 = 0.0;
    let mut  old_time = 0.0;
    //velocity: [x:[min, mean, max],y:[min,mean,max],z:[min,mean,max]]
    let mut velocity: ndarray::Array2<f64> = ndarray::Array2::<f64>::zeros((3,3));
    velocity.slice_mut(ndarray::s![..,0]).fill(f64::MAX);
    velocity.slice_mut(ndarray::s![..,2]).fill(f64::MIN);
    // vel mag = [min,mean,max]
    let mut velocity_mag: ndarray::Array1<f64>= ndarray::Array1::<f64>::zeros(3);
    velocity_mag[0]=f64::MAX;
    velocity_mag[2]=f64::MIN;

    let mut mean_counter: usize = 0;
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
        if step == 0{
            time[0]= current_time;
        }
        if current_time < time[1]{
            panic!("Vtk files are not sorted into the correct order!");
        }
        time[1]= current_time;
        let builder = group.new_dataset::<f64>().create("time").unwrap().write_scalar(&current_time).unwrap();
        // VTK data reading
        print_debug!("Recieving data from VTKio and creating datasets");
        let particle_id = vtktools::get_field::<u64>(filename,"id");
        let max_particle = particle_id.iter().max().unwrap().clone();
        if max_particle > nparticles{ nparticles =  max_particle}
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
        let ppclouds = ndarray::Array1::<u64>::ones(particle_radius.len());
        let builder = group.new_dataset_builder();
        builder.with_data(&ppclouds)
                        .create("ppcloud")
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
        print_debug!("Extracting statistical velocity information");
        for vel in particle_velocity.axis_iter(ndarray::Axis(0)){
            for i in (0..3){
                print_debug!("  i: {}",i);
                if vel[i] < velocity[[i,0]]{
                    velocity[[i,0]] = vel[i];
                }else if vel[i] > velocity[[i,2]]{
                    velocity[[i,2]] = vel[i];
                }
                velocity[[i,1]] += vel[i];
            }
            let vel_mag = vel.map(|v| v*v).sum().sqrt();
            if vel_mag > velocity_mag[0]{
                velocity_mag[0]=vel_mag;
            } else if vel_mag < velocity_mag[2]{
                velocity_mag[2]=vel_mag;
            }
            velocity_mag[1] += vel_mag;
        }
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
        for pos in particle_positions.axis_iter(ndarray::Axis(0)){
            for i in 0..3 {
                if pos[i] > dimensions[[0,i]]{
                     dimensions[[0,i]] = pos[i];
                } else if pos[i] < dimensions[[1,i]]{
                    dimensions[[0,i]] = pos[i];
                }
            }
        }
        let builder = group.new_dataset_builder();
        builder.with_data(&particle_positions)
                .create("position")
                .expect(
                    &format!("Unable to create dataset \"position\" in file {}", filename)
                );
        step+=1;
        mean_counter += particle_id.len();
        sample_rate = current_time-old_time;
        old_time = current_time;
        bar.inc(1);
        check_signals!();
    }// end filename forloop
    velocity_mag[1] /= mean_counter as f64;
    velocity[[0,1]] /= mean_counter as f64;
    velocity[[1,1]] /= mean_counter as f64;
    velocity[[2,1]] /= mean_counter as f64;
    print_debug!("Mean Velocity: \nmagnitude:  {}\nx:  {}\ny:  {}\nz:  {}\n",
        velocity_mag[1],velocity[[0,1]],velocity[[1,1]],velocity[[2,1]]
    );
    print_debug!("Dimensions: {:?}",dimensions);
    hdf5file.new_attr_builder().with_data(&dimensions).create("dimensions").unwrap();
    hdf5file.new_attr::<u64>().create("particle number").unwrap().write_scalar(&nparticles).unwrap();
    hdf5file.new_attr::<u64>().create("timesteps").unwrap().write_scalar(&timesteps).unwrap();
    hdf5file.new_attr::<u64>().create("sample rate").unwrap().write_scalar(&sample_rate).unwrap();
    hdf5file.new_attr_builder().with_data(&time).create("time").unwrap();
    hdf5file.new_attr_builder().with_data(&velocity).create("velocity").unwrap();
    hdf5file.new_attr_builder().with_data(&velocity_mag).create("velocity magnitude").unwrap();

    bar.finish();
    print_debug!("Finished with conversion from vtk to HDF5 ");
}// end vtk function
