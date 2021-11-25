//! Plots 3D quivers given a grid of arrow starting positions and corresponding vector components
//! This code adapts the quiver plot from plotly.py, improving both the speed of execution and arrow
//! visual appearance and the speed of execution.
use ndarray;
use itertools;
use itertools::izip;
use ndarray::prelude::*;
use std::f64::consts::PI;
use plotly::common::{Fill, Line, Mode};
use plotly::layout::{Layout};
use plotly::{NamedColor, Plot, Scatter3D, Surface};
use core::panic;
use numpy;

/// Define struct to contain raw data for plotting 2D quivers with content:
/// xdata: arrow starting x coordinates
/// ydata: arrow starting y coordinates
/// udata: vector x components
/// vdata: vector y components
/// The following 2 fields are defined through the use of associated function "scale"
/// xdata_end: arrow ending x coordinates
/// ydata_end: arrow ending y coordinates
pub struct ArrowData {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    zdata: Array1<f64>,
    udata: Array1<f64>,
    vdata: Array1<f64>,
    wdata: Array1<f64>,
    xdata_end: Array1<f64>,
    ydata_end: Array1<f64>,
    zdata_end: Array1<f64>,
}


pub enum ScaleMode{
    Global(f64),
    Elementwise(Array3<f64>),
    Default,
    None,
}

impl ArrowData {
//constructor for ArrowData struct
    pub fn new(x:Array3<f64>, y:Array3<f64>, z: Array3<f64>, u:Array3<f64>, v:Array3<f64>, w:Array3<f64>, scale_mode: ScaleMode) -> ArrowData {
        //supercede the default error message for shape mismatch as it doesn't identify the offending array
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\ny has dimensions {:?}", &x.dim(), &y.dim());
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nz has dimensions {:?}", &x.dim(), &z.dim());
        assert!(&x.dim() == &u.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nu has dimensions {:?}", &x.dim(), &u.dim());
        assert!(&x.dim() == &v.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nv has dimensions {:?}", &x.dim(), &v.dim());
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nw has dimensions {:?}", &x.dim(), &w.dim());
        return ArrowData {
        xdata: ArrowData::flatten(&x),
        ydata: ArrowData::flatten(&y),
        zdata: ArrowData::flatten(&z),
        udata: ArrowData::flatten(&ArrowData::scale(&scale_mode,&u)), 
        vdata: ArrowData::flatten(&ArrowData::scale(&scale_mode,&v)),
        wdata: ArrowData::flatten(&ArrowData::scale(&scale_mode,&w)),
        xdata_end: ArrowData::flatten(&x) + ArrowData::flatten(&ArrowData::scale(&scale_mode,&u)), 
        ydata_end: ArrowData::flatten(&y) + ArrowData::flatten(&ArrowData::scale(&scale_mode,&v)),
        zdata_end: ArrowData::flatten(&z) + ArrowData::flatten(&ArrowData::scale(&scale_mode,&w)),
            }
    }
        
    fn flatten(arr:&Array3<f64>) -> Array1<f64>{
    //helper associated function for constructor - flattens a 2D array into a 1D array
        return arr.slice(s![0..arr.shape()[0], 0..arr.shape()[1], 0..arr.shape()[2]]) //create slice of all elements
                .iter() //create iterable
                .copied() //iterate through
                .collect::<Array1<f64>>() //collect into array
    }
    //helper associated function for constructor - scales u and v
    fn scale(scale_mode: &ScaleMode, arr:&Array3<f64>) ->  Array3<f64> {
            //use match enum to decide whether to apply global, elementwise, default or no arrow scaling
            match scale_mode{
                ScaleMode::Global(scale_factor) => {
                    let scale_factor = *scale_factor;
                    return arr*scale_factor
                },
    
                ScaleMode::Elementwise(scale_array) => {
                    let scale_array = scale_array;
                    println!("original\n{:?})", arr);
                    println!("scaled\n{:?}", arr*scale_array);
                    return arr*scale_array
                },
            
                ScaleMode::Default => { 
                    //default to global scaling, with scale factor = 0.3
                    let default_scale = 0.3;
                    return arr*default_scale
                },
                ScaleMode::None => {
                    //perform no scaling
                    return arr*1.0 //not sure why multiplying by 1 removes the borrow checker error
                }
            }
        }
        
    }
    

pub fn quiver_barbs(data: &ArrowData) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>) {
    //create quiver barbs
    let mut barb_x = Vec::new();
    let mut barb_y = Vec::new();
    let mut barb_z = Vec::new();
    for (start, end) in izip!(&data.xdata, &data.xdata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_x.push(tup);    
    }
    for (start, end) in izip!(&data.ydata, &data.ydata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_y.push(tup);    
    }
    for (start, end) in izip!(&data.zdata, &data.zdata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_z.push(tup);
    }
    //println!("{:?}", barb_x);
    return (barb_x, barb_y, barb_z)
}

pub fn gen_quiver_arrows(data: &ArrowData) -> 
            (Vec<(f64, f64, f64, f64, f64)>, 
            Vec<(f64, f64, f64, f64, f64)>, 
            Vec<(f64, f64, f64, f64, f64)>) {
//gen the list of x and y values to plot arrows
    
    //default angle is pi/9
    const ANGLE: f64 = PI/9.0; //TODO add this as an optional argument

    //let default scale be 0.5
    const ARROW_SCALE: f64 = 0.5;
    
    //length is simply (x+u) - x = u etc  
    //different syntax used compared to 2D as it is assumed that 
    //f64::hypot is more efficiently implemented than what is done here
    let arrow_len: Array1<f64> = izip!(&data.vdata,&data.udata,&data.wdata)
                                .map(|(u,v,w)| (u*u + v*v + w*w).powf(0.5) )
                                .collect::<Array1<f64>>()
                                *ARROW_SCALE; 
    
    // get barb angles for xy and xz plane
    let barb_ang_xy: Array1<f64> = izip!(&data.vdata, &data.udata).map(|(v,u)| f64::atan2(*v,*u)).collect::<Array1<f64>>();
    let barb_ang_xz: Array1<f64> = izip!(&data.wdata, &data.udata).map(|(w,u)| f64::atan2(*w,*u)).collect::<Array1<f64>>();

    //find angles for both lines of arrow for both planes
    let arrow_ang_xy_1: Array1<f64> = &barb_ang_xy + ANGLE;
    let arrow_ang_xy_2: Array1<f64> = &barb_ang_xy - ANGLE;
    let arrow_ang_xz_1: Array1<f64> = &barb_ang_xz + ANGLE;
    let arrow_ang_xz_2: Array1<f64> = &barb_ang_xz - ANGLE;

    //do some trig on these
    let sin_ang_xy_1: Array1<f64> = arrow_ang_xy_1.mapv(f64::sin);
    let cos_ang_xy_1: Array1<f64> = arrow_ang_xy_1.mapv(f64::cos);
    let sin_ang_xy_2: Array1<f64> = arrow_ang_xy_2.mapv(f64::sin);
    let cos_ang_xy_2: Array1<f64> = arrow_ang_xy_2.mapv(f64::cos);
    let sin_ang_xz_1: Array1<f64> = arrow_ang_xz_1.mapv(f64::sin);
    let cos_ang_xz_1: Array1<f64> = arrow_ang_xz_1.mapv(f64::cos);
    let sin_ang_xz_2: Array1<f64> = arrow_ang_xz_2.mapv(f64::sin);
    let cos_ang_xz_2: Array1<f64> = arrow_ang_xz_2.mapv(f64::cos);
    
    //find corresponding components
    let seg_1_xy_x: Array1<f64> = &arrow_len * &cos_ang_xy_1;
    let seg_1_xy_y: Array1<f64> = &arrow_len * &sin_ang_xy_1;
    let seg_1_xy_z: Array1<f64> = &arrow_len * &sin_ang_xy_1;
    let seg_1_xz_x: Array1<f64> = &arrow_len * &cos_ang_xz_1;
    let seg_1_xz_y: Array1<f64> = &arrow_len * &sin_ang_xz_1;
    let seg_1_xz_z: Array1<f64> = &arrow_len * &sin_ang_xz_1;
    let seg_2_xy_x: Array1<f64> = &arrow_len * &cos_ang_xy_2;
    let seg_2_xy_y: Array1<f64> = &arrow_len * &sin_ang_xy_2;
    let seg_2_xy_z: Array1<f64> = &arrow_len * &sin_ang_xy_2;
    let seg_2_xz_x: Array1<f64> = &arrow_len * &cos_ang_xz_2;
    let seg_2_xz_y: Array1<f64> = &arrow_len * &sin_ang_xz_2;
    let seg_2_xz_z: Array1<f64> = &arrow_len * &sin_ang_xz_2;
    
    //set coordinates of the arrow
    let point_n_x: Array1<f64> = &data.xdata_end - &seg_1_xz_x;
    let point_n_y: Array1<f64> = &data.ydata_end - &seg_1_xz_y;
    let point_n_z: Array1<f64> = &data.zdata_end - &seg_1_xz_z;
    let point_e_x: Array1<f64> = &data.xdata_end - &seg_1_xy_x;
    let point_e_y: Array1<f64> = &data.ydata_end - &seg_1_xy_y;
    let point_e_z: Array1<f64> = &data.zdata_end - &seg_1_xy_z;
    let point_s_x: Array1<f64> = &data.xdata_end - &seg_2_xz_x;
    let point_s_y: Array1<f64> = &data.ydata_end - &seg_2_xz_y;
    let point_s_z: Array1<f64> = &data.zdata_end - &seg_2_xz_z;
    let point_w_x: Array1<f64> = &data.xdata_end - &seg_2_xy_x;
    let point_w_y: Array1<f64> = &data.ydata_end - &seg_2_xy_y;
    let point_w_z: Array1<f64> = &data.zdata_end - &seg_2_xy_z;
    
    //finally, combine this data into something usable
    let mut arrow_x = Vec::new();
    let mut arrow_y = Vec::new();
    let mut arrow_z = Vec::new();
    
    for (n, e, s, w, end) in izip!(point_n_x, point_e_x, point_s_x, point_w_x, &data.xdata_end) {
        let tup: (f64, f64, f64, f64, f64) = (n, e, s, w, *end);
        arrow_x.push(tup);    
    }
    for (n, e, s, w, end) in izip!(point_n_y, point_e_y, point_s_y, point_w_y, &data.ydata_end) {
        let tup: (f64, f64, f64, f64, f64) = (n, e, s, w, *end);
        arrow_y.push(tup);    
    }
    for (n, e, s, w, end) in izip!(point_n_z, point_e_z, point_s_z, point_w_z, &data.zdata_end) {
        let tup: (f64, f64, f64, f64, f64) = (n, e, s, w, *end);
        arrow_z.push(tup);    
    }

    return (arrow_x, arrow_y, arrow_z)
}

pub fn trace_arrows(data: ArrowData) -> Vec<Box<Scatter3D<f64,f64, f64>>>  {
     let (barb_x, barb_y, barb_z) = quiver_barbs(&data);
     let (arrow_x, arrow_y, arrow_z) = gen_quiver_arrows(&data);
     let mut traces = Vec::new();
     //janky unpacking
     for (x_line, y_line, z_line, x_head, y_head, z_head) in izip!(barb_x, barb_y, barb_z, arrow_x, arrow_y, arrow_z) {
        let xpl = vec![x_line.0, 
                       x_line.1, 
                       x_head.0,
                       x_head.1,
                       x_head.2,
                       x_head.3,
                       x_head.4];
                       
        let ypl = vec![y_line.0, 
                       y_line.1, 
                       y_head.0,
                       y_head.1,
                       y_head.2,
                       y_head.3,
                       y_head.4];

        let zpl = vec![z_line.0, 
                        z_line.1, 
                        z_head.0,
                        z_head.1,
                        z_head.2,
                        z_head.3,
                        z_head.4];

        let trace = Scatter3D::new(xpl, ypl, zpl)
                        .mode(Mode::Lines)
                        .show_legend(false)
                        .line(Line::new().color(NamedColor::Blue))
                        //.fill(Fill::ToSelf)
                        //.fill_color(NamedColor::Blue)
                        ;
        traces.push(trace);
        
     }      
      
    //plot.set_layout(layout);
    return traces
}

pub fn plot(traces:Vec<Box<Scatter3D<f64, f64,f64>>>, layout:Layout) -> Plot {
    //create a quiver plot based on user defined layout and return the plot object for the user to further customise as they wish
    let mut plot = Plot::new();
    //use quicker render version
    plot.use_local_plotly();
    for trace in traces{
        plot.add_trace(trace);
    }
    plot.set_layout(layout);
    return (plot)
}

