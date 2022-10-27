#![allow(dead_code)]
mod datamanager;
//use std::time::{Duration, Instant};
/// Module that implements nD grids and basic functionality on them.
pub mod grid;
pub use grid::Dim;
use grid::{CartesianGrid3D, CylindricalGrid3D};

/// Module that implements the `ParticleSelector`, a struct deciding if a particle is valid or not
pub mod particleselector;
pub use particleselector::*;

pub mod converter;
pub use converter::*;

pub mod functions;
pub mod utilities;

use datamanager::pdata::PData;
use datamanager::tdata::TData;
use datamanager::DataManager;
use functions::Granular;
pub mod types;

use std::time::Instant;

use plotly::{HeatMap, Plot};

/*
fn main() {

    println!("Welcome to uPPPP!\nTesting current version...!\nTesting PData");
    let now = Instant::now();
    let mut pdata = PData::new("TEST/HSM_Glass_2l_250.hdf5");
    pdata.stats();

    pdata.test();
    println!("End time: {}", now.elapsed().as_millis());
    println!("Check completed. No errors found in PData!\n\nChecking TData");//lol
    let now = Instant::now();
    let mut tdata = TData::new("TEST/drum.hdf5");
    tdata.stats();

    tdata.test();
    println!("End time: {}", now.elapsed().as_millis());
    println!("Check completed. No errors found in TData!");


} */
#[allow(dead_code, unused_imports, unused_variables)]
fn main() {
    let now = Instant::now();
    let mut pdata = PData::new("tests/fixtures/1p5u_HD1_glass.hdf5");
    let stats = pdata.global_stats();
    let dim = stats.dimensions();
    let grid = Box::new(CartesianGrid3D::new(
        //[800, 800, 800],
        [10, 10, 10],
        Dim::ThreeD([
            [dim[[0, 0]], dim[[1, 0]]],
            [dim[[0, 1]], dim[[1, 1]]],
            [dim[[0, 2]], dim[[1, 2]]],
        ]),
    ));
    let selector = ParticleSelector::default();
    let disp = pdata.dispersion_new(grid, &selector, 4.0);
    let _y = 0;

    //plot.show();
    println!("End time: {}", now.elapsed().as_millis());
}
