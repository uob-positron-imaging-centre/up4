#![allow(dead_code, unused_variables, unused_imports)]
mod datamanager;
//use std::time::{Duration, Instant};
/// Module that implements nD grids and basic functionality on them.
pub mod grid;
pub use grid::Dim;
use grid::{CartesianGrid3D, CylindricalGrid3D};

/// Module that implements the `ParticleSelector`, a struct deciding if a particle is valid or not
pub mod particleselector;
use ndarray::{Array, Array1, Array2, Zip};
pub use particleselector::*;
pub mod converter;
pub use converter::*;

pub mod functions;
pub mod utilities;
use datamanager::pdata::PData;
use datamanager::tdata::TData;
use datamanager::DataManager;
use functions::Granular;
use upppp_rust::plotting::plot;
use upppp_rust::quiver::QuiverPlot;
pub mod types;

//pub mod comparison;

use std::f64::consts;
use std::time::Instant;

use plotly::traces::heat_map;
use plotly::{HeatMap, Layout, Plot, Trace};

use crate::utilities::maths::meshgrid;

fn main() {
    // const PI: f64 = consts::PI;
    // const PTS: f64 = 10.; //number of points
    // let x: Array1<f64> = Array::range(0., 2. * PI + PI / PTS, 2. * PI / PTS);
    // let y: Array1<f64> = Array::range(0., 2. * PI + PI / PTS, 2. * PI / PTS);
    // let (xx, yy) = meshgrid(x.clone(), y.clone());
    // let u: Array2<f64> = &xx.mapv(f64::sin) * &yy.mapv(f64::cos);
    // let v: Array2<f64> = -&yy.mapv(f64::sin) * &xx.mapv(f64::cos);
    // let mut norm = Array2::zeros(u.dim());
    // Zip::from(&mut norm).and(&u).and(&v).for_each(|n, &u, &v| {
    //     *n = f64::hypot(u, v);
    // });
    // let true_norm = norm.clone();

    // let arrows = QuiverPlot::new(x, y, u, v, norm, true_norm);
    // let tr = arrows.create_quiver_traces();
    // let l = Layout::new();
    // let plot = plot(tr, l);
    // // plot.show();
    // plot.write_html("test.html")
}

// #[allow(dead_code, unused_imports, unused_variables, unreachable_code)]
// fn main() {
//     converter::vtu_from_folder(
//         "/home/dan/radioactive-mill/post",
//         1e-5,
//         "lethe.hdf5",
//         r"(\d+).pvtu",
//     );
//     let now = Instant::now();
//     let mut pdata = TData::new("lethe.hdf5");
//     let stats = pdata.global_stats();
//     let dim = stats.dimensions();
//     let grid = Box::new(CartesianGrid3D::new(
//         //[800, 800, 800],
//         [10, 10, 10],
//         Dim::ThreeD([
//             [dim[[0, 0]], dim[[1, 0]]],
//             [dim[[0, 1]], dim[[1, 1]]],
//             [dim[[0, 2]], dim[[1, 2]]],
//         ]),
//     ));
//     let selector = ParticleSelector::default();
//     let disp = pdata.velocityfield(grid.clone(), &selector, "x", 0.0, 10000.0);
//     println!("Disp: {}", disp);
//     let disp = pdata.numberfield(grid.clone(), &selector);
//     println!("Disp: {}", disp);
//     let disp = pdata.vectorfield(grid, &selector);
//     println!("Disp: {}", disp);

//     println!("time needed: {} milliseconds", now.elapsed().as_millis());
//     let _y = 0;

//     //plot.show();
//     //println!("End time: {}", now.elapsed().as_millis());
// }
