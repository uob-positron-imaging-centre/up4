//! Submodule for handling 3D vector data.

use derive_getters::Getters;
use itertools::izip;
use ndarray::prelude::*;
use plotly::{Cone, Scatter3D, Surface};
use plotly::color::NamedColor;
use plotly::common::{ColorScale};
use std::f64::consts::PI;
use plotly::common::{Fill, Line, Mode, Marker, ColorBar, ColorScaleElement};
use plotly::{Plot, Scatter, HeatMap, Trace, layout::{Layout, AxisConstrain}};
use core::panic;
use plotly::cone::Anchor;
use plotly::heat_map::Smoothing;
use ndarray_stats::QuantileExt;
use colorous::Gradient;
use crate::{GridFunctions3D,  component_data_selector};
use crate::vector_grid::VectorGrid;
use crate::utilities::maths::{meshgrid, meshgrid3d};

/// Vector data handling struct. The `true_norm` field contains the original vector norms, and is used for shading. Norms used for drawing arrows are not necessarily the same as they may be scaled for display reasons.
#[derive(Getters, Clone)]
pub struct VectorPlotter {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    zdata: Array1<f64>,
    udata: Array3<f64>,
    vdata: Array3<f64>,
    wdata: Array3<f64>,
    true_norm: Array3<f64>,
}


impl VectorPlotter {
    /// Constructor
    pub fn new(grid: VectorGrid) -> VectorPlotter {
        let xdata: Array1<f64> = grid.get_xpositions().to_owned();
        let ydata: Array1<f64> = grid.get_ypositions().to_owned();
        let zdata: Array1<f64> = grid.get_zpositions().to_owned();
        let udata: Array3<f64> = grid.data[0].get_data().to_owned();
        let vdata: Array3<f64> = grid.data[1].get_data().to_owned();
        let wdata: Array3<f64> = grid.data[2].get_data().to_owned();
        let norm: Vec<f64> = izip!(&udata, &vdata, &wdata).map(|x| (x.0.powi(2) + x.1.powi(2) + x.2.powi(2)).powf(0.5)).collect::<Vec<f64>>();
        let true_norm: Array3<f64> = Array3::from_shape_vec(udata.raw_dim(), norm).unwrap();    
        return VectorPlotter {
            xdata: xdata,
            ydata: ydata,
            zdata: zdata,
            udata: udata,
            vdata: vdata,
            wdata: wdata,
            true_norm: true_norm,
        }
    }

    /// Scale all vector elements by a singular scale factor.
    pub fn scale_global(&mut self, scale_factor: f64) {
        self.udata *= scale_factor;
        self.vdata *= scale_factor;
        self.wdata *= scale_factor;
    } 
    /// Scale vector elements elementwise. Each component with the same indices will be scaled identically.
    pub fn scale_elementwise(&mut self, scale_factor:Array3<f64>) {
        self.udata = &self.udata * &scale_factor;
        self.vdata = &self.vdata * &scale_factor;
        self.wdata = &self.wdata * &scale_factor;
    }

    /// Set minimum length for arrows.    
    pub fn bound_min(&mut self, min: f64) {
        let mut scale_factor: Array3<f64> = Array3::ones(self.udata.raw_dim());
        for i in 0..self.xdata.len() {
            for j in 0..self.xdata.len() {
                for k in 0..self.xdata.len() {
                    if self.true_norm[[i, j, k]] < min {
                        scale_factor[[i, j, k]] = min/self.true_norm[[i, j, k]];
                    }
                    else {
                        scale_factor[[i, j, k]] = 1.;
                    }
                }
            } 
        }
        self.udata *= &scale_factor;
        self.vdata *= &scale_factor;
        self.wdata *= &scale_factor;
    }

    /// Set maximum length for arrows.
    pub fn bound_max(&mut self, max: f64) {
        let mut scale_factor: Array3<f64> = Array3::ones(self.udata.raw_dim());
        for i in 0..self.xdata.len() {
            for j in 0..self.xdata.len() {
                for k in 0..self.xdata.len() {
                    if self.true_norm[[i, j, k]] > max {
                        scale_factor[[i, j, k]] = max/self.true_norm[[i, j, k]];
                    }
                    else {
                        scale_factor[[i, j, k]] = 1.;
                    }
                }
            } 
        }
    }

    /// Set minimum and maximum lengths for arrows.
    pub fn bound_min_max(&mut self, min: f64, max: f64) {
        let mut scale_factor: Array3<f64> = Array3::ones(self.udata.raw_dim());
        for i in 0..self.xdata.len() {
            for j in 0..self.xdata.len() {
                for k in 0..self.xdata.len() {
                    if self.true_norm[[i, j, k]] > max {
                        scale_factor[[i, j, k]] = max/self.true_norm[[i, j, k]];
                    }
                    else if self.true_norm[[i, j, k]] < min {
                        scale_factor[[i, j, k]] = min/self.true_norm[[i, j, k]];
                    }
                    else {
                        scale_factor[[i, j, k]] = 1.;
                    }
                }
            } 
        }
        self.udata *= &scale_factor;
        self.vdata *= &scale_factor;
        self.wdata *= &scale_factor;
    }

    // FIXME select the smallest circle for each cell so that we can handle cuboid cells
    /// Constrain all arrows to lie within circle of radius dx/2 from each node.
    /// On a non-uniform grid, this *will* distort the plot.
    pub fn bound_node(&mut self, dx: f64) {
        let scale_factor: f64 = 0.5*dx/self.true_norm.max_skipnan();
        self.scale_global(scale_factor);
    }

    /// Convert vectors into unit vectors.
    pub fn normalise_vectors(&mut self) {
        self.udata/=&self.true_norm;
        self.vdata/=&self.true_norm;
        self.wdata/=&self.true_norm;
    }

    /// Map vector norm values to the interval [0, 1]
    pub fn normalise_colour(&self, colour_bounds: Option<(f64, f64)>) -> (Array3<f64>, f64, f64) {
        match colour_bounds {
            None => {
                let min: f64 = *self.true_norm.min_skipnan();
                let max: f64 = *self.true_norm.max_skipnan();
                let colour_vector: Array3<f64> = (&self.true_norm - min)/(max - min);
                return (colour_vector, min, max)
            },
    
            Some((min, max)) => {
                assert!(min < max, "Max needs to be greater than min!");
                let min = min;
                let max = max;
                let colour_vector = (&self.true_norm - min)/(max - min);
                return (colour_vector, min, max)
            }
        }
    }
    
    /// Create arrow traces for vectors.
    pub fn create_quiver_traces(&self, arrow_scale: Option<f64>, colourmap: Gradient, colour_bounds: Option<(f64, f64)>, axis: usize, index: usize) -> Vec<Box<Scatter<f64,f64>>>  {
        let (colour_vector, min, max) = self.normalise_colour(colour_bounds);
        let (barb_x, barb_y) = self.create_quiver_barbs(axis, index);
        let (arrow_x, arrow_y) = self.create_quiver_arrows(arrow_scale, axis, index);
        let mut traces = Vec::new();
        let colour_elements: Vec<ColorScaleElement> = Vec::new();
        //unpack the vectors of arrow barbs and heads into new vector containing each arrow as a tuple
        for (x_line, y_line, x_head, y_head, c) in izip!(barb_x, barb_y, arrow_x, arrow_y, colour_vector) {
            let xpl: Vec<f64> = vec![x_line.0, x_line.1, x_head.0, x_head.1, x_head.2 ];
            let ypl: Vec<f64> = vec![y_line.0, y_line.1, y_head.0, y_head.1, y_head.2];
            //color for trace
            let color: String = format!("#{:x}", colourmap.eval_continuous(c));
            let s: String = color.clone();
            //let element: ColorScaleElement = ColorScaleElement::new(c, s);
            //colour_elements.push(element);
            let trace = Scatter::new(xpl, ypl).mode(Mode::Lines).show_legend(false).fill(Fill::ToSelf).fill_color(color).show_legend(false).line(Line::new().color(s));
            traces.push(trace);
        }    
        //create an invisible marker to get the colorbar to appear - use the same map as above
        let invisible_marker = Scatter::new(vec![self.xdata[0]],vec![self.ydata[0]])
        .mode(Mode::Markers)
        .marker(Marker::new().cmin(min).cmax(max).color_scale(ColorScale::Vector(colour_elements)).color_bar(ColorBar::new()).size(1)).show_legend(false);
        traces.push(invisible_marker);
        return traces
    }

    // TODO choose dims to select min and max from
    /// Automatically set axis limits for 2D plots.
    pub fn auto_axis_range(&self, layout: Layout, axes: Vec<plotly::layout::Axis>, dtick: f64) -> Layout {
        let xmin: f64 = self.xdata.min_skipnan() - dtick;
        let xmax: f64 = self.xdata.max_skipnan() + dtick;
        let ymin: f64 = self.ydata.min_skipnan() - dtick;
        let ymax: f64 = self.ydata.max_skipnan() + dtick;
        let mut axes_iter = axes.into_iter();
        let xaxis: plotly::layout::Axis = axes_iter.next().unwrap();
        let yaxis: plotly::layout::Axis = axes_iter.next().unwrap();
        let x_auto: Layout = axis_range_x(layout, xaxis, xmin, xmax);
        let xy_auto: Layout = axis_range_y(x_auto, yaxis, ymin, ymax);
        return xy_auto
    }

    /// Create the arrow shafts.
    fn create_quiver_barbs(&self, axis: usize, index: usize) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let mut barb_x = Vec::new();
        let mut barb_y = Vec::new();
        let (x, y, u, v) = self.component_axis_selector(axis);
        let (x, y) = meshgrid(x, y);
        let u = component_data_selector(u, axis, index);
        let v = component_data_selector(v, axis, index);
        for (x, y, u , v) in izip!(x, y, u, v) {
            let tupx: (f64, f64) = (x, x + u);
            let tupy: (f64, f64) = (y, y + v);
            barb_x.push(tupx); 
            barb_y.push(tupy);  
        }
        return (barb_x, barb_y)
    }
    /// Create the arrowheads.
    fn create_quiver_arrows(&self, arrow_scale: Option<f64>, axis: usize, index: usize) -> (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>) {
        // select the required data 
        let (x, y, u, v) = self.component_axis_selector(axis);
        let (x, y) = meshgrid(x, y);
        let x: Array1<f64> = Array1::from_iter(x);
        let y: Array1<f64> = Array1::from_iter(y);
        let u: Array1<f64> = Array1::from_iter(component_data_selector(u, axis, index));
        let v: Array1<f64> = Array1::from_iter(component_data_selector(v, axis, index));
        let norm: Array1<f64> = izip!(&u, &v).map(|(v,u)| f64::hypot(*u,*v)).collect::<Array1<f64>>();

        // angle between arrows
        const ANGLE: f64 = PI/9.0; 
        // default scale is 0.5
        let scale_factor: f64 = arrow_scale.unwrap_or(0.5);
        
        let arrow_len: Array1<f64> = scale_factor * norm;
        // get barb angles
        let barb_ang: Array1<f64> = izip!(&v, &u).map(|(v, u)| f64::atan2(*v,*u)).collect::<Array1<f64>>();
        
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
        let point_1_x: Array1<f64> = (&x + &u).into_iter().collect::<Array1<f64>>() - seg_1_x;
        let point_1_y: Array1<f64> = (&y + &v).into_iter().collect::<Array1<f64>>() - seg_1_y;
        let point_2_x: Array1<f64> = (&x + &u).into_iter().collect::<Array1<f64>>() - seg_2_x;
        let point_2_y: Array1<f64> = (&y + &v).into_iter().collect::<Array1<f64>>() - seg_2_y;
        
        //finally, combine this data into something usable
        let mut arrow_x = Vec::new();
        let mut arrow_y = Vec::new();
        
        for (start, mid, end) in izip!(point_1_x, point_2_x, &x + &u) {
            let tup: (f64, f64, f64) = (start, mid, end);
            arrow_x.push(tup);    
        }
        for (start, mid, end) in izip!(point_1_y, point_2_y, &y + &v) {
            let tup: (f64, f64, f64) = (start, mid, end);
            arrow_y.push(tup);    
        }
        return (arrow_x, arrow_y)
    }

    // TODO figure out a zero(ish) vector handling strategy
    /// Create arrow traces, but for the data converted into unit vectors.
    pub fn create_unit_vector_traces(&mut self, arrow_scale: Option<f64>, uniform: bool, axis: usize, index: usize) -> Vec<Box<Scatter<f64, f64>>> {
        self.normalise_vectors();
        if uniform{
            let dx: f64 = self.xdata[1] - self.xdata[0];
            self.bound_node(dx);
        }
        let (barb_x, barb_y) = self.create_quiver_barbs(axis, index);
        let (arrow_x, arrow_y) = self.create_quiver_arrows(arrow_scale, axis, index);
        let mut traces: Vec<Box<Scatter<f64, f64>>>  = Vec::new();
        for (x_line, y_line, x_head, y_head) in izip!(barb_x, barb_y, arrow_x, arrow_y) {
            let xpl: Vec<f64> = vec![x_line.0, x_line.1, x_head.0, x_head.1, x_head.2 ];
            let ypl: Vec<f64> = vec![y_line.0, y_line.1, y_head.0, y_head.1, y_head.2];
            let trace = Scatter::new(xpl, ypl).mode(Mode::Lines).show_legend(false)
            .fill(Fill::ToSelf).fill_color(NamedColor::Black)
            .show_legend(false).line(Line::new().color(NamedColor::Black));
            traces.push(trace);
        }
        return traces
    }

    /// Create the background for the unit vector plot, coloured by norm values.
    pub fn create_unit_vector_background(&self, layout: Layout, square: bool, axes: Vec<Option<plotly::layout::Axis>>, smoothing: Option<Smoothing>, axis: usize, index: usize) -> (Box<HeatMap<f64, f64, f64>>, Layout) {
        // TODO remove the need to use bound_node() so that actual unit vectors are plotted
        // TODO ensure that the arrows fit in each cell
        // Add a heatmap background to give the vectors some colour
        let (xaxis, yaxis) = self.axis_selector(axis);
        let (xaxis, yaxis) = meshgrid(xaxis, yaxis);
        let plot_data = component_data_selector(self.true_norm.to_owned(), axis, index);
        let smooth_setting = smoothing.unwrap_or(Smoothing::False); 
        let heatmap: Box<HeatMap<f64, f64, f64>> = HeatMap::new(xaxis.into_raw_vec(), yaxis.into_raw_vec(), plot_data.into_raw_vec()).zsmooth(smooth_setting);
        // if this is true, then perform some additional plotly calls to create a plot where the x and y axes are equal
        // FIXME dx and dy parameters need to be independent of the plane selected
        // TODO implement scale_ratio in plotly fork
        if square{  
            let mut axes_iter = axes.into_iter();
            let x_axis: plotly::layout::Axis = axes_iter.next().unwrap().unwrap_or(plotly::layout::Axis::new());
            let y_axis: plotly::layout::Axis = axes_iter.next().unwrap().unwrap_or(plotly::layout::Axis::new());
            let dx_start: f64 = self.xdata[1] - self.xdata[0];
            let dx_end: f64 = self.xdata[self.xdata.len()-1] - self.xdata[self.xdata.len()-2];
            let dy_start: f64 = self.ydata[1] - self.ydata[0];
            let dy_end: f64 = self.ydata[self.ydata.len()-1] - self.ydata[self.ydata.len()-2]; 
            // with a uniform grid we can easily rescale vectors to fit in grid
            let layout = layout.y_axis(y_axis.anchor("x")
                .range(vec![*self.ydata.min_skipnan() - dy_start*0.5, *self.ydata.max_skipnan() + dy_end*0.5])//.scale_ratio(1.)
                )
                .x_axis(x_axis.constrain(AxisConstrain::Domain)
                .range(vec![*self.xdata.min_skipnan() - dx_start*0.5, *self.xdata.max_skipnan() + dx_end*0.5])
            );
            return (heatmap, layout)
        }
        return (heatmap, layout)
    }

    /// Take traces and plot them
    pub fn plot(&self, traces: Vec<Box<dyn Trace>>, layout: Layout, show: bool) -> Plot {
        let mut plot: Plot = Plot::new();
        //use local render version
        plot.use_local_plotly();
        for trace in traces{
            plot.add_trace(trace);
        }
        plot.set_layout(layout);
        if show{
            plot.show();
        }
        return plot
    }

    // TODO create
    //fn quiver_slices(&self, nevery: usize,  traces: Vec<Box<Scatter<f64, f64>>>, layout: Layout, square: bool, axes: Vec<Option<Axis>>) {
        
    //}

    // TODO create
    //fn save(&self, plot: Plot, filename: &str, dpi: usize) {
        
    //}

    // FIXME doc
    // range is [start, stop, step]
    // use https://plotly.com/python/v3/3d-filled-line-plots/ to fill in the arrows
    // regardless of selected axis, we use xaxis as "x" and zaxis as "y", stepping along yaxis
    pub fn unit_vector_slice_traces(&mut self, range: [usize; 3], axis: usize, arrow_scale: Option<f64>, uniform: bool) -> Vec<Box<Scatter3D<f64, f64, f64>>> {
        self.normalise_vectors();
        if uniform{
            let dx: f64 = self.ydata[1] - self.ydata[0];
            self.bound_node(dx);
        }
        let mut traces = Vec::new();
        for index in (range[0]..range[1]).step_by(range[2]) {
            let (barb_x, barb_y) = self.create_quiver_barbs(axis, index);
            let (arrow_x, arrow_y) = self.create_quiver_arrows(arrow_scale, axis, index);
            for (x_line, y_line, x_head, y_head) in izip!(barb_x, barb_y, arrow_x, arrow_y) {
                let xpl: Vec<f64> = vec![x_line.0, x_line.1, x_head.0, x_head.1, x_head.2 ];
                let mut ypl: Array1<f64> = Array1::ones(5);
                ypl *= self.ydata[index];
                // cast ypl to Vec
                let ypl = ypl.into_raw_vec();
                let zpl: Vec<f64> = vec![y_line.0, y_line.1, y_head.0, y_head.1, y_head.2];
                let trace = Scatter3D::new(xpl, ypl, zpl).mode(Mode::Lines).show_legend(false).show_legend(false).line(Line::new().color(NamedColor::Black));
                traces.push(trace);
            }
        }
        return traces
    }

    // FIXME doc
    // FIXME create colorbar separate to traces which covers the whole range of data selected
    // same as above -> slices plotted along y
    pub fn unit_vector_slice_background(&self, range: [usize; 3], axis: usize) -> Vec<Box<Surface<f64, f64, f64>>> {
        let mut traces = Vec::new();
        for index in (range[0]..range[1]).step_by(range[2]) {
            // select data
            let (x, _y, _ ,_) = self.component_axis_selector(axis);
            let norm: Array2<f64> = component_data_selector(self.true_norm.to_owned(), axis, index);
            let xpl: Vec<f64> = x.to_owned().into_raw_vec();
            let mut ypl: Array1<f64> = Array1::ones(x.len());
            ypl *= self.ydata[index];
            // cast ypl to Vec
            let ypl = ypl.into_raw_vec();
            let mut zpl = Vec::new();
            for row in norm.axis_iter(ndarray::Axis(1)) {
                let inner_vec = row.to_vec();
                zpl.push(inner_vec);
            }
            let heatmap: Box<Surface<f64, f64, f64>> = Surface::new(zpl).x(xpl).y(ypl);
            traces.push(heatmap);
        }
        return traces
    }

    fn create_cone_traces(&self, colour_bounds: Option<(f64, f64)>) -> Vec<Box<Cone<f64,f64,f64,f64,f64,f64>>> {
        let (_colour_vector, _min, _max) = self.normalise_colour(colour_bounds);
        let (x, y, z) = meshgrid3d(self.xdata.to_owned(), self.ydata.to_owned(), self.zdata.to_owned());
        let x: Vec<f64> = x.into_raw_vec();
        let y: Vec<f64> = y.into_raw_vec();
        let z: Vec<f64> = z.into_raw_vec();
        let u: Vec<f64> = self.udata.clone().into_raw_vec();
        let v: Vec<f64> = self.vdata.clone().into_raw_vec();
        let w: Vec<f64> = self.wdata.clone().into_raw_vec();

        let trace = Cone::new(x, y, z, u, v, w)
                .anchor(Anchor::Tip)
                .show_legend(false);
        let cone_trace = vec![trace];
        return cone_trace
        }  
    
        //FIXME doc
    fn component_axis_selector(&self, axis: usize) -> (Array1<f64>, Array1<f64>, Array3<f64>, Array3<f64>) {
        match axis {
            // yz view
            0 => {
                let xcomponent = self.ydata.to_owned();
                let ycomponent = self.zdata.to_owned();
                let xdata = self.vdata.to_owned();
                let ydata = self.wdata.to_owned();
                return (xcomponent, ycomponent, xdata, ydata)
            }
            // xz view
            1 => {
                let xcomponent = self.xdata.to_owned();
                let ycomponent = self.zdata.to_owned();
                let xdata = self.udata.to_owned();
                let ydata = self.wdata.to_owned();
                return (xcomponent, ycomponent, xdata, ydata)
            }
            // xy view
            2 => {
                let xcomponent = self.xdata.to_owned();
                let ycomponent = self.ydata.to_owned();
                let xdata = self.udata.to_owned();
                let ydata = self.vdata.to_owned();
                return (xcomponent, ycomponent, xdata, ydata)
            }
            // panic
            _ => panic!("axis value must be either 0, 1 or 2!")
        };
    }
    //FIXME doc
    fn axis_selector(&self, axis: usize) -> (Array1<f64>, Array1<f64>) {
        match axis {
            // yz view
            0 => {
                let xcomponent = self.ydata.to_owned();
                let ycomponent = self.zdata.to_owned();
                return (xcomponent, ycomponent)
            }
            // xz view
            1 => {
                let xcomponent = self.xdata.to_owned();
                let ycomponent = self.zdata.to_owned();
                return (xcomponent, ycomponent)
            }
            // xy view
            2 => {
                let xcomponent = self.xdata.to_owned();
                let ycomponent = self.ydata.to_owned();
                return (xcomponent, ycomponent)
            }
            // panic
            _ => panic!("axis value must be either 0, 1 or 2!")
        };
    }
}

/// Manually set x axis range
pub fn axis_range_x(layout: Layout, xaxis: plotly::layout::Axis, xmin: f64, xmax:f64) -> Layout {
    let new_layout: Layout = layout.x_axis(xaxis.range(vec![xmin, xmax]));
    return new_layout
}

/// Manually set y axis range
pub fn axis_range_y(layout: Layout, yaxis: plotly::layout::Axis, ymin: f64, ymax:f64) -> Layout {
    let new_layout: Layout = layout.y_axis(yaxis.range(vec![ymin, ymax]));
    return new_layout
}