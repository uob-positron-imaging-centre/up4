use super::PyData;
use crate::grid;
use crate::grid::*;
use ndarray_stats::QuantileExt;
use numpy::{IntoPyArray, PyArray1, PyArray3};
use plotly::{HeatMap, Plot};
use pyo3::prelude::*;

/// A class containing all information for a 3D grid wrapping your system
///
/// Methods
/// -------
/// cartesian3d_from_data:
///     Create a 3D cartesian grid with limits and spacing from a PyData object
///
/// cartesian3d:
///     Create a 3D cartesian grid with limits and spacing provided by user
///
/// cylindrical3d_from_data:
///     Create a 3D cylindrical grid with limits and spacing from a PyData object
///
/// cylindrical3d:
///     Create a 3D cylindrical grid with limits and spacing provided by user
///
/// cell_positions:
///     Return the positions of all cells in the grid
///
/// is_inside:
///     Return a boolean array indicating if a point is inside the grid
///
/// plot:
///     Simple plotter using plotly. Data is depth averaged along the axis provided by user
///
/// shape:
///     Return the shape of the grid
///
/// to_numpy:
///     Return the grid as a numpy array
///
///
#[pyclass(name = "Grid")]
pub struct PyGrid {
    pub grid: Box<dyn grid::GridFunctions3D>,
}

#[pymethods]
impl PyGrid {
    /// Create a 3D cartesian grid with limits and spacing from a PyData object
    ///
    /// Parameters
    /// ----------
    /// data : PyData
    ///     A PyData object containing the data to be used to generate the grid
    /// cells : List(int)
    ///     A list containing the number of cells in each direction. Must be of length 3
    ///
    /// Returns
    /// -------
    /// grid : Grid
    ///     A up4.Grid object with the same dimensions as the input data
    ///
    #[staticmethod]
    fn cartesian3d_from_data(pydata: &PyData, cells: Vec<usize>) -> Self {
        let stats = pydata.data.global_stats();
        let dim = stats.dimensions();
        let grid = CartesianGrid3D::new(
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

    /// Create a 3D cartesian grid with limits and spacing provided by user
    ///
    /// Parameters
    /// ----------
    /// cells : List(int)
    ///     A list containing the number of cells in each direction. Must be of length 3
    /// limit : List(float)
    ///     A list containing the limits of the grid in each direction. Must be of length 6
    ///     The order is [xmin, xmax, ymin, ymax, zmin, zmax]
    ///
    /// Returns
    /// -------
    /// grid : Grid
    ///    A up4.Grid object with the same dimensions dimensions defined by user
    #[staticmethod]
    fn cartesian3d(cells: Vec<usize>, limit: Vec<f64>) -> Self {
        if cells.len() != 3 {
            panic!("Cylindrical3D requires cells for 3 dimensions --> shape mismatch");
        }
        if limit.len() != 6 {
            panic!("Cylindrical3D requires 2 limits for all 3 Dimensions --> shape mismatch ");
        }
        let grid = Box::new(grid::CartesianGrid3D::new(
            [cells[0], cells[1], cells[2]],
            grid::Dim::ThreeD([
                [limit[0], limit[1]],
                [limit[2], limit[3]],
                [limit[4], limit[5]],
            ]),
        ));
        let grid = PyGrid {
            grid: grid, //Box::new(grid),
        };
        grid
    }

    /// Return the positions of all cells in the grid
    /// Parameters
    ///
    /// Returns
    /// -------
    /// positions : Touple( ndarray )
    ///    A touple containing the 1D arrays of the x, y and z positions of the cells
    fn cell_positions<'py>(
        &self,
        _py: Python<'py>,
    ) -> (&'py PyArray1<f64>, &'py PyArray1<f64>, &'py PyArray1<f64>) {
        (
            self.grid.get_xpositions().to_owned().into_pyarray(_py),
            self.grid.get_ypositions().to_owned().into_pyarray(_py),
            self.grid.get_zpositions().to_owned().into_pyarray(_py),
        )
    }

    /// Create a 3D cylindrical grid with limits provided by user
    ///
    /// Parameters
    /// ----------
    /// cells : List(int)
    ///     A list containing the number of cells in each direction. Must be of length 3
    /// limit : List(float)
    ///     A list containing the limits of the grid in each direction. Must be of length 6
    ///     The order is [rmin, rmax, zmin, zmax, phimin, phimax]
    /// mode: str, optional
    ///     The mode of the grid. Can be "volume". Other methods are not implemented yet.
    ///     Default is "volume" (what a surprise)
    ///
    /// Returns
    /// -------
    /// grid : Grid
    ///     A up4.Grid object with the same dimensions dimensions defined by user
    #[args(mode = "\"volume\"")]
    #[staticmethod]
    fn cylindrical3d(cells: Vec<usize>, limit: Vec<f64>, mode: &str) -> Self {
        if cells.len() != 3 {
            panic!("Cylindrical3D requires cells for 3 dimensions --> shape mismatch");
        }
        if limit.len() != 6 {
            panic!("Cylindrical3D requires 2 limits for all 3 Dimensions --> shape mismatch ");
        }
        let grid = Box::new(grid::CylindricalGrid3D::new(
            [cells[0], cells[1], cells[2]],
            grid::Dim::ThreeD([
                [limit[0], limit[1]],
                [limit[2], limit[3]],
                [limit[4], limit[5]],
            ]),
            mode,
        ));
        let grid = PyGrid {
            grid: grid, //Box::new(grid),
        };
        grid
    }

    /// Create a 3D cylindrical grid with limits and spacing from a PyData object
    ///
    /// Parameters
    /// ----------
    /// data : PyData
    ///     A PyData object containing the data to be used to generate the grid
    /// cells : List(int)
    ///     A list containing the number of cells in each direction. Must be of length 3
    /// mode: str, optional
    ///     The mode of the grid. Can be "volume". Other methods are not implemented yet.
    ///     Default is "volume" (what a surprise)
    ///
    /// Returns
    /// -------
    /// grid : Grid
    ///     A up4.Grid object with the same dimensions as the input data
    #[args(mode = "\"volume\"")]
    #[staticmethod]
    fn cylindrical3d_from_data(pydata: &PyData, cells: Vec<usize>, mode: &str) -> Self {
        let stats = pydata.data.global_stats();
        let dim = stats.dimensions();
        let grid = CylindricalGrid3D::new(
            [cells[0], cells[1], cells[2]],
            grid::Dim::ThreeD([
                [dim[[0, 0]], dim[[1, 0]]],
                [dim[[0, 1]], dim[[1, 1]]],
                [dim[[0, 2]], dim[[1, 2]]],
            ]),
            mode,
        );
        PyGrid {
            grid: Box::new(grid),
        }
    }

    /// Check if the particle is within the grid
    ///
    /// Parameters
    /// ----------
    /// position : List(float)
    ///    A list containing the position of the particle. Must be of length 3
    ///
    /// Returns
    /// -------
    /// is_inside : bool
    ///   True if the particle is inside the grid, False otherwise
    fn is_inside(&self, position: Vec<f64>) -> bool {
        self.grid.is_inside([position[0], position[1], position[2]])
    }

    /// Plot the grid using plotly plotter. A simple depth averaged plot.
    ///
    /// Parameters
    /// ----------
    /// axis : int, optional
    ///     The axis to plot the grid on
    ///     Default is 0
    ///
    /// Returns
    /// -------
    /// None
    ///
    fn plot(&self, axis: usize) {
        let mut plot = Plot::new();
        let vec2d = self
            .grid
            .collapse(axis)
            .outer_iter()
            .map(|arr| arr.to_vec())
            .collect::<Vec<_>>();
        //let lims = self.grid.get_limits();
        //let cells = self.grid.get_cells();
        //let dx = (lims[0][0]-lims[0][0]);
        //let x = (0..cells[0]).map(f)
        let x;
        let y;
        let trace;
        if axis == 0 {
            y = self.grid.get_ypositions().to_owned().to_vec();
            x = self.grid.get_zpositions().to_owned().to_vec();
            trace = HeatMap::new(x, y, vec2d);
        } else if axis == 1 {
            y = self.grid.get_xpositions().to_owned().to_vec();
            x = self.grid.get_zpositions().to_owned().to_vec();
            trace = HeatMap::new(x, y, vec2d);
        } else {
            y = self.grid.get_ypositions().to_owned().to_vec();
            x = self.grid.get_xpositions().to_owned().to_vec();
            trace = HeatMap::new(y, x, vec2d);
        }
        plot.add_trace(trace);
        plot.show()
    }

    /// Return the shape of the grid
    ///
    /// Parameters
    /// ----------
    /// None
    ///
    /// Returns
    /// -------
    /// shape : List(int)
    ///     A list containing the number of cells in each dimension.
    fn shape(&self) -> Vec<usize> {
        self.grid.get_cells().to_vec()
    }

    /// Return the grid as a numpy array
    ///
    /// Parameters
    /// ----------
    /// None
    ///
    /// Returns
    /// -------
    /// grid : ndarray
    ///    A numpy array containing the grid-data with the same shape as grid
    fn to_numpy<'py>(&self, _py: Python<'py>) -> &'py PyArray3<f64> {
        self.grid.get_data().to_owned().into_pyarray(_py)
    }
}

#[pyproto]
impl pyo3::PyObjectProtocol for PyGrid {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Grid3D: \n\tCells: {:?} \n\txlim: {:?} \
            \n\tylim: {:?} \n\tzlim: {:?}\n\tData information:\n\t\tMean: {:?}\
            \n\t\tStd: {:?}\n\t\tMin: {:?}\n\t\tMax: {:?}",
            self.grid.get_cells(),
            self.grid.get_limits()[0],
            self.grid.get_limits()[1],
            self.grid.get_limits()[2],
            self.grid
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
            self.grid.get_data().std(1.),
            self.grid.get_data().min_skipnan(),
            self.grid.get_data().max_skipnan()
        ))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "3D Mesh containing data with the shape: \n\
            \tx: {}\n\
            \ty: {}\n\
            \tz: {}\n\
            Data: \n\
            ---------------------------------------------------------
            {:?}",
            self.grid.get_cells()[0],
            self.grid.get_cells()[1],
            self.grid.get_cells()[2],
            self.grid.get_data()
        ))
    }
}

#[pyclass(name = "VectorGrid")]
pub struct PyVecGrid {
    pub grid: vector_grid::VectorGrid,
}

#[pyproto]
impl pyo3::PyObjectProtocol for PyVecGrid {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "3D Vector Mesh containing data with the shape: \n\
            \tx: {}\n\
            \ty: {}\n\
            \tz: {}\
",
            self.grid.get_cells()[0],
            self.grid.get_cells()[1],
            self.grid.get_cells()[2]
        ))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "3D Vector Mesh containing data with the shape: \n\
            \tx: {}\n\
            \ty: {}\n\
            \tz: {}\
",
            self.grid.get_cells()[0],
            self.grid.get_cells()[1],
            self.grid.get_cells()[2]
        ))
    }
}

#[pymethods]
impl PyVecGrid {
    fn to_numpy<'py>(
        &self,
        _py: Python<'py>,
    ) -> (&'py PyArray3<f64>, &'py PyArray3<f64>, &'py PyArray3<f64>) {
        (
            self.grid.data[0].get_data().to_owned().into_pyarray(_py),
            self.grid.data[1].get_data().to_owned().into_pyarray(_py),
            self.grid.data[2].get_data().to_owned().into_pyarray(_py),
        )
    }
    fn cell_positions<'py>(
        &self,
        _py: Python<'py>,
    ) -> (&'py PyArray1<f64>, &'py PyArray1<f64>, &'py PyArray1<f64>) {
        (
            self.grid.data[0]
                .get_xpositions()
                .to_owned()
                .into_pyarray(_py),
            self.grid.data[0]
                .get_ypositions()
                .to_owned()
                .into_pyarray(_py),
            self.grid.data[0]
                .get_zpositions()
                .to_owned()
                .into_pyarray(_py),
        )
    }
    fn shape(&self) -> Vec<usize> {
        self.grid.data[0].get_cells().to_vec()
    }
}
