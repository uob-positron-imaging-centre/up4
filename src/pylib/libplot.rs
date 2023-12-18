//! Submodule for export of plotting code to Python.
// TODO break this up into multiple files
use crate::{
    libgrid::{PyGrid, PyVecGrid},
    parity_contour::ParityContour,
    parity_map::ParityMap,
    parity_plot::ParityPlot,
    plotting::*,
    quiver::QuiverPlot,
    scalar_contour::ScalarContour,
    scalar_map::ScalarMap,
    unit_vector::UnitVectorPlot,
    GridFunctions3D, VectorGrid,
};

use colorous::Gradient;
use plotly::{Layout, Plot, Trace};
use pyo3::{prelude::*, exceptions::PyValueError};

#[pyclass(name = "RustPlotter2D", subclass)]
pub struct PyPlotter2D {
    plotting_string: String,
    grid: Box<dyn GridFunctions3D>,
}

#[pymethods]
impl PyPlotter2D {
    #[staticmethod]
    fn _from_vector_grid(vector_grid: &PyVecGrid) -> PyPlotter2D {
        let grid: Box<VectorGrid> = Box::new(vector_grid.grid.to_owned());
        let plotting_string = String::new();

        PyPlotter2D {
            plotting_string,
            grid,
        }
    }

    #[staticmethod]
    fn _from_grid(grid: &PyGrid) -> PyPlotter2D {
        let grid = grid.grid.to_owned();
        let plotting_string = String::new();

        PyPlotter2D {
            plotting_string,
            grid,
        }
    }

    //TODO add scaleratio to the signature of this method
    // TODO default layout should remove the x and y axis lines
    #[pyo3(signature = (axis, selection = "depth_average", index = None, scaling_mode = None, scaling_args = None, colour_map = "viridis"))]
    fn _quiver_plot(
        &mut self,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        scaling_mode: Option<&str>,
        scaling_args: Option<Vec<f64>>,
        colour_map: Option<&str>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let vector_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let quiver_plotter = if selection == "depth_average" {
            QuiverPlot::from_vector_grid_depth_averaged(vector_grid, axis)
        } else {
            QuiverPlot::from_vector_grid_single_plane(vector_grid, axis, index.unwrap())
        };
        let len = quiver_plotter.norm().len();
        let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(len);
        let cmap = match colour_map {
            Some(colour_map) => Some(self.get_gradient(colour_map)),
            None => None,
        };
        let quiver_traces = match scaling_mode {
            Some("min") => quiver_plotter
                .bound_min(scaling_args.unwrap()[0])
                .create_quiver_traces(1.0, cmap),
            Some("max") => quiver_plotter
                .bound_max(scaling_args.unwrap()[0])
                .create_quiver_traces(1.0, cmap),
            Some("minmax") => quiver_plotter
                // FIXME sort unpacking an optional vector because i forget how
                .bound_min_max(scaling_args[0], scaling_args.unwrap()[1])
                .create_quiver_traces(1.0, cmap),
            Some("half_node") => {
                let dx = quiver_plotter.x()[1] - quiver_plotter.x()[0];
                let dy = quiver_plotter.y()[1] - quiver_plotter.y()[0];
                quiver_plotter
                    .bound_half_node(dx, dy)
                    .create_quiver_traces(1.0, cmap)
            }
            Some("full_node") => {
                let dx = quiver_plotter.x()[1] - quiver_plotter.x()[0];
                let dy = quiver_plotter.y()[1] - quiver_plotter.y()[0];
                quiver_plotter
                    .bound_full_node(dx, dy)
                    .create_quiver_traces(1.0, cmap)
            }
            None => quiver_plotter.create_quiver_traces(1.0, cmap),
            _ => quiver_plotter.create_quiver_traces(1.0, cmap),
        };

        for trace in quiver_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (axis, selection = "depth_average", index = None, scaling_mode = "none", scaling_args = None, colour_map = "viridis"))]
    fn _unit_vector_plot(
        &mut self,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        scaling_mode: Option<&str>,
        scaling_args: Option<Vec<f64>>,
        colour_map: Option<&str>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let vector_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let unit_vector_plotter = if selection == "depth_average" {
            UnitVectorPlot::from_vector_grid_depth_averaged(vector_grid, axis)
        } else {
            UnitVectorPlot::from_vector_grid_single_plane(vector_grid, axis, index.unwrap())
        };
        let len = unit_vector_plotter.x().len();
        let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(len);
        let cmap = match colour_map {
            Some(colour_map) => Some(self.get_gradient(colour_map)),
            None => None,
        };
        let unit_vector_traces = match scaling_mode {
            Some("min") => unit_vector_plotter
                .bound_min(scaling_args.unwrap()[0])
                .create_quiver_traces(1.0, cmap),
            Some("max") => unit_vector_plotter
                .bound_max(scaling_args.unwrap()[0])
                .create_quiver_traces(1.0, cmap),
            Some("minmax") => unit_vector_plotter
                .bound_min_max(scaling_args.unwrap()[0], scaling_args.unwrap()[1])
                .create_quiver_traces(1.0, cmap),
            Some("half_node") => {
                let dx = unit_vector_plotter.x()[1] - unit_vector_plotter.x()[0];
                let dy = unit_vector_plotter.y()[1] - unit_vector_plotter.y()[0];
                unit_vector_plotter
                    .bound_half_node(dx, dy)
                    .create_quiver_traces(1.0, cmap)
            }
            Some("full_node") => {
                let dx = unit_vector_plotter.x()[1] - unit_vector_plotter.x()[0];
                let dy = unit_vector_plotter.y()[1] - unit_vector_plotter.y()[0];
                unit_vector_plotter
                    .bound_full_node(dx, dy)
                    .create_quiver_traces(1.0, cmap)
            }
            None => unit_vector_plotter.create_quiver_traces(1.0, cmap),
        };
        for trace in unit_vector_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    // BUG something isn't being done correctly as the square axes aren't behaving
    #[pyo3(signature = (grid_type, axis, selection = "depth_average", index = None, colour_map = "viridis"))]
    fn _scalar_map(
        &mut self,
        grid_type: &str,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        colour_map: Option<&str>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let scalar_plotter = if grid_type == "vector_grid" {
            let grid = self
                .grid
                .as_any()
                .downcast_ref::<VectorGrid>()
                .unwrap()
                .clone();
            if selection == "depth_average" {
                ScalarMap::from_vector_grid_depth_averaged(grid, axis)
            } else {
                ScalarMap::from_vector_grid_single_plane(grid, axis, index.unwrap())
            }
        } else {
            // type == "grid"
            let grid = self.grid.clone();
            if selection == "depth_average" {
                ScalarMap::from_grid_depth_averaged(grid, axis)
            } else {
                ScalarMap::from_grid_single_plane(grid, axis, index.unwrap())
            }
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let heatmap_traces = scalar_plotter.create_scalar_map();
        for trace in heatmap_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    // BUG something isn't being done correctly as the square axes aren't behaving
    #[pyo3(signature = (grid_type, axis, selection = "depth_average", index = None, colour_map = "viridis"))]
    fn _scalar_contour(
        &mut self,
        grid_type: &str,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        colour_map: Option<&str>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let scalar_plotter = if grid_type == "vector_grid" {
            let grid = self
                .grid
                .as_any()
                .downcast_ref::<VectorGrid>()
                .unwrap()
                .clone();
            if selection == "depth_average" {
                ScalarContour::from_vector_grid_depth_averaged(grid, axis)
            } else {
                ScalarContour::from_vector_grid_single_plane(grid, axis, index.unwrap())
            }
        } else {
            // type == "grid"
            let grid = self.grid.clone();
            if selection == "depth_average" {
                ScalarContour::from_grid_depth_averaged(grid, axis)
            } else {
                ScalarContour::from_grid_single_plane(grid, axis, index.unwrap())
            }
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let contour_traces = scalar_plotter.create_scalar_contour();
        for trace in contour_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (comparison_grid, axis, selection = "depth_average", index = None))]
    fn _parity_plot_from_vector_grid(
        &mut self,
        comparison_grid: &PyVecGrid,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let ref_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let comp_grid = comparison_grid
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let parity_plotter = if selection == "depth_average" {
            ParityPlot::from_vector_grids_depth_averaged(ref_grid, comp_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                ParityPlot::from_vector_grids_single_plane(ref_grid, comp_grid, axis, index)
            } else {
                return Err(PyValueError::new_err(
                    "A valid index is required to select an individual plane.",
                ));
            }
        } else {
            ParityPlot::from_vector_grids(ref_grid, comp_grid)
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let heatmap_traces = parity_plotter.create_parity_scatter();
        for trace in heatmap_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (comparison_grid, axis, selection = "depth_average", index = None))]
    fn _parity_plot_from_grid(
        &mut self,
        comparison_grid: &PyGrid,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let ref_grid = self.grid.clone();
        let comp_grid = comparison_grid.grid.clone();
        let parity_plotter = if selection == "depth_average" {
            ParityPlot::from_grids_depth_averaged(ref_grid, comp_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                ParityPlot::from_grids_single_plane(ref_grid, comp_grid, axis, index)
            } else {
                return Err(PyValueError::new_err(
                    "A valid index is required to select an individual plane.",
                ));
            }
        } else {
            ParityPlot::from_grids(ref_grid, comp_grid)
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let heatmap_traces = parity_plotter.create_parity_scatter();
        for trace in heatmap_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (comparison_grid, axis, selection = "depth_average", index = None))]
    fn _parity_map_from_vector_grid(
        &mut self,
        comparison_grid: &PyVecGrid,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let ref_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let comp_grid = comparison_grid
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let parity_plotter = if selection == "depth_average" {
            ParityMap::from_vector_grids_depth_averaged(ref_grid, comp_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                ParityMap::from_vector_grids_single_plane(ref_grid, comp_grid, axis, index)
            } else {
                return Err(PyValueError::new_err(
                    "A valid index is required to select an individual plane.",
                ));
            }
        } else {
            return Err(PyValueError::new_err(
                "Valid selection modes are 'depth_average' and 'plane' only.",
            ));
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let parity_traces = parity_plotter.create_parity_map();
        for trace in parity_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (comparison_grid, axis, selection = "depth_average", index = None))]
    fn _parity_map_from_grid(
        &mut self,
        comparison_grid: &PyGrid,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let ref_grid = self.grid.clone();
        let comp_grid = comparison_grid.grid.clone();
        let parity_plotter = if selection == "depth_average" {
            ParityMap::from_grids_depth_averaged(ref_grid, comp_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                ParityMap::from_grids_single_plane(ref_grid, comp_grid, axis, index)
            } else {
                return Err(PyValueError::new_err(
                    "A valid index is required to select an individual plane.",
                ));
            }
        } else {
            return Err(PyValueError::new_err(
                "Valid selection modes are 'depth_average' and 'plane' only.",
            ));
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let parity_traces = parity_plotter.create_parity_map();
        for trace in parity_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (comparison_grid, axis, selection = "depth_average", index = None))]
    fn _parity_contour_from_vector_grid(
        &mut self,
        comparison_grid: &PyVecGrid,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let ref_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let comp_grid = comparison_grid
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let parity_plotter = if selection == "depth_average" {
            ParityContour::from_vector_grids_depth_averaged(ref_grid, comp_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                ParityContour::from_vector_grids_single_plane(ref_grid, comp_grid, axis, index)
            } else {
                return Err(PyValueError::new_err(
                    "A valid index is required to select an individual plane.",
                ));
            }
        } else {
            return Err(PyValueError::new_err(
                "Valid selection modes are 'depth_average' and 'plane' only.",
            ));
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let parity_traces = parity_plotter.create_parity_contour();
        for trace in parity_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (comparison_grid, axis, selection = "depth_average", index = None))]
    fn _parity_contour_from_grid(
        &mut self,
        comparison_grid: &PyGrid,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let ref_grid = self.grid.clone();
        let comp_grid = comparison_grid.grid.clone();
        let parity_plotter = if selection == "depth_average" {
            ParityContour::from_grids_depth_averaged(ref_grid, comp_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                ParityContour::from_grids_single_plane(ref_grid, comp_grid, axis, index)
            } else {
                return Err(PyValueError::new_err(
                    "A valid index is required to select an individual plane.",
                ));
            }
        } else {
            return Err(PyValueError::new_err(
                "Valid selection modes are 'depth_average' and 'plane' only.",
            ));
        };
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let heatmap_traces = parity_plotter.create_parity_contour();
        for trace in heatmap_traces {
            traces.push(trace)
        }
        let layout: Layout = Layout::new();
        let plot: Plot = plot(traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[getter]
    fn get_plotting_string(&self) -> PyResult<String> {
        Ok(self.plotting_string.to_owned())
    }
}

impl PyPlotter2D {
    fn get_gradient(&self, colour_map: &str) -> Gradient {
        // Once we get reflection, maybe this will look less bad??
        match colour_map {
            "turbo" => colorous::TURBO,
            "viridis" => colorous::VIRIDIS,
            "inferno" => colorous::INFERNO,
            "spectral" => colorous::SPECTRAL,
            "magma" => colorous::MAGMA,
            "plasma" => colorous::PLASMA,
            "cividis" => colorous::CIVIDIS,
            "warm" => colorous::WARM,
            "cool" => colorous::COOL,
            "cubehelix" => colorous::CUBEHELIX,
            "blue_green" => colorous::BLUE_GREEN,
            "blue_purple" => colorous::BLUE_PURPLE,
            "green_blue" => colorous::GREEN_BLUE,
            "orange_red" => colorous::ORANGE_RED,
            "purple_blue_green" => colorous::PURPLE_BLUE_GREEN,
            "purple_blue" => colorous::PURPLE_BLUE,
            "purple_red" => colorous::PURPLE_RED,
            "red_purple" => colorous::RED_PURPLE,
            "yellow_green_blue" => colorous::YELLOW_GREEN_BLUE,
            "yellow_green" => colorous::YELLOW_GREEN,
            "yellow_orange_brown" => colorous::YELLOW_ORANGE_BROWN,
            "yellow_orange_red" => colorous::YELLOW_ORANGE_RED,
            "blues" => colorous::BLUES,
            "greens" => colorous::GREENS,
            "greys" => colorous::GREYS,
            "oranges" => colorous::ORANGES,
            "purples" => colorous::PURPLES,
            "reds" => colorous::REDS,
            "brown_green" => colorous::BROWN_GREEN,
            "purple_green" => colorous::PURPLE_GREEN,
            "pink_green" => colorous::PINK_GREEN,
            "purple_orange" => colorous::PURPLE_ORANGE,
            "red_blue" => colorous::RED_BLUE,
            "red_grey" => colorous::RED_GREY,
            "red_yellow_blue" => colorous::RED_YELLOW_BLUE,
            "red_yellow_green" => colorous::RED_YELLOW_GREEN,
            "spectral" => colorous::SPECTRAL,
        }
    }
}
