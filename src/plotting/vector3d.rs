//! this will eventually plot 3D vectors

use ndarray;
use itertools;
use itertools::izip;
use ndarray::prelude::*;
use plotly::cone::Anchor;
use plotly::common::{Line, Mode};
use plotly::{Plot, Cone, Layout};
use plotly::layout::Axis;
use core::panic;
use ndarray_stats::QuantileExt;

use super::VectorData;
use super::vector2d::{axis_range_x, axis_range_y};

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
/// The following 2 fields are defined through the use of associated function ```VectorData3D::scale```.
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
/// let arrows: VectorData3D = vector3d::VectorData3D::new(x,y,z,u,v,w,scale_mode);
/// ```
pub struct VectorData3D {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    zdata: Array1<f64>,
    udata: Array1<f64>,
    vdata: Array1<f64>,
    wdata: Array1<f64>,
    xdata_end: Array1<f64>,
    ydata_end: Array1<f64>,
    zdata_end: Array1<f64>,
    normdata_scaled: Array1<f64>,
    normdata_abs: Array1<f64>,
}

impl VectorData3D{
    /// constructor for VectorData3D struct
    pub fn new(x:Array3<f64>, y:Array3<f64>, z:Array3<f64>, u:Array3<f64>, v:Array3<f64>, w:Array3<f64>) -> VectorData3D { 
        //supercede the default error message for shape mismatch as it doesn't identify the offending array
        assert!(&x.dim() == &y.dim(),"Array dimension mismatch!\nx has dimensions {:?}\ny has dimensions {:?}", &x.dim(), &y.dim());
        assert!(&x.dim() == &z.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nz has dimensions {:?}", &x.dim(), &z.dim());
        assert!(&x.dim() == &u.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nu has dimensions {:?}", &x.dim(), &u.dim());
        assert!(&x.dim() == &v.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nv has dimensions {:?}", &x.dim(), &v.dim());
        assert!(&x.dim() == &w.dim(),"Array dimension mismatch!\nx has dimensions {:?}\nw has dimensions {:?}", &x.dim(), &w.dim());
        let xdata: Array1<f64> = Array::from_iter(x);
        let ydata: Array1<f64> = Array::from_iter(y);
        let zdata: Array1<f64> = Array::from_iter(z);
        let udata: Array1<f64> = Array::from_iter(u);
        let vdata: Array1<f64> = Array::from_iter(v);
        let wdata: Array1<f64> = Array::from_iter(w);
        let xdata_end: Array1<f64> = &xdata + &udata;
        let ydata_end: Array1<f64> = &ydata + &vdata;
        let zdata_end: Array1<f64> = &zdata + &wdata;
        let normdata_scaled: Array1<f64> = izip!(&udata, &vdata, &wdata).map(|x| (x.0.powi(2) + x.1.powi(2) + x.2.powi(2)).powf(0.5)).collect::<Array1<f64>>();
        // initially unscaled
        let normdata_abs: Array1<f64> = normdata_scaled.clone();

        return VectorData3D {
            xdata,
            ydata,
            zdata,
            udata, 
            vdata,
            wdata,
            xdata_end, 
            ydata_end,
            zdata_end,
            normdata_abs,
            normdata_scaled,
        }
    }

}
impl VectorData<f64, Ix3, Cone<f64,f64,f64,f64,f64,f64>> for VectorData3D {

    /// Scales 3D vector data by a single scale factor
    fn scale_global(&mut self, scale_factor: f64) {
        self.udata *= scale_factor;
        self.vdata *= scale_factor;
        self.wdata *= scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        // update scaled norms
        self.normdata_scaled *= scale_factor;
    }

    fn scale_elementwise(&mut self, scale_array:Array<f64, Ix3>) {
        let scale_factor: Array1<f64> = Array::from_iter(scale_array);
        self.udata = &self.udata * &scale_factor;
        self.vdata = &self.vdata * &scale_factor;
        self.wdata = &self.udata * &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        // update scaled norms
        //self.normdata_scaled.assign(&(&self.normdata_scaled * scale_factor));
        self.normdata_scaled *= &scale_factor;
    }

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
        self.udata *= &scale_factor;
        self.vdata *= &scale_factor;
        self.wdata *= &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        self.normdata_scaled *= &scale_factor;
    }

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
        self.wdata *= &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        self.normdata_scaled *= &scale_factor;
    }

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
        self.wdata *= &scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        self.normdata_scaled *= &scale_factor;
    }

    /// Constrain all arrows to lie within circle of radius dx/2 from each node.
    /// On a non-uniform grid, this *will* distort the plot.
    fn bound_node(&mut self, dx: f64) {
        //println!("{},{},{}",self.xdata[1], self.xdata[2], dx);
        let scale_factor: f64 = 0.5*dx/self.normdata_scaled.max().unwrap();
        self.udata *= scale_factor;
        self.vdata *= scale_factor;
        self.wdata *= scale_factor;
        self.xdata_end = &self.xdata + &self.udata;
        self.ydata_end = &self.ydata + &self.vdata;
        self.zdata_end = &self.zdata + &self.wdata;
        self.normdata_scaled *= scale_factor;
    }

    /// Convert u, v and w into unit vectors
    fn normalise_vectors(&mut self) {
        self.udata /= &self.normdata_scaled;
        self.vdata /= &self.normdata_scaled;
        self.wdata /= &self.normdata_scaled;
    }

    fn normalise_colour(&self, colour_bounds: Option<(f64, f64)>) -> (Array1<f64>, f64, f64) {
        match colour_bounds {
            None => {
                let min: f64 = *self.normdata_scaled.min().unwrap();
                let max: f64 = *self.normdata_scaled.max().unwrap();
                let colour_vector: Array1<f64> = (&self.normdata_scaled - min)/(max - min);
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

    fn create_plotly_traces(&self, arrow_scale: Option<f64>, colour: colorous::Gradient, colour_bounds: Option<(f64, f64)>) -> Vec<Box<Cone<f64,f64,f64,f64,f64,f64>>> {
        let (colour_vector, min, max) = self.normalise_colour(colour_bounds);
        let x: Array1<f64> = self.xdata_end.clone();
        let y: Array1<f64> = self.ydata_end.clone();
        let z: Array1<f64> = self.zdata_end.clone();
        let u: Array1<f64> = self.udata.clone();
        let v: Array1<f64> = self.vdata.clone();
        let w: Array1<f64> = self.wdata.clone();

        let trace = Cone::new(x, y, z, u, v, w)
                .anchor(Anchor::Tip)
                .show_legend(false)
                
                ;
        let cone_trace = vec![trace];
        return cone_trace
        }   

    fn vector_plot(&self, traces: Vec<Box<Cone<f64,f64,f64,f64,f64,f64>>>, layout: plotly::Layout, square: bool, axes: Vec<Option<plotly::layout::Axis>>) -> Plot {
        let mut plot: Plot = Plot::new();
        plot.use_local_plotly();
        for trace in traces{
            plot.add_trace(trace);
        }
        let mut axes_iter = axes.into_iter();
        let x_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
        let y_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
        let z_axis: Axis = axes_iter.next().unwrap().unwrap_or(Axis::new());
        plot.set_layout(layout);
        return plot
    }
    
    fn auto_axis_range(&self, layout: plotly::Layout, axes: Vec<plotly::layout::Axis>, dtick: f64) -> plotly::Layout {
        let xmin: f64 = self.xdata.min().unwrap() - dtick;
        let xmax: f64 = self.xdata.max().unwrap() + dtick;
        let ymin: f64 = self.ydata.min().unwrap() - dtick;
        let ymax: f64 = self.ydata.max().unwrap() + dtick;
        let zmin: f64 = self.zdata.min().unwrap() - dtick;
        let zmax: f64 = self.zdata.max().unwrap() + dtick;
        let mut axes_iter = axes.into_iter();
        let xaxis: Axis = axes_iter.next().unwrap();
        let yaxis: Axis = axes_iter.next().unwrap();
        let zaxis: Axis = axes_iter.next().unwrap();
        // TODO mess with scene to achieve this
        let x_auto: Layout = axis_range_x(layout, xaxis, xmin, xmax);
        let xy_auto: Layout = axis_range_y(x_auto, yaxis, ymin, ymax);
        return xy_auto
    }

}