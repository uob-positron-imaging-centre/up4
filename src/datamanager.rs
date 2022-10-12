//! Data Managment Module. Organizes file access.
//use crate::functions::*;
//use crate::functions::fields::*;
use super::functions::extractions::Extraction;
use super::functions::Granular;
use crate::{print_warning, types::*};
use derive_getters::Getters;
use ndarray::prelude::*;

pub mod pdata;
pub use pdata::PData;
pub mod tdata;
pub use tdata::TData;
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
}

pub trait Manager: DataManager + Granular + Extraction {}

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
    pub fn timestep_at_seconds(&self, seconds: f64) -> Result<usize, &'static str> {
        print_warning!("This function is not tested yet!");
        // finding index in ordered list of self.time_array
        // using the binary search algorithm
        let mut low = 0;
        let mut high = self.time_array.len() - 1;
        let mut mid;
        while low <= high {
            mid = (low + high) / 2;
            if self.time_array[mid] == seconds {
                return Ok(mid);
            } else if self.time_array[mid] < seconds {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        // if binary search fails, get the closest timestep with a linear search
        let mut timestep = 0;
        for i in 0..self.time_array.len() {
            if self.time_array[i] > seconds {
                timestep = i;
                break;
            }
        }

        if timestep > self.ntimesteps {
            return Err("Timestep is greater than the number of timesteps");
        }
        Ok(timestep as usize)
    }
}
