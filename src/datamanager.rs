//! Data Managment Module. Organizes file access.
//use crate::functions::*;
//use crate::functions::fields::*;
use super::functions::Granular;

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
}

pub trait Manager: DataManager + Granular {}

/// Data-struct containing all necessery information for a timestep
#[derive(Debug, Default, Getters, Clone)]
pub struct Timestep {
    time: f64,
    position: Array2<f64>,
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
}
