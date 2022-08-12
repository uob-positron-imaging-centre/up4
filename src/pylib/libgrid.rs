use super::PyData;
use crate::grid;
use crate::grid::*;
use pyo3::prelude::*;

#[pyclass(name = "Grid")]
pub struct PyGrid {
    pub grid: Box<dyn grid::GridFunctions3D>,
}

#[pymethods]
impl PyGrid {
    #[staticmethod]
    fn cartesian3d(filename: &str) -> Self {
        let grid = Box::new(grid::KartesianGrid3D::new(
            [1, 60, 60],
            grid::Dim::ThreeD([[0., 1.], [0., 1.], [0., 1.]]),
        ));
        let grid = PyGrid {
            grid: grid, //Box::new(grid),
        };
        grid
    }

    #[staticmethod]
    fn databound_cartesian3d(pydata: &PyData, cells: Vec<usize>) -> Self {
        let stats = pydata.data.global_stats();
        let dim = stats.dimensions();
        let grid = KartesianGrid3D::new(
            [cells[0], cells[1], cells[2]],
            grid::Dim::ThreeD([
                [dim[[0, 0]], dim[[1, 0]]],
                [dim[[0, 1]], dim[[1, 1]]],
                [dim[[0, 2]], dim[[1, 2]]],
            ]),
        );
        PyGrid {
            grid: Box::new(grid),
        }
    }

    // return shape of the data grid
    fn shape(&self) -> Vec<usize> {
        self.grid.get_cells().to_vec()
    }
}
// Rust methods not available in python
impl PyGrid {
    fn from_grid(grid: Box<dyn grid::GridFunctions3D>) -> Self {
        PyGrid { grid }
    }
}
