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
use plotly::{Layout, Plot, Trace};
use pyo3::{exceptions::PyValueError, prelude::*};

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

    #[pyo3(signature = (axis, selection = "depth_average", index = None, scaling_mode = "none", scaling_args = None))]
    fn _quiver_plot(
        &mut self,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        scaling_mode: &str,
        scaling_args: Option<Vec<f64>>,
    ) -> PyResult<()> {
        let vector_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let quiver_plotter = if selection == "depth_average" {
            QuiverPlot::from_vector_grid_depth_averaged(vector_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                QuiverPlot::from_vector_grid_single_plane(vector_grid, axis, index)
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
        let len = quiver_plotter.x().len();
        let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(len);
        let quiver_traces = if scaling_mode == "none" {
            quiver_plotter.create_quiver_traces()
        } else if scaling_mode == "min" {
            if let Some(scaling_args) = scaling_args {
                quiver_plotter
                    .bound_min(scaling_args[0])
                    .create_quiver_traces()
            } else {
                return Err(PyValueError::new_err(
                    "A valid scaling argument is required.",
                ));
            }
        } else if scaling_mode == "max" {
            if let Some(scaling_args) = scaling_args {
                quiver_plotter
                    .bound_max(scaling_args[0])
                    .create_quiver_traces()
            } else {
                return Err(PyValueError::new_err(
                    "A valid scaling argument is required.",
                ));
            }
        } else if scaling_mode == "minmax" {
            if let Some(scaling_args) = scaling_args {
                let scaling_vector = scaling_args;
                if scaling_vector.len() < 2 {
                    return Err(PyValueError::new_err(
                        "Min-max scaling requires 2 arguments.",
                    ));
                }
                quiver_plotter
                    .bound_min_max(scaling_vector[0], scaling_vector[1])
                    .create_quiver_traces()
            } else {
                return Err(PyValueError::new_err(
                    "A valid scaling argument is required.",
                ));
            }
        } else if scaling_mode == "half_node" {
            let dx = quiver_plotter.x()[1] - quiver_plotter.x()[0];
            let dy = quiver_plotter.y()[1] - quiver_plotter.y()[0];
            quiver_plotter
                .bound_half_node(dx, dy)
                .create_quiver_traces()
        } else if scaling_mode == "full_node" {
            let dx = quiver_plotter.x()[1] - quiver_plotter.x()[0];
            let dy = quiver_plotter.y()[1] - quiver_plotter.y()[0];
            quiver_plotter
                .bound_full_node(dx, dy)
                .create_quiver_traces()
        } else {
            return Err(PyValueError::new_err("Invalid scaling mode provided, valid types are: 'none', 'min', 'max', 'minmax', 'half_node', 'full_node'."));
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

    #[pyo3(signature = (axis, selection = "depth_average", index = None, scaling_mode = "none", scaling_args = None))]
    fn _unit_vector_plot(
        &mut self,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        scaling_mode: &str,
        scaling_args: Option<Vec<f64>>,
    ) -> PyResult<()> {
        let vector_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .unwrap()
            .clone();
        let unit_vector_plotter = if selection == "depth_average" {
            UnitVectorPlot::from_vector_grid_depth_averaged(vector_grid, axis)
        } else if selection == "plane" {
            if let Some(index) = index {
                UnitVectorPlot::from_vector_grid_single_plane(vector_grid, axis, index)
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
        let len = unit_vector_plotter.x().len();
        let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(len);
        let unit_vector_traces = if scaling_mode == "none" {
            unit_vector_plotter.create_quiver_traces()
        } else if scaling_mode == "min" {
            if let Some(scaling_args) = scaling_args {
                unit_vector_plotter
                    .bound_min(scaling_args[0])
                    .create_quiver_traces()
            } else {
                return Err(PyValueError::new_err(
                    "A valid scaling argument is required.",
                ));
            }
        } else if scaling_mode == "max" {
            if let Some(scaling_args) = scaling_args {
                unit_vector_plotter
                    .bound_max(scaling_args[0])
                    .create_quiver_traces()
            } else {
                return Err(PyValueError::new_err(
                    "A valid scaling argument is required.",
                ));
            }
        } else if scaling_mode == "minmax" {
            if let Some(scaling_args) = scaling_args {
                let scaling_vector = scaling_args;
                if scaling_vector.len() < 2 {
                    return Err(PyValueError::new_err(
                        "Min-max scaling requires 2 arguments.",
                    ));
                }
                unit_vector_plotter
                    .bound_min_max(scaling_vector[0], scaling_vector[1])
                    .create_quiver_traces()
            } else {
                return Err(PyValueError::new_err(
                    "A valid scaling argument is required.",
                ));
            }
        } else if scaling_mode == "half_node" {
            let dx = unit_vector_plotter.x()[1] - unit_vector_plotter.x()[0];
            let dy = unit_vector_plotter.y()[1] - unit_vector_plotter.y()[0];
            unit_vector_plotter
                .bound_half_node(dx, dy)
                .create_quiver_traces()
        } else if scaling_mode == "full_node" {
            let dx = unit_vector_plotter.x()[1] - unit_vector_plotter.x()[0];
            let dy = unit_vector_plotter.y()[1] - unit_vector_plotter.y()[0];
            unit_vector_plotter
                .bound_full_node(dx, dy)
                .create_quiver_traces()
        } else {
            return Err(PyValueError::new_err("Invalid scaling mode provided, valid types are: 'none', 'min', 'max', 'minmax', 'half_node', 'full_node'."));
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
    #[pyo3(signature = (grid_type, axis, selection = "depth_average", index = None))]
    fn _scalar_map(
        &mut self,
        grid_type: &str,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let scalar_plotter = if grid_type == "vector_grid" {
            let grid = self
                .grid
                .as_any()
                .downcast_ref::<VectorGrid>()
                .unwrap()
                .clone();
            if selection == "depth_average" {
                ScalarMap::from_vector_grid_depth_averaged(grid, axis)
            } else if selection == "plane" {
                if let Some(index) = index {
                    ScalarMap::from_vector_grid_single_plane(grid, axis, index)
                } else {
                    return Err(PyValueError::new_err(
                        "A valid index is required to select an individual plane.",
                    ));
                }
            } else {
                return Err(PyValueError::new_err(
                    "Valid selection modes are 'depth_average' and 'plane' only.",
                ));
            }
        } else if grid_type == "grid" {
            let grid = self.grid.clone();
            if selection == "depth_average" {
                ScalarMap::from_grid_depth_averaged(grid, axis)
            } else if selection == "plane" {
                if let Some(index) = index {
                    ScalarMap::from_grid_single_plane(grid, axis, index)
                } else {
                    return Err(PyValueError::new_err(
                        "A valid index is required to select an individual plane.",
                    ));
                }
            } else {
                return Err(PyValueError::new_err(
                    "Valid selection modes are 'depth_average' and 'plane' only.",
                ));
            }
        } else {
            return Err(PyValueError::new_err("Valid grid types are 'vector_grid' and 'grid' only. This was set implicitly
                        by the Plotter2D class and you shouldn't be seeing this error. Please contact the developers."));
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
    #[pyo3(signature = (grid_type, axis, selection = "depth_average", index = None))]
    fn _scalar_contour(
        &mut self,
        grid_type: &str,
        axis: usize,
        selection: &str,
        index: Option<usize>,
    ) -> PyResult<()> {
        let scalar_plotter = if grid_type == "vector_grid" {
            let grid = self
                .grid
                .as_any()
                .downcast_ref::<VectorGrid>()
                .unwrap()
                .clone();
            if selection == "depth_average" {
                ScalarContour::from_vector_grid_depth_averaged(grid, axis)
            } else if selection == "plane" {
                if let Some(index) = index {
                    ScalarContour::from_vector_grid_single_plane(grid, axis, index)
                } else {
                    return Err(PyValueError::new_err(
                        "A valid index is required to select an individual plane.",
                    ));
                }
            } else {
                return Err(PyValueError::new_err(
                    "Valid selection modes are 'depth_average' and 'plane' only.",
                ));
            }
        } else if grid_type == "grid" {
            let grid = self.grid.clone();
            if selection == "depth_average" {
                ScalarContour::from_grid_depth_averaged(grid, axis)
            } else if selection == "plane" {
                if let Some(index) = index {
                    ScalarContour::from_grid_single_plane(grid, axis, index)
                } else {
                    return Err(PyValueError::new_err(
                        "A valid index is required to select an individual plane.",
                    ));
                }
            } else {
                return Err(PyValueError::new_err(
                    "Valid selection modes are 'depth_average' and 'plane' only.",
                ));
            }
        } else {
            return Err(PyValueError::new_err("Valid grid types are 'vector_grid' and 'grid' only. This was set implicitly
                        by the Plotter2D class and you shouldn't be seeing this error. Please contact the developers."));
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
