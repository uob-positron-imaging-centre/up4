//! Submodule handling plots comparing 2 datasets (reference ("the ground truth") and comparison data) for equality this submodule assumes that the grids containing both datasets are identically laid out.
use ndarray_stats::QuantileExt;
use plotly::{Scatter, Layout, HeatMap, common::{Line, Mode, Marker, MarkerSymbol}, Surface, surface::SurfaceContours, Plot, Trace, color::NamedColor};
use crate::{GridFunctions3D, axis_selector, data_selector};
use ndarray::prelude::*;
use crate::utilities::maths::{meshgrid, flatten_2d};

/// Comparison data handling struct.
pub struct ComparisonPlotter {
    reference_data: Box<dyn GridFunctions3D>,
    comparison_data: Box<dyn GridFunctions3D>,
}

impl ComparisonPlotter {
    /// Constructor
    pub fn new(reference_data: Box<dyn GridFunctions3D>, comparison_data: Box<dyn GridFunctions3D>) -> ComparisonPlotter {
        if reference_data.get_data().len() != comparison_data.get_data().len() {
            panic!("Provided reference and comparison grids are unequal shapes!")
        }
        return ComparisonPlotter { 
            reference_data: reference_data, 
            comparison_data: comparison_data 
        }
    }

    /// Return traces corresponding to the parity line, and the data itself.
    pub fn create_parity_traces(&self) -> Vec<Box<Scatter<f64, f64>>> {
        // determine the extent of the parity line
        // assign all possible values for (xmin, ymin), (xmax, ymax)
        // so that one can actually read the code!
        let xmin_reference = *self.reference_data.get_data().min_skipnan();
        let xmax_reference = *self.reference_data.get_data().max_skipnan();
        let xmin_comparison = *self.comparison_data.get_data().min_skipnan();
        let xmax_comparison = *self.comparison_data.get_data().max_skipnan();
        let ymin_reference = *self.reference_data.get_data().min_skipnan();
        let ymax_reference = *self.reference_data.get_data().max_skipnan();
        let ymin_comparison = *self.comparison_data.get_data().min_skipnan();
        let ymax_comparison = *self.comparison_data.get_data().max_skipnan();
        let xmin = if xmin_reference < xmin_comparison {xmin_reference} else {xmin_comparison};
        let xmax = if xmax_reference < xmax_comparison {xmax_reference} else {xmax_comparison};
        let ymin = if ymin_reference < ymin_comparison {ymin_reference} else {ymin_comparison};
        let ymax = if ymax_reference < ymax_comparison {ymax_reference} else {ymax_comparison};
        // parity line trace
        let parity_line = Scatter::new(vec![xmin, xmax], vec![ymin, ymax]).mode(Mode::Lines).show_legend(false).line(Line::new().color(NamedColor::Black));
        // parity scatter trace
        let parity_scatter = Scatter::new(self.reference_data.get_data().to_owned().into_raw_vec(), self.comparison_data.get_data().to_owned().into_raw_vec()).mode(Mode::Markers).marker(Marker::new().symbol(MarkerSymbol::Cross)).show_legend(false);
        let traces = vec![parity_line, parity_scatter];
        return traces
    }

    /// Return heatmap trace coloured by the signed difference between reference and comparison data. This data
    /// is selected perpendicular to the provided axis and located at the given index.
    pub fn create_comparison_heatmap_traces(&self, axis: usize, index: usize) -> Vec<Box<HeatMap<f64, f64, f64>>> {
        // select what 'x' and 'y' on the heatmap are according to the axis value
        
        let (xaxis, yaxis) = axis_selector(self.reference_data.to_owned(), axis);
        let (xaxis, yaxis) = meshgrid(xaxis, yaxis);
        // select the data along given axis at index location
        let reference_data = data_selector(self.reference_data.to_owned(), axis, index);    
        let comparison_data = data_selector(self.comparison_data.to_owned(), axis, index); 
        let delta: Array2<f64> = reference_data.into_owned() - comparison_data.into_owned();
        let heatmap = HeatMap::new(flatten_2d(&xaxis).to_vec(),flatten_2d(&yaxis).to_vec(),flatten_2d(&delta).to_vec());
        let traces = vec![heatmap];
        return traces
    }

    // TODO create
    // FIXME doc
    pub fn create_comparison_heatmap_slice_traces(&self, axis: usize, start: usize, stop: usize, step: usize) 
    //-> Vec<Box<Surface<f64, f64, f64> 
    { 

    }
    // TODO comparison contour plot
    // FIXME doc
    pub fn create_comparison_contour_traces(&self)
     //-> Vec<Box<Contour<f64, f64>>> 
     {
        
    }
    // TODO comparison contour plot
    // FIXME doc
    pub fn create_comparison_surface_traces(&self) 
    //-> Vec<Box<SurfaceContours<f64, f64, f64>>> 
    {

    }
    // FIXME doc
    pub fn auto_axis_range(&self) {
        // TODO
    }

    /// Take created traces and plot them.
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

}