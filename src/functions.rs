pub mod extractions;
use crate::datamanager::DataManager;
//use crate::utilities::print_debug;
use crate::{check_signals, print_debug};
extern crate ndarray;
//extern crate ndarray_linalg;
extern crate numpy;

use crate::{
    grid::{GridFunctions3D, VectorGrid},
    ParticleSelector, Selector,
};
use ndarray::prelude::*;
pub trait Granular: DataManager {
    /// Calculate a 2D velocity vectorfield across `grid`, optionally normalising values to 1.
    /// The 3D data is projected in 2D according to `axis`.
    ///
    /// # Examples
    ///
    ///
    ///
    ///
    fn vectorfield(
        &mut self,
        gridbox: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
    ) -> VectorGrid {
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        let mut vectorgrid = VectorGrid::new(gridbox);
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                print_debug!("Timestep {} is not valid", timestep);
                continue;
            }
            print_debug!("Timestep {} is valid", timestep);
            let positions = timestep_data.position();
            let velocities = timestep_data.velocity();
            let particle_ids = timestep_data.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            let particles = positions.len();
            // loop over all particles in this timestep, calculate the velocity vector and add it to the
            // vectorfield array
            print_debug!(
                "velocityfield: looping over particles form 0 to {}",
                particles
            );
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
                print_debug!("Particle {} is valid", particle);
                let position = positions[particle];
                let velocity = velocities.slice(s![particle, ..]).to_owned();
                //reset the position. the lowest value should be at 0,0,0
                if !vectorgrid.is_inside(position) {
                    // the particle is out of the field of view
                    print_debug!("Particle {} is out of FoV", particle);
                    continue;
                }
                let velocity = vectorgrid.velocity_calculation(position, velocity);

                vectorgrid.data[0].add_value(position, velocity[0]);
                vectorgrid.data[1].add_value(position, velocity[1]);
                vectorgrid.data[2].add_value(position, velocity[2]);
            }
            check_signals!();
        }
        vectorgrid.divide_by_weight();
        vectorgrid
    }

    fn velocityfield(
        &mut self,
        grid: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
    ) -> Box<dyn GridFunctions3D> {
        //read the number of timesteps inside this hdf5file
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        let mut velocity_grid = grid.new_zeros();
        print_debug!("velocityfield: Initiation over, entering time loop");
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                print_debug!("Timestep {} is not valid", timestep);
                continue;
            }
            print_debug!("Timestep {} is valid", timestep);
            let positions = timestep_data.position();
            let velocities = timestep_data.velocity();
            let particle_ids = timestep_data.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            let particles = positions.len();
            // loop over all particles in this timestep, calculate the velocity vector and add it to the
            // vectorfield array
            print_debug!(
                "velocityfield: looping over particles form 0 to {}",
                particles
            );
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
                print_debug!("Particle {} is valid", particle);
                let position = positions[particle];

                let velocity = velocities.slice(s![particle, ..]).to_owned();
                //reset the position. the lowest value should be at 0,0,0

                //velocities
                let vx: f64 = velocity[0];
                let vy: f64 = velocity[1];
                let vz: f64 = velocity[2];
                let abs_vel = (vx.powi(2) + vy.powi(2) + vz.powi(2)).sqrt();

                if abs_vel.is_nan() {
                    print_debug!("Particle {} has no velocity", particle);
                    continue;
                }

                if !grid.is_inside(position) {
                    // the particle is out of the field of view
                    print_debug!("Particle {} is out of FoV", particle);
                    continue;
                }
                print_debug!("Particle {} is in the grid", particle);
                print_debug!("Cell ids: {:?}", grid.cell_id(position));
                print_debug!(
                    "Grid Positions \n{:?},\n{:?},\n{:?}",
                    grid.get_xpositions(),
                    grid.get_ypositions(),
                    grid.get_zpositions()
                );
                velocity_grid.add_value(position, abs_vel);
            }
            // checking for kill signals after each timestep
            check_signals!();
        }
        velocity_grid.divide_by_weight();
        velocity_grid
    }

    fn numberfield(
        &mut self,
        grid: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
    ) -> Box<dyn GridFunctions3D> {
        //read the number of timesteps inside this hdf5file
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        let mut number_grid = grid.new_zeros();
        print_debug!("velocityfield: Initiation over, entering time loop");
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                print_debug!("Timestep {} is not valid", timestep);
                continue;
            }
            print_debug!("Timestep {} is valid", timestep);
            let positions = timestep_data.position();

            let particle_ids = timestep_data.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            let particles = positions.len();
            // loop over all particles in this timestep, calculate the velocity vector and add it to the
            // vectorfield array
            print_debug!(
                "velocityfield: looping over particles form 0 to {}",
                particles
            );

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

                print_debug!("Particle {} is valid", particle);
                let position = positions[particle];
                if !grid.is_inside(position) {
                    // the particle is out of the field of view
                    print_debug!("Particle {} is out of FoV", particle);
                    continue;
                }
                print_debug!("Particle {} is in the grid", particle);
                print_debug!("Cell ids: {:?}", grid.cell_id(position));
                print_debug!(
                    "Grid Positions \n{:?},\n{:?},\n{:?}",
                    grid.get_xpositions(),
                    grid.get_ypositions(),
                    grid.get_zpositions()
                );
                //here bug already

                number_grid.add_value(position, 1.0);
            }
            // checking for kill signals after each timestep
            check_signals!();
        }
        number_grid
    }

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
    }

    /// Return the dispersion of the particles in the system.
    /// See Martin, T. W., J. P. K. Seville, and D. J. Parker. "A general method for quantifying dispersion in multiscale systems using trajectory analysis."
    ///
    /// parameters
    /// ----------
    /// grid : up4.Grid
    ///    Grid class containing the grid layout.
    /// time_for_dispersion : float
    ///   Time for which the dispersion is calculated.
    ///
    /// returns
    /// -------
    /// up4.Grid
    ///   Grid class containing the dispersion field.
    ///
    ///
    fn dispersion(
        &mut self,
        grid: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
        time_for_dispersion: f64,
    ) -> Box<dyn GridFunctions3D> {
        let global_stats = self.global_stats();
        let timesteps = global_stats.timesteps();
        // Allocate arrays needed for calculation
        // those arrays are needed for the calculation of the variance with a special algorithm
        let mut squared_sum_x = grid.get_data().clone();
        let mut squared_sum_y = grid.get_data().clone();
        let mut squared_sum_z = grid.get_data().clone();
        let mut sum_x = grid.get_data().clone();
        let mut sum_y = grid.get_data().clone();
        let mut sum_z = grid.get_data().clone();
        let mut num_counts = grid.get_data().clone();
        let mut dispersion_grid = grid.new_zeros();
        print_debug!("Dispersion: Initiation over, entering time loop");
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep).clone();
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                print_debug!("Timestep {} is not valid", timestep);
                continue;
            }
            print_debug!("Timestep {} is valid", timestep);
            let positions = timestep_data.position();

            let particle_ids = timestep_data.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            let particles = positions.len();
            print_debug!("Dispersion: looping over particles form 0 to {}", particles);
            let next_timestep =
                global_stats.timestep_at_seconds(current_time + time_for_dispersion);
            let next_timestep = match next_timestep {
                Ok(x) => x,
                Err(_) => {
                    print_debug!(
                        "Dispersion: No next timestep found for time {}",
                        current_time + time_for_dispersion
                    );
                    continue;
                }
            };
            print_debug!("Next timestep is {}", next_timestep);
            let timestep_future = self.get_timestep_unbuffered(next_timestep);
            print_debug!("extracting position");
            let position_future = timestep_future.position();
            print_debug!("Starting particle loop");
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
                if !grid.is_inside(positions[particle]) {
                    // the particle is out of the field of view
                    print_debug!("Particle {} is out of FoV", particle);
                    continue;
                }
                let cell_id = grid.cell_id(positions[particle]);
                let position_future_particle = position_future[particle];
                squared_sum_x[cell_id] += position_future_particle[0] * position_future_particle[0];
                squared_sum_y[cell_id] += position_future_particle[1] * position_future_particle[1];
                squared_sum_z[cell_id] += position_future_particle[2] * position_future_particle[2];
                sum_x[cell_id] += position_future_particle[0];
                sum_y[cell_id] += position_future_particle[1];
                sum_z[cell_id] += position_future_particle[2];
                num_counts[cell_id] += 1.0;
            }
            print_debug!("Dispersion: looping over all cells");
            let cells = grid.get_cells();
            // for loop over all 3 dimensions to get to each cell
            for x in 0..cells[0] {
                for y in 0..cells[1] {
                    for z in 0..cells[2] {
                        let n = num_counts[[x, y, z]];
                        print_debug!("Dispersion: cell {:?} has {} particles", [x, y, z], n);
                        if n > 0.0 {
                            let dispersion_x = squared_sum_x[[x, y, z]] / n
                                - sum_x[[x, y, z]] * sum_x[[x, y, z]] / n / n;
                            let dispersion_y = squared_sum_y[[x, y, z]] / n
                                - sum_y[[x, y, z]] * sum_y[[x, y, z]] / n / n;
                            let dispersion_z = squared_sum_z[[x, y, z]] / n
                                - sum_z[[x, y, z]] * sum_z[[x, y, z]] / n / n;
                            dispersion_grid.add_to_cell(
                                [x, y, z],
                                (dispersion_x + dispersion_y + dispersion_z) * n / (n - 1.0),
                            );
                        }
                    }
                }
            }
        }
        dispersion_grid
    }

    /// Return the dispersion of the particles in the system.
    /// See Martin, T. W., J. P. K. Seville, and D. J. Parker. "A general method for quantifying dispersion in multiscale systems using trajectory analysis."
    ///
    /// parameters
    /// ----------
    /// grid : up4.Grid
    ///    Grid class containing the grid layout.
    /// time_for_dispersion : float
    ///   Time for which the dispersion is calculated.
    ///
    /// returns
    /// -------
    /// up4.Grid
    ///   Grid class containing the dispersion field.
    ///
    ///
    fn dispersion_new(
        &mut self,
        grid: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
        time_for_dispersion: f64,
    ) -> Box<dyn GridFunctions3D> {
        let global_stats = self.global_stats();
        let timesteps = global_stats.timesteps();
        self.setup_buffer(); // add another buffer to the system
                             // Allocate arrays needed for calculation
                             // those arrays are needed for the calculation of the variance with a special algorithm
        let mut squared_sum_x = grid.get_data().clone();
        let mut squared_sum_y = grid.get_data().clone();
        let mut squared_sum_z = grid.get_data().clone();
        let mut sum_x = grid.get_data().clone();
        let mut sum_y = grid.get_data().clone();
        let mut sum_z = grid.get_data().clone();
        let mut num_counts = grid.get_data().clone();
        let mut dispersion_grid = grid.new_zeros();
        print_debug!("Dispersion: Initiation over, entering time loop");
        for timestep in 0..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep).clone();
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                print_debug!("Timestep {} is not valid", timestep);
                continue;
            }
            print_debug!("Timestep {} is valid", timestep);
            let positions = timestep_data.position();

            let particle_ids = timestep_data.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            let particles = positions.len();
            print_debug!("Dispersion: looping over particles form 0 to {}", particles);
            let next_timestep =
                global_stats.timestep_at_seconds(current_time + time_for_dispersion);
            let next_timestep = match next_timestep {
                Ok(x) => x,
                Err(_) => {
                    print_debug!(
                        "Dispersion: No next timestep found for time {}",
                        current_time + time_for_dispersion
                    );
                    continue;
                }
            };
            print_debug!("Next timestep is {}", next_timestep);
            let timestep_future = self.get_timestep_buffer(next_timestep, 0);
            print_debug!("extracting position");
            let position_future = timestep_future.position();
            print_debug!("Starting particle loop");
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
                if !grid.is_inside(positions[particle]) {
                    // the particle is out of the field of view
                    print_debug!("Particle {} is out of FoV", particle);
                    continue;
                }
                let cell_id = grid.cell_id(positions[particle]);
                let position_future_particle = position_future[particle];
                squared_sum_x[cell_id] += position_future_particle[0] * position_future_particle[0];
                squared_sum_y[cell_id] += position_future_particle[1] * position_future_particle[1];
                squared_sum_z[cell_id] += position_future_particle[2] * position_future_particle[2];
                sum_x[cell_id] += position_future_particle[0];
                sum_y[cell_id] += position_future_particle[1];
                sum_z[cell_id] += position_future_particle[2];
                num_counts[cell_id] += 1.0;
            }
            print_debug!("Dispersion: looping over all cells");
            let cells = grid.get_cells();
            // for loop over all 3 dimensions to get to each cell
            for x in 0..cells[0] {
                for y in 0..cells[1] {
                    for z in 0..cells[2] {
                        let n = num_counts[[x, y, z]];
                        print_debug!("Dispersion: cell {:?} has {} particles", [x, y, z], n);
                        if n > 0.0 {
                            let dispersion_x = squared_sum_x[[x, y, z]] / n
                                - sum_x[[x, y, z]] * sum_x[[x, y, z]] / n / n;
                            let dispersion_y = squared_sum_y[[x, y, z]] / n
                                - sum_y[[x, y, z]] * sum_y[[x, y, z]] / n / n;
                            let dispersion_z = squared_sum_z[[x, y, z]] / n
                                - sum_z[[x, y, z]] * sum_z[[x, y, z]] / n / n;
                            dispersion_grid.add_to_cell(
                                [x, y, z],
                                (dispersion_x + dispersion_y + dispersion_z) * n / (n - 1.0),
                            );
                        }
                    }
                }
            }
        }
        dispersion_grid
    }

    /// Calculate the mean velocity of the valid particles within the system.
    ///
    /// # Examples
    ///
    /// Calculate the mean velocity of all particles.
    ///’’’
    ///mean_velocity = data.mean_velocity_showcase(particleselector)
    ///'''
    fn grid_test(&mut self, _selector: &ParticleSelector, _grid: Box<dyn GridFunctions3D>) -> f64 {
        unimplemented!()
    } //end mean velocity

    /// Calculate the mean velocity of the valid particles within the system.
    ///
    /// # Examples
    ///
    /// Calculate the mean velocity of all particles.
    ///’’’
    ///mean_velocity = data.mean_velocity(particleselector)
    ///'''
    fn mean_velocity(&mut self, _selector: &ParticleSelector) -> f64 {
        let global_stats = self.global_stats();
        global_stats.velocity_mag()[1]
    } //end mean velocity
} //End Granular trait

impl<T> Granular for T where T: DataManager {}
impl<T> extractions::Extraction for T where T: DataManager {}
