//! Submodule for export of plotting code to Python.

use crate::{
    comparison_plot::ComparisonPlotter,
    libgrid::{PyGrid, PyVecGrid},
    scalar_plot::ScalarPlotter,
    vector_plot::VectorPlotter,
};
use itertools::izip;
use plotly::heat_map::Smoothing;
use plotly::{layout::Axis, Layout, Plot, Trace};
use pyo3::prelude::*;

/// Class that handles plotting of vector data. This class produces 2D and 3D plots of vector data.
/// To enable transfer of plotting from Rust to Python, the Rust backend serialises plots into JSON strings
/// that are then parsed by up4.VectorPlotter.plot to return a plotly.graph_objects.Figure object.
///
/// Methods
/// -------
/// unit_vector_plot
///     Create unit vector plot JSON string.
#[pyclass(name = "VectorPlotter")]
pub struct PyVectorPlotter {
    plotting_string: String,
    plotting_data: VectorPlotter,
}

#[pymethods]
impl PyVectorPlotter {
    /// Create new instance of up4.plotting.VectorPlotter class.
    ///
    /// Returns
    /// -------
    /// up4.plotting.VectorPlotter
    ///     Vector plotting class.
    #[new]
    fn constructor(vector_grid: &PyVecGrid) -> PyVectorPlotter {
        let plotter: VectorPlotter = VectorPlotter::new(vector_grid.grid.to_owned());
        return PyVectorPlotter {
            plotting_string: String::new(),
            plotting_data: plotter,
        };
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }

    /// Create unit vector plot JSON string. The unit vector plot is perpendicular to the provided axis and located at the index value.
    ///
    /// Parameters
    /// ----------
    /// axis : int
    ///     Axis that the plane is perpendicular to.
    /// index : int
    ///     Index along supplied `axis` to select data from.
    fn unit_vector_plot(&mut self, axis: usize, index: usize) {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let arrows = self
            .plotting_data
            .create_unit_vector_traces(None, true, axis, index);
        let layout: Layout = Layout::new();
        let square: bool = false;
        let smoothing = Some(Smoothing::False);
        let xaxis: Option<Axis> = Some(Axis::new().title("x position".into()));
        let yaxis: Option<Axis> = Some(Axis::new().title("y position".into()));
        let axes = vec![xaxis, yaxis];
        let show = false;
        let (heatmap, layout) = self
            .plotting_data
            .create_unit_vector_background(layout, square, axes, smoothing, axis, index);
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
        let arrows = self
            .plotting_data
            .unit_vector_slice_traces(range, axis, None, false);
        let backgrounds = self.plotting_data.unit_vector_slice_background(range, axis);
        for (arrow, background) in izip!(arrows, backgrounds) {
            traces.push(arrow);
            traces.push(background);
        }
        let layout: Layout = Layout::new();
        let show: bool = false;
        let plot: Plot = self.plotting_data.plot(traces, layout, show);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;
    }

    // TODO see if serde deserialise works for enum selection
    // TODO offer a sliced variant
    fn quiver_plot(&self) {}

    fn cone_plot(&self) {}
}

/// Class that handles plotting of scalar data. This class produces 2D and 3D plots of scalar data.
/// To enable transfer of plotting from Rust to Python, the Rust backend serialises plots into JSON strings
/// that are then parsed by up4.ScalarPlotter.plot to return a plotly.graph_objects.Figure object.
///
/// Methods
/// -------
/// scalar_map_plot
///     Create heatmap plot JSON string.
#[pyclass(name = "ScalarPlotter")]
pub struct PyScalarPlotter {
    plotting_string: String,
    plotting_data: ScalarPlotter,
}

#[pymethods]
impl PyScalarPlotter {
    /// Create new instance of up4.plotting.ScalarPlotter class.
    ///
    /// Returns
    /// -------
    /// up4.plotting.ScalarPlotter
    ///     Scalar plotting class.
    #[new]
    fn constructor(scalar_grid: &PyGrid) -> PyScalarPlotter {
        let plotter: ScalarPlotter = ScalarPlotter::new(scalar_grid.grid.to_owned());
        return PyScalarPlotter {
            plotting_string: String::new(),
            plotting_data: plotter,
        };
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }

    /// Create heatmap plot JSON string. The heatmap plot is perpendicular to the provided axis and located at the index value.
    ///
    /// Parameters
    /// ----------
    /// axis : int
    ///     Axis that the plane is perpendicular to.
    /// index : int
    ///     Index along supplied `axis` to select data from.
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
    fn scalar_contour_plot(&mut self) {}

    fn volume_plot(&self) {}
}

/// Class that handles plotting of comparison data. This class produces 2D plots of reference ("the ground truth") data compared against another dataset ("comparison data").
/// To enable transfer of plotting from Rust to Python, the Rust backend serialises plots into JSON strings
/// that are then parsed by up4.ComparisonPlotter.plot to return a plotly.graph_objects.Figure object.
///
/// Methods
/// -------
/// parity_plot
///     Create parity plot JSON string.
#[pyclass(name = "ComparisonPlotter")]
pub struct PyComparisonPlotter {
    plotting_string: String,
    plotting_data: ComparisonPlotter,
}

#[pymethods]
impl PyComparisonPlotter {
    /// Create new instance of up4.plotting.ComparisonPlotter class.
    ///
    /// Returns
    /// -------
    /// up4.plotting.ComparisonPlotter
    ///     Comparison plotting class.
    #[new]
    fn constructor(reference_grid: &PyGrid, comparison_grid: &PyGrid) -> PyComparisonPlotter {
        let plotter: ComparisonPlotter = ComparisonPlotter::new(
            reference_grid.grid.to_owned(),
            comparison_grid.grid.to_owned(),
        );
        return PyComparisonPlotter {
            plotting_string: String::new(),
            plotting_data: plotter,
        };
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }

    /// Create parity plot JSON string. This plots the comparison dataset on the x axis and reference data on the y axis.
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
