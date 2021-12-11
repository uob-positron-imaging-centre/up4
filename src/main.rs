mod datamanager;
use std::time::{Duration, Instant};
mod base;
pub mod utilities;
pub mod functions;
use datamanager::{GlobalStats,DataManager, Timestep};
use datamanager::pdata::PData;
use datamanager::tdata::TData;
mod plotting;
use itertools::izip;
use plotting::vector2d;
use ndarray::prelude::*;
use core::panic;
use std::f64::consts;
use ndarray::stack;
use plotly::common::{Mode, Fill, Line, Anchor, ColorBar, Marker};
use plotly::{Plot,NamedColor, Scatter, Scatter3D};
use ndarray::{Array, Ix1, Ix2};
use plotly::ndarray::ArrayTraces;
use functions::meshgrid;
use plotly::Layout;

use crate::plotting::vector2d::BoundMode;
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
     // test data
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
    let arrows = vector2d::ArrowData::new(x,y,u,v,vector2d::ScaleMode::Default);
    //let arrows = quiver2d::ArrowData::new(x,y,u,v,quiver2d::ScaleMode::None); 
    let arrow_scale: Option<f64> = None;
    let mode: BoundMode = BoundMode::Max(0.001);
    let traces = vector2d::trace_arrows(arrows,arrow_scale, mode);
    //let cbar = plotly::common::ColorBar::default();
    //let caxis = plotly::layout::ColorAxis::new()
                              //.color_bar(cbar)
                              //.auto_color_scale(true)
                              //.show_scale(true)
                                ;
    let layout = plotly::layout::Layout::new()
                       .title("oh wow it works!!!".into())  
                       //.color_axis(caxis)   
                       ;
    let mut plot = vector2d::plot(traces, layout, true); 
    
    //uncomment to see how the grid looks, each point is half the width of the grid the arrows are plotted on
    /*let valx: Array1<f64> = Array::range(0.,2.*PI+PI/(2.*PTS),PI/PTS);
    let valy: Array1<f64> = Array::range(0.,2.*PI+PI/(2.*PTS),PI/PTS);
    let (x, y) = meshgrid(valx,valy);
                     
    let mut grid_traces = Vec::new();
    for (x_it, y_it) in izip!(x, y){
      let xpl = vec![x_it];
      let ypl = vec![y_it];
      let trace = Scatter::new(xpl,ypl)
                  .mode(Mode::Markers)
                  .show_legend(false)
                  .marker(Marker::new().color(NamedColor::Black));
      grid_traces.push(trace);
    }

    
    for trace in grid_traces{
      plot.add_trace(trace);
  }*/
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

    3) Add option/enum structs for titles, axis labels and ranges
    4) Investigate colourbar addition
      */
}
