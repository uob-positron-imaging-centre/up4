extern crate ndarray;
extern crate ndarray_linalg;
extern crate numpy;


use core::panic;use ndarray::prelude::*;

/// [Vectorfield Function]:
/// Calculate the velocity Vectorfiels of your System
/// Inputs:
///     Filename:   &str,               Filename to the HDF5 file
///     Cells:      ndarray::Array1     Array Containing the # cells in x,y,z dimension
///     min_time    f64,                Time when averaging starts
///     max_time    f64,                Time when averaging ends
///     dimensions: ndarray::Array2,    region of interest
///     norm:       bool,               normalise size of arrow
///     radius:     ndarray::Array1,    min and max value for the particle radius
///     particle_id ndarray::Array1,    min and max value for the ids to look at
///
/// Outputs:
///     vx_array    Array2,             The vectordata in x direction
///     vy_array    Array2,             The vectordata in y direction
///     vz_array    Array2,             The vectordata in z direction
///     sx          Array2,             Mesh data for plotting in x Dimension
///     sy          Array2,             Mesh data for plotting in y Dimension
///
///’’’
///vx,vy,vz = vectorfield(
///           "test.hdf5".
///           array![10,10,10],
///           0.0,
///           1.0
///           array![[0,1],[0,1],[0,1]],
///           True,
///           array![-1,-1], //all Particles
///           array![-1,-1], //all Particles
///         )
///'''
pub fn vectorfield(
    filename: &str,          //filename of hdf5 file
    cells: Array1<f64>,      //number of cells to store vec-data
    min_time: f64,           //where to start the averaging
    max_time: f64,           //where to end the averaging
    dimensions: Array2<f64>, // Region where to look at, rest ignored
    norm: bool,              //normalise the size of the vectors
    radius: Array1<f64>,     // include a radius
    particle_id: Array1<i64>,
    axis: usize,
) -> (
    Array2<f64>,
    Array2<f64>,
    Array2<f64>,
    Array2<f64>,
) {
    // Opening hdf5 file
    let file = hdf5::File::open(&filename).expect(&format!(
        "Can not open HDF5 file {:?}",
        &filename
    ));
    //read the number of timesteps inside this hdf5file
    let timesteps: u64 = timesteps(&file);
    //Extracting the min and max dimensions of the simulation
    let array = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let mut min_array = array.slice(s![0, ..]).to_owned();
    let mut  max_array = array.slice(s![1, ..]).to_owned();

    //let cells_int =
    //before going through timestep, implement:
    // dimension check?

    if dimensions.slice(s![0, ..]).to_owned()[0usize] > min_array[0usize]  {
        min_array[0usize] = dimensions.slice(s![0, ..]).to_owned()[0usize]
    }
    if dimensions.slice(s![0, ..]).to_owned()[1usize] > min_array[1usize]  {
        min_array[1usize] = dimensions.slice(s![0, ..]).to_owned()[1usize]
    }
    if dimensions.slice(s![0, ..]).to_owned()[2usize] > min_array[2usize]  {
        min_array[2usize] = dimensions.slice(s![0, ..]).to_owned()[2usize]
    }

    if dimensions.slice(s![1, ..]).to_owned()[0usize] < max_array[0usize]  {
        max_array[0usize] = dimensions.slice(s![1, ..]).to_owned()[0usize]
    }
    if dimensions.slice(s![1, ..]).to_owned()[1usize] < max_array[1usize]  {
        max_array[1usize] = dimensions.slice(s![1, ..]).to_owned()[1usize]
    }
    if dimensions.slice(s![1, ..]).to_owned()[2usize] < max_array[2usize]  {
        max_array[2usize] = dimensions.slice(s![1, ..]).to_owned()[2usize]
    }

    let cell_size: Array1<f64> = array![
        (max_array[0usize] - min_array[0usize]) / (cells[0usize]),
        (max_array[1usize] - min_array[1usize]) / (cells[1usize]),
        (max_array[2usize] - min_array[2usize]) / (cells[2usize])
    ];

    //initiate needed 2d arrays:
    let mut v_x_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut v_z_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    //array to count how many particles found per cell
    let mut n_x_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut n_z_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    // find the two axis indizes which we want to "see"
    let mut first_axis = 4;
    let mut sec_axis = 4;
    for x in 0..3{
        if x == axis{continue};
        if first_axis == 4{ first_axis = x;
        }else if sec_axis ==4{
            sec_axis =x ;
        }else{
            panic!(&format!(
                "variable axis in vectorfield must be between 0 and 2 ! Currently it is {:?}",
                axis,
            ))
        }
    }
    //println!("{:?}, {:?}",first_axis,sec_axis);
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).expect(&format!(
                "Could not find group {:?} in file {:?}",
                &name,
                &filename
            ));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ))[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        if current_time > max_time {
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"position\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let velocities = group
            .dataset("velocity")
            .expect(&format!(
                "Can not find dataset \"velocity\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"velocity\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particle_ids = group
            .dataset("particleid")
            .expect(&format!(
                "Can not find dataset \"particleid\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"particleid\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let rad_array = group
            .dataset("radius")
            .expect(&format!(
                "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"dimensions\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {

            //check if this particle is fitting the criteria
            if !check_id(particle_ids[particle] as usize, &particle_id) {
                println!("skipping because of ID");
                continue;
            }
            if !check_radius(rad_array[particle] as f64, &radius) {
                println!("Skipping cause of radius");
                continue;
            }
            let position = positions.slice(s![particle, ..]).to_owned();
            let velocity = velocities.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[first_axis] - min_array[first_axis];
            let z: f64 = position[sec_axis] - min_array[sec_axis];
            let x_abs: f64 = position[first_axis];
            let z_abs: f64 = position[sec_axis];

            //velocities
            let vx: f64 = velocity[first_axis];
            let vz: f64 = velocity[sec_axis];

            let pos_abs = position;

            if     pos_abs[0] < dimensions[[0usize, 0]]
                || pos_abs[0] > dimensions[[1usize, 0]]
                || pos_abs[1] < dimensions[[0usize, 1]]
                || pos_abs[1] > dimensions[[1usize, 1]]
                || pos_abs[2] < dimensions[[0usize, 2]]
                || pos_abs[2] > dimensions[[1usize, 2]]
            {
                // the particle is out of the field of view
                //println!("Skipping a particle as it is out of system");
                continue;
            }
            // find the cell indice where particle is right now

            let i: usize = (x / cell_size[first_axis]).floor() as usize;
            let k: usize = (z / cell_size[sec_axis]).floor() as usize;
            // check if indexes are higher then maximum
            if i >= cells[first_axis] as usize
                || k >= cells[sec_axis] as usize
            {
                continue;
            }


            v_x_grid[[k, i]] = v_x_grid[[k, i]] + vx;
            v_z_grid[[k, i]] = v_z_grid[[k, i]] + vz;

            n_x_grid[[k, i]] = n_x_grid[[k, i]] + 1.0;
            n_z_grid[[k, i]] = n_z_grid[[k, i]] + 1.0;
        }

    }

    v_x_grid = v_x_grid / &n_x_grid;
    v_z_grid = v_z_grid / &n_z_grid;
    let (sx, sy) = meshgrid(
        Array::linspace(
            min_array[first_axis],
            max_array[first_axis],
            cells[0usize] as usize,
        ),
        Array::linspace(
            min_array[sec_axis],
            max_array[sec_axis],
            cells[2usize] as usize,
        ),
    );
    if norm {
        let norm_arr = norm_two(&v_x_grid, &v_z_grid).to_owned();
        v_x_grid = v_x_grid / &norm_arr;
        v_z_grid = v_z_grid / &norm_arr;
    }
    file.close();

    (v_x_grid, v_z_grid, sx, sy)
}

pub fn occupancy_plot1d(
    filename: &str,
    radius: Array1<f64>,
    particle_id: Array1<i64>,
    clouds: bool,
    axis: usize,
    norm: bool,
    min_time: f64,
    cells: f64,
) -> (Array1<f64>, Array1<f64>) {
    /*
    function to calculate the time averaged occupancy plot of a particle system


     */
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);

    let array_temp = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let min_array: Array1<f64> = array_temp.slice(s![0, ..]).to_owned();
    let max_array: Array1<f64> = array_temp.slice(s![1, ..]).to_owned();
    let cell_size: Array1<f64> = (&max_array - &min_array) / cells;
    let mut occu: Array1<f64> = Array1::zeros(cells as usize);
    let array: Array1<f64> =
        Array1::linspace(0.0, &max_array[axis] - &min_array[axis], cells as usize);
    let dt = get_dt(&file);
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).expect(&format!("Can not open timestep {:?} in file {:?}",timestep,filename));
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        let positions = group
            .dataset("position")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let velocities = group
            .dataset("velocity")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let particle_ids = group
            .dataset("particleid")
            .expect("error")
            .read_1d::<f64>()
            .unwrap();
        let rad_array = group
            .dataset("radius")
            .expect("error")
            .read_1d::<f64>()
            .unwrap();
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {
            /**if !check_id(particle_ids[particle] as usize, &particle_id) {
                continue;
            }
            if !check_radius(rad_array[particle] as f64, &radius) {
                continue;
            }
            **/
            let position = positions.slice(s![particle, ..]).to_owned();
            let velocity = velocities.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[axis] - min_array[axis];
            let vel: f64 = velocity[axis].abs();
            //find current cell location
            let cell_id = find_closest(&array, x);
            //calculate the time this particle spent in the cell
            let mut time_spent = cell_size[axis] / vel;
            if time_spent > dt {
                time_spent = dt;
            }
            occu[cell_id] = occu[cell_id] + 1.;// time_spent;
        }
    }
    if norm {
        let mut xmax = 0;
        for x in 0..occu.len() {
            if occu[x] > occu[xmax] {
                xmax = x;
            }
        }
        let max_num = occu[xmax];
        for x in 0..occu.len() {
            occu[x] = occu[x] / max_num;
        }
    }
    file.close();
    (occu, array)
}

pub fn velocity_plot1d(
    filename: &str,
    radius: Array1<f64>,
    particle_id: Array1<i64>,
    clouds: bool,
    axis: usize,
    norm: bool,
    min_time: f64,
    cells: f64,
) -> (Array1<f64>, Array1<f64>) {
    /*
    function to calculate the time averaged occupancy plot of a particle system


     */
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);

    let array_temp = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let min_array: Array1<f64> = array_temp.slice(s![0, ..]).to_owned();
    let max_array: Array1<f64> = array_temp.slice(s![1, ..]).to_owned();
    let cell_size: Array1<f64> = (&max_array - &min_array) / cells;
    let mut norm_array: Array1<f64> = Array1::zeros(cells as usize);
    let mut vel_array: Array1<f64> = Array1::zeros(cells as usize);
    let array: Array1<f64> =
        Array1::linspace(0.0, &max_array[axis] - &min_array[axis], cells as usize);
    let dt = get_dt(&file);
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).expect(&format!("Can not open timestep {:?} in file {:?}",timestep,filename));
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        let positions = group
            .dataset("position")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let velocities = group
            .dataset("velocity")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let particle_ids = group
            .dataset("particleid")
            .expect("error")
            .read_1d::<f64>()
            .unwrap();
        let rad_array = group
            .dataset("radius")
            .expect("error")
            .read_1d::<f64>()
            .unwrap();
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {
            /**if !check_id(particle_ids[particle] as usize, &particle_id) {
                continue;
            }
            if !check_radius(rad_array[particle] as f64, &radius) {
                continue;
            }
            **/
            let position = positions.slice(s![particle, ..]).to_owned();
            let velocity = velocities.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[axis] - min_array[axis];
            //find current cell location
            let cell_id = find_closest(&array, x);
            //calculate the time this particle spent in the cell
            let vel = velocity[0];
            if vel > 0. {continue}

            norm_array[cell_id] = norm_array[cell_id] + 1.0;//time_spent;
            vel_array[cell_id] = vel_array[cell_id] -vel;//norm_l2(&velocity);//(velocity[1]*velocity[1]+velocity[0]*velocity[0]).sqrt();// norm_l2(&velocity);
        }
    }
    if norm {
        vel_array= vel_array/norm_array;
    }
    file.close();
    //let(x,y )= derivative(array,vel_array,2);
    //(y, x)
    (vel_array,array)
}

pub fn mean_velocity(filename: &str, min_time: f64) -> f64 {
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);
    let mut mean_velocity = 0.0;
    let mut num_counts = 0.0;
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        let velocities = group
            .dataset("velocity")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for vel in velocities.outer_iter(){
            mean_velocity += (vel[0]*vel[0] + vel[1]*vel[1] +vel[2]*vel[2]).sqrt();
            num_counts += 1.0;
        }

    }
    mean_velocity /= num_counts;
    file.close();
    mean_velocity
}

pub fn mean_angular_velocity(filename: &str, min_time: f64) -> f64 {
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);
    let mut mean_velocity = 0.0;
    let mut num_counts = 0.0;
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        let velocities = group
            .dataset("angular_velocity")
            .expect("This dataset does not contain angular velocity data")
            .read_2d::<f64>()
            .unwrap();
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for vel in velocities.outer_iter(){
            mean_velocity += (vel[0]*vel[0] + vel[1]*vel[1] +vel[2]*vel[2]).sqrt();
            num_counts += 1.0;
        }

    }
    mean_velocity /= num_counts;
    file.close();
    mean_velocity
}
/// [Velocity Distribution Function]:
/// Calculate teh velocity distribution in your system

pub fn velocity_distribution(
    filename: &str, // which contains vel & pos data
    mut bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
) -> (Vec<Vec<i64>>, Array1<f64>, Array1<f64>) {
    /*
    Description:
    This function to calculates the velocity distribution of a particle system.
    The function takes into account all velocities defined between the user defined time period:
        min_time
        max_time

    !n.b! This distribution has not been normalised.

    Internals:
    Particle velocity is normalised ('.norm()' -> sqrt(vx**2 + vy**2 + vz**2)) and placed into bins
    Particle particle_ids can be matched with the bins to find what particles are in each bin.

     */
    // Opening the file and defining the iterable (time-steps) - giving the num of timesteps
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);

    // finding the dimensions of the system,
    let array_temp = file
        .dataset("dimensions")
        .unwrap()
        .read_2d::<f64>()
        .expect("Dimensions can not be read from the HDF5");

    // arbitrary values set for variables - will be used later
    let mut total_no_of_particles: i64 = 0;
    let mut global_v_abs_min: f64 = f64::MAX;
    let mut global_v_abs_max: f64 = f64::MIN;

    // Part 1
    // for loop used to find the max and min vel's for the system
    // this is used define the scale of the axis, which is used in the following loop.
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file
            .group(&name)
            .expect(&format!("Could not open group {:?}", &name));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Couldn't open dataset time in dataset {:?}",
                &name
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step:{:?}, check the raw data",
                &timestep
            ))[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        };
        if current_time > max_time {
            continue;
        };

        // pulling the data from the timestep's group
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Expected Dataset 'Position':{:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'Position':{:?}, dataset could not be read",
                &timestep
            ));

        let velocities: Array2<f64> = group
            .dataset("velocity")
            .expect(&format!(
                "Expected Dataset 'Velocity':{:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity':{:?}, dataset could not be read",
                &timestep
            ));

        // finding the total no of particles in the system
        total_no_of_particles = (positions.len() / 3 as usize) as i64;

        // creating the absolute velocity array based on the prior 2D array
        // norm -> sqrt(vx**2 + vy**2 + vz**2)
        // collect -> forms the result into an array
        let mut v_abs: Array1<f64> = Array1::zeros(velocities.len());

        let mut v_abs: Array1<f64> = velocities
            .axis_iter(ndarray::Axis(0))
            .map(|xyz| norm_l2(&xyz.to_owned()))
            .collect::<Array1<f64>>();

        // to find max and min within the vel group
        let (min_abs_vel, max_abs_vel) = minmax(&v_abs);

        // Replacing the existing global value with the new local min & max,
        // if the min or max is lower/higher they are replaced
        if max_abs_vel > global_v_abs_max {
            global_v_abs_max = max_abs_vel;
        };

        if min_abs_vel < global_v_abs_min {
            global_v_abs_min = min_abs_vel;
        };
    }
    // define the default no of bins, which is 10* less than the no of particles used
    if bins == -1 as i64 {
        bins = (&total_no_of_particles / 10) as i64;
    };

    // Converting bins i64 to f64 to allow f/f division
    let bins = bins as f64;

    // allow the user to define the number of bins for the vel distribution
    let bin_scale = (global_v_abs_max - global_v_abs_min) / bins;

    // ensuring that 'bins' is back in int form
    let bins = bins.floor() as i64;

    // creating the num axis for the bins
    let num_axis_array: Array1<f64> = Array::range(global_v_abs_min, global_v_abs_max, bin_scale);

    // establish an empty array containing the vel distribution
    let mut vel_dist = ndarray::Array1::<f64>::zeros((bins as usize));

    // establish an empty nested array - for the particle particle_ids data [timestep[particle]]
    let mut party_id: Vec<Vec<i64>> = Vec::with_capacity(bins as usize);
    party_id.resize(*&bins as usize, Vec::new());
    // Part 2
    // Assigning the particles to their bin, open the loop back up to access the data
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();

        let group = file
            .group(&name)
            .expect(&format!("Could not open group {:?}", &name));

        let current_time = group
            .dataset("time")
            .expect(&format!("Couldn't open dataset: {:?}", &name))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step: {:?}, check the raw data",
                &timestep
            ))[0];

        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        };
        if current_time > max_time {
            continue;
        };
        // opening the data from the file
        let velocities: Array2<f64> = group
            .dataset("velocity")
            .expect(&format!(
                "Expected Dataset 'Velocity': {:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity': {:?}, dataset could not be read",
                &timestep
            ));
        let particle_id = group
            .dataset("particleid")
            .expect(&format!(
                "Expected Dataset 'particle_id': {:?}, failed to read",
                &name
            ))
            .read_1d::<i64>()
            .expect(&format!(
                "Error: Data within 'particle_id': {:?}, dataset could not be read",
                &timestep
            ));
        // assigning the data to arrays (and calculating the 'abs' value within)
        for particles in 0..&total_no_of_particles - 1 {
            let particle_ids = particle_id[particles as usize]; // particle_ids of particle0 = particle_id[0]
            let veldata = norm_l2(
                &velocities
                    .index_axis(ndarray::Axis(0), particles as usize)
                    .to_owned(),
            ); // Velocity data
            let index: usize = (veldata / bin_scale).floor() as usize; // Calculating what bin the particle is in
            if index >= vel_dist.len() {
                continue;
            }
            // assigning the vel to the correct bin
            vel_dist[index] = vel_dist[index] + 1.0; // +1, to count in vel_dist array, building the hist plot
                                                     // Assign the particle id to that bin
                                                     // this will be useful when trying to look at the free surface velocity
            party_id[index].push(particle_ids);
        }
        // Closing the loop for the specific time-step
    }
    // closing the HDF5 file
    file.close();
    // sending the data back to Python
    (party_id, vel_dist, num_axis_array)
}

pub fn force_distribution(
    filename: &str, // which contains vel & pos data
    mut bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
) -> (Vec<Vec<i64>>, Array1<f64>, Array1<f64>) {
    /*
    Description:
    This function to calculates the velocity distribution of a particle system.
    The function takes into account all velocities defined between the user defined time period:
        min_time
        max_time

    !n.b! This distribution has not been normalised.

    Internals:
    Particle velocity is normalised ('.norm()' -> sqrt(vx**2 + vy**2 + vz**2)) and placed into bins
    Particle particle_ids can be matched with the bins to find what particles are in each bin.

     */
    // Opening the file and defining the iterable (time-steps) - giving the num of timesteps
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);

    // finding the dimensions of the system,
    let array_temp = file
        .dataset("dimensions")
        .unwrap()
        .read_2d::<f64>()
        .expect("Dimensions can not be read from the HDF5");

    // arbitrary values set for variables - will be used later
    let mut total_no_of_particles: i64 = 0;
    let mut global_f_abs_min: f64 = f64::MAX;
    let mut global_f_abs_max: f64 = f64::MIN;

    // Part 1
    // for loop used to find the max and min vel's for the system
    // this is used define the scale of the axis, which is used in the following loop.
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file
            .group(&name)
            .expect(&format!("Could not open group {:?}", &name));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Couldn't open dataset time in dataset {:?}",
                &name
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step:{:?}, check the raw data",
                &timestep
            ))[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        };
        if current_time > max_time {
            continue;
        };

        // pulling the data from the timestep's group
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Expected Dataset 'Position':{:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'Position':{:?}, dataset could not be read",
                &timestep
            ));

        let forces: Array2<f64> = group
            .dataset("force")
            .expect(&format!(
                "Expected Dataset 'Velocity':{:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity':{:?}, dataset could not be read",
                &timestep
            ));

        // finding the total no of particles in the system
        total_no_of_particles = (positions.len() / 3 as usize) as i64;

        // creating the absolute velocity array based on the prior 2D array
        // norm -> sqrt(vx**2 + vy**2 + vz**2)
        // collect -> forms the result into an array


        let f_abs: Array1<f64> = forces
            .outer_iter()
            .map(|xyz| norm_l2(&xyz.to_owned()))
            .collect::<Array1<f64>>();

        // to find max and min within the force group
        let (min_abs_f, max_abs_f) = minmax(&f_abs);

        // Replacing the existing global value with the new local min & max,
        // if the min or max is lower/higher they are replaced
        if max_abs_f > global_f_abs_max {
            global_f_abs_max = max_abs_f;
        };

        if min_abs_f < global_f_abs_min {
            global_f_abs_min = min_abs_f;
        };
    }
    // define the default no of bins, which is 10* less than the no of particles used
    if bins == -1 as i64 {
        bins = (&total_no_of_particles / 10) as i64;
    };

    // Converting bins i64 to f64 to allow f/f division
    let bins = bins as f64;

    // allow the user to define the number of bins for the vel distribution
    let bin_scale = (global_f_abs_max - global_f_abs_min) / bins;

    // ensuring that 'bins' is back in int form
    let bins = bins.floor() as i64;

    // creating the num axis for the bins
    let num_axis_array: Array1<f64> = Array::range(global_f_abs_min, global_f_abs_max, bin_scale);

    // establish an empty array containing the vel distribution
    let mut force_dist = ndarray::Array1::<f64>::zeros((bins as usize));

    // establish an empty nested array - for the particle particle_ids data [timestep[particle]]
    let mut party_id: Vec<Vec<i64>> = Vec::with_capacity(bins as usize);
    party_id.resize(*&bins as usize, Vec::new());
    // Part 2
    // Assigning the particles to their bin, open the loop back up to access the data
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();

        let group = file
            .group(&name)
            .expect(&format!("Could not open group {:?}", &name));

        let current_time = group
            .dataset("time")
            .expect(&format!("Couldn't open dataset: {:?}", &name))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step: {:?}, check the raw data",
                &timestep
            ))[0];

        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        };
        if current_time > max_time {
            continue;
        };
        // opening the data from the file
        let forces: Array2<f64> = group
            .dataset("force")
            .expect(&format!(
                "Expected Dataset 'Velocity': {:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity': {:?}, dataset could not be read",
                &timestep
            ));
        let particle_id = group
            .dataset("particleid")
            .expect(&format!(
                "Expected Dataset 'particle_id': {:?}, failed to read",
                &name
            ))
            .read_1d::<i64>()
            .expect(&format!(
                "Error: Data within 'particle_id': {:?}, dataset could not be read",
                &timestep
            ));
        // assigning the data to arrays (and calculating the 'abs' value within)
        for particles in 0..&total_no_of_particles - 1 {
            let particle_ids = particle_id[particles as usize]; // particle_ids of particle0 = particle_id[0]
            let forcedata = norm_l2(
                &forces
                    .index_axis(ndarray::Axis(0), particles as usize)
                    .to_owned(),
            ); // Velocity data
            let index: usize = (forcedata / bin_scale).floor() as usize; // Calculating what bin the particle is in
            if index >= force_dist.len() {
                continue;
            }
            // assigning the vel to the correct bin
            force_dist[index] = force_dist[index] + 1.0; // +1, to count in vel_dist array, building the hist plot
                                                     // Assign the particle id to that bin
                                                     // this will be useful when trying to look at the free surface velocity
            party_id[index].push(particle_ids);
        }
        // Closing the loop for the specific time-step
    }
    // closing the HDF5 file
    file.close();
    // sending the data back to Python
    (party_id, force_dist, num_axis_array)
}

/// [Velocity Distribution Function]:
/// Calculate teh velocity distribution in your system

pub fn rotational_velocity_distribution(
    filename: &str, // which contains vel & pos data
    mut bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
    rot_speed: f64,  // rotation of the drum in rpm
    drum_center: Array1<f64>, // center of the drum
    timestep: usize,
) -> ( Vec<Vec<i64>>, Array1<f64>, Array1<f64>) {
    /*
    Description:
    This function to calculates the velocity distribution of a particle system.
    The function takes into account all velocities defined between the user defined time period:
        min_time
        max_time

    !n.b! This distribution has not been normalised.

    Internals:
    Particle velocity is normalised ('.norm()' -> sqrt(vx**2 + vy**2 + vz**2)) and placed into bins
    Particle particle_ids can be matched with the bins to find what particles are in each bin.

     */
    // Opening the file and defining the iterable (time-steps) - giving the num of timesteps
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);

    // finding the dimensions of the system,
    let array_temp = file
        .dataset("dimensions")
        .unwrap()
        .read_2d::<f64>()
        .expect("Dimensions can not be read from the HDF5");

    // arbitrary values set for variables - will be used later
    let mut total_no_of_particles: i64 = 0;
    let mut global_v_abs_min: f64 = f64::MAX;
    let mut global_v_abs_max: f64 = f64::MIN;

    // Part 1
    // for loop used to find the max and min vel's for the system
    // this is used define the scale of the axis, which is used in the following loop.
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file
            .group(&name)
            .expect(&format!("Could not open group {:?}", &name));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Couldn't open dataset time in dataset {:?}",
                &name
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step:{:?}, check the raw data",
                &timestep
            ))[0];

        // pulling the data from the timestep's group
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Expected Dataset 'Position':{:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'Position':{:?}, dataset could not be read",
                &timestep
            ));

        let velocities: Array2<f64> = group
            .dataset("velocity")
            .expect(&format!(
                "Expected Dataset 'Velocity':{:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity':{:?}, dataset could not be read",
                &timestep
            ));

        // finding the total no of particles in the system
        total_no_of_particles = (positions.len() / 3 as usize) as i64;

        // creating the absolute velocity array based on the prior 2D array
        // norm -> sqrt(vx**2 + vy**2 + vz**2)
        // collect -> forms the result into an array

        // to find max and min within the vel group
        let (min_abs_vel, max_abs_vel) = minmax_rot(&velocities,&positions,rot_speed, &drum_center);

        // Replacing the existing global value with the new local min & max,
        // if the min or max is lower/higher they are replaced
        if max_abs_vel > global_v_abs_max {
            global_v_abs_max = max_abs_vel;
        };

        if min_abs_vel < global_v_abs_min {
            global_v_abs_min = min_abs_vel;
        };

    // define the default no of bins, which is 10* less than the no of particles used
    if bins == -1 as i64 {
        bins = (&total_no_of_particles / 10) as i64;
    };

    // Converting bins irray2<f64>64 to f64 to allow f/f division
    let bins = bins as f64;

    // allow the user to define the number of bins for the vel distribution
    let bin_scale = (global_v_abs_max - global_v_abs_min) / bins;

    // ensuring that 'bins' is back in int form
    let bins = bins.floor() as i64;

    // creating the num axis for the bins
    let num_axis_array: Array1<f64> = Array::range(global_v_abs_min, global_v_abs_max, bin_scale);

    // establish an empty array containing the vel distribution
    let mut vel_dist = ndarray::Array1::<f64>::zeros(bins as usize);

    // establish an empty nested array - for the particle particle_ids data [timestep[particle]]
    let mut party_id: Vec<Vec<i64>> = Vec::with_capacity(bins as usize);
    party_id.resize(*&bins as usize, Vec::new());

    // Part 2
    // Assigning the particles to their bin, open the loop back up to access the data

        let name: String = "timestep ".to_string() + &timestep.to_string();

        let group = file
            .group(&name)
            .expect(&format!("Could not open group {:?}", &name));

        let current_time = group
            .dataset("time")
            .expect(&format!("Couldn't open dataset: {:?}", &name))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step: {:?}, check the raw data",
                &timestep
            ))[0];


        // opening the data from the file
        let velocities: Array2<f64> = group
            .dataset("velocity")
            .expect(&format!(
                "Expected Dataset 'Velocity': {:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity': {:?}, dataset could not be read",
                &timestep
            ));
        let positions: Array2<f64> = group
                .dataset("position")
                .expect(&format!(
                    "Expected Dataset 'Position': {:?}, failed to read",
                    &name
                ))
                .read_2d::<f64>()
                .expect(&format!(
                    "Error: Data within 'Position': {:?}, dataset could not be read",
                    &timestep
                ));
        let particle_id = group
            .dataset("particleid")
            .expect(&format!(
                "Expected Dataset 'particle_id': {:?}, failed to read",
                &name
            ))
            .read_1d::<i64>()
            .expect(&format!(
                "Error: Data within 'particle_id': {:?}, dataset could not be read",
                &timestep
            ));
        // assigning the data to arrays (and calculating the 'abs' value within)

        for particles in 0..&total_no_of_particles - 1 {
            let particle_ids = particle_id[particles as usize]; // particle_ids of particle0 = particle_id[0]
            let vel = velocities.slice(s![particles as usize, ..]).to_owned();
            let position = positions.slice(s![particles as usize , ..]).to_owned();
            let distance = ((position[0usize]-drum_center[0]) * (position[0usize]-drum_center[0])
            //+ (position[1usize]-drum_center[1]) * (position[1usize]-drum_center[1])
            + (position[2usize]-drum_center[2]) * (position[2usize]-drum_center[2])).sqrt();
            let alpha = ((position[0usize]-drum_center[0])/(position[2usize]-drum_center[2])).atan();

            let rot_vel_at_dist = distance  * (2.0 * std::f64::consts::PI *rot_speed / 60.0);
            let rot_vec = array![alpha.cos()*rot_vel_at_dist,0.0,alpha.sin()*rot_vel_at_dist];
            // see notes at 13.feb 2021 (orange note book)

            //here we can do it better! save these values as array and return them
            //later
            let veldata = ((rot_vec[0]-vel[0])*(rot_vec[0]-vel[0])//(norm_l2(&vel) / distance-2.*std::f64::consts::PI *rot_speed / 60.0);
            +(rot_vec[1]-vel[1])*(rot_vec[1]-vel[1])
            +(rot_vec[2]-vel[2])*(rot_vec[2]-vel[2])
            ).sqrt();

            let index: usize = (veldata.abs() / bin_scale).floor() as usize; // Calculating what bin the particle is in
            if index >= vel_dist.len() {
            //   println!("{:?},{:?}", index,veldata);
                continue;
            }
            // assigning the vel to the correct bin
            vel_dist[index] = vel_dist[index] + 1.0; // +1, to count in vel_dist array, building the hist plot                                    // Assign the particle id to that bin
                                                     // this will be useful when trying to look at the free surface velocity
            party_id[index].push(particle_ids);
        }
        // Closing the loop for the specific time-step

    // closing the HDF5 file
    //file.close();
    // sending the data back to Python
    (party_id, vel_dist, num_axis_array)
}


pub fn mean_surface_velocity(
    filename: &str, // which contains vel & pos data
    mut bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
    rot_speed: f64,  // rotation of the drum in rpm
    drum_center: Array1<f64>, // center of the drum

)->f64{
    // Opening the file and defining the iterable (time-steps) - giving the num of timesteps
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);
    let mut mean_velocity = 0.0;
    let mut mean_velocity_bulk = 0.0;
    let mut num =0;
    let mut num_bulk = 0;

    for timestep in 0..timesteps-1{
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&"timestep 0".to_string()).expect(&format!("Can not open timestep {:?} in file {:?}",timestep,filename));

        let current_time = group
            .dataset("time")
            .expect(&format!("Couldn't open dataset: {:?}", &name))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step: {:?}, check the raw data",
                &timestep
            ))[0];


        // opening the data from the file
        let velocities: Array2<f64> = group
            .dataset("velocity")
            .expect(&format!(
                "Expected Dataset 'Velocity': {:?}, failed to read",
                &name
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Error: Data within 'velocity': {:?}, dataset could not be read",
                &timestep
            ));
        let positions: Array2<f64> = group
                .dataset("position")
                .expect(&format!(
                    "Expected Dataset 'Position': {:?}, failed to read",
                    &name
                ))
                .read_2d::<f64>()
                .expect(&format!(
                    "Error: Data within 'Position': {:?}, dataset could not be read",
                    &timestep
                ));
        let particle_ids = group
            .dataset("particleid")
            .expect(&format!(
                "Expected Dataset 'particle_id': {:?}, failed to read",
                &name
            ))
            .read_1d::<i64>()
            .expect(&format!(
                "Error: Data within 'particle_id': {:?}, dataset could not be read",
                &timestep
            ));
        let total_no_of_particles = (positions.len() / 3 as usize) as i64;
        let (party_id, vel_dist, num_axis_array) = rotational_velocity_distribution(
            filename, // which contains vel & pos data
            bins,  // bins can be defined by the user, but the default value is */10 the amount
            min_time,  // where to start the averaging
            max_time,  // where to end the averaging
            rot_speed,  // rotation of the drum in rpm
            drum_center.clone() , // center of the drum
            timestep as usize,
        );
        let mut max_diff = 0.0;
        let mut max_diff_id = 0;
        let mut diff= Array1::<f64>::zeros(vel_dist.len()-2);

        let mut smooth = Array1::<f64>::zeros(vel_dist.len()-2);
        for id in 0..vel_dist.len()-2{
             smooth[id] = vel_dist[id]+vel_dist[id+1]+vel_dist[id+2]/3.;
        }

        for id in 0..smooth.len()-1{
             diff[id] = smooth[id]-smooth[id+1];
             if diff[id] > max_diff{
                 max_diff = diff[id];
                 max_diff_id = id;
             }
        }
        let mut particles_to_use = Vec::new();
        for x in max_diff_id..party_id.len(){
            for id in party_id[x].iter(){
                particles_to_use.push(id);
            }
        }
        for particle in 0..total_no_of_particles{
            let particle_id = particle_ids[particle as usize];
            let mut flag:bool = false;
            for use_id in particles_to_use.iter(){
                if particle_id == use_id.to_owned().to_owned(){
                        flag = true;
                        break;
                }
                else{
                    flag = false;
                }
            }
            if !flag {
                mean_velocity_bulk += norm_l2(&velocities.slice(s![particle as usize, ..]).to_owned());
                num_bulk +=1;
                continue;
            }
            mean_velocity += norm_l2(&velocities.slice(s![particle as usize, ..]).to_owned());
            num +=1;
        }
    }


    mean_velocity /= num as f64;
    mean_velocity_bulk /= num_bulk as f64;
    println!("Surface: {}/nBulk: {}",mean_velocity,mean_velocity_bulk);
    file.close();
    mean_velocity
}

pub fn surface_polynom(
    filename: &str,
    axis: usize,
    timestep: u64,
    mut ntimesteps: u64,
    threshold: f64,
) -> (Array2<f64>, f64) {
    /*
    function to calculate the time averaged occupancy plot of a particle system
     */
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let array_temp = file
        .dataset("dimensions")
        .unwrap()
        .read_2d::<f64>()
        .unwrap();
    let min_array: Array1<f64> = array_temp.slice(s![0, ..]).to_owned();
    let max_array: Array1<f64> = array_temp.slice(s![1, ..]).to_owned();
    let dt = get_dt(&file);
    let cell_size = (&max_array[0usize] - &min_array[0usize]) / 200.0;
    let cells: Array1<f64> = (&max_array - &min_array) / cell_size;
    let mut image: Array2<f64> =
        ndarray::Array2::<f64>::zeros((cells[0usize] as usize, cells[2usize] as usize));
    // loop over n timesteps and generate the image
    // find the number of timesteps

    if ntimesteps == 0 {
        let timesteps = timesteps(&file);
        ntimesteps = timesteps - timestep - 1;
    }
    for t in 0..ntimesteps - 1 {
        let curstep = t + timestep;
        let name: String = "timestep ".to_string() + &curstep.to_string();
        let group = file.group(&name).unwrap();
        let positions = group
            .dataset("position")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {
            let position = positions.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[0usize] - min_array[0usize];
            let y: f64 = position[2usize] - min_array[2usize];
            let mut i: usize = (x / cell_size).floor() as usize;
            let mut j: usize = (y / cell_size).floor() as usize;
            if i >= cells[0usize] as usize {
                i = cells[0usize] as usize - 1;
            }
            if j >= cells[2usize] as usize {
                j = cells[2usize] as usize - 1;
            }
            //let k = (cells[2usize] as i64 - j - 1) as usize;
            //println!("{:?},{:?}",i,j);
            image[[i, j]] = image[[i, j]] + 1.0;
        }
    }

    let rows = image.nrows();
    let cols = image.ncols();
    //find the highest value of each coloumn

    for x in 0..rows {
        let mut max_y: usize = 0;
        for y in 0..cols {
            if image[[x, y]] > threshold {
                image[[x, max_y]] = 0.0;
                max_y = y;
                image[[x, y]] = 1.0;
                continue;
            }
            image[[x, y]] = 0.0;
        }
    }
    //return the current image
    (image, cell_size)
}
///Granular Temperature Calculator
/// This function creates a 1D Array containing the overall
/// Granular temperature of the system for each timestep
///
/// Input parameters:
///     Filename: Filename to the Hdf5 file Type: string
///
/// Outputs
///     PythonArray of size (1xN)
///
/// '''
/// let file = "test.hdf5"
/// let Array = granular_temperature(file)
/// '''

pub fn granular_temperature(filename: &str,min_time: f64) -> Array1<f64> {
    /*
    function to calculate the granular temperature of a system for a amount of timesteps

     */
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");

    // loop over all timesteps
    let timesteps: u64 = timesteps(&file);
    let mut result: Array1<f64> = Array1::<f64>::zeros(timesteps as usize);
    for t in 0..timesteps {
        let name: String = "timestep ".to_string() + &t.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group
            .dataset("time")
            .expect(&format!("Couldn't open dataset: {:?}", &name))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step: {:?}, check the raw data",
                &t
            ))[0];
        if current_time < min_time{continue}
        let velocity = group
            .dataset("velocity")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let particles = velocity.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        let mut sum_vel = 0.0;
        let mut sum_vel_sq = 0.0;

        for particle in 0..particles {
            let vel = velocity.slice(s![particle, ..]).to_owned();
            let abs_vel: f64 =
                (vel[0usize] * vel[0usize] + vel[1usize] * vel[1usize] + vel[2usize] * vel[2usize])
                    .sqrt();
            sum_vel += abs_vel;
            sum_vel_sq += abs_vel * abs_vel;
        }
        let temp = 1.0 / 3.0 * 1.0 / particles as f64
            * (sum_vel_sq - 1.0 / particles as f64 * sum_vel * sum_vel);

        result[t as usize] = temp;
    }
    //return
    result
}
pub fn granular_temperature_2d(
    filename: &str,          //filename of hdf5 file
    cells: Array1<f64>,      //number of cells to store vec-data
    min_time: f64,           //where to start the averaging
    max_time: f64,           //where to end the averaging
    dimensions: Array2<f64>, // Region where to look at, rest ignored
    norm: bool,              //normalise the size of the vectors
    radius: Array1<f64>,     // include a radius
    particle_id: Array1<i64>,
    axis: usize,
) -> (
    Array2<f64>,
    Array2<f64>,
    Array2<f64>,
    Array2<f64>,
) {
    // Opening hdf5 file
    let file = hdf5::File::open(&filename).expect(&format!(
        "Can not open HDF5 file {:?}",
        &filename
    ));
    //read the number of timesteps inside this hdf5file
    let timesteps: u64 = timesteps(&file);
    //Extracting the min and max dimensions of the simulation
    let array = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let min_array = array.slice(s![0, ..]).to_owned();
    let max_array = array.slice(s![1, ..]).to_owned();

    //let cells_int =
    //before going through timestep, implement:
    // dimension check?

    let cell_size: Array1<f64> = array![
        (max_array[0usize] - min_array[0usize]) / (cells[0usize]),
        (max_array[1usize] - min_array[1usize]) / (cells[1usize]),
        (max_array[2usize] - min_array[2usize]) / (cells[2usize])
    ];

    //array to count how many particles found per cell
    let mut ngrid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut sum_vel_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut sum_vel_sq_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    // find the two axis indizes which we want to "see"
    let mut first_axis = 4;
    let mut sec_axis = 4;
    for x in 0..3{
        if x == axis{continue};
        if first_axis == 4{ first_axis = x;
        }else if sec_axis ==4{
            sec_axis =x ;
        }else{
            panic!(&format!(
                "variable axis in vectorfield must be between 0 and 2 ! Currently it is {:?}",
                axis,
            ))
        }
    }
    println!("{:?}, {:?}",first_axis,sec_axis);
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).expect(&format!(
                "Could not find group {:?} in file {:?}",
                &name,
                &filename
            ));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ))[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        if current_time > max_time {
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"position\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let velocities = group
            .dataset("velocity")
            .expect(&format!(
                "Can not find dataset \"velocity\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"velocity\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particle_ids = group
            .dataset("particleid")
            .expect(&format!(
                "Can not find dataset \"particleid\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"particleid\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let rad_array = group
            .dataset("radius")
            .expect(&format!(
                "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"dimensions\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {

            //check if this particle is fitting the criteria
            if !check_id(particle_ids[particle] as usize, &particle_id) {
                println!("skipping because of ID");
                continue;
            }
            if !check_radius(rad_array[particle] as f64, &radius) {
                println!("Skipping cause of radius");
                continue;
            }
            let position = positions.slice(s![particle, ..]).to_owned();
            let velocity = velocities.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[first_axis] - min_array[first_axis];
            let z: f64 = position[sec_axis] - min_array[sec_axis];

            //velocities
            let vx: f64 = velocity[first_axis];
            let vz: f64 = velocity[sec_axis];

            // check if the current particle position falls into the specified dimension
            if     x < dimensions[[0usize, first_axis]]
                || x > dimensions[[1usize, first_axis]]
                || z < dimensions[[0usize, sec_axis]]
                || z > dimensions[[1usize, sec_axis]]
            {
                // the particle is out of the field of view
                println!("Skipping a particle as it is out of system");
                continue;
            }
            // find the cell indice where particle is right now

            let i: usize = (x / cell_size[first_axis]).floor() as usize;
            let k: usize = (z / cell_size[sec_axis]).floor() as usize;
            // check if indexes are higher then maximum
            if i >= cells[first_axis] as usize
                || k >= cells[sec_axis] as usize
            {
                continue;
            }


            let vel = velocity;
            let abs_vel: f64 =
                (vel[0usize] * vel[0usize] + vel[1usize] * vel[1usize] + vel[2usize] * vel[2usize])
                    .sqrt();



            ngrid[[k, i]] = ngrid[[k, i]] + 1.0;
            sum_vel_grid[[k, i]] = sum_vel_grid[[k, i]] + abs_vel;
            sum_vel_sq_grid[[k, i]] = sum_vel_sq_grid[[k, i]] + abs_vel*abs_vel;
        }


    }
    let temp = 1.0 / 3.0 * 1.0 / &ngrid
        * (sum_vel_sq_grid - 1.0 / &ngrid * &sum_vel_grid * &sum_vel_grid);

    let (sx, sy) = meshgrid(
        Array::linspace(
            min_array[first_axis],
            max_array[first_axis],
            cells[0usize] as usize,
        ),
        Array::linspace(
            min_array[sec_axis],
            max_array[sec_axis],
            cells[2usize] as usize,
        ),
    );

    file.close();

    (temp, ngrid, sx, sy)
}


pub fn surface_velocity(
    filename: &str,          //filename of hdf5 file
    cells: Array1<f64>,      //number of cells to store vec-data
    min_time: f64,           //where to start the averaging
    max_time: f64,           //where to end the averaging
    dimensions: Array1<f64>, // Region where to look at, rest ignored
    radius: Array1<f64>,     // include a radius, only available for sim-data
    particle_id: Array1<i64>,
) -> Array2<f64> {
    // Opening hdf5 file
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");

    //read the number of timesteps inside this hdf5file
    let mut timesteps: u64 = timesteps(&file);

    let array = file
        .dataset("dimensions")
        .unwrap()
        .read_2d::<f64>()
        .unwrap();
    let min_array = array.slice(s![0, ..]).to_owned();
    let max_array = array.slice(s![1, ..]).to_owned();

    //before going through timestep, implement:
    // dimension check?

    let cell_size: Array1<f64> = array![
        (max_array[0usize] - min_array[0usize]) / (cells[0usize]),
        (max_array[1usize] - min_array[1usize]) / (cells[1usize]),
        (max_array[2usize] - min_array[2usize]) / (cells[2usize])
    ];

    //initiate needed 2d arrays:
    let mut v_x_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut v_y_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[1usize].floor() as usize,
    ));
    let mut v_z_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    //array to count how many particles found per cell
    let mut n_x_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut n_y_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[1usize].floor() as usize,
    ));
    let mut n_z_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        if current_time > max_time {
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group
            .dataset("position")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let velocities = group
            .dataset("velocity")
            .expect("error")
            .read_2d::<f64>()
            .unwrap();
        let particle_ids = group
            .dataset("particleid")
            .expect("error")
            .read_1d::<f64>()
            .unwrap();
        let rad_array = group
            .dataset("radius")
            .expect("error")
            .read_1d::<f64>()
            .unwrap();
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {
            //check if this particle is fitting the criteria
            if !check_id(particle_ids[particle] as usize, &particle_id) {
                continue;
            }
            if !check_radius(rad_array[particle] as f64, &radius) {
                continue;
            }
            let position = positions.slice(s![particle, ..]).to_owned();
            let velocity = velocities.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[0usize] - min_array[0usize];
            let y: f64 = position[1usize] - min_array[1usize];
            let z: f64 = position[2usize] - min_array[2usize];

            //velocities
            let vx: f64 = velocity[0usize];
            let vy: f64 = velocity[1usize];
            let vz: f64 = velocity[2usize];

            // find the cell indice where particle is right now

            let i: usize = (x / cell_size[0usize]).floor() as usize;
            let j: usize = (y / cell_size[1usize]).floor() as usize;
            let k: usize = (z / cell_size[2usize]).floor() as usize;
            if i == cells[0usize] as usize
                || j == cells[1usize] as usize
                || k == cells[2usize] as usize
            {
                continue;
            }

            v_x_grid[[k, i]] = v_x_grid[[k, i]] + vx;
            v_z_grid[[k, i]] = v_z_grid[[k, i]] + vz;
            v_y_grid[[k, j]] = v_y_grid[[k, j]] + vy;

            n_x_grid[[k, i]] = n_x_grid[[k, i]] + 1.0;
            n_z_grid[[k, i]] = n_z_grid[[k, i]] + 1.0;
            n_y_grid[[k, j]] = n_y_grid[[k, j]] + 1.0;
        }
    }

    v_x_grid = v_x_grid / &n_x_grid;
    v_y_grid = v_y_grid / &n_y_grid;
    v_z_grid = v_z_grid / &n_z_grid;
    let (sx, sy) = meshgrid(
        Array::linspace(
            0.0,
            cells[0usize] * cell_size[0usize],
            cells[0usize] as usize,
        ),
        Array::linspace(
            0.0,
            cells[2usize] * cell_size[2usize],
            cells[2usize] as usize,
        ),
    );
    let norm_arr = norm_three(&v_x_grid, &v_y_grid, &v_z_grid).to_owned();
    v_x_grid = v_x_grid / &norm_arr;
    v_y_grid = v_y_grid / &norm_arr;
    v_z_grid = v_z_grid / &norm_arr;

    file.close();

    // find the value of the highest nonzero value

    for colomn in n_x_grid.axis_iter(Axis(0)) {
        for value in colomn.axis_iter(Axis(0)) {
            println!("{:?}", value);
        }
    }

    //return
    v_x_grid
}

///Function to calculate the disperson of a system in a given point
/// Output:
///     Dispersion: PyArray1<f64>
/// Input:
///     delta_t: timestep difference for the dispersion calculation
///     time: timestep where to start the calculation
///     position: current position where to calculate the dispersion
///     delta_position: cubic cell around "position"
///
/// Errors:
///     If delta_t + time is bigger then the max timesteps

pub fn dispersion(
    file: &str,
    timestep: Array1<usize>,
    delta_t: usize,
    mesh_size: Array1<f64>,
    dimensions: Array2<f64>,
    cells: Array1<i64>,
) -> (Array3<f64>,f64) {
    // opening file
    let file = hdf5::File::open(file).expect(&format!("Could not open file {:?}", file));
    //extract system dimensions from current datafile
    let temporary_array = file
        .dataset("dimensions")
        .expect(&format!(
            "Could not find dataset \"dimensions\" in file {:?}",
            file
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Could not read data from dataset \"dimensions\" in file {:?}",
            file
        ));
    let mut min_array = temporary_array.slice(s![0, ..]).to_owned();
    let mut max_array = temporary_array.slice(s![1, ..]).to_owned();
    if dimensions.slice(s![0, ..]).to_owned()[0usize] > min_array[0usize]  {
        min_array[0usize] = dimensions.slice(s![0, ..]).to_owned()[0usize]
    }
    if dimensions.slice(s![0, ..]).to_owned()[1usize] > min_array[1usize]  {
        min_array[1usize] = dimensions.slice(s![0, ..]).to_owned()[1usize]
    }
    if dimensions.slice(s![0, ..]).to_owned()[2usize] > min_array[2usize]  {
        min_array[2usize] = dimensions.slice(s![0, ..]).to_owned()[2usize]
    }

    if dimensions.slice(s![1, ..]).to_owned()[0usize] < max_array[0usize]  {
        max_array[0usize] = dimensions.slice(s![1, ..]).to_owned()[0usize]
    }
    if dimensions.slice(s![1, ..]).to_owned()[1usize] < max_array[1usize]  {
        max_array[1usize] = dimensions.slice(s![1, ..]).to_owned()[1usize]
    }
    if dimensions.slice(s![1, ..]).to_owned()[2usize] < max_array[2usize]  {
        max_array[2usize] = dimensions.slice(s![1, ..]).to_owned()[2usize]
    }

    let system_length = &max_array - &min_array;
    // Initialize mesh
    let mut own_mesh_size: Array1<f64> = mesh_size;
    let mut own_cells: Array1<usize> = cells.mapv(|x| x as usize);
    if own_mesh_size[0] == 0.0 {
        if own_cells[0] == 0usize {
            panic!(&format!(
                "No clear instuctions in how to make Mesh for dispersion calculation. Aborting!"
            ))
        } else {
            // how to calculate the cell if the number of cells are given
            // we know the number of cells in the cystem, using min and max array we can calculate
            // the size of the cell_size
            own_mesh_size = system_length / own_cells.mapv(|elem| elem as f64);
        }
    } else {
        //check if own_cells is also valid
        if own_cells[0] != 0usize {
            panic!(&format!(
                "Unclear instructions in building mesh for Dispersion. Check inputs!"
            ))
        } else {
            // how to calculate mesh when the cell size is given
            own_cells = (system_length / &own_mesh_size).mapv(|elem| elem as usize)
        }
    }

    let mut cells_sqsum_x = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
        //vec![vec![vec![0.0; own_cells.clone()[2]]; own_cells.clone()[1]]; own_cells.clone()[0]];
    let mut cells_sum_x = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut cells_sqsum_y = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
    let mut cells_sum_y = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut cells_sqsum_z = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
    let mut cells_sum_z = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut n_cells = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut dispersion_cells = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
    // Get all necessery information
    // for the first timestep:
    let timesteps = timesteps(&file) as usize;
    for step in timestep[0]..timestep[1] {
        if step+delta_t >= timesteps{continue};
        let name: String = "timestep ".to_string() + &step.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];

        let positions = group
            .dataset("position")
            .expect("Can not find dataset \"position\" in the group")
            .read_2d::<f64>()
            .unwrap();

        let particle_ids = group
            .dataset("particleid")
            .expect("Can not find dataset \"particleid\" in the group")
            .read_1d::<usize>()
            .unwrap();

        let particles = positions.len() / 3;
        // for the second timestep :

        let name: String = "timestep ".to_string() + &(&step + &delta_t).to_string();
        let group2 = file.group(&name).unwrap();
        let current_time2 = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];

        let positions2 = group2
            .dataset("position")
            .expect("Can not find dataset \"position\" in the group")
            .read_2d::<f64>()
            .unwrap();

        let particle_ids2 = group2
            .dataset("particleid")
            .expect("Can not find dataset \"particleid\" in the group")
            .read_1d::<usize>()
            .unwrap();

        //particles might not be sorted by particle_ids!
        // loop over particle. find its cell and store pparticle_ids in cell
        for position_in_array in 0..particles {
            // position of particle in ARrray 2 is assumed to be
            // in the same spot first
            let mut position_in_array_2 = position_in_array; // this might be not true as LIGGHTS is stupid
                      // find the current cell of this particle
            let p_id = particle_ids[position_in_array];
            let pos = positions.slice(s![position_in_array, ..]).to_owned() - &min_array;
            let pos_abs = positions.slice(s![position_in_array, ..]).to_owned();

            if     pos_abs[0] < dimensions[[0usize, 0]]
                || pos_abs[0] > dimensions[[1usize, 0]]
                || pos_abs[1] < dimensions[[0usize, 1]]
                || pos_abs[1] > dimensions[[1usize, 1]]
                || pos_abs[2] < dimensions[[0usize, 2]]
                || pos_abs[2] > dimensions[[1usize, 2]]
            {
                // the particle is out of the field of view
                //println!("Skipping a particle as it is out of system");
                continue;
            }

            let p_id_2;
            let mut true_id = false;
            if position_in_array_2 >= particle_ids2.len(){
                p_id_2 =0;
            }else{
                p_id_2 = particle_ids2[position_in_array_2];
            };
            if p_id_2 != p_id {
                // if they are not the same, the right particle position must be found
                for x in 0..particle_ids2.len()-1 {
                    // loop though every particle in the second timestep and
                    // check if particle particle_ids matches
                    if particle_ids2[x] == p_id {
                        // if it matches save the new array postiion and breack the loop
                        position_in_array_2 = x;
                        true_id = true;
                        break;
                    }
                }
            }else {true_id = true;}

            if !true_id{
                // particle dissapeared
                continue ;
            }
            //now we can for sure get the right position in second timesteps
            let pos_2 = positions2.slice(s![position_in_array_2, ..]).to_owned() - &min_array;
            // variance algorythm see:
            // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance

            // find the cell in which the particle is
            // for timestep 0
            let cellx = (pos[0] / &own_mesh_size[0]).floor() as usize;
            let celly = (pos[1] / &own_mesh_size[1]).floor() as usize;
            let cellz = (pos[2] / &own_mesh_size[2]).floor() as usize;
            if cellx == own_cells[0] || celly == own_cells[1] || cellz == own_cells[2] {
                continue;
            }
            // add to calculate how many particles have been in this scell
            n_cells[[cellx,celly,cellz]] += 1.0;
            // calculate the sum of all positions
            cells_sum_x[[cellx,celly,cellz]] += pos_2[0];
            cells_sum_y[[cellx,celly,cellz]] += pos_2[1];
            cells_sum_z[[cellx,celly,cellz]] += pos_2[2];
            //calculate the square of all positions
            cells_sqsum_x[[cellx,celly,cellz]] += pos_2[0] * pos_2[0];
            cells_sqsum_y[[cellx,celly,cellz]]+= pos_2[1] * pos_2[1];
            cells_sqsum_z[[cellx,celly,cellz]] += pos_2[2] * pos_2[2];
        }
    }
    let mut mixing_efficiency = 0.0;
    let mut total_passes = 0.0;
    //go through all cells and calculaate the dispersion number
    for cellx in 0..own_cells[0] {
        for celly in 0..own_cells[1] {
            for cellz in 0..own_cells[2] {
                let n = n_cells[[cellx,celly,cellz]];
                // skipp this cell if the number of particles is small
                if n <= 10.0 {
                    continue;
                }
                //varianze in x direction
                let x = cells_sqsum_x[[cellx,celly,cellz]] / n
                    - cells_sum_x[[cellx,celly,cellz]] / n * cells_sum_x[[cellx,celly,cellz]] / n;
                //varianze in y direction
                let y = cells_sqsum_y[[cellx,celly,cellz]] / n
                    - cells_sum_y[[cellx,celly,cellz]] / n * cells_sum_y[[cellx,celly,cellz]]/ n;
                //varianze in z direction
                let z = cells_sqsum_z[[cellx,celly,cellz]] / n
                    - cells_sum_z[[cellx,celly,cellz]] / n * cells_sum_z[[cellx,celly,cellz]] / n;

                dispersion_cells[[cellx,celly,cellz]] = (x + y + z) * n / (n - 1.0);
                total_passes += n;
                mixing_efficiency += dispersion_cells[[cellx,celly,cellz]] * n;
            }
        }
    }
    mixing_efficiency /= total_passes;
    // return dispersion
    (dispersion_cells, mixing_efficiency)
}


pub fn dispersion_pept(
    file: &str,
    delta_t: f64,
    mesh_size: Array1<f64>,
    dimensions: Array2<f64>,
    cells: Array1<i64>,
    max_error: f64,
) -> (Array3<f64>,f64) {
    // opening file
    let file = hdf5::File::open(file).expect(&format!("Could not open file {:?}", file));
    //extract system dimensions from current datafile
    let temporary_array = file
        .dataset("dimensions")
        .expect(&format!(
            "Could not find dataset \"dimensions\" in file {:?}",
            file
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Could not read data from dataset \"dimensions\" in file {:?}",
            file
        ));
    let mut min_array = temporary_array.slice(s![0, ..]).to_owned();
    let mut  max_array = temporary_array.slice(s![1, ..]).to_owned();

    if dimensions.slice(s![0, ..]).to_owned()[0usize] > min_array[0usize]  {
        min_array[0usize] = dimensions.slice(s![0, ..]).to_owned()[0usize]
    }
    if dimensions.slice(s![0, ..]).to_owned()[1usize] > min_array[1usize]  {
        min_array[1usize] = dimensions.slice(s![0, ..]).to_owned()[1usize]
    }
    if dimensions.slice(s![0, ..]).to_owned()[2usize] > min_array[2usize]  {
        min_array[2usize] = dimensions.slice(s![0, ..]).to_owned()[2usize]
    }

    if dimensions.slice(s![1, ..]).to_owned()[0usize] < max_array[0usize]  {
        max_array[0usize] = dimensions.slice(s![1, ..]).to_owned()[0usize]
    }
    if dimensions.slice(s![1, ..]).to_owned()[1usize] < max_array[1usize]  {
        max_array[1usize] = dimensions.slice(s![1, ..]).to_owned()[1usize]
    }
    if dimensions.slice(s![1, ..]).to_owned()[2usize] < max_array[2usize]  {
        max_array[2usize] = dimensions.slice(s![1, ..]).to_owned()[2usize]
    }
    let system_length = &max_array - &min_array;
    // Initialize mesh
    let mut own_mesh_size: Array1<f64> = mesh_size;
    let mut own_cells: Array1<usize> = cells.mapv(|x| x as usize);
    if own_mesh_size[0] == 0.0 {
        if own_cells[0] == 0usize {
            panic!(&format!(
                "No clear instuctions in how to make Mesh for dispersion calculation. Aborting!"
            ))
        } else {
            // how to calculate the cell if the number of cells are given
            // we know the number of cells in the cystem, using min and max array we can calculate
            // the size of the cell_size
            own_mesh_size = system_length / own_cells.mapv(|elem| elem as f64);
        }
    } else {
        //check if own_cells is also valid
        if own_cells[0] != 0usize {
            panic!(&format!(
                "Unclear instructions in building mesh for Dispersion. Check inputs!"
            ))
        } else {
            // how to calculate mesh when the cell size is given
            own_cells = (system_length / &own_mesh_size).mapv(|elem| elem as usize)
        }
    }

    let mut cells_sqsum_x = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
        //vec![vec![vec![0.0; own_cells.clone()[2]]; own_cells.clone()[1]]; own_cells.clone()[0]];
    let mut cells_sum_x = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut cells_sqsum_y = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
    let mut cells_sum_y = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut cells_sqsum_z = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
    let mut cells_sum_z = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut n_cells = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));

    let mut dispersion_cells = Array3::<f64>::zeros((own_cells.clone()[0],own_cells.clone()[1],own_cells.clone()[2]));
    // Get all necessery information
    // for the first timestep:

        let name: String = "timestep 0".to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];

        let positions = group
            .dataset("position")
            .expect("Can not find dataset \"position\" in the group")
            .read_2d::<f64>()
            .unwrap();
        let times = group
            .dataset("time_pept")
            .expect("Can not find dataset \"time_pept\" in the group")
            .read_1d::<f64>()
            .unwrap();


        let max_time_step = positions.len() / 3;
        // for the second timestep :

        //particles might not be sorted by particle_ids!
        // loop over particle. find its cell and store pparticle_ids in cell
        for position_in_array in 0..max_time_step-1 {
            // check if particle is in FoV
            let pos_abs = positions.slice(s![position_in_array, ..]).to_owned();

            if     pos_abs[0] < dimensions[[0usize, 0]]
                || pos_abs[0] > dimensions[[1usize, 0]]
                || pos_abs[1] < dimensions[[0usize, 1]]
                || pos_abs[1] > dimensions[[1usize, 1]]
                || pos_abs[2] < dimensions[[0usize, 2]]
                || pos_abs[2] > dimensions[[1usize, 2]]
            {
                // the particle is out of the field of view
                //println!("Skipping a particle as it is out of system");
                continue;
            }
            // find the next particle
            // looking into the future
            let mut position_in_array_2 = 0;
            let mut error = f64::MAX;
            let time_now = times[position_in_array] + delta_t;
            for position_2 in position_in_array..max_time_step{
                let new_error = (times[position_2] - time_now).abs();
                if new_error < error{

                    error = new_error;
                    position_in_array_2 = position_2;
                }

                if new_error - error > 0.{
                    // next error is bigger than it was before, end loop

                    break;
                }
            }
            //
            if error > max_error {continue}
            if position_in_array_2 == 0 {continue}


            let pos = positions.slice(s![position_in_array, ..]).to_owned() - &min_array;
            let pos_2 = positions.slice(s![position_in_array_2, ..]).to_owned() - &min_array;
            // variance algorythm see:
            // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance

            // find the cell in which the particle is
            // for timestep 0
            let cellx = (pos[0] / &own_mesh_size[0]).floor() as usize;
            let celly = (pos[1] / &own_mesh_size[1]).floor() as usize;
            let cellz = (pos[2] / &own_mesh_size[2]).floor() as usize;
            if cellx == own_cells[0] || celly == own_cells[1] || cellz == own_cells[2] {
                continue;
            }
            // add to calculate how many particles have been in this scell
            n_cells[[cellx,celly,cellz]] += 1.0;
            // calculate the sum of all positions
            cells_sum_x[[cellx,celly,cellz]] += pos_2[0];
            cells_sum_y[[cellx,celly,cellz]] += pos_2[1];
            cells_sum_z[[cellx,celly,cellz]] += pos_2[2];
            //calculate the square of all positions
            cells_sqsum_x[[cellx,celly,cellz]] += pos_2[0] * pos_2[0];
            cells_sqsum_y[[cellx,celly,cellz]] += pos_2[1] * pos_2[1];
            cells_sqsum_z[[cellx,celly,cellz]] += pos_2[2] * pos_2[2];
        }

    let mut mixing_efficiency = 0.0;
    let mut total_passes = 0.0;
    //go through all cells and calculaate the dispersion number
    for cellx in 0..own_cells[0] {
        for celly in 0..own_cells[1] {
            for cellz in 0..own_cells[2] {
                let n = n_cells[[cellx,celly,cellz]];
                // skipp this cell if the number of particles is small
                if n <= 10.0 {
                    continue;
                }
                //varianze in x direction
                let x = cells_sqsum_x[[cellx,celly,cellz]] / n
                    - cells_sum_x[[cellx,celly,cellz]] / n * cells_sum_x[[cellx,celly,cellz]] / n;
                //varianze in y direction
                let y = cells_sqsum_y[[cellx,celly,cellz]] / n
                    - cells_sum_y[[cellx,celly,cellz]] / n * cells_sum_y[[cellx,celly,cellz]]/ n;
                //varianze in z direction
                let z = cells_sqsum_z[[cellx,celly,cellz]] / n
                    - cells_sum_z[[cellx,celly,cellz]] / n * cells_sum_z[[cellx,celly,cellz]] / n;

                dispersion_cells[[cellx,celly,cellz]] = (x + y + z) * n / (n - 1.0);
                total_passes += n;
                mixing_efficiency += dispersion_cells[[cellx,celly,cellz]] * n;
            }
        }
    }
    mixing_efficiency /= total_passes;
    // return dispersion
    (dispersion_cells, mixing_efficiency)
}

pub fn mean_squared_displacement(
    filename: &str,
    start_timestep: usize,
) -> (Array1<f64>, Array1<f64>) {
    // Calculate the MSD for all times and return the array containing its values and
    // one array with the time shifts

    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");

    let timesteps: u64 = timesteps(&file); // get the number of timesteps in the HDF5 file
    let mut MSD_result: Array1<f64> = Array1::<f64>::zeros(timesteps as usize - start_timestep);
    let mut times_result: Array1<f64> = Array1::<f64>::zeros(timesteps as usize - start_timestep);
    //initiation of MSD
    let name: String = "timestep ".to_string() + &start_timestep.to_string();
    let group = file.group(&name).expect(&format!(
        "Could not find group \"{:?}\" in file {:?}",
        &name, filename
    ));
    // save the initial position
    let init_positions = group
        .dataset("position")
        .expect(&format!(
            "Could not find dataset \"position\" in file {:?}",
            filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Could not read dataset \"position\" in file {:?}\nCheck datatype in file",
            filename
        ));
    // we need the ID of the particles as they might mix during the run
    let init_ids = group
        .dataset("particleid")
        .expect(&format!(
            "Could not find dataset \"particleid\" in file {:?}",
            filename
        ))
        .read_1d::<usize>()
        .expect(&format!(
            "Could not read dataset \"particleid\" in file {:?}\nCheck datatype in file",
            filename
        ));
    let init_time = group
        .dataset("time")
        .expect(&format!(
            "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_raw::<f64>()
        .expect(&format!(
            "Can not read data from \"time\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ))[0];

    for t in start_timestep..timesteps as usize {
        let name: String = "timestep ".to_string() + &t.to_string();
        let group = file.group(&name).expect(&format!(
            "Could not find group \"{:?}\" in file {:?}",
            &name, filename
        ));
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Could not find dataset \"position\" in file {:?}",
                filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Could not read dataset \"position\" in file {:?}\nCheck datatype in file",
                filename
            ));
        let ids = group
            .dataset("particleid")
            .expect(&format!(
                "Could not find dataset \"particleid\" in file {:?}",
                filename
            ))
            .read_1d::<usize>()
            .expect(&format!(
                "Could not read dataset \"particleid\" in file {:?}\nCheck datatype in file",
                filename
            ));
        let particles = positions.len() / 3; // get number of particles

        for particle in 0..particles {
            // check if particle is still in the same position
            // generate variable "pos_in_arr" which is the real position in the array
            let mut pos_in_init_array: usize = particle;
            let p_id = ids[particle];
            if p_id != init_ids[pos_in_init_array] {
                for x in 0..particles {
                    if p_id == init_ids[x] {
                        pos_in_init_array = x;
                        break;
                    }
                }
                // check here if particle dissapeared raise error if so
                if pos_in_init_array == particle {
                    println!("WARNING: New Particle discovered. This particle will be ignored")
                }
            }
            // calculate the distance travled from init to now
            let init_pos = init_positions.slice(s![pos_in_init_array, ..]).to_owned();
            let pos = positions.slice(s![particle, ..]).to_owned();
            let distance = (pos[0usize] - init_pos[0usize]) * (pos[0usize] - init_pos[0usize])
                + (pos[1usize] - init_pos[1usize]) * (pos[1usize] - init_pos[1usize])
                + (pos[2usize] - init_pos[2usize]) * (pos[2usize] - init_pos[2usize]);
            // sum distance in array
            MSD_result[t as usize - start_timestep] += distance;
        }
        times_result[t as usize - start_timestep] = group
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ))[0]
            - init_time;
        MSD_result[t as usize - start_timestep] /= particles as f64;
    }
    //return
    (MSD_result, times_result)
}

pub fn mean_squared_displacement_pept(
    filename: &str,
    start_timestep: usize,
    max_msd: f64,
    bins: usize,
) -> (Array1<f64>, Array1<f64>) {
    // Calculate the MSD for all times and return the array containing its values and
    // one array with the time shifts

    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");



    //initiation of MSD
    let name: String = "timestep ".to_string() + &start_timestep.to_string();
    let group = file.group(&name).expect(&format!(
        "Could not find group \"{:?}\" in file {:?}",
        &name, filename
    ));
    // save the initial position
    let positions = group
        .dataset("position")
        .expect(&format!(
            "Could not find dataset \"position\" in file {:?}",
            filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Could not read dataset \"position\" in file {:?}\nCheck datatype in file",
            filename
        ));
    // we need the ID of the particles as they might mix during the run

    let times: Array1<f64> = group
        .dataset("time_pept")
        .expect(&format!(
            "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_1d::<f64>()
        .expect(&format!(
            "Can not read data from \"time\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));

    let timesteps = times.len();

    //let (min_time,max_time) = minmax(&times);

    let mut MSD_result: Array1<f64> = Array1::<f64>::zeros(bins);
    let mut norm_array: Array1<f64> = Array1::<f64>::zeros(bins);
    let times_result: Array1<f64> = Array1::<f64>::linspace(0.,max_msd,bins);
    for start in start_timestep..timesteps as usize - 1 {
        let init_pos = positions.slice(s![start, ..]).to_owned();
        let init_time = times[start];
        for t in start..timesteps as usize {
                let dt =  times[t]-init_time;
                if dt > max_msd{break;}
                //println!("{}",dt);
                //println!("{}",max_msd);
                //println!("{:?}",times_result);
                // find incex of the current dt
                let id = find_closest(&times_result,dt);

                // calculate the distance travled from init to now
                let pos = positions.slice(s![t, ..]).to_owned();
                let distance = (pos[0usize] - init_pos[0usize]) * (pos[0usize] - init_pos[0usize])
                    + (pos[1usize] - init_pos[1usize]) * (pos[1usize] - init_pos[1usize])
                    + (pos[2usize] - init_pos[2usize]) * (pos[2usize] - init_pos[2usize]);
                // sum distance in array
                MSD_result[id] += distance;
                norm_array[id] += 1.;

        }

    }
    //return
    (MSD_result/norm_array, times_result)
}

pub fn power_draw(filename: &str, min_time: f64) -> Array1<f64> {
    // Calculate the power which the particles in the system currently needed
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);
    let mut power = Array1::<f64>::zeros(timesteps as usize);
    let force_exists: bool = file
        .group("timestep 0")
        .expect(&format!("Could not open timestep 0 in file {:?}", filename))
        .link_exists("force");
    let radius_exists: bool = file
        .group("timestep 0")
        .expect(&format!("Could not open timestep 0 in file {:?}", filename))
        .link_exists("radius");
    let density_exists: bool = file
        .group("timestep 0")
        .expect(&format!("Could not open timestep 0 in file {:?}", filename))
        .link_exists("density");
    let ppcloud_exists: bool = file
        .group("timestep 0")
        .expect(&format!("Could not open timestep 0 in file {:?}", filename))
        .link_exists("ppcloud");
    let start = if force_exists { 0 } else { 1 };
    for timestep in start..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file
            .group(&name)
            .expect(&format!("Could not open {:?} in file {:?}", name, filename));
        let current_time = group
            .dataset("time")
            .expect(&format!("Couldn't open dataset: {:?}", &name))
            .read_raw::<f64>()
            .expect(&format!(
                "Couldn't read data from time-step: {:?}, check the raw data",
                &timestep
            ))[0];
        if current_time < min_time{continue}
        let velocities = group
            .dataset("velocity")
            .expect(
                &format!("Could not find dataset velocity in file {:?}",filename)
            )
            .read_2d::<f64>()
            .expect(
                &format!("Could not read data from dataset velocity in file {:?}. Filetype must be Array2<f64>",filename)
            );

        let velocities2 = if force_exists {
            Array2::<f64>::zeros((1, 1))
        } else {
            file.group(&("timestep ".to_string() + &(timestep-1).to_string()))
            .expect("Error reading the previous timestep in the alternative power draw calculation loop. This is strange!")
            .dataset("velocity")
            .expect(
                &format!("Could not find dataset velocity in file {:?}",filename)
            )
            .read_2d::<f64>()
            .expect(
                &format!("Could not read data from dataset velocity in file {:?}. Filetype must be Array2<f64>",filename)
            )
        };
        let dt = get_dt(&file);
        let particles = velocities.len() / 3;
        // if force data on every particle exists read it
        // if not we must calculate it from the last two timesteps
        // F = m * a = m * (v2-v1)/dt
        let forces = if force_exists {
            group
                .dataset("force")
                .expect("error")
                .read_2d::<f64>()
                .unwrap()
        } else {
            Array2::<f64>::zeros((particles, 3))
        };
        let radius = if radius_exists {
            group
                .dataset("radius")
                .expect("error")
                .read_1d::<f64>()
                .unwrap()
        } else {
            Array1::<f64>::ones(particles)
        };
        let density = if density_exists {
            group
                .dataset("density")
                .expect("error")
                .read_1d::<f64>()
                .unwrap()
        } else {
            Array1::<f64>::ones(particles)
        };
        let ppcloud = if ppcloud_exists {
            group
                .dataset("ppcloud")
                .expect("error")
                .read_1d::<f64>()
                .unwrap()
        } else {
            Array1::<f64>::ones(particles)
        };
        for particle in 0..particles {
            let vel = velocities.slice(s![particle, ..]).to_owned();
            let force = forces.slice(s![particle, ..]).to_owned();

            //calculate the power of this particles
            let po = if force_exists {
                (force[0] * force[0] + force[1] * force[1] + force[2] * force[2]).sqrt()
                    * (vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]).sqrt()
            } else {
                let vel2 = velocities2.slice(s![particle, ..]).to_owned();
                let v1 = (vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]).sqrt();
                let v2 = (vel2[0] * vel2[0] + vel2[1] * vel2[1] + vel2[2] * vel2[2]).sqrt();

                //println!("v1: {:?}, v2 {:?}, dt {:?}\nvel2 {:?}", v1, v2, dt, vel2);
                //return
                let extra = if ppcloud_exists && density_exists && radius_exists{
                            ppcloud[particle]
                            *density[particle]
                            * 4.0/3.0 * std::f64::consts::PI
                            *radius[particle]
                            *radius[particle]
                            *radius[particle]
                        } else{1.0};
                (v2 - v1) / dt * v2 * extra
            };
            power[timestep as usize] += po;
            //println!("{:?}", power[timestep as usize]);
            //println!("{:?}", po);
        }
    }

    power
}
pub fn circulation_time_boundary(
    filename: &str,
    boundary: Array1<f64>,
    axis: usize,
    min_time: f64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // Opening hdf5 file
    let file = hdf5::File::open(&filename).expect(&format!(
        "Can not open HDF5 file {:?}",
        &filename
    ));
    //read the number of timesteps inside this hdf5file
    let timesteps: u64 = timesteps(&file);
    //Extracting the min and max dimensions of the simulation
    let array = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let min_array = array.slice(s![0, ..]).to_owned();
    let max_array = array.slice(s![1, ..]).to_owned();
    let name = "timestep 0";
    let particle_number = file.group(&name).expect(&format!(
            "Could not find group {:?} in file {:?}",
            &name,
            &filename
        ))
        .dataset("position")
        .expect(&format!(
            "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"position\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ))
        .len()/3;

    // Init variables
    let mut times: Vec<f64> = Vec::with_capacity(0);
    let mut up_times: Vec<f64> = Vec::with_capacity(0);
    let mut down_times: Vec<f64> = Vec::with_capacity(0);
    let mut start_flag_array = Array1::<usize>::zeros(particle_number+1);
    let mut mid_flag_array = Array1::<usize>::zeros(particle_number+1);
    let mut time_flag_array = Array1::<f64>::zeros(particle_number+1);
    let mut mid_time_flag_array = Array1::<f64>::zeros(particle_number+1);
    let mut count =0;
    for timestep in 1..timesteps {
        // load positions now and positions in next timetsep
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).expect(&format!(
                "Could not find group {:?} in file {:?}",
                &name,
                &filename
            ));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ))[0];
        if current_time < min_time{
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"position\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));


        let particle_ids = group
            .dataset("particleid")
            .expect("Can not find dataset \"particleid\" in the group")
            .read_1d::<usize>()
            .unwrap();
        // smae forthe second timestep
        let name2: String = "timestep ".to_string() + &(timestep-1).to_string() ;
        let group2 = file.group(&name2).expect(&format!(
                "Could not find group {:?} in file {:?}",
                &name,
                &filename
            ));
        let current_time2 = group2
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ))[0];

        //let dataset = group.dataset("position").expect( "error");
        let positions2 = group2
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"position\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particle_ids2 = group2
            .dataset("particleid")
            .expect("Can not find dataset \"particleid\" in the group")
            .read_1d::<usize>()
            .unwrap();
        for position_in_array in 0..particle_number{

            let mut position_in_array_2 = position_in_array; // this might be not true as LIGGHTS is stupid
                      // find the current cell of this particle
            let p_id = particle_ids[position_in_array];
            let pos = (positions.slice(s![position_in_array, ..]).to_owned() - &min_array)[axis];
            let p_id_2;
            if position_in_array_2 >= particle_ids2.len(){
                p_id_2 =0;
            }else{
                p_id_2 = particle_ids2[position_in_array_2];
            };
            if p_id_2 != p_id {
                // if they are not the same, the right particle position must be found
                let mut flag = false;
                for x in 0..particle_ids2.len()-1 {

                    // loop though every particle in the second timestep and
                    // check if particle particle_ids matches
                    if particle_ids2[x] == p_id {
                        // if it matches save the new array postiion and breack the loop
                        position_in_array_2 = x;
                        break;
                    }
                        if x == particle_ids2.len()-1{
                            flag = true;
                        }



                }
            // continue if the particle wasnt found: e.g got kicked out of system
            if flag{continue;}


            }
            //now we can for sure get the right position in second timesteps
            let pos_old = (positions2.slice(s![position_in_array_2, ..]).to_owned() - &min_array)[axis];
            // start_flag_array
            // mid_flag_array
            // time_flag_array
            // check where the particle currently is
            // we are only checking the axis "axis"

            // particle crosses into upper border

            if pos  > boundary[1] && pos_old  < boundary[1]{
                count +=1;
                //println!("particle inside upper ");
                //println!("{}->{},{},{},{}",pos_old,pos,boundary[1],boundary[0],pos  < boundary[0] && pos_old  > boundary[0]);
                // if particle before was below
                if start_flag_array[p_id] == 2{
                    // say " particle was in the other region"
                    mid_flag_array[p_id] = 1;
                    up_times.push(current_time - time_flag_array[p_id]);

                }else if start_flag_array[p_id] == 1 && mid_flag_array[p_id] == 1 {
                    up_times.push(current_time - mid_time_flag_array[p_id]);
                }

            }
            // particle crosses out of upper border
            else if pos  < boundary[1] && pos_old  > boundary[1]{
                //count -=1;
                //println!("particle out of upper");
                //println!("{}->{},{},{},{}",pos_old,pos,boundary[1],boundary[0],pos  < boundary[0] && pos_old  > boundary[0]);
                // if particle before was nowhere
                if start_flag_array[p_id] == 0 {
                    // start clock
                    time_flag_array[p_id] = current_time;
                    start_flag_array[p_id] = 1;
                }else if start_flag_array[p_id] == 1 && mid_flag_array[p_id] == 1{
                    // end of one circulation
                    times.push(current_time - time_flag_array[p_id]);
                    // restart the run
                    mid_flag_array[p_id] = 0;
                    time_flag_array[p_id] = current_time;
                }else if start_flag_array[p_id] == 1 {
                    time_flag_array[p_id] = current_time;
                }else if start_flag_array[p_id] == 2{
                    mid_time_flag_array[p_id] = current_time;
                }
                continue;
            }



            ///// same vor lower border
            // particle crosses into low border
            else if pos  < boundary[0] && pos_old  > boundary[0]{
                //println!("particle in lower");
                //println!("{}->{},{},{},{}",pos_old,pos,boundary[1],boundary[0],pos  < boundary[0] && pos_old  > boundary[0]);
                // if particle before was below
                if start_flag_array[p_id] == 1{
                    // say " particle was in the other region"
                    mid_flag_array[p_id] = 1;
                    down_times.push(current_time - time_flag_array[p_id]);
                }else if start_flag_array[p_id] == 2  && mid_flag_array[p_id] == 1  {
                    down_times.push(current_time - mid_time_flag_array[p_id]);
                }


            }
            // particle crosses out of lower border
            else if pos  > boundary[0] && pos_old  < boundary[0]{
                // if particle before was nowhere
                if start_flag_array[p_id] == 0 {
                    // start clock
                    time_flag_array[p_id] = current_time;
                    start_flag_array[p_id] = 2;
                }else if start_flag_array[p_id] == 2 && mid_flag_array[p_id] == 1{
                    // end of one circulation
                    times.push(current_time - time_flag_array[p_id]);
                    // restart the run
                    mid_flag_array[p_id] = 0;
                    time_flag_array[p_id] = current_time;
                }else if start_flag_array[p_id] == 2 {
                    time_flag_array[p_id] = current_time;
                }else if start_flag_array[p_id] == 1{
                    mid_time_flag_array[p_id] = current_time;
                }

            }



        }// end particle for loop
    }; // end time for loop
    (up_times,down_times,times)
}
pub fn circulation_time_boundary_pept(
    filename: &str,
    boundary: Array1<f64>,
    axis: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // Opening hdf5 file
    let file = hdf5::File::open(&filename).expect(&format!(
        "Can not open HDF5 file {:?}",
        &filename
    ));
    //read the number of timesteps inside this hdf5file
    let timesteps: u64 = timesteps(&file);
    //Extracting the min and max dimensions of the simulation
    let array = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let min_array = array.slice(s![0, ..]).to_owned();
    let max_array = array.slice(s![1, ..]).to_owned();
    let name = "timestep 0";
    let particle_number = file.group(&name).expect(&format!(
            "Could not find group {:?} in file {:?}",
            &name,
            &filename
        ))
        .dataset("position")
        .expect(&format!(
            "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"position\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ))
        .len()/3;

    // Init variables
    let mut times: Vec<f64> = Vec::with_capacity(0);
    let mut up_times: Vec<f64> = Vec::with_capacity(0);
    let mut down_times: Vec<f64> = Vec::with_capacity(0);
    let mut start_flag_array = 0;
    let mut mid_flag_array = 0;
    let mut time_flag_array = 0.0;
    let mut mid_time_flag_array = 0.0;
    let mut count =0;

        // load positions now and positions in next timetsep
        let name: String = "timestep ".to_string() + &0.to_string();
        let group = file.group(&name).expect(&format!(
                "Could not find group {:?} in file {:?}",
                &name,
                &filename
            ));
        let current_time = group
            .dataset("time_pept")
            .expect(&format!(
                "Can not find dataset \"time_pept\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"time_pept\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));

        //let dataset = group.dataset("position").expect( "error");
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"position\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));






        for position_in_array in 1..particle_number{

            let mut position_in_array_2 = position_in_array-1; // this might be not true as LIGGHTS is stupid
                      // find the current cell of this particle
            let pos = (positions.slice(s![position_in_array, ..]).to_owned() )[axis];

            //now we can for sure get the right position in second timesteps
            let pos_old = (positions.slice(s![position_in_array_2, ..]).to_owned() )[axis];
            // start_flag_array
            // mid_flag_array
            // time_flag_array
            // check where the particle currently is
            // we are only checking the axis "axis"

            // particle crosses into upper border

            if pos  > boundary[1] && pos_old  < boundary[1]{
                count +=1;
                //println!("particle inside upper ");
                //println!("{}->{},{},{},{}",pos_old,pos,boundary[1],boundary[0],pos  < boundary[0] && pos_old  > boundary[0]);
                // if particle before was below
                if start_flag_array== 2{
                    // say " particle was in the other region"
                    mid_flag_array= 1;
                    up_times.push(current_time[position_in_array] - time_flag_array);

                }else if start_flag_array == 1 && mid_flag_array == 1 {
                    up_times.push(current_time[position_in_array] - mid_time_flag_array);
                }

            }
            // particle crosses out of upper border
            else if pos  < boundary[1] && pos_old  > boundary[1]{
                //count -=1;
                //println!("particle out of upper");
                //println!("{}->{},{},{},{}",pos_old,pos,boundary[1],boundary[0],pos  < boundary[0] && pos_old  > boundary[0]);
                // if particle before was nowhere
                if start_flag_array == 0 {
                    // start clock
                    time_flag_array= current_time[position_in_array];
                    start_flag_array = 1;
                }else if start_flag_array == 1 && mid_flag_array == 1{
                    // end of one circulation
                    times.push(current_time[position_in_array] - time_flag_array);
                    // restart the run
                    mid_flag_array= 0;
                    time_flag_array= current_time[position_in_array];
                }else if start_flag_array == 1 {
                    time_flag_array = current_time[position_in_array];
                }else if start_flag_array == 2{
                    mid_time_flag_array= current_time[position_in_array];
                }
                continue;
            }



            ///// same vor lower border
            // particle crosses into low border
            else if pos  < boundary[0] && pos_old  > boundary[0]{
                //println!("particle in lower");
                //println!("{}->{},{},{},{}",pos_old,pos,boundary[1],boundary[0],pos  < boundary[0] && pos_old  > boundary[0]);
                // if particle before was below
                if start_flag_array== 1{
                    // say " particle was in the other region"
                    mid_flag_array = 1;
                    down_times.push(current_time[position_in_array]- time_flag_array);
                }else if start_flag_array == 2  && mid_flag_array == 1  {
                    down_times.push(current_time[position_in_array]- mid_time_flag_array);
                }


            }
            // particle crosses out of lower border
            else if pos  > boundary[0] && pos_old  < boundary[0]{
                // if particle before was nowhere
                if start_flag_array == 0 {
                    // start clock
                    time_flag_array = current_time[position_in_array];
                    start_flag_array= 2;
                }else if start_flag_array == 2 && mid_flag_array == 1{
                    // end of one circulation
                    times.push(current_time[position_in_array] - time_flag_array);
                    // restart the run
                    mid_flag_array = 0;
                    time_flag_array= current_time[position_in_array];
                }else if start_flag_array== 2 {
                    time_flag_array = current_time[position_in_array];
                }else if start_flag_array == 1{
                    mid_time_flag_array = current_time[position_in_array];
                }

            }



        }// end particle for loop
    (up_times,down_times,times)
}


pub fn shear_rate(
    filename: &str,          //filename of hdf5 file
    cells: Array1<f64>,      //number of cells to store vec-data
    min_time: f64,           //where to start the averaging
    max_time: f64,           //where to end the averaging
    dimensions: Array2<f64>, // Region where to look at, rest ignored
    radius: Array1<f64>,     // include a radius
    particle_id: Array1<i64>,
    axis: usize,
    mut line_offset: f64,
    line_gradient: f64,
) -> (
    Array2<f64>,
    Array2<f64>,
    Array2<f64>,
    Array2<f64>,
    Array1<i64>,
    Array1<i64>,
    f64
) {
    let (vx,vy,sx,sy) = vectorfield(filename,cells,min_time,max_time,dimensions,false,radius,particle_id,axis);
    let len_x = vx.shape()[0];
    if line_offset == -1.0 {
        line_offset = 0.0;//vx.shape()[1] as f64;
    }
    let x1 = Array1::range(0.0, len_x as f64, 1.0);
    let mut velocitys = Array1::zeros( len_x);
    let y = round_array_to_int(line_gradient * &x1 + line_offset);
    let x = round_array_to_int(x1.clone());

    for id_x in 1..len_x-1{
        if id_x > y.len(){continue}
        let id_y = y[id_x] as usize;
        let v_mag = (vx[[id_x,id_y]]*vx[[id_x,id_y]]+vy[[id_x,id_y]]*vy[[id_x,id_y]]).sqrt();
        let angle_1 = (vy[[id_x,id_y]]/vx[[id_x,id_y]]).atan();
        let angle_2 = line_gradient.atan();
        let v_rel = (-vy[[id_x,id_y]]).signum()*v_mag * (angle_2-angle_1).cos();

        velocitys[id_x] = v_rel;
        if velocitys[id_x] == 0. {velocitys[id_x]=f64::NAN }
    }
    let step = (( &sx[[0,1]]- &sx[[0,0]])*( &sx[[0,1]]- &sx[[0,0]]) + ( &sy[[1,0]]- &sy[[0,0]])*( &sy[[1,0]]- &sy[[0,0]])).sqrt();
    let (_,dy) = derivative(x1*step,velocitys.clone(),1);
    let (_,tau) = minmax(&dy);
    (vx,vy,sx,sy,x,y,tau)
}

fn round_array_to_int(arr: Array1<f64>)->Array1<i64>{
    let mut int_arr = arr.iter().map(|x| x.round() as i64).collect();
    int_arr
}

/**
pub fn residence_time_distribution(
    filename: &str,          //filename of hdf5 file
    cells: Array1<f64>,      //number of cells to store vec-data
    min_time: f64,           //where to start the averaging
    max_time: f64,           //where to end the averaging
    dimensions: Array2<f64>, // Region where to look at, rest ignored
    norm: bool,              //normalise the size of the vectors
    radius: Array1<f64>,     // include a radius
    particle_id: Array1<i64>,
) -> (
    Array1<f64>,
) {
    // Opening hdf5 file
    let file = hdf5::File::open(&filename).expect(&format!(
        "Can not open HDF5 file {:?}",
        &filename
    ));
    //read the number of timesteps inside this hdf5file
    let timesteps: u64 = timesteps(&file);
    //Extracting the min and max dimensions of the simulation
    let array = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            &filename
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            &filename
        ));
    let min_array = array.slice(s![0, ..]).to_owned();
    let max_array = array.slice(s![1, ..]).to_owned();



    //initiate needed arrays:
    let mut rtd = ndarray::Array1::<i64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut v_z_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    //array to count how many particles found per cell
    let mut n_x_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));
    let mut n_z_grid = ndarray::Array2::<f64>::zeros((
        cells[2usize].floor() as usize,
        cells[0usize].floor() as usize,
    ));

    // find the two axis indizes which we want to "see"
    let mut first_axis = 4;
    let mut sec_axis = 4;
    for x in 0..3{
        if x == axis{continue};
        if first_axis == 4{ first_axis = x;
        }else if sec_axis ==4{
            sec_axis =x ;
        }else{
            panic!(&format!(
                "variable axis in vectorfield must be between 0 and 2 ! Currently it is {:?}",
                axis,
            ))
        }
    }
    println!("{:?}, {:?}",first_axis,sec_axis);
    for timestep in 0..timesteps - 1 {
        let name: String = "timestep ".to_string() + &timestep.to_string();
        let group = file.group(&name).expect(&format!(
                "Could not find group {:?} in file {:?}",
                &name,
                &filename
            ));
        let current_time = group
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ))[0];
        // check if timestep is in the timeframe given
        if current_time < min_time {
            continue;
        }
        if current_time > max_time {
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"position\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let velocities = group
            .dataset("velocity")
            .expect(&format!(
                "Can not find dataset \"velocity\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"velocity\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particle_ids = group
            .dataset("particleid")
            .expect(&format!(
                "Can not find dataset \"particleid\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"particleid\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let rad_array = group
            .dataset("radius")
            .expect(&format!(
                "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                &filename
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"dimensions\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                &filename
            ));
        let particles = positions.len() / 3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in 0..particles {
            //check if this particle is fitting the criteria
            if !check_id(particle_ids[particle] as usize, &particle_id) {
                println!("skipping because of ID");
                continue;
            }
            if !check_radius(rad_array[particle] as f64, &radius) {
                println!("Skipping cause of radius");
                continue;
            }
            let position = positions.slice(s![particle, ..]).to_owned();
            let velocity = velocities.slice(s![particle, ..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x: f64 = position[first_axis] - min_array[first_axis];
            let z: f64 = position[sec_axis] - min_array[sec_axis];

            //velocities
            let vx: f64 = velocity[first_axis];
            let vz: f64 = velocity[sec_axis];

            // check if the current particle position falls into the specified dimension
            if     x < dimensions[[0usize, first_axis]]
                || x > dimensions[[1usize, first_axis]]
                || z < dimensions[[0usize, sec_axis]]
                || z > dimensions[[1usize, sec_axis]]
            {
                // the particle is out of the field of view
                println!("Skipping a particle as it is out of system");
                continue;
            }
            // find the cell indice where particle is right now

            let i: usize = (x / cell_size[first_axis]).floor() as usize;
            let k: usize = (z / cell_size[sec_axis]).floor() as usize;
            // check if indexes are higher then maximum
            if i == cells[first_axis] as usize
                || k == cells[sec_axis] as usize
            {
                continue;
            }


            v_x_grid[[k, i]] = v_x_grid[[k, i]] + vx;
            v_z_grid[[k, i]] = v_z_grid[[k, i]] + vz;

            n_x_grid[[k, i]] = n_x_grid[[k, i]] + 1.0;
            n_z_grid[[k, i]] = n_z_grid[[k, i]] + 1.0;
        }

    }

    v_x_grid = v_x_grid / &n_x_grid;
    v_z_grid = v_z_grid / &n_z_grid;
    let (sx, sy) = meshgrid(
        Array::linspace(
            0.0,
            cells[0usize] * cell_size[0usize],
            cells[0usize] as usize,
        ),
        Array::linspace(
            0.0,
            cells[2usize] * cell_size[2usize],
            cells[2usize] as usize,
        ),
    );
    if norm {
        let norm_arr = norm_two(&v_x_grid, &v_z_grid).to_owned();
        v_x_grid = v_x_grid / &norm_arr;
        v_z_grid = v_z_grid / &norm_arr;
    }
    file.close();

    (v_x_grid, v_z_grid, sx, sy)
}
**/

fn derivative(x:Array1<f64>,y : Array1<f64>,avg: usize)->(Array1<f64>, Array1<f64>){
    if x.len() != y.len(){
        panic!("X and y should have the same len");
    }

    let mut result_x = Array1::<f64>::zeros(x.len());
    let mut diffy = Array1::<f64>::zeros(y.len());
    let mut diffx = Array1::<f64>::zeros(x.len());
    for id in 0..x.len()-avg{
        diffx[id] = x[id+avg]-x[id];
        diffy[id] = y[id+avg]-y[id];
        result_x[id] = 0.5 * (x[id+avg]+x[id]);
    }
    let result_y = diffy/diffx;//Array1::<f64>::zeros(y.len());

    (result_x,result_y)
}
fn check_id(id: usize, var: &Array1<i64>) -> bool {
    let mut ret_val = false;
    if var[0usize] == -1 && var[1usize] == -1 {
        ret_val = true;
    } else {
        if id >= var[0usize] as usize && id <= var[1usize] as usize {
            ret_val = true;
        }
    }
    ret_val
}
fn check_radius(id: f64, var: &Array1<f64>) -> bool {
    let mut ret_val = false;
    //println!("{:?}",var[1usize]);
    if var[0usize] == -1.0 && var[1usize] == -1.0 {
        ret_val = true;
    } else {
        if id >= var[0usize] && id <= var[1usize] {
            ret_val = true;
        }
    }
    //println!("{:?}",ret_val);
    ret_val
}

fn meshgrid(
    x: ndarray::Array1<f64>,
    y: ndarray::Array1<f64>,
) -> (ndarray::Array2<f64>, ndarray::Array2<f64>) {
    let mut xx = ndarray::Array2::<f64>::zeros((y.len(), x.len()));
    let mut yy = ndarray::Array2::<f64>::zeros((y.len(), x.len()));

    for idx in (0..x.len()) {
        for idy in (0..y.len()) {
            xx[[idy, idx]] = x[idx];
            yy[[idy, idx]] = y[idy];
        }
    }
    return (xx, yy);
}

/// calculates the cartesian norm of 3 velocity Vectors
/// representative for the velocity in x,y and z direction
fn norm_two(arr1: &Array2<f64>, arr2: &Array2<f64>) -> Array2<f64> {
    let mut norm_array: Array2<f64> = Array2::zeros((arr1.shape()[0usize], arr1.shape()[1usize]));

    for idx in (0..norm_array.shape()[0usize]) {
        for idy in (0..norm_array.shape()[1usize]) {
            norm_array[[idx, idy]] = (arr1[[idx, idy]].powf(2.0)
                + arr2[[idx, idy]].powf(2.0))
            .sqrt()
        }
    }

    norm_array
}

fn norm_l2(arr: &Array1<f64>) -> f64 {
    arr.iter()
        .map(|x| x * x)
        .collect::<Array1<f64>>()
        .sum()
        .powf(0.5)
}

fn find_closest(arr: &Array1<f64>, num: f64) -> usize {
    let mut id: usize = 0;

    let len_arr = arr.len();
    let mut smallest: f64 = std::f64::MAX;
    for x in 0..len_arr {
        if (arr[x] - num).abs() < smallest {
            smallest = (arr[x] - num).abs();
            id = x;
        }
    }
    return id;
}

pub fn timesteps(file: &hdf5::File) -> u64 {
    let mut timesteps: u64 = 0;
    let vec = file.member_names().unwrap();
    for x in file.member_names().unwrap() {
        if x.contains("timestep") {
            timesteps += 1;
        }
    }
    timesteps
}

fn get_dt(file: &hdf5::File) -> f64 {
    let t1 = file
        .group("timestep 0")
        .unwrap()
        .dataset("time")
        .unwrap()
        .read_raw::<f64>()
        .unwrap()[0];
    let t2 = file
        .group("timestep 1")
        .expect("Unable to find timestep 1!")
        .dataset("time")
        .expect("Unable to find dataset time in timestep 1")
        .read_raw::<f64>()
        .expect("Unable to read data from dataset time")[0];

    t2 - t1
}

fn minmax(arr: &Array1<f64>) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for value in arr.iter() {
        if value > &max {
            max = value.clone();
        }
        if value < &min {
            min = value.clone();
        }
    }
    (min, max)
}
fn minidx(arr: &Array1<f64>) -> (f64, usize) {
    let mut min = f64::MAX;
    let mut idx = 0;
    for id in 0..arr.len() {
        let value = arr[id];
        if value < min {
            min = value;
            idx = id;
        }
    }
    (min, idx)
}

fn minmax_rot(vel: &Array2<f64> ,pos: &Array2<f64> , rot_speed: f64, drum_center: &Array1<f64>) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let mut count = 0;
    for vel in vel.outer_iter(){
        let position = pos.slice(s![count, ..]).to_owned();
        let distance = ((position[0usize]-drum_center[0]) * (position[0usize]-drum_center[0])
        + (position[1usize]-drum_center[1]) * (position[1usize]-drum_center[1])
        + (position[2usize]-drum_center[2]) * (position[2usize]-drum_center[2])).sqrt();
        let alpha = ((position[0usize]-drum_center[0])/(position[2usize]-drum_center[2])).atan();
        let rot_vel_at_dist = distance  * (2.0 * std::f64::consts::PI  *rot_speed / 60.0);
        let rot_vec = array![alpha.cos()*rot_vel_at_dist,0.0,alpha.sin()*rot_vel_at_dist];
        // see notes at 13.feb 2021 (orange book)

        //here we can do it better! save these values as array and return them
        //later
        let part_rot_val = ((rot_vec[0]-vel[0])*(rot_vec[0]-vel[0])
        +(rot_vec[1]-vel[1])*(rot_vec[1]-vel[1])
        +(rot_vec[2]-vel[2])*(rot_vec[2]-vel[2]))
        .sqrt();

        // now finnalyy check the velocitys
        if part_rot_val < min{
            min = part_rot_val;
        }
        if part_rot_val > max{
            max = part_rot_val;
        }

    }
    (min,max)
}

/// calculates the cartesian norm of 3 velocity Vectors
/// representative for the velocity in x,y and z direction
fn norm_three(arr1: &Array2<f64>, arr2: &Array2<f64>, arr3: &Array2<f64>) -> Array2<f64> {
    let mut norm_array: Array2<f64> = Array2::zeros((arr1.shape()[0usize], arr1.shape()[1usize]));

    for idx in (0..norm_array.shape()[0usize]) {
        for idy in (0..norm_array.shape()[1usize]) {
            norm_array[[idx, idy]] = (arr1[[idx, idy]].powf(2.0)
                + arr2[[idx, idy]].powf(2.0)
                + arr3[[idx, idy]].powf(2.0))
            .sqrt()
        }
    }

    norm_array
}
