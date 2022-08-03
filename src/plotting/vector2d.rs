//! Plots 2D quivers given a grid of arrow starting positions and corresponding vector components.
//! 
//! This code adapts the quiver plot code from plotly.py, improving both the speed of execution and arrow
//! visual appearance. Like with quiver plots in Python and MATLAB, a 2D grid of x and y coordinates, and 
//! corresponding vector components (u and v) are expected with this module. Throughout this module, the f64 type is expected.
use ndarray;
use itertools;
use itertools::izip;
use ndarray::prelude::*;
use plotly::common::ColorScale;
use std::f64::consts::PI;
use plotly::common::{Fill, Line, Mode, Marker, ColorBar, ColorScaleElement};
use plotly::{Plot, Scatter, NamedColor, HeatMap};
use plotly::layout::{Axis, Layout, AxisConstrain};
use core::panic;
use ndarray_stats::QuantileExt;
use colorous::Gradient;
use super::{VectorData};

/// Define struct to contain raw data for plotting 2D quivers.
/// Contents:
/// * xdata: arrow starting x coordinates.
/// * ydata: arrow starting y coordinates.
/// * udata: vector x components.
/// * vdata: vector y components.
/// 
/// The following 2 fields are defined through the use of associated function ```VectorData2D::scale```.
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
/// let arrows: VectorData2D = vector2d::VectorData2D::new(x,y,u,v,scale_mode);
/// ```
pub struct VectorData2D {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    udata: Array1<f64>,
    vdata: Array1<f64>,
    xdata_end: Array1<f64>,
    ydata_end: Array1<f64>,
    normdata_scaled: Array1<f64>,
    normdata_abs: Array1<f64>,
}

// Struct specific impls
impl VectorData2D {
    ///constructor for VectorData2D struct
    pub fn new(x:Array2<f64>, y:Array2<f64>, u:Array2<f64>, v:Array2<f64>) -> VectorData2D { 
        //supercede the default error message for shape mismatch as it doesn't identify the offending array
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\ny has dimensions {:?}", &x.dim(), &y.dim());
        assert!(&x.dim() == &u.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nu has dimensions {:?}", &x.dim(), &u.dim());
        assert!(&x.dim() == &v.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nv has dimensions {:?}", &x.dim(), &v.dim());
        let xdata: Array1<f64> = Array::from_iter(x);
        let ydata: Array1<f64> = Array::from_iter(y);
        let udata: Array1<f64> = Array::from_iter(u);
        let vdata: Array1<f64> = Array::from_iter(v);
        let xdata_end: Array1<f64> = &xdata + &udata;
        let ydata_end: Array1<f64> = &ydata + &vdata;
        return VectorData2D {
            normdata_scaled: izip!(&udata,&vdata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>(),
            normdata_abs: izip!(&udata,&vdata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>(),
            xdata: xdata,
            ydata: ydata,
            udata: udata,
            vdata: vdata,
            xdata_end: xdata_end, 
            ydata_end: ydata_end,
            }
    }
    /// Returns vectors containing arrow start and stop coordinates.
    /// 
    /// Inputs:
    /// 1. data
    /// * Reference to ```VectorData2D``` struct.
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
    fn quiver_barbs(&self) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let mut barb_x = Vec::new();
        let mut barb_y = Vec::new();
        for (start, end) in izip!(&self.xdata, &self.xdata_end) {
            let tup: (f64, f64) = (*start, *end);
            barb_x.push(tup);    
        }
        for (start, end) in izip!(&self.ydata, &self.ydata_end) {
            let tup: (f64, f64) = (*start, *end);
            barb_y.push(tup);    
        }
        return (barb_x, barb_y)
    }

    /// Returns vectors of coordinates to draw arrowheads.
    /// 
    /// Inputs:
    /// 1. data
    /// * ```VectorData2D``` struct.
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
    fn gen_quiver_arrows(&self, arrow_scale: Option<f64>) -> (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>) {

        const ANGLE: f64 = PI/9.0; 
        // default scale is 0.5
        let scale_factor: f64 = arrow_scale.unwrap_or(0.5);
        
        let arrow_len: Array1<f64> = scale_factor * &self.normdata_scaled;
        // get barb angles
        let barb_ang: Array1<f64> = izip!(self.vdata.view(), self.udata.view()).map(|(v,u)| f64::atan2(*v,*u)).collect::<Array1<f64>>();
        
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
        let point_1_x: Array1<f64> = &self.xdata_end - &seg_1_x;
        let point_1_y: Array1<f64> = &self.ydata_end - &seg_1_y;
        let point_2_x: Array1<f64> = &self.xdata_end - &seg_2_x;
        let point_2_y: Array1<f64> = &self.ydata_end - &seg_2_y;
        
        //finally, combine this data into something usable
        let mut arrow_x = Vec::new();
        let mut arrow_y = Vec::new();
        
        for (start, mid, end) in izip!(point_1_x, point_2_x, &self.xdata_end) {
            let tup: (f64, f64, f64) = (start, mid, *end);
            arrow_x.push(tup);    
        }
        for (start, mid, end) in izip!(point_1_y, point_2_y, &self.ydata_end) {
            let tup: (f64, f64, f64) = (start, mid, *end);
            arrow_y.push(tup);    
        }
        return (arrow_x, arrow_y)
    }

    // TODO figure out a zero(ish) vector handling strategy
    pub fn create_unit_plotly_traces(&mut self, arrow_scale: Option<f64>, uniform: bool) -> Vec<Box<Scatter<f64, f64>>> {
        self.normalise_vectors(); // normalise the vectors
        if uniform{
            let dx: f64 = self.ydata[1] - self.ydata[0];
            self.bound_node(dx);
        }
        let (barb_x, barb_y) = self.quiver_barbs();
        let (arrow_x, arrow_y) = self.gen_quiver_arrows(arrow_scale);
        let mut traces = Vec::new();
        for (x_line, y_line, x_head, y_head) in izip!(barb_x, barb_y, arrow_x, arrow_y) {
            let xpl: Vec<f64> = vec![x_line.0, x_line.1, x_head.0, x_head.1, x_head.2 ];
            let ypl: Vec<f64> = vec![y_line.0, y_line.1, y_head.0, y_head.1, y_head.2];
            let trace = Scatter::new(xpl, ypl).mode(Mode::Lines).show_legend(false).fill(Fill::ToSelf).fill_color(NamedColor::Black).show_legend(false).line(Line::new().color(NamedColor::Black));
            traces.push(trace);
        }
        return traces
    }
    // TODO add a heatmap background to the regular plot
    pub fn unit_vector_plot(&mut self, traces: Vec<Box<Scatter<f64, f64>>>, layout: Layout, square: bool, palette: ColorScale, axes: Vec<Option<Axis>>, smoothing: Option<&str>) -> Plot {
        let mut plot = Plot::new();
        //use local render version
        plot.use_local_plotly();
        for trace in traces{
            plot.add_trace(trace);
        }
        // TODO remove the need to use bound_node() so that actual unit vectors are plotted
        // TODO ensure that the arrows fit in each cell
        // Add a heatmap background to give the vectors some colour
        let smooth_setting = smoothing.unwrap_or("best".into()); 
        let heatmap: Box<HeatMap<f64, f64, f64>> = HeatMap::new(self.xdata.to_vec(), self.ydata.to_vec(), self.normdata_abs.to_vec()).zsmooth(smooth_setting).color_scale(palette);
        plot.add_trace(heatmap);
        // if this is true, then perform some additional plotly calls to create a plot where the x and y axes are equal
        if square{  
            let mut axes_iter = axes.into_iter();
            let x_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
            let y_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
            let dx_start: f64 = self.xdata[1] - self.xdata[0];
            let dx_end: f64 = self.xdata[self.xdata.len()-1] - self.xdata[self.xdata.len()-2];
            let dy_start: f64 = self.ydata[1] - self.ydata[0];
            let dy_end: f64 = self.ydata[self.ydata.len()-1] - self.ydata[self.ydata.len()-2]; 
            // with a uniform grid we can easily rescale vectors to fit in grid
            let square_layout: Layout = layout
                .y_axis(y_axis.scale_anchor("x".to_string())
                .range(vec![*self.ydata.min().unwrap() - dy_start*0.5, *self.ydata.max().unwrap() + dy_end*0.5]).scale_ratio(1.)
                )
                .x_axis(x_axis.constrain(AxisConstrain::Domain)
                .range(vec![*self.xdata.min().unwrap() - dy_start*0.5, *self.xdata.max().unwrap() + dy_end*0.5])
            );
            plot.set_layout(square_layout);
        } else{
            //plot as-is
            plot.set_layout(layout);
        }
        return plot
    }
}
    

// Generic impls
impl VectorData<f64, Ix2, Scatter<f64, f64>> for VectorData2D {

    fn scale_global(&mut self, scale_factor: f64) {
        self.udata *= scale_factor;
        self.vdata *= scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        // update scaled norms 
        self.normdata_scaled = izip!(&self.udata, &self.vdata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>();

    } 

    fn scale_elementwise(&mut self, scale_array:Array2<f64>) {
        let scale_factor: Array1<f64> = Array::from_iter(scale_array);
        self.udata = &self.udata * &scale_factor;
        self.vdata = &self.vdata * &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        // update scaled norms 
        self.normdata_scaled = izip!(&self.udata, &self.vdata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>();

    }

    /// Set minimum length for arrows.    
    fn bound_min(&mut self, min: f64) {
        let mut scale_factor: Array1<f64> = Array1::ones(self.normdata_scaled.len_of(Axis(0)));
        for i in 0..self.normdata_scaled.len_of(Axis(0)) {
            if self.normdata_scaled[i] < min {
                self.normdata_scaled[i] = min/self.normdata_scaled[i];
                scale_factor[i] = self.normdata_scaled[i];
            }
            else {
                scale_factor[i] = 1.;
            }
        }
        self.udata *= &self.normdata_scaled;
        self.vdata *= &self.normdata_scaled;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.normdata_scaled *= &scale_factor;
    }

    /// Set maximum length for arrows.
    fn bound_max(&mut self, max: f64) {
        let mut scale_factor: Array1<f64> = Array1::ones(self.normdata_scaled.len_of(Axis(0)));
        for i in 0..self.normdata_scaled.len_of(Axis(0)) {
            if self.normdata_scaled[i] > max {
                self.normdata_scaled[i] = max/self.normdata_scaled[i];
                scale_factor[i] = self.normdata_scaled[i];
            }
            else {
                scale_factor[i] = 1.;
            }
        }
        self.udata *= &scale_factor;
        self.vdata *= &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.normdata_scaled *= &scale_factor;
    }

    /// Set minimum and maximum lengths for arrows.
    fn bound_min_max(&mut self, min: f64, max: f64) {
        let mut scale_factor: Array1<f64> = Array1::ones(self.normdata_scaled.len_of(Axis(0)));
        for i in 0..self.normdata_scaled.len_of(Axis(0)) {
            if self.normdata_scaled[i] > max {
                self.normdata_scaled[i] = max/self.normdata_scaled[i];
                scale_factor[i] = self.normdata_scaled[i];
            }
            else if self.normdata_scaled[i] < min {
                self.normdata_scaled[i] = min/self.normdata_scaled[i];
                scale_factor[i] = self.normdata_scaled[i];
            }
            else {
                self.normdata_scaled[i] = 1.;
            }
        }
        self.udata *= &scale_factor;
        self.vdata *= &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.normdata_scaled *= &scale_factor;
    }

    /// Constrain all arrows to lie within circle of radius dx/2 from each node.
    /// On a non-uniform grid, this *will* distort the plot.
    fn bound_node(&mut self, dx: f64) {
        let scale_factor: f64 = 0.5*dx/self.normdata_scaled.max().unwrap();
        self.udata *= scale_factor;
        self.vdata *= scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.normdata_scaled *= scale_factor;
    }

    /// Convert u and v into unit vectors
    fn normalise_vectors(&mut self) {
        self.udata /= &self.normdata_scaled;
        self.vdata /= &self.normdata_scaled;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
    }

    fn normalise_colour(&self, colour_bounds: Option<(f64, f64)>) -> (Array1<f64>, f64, f64) {
        match colour_bounds {
            None => {
                let min: f64 = *self.normdata_scaled.min().unwrap();
                let max: f64 = *self.normdata_scaled.max().unwrap();
                let colour_vector: Array1<f64> = (&self.normdata_abs - min)/(max - min);
                return (colour_vector, min, max)
            },
    
            Some((min, max)) => {
                assert!(min < max, "Max needs to be greater than min!");
                let min = min;
                let max = max;
                let colour_vector = (&self.normdata_scaled - min)/(max - min);
                return (colour_vector, min, max)
            }
        }
    }
    /// Returns ```Plotly::Scatter``` traces for plotting.
    /// 
    /// Inputs:
    /// 1. data
    /// * ```VectorData2D``` struct.
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
    fn create_plotly_traces(&self, arrow_scale: Option<f64>, colourmap: Gradient, colour_bounds: Option<(f64, f64)>) -> Vec<Box<Scatter<f64,f64>>>  {
        let (colour_vector, min, max) = self.normalise_colour(colour_bounds);
        let (barb_x, barb_y) = self.quiver_barbs();
        let (arrow_x, arrow_y) = self.gen_quiver_arrows(arrow_scale);
        let mut traces = Vec::new();
        let mut colour_elements: Vec<ColorScaleElement> = Vec::new();
        //unpack the vectors of arrow barbs and heads into new vector containing each arrow as a tuple
        for (x_line, y_line, x_head, y_head, c) in izip!(barb_x, barb_y, arrow_x, arrow_y, colour_vector) {
            let xpl: Vec<f64> = vec![x_line.0, x_line.1, x_head.0, x_head.1, x_head.2 ];
            let ypl: Vec<f64> = vec![y_line.0, y_line.1, y_head.0, y_head.1, y_head.2];
            //color for trace
            let color: String = format!("#{:x}", colourmap.eval_continuous(c));
            let s: String = color.clone();
            let element: ColorScaleElement = ColorScaleElement::new(c, s);
            colour_elements.push(element);
            let trace = Scatter::new(xpl, ypl).mode(Mode::Lines).show_legend(false).fill(Fill::ToSelf).fill_color(&color).show_legend(false).line(Line::new().color(&color));
            traces.push(trace);
        }    
        //create an invisible marker to get the colorbar to appear - use the same map as above
        let invisible_marker = Scatter::new(vec![self.xdata[0]],vec![self.ydata[0]])
        .mode(Mode::Markers)
        .marker(Marker::new().cmin(min).cmax(max).color_scale(ColorScale::Vector(colour_elements)).color_bar(ColorBar::new()).size(1)).show_legend(false);
        traces.push(invisible_marker);
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
    /// let layout = Layout::new()
    ///                .title("Quiver plot".into());
    /// let mut plot = vector2d::plot(traces, layout, true); 
    fn vector_plot(&self, traces: Vec<Box<Scatter<f64, f64>>>, layout: Layout, square: bool, axes: Vec<Option<Axis>>) -> Plot {
        let mut plot = Plot::new();
        //use local render version
        plot.use_local_plotly();
        for trace in traces{
            plot.add_trace(trace);
        }
        // if this is true, then perform some additional plotly calls to create a plot where the x and y axes are equal
        if square{  
            let mut axes_iter = axes.into_iter();
            let x_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
            let y_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
            let square_layout: Layout = layout.y_axis(y_axis.scale_anchor("x".to_string())
            .scale_ratio(1.)).x_axis(x_axis.constrain(AxisConstrain::Domain));
            plot.set_layout(square_layout);
        } else{
            //plot as-is
            plot.set_layout(layout);
        }
        return plot
    }

    fn auto_axis_range(&self, layout: Layout, axes: Vec<Axis>, dtick: f64) -> Layout {
        let xmin: f64 = self.xdata.min().unwrap() - dtick;
        let xmax: f64 = self.xdata.max().unwrap() + dtick;
        let ymin: f64 = self.ydata.min().unwrap() - dtick;
        let ymax: f64 = self.ydata.max().unwrap() + dtick;
        let mut axes_iter = axes.into_iter();
        let xaxis: Axis = axes_iter.next().unwrap();
        let yaxis: Axis = axes_iter.next().unwrap();
        let x_auto: Layout = axis_range_x(layout, xaxis, xmin, xmax);
        let xy_auto: Layout = axis_range_y(x_auto, yaxis, ymin, ymax);
        return xy_auto
    }

}

// Convenience functions that act on a plotly plot
/// Manually set x axis range
pub fn axis_range_x(layout: Layout, xaxis: Axis, xmin: f64, xmax:f64) -> Layout {
    let new_layout: Layout = layout.x_axis(xaxis.range(vec![xmin, xmax]));
    return new_layout
}

/// Manually set y axis range
pub fn axis_range_y(layout: Layout, yaxis: Axis, ymin: f64, ymax:f64) -> Layout {
    let new_layout: Layout = layout.y_axis(yaxis.range(vec![ymin, ymax]));
    return new_layout
}