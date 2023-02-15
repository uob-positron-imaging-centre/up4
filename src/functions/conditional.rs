use super::*;
pub trait Conditional: DataManager {
    fn circulation_time(
        &mut self,
        selector: &ParticleSelector,
        axis: usize,
        boundary_position: f64,
    ) -> Vec<f64> {
        //read the number of timesteps inside this hdf5file

        let global_stats = self.global_stats();
        let dim = global_stats.dimensions();
        if boundary_position > dim[[1, axis]] || boundary_position < dim[[0, axis]] {
            panic!("Boundary position is outside of the system");
        }
        let particle_number = global_stats.nparticles();
        let timesteps: &usize = global_stats.timesteps();
        print_debug!("velocityfield: Initiation over, entering time loop");
        let mut circulation_time = Vec::<f64>::new();
        let mut start_flag_array = Array1::<usize>::zeros(particle_number + 1);
        let mut time_flag_array = Array1::<f64>::zeros(particle_number + 1);

        let mut old_timestep = self.get_timestep(0).to_owned();
        for timestep in 1..timesteps - 1 {
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            // check if timestep is in the timeframe given
            if !selector.timestep_valid(current_time) {
                print_debug!("Timestep {} is not valid", timestep);
                continue;
            }
            print_debug!("Timestep {} is valid", timestep);
            let positions = timestep_data.position();
            let old_positions = old_timestep.position();
            let particle_ids = timestep_data.particleid();
            let old_particle_ids = old_timestep.particleid();
            let rad_array = timestep_data.radius();
            let clouds = timestep_data.clouds();
            let density = timestep_data.density();
            //let particle_type = timestep_data.ptype();
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
                if particle_ids[particle] != old_particle_ids[particle] {
                    print_debug!("Particle {} is not valid", particle);
                    continue;
                }
                let pos = positions[particle];
                let old_pos = old_positions[particle];
                // check if particle crossed boundary
                if (pos[axis] - boundary_position) * (old_pos[axis] - boundary_position) < 0.0 {
                    // check if particle is already in the list
                    if start_flag_array[particle] == 0 {
                        start_flag_array[particle] = 1;
                        time_flag_array[particle] = current_time;
                    } else if start_flag_array[particle] == 1 {
                        start_flag_array[particle] = 2;
                    } else {
                        circulation_time.push(current_time - time_flag_array[particle]);
                        start_flag_array[particle] = 0;
                    }
                }
            }

            old_timestep = timestep_data.to_owned();
            check_signals!()
        }
        circulation_time
    }
}
