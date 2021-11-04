mod datamanager;
use std::time::{Duration, Instant};
mod base;
pub mod utilities;
pub mod functions;
use datamanager::{GlobalStats,DataManager, Timestep};
use datamanager::pdata::PData;
use datamanager::tdata::TData;
mod quiver_funcs;
use ndarray::prelude::*;
use core::panic;
use std::f64::consts;
use ndarray::stack;
use plotly::common::{Mode};
use plotly::{Plot, Scatter};
use ndarray::{Array, Ix1, Ix2};
use plotly::ndarray::ArrayTraces;
use functions::meshgrid;
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
     println!("Welcome to Dan's attempts at a quiver plot, oh wow!\n
     To DO:\n
     1) add optional arguments for scaling\n
     2) expose the plot call so the user can mess with options\n
     3) get my code called rubbish and learn how to improve it!!!");
     //time for some shitty hardcoded test data
     const PI: f64 = consts::PI;
     const PTS: f64 = 4.; //number of points
     let valx: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
     let valy: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
     let (x, y) = meshgrid(valx,valy);
     let sf: Array2<f64> = array![[1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.]];
     /*                       
     let y: Array2<f64> = array![[0., 0., 0., 0., 0.],
                           [1., 1., 1., 1., 1.],
                           [2., 2., 2., 2., 2.],
                           [3., 3., 3., 3., 3.],
                           [4., 4., 4., 4., 4.]];*/
                           
     let u: Array2<f64> = &x.mapv(f64::sin)*&y.mapv(f64::cos);
     let v: Array2<f64> = -&y.mapv(f64::sin)*&x.mapv(f64::cos);
     //let arrows = quiver_funcs::ArrowData::new(x,y,u,v,quiver_funcs::ScaleMode::Global(0.5));
     //let arrows = quiver_funcs::ArrowData::new(x,y,u,v,quiver_funcs::ScaleMode::Elementwise(sf));
     //let arrows = quiver_funcs::ArrowData::new(x,y,u,v,quiver_funcs::ScaleMode::Default);
     let arrows = quiver_funcs::ArrowData::new(x,y,u,v,quiver_funcs::ScaleMode::None);
     quiver_funcs::plot_arrows(arrows, PTS);
}
