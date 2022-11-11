use crate::datamanager::DataManager;
//use crate::utilities::print_debug;
use crate::check_signals;
extern crate ndarray;
//extern crate ndarray_linalg;
extern crate numpy;
use core::panic;
use ndarray::prelude::*;

pub trait Extraction: DataManager {
    /// Extract the particle information over a given time period
    fn extract(&mut self, particle_id: usize, timestep: (usize, usize)) -> ndarray::Array2<f64> {
        let global_stats = self.global_stats();
        let timesteps: &usize = global_stats.timesteps();
        if &timestep.0 >= timesteps || &timestep.1 >= timesteps {
            panic!("Timestep out of bounds");
        }
        let mut return_data: ndarray::Array2<f64> =
            ndarray::Array2::zeros((timestep.1 - timestep.0, 7));
        for (id, timestep) in (timestep.0..timestep.1).enumerate() {
            let mut mut_return = return_data.slice_mut(s![id, ..]);
            let timestep_data = self.get_timestep(timestep);
            let current_time = *timestep_data.time();
            let positions = timestep_data.position();
            let velocities = timestep_data.velocity();
            let position = positions[particle_id];
            let velocity = velocities.slice(s![particle_id, ..]).to_owned();
            let x = array![
                current_time,
                position[0],
                position[1],
                position[2],
                velocity[0],
                velocity[1],
                velocity[2],
            ];
            mut_return.assign(&x);
            check_signals!();
        }

        return_data
    }
}
