#![allow(dead_code, unused_variables, unused_imports)]
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

//pub mod comparison;

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
#[allow(dead_code, unused_imports, unused_variables, unreachable_code)]
fn main() {
    converter::vtu_from_folder(
        "/home/dan/radioactive-mill/post",
        1e-5,
        "lethe.hdf5",
        r"(\d+).pvtu",
    );
    let now = Instant::now();
    let mut pdata = TData::new("lethe.hdf5");
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
    let disp = pdata.velocityfield(grid.clone(), &selector, "x", 0.0, 10000.0);
    println!("Disp: {}", disp);
    let disp = pdata.numberfield(grid.clone(), &selector);
    println!("Disp: {}", disp);
    let disp = pdata.vectorfield(grid, &selector);
    println!("Disp: {}", disp);

    println!("time needed: {} milliseconds", now.elapsed().as_millis());
    let _y = 0;

    //plot.show();
    //println!("End time: {}", now.elapsed().as_millis());
}
