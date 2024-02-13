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

use plotly::{contour::Contours, layout::themes::PLOTLY_WHITE, Layout, Plot, Trace};
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

    //TODO add scaleratio to the signature of this method
    // TODO default layout should remove the x and y axis lines
    #[pyo3(signature = (axis, selection = "depth_average", index = None, scaling_mode = None, min_size = None, max_size = None, colour_map = "viridis"))]
    fn _quiver_plot(
        &mut self,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        scaling_mode: Option<&str>,
        min_size: Option<f64>,
        max_size: Option<f64>,
        colour_map: Option<&str>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let vector_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .expect("This method should only be called from data that is a vector grid.")
            .clone();
        let quiver_plotter = if selection == "depth_average" {
            QuiverPlot::from_vector_grid_depth_averaged(vector_grid, axis)
        } else {
            QuiverPlot::from_vector_grid_single_plane(vector_grid, axis, index.unwrap())
        };
        let cmap = self.get_gradient(colour_map);
        let quiver_traces = match scaling_mode {
            Some("min") => quiver_plotter
                .bound_min(min_size.unwrap())
                .create_quiver_traces(1.0, cmap),
            Some("max") => quiver_plotter
                .bound_max(max_size.unwrap())
                .create_quiver_traces(1.0, cmap),
            Some("minmax") => quiver_plotter
                .bound_min_max(min_size.unwrap(), max_size.unwrap())
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
        // println!("quiver_traces: {:?}", quiver_traces.len());
        // for trace in quiver_traces {
        //     traces.push(trace);
        // }
        let template = &*PLOTLY_WHITE;
        let layout = Layout::new().template(template);
        let plot: Plot = plot(quiver_traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    #[pyo3(signature = (axis, selection = "depth_average", index = None, scaling_mode = "none", min_size = None, max_size = None, colour_map = "viridis"))]
    fn _unit_vector_plot(
        &mut self,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        scaling_mode: Option<&str>,
        min_size: Option<f64>,
        max_size: Option<f64>,
        colour_map: Option<&str>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let vector_grid = self
            .grid
            .as_any()
            .downcast_ref::<VectorGrid>()
            .expect("This method should only be called from data that is a vector grid.")
            .clone();
        let unit_vector_plotter = if selection == "depth_average" {
            UnitVectorPlot::from_vector_grid_depth_averaged(vector_grid, axis)
        } else {
            UnitVectorPlot::from_vector_grid_single_plane(vector_grid, axis, index.unwrap())
        };
        let cmap = self.get_gradient(colour_map);
        let unit_vector_traces = match scaling_mode {
            Some("min") => unit_vector_plotter
                .bound_min(min_size.unwrap())
                .create_quiver_traces(1.0, cmap),
            Some("max") => unit_vector_plotter
                .bound_max(max_size.unwrap())
                .create_quiver_traces(1.0, cmap),
            Some("minmax") => unit_vector_plotter
                .bound_min_max(min_size.unwrap(), max_size.unwrap())
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
            // None => unit_vector_plotter.create_quiver_traces(1.0, cmap),
            _ => unit_vector_plotter.create_quiver_traces(1.0, cmap),
        };

        let layout: Layout = Layout::new();
        let plot: Plot = plot(unit_vector_traces, layout);
        let plotting_string = plot.to_json();
        self.plotting_string = plotting_string;

        Ok(())
    }

    // BUG something isn't being done correctly as the square axes aren't behaving
    // TODO remove unwrap
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
        let cmap = self.get_gradient(colour_map);
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
        let heatmap_traces = scalar_plotter.create_scalar_map(cmap);
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
    // TODO do something with sizes because the default is SHITE for the contour plot (too big)
    // this may mean figuring out some optimal spacing
    #[pyo3(signature = (grid_type, axis, selection = "depth_average", index = None, colour_map = "viridis", n_contours = 10))]
    fn _scalar_contour(
        &mut self,
        grid_type: &str,
        axis: usize,
        selection: &str,
        index: Option<usize>,
        colour_map: Option<&str>,
        n_contours: Option<usize>,
    ) -> PyResult<()> {
        // Arguments are checked for validity at the Python layer, so we can relax
        // checks here.
        let cmap = self.get_gradient(colour_map);
        let scalar_plotter = if grid_type == "vector_grid" {
            let grid = self
                .grid
                .as_any()
                .downcast_ref::<VectorGrid>()
                .expect("This method should only be called from data that is a vector grid.")
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
        let contour_traces = scalar_plotter.create_scalar_contour(cmap);
        for trace in contour_traces {
            let min = scalar_plotter.min;
            let max = scalar_plotter.max;
            traces.push(
                trace
                    .contours(
                        Contours::new()
                            .start(min)
                            .end(max)
                            .size((max - min) / n_contours.unwrap() as f64),
                    )
                    .auto_contour(false),
            );
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
    fn get_gradient(&self, colour_map: Option<&str>) -> Option<Gradient> {
        // Once we get reflection, maybe this will look less bad??
        match colour_map {
            Some("turbo") => Some(colorous::TURBO),
            Some("viridis") => Some(colorous::VIRIDIS),
            Some("inferno") => Some(colorous::INFERNO),
            Some("spectral") => Some(colorous::SPECTRAL),
            Some("magma") => Some(colorous::MAGMA),
            Some("plasma") => Some(colorous::PLASMA),
            Some("cividis") => Some(colorous::CIVIDIS),
            Some("warm") => Some(colorous::WARM),
            Some("cool") => Some(colorous::COOL),
            Some("cubehelix") => Some(colorous::CUBEHELIX),
            Some("blue_green") => Some(colorous::BLUE_GREEN),
            Some("blue_purple") => Some(colorous::BLUE_PURPLE),
            Some("green_blue") => Some(colorous::GREEN_BLUE),
            Some("orange_red") => Some(colorous::ORANGE_RED),
            Some("purple_blue_green") => Some(colorous::PURPLE_BLUE_GREEN),
            Some("purple_blue") => Some(colorous::PURPLE_BLUE),
            Some("purple_red") => Some(colorous::PURPLE_RED),
            Some("red_purple") => Some(colorous::RED_PURPLE),
            Some("yellow_green_blue") => Some(colorous::YELLOW_GREEN_BLUE),
            Some("yellow_green") => Some(colorous::YELLOW_GREEN),
            Some("yellow_orange_brown") => Some(colorous::YELLOW_ORANGE_BROWN),
            Some("yellow_orange_red") => Some(colorous::YELLOW_ORANGE_RED),
            Some("blues") => Some(colorous::BLUES),
            Some("greens") => Some(colorous::GREENS),
            Some("greys") => Some(colorous::GREYS),
            Some("oranges") => Some(colorous::ORANGES),
            Some("purples") => Some(colorous::PURPLES),
            Some("reds") => Some(colorous::REDS),
            Some("brown_green") => Some(colorous::BROWN_GREEN),
            Some("purple_green") => Some(colorous::PURPLE_GREEN),
            Some("pink_green") => Some(colorous::PINK_GREEN),
            Some("purple_orange") => Some(colorous::PURPLE_ORANGE),
            Some("red_blue") => Some(colorous::RED_BLUE),
            Some("red_grey") => Some(colorous::RED_GREY),
            Some("red_yellow_blue") => Some(colorous::RED_YELLOW_BLUE),
            Some("red_yellow_green") => Some(colorous::RED_YELLOW_GREEN),
            _ => None,
        }
    }
}
