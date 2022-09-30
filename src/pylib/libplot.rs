

use itertools::izip;
use plotly::{Layout, Plot, layout::Axis, Trace};
// TODO export plotting class
use pyo3::prelude::*;
use crate::{vector_plot::VectorPlotter, libgrid::{PyVecGrid, PyGrid}, scalar_plot::ScalarPlotter, comparison_plot::ComparisonPlotter};
use plotly::heat_map::Smoothing;
#[pyclass(name="VectorPlotter")]
pub struct PyVectorPlotter {
    plotting_string: String,
    plotting_data: VectorPlotter,
}

#[pymethods]
impl PyVectorPlotter {

    // TODO remove debug test string when done
    #[new]
    fn constructor(vector_grid: &PyVecGrid) -> PyVectorPlotter {
        let plotter: VectorPlotter = VectorPlotter::new(vector_grid.grid.to_owned());
        return PyVectorPlotter { plotting_string: String::from("this is a test string"), plotting_data: plotter }
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }

    // TODO offer a slice variant
    fn unit_vector_plot(&mut self, axis: usize, index: usize)  {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        self.plotting_data.scale_global(0.0001);
        let arrows = self.plotting_data.create_unit_vector_traces(None, true, axis, index);
        let layout: Layout = Layout::new();
        let square: bool = true;
        let smoothing = Some(Smoothing::False);
        let xaxis: Option<Axis> = Some(Axis::new().title("x position".into()));
        let yaxis: Option<Axis> = Some(Axis::new().title("y position".into()));
        let axes = vec![xaxis, yaxis];
        let show = false;
        let (heatmap, layout) = self.plotting_data.create_unit_vector_background(layout, square, axes, smoothing, axis, index);
        for trace in arrows {
            traces.push(trace);
        }
        traces.push(heatmap);
        let plot: Plot = self.plotting_data.plot(traces, layout, show);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;
    }

    fn unit_vector_slice_plot(&mut self, axis: usize, range: [usize; 3]) {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let arrows = self.plotting_data.unit_vector_slice_traces(range, axis, None, false);
        let backgrounds = self.plotting_data.unit_vector_slice_background(range, axis);
        for (arrow, background) in izip!(arrows, backgrounds) {
            traces.push(arrow);
            //traces.push(background);
        }
        let layout: Layout = Layout::new();
        let show: bool = false;
        let plot: Plot = self.plotting_data.plot(traces, layout, show);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;
    }

    // TODO see if serde deserialise works for enum selection
    // TODO offer a sliced variant
    fn quiver_plot(&self)  {
        
    }

    fn cone_plot(&self)  {
        
    }

    fn volume_plot(&self)  {
       
    }

}

#[pyclass(name="ScalarPlotter")]
pub struct PyScalarPlotter {
    plotting_string: String,
    plotting_data: ScalarPlotter,
}

#[pymethods]
impl PyScalarPlotter {
    #[new]
    fn constructor(scalar_grid: &PyGrid) -> PyScalarPlotter {
        let plotter: ScalarPlotter = ScalarPlotter::new(scalar_grid.grid.to_owned());
        return PyScalarPlotter { 
            plotting_string: String::from("this is a test string"), 
            plotting_data: plotter 
        }
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }

    // TODO offer slices
    fn scalar_map_plot(&mut self, axis: usize, index: usize) {
        let mut trace = self.plotting_data.scalar_map_plot(axis, index);
        let traces: Vec<Box<dyn Trace>> = vec![trace.pop().unwrap()];
        let layout = Layout::new();
        let show = false;
        let plot: Plot = self.plotting_data.plot(traces, layout, show);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;
    }

    // TODO offer slices
    fn scalar_contour_plot(&mut self) {
        
    }

}

#[pyclass(name="ComparisonPlotter")]
pub struct PyComparisonPlotter {
    plotting_string: String,
    plotting_data: ComparisonPlotter,
}

#[pymethods]
impl PyComparisonPlotter {

    #[new]
    fn constructor(reference_grid: &PyGrid, comparison_grid: &PyGrid) -> PyComparisonPlotter {
        let plotter: ComparisonPlotter = ComparisonPlotter::new(reference_grid.grid.to_owned(), comparison_grid.grid.to_owned());
        return PyComparisonPlotter { plotting_string: String::from("this is a test string"), plotting_data: plotter }
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }

    // TODO offer slices
    fn parity_plot(&mut self) {
        let trace = self.plotting_data.create_parity_traces();
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        for trace in trace {
            traces.push(trace);
        }
        let layout = Layout::new();
        let show = false;
        let plot: Plot = self.plotting_data.plot(traces, layout, show);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;
    }

    // TODO offer slices
    fn comparison_map(&self) -> String {
        String::from("compmap")
    } 

    // TODO offer slices
    fn comparison_contour(&self) -> String {
        String::from("compcontour")
    } 

}