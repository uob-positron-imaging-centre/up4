extern crate ndarray;
//extern crate ndarray_linalg;
extern crate numpy;

use super::*;

pub trait Mixing: DataManager {
    fn lacey_mixing(
        &mut self,
        grid: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
        type_a: usize,
        type_b: usize,
    ) -> (Array1<f64>, Array1<f64>) {
        //read the number of timesteps inside this hdf5file
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        print_debug!("velocityfield: Initiation over, entering time loop");
        let mut mixing = ndarray::Array1::<f64>::zeros(*timesteps - 1);
        let mut time = ndarray::Array1::<f64>::zeros(*timesteps - 1);

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
            let particle_type = timestep_data.ptype();
            let particles = positions.len();
            // loop over all particles in this timestep, calculate the velocity vector and add it to the
            // vectorfield array
            print_debug!(
                "velocityfield: looping over particles form 0 to {}",
                particles
            );
            let mut type_a_grid = grid.new_zeros();
            let mut type_b_grid = grid.new_zeros();
            let mut type_counter: Vec<usize> = Vec::with_capacity(3);
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
                if particle_type[particle] as usize >= type_counter.len() {
                    type_counter.resize(particle_type[particle] as usize + 2, 0);
                }
                type_counter[particle_type[particle] as usize] += 1;
                print_debug!("Particle {} is valid", particle);
                let position = positions[particle];
                if position[0].is_nan() || position[1].is_nan() || position[2].is_nan() {
                    print_debug!("Position is NaN");
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
                if particle_type[particle] as usize == type_a {
                    type_a_grid.add_value(position, 1.0);
                } else if particle_type[particle] as usize == type_b {
                    type_b_grid.add_value(position, 1.0);
                }
            }

            let sum_grid = type_b_grid.get_data() + type_a_grid.get_data();
            type_a_grid.divide_by(&sum_grid);
            let concentration = type_a_grid.get_data();
            let mean_concentration =
                type_counter[type_a] as f64 / (type_counter[type_a] + type_counter[type_b]) as f64;
            let mut mixing_value = 0.0;
            for conc in concentration.iter() {
                if conc.is_nan() {
                    continue;
                }
                mixing_value += (conc - mean_concentration).powi(2);
            }
            mixing_value /= concentration.len() as f64 - 1.0;
            let sigma_max = (mean_concentration * (1.0 - mean_concentration)).sqrt();
            let sigma_min = (mean_concentration * (1.0 - mean_concentration)
                / concentration.len() as f64)
                .sqrt();
            let mixing_value =
                (sigma_max.powi(2) - mixing_value) / (sigma_max.powi(2) - sigma_min.powi(2));
            mixing[timestep] = mixing_value;
            time[timestep] = current_time;
            check_signals!();
        }
        (time, mixing)
    }

    fn concentration_field(
        &mut self,
        grid: Box<dyn GridFunctions3D>,
        selector: &ParticleSelector,
        type_a: usize,
        type_b: usize,
    ) -> Box<dyn GridFunctions3D> {
        //read the number of timesteps inside this hdf5file
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        let mut type_a_grid = grid.new_zeros();
        let mut type_b_grid = grid.new_zeros();
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
            let particle_type = timestep_data.ptype();
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
                if position[0].is_nan() || position[1].is_nan() || position[2].is_nan() {
                    print_debug!("Position is NaN");
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
                if particle_type[particle] as usize == type_a {
                    type_a_grid.add_value(position, 1.0);
                } else if particle_type[particle] as usize == type_b {
                    type_b_grid.add_value(position, 1.0);
                }
            }

            check_signals!();
        }
        let mut grid = grid.new_zeros();
        grid.set_data(type_a_grid.get_data() / (type_a_grid.get_data() + type_b_grid.get_data()));
        grid.set_weights(type_a_grid.get_weights() + type_b_grid.get_weights());
        grid
    }
}
