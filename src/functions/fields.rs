extern crate ndarray;
//extern crate ndarray_linalg;
extern crate numpy;

use super::*;
use core::panic;
use ndarray::prelude::*;

pub trait Fields {
    fn occupancyfield(
        filename: &str,          //filename of hdf5 file
        cells: Array1<f64>,      //number of cells to store vec-data
        min_time: f64,           //where to start the averaging
        max_time: f64,           //where to end the averaging
        dimensions: Array2<f64>, // Region where to look at, rest ignored
        radius: Array1<f64>,     // include a radius
        particle_id: Array1<i64>,
        axis: usize,
    ) -> (Array1<f64>, Array1<f64>, Array2<f64>) {
        // Opening hdf5 file
        let file =
            hdf5::File::open(&filename).expect(&format!("Can not open HDF5 file {:?}", &filename));
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
        let mut max_array = array.slice(s![1, ..]).to_owned();
        //let cells_int =
        //before going through timestep, implement:
        // dimension check?

        if dimensions.slice(s![0, ..]).to_owned()[0usize] > min_array[0usize] {
            min_array[0usize] = dimensions.slice(s![0, ..]).to_owned()[0usize]
        }
        if dimensions.slice(s![0, ..]).to_owned()[1usize] > min_array[1usize] {
            min_array[1usize] = dimensions.slice(s![0, ..]).to_owned()[1usize]
        }
        if dimensions.slice(s![0, ..]).to_owned()[2usize] > min_array[2usize] {
            min_array[2usize] = dimensions.slice(s![0, ..]).to_owned()[2usize]
        }

        if dimensions.slice(s![1, ..]).to_owned()[0usize] < max_array[0usize] {
            max_array[0usize] = dimensions.slice(s![1, ..]).to_owned()[0usize]
        }
        if dimensions.slice(s![1, ..]).to_owned()[1usize] < max_array[1usize] {
            max_array[1usize] = dimensions.slice(s![1, ..]).to_owned()[1usize]
        }
        if dimensions.slice(s![1, ..]).to_owned()[2usize] < max_array[2usize] {
            max_array[2usize] = dimensions.slice(s![1, ..]).to_owned()[2usize]
        }

        let cell_size: Array1<f64> = array![
            (max_array[0usize] - min_array[0usize]) / (cells[0usize]),
            (max_array[1usize] - min_array[1usize]) / (cells[1usize]),
            (max_array[2usize] - min_array[2usize]) / (cells[2usize])
        ];

        //array to count how many particles found per cell
        let mut n_grid = ndarray::Array2::<f64>::zeros((
            cells[2usize].floor() as usize,
            cells[0usize].floor() as usize,
        ));
        let mut t_grid = ndarray::Array2::<f64>::zeros((
            cells[2usize].floor() as usize,
            cells[0usize].floor() as usize,
        ));
        let mut complete_time = 0.0;

        // find the two axis indizes which we want to "see"
        let mut first_axis = 4;
        let mut sec_axis = 4;
        for x in 0..3 {
            if x == axis {
                continue;
            };
            if first_axis == 4 {
                first_axis = x;
            } else if sec_axis == 4 {
                sec_axis = x;
            } else {
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
                &name, &filename
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
            let dt = get_dt(&file);
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

                let pos_abs = position;

                if pos_abs[0] < dimensions[[0usize, 0]]
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
                let vel: f64 = (velocity[first_axis] * velocity[first_axis]
                    + velocity[sec_axis] * velocity[sec_axis])
                    .sqrt();
                //calculate the time this particle spent in the cell
                let time_spent;
                if vel <= cell_size[first_axis] / dt {
                    time_spent = dt;
                } else {
                    time_spent = cell_size[first_axis] / vel;
                }

                // check if indexes are higher then maximum
                if i >= cells[first_axis] as usize || k >= cells[sec_axis] as usize {
                    continue;
                }
                t_grid[[k, i]] = t_grid[[k, i]] + time_spent;
                complete_time += time_spent;
            }
        }
        file.close();

        let sx = Array::linspace(
            min_array[first_axis],
            max_array[first_axis],
            cells[first_axis] as usize,
        );
        let sy = Array::linspace(
            min_array[sec_axis],
            max_array[sec_axis],
            cells[sec_axis] as usize,
        );
        (sx, sy, &t_grid / complete_time)
    }

    fn velocityfield(
        filename: &str,          //filename of hdf5 file
        cells: Array1<f64>,      //number of cells to store vec-data
        min_time: f64,           //where to start the averaging
        max_time: f64,           //where to end the averaging
        dimensions: Array2<f64>, // Region where to look at, rest ignored
        radius: Array1<f64>,     // include a radius
        particle_id: Array1<i64>,
        axis: usize,
    ) -> (Array1<f64>, Array1<f64>, Array2<f64>) {
        // Opening hdf5 file
        let file =
            hdf5::File::open(&filename).expect(&format!("Can not open HDF5 file {:?}", &filename));
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
        let mut max_array = array.slice(s![1, ..]).to_owned();

        //let cells_int =
        //before going through timestep, implement:
        // dimension check?

        if true {
            //dimensions.slice(s![0, ..]).to_owned()[0usize] > min_array[0usize]  {
            min_array[0usize] = dimensions.slice(s![0, ..]).to_owned()[0usize]
        }
        if true {
            //dimensions.slice(s![0, ..]).to_owned()[1usize] > min_array[1usize]  {
            min_array[1usize] = dimensions.slice(s![0, ..]).to_owned()[1usize]
        }
        if true {
            //dimensions.slice(s![0, ..]).to_owned()[2usize] > min_array[2usize]  {
            min_array[2usize] = dimensions.slice(s![0, ..]).to_owned()[2usize]
        }

        if true {
            //dimensions.slice(s![1, ..]).to_owned()[0usize] < max_array[0usize]  {
            max_array[0usize] = dimensions.slice(s![1, ..]).to_owned()[0usize]
        }
        if true {
            //dimensions.slice(s![1, ..]).to_owned()[1usize] < max_array[1usize]  {
            max_array[1usize] = dimensions.slice(s![1, ..]).to_owned()[1usize]
        }
        if true {
            // dimensions.slice(s![1, ..]).to_owned()[2usize] < max_array[2usize]  {
            max_array[2usize] = dimensions.slice(s![1, ..]).to_owned()[2usize]
        }

        let cell_size: Array1<f64> = array![
            (max_array[0usize] - min_array[0usize]) / (cells[0usize]),
            (max_array[1usize] - min_array[1usize]) / (cells[1usize]),
            (max_array[2usize] - min_array[2usize]) / (cells[2usize])
        ];

        //array to count how many particles found per cell
        let mut n_grid = ndarray::Array2::<f64>::zeros((
            cells[2usize].floor() as usize,
            cells[0usize].floor() as usize,
        ));
        let mut v_grid = ndarray::Array2::<f64>::zeros((
            cells[2usize].floor() as usize,
            cells[0usize].floor() as usize,
        ));

        // find the two axis indizes which we want to "see"
        let mut first_axis = 4;
        let mut sec_axis = 4;
        for x in 0..3 {
            if x == axis {
                continue;
            };
            if first_axis == 4 {
                first_axis = x;
            } else if sec_axis == 4 {
                sec_axis = x;
            } else {
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
                &name, &filename
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

                let pos_abs = position;

                if pos_abs[0] < dimensions[[0usize, 0]]
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
                if i >= cells[first_axis] as usize || k >= cells[sec_axis] as usize {
                    continue;
                }
                n_grid[[k, i]] = n_grid[[k, i]] + 1.0;
                v_grid[[k, i]] = v_grid[[k, i]] + norm_l2(&velocity).abs();
            }
        }
        file.close();
        // make arrays for position of each cell in each diemnsion

        let sx = Array::linspace(
            min_array[first_axis],
            max_array[first_axis],
            cells[first_axis] as usize,
        );
        let sy = Array::linspace(
            min_array[sec_axis],
            max_array[sec_axis],
            cells[sec_axis] as usize,
        );

        (sx, sy, v_grid / n_grid)
    }

    fn numberfield(
        filename: &str,          //filename of hdf5 file
        cells: Array1<f64>,      //number of cells to store vec-data
        min_time: f64,           //where to start the averaging
        max_time: f64,           //where to end the averaging
        dimensions: Array2<f64>, // Region where to look at, rest ignored
        radius: Array1<f64>,     // include a radius
        particle_id: Array1<i64>,
        axis: usize,
    ) -> (Array1<f64>, Array1<f64>, Array2<f64>) {
        // Opening hdf5 file
        let file =
            hdf5::File::open(&filename).expect(&format!("Can not open HDF5 file {:?}", &filename));
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
        let mut max_array = array.slice(s![1, ..]).to_owned();

        //let cells_int =
        //before going through timestep, implement:
        // dimension check?

        if dimensions.slice(s![0, ..]).to_owned()[0usize] > min_array[0usize] {
            min_array[0usize] = dimensions.slice(s![0, ..]).to_owned()[0usize]
        }
        if dimensions.slice(s![0, ..]).to_owned()[1usize] > min_array[1usize] {
            min_array[1usize] = dimensions.slice(s![0, ..]).to_owned()[1usize]
        }
        if dimensions.slice(s![0, ..]).to_owned()[2usize] > min_array[2usize] {
            min_array[2usize] = dimensions.slice(s![0, ..]).to_owned()[2usize]
        }

        if dimensions.slice(s![1, ..]).to_owned()[0usize] < max_array[0usize] {
            max_array[0usize] = dimensions.slice(s![1, ..]).to_owned()[0usize]
        }
        if dimensions.slice(s![1, ..]).to_owned()[1usize] < max_array[1usize] {
            max_array[1usize] = dimensions.slice(s![1, ..]).to_owned()[1usize]
        }
        if dimensions.slice(s![1, ..]).to_owned()[2usize] < max_array[2usize] {
            max_array[2usize] = dimensions.slice(s![1, ..]).to_owned()[2usize]
        }

        let cell_size: Array1<f64> = array![
            (max_array[0usize] - min_array[0usize]) / (cells[0usize]),
            (max_array[1usize] - min_array[1usize]) / (cells[1usize]),
            (max_array[2usize] - min_array[2usize]) / (cells[2usize])
        ];

        //array to count how many particles found per cell
        let mut n_grid = ndarray::Array2::<f64>::zeros((
            cells[2usize].floor() as usize,
            cells[0usize].floor() as usize,
        ));

        // find the two axis indizes which we want to "see"
        let mut first_axis = 4;
        let mut sec_axis = 4;
        for x in 0..3 {
            if x == axis {
                continue;
            };
            if first_axis == 4 {
                first_axis = x;
            } else if sec_axis == 4 {
                sec_axis = x;
            } else {
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
                &name, &filename
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

                if pos_abs[0] < dimensions[[0usize, 0]]
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
                if i >= cells[first_axis] as usize || k >= cells[sec_axis] as usize {
                    continue;
                }
                n_grid[[k, i]] = n_grid[[k, i]] + 1.0;
            }
        }
        file.close();
        let sx = Array::linspace(
            min_array[first_axis],
            max_array[first_axis],
            cells[first_axis] as usize,
        );
        let sy = Array::linspace(
            min_array[sec_axis],
            max_array[sec_axis],
            cells[sec_axis] as usize,
        );

        (sx, sy, n_grid)
    }
}

impl<T> Fields for T where T: DataManager {}
