//! Data Managment Module. Organizes file access.
//use crate::functions::*;
//use crate::functions::fields::*;
use super::functions::conditional::Conditional;
use super::functions::extractions::Extraction;
use super::functions::mixing::Mixing;
use super::functions::Granular;
use crate::{print_warning, types::*};
use derive_getters::Getters;
use ndarray::prelude::*;

pub mod pdata;
pub use pdata::PData;
pub mod tdata;
pub use tdata::TData;
mod utilities;
/// Defines main access functions for post processing functions.
pub trait DataManager {
    /// return the pointer to `Timestep` at the step `timestep`
    fn get_timestep(&mut self, timestep: usize) -> &Timestep;

    /// Return `GlobalStats` for global system information
    fn global_stats(&self) -> GlobalStats;

    /// Development function
    fn stats(&self);

    /// return a timetsep that is not in the buffer
    fn get_timestep_unbuffered(&self, timestep: usize) -> Timestep;

    /// setup a new buffer
    fn setup_buffer(&mut self);

    /// read from other buffer then the main one
    fn get_timestep_buffer(&mut self, timestep: usize, buffer: usize) -> &Timestep;

    /// Return a string containung information about the dataset
    fn info(&self) -> Result<String, &'static str>;

    /// set rotation angle
    fn set_rotation_angle(&mut self, angle: f64, axis: f64);

    /// set rotation anker, the point around which the rotation is performed
    /// default is zero
    fn set_rotation_anker(&mut self, point: [f64; 3]);
}

pub trait Manager: DataManager + Granular + Extraction + Mixing + Conditional {}

/// Data-struct containing all necessery information for a timestep
#[derive(Debug, Default, Getters, Clone)]
pub struct Timestep {
    time: f64,
    position: Array1<Position>,
    velocity: Array2<f64>,
    radius: Array1<f64>,
    particleid: Array1<f64>,
    clouds: Array1<f64>,
    density: Array1<f64>,
    ptype: Array1<f64>, // type of particles
}

/// Data-struct containing overall stats for a dataset. (e.g system dimensions)
#[derive(Debug, Default, Getters)]
pub struct GlobalStats {
    dimensions: Array2<f64>,
    nparticles: usize,
    #[getter(rename = "timesteps")]
    ntimesteps: usize,
    min_time: f64,
    max_time: f64,
    sample_rate: f64,
    velocity: Array2<f64>,
    velocity_mag: Array1<f64>,
    time_array: Array1<f64>,
}

impl GlobalStats {
    /// Returns the timestep at this time in the dataset
    pub fn timestep_at_seconds_closest(&self, seconds: f64) -> Result<usize, &'static str> {
        print_warning!("This function is not tested yet!");
        // finding index in ordered list of self.time_array
        // using the binary search algorithm
        let mut low = 0;
        let mut high = self.time_array.len() - 1;

        if self.time_array[high] < seconds {
            return Err("Timestep is greater than the number of timesteps");
        }
        if self.time_array[low] > seconds {
            return Err("Timestep is smaller than the number of timesteps");
        }
        let mut mid = (low + high) / 2;
        while low <= high {
            mid = (low + high) / 2;
            if (self.time_array[mid] - seconds).abs() < 0.01 {
                return Ok(mid);
            } else if self.time_array[mid] < seconds {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        return Ok(mid);
    }
}
