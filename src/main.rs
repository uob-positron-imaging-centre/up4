mod datamanager;
//use std::time::{Duration, Instant};
/// Module that implements nD grids and basic functionality on them.
pub mod grid;
pub use grid::Dim;
use grid::{CylindricalGrid3D, GridFunctions3D, KartesianGrid3D};

/// Module that implements the `ParticleSelector`, a struct deciding if a particle is valid or not
pub mod particleselector;
pub use particleselector::*;

pub mod converter;
pub use converter::*;

pub mod functions;
pub mod utilities;
use datamanager::pdata::PData;
use datamanager::tdata::TData;
use datamanager::{DataManager, Manager};
use functions::Granular;
mod quiver_funcs;
pub mod types;
use ndarray::prelude::*;
use std::time::{Duration, Instant};

use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};

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

fn main() {
    let now = Instant::now();
    let mut pdata = TData::new("tests/fixtures/drum.hdf5");
    let stats = pdata.global_stats();
    let dim = stats.dimensions();
    let grid = Box::new(KartesianGrid3D::new(
        [7, 1, 7],
        Dim::ThreeD([
            [dim[[0, 0]], dim[[1, 0]]],
            [dim[[0, 1]], dim[[1, 1]]],
            [dim[[0, 2]], dim[[1, 2]]],
        ]),
    ));

    let x = pdata.numberfield(grid, &ParticleSelector::default());
    let vec2d = x
        .collapse(1)
        .outer_iter()
        .map(|arr| arr.to_vec())
        .collect::<Vec<_>>();
    let trace = HeatMap::new_z(vec2d);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
    println!("End time: {}", now.elapsed().as_millis());
}
