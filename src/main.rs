mod datamanager;
use std::time::{Duration, Instant};
mod base;

pub mod utilities;
pub mod functions;
use functions::Granular;
use datamanager::{GlobalStats,DataManager, Manager,Timestep};
use datamanager::pdata::PData;
use datamanager::tdata::TData;
mod plotting;
use itertools::izip;
use plotting::vector3d;
use base::{ParticleSelector,Grid2D,Dim};
use ndarray::prelude::*;
use core::panic;
use std::f64::consts;
use ndarray::stack;
use plotly::common::{Mode, Fill, Line, Anchor, ColorBar, Marker};
use plotly::{Plot,NamedColor, Scatter, Scatter3D};
use ndarray::{Array, Ix1, Ix2};
use plotly::ndarray::ArrayTraces;
use functions::meshgrid3d;
use plotly::Layout;

use crate::plotting::vector3d::BoundMode;
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

  
     /*let valx: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let valy: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let valz: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let (x, y, z) = meshgrid3d(valx,valy, valz);
    let sf: Array2<f64> = array![[1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.],
                           [1., 1., 2., 3., 4.]];                          
    let u: Array3<f64> = &x.mapv(f64::sin)*&y.mapv(f64::cos)*&z.mapv(f64::cos);
    let v: Array3<f64> = -&y.mapv(f64::sin)*&x.mapv(f64::cos)*&z.mapv(f64::cos);
    let w: Array3<f64> = f64::sqrt(2.0/3.0)*&x.mapv(f64::cos)*&y.mapv(f64::cos)*&z.mapv(f64::sin);

    let arrows = vector3d::ConeData::new(x,y,z,u,v,w,vector3d::ScaleMode::Default);
    let mode: BoundMode = BoundMode::None;
    let cone_traces = vector3d::trace_arrows(arrows,arrow_scale, mode);
    let mut plot = vector3d::plot(cone_traces, layout, true); */
    
    

     let now = Instant::now();
     let mut data = TData::new("tests/fixtures/drum.hdf5");
     let grid = Grid2D::new(array![30,30], Dim::TwoD((-0.04,0.04),(-0.04,0.04)));
     let selector = ParticleSelector::new((f64::MIN,f64::MAX),
     vec![f64::MIN,f64::MAX],
     vec![f64::MIN,f64::MAX],
     vec![f64::MIN,f64::MAX],
     vec![usize::MIN,usize::MAX]);
     let (u, v, x, y) = data.vectorfield(
         grid,
         &selector,
         false,
         1
     );

     println!("End time: {}", now.elapsed().as_millis());

}
