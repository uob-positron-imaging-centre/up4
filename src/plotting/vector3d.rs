//! this will eventually plot 3D vectors

use ndarray;
use itertools;
use itertools::izip;
use ndarray::prelude::*;
use plotly::cone::Anchor;
use plotly::common::{Line, Mode};
use plotly::{NamedColor, Plot, Cone, Scatter3D};
use core::panic;
use ndarray_stats::QuantileExt;
use crate::utilities::maths::flatten_3d;

/// Set required scaling mode for arrow lengths.
/// 
/// Contents:
/// 1. Global
/// * Applies a single scaling factor to *all* elements of u and v.
/// 2. Elementwise
/// * Applies elementwise scaling factors to elements of u and v. Must be same shape as u and v.
/// 3. Default
/// * Applies default scaling that is applied in plotly.py. 
/// * This is equivalent to ScaleMode::Global(0.3).
/// 4. None
/// * No scaling applied.
pub enum ScaleMode{
    Global(f64),
    Elementwise(Array3<f64>),
    Default,
    None,
}

/// Set required bounding mode for arrow lengths.
/// 
/// Contents:
/// 1. Min
/// * Prescribe minimum allowable length for arrows.
/// 2. Max
/// * Prescribe maximum allowable length for arrows.
/// 3. Minmax
/// * Prescribe minimum and maximum allowable lengths for arrows.
/// 4. Node
/// * Adjusts the lengths of all arrows on a uniform grid of width dx by dx/max(arrow length).
/// * This enforces that the arrows lie within a circle of radius dx/2 originating from the node.
/// 5. None
/// * No bounds applied.
pub enum BoundMode{
    Min(f64),
    Max(f64),
    Minmax((f64,f64)),
    Node,
    None,
}

/// Define struct to contain raw data for plotting 3D cones.
/// Contents:
/// * xdata: arrow starting x coordinates.
/// * ydata: arrow starting y coordinates.
/// * zdata: arrow starting z coordinates.
/// * udata: vector x components.
/// * vdata: vector y components.
/// * wdata: vector z components.
/// 
/// The following 2 fields are defined through the use of associated function ```ConeData::scale```.
/// * xdata_end: arrow ending x coordinates.
/// * ydata_end: arrow ending y coordinates.
/// * zdata_end: arrow ending z coordinates.
/// 
/// # Examples
/// 
/// ```
/// use ndarray::Array;
/// let x: Array3<f64> = array![[0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.]];  
/// 
/// let y: Array3<f64> = array![[0., 0., 0., 0., 0.],
///                             [1., 1., 1., 1., 1.],
///                             [2., 2., 2., 2., 2.],
///                             [3., 3., 3., 3., 3.],
///                             [4., 4., 4., 4., 4.]];  
/// 
/// let z: Array3<f64> = array![[0., 0., 0., 0., 0.],
///                             [1., 1., 1., 1., 1.],
///                             [2., 2., 2., 2., 2.],
///                             [3., 3., 3., 3., 3.],
///                             [4., 4., 4., 4., 4.]]; 
/// 
/// let u: Array3<f64> = array![[0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.]];  
/// 
/// let v: Array3<f64> = array![[0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.]];  
/// 
/// let w: Array3<f64> = array![[0., 0., 0., 0., 0.],
///                             [1., 1., 1., 1., 1.],
///                             [2., 2., 2., 2., 2.],
///                             [3., 3., 3., 3., 3.],
///                             [4., 4., 4., 4., 4.]]; 
/// 
/// let scale_mode = vector2d::ScaleMode::Default;
/// 
/// let arrows: ConeData = vector3d::ConeData::new(x,y,z,u,v,w,scale_mode);
/// ```
pub struct ConeData {
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

impl ConeData{
    /// constructor for ConeData struct
    pub fn new(x:Array3<f64>, y:Array3<f64>, z:Array3<f64>, u:Array3<f64>, v:Array3<f64>, w:Array3<f64>, scale_mode: ScaleMode) -> ConeData { 
        //supercede the default error message for shape mismatch as it doesn't identify the offending array
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\ny has dimensions {:?}", &x.dim(), &y.dim());
        assert!(&x.dim() == &z.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nz has dimensions {:?}", &x.dim(), &z.dim());
        assert!(&x.dim() == &u.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nu has dimensions {:?}", &x.dim(), &u.dim());
        assert!(&x.dim() == &v.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nv has dimensions {:?}", &x.dim(), &v.dim());
        assert!(&x.dim() == &w.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nw has dimensions {:?}", &x.dim(), &w.dim());
        return ConeData {
            xdata: flatten_3d(&x),
            ydata: flatten_3d(&y),
            zdata: flatten_3d(&z),
            udata: flatten_3d(&ConeData::scale(&scale_mode,&u)), 
            vdata: flatten_3d(&ConeData::scale(&scale_mode,&v)),
            wdata: flatten_3d(&ConeData::scale(&scale_mode,&w)),
            xdata_end: flatten_3d(&x) + flatten_3d(&ConeData::scale(&scale_mode,&u)), 
            ydata_end: flatten_3d(&y) + flatten_3d(&ConeData::scale(&scale_mode,&v)),
            zdata_end: flatten_3d(&z) + flatten_3d(&ConeData::scale(&scale_mode,&w)),
        }
    }
    /// Performs scaling as determined by ScaleMode enum value.
    fn scale(scale_mode: &ScaleMode, arr:&Array3<f64>) ->  Array3<f64> {
        //use match enum to decide whether to apply global, elementwise, default or no arrow scaling
        match scale_mode{
            /// Apply single scaling factor to all elements.
            ScaleMode::Global(scale_factor) => {
                let scale_factor = *scale_factor;
                return arr*scale_factor
            },
            
            /// Apply an array of scale factors to elements.
            ScaleMode::Elementwise(scale_array) => {
                let scale_array = scale_array;
                println!("original\n{:?})", arr);
                println!("scaled\n{:?}", arr*scale_array) ;
                return arr*scale_array
            },
        
            /// Apply default scaling, equivalent to ``` ScaleMode::Global(0.3)```
            ScaleMode::Default => { 
                let default_scale = 0.3;
                return arr*default_scale
            },

            /// Apply no scaling.
            ScaleMode::None => {
                //perform no scaling
                return arr*1.0
            }
        }
    }

    /// Set minimum length for arrows.    
    pub fn min_bound(mut self, min: &f64, mut arrow_len: Array1<f64>) -> ConeData {
        for i in 0..arrow_len.len_of(Axis(0)) {
            if arrow_len[i] < *min {
                arrow_len[i] = min/arrow_len[i];
            }
            else {
                arrow_len[i] = 1.;
            }
        }
        self.udata *= &arrow_len;
        self.vdata *= &arrow_len;
        self.wdata *= &arrow_len;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        return self
        }

    /// Set maximum length for arrows.
    pub fn max_bound(mut self, max: &f64, mut arrow_len: Array1<f64>) -> ConeData {
        for i in 0..arrow_len.len_of(Axis(0)) {
            if arrow_len[i] > *max {
            arrow_len[i] = max/arrow_len[i];
            }
            else {
                arrow_len[i] = 1.;
            }
        }
        self.udata *= &arrow_len;
        self.vdata *= &arrow_len;
        self.wdata *= &arrow_len;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        return self
    }

    /// Set minimum and maximum lengths for arrows.
    pub fn min_max_bound(mut self, min: &f64, max: &f64, mut arrow_len: Array1<f64>) -> ConeData {
        for i in 0..arrow_len.len_of(Axis(0)) {
            if arrow_len[i] > *max {
                arrow_len[i] = max/arrow_len[i];
            }
            else if arrow_len[i] < *min {
                arrow_len[i] = min/arrow_len[i];
            }
            else {
                arrow_len[i] = 1.;
            }
        }
        self.udata *= &arrow_len;
        self.vdata *= &arrow_len;
        self.wdata *= &arrow_len;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        return self
    }

    /// Constrain all arrows to lie within circle of radius dx/2 from each node.
    /// On a non-uniform grid, this *will* distort the plot.
    pub fn node_bound(mut self, arrow_len: Array1<f64>) -> ConeData {
        let dx = (self.xdata[0] - self.xdata[1]).abs();
        //println!("{},{},{}",self.xdata[1], self.xdata[2], dx);
        self.udata *= 0.5*dx/arrow_len.max().unwrap();
        self.vdata *= 0.5*dx/arrow_len.max().unwrap();
        self.wdata *= 0.5*dx/arrow_len.max().unwrap();
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        return self
    }

    /// Apply bounds to arrows based on BoundMode enum value.
    pub fn bound(self, bound_mode: BoundMode) -> ConeData {
        let arrow_len: Array1<f64> = izip!(&self.udata,&self.vdata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>();
        match bound_mode{
            BoundMode::Min(min) => {
                let data_bounded: ConeData = self.min_bound(&min, arrow_len);
                return data_bounded
            },

            BoundMode::Max(max) => {
                let data_bounded: ConeData = self.max_bound(&max, arrow_len);
                return data_bounded
            },

            BoundMode::Minmax((min,max)) => {
                let data_bounded: ConeData = self.min_max_bound(&min, &max, arrow_len);
                return data_bounded
            },
            
            //Note, this is  only valid for uniform grids
            BoundMode::Node => {
                let data_bounded: ConeData = self.node_bound(arrow_len);
                return data_bounded
            },
            //Do nothing as no bounds are requested
            BoundMode::None => { 
                let data_bounded: ConeData = self; 
                return data_bounded
            },
        } 
    }
}

pub fn trace_arrows(data: ConeData, arrow_scale: Option<f64>, bound_mode: BoundMode) -> Vec<Box<Cone<f64,f64,f64,f64,f64,f64>>> {
    let data_bounded = data.bound(bound_mode);
    let mut cone_traces = Vec::new();
    
    for idx in 0..data_bounded.xdata.len(){
        let xpl = vec![data_bounded.xdata_end[idx]];
        let ypl = vec![data_bounded.ydata_end[idx]];
        let zpl = vec![data_bounded.zdata_end[idx]];
        let upl = vec![data_bounded.udata[idx]];
        let vpl = vec![data_bounded.vdata[idx]];
        let wpl = vec![data_bounded.wdata[idx]];

        let trace = Cone::new(xpl,ypl,zpl,upl,vpl,wpl)
            .anchor(Anchor::Tip)
            .show_legend(false)
            .show_scale(false);
        cone_traces.push(trace);
    }   
    return cone_traces
}

pub fn plot(cone_traces: Vec<Box<Cone<f64,f64,f64,f64,f64,f64>>>,  layout:plotly::layout::Layout, square: bool) -> Plot {
    let mut plot = Plot::new();
    //use local render version
    plot.use_local_plotly();

    for trace in cone_traces{
        plot.add_trace(trace);
    }

    // if this is true, then perform some additional plotly calls to create
    // a plot where the x and y axes are equal
    if square{
        let y_axis = plotly::layout::Axis::new()
                .scale_anchor("x".to_string())
                .scale_ratio(1.);

        let square_layout = layout.y_axis(y_axis);
        
        plot.set_layout(square_layout);
    } else{
        //plot as-is
        plot.set_layout(layout);
    }
    return plot
}