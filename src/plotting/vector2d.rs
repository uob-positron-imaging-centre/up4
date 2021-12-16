//! Plots 2D quivers given a grid of arrow starting positions and corresponding vector components.
//! 
//! This code adapts the quiver plot code from plotly.py, improving both the speed of execution and arrow
//! visual appearance. Like with quiver plots in Python and MATLAB, a 2D grid of x and y coordinates, and 
//! corresponding vector components (u and v) are expected with this module. Throughout this module, the f64 type is expected.
use ndarray;
use itertools;
use itertools::izip;
use ndarray::prelude::*;
use std::f64::consts::PI;
use plotly::common::{Fill, Line, Mode};
use plotly::{NamedColor, Plot, Scatter};
use core::panic;
use ndarray_stats::QuantileExt;
use crate::utilities::maths::flatten_2d;

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
    Elementwise(Array2<f64>),
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

/// Define struct to contain raw data for plotting 2D quivers.
/// Contents:
/// * xdata: arrow starting x coordinates.
/// * ydata: arrow starting y coordinates.
/// * udata: vector x components.
/// * vdata: vector y components.
/// 
/// The following 2 fields are defined through the use of associated function ```ArrowData::scale```.
/// * xdata_end: arrow ending x coordinates.
/// * ydata_end: arrow ending y coordinates.
/// 
/// # Examples
/// 
/// ```
/// use ndarray::Array;
/// let x: Array2<f64> = array![[0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.]];  
/// 
/// let y: Array2<f64> = array![[0., 0., 0., 0., 0.],
///                             [1., 1., 1., 1., 1.],
///                             [2., 2., 2., 2., 2.],
///                             [3., 3., 3., 3., 3.],
///                             [4., 4., 4., 4., 4.]];  
/// 
/// let u: Array2<f64> = array![[0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.]];  
/// 
/// let v: Array2<f64> = array![[0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.],
///                             [0., 1., 2., 3., 4.]];  
/// 
/// let scale_mode = vector2d::ScaleMode::Default;
/// 
/// let arrows: ArrowData = vector2d::ArrowData::new(x,y,u,v,scale_mode);
/// ```
pub struct ArrowData {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    udata: Array1<f64>,
    vdata: Array1<f64>,
    xdata_end: Array1<f64>,
    ydata_end: Array1<f64>,
}

impl ArrowData {
    ///constructor for ArrowData struct
    pub fn new(x:Array2<f64>, y:Array2<f64>, u:Array2<f64>, v:Array2<f64>, scale_mode: ScaleMode) -> ArrowData { 
        //supercede the default error message for shape mismatch as it doesn't identify the offending array
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\ny has dimensions {:?}", &x.dim(), &y.dim());
        assert!(&x.dim() == &u.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nu has dimensions {:?}", &x.dim(), &u.dim());
        assert!(&x.dim() == &v.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nv has dimensions {:?}", &x.dim(), &v.dim());
        return ArrowData {
        xdata: flatten_2d(&x),
        ydata: flatten_2d(&y),
        udata: flatten_2d(&ArrowData::scale(&scale_mode,&u)), 
        vdata: flatten_2d(&ArrowData::scale(&scale_mode,&v)),
        xdata_end: flatten_2d(&x) + flatten_2d(&ArrowData::scale(&scale_mode,&u)), 
        ydata_end: flatten_2d(&y) + flatten_2d(&ArrowData::scale(&scale_mode,&v))
            }
    }
    
    /// Performs scaling as determined by ScaleMode enum value.
    fn scale(scale_mode: &ScaleMode, arr:&Array2<f64>) ->  Array2<f64> {
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
    pub fn min_bound(mut self, min: &f64, mut arrow_len: Array1<f64>) -> ArrowData {
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
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        return self
        }

    /// Set maximum length for arrows.
    pub fn max_bound(mut self, max: &f64, mut arrow_len: Array1<f64>) -> ArrowData {
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
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        return self
    }

    /// Set minimum and maximum lengths for arrows.
    pub fn min_max_bound(mut self, min: &f64, max: &f64, mut arrow_len: Array1<f64>) -> ArrowData {
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
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        return self
    }

    /// Constrain all arrows to lie within circle of radius dx/2 from each node.
    /// On a non-uniform grid, this *will* distort the plot.
    pub fn node_bound(mut self, arrow_len: Array1<f64>) -> ArrowData {
        let dx = (self.xdata[0] - self.xdata[1]).abs();
        //println!("{},{},{}",self.xdata[1], self.xdata[2], dx);
        self.udata *= 0.5*dx/arrow_len.max().unwrap();
        self.vdata *= 0.5*dx/arrow_len.max().unwrap();
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        return self
    }

    /// Apply bounds to arrows based on BoundMode enum value.
    pub fn bound(self, bound_mode: BoundMode) -> ArrowData {
        let arrow_len: Array1<f64> = izip!(&self.udata,&self.vdata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>();
        match bound_mode{
            BoundMode::Min(min) => {
                let data_bounded: ArrowData = self.min_bound(&min, arrow_len);
                return data_bounded
            },

            BoundMode::Max(max) => {
                let data_bounded: ArrowData = self.max_bound(&max, arrow_len);
                return data_bounded
            },

            BoundMode::Minmax((min,max)) => {
                let data_bounded: ArrowData = self.min_max_bound(&min, &max, arrow_len);
                return data_bounded
            },
            
            //Note, this is  only valid for uniform grids
            BoundMode::Node => {
                let data_bounded: ArrowData = self.node_bound(arrow_len);
                return data_bounded
            },
            //Do nothing as no bounds are requested
            BoundMode::None => { 
                let data_bounded: ArrowData = self; 
                return data_bounded
            },
        } 
    }
}
    
/// Returns vectors containing arrow start and stop coordinates.
/// 
/// Inputs:
/// 1. data
/// * Reference to ```ArrowData``` struct.
/// 
/// Outputs:
/// 1. barb_x
/// * vector of x coordinates to draw arrow barb.
/// 2. barb_y
/// * vector of y coordinates to draw arrow barb.
/// 
/// # Examples
/// ```
/// let (barb_x, barb_y) = vector2d::quiver_barbs(data);
/// ```
pub fn quiver_barbs(data: &ArrowData) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut barb_x = Vec::new();
    let mut barb_y = Vec::new();
    for (start, end) in izip!(&data.xdata, &data.xdata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_x.push(tup);    
    }
    for (start, end) in izip!(&data.ydata, &data.ydata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_y.push(tup);    
    }
    return (barb_x, barb_y)
}

/// Returns vectors of coordinates to draw arrowheads.
/// 
/// Inputs:
/// 1. data
/// * ```ArrowData``` struct.
/// 2. arrow_scale
/// * Length of arrowhead relative to arrow barb.
/// 
/// Outputs:
/// 1. arrow_x
/// * x coordinates to draw arrowhead.
/// 2. arrow_y
/// * y coordinates to draw arrowhead.
/// 
/// # Examples
/// ```
/// let arrow_scale = Some(0.3);
/// let (arrow_x, arrow_y) = vector2d::gen_quiver_arrows(data, arrow_scale);
/// ```
pub fn gen_quiver_arrows(data: ArrowData, arrow_scale: Option<f64>) -> (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>) {

    const ANGLE: f64 = PI/9.0; 

    // default scale is 0.5
    let arr_scale = arrow_scale.unwrap_or(0.5);
    
    let arrow_len: Array1<f64> = arr_scale*izip!(&data.udata, &data.vdata).map(|(u, v)| f64::hypot(*u,*v)).collect::<Array1<f64>>();
    // get barb angles
    let barb_ang: Array1<f64> = izip!(&data.vdata, &data.udata).map(|(v,u)| f64::atan2(*v,*u)).collect::<Array1<f64>>();
    
    //find angles for both lines of arrow
    let arrow_ang_1: Array1<f64> = &barb_ang + ANGLE;
    let arrow_ang_2: Array1<f64> = &barb_ang - ANGLE;
    
    //find angles for both sides of arrow point
    let sin_ang_1: Array1<f64> = arrow_ang_1.mapv(f64::sin);
    let cos_ang_1: Array1<f64> = arrow_ang_1.mapv(f64::cos);
    let sin_ang_2: Array1<f64> = arrow_ang_2.mapv(f64::sin);
    let cos_ang_2: Array1<f64> = arrow_ang_2.mapv(f64::cos);
    
    //find corresponding components
    let seg_1_x: Array1<f64> = &arrow_len * &cos_ang_1;
    let seg_1_y: Array1<f64> = &arrow_len * &sin_ang_1;
    let seg_2_x: Array1<f64> = &arrow_len * &cos_ang_2;
    let seg_2_y: Array1<f64> = &arrow_len * &sin_ang_2;
    
    //set coordinates of the arrow
    let point_1_x: Array1<f64> = &data.xdata_end - &seg_1_x;
    let point_1_y: Array1<f64> = &data.ydata_end - &seg_1_y;
    let point_2_x: Array1<f64> = &data.xdata_end - &seg_2_x;
    let point_2_y: Array1<f64> = &data.ydata_end - &seg_2_y;
    
    //finally, combine this data into something usable
    let mut arrow_x = Vec::new();
    let mut arrow_y = Vec::new();
    
    for (start, mid, end) in izip!(point_1_x, point_2_x, &data.xdata_end) {
        let tup: (f64, f64, f64) = (start, mid, *end);
        arrow_x.push(tup);    
    }
    for (start, mid, end) in izip!(point_1_y, point_2_y, &data.ydata_end) {
        let tup: (f64, f64, f64) = (start, mid, *end);
        arrow_y.push(tup);    
    }
    return (arrow_x, arrow_y)
}

/// Returns ```Plotly::Scatter``` traces for plotting.
/// 
/// Inputs:
/// 1. data
/// * ```ArrowData``` struct.
/// 2. arrow_Scale
/// * Length of arrowhead relative to arrow barb.
/// 3. bound_mode
/// * ```BoundMode``` struct.
/// 
/// Outputs:
/// 1. traces
/// * Vector of ```Plotly::Scatter``` traces.
/// 
/// # Examples
/// ```
/// let bound_mode = vector2d::BoundMode::None;
/// let traces = vector2d::trace_arrows(data, arrow_scale, bound_mode);
/// ```
pub fn trace_arrows(data: ArrowData, arrow_scale: Option<f64>, bound_mode: BoundMode) -> Vec<Box<Scatter<f64,f64>>>  {
    let data_bounded = data.bound(bound_mode);
    let (barb_x, barb_y) = quiver_barbs(&data_bounded);
    let (arrow_x, arrow_y) = gen_quiver_arrows(data_bounded, arrow_scale);
    let mut traces = Vec::new();
    //unpack the vectors of arrow barbs and heads into new vector containing each arrow as a tuple
    for (x_line, y_line, x_head, y_head) in izip!(barb_x, barb_y, arrow_x, arrow_y) {
       let xpl = vec![x_line.0, 
                    x_line.1, 
                    x_head.0,
                    x_head.1,
                    x_head.2];
                       
        let ypl = vec![y_line.0, 
                    y_line.1, 
                    y_head.0,
                    y_head.1,
                    y_head.2];

        let trace = Scatter::new(xpl, ypl)
                        .mode(Mode::Lines)
                        .show_legend(false)
                        .fill(Fill::ToSelf)
                        .fill_color(NamedColor::Blue)
                        .line(Line::new().color(NamedColor::Blue));

        traces.push(trace);
        
    }    

    return traces
}

/// Returns quiver plot based on user defined layout.
/// 
/// Inputs:
/// 1. traces
/// * Vector of ```Plotly::Scatter``` traces.
/// 2. layout
/// * ```Plotly::layout::Layout``` struct.
/// * See documentation for plotly.rs for available options for layout.
/// 3. square
/// * Boolean to enable equal axes for a square plot.
/// 
/// # Examples
/// ```
/// let layout = plotly::layout::Layout::new()
///                .title("Quiver plot".into());
/// let mut plot = vector2d::plot(traces, layout, true); 
pub fn plot(traces:Vec<Box<Scatter<f64,f64>>>, layout:plotly::layout::Layout, square: bool) -> Plot {
    let mut plot = Plot::new();
    //use local render version
    plot.use_local_plotly();
    for trace in traces{
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

