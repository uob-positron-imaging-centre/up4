pub mod fields;
use crate::datamanager::DataManager;
//use crate::utilities::print_debug;
use crate::base::*;
use crate::{check_signals, print_debug, print_warning};
extern crate ndarray;
//extern crate ndarray_linalg;
extern crate numpy;

use crate::base::{Grid1D, Grid2D, Grid3D, GridType};
use core::panic;
use ndarray::prelude::*;
pub trait Granular: DataManager {
    /// Calculate a 2D velocity vectorfield across `grid`, optionally normalising values to 1.
    /// The 3D data is projected in 2D according to `axis`.
    ///
    /// # Examples
    ///
    /*
    fn vectorfield(
        &mut self,
        gridbox: Box<dyn GridFunctions>,
        selector: &ParticleSelector,
        norm: bool, //normalise the size of the vectors
        axis: usize,
    ) -> (ArrayD<Vec<f64>>) {
        //read the number of timesteps inside this hdf5file
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        let grid = gridbox;
        let x = grid.cell_id(vec![10., 10., 10.]);
        //initiate needed 2d arrays:
        print_debug!("vectorfield: Initialising array");
        let mut v_x_grid = grid.data_array::<f64>();
        let mut v_z_grid = grid.data_array::<f64>();
        print_debug!("Initialised vec_field with shape: {:?}", v_z_grid.shape());
        //array to count how many particles found per cell
        let mut n_x_grid = grid.data_array::<f64>();
        let mut n_z_grid = grid.data_array::<f64>();

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
                panic!(
                    "variable axis in vectorfield must be between 0 and 2 ! Currently it is {:?}",
                    axis,
                )
            }
        }
        print_debug!("Axis choosen: {:?}, {:?}", first_axis, sec_axis);
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                continue;
            }
            let positions = timestep_data.position();
            let velocities = timestep_data.velocity();
            let particle_ids = timestep_data.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            let particles = positions.len() / 3;
            // loop over all particles in this timestep, calculate the velocity vector and add it to the
            // vectorfield array
            for particle in 0..particles {
                if !selector.is_valid(
                    rad_array[particle],
                    clouds[particle],
                    density[particle],
                    particle_ids[particle] as usize,
                ) {
                    print_debug!("Particle {} is not valid", particle);
                    continue;
                }
                let position = positions.slice(s![particle, ..]).to_owned();
                let velocity = velocities.slice(s![particle, ..]).to_owned();
                //reset the position. the lowest value should be at 0,0,0

                let x_pos: f64 = position[first_axis];
                let y_pos: f64 = position[sec_axis];

                //velocities
                let vx: f64 = velocity[first_axis];
                let vz: f64 = velocity[sec_axis];

                if !grid.is_inside(vec![x_pos, y_pos]) {
                    // the particle is out of the field of view
                    print_debug!("Particle {} is out of FoV", particle);
                    continue;
                }
                // find the cell indice where particle is right now
                let cell_ids = grid.cell_id(vec![x_pos, y_pos]);

                let i = cell_ids[0];
                let k = cell_ids[1];
                print_debug!("Particle is in the grid, cells: {},{}", i, k);
                v_x_grid[[k, i]] = v_x_grid[[k, i]] + vx;
                v_z_grid[[k, i]] = v_z_grid[[k, i]] + vz;

                n_x_grid[[k, i]] = n_x_grid[[k, i]] + 1.0;
                n_z_grid[[k, i]] = n_z_grid[[k, i]] + 1.0;
            }
            // checking for kill signals after each timestep
            check_signals!();
        }

        v_x_grid = v_x_grid / &n_x_grid;
        v_z_grid = v_z_grid / &n_z_grid;
        let (sx, sy) = meshgrid(
            Array::linspace(grid.xlim().0, grid.xlim().1, grid.cells()[0usize] as usize),
            Array::linspace(grid.ylim().0, grid.ylim().1, grid.cells()[1usize] as usize),
        );
        if norm {
            let norm_arr = norm_two(&v_x_grid, &v_z_grid).to_owned();
            v_x_grid = v_x_grid / &norm_arr;
            v_z_grid = v_z_grid / &norm_arr;
        }

        (v_x_grid, v_z_grid, sx, sy)
    } //end vectorfield
    */

    /// Calculate the mean velocity of the valid particles within the system.
    ///
    /// # Examples
    ///
    /// Calculate the mean velocity of all particles.
    ///’’’
    ///mean_velocity = data.mean_velocity_showcase(particleselector)
    ///'''
    fn mean_velocity_showcase(&mut self, selector: &ParticleSelector) -> f64 {
        let global_stats = self.global_stats();
        let timesteps = global_stats.timesteps();
        let mut mean_velocity = 0.0;
        let mut num_counts = 0.0;
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                continue;
            }
            let velocities = timestep_data.velocity();
            for vel in velocities.outer_iter() {
                let velocity = (vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]).sqrt();
                if f64::is_nan(velocity) {
                    continue;
                }
                mean_velocity += velocity;
                num_counts += 1.0;
            }
            // checking for kill signals
            check_signals!()
        }
        mean_velocity /= num_counts;

        mean_velocity
    } //end mean velocity

    /// Calculate the mean velocity of the valid particles within the system.
    ///
    /// # Examples
    ///
    /// Calculate the mean velocity of all particles.
    ///’’’
    ///mean_velocity = data.mean_velocity(particleselector)
    ///'''
    fn mean_velocity(&mut self, selector: &ParticleSelector) -> f64 {
        let global_stats = self.global_stats();
        global_stats.velocity_mag()[1]
    } //end mean velocity
} //End Granular trait

impl<T> Granular for T where T: DataManager {}

fn derivative(x: Array1<f64>, y: Array1<f64>, avg: usize) -> (Array1<f64>, Array1<f64>) {
    if x.len() != y.len() {
        panic!("X and y should have the same len");
    }

    let mut result_x = Array1::<f64>::zeros(x.len());
    let mut diffy = Array1::<f64>::zeros(y.len());
    let mut diffx = Array1::<f64>::zeros(x.len());
    for id in 0..x.len() - avg {
        diffx[id] = x[id + avg] - x[id];
        diffy[id] = y[id + avg] - y[id];
        result_x[id] = 0.5 * (x[id + avg] + x[id]);
    }
    let result_y = diffy / diffx; //Array1::<f64>::zeros(y.len());

    (result_x, result_y)
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
    //-1!("{:?}",ret_val);
    ret_val
}

pub fn meshgrid(
    x: ndarray::Array1<f64>,
    y: ndarray::Array1<f64>,
) -> (ndarray::Array2<f64>, ndarray::Array2<f64>) {
    let mut xx = ndarray::Array2::<f64>::zeros((x.len(), y.len()));
    let mut yy = ndarray::Array2::<f64>::zeros((x.len(), y.len()));

    for idx in 0..x.len() {
        for idy in 0..y.len() {
            xx[[idx, idy]] = x[idx];
            yy[[idx, idy]] = y[idy];
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
            norm_array[[idx, idy]] =
                (arr1[[idx, idy]].powf(2.0) + arr2[[idx, idy]].powf(2.0)).sqrt()
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
    let vec = file.member_names().expect("");
    for x in file.member_names().expect("") {
        if x.contains("timestep") {
            timesteps += 1;
        }
    }
    timesteps
}

fn get_dt(file: &hdf5::File) -> f64 {
    let t1 = file
        .group("timestep 0")
        .expect("")
        .dataset("time")
        .expect("")
        .read_raw::<f64>()
        .expect("")[0];
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

fn minmax_rot(
    vel: &Array2<f64>,
    pos: &Array2<f64>,
    rot_speed: f64,
    drum_center: &Array1<f64>,
) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let mut count = 0;
    for vel in vel.outer_iter() {
        let position = pos.slice(s![count, ..]).to_owned();
        let distance = ((position[0usize] - drum_center[0]) * (position[0usize] - drum_center[0])
            + (position[1usize] - drum_center[1]) * (position[1usize] - drum_center[1])
            + (position[2usize] - drum_center[2]) * (position[2usize] - drum_center[2]))
            .sqrt();
        let alpha =
            ((position[0usize] - drum_center[0]) / (position[2usize] - drum_center[2])).atan();
        let rot_vel_at_dist = distance * (2.0 * std::f64::consts::PI * rot_speed / 60.0);
        let rot_vec = array![
            alpha.cos() * rot_vel_at_dist,
            0.0,
            alpha.sin() * rot_vel_at_dist
        ];
        // see notes at 13.feb 2021 (orange book)

        //here we can do it better! save these values as array and return them
        //later
        let part_rot_val = ((rot_vec[0] - vel[0]) * (rot_vec[0] - vel[0])
            + (rot_vec[1] - vel[1]) * (rot_vec[1] - vel[1])
            + (rot_vec[2] - vel[2]) * (rot_vec[2] - vel[2]))
            .sqrt();

        // now finnalyy check the velocitys
        if part_rot_val < min {
            min = part_rot_val;
        }

        if part_rot_val > max {
            max = part_rot_val;
        }
    }
    (min, max)
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
