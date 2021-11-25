mod datamanager;
use std::time::{Duration, Instant};
mod base;
pub mod utilities;
pub mod functions;
use datamanager::{GlobalStats,DataManager, Timestep};
use datamanager::pdata::PData;
use datamanager::tdata::TData;
mod plotting;
use plotting::quiver2d;
use ndarray::prelude::*;
use core::panic;
use std::f64::consts;
use ndarray::stack;
use plotly::common::{Mode, Fill, Line};
use plotly::{Plot,NamedColor, Scatter, Scatter3D};
use ndarray::{Array, Ix1, Ix2};
use plotly::ndarray::ArrayTraces;
use functions::meshgrid;
use plotly::Layout;
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
     println!("Welcome to Dan's attempts at a quiver plot, oh wow!");
     //time for some shitty hardcoded test data
     const PI: f64 = consts::PI;
     const PTS: f64 = 60.; //number of points
     let valx: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
     let valy: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
     let (x, y) = meshgrid(valx,valy);
     let sf: Array2<f64> = array![[1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.]];                          
    let u: Array2<f64> = &x.mapv(f64::sin)*&y.mapv(f64::cos);
    let v: Array2<f64> = -&y.mapv(f64::sin)*&x.mapv(f64::cos);
    //let arrows = quiver2d::ArrowData::new(x,y,u,v,quiver2d::ScaleMode::Global(0.5));
    //let arrows = quiver2d::ArrowData::new(x,y,u,v,quiver2d::ScaleMode::Elementwise(sf));
    let arrows = quiver2d::ArrowData::new(x,y,u,v,quiver2d::ScaleMode::Default);
    //let arrows = quiver2d::ArrowData::new(x,y,u,v,quiver2d::ScaleMode::None); 
    let traces = quiver2d::trace_arrows(arrows);
    let layout = Layout::new()
                       .title("oh wow it works!!!".into())                    
                       ;
    let plot = quiver2d::plot(traces, layout); 
    /*let x: Array3<f64> = array![[[0.]]];
    let y: Array3<f64> = array![[[0.]]];
    let z: Array3<f64> = array![[[0.]]];

    let u: Array3<f64> = array![[[1.]]];
    let v: Array3<f64> = array![[[1.]]];
    let w: Array3<f64> = array![[[1.]]];
       
    //let arrows = quiver2d::ArrowData::new(x,y,u,v,quiver2d::ScaleMode::Default);
    
    
    //consider 3 point and 4 point arrows (triangle and square based pyramids)
    const arrow_ang: f64 = PI/4.0;
    const barb_ang: f64 = PI/6.0;
    let x: Vec<f64> = vec![0.,1.,
                           1.-0.3*f64::cos(arrow_ang+barb_ang),1., //west
                           1.-0.3*f64::cos(arrow_ang+barb_ang),1., //south
                           1.-0.3*f64::cos(arrow_ang-barb_ang),1., //north
                           1.-0.3*f64::cos(arrow_ang-barb_ang), //east
                           1.-0.3*f64::cos(arrow_ang+barb_ang), //south
                           1.-0.3*f64::cos(arrow_ang+barb_ang), //west
                           1.-0.3*f64::cos(arrow_ang-barb_ang), //north
                           1.-0.3*f64::cos(arrow_ang-barb_ang), //east
                           ];

    let y: Vec<f64> = vec![0.,1.,
                           1.-0.3*f64::sin(arrow_ang+barb_ang),1., //west
                           1.-0.3*f64::sin(arrow_ang-barb_ang),1., //south
                           1.-0.3*f64::sin(arrow_ang+barb_ang),1., //north
                           1.-0.3*f64::sin(arrow_ang-barb_ang), //east
                           1.-0.3*f64::sin(arrow_ang-barb_ang), //south
                           1.-0.3*f64::sin(arrow_ang+barb_ang), //west
                           1.-0.3*f64::sin(arrow_ang+barb_ang), //north
                           1.-0.3*f64::sin(arrow_ang-barb_ang), //east
                           ];

    let z: Vec<f64> = vec![0.,1.,
                          1.-0.3*f64::sin(arrow_ang),1., //west
                          1.-0.3*f64::sin(arrow_ang+barb_ang),1., //south
                          1.-0.3*f64::sin(arrow_ang-barb_ang),1., //north
                          1.-0.3*f64::sin(arrow_ang), //east
                          1.-0.3*f64::sin(arrow_ang+barb_ang), //south
                          1.-0.3*f64::sin(arrow_ang), //west
                          1.-0.3*f64::sin(arrow_ang-barb_ang), //north
                          1.-0.3*f64::sin(arrow_ang), //east
                                                     ];

    let trace = Scatter3D::new(x,y,z)
                                      .mode(Mode::Lines)
                                      .show_legend(false)
                                      .fill(Fill::ToSelf)
                                      .fill_color(NamedColor::Blue)
                                      .line(Line::new().color(NamedColor::Blue)); */
    plot.show();

     /*TODO

    1) Implement global, min-max and elementwise scaling of u and v data
    2) Remove hardcoded axis formatting
    3) Add option/enum structs for titles, axis labels and ranges
    4) Investigate colourbar addition
    5) Enable square plots
    6) Create 3D quiver plot
    7) Create plot directory and update paths
    8) Fuck about with range buttons and sliders to make slice plots

      */
}
