//! The `uppp` crate provides an tool for post processing particle based data.
//!
//! in `uppp` we provide different structs allowing the accessing, processing and visualisation
//! of quantities such as the velocity vectorfield, mean-squared-displacement or
//! dispersion.
//!
//! It allows you to handle different data types via the `DataManager` trait.
//! The currently implemented datatype consists of a [HDF5](https://github.com/aldanor/hdf5-rust)
//! based data structure similar as [H5Part](https://dav.lbl.gov/archive/Research/AcceleratorSAPP/)
//! with some modifications according to the type of data: Experimental or simulational.
//! Hence we implement two structs that implement the `DataManager` trait: `PData` and `TData`:
//!
//! - **`PData`**: Particle based saving of data, for experimental data with many timestep
//!    but few particles such as [PEPT](https://www.birmingham.ac.uk/research/activity/physics/particle-nuclear/positron-imaging-centre/positron-emission-particle-tracking-pept/pept-overview.aspx)
//! - **`TData`**: A timestep based saving of data, for simulational data from different engines such
//!    as [LIGGGHTS](https://www.cfdem.com/liggghtsr-open-source-discrete-element-method-particle-simulation-code)
#![allow(dead_code)]
extern crate ndarray;
extern crate plotly;
/// Module that implements nD grids and basic functionality on them.
pub mod grid;
pub use grid::*;

/// Module that implements the `ParticleSelector`, a struct deciding if a particle is valid or not
pub mod particleselector;
pub use particleselector::*;

pub mod converter;
pub use converter::*;

pub mod datamanager;
mod functions;
pub mod pylib;
pub mod types;
pub mod utilities;

pub mod plotting;
pub use plotting::*;

pub use pylib::*;
