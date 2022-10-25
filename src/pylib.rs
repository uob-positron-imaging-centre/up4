//! Create Python bindings for crate.
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
extern crate ndarray;
extern crate plotly;
use crate::particleselector::*;

use crate::print_debug;
use numpy::{IntoPyArray, PyArray2};
pub mod libconv;
pub mod libgrid;
pub mod libplot;
use crate::datamanager::{Manager, PData, TData};
use libconv::*;
use libgrid::*;
use libplot::*;

/// Class that holds the particle data for processing, if you have simulation data, you will *probably*
/// want to use ``Data.from_pdata()`` to instantiate this class as this handles a large number of particles.
/// For experimental data, ``Data.from_tdata()`` is recommended. However, as the choice ultimately makes no
/// difference to how you use this library, that choice is down to you!
///
/// Methods
/// -------
/// from_pdata:
///     Constructor for up4.Data using data that has large numbers of particles, typically from simulation.
///
/// from_tdata:
///     Constructor for up4.Data using data that has low numbers of particles, typically from experiment.
///
/// stats:
///     Calculate statistics of the dataset such as dimensions, mean velocity and number of particles.
///
/// set_time:
///     Select the dataset between two times.
///
/// vectorfield:
///     Return vector data as a vector field.
///
/// velocityfield:
///     Return the velocity data as a velocity field.
///
/// extract:
///     Return particle information over specified duration.
///
/// numberfield:
///     Return the number density field.
///
/// mean_velocity:
///     Return the mean velocity of all valid particles in the system.
///
/// dispersion:
///    Return the dispersion of all valid particles in the system for a given time
#[pyclass(name = "Data")]
struct PyData {
    data: Box<dyn Manager + Send>,
    selector: Box<dyn Selector + Send>,
}

#[pymethods]
impl PyData {
    /// Create new i0nstance of up4.Data class. Time or particle oriented formats are parsed automatically.
    ///
    /// Parameters
    /// ----------
    /// filename : str
    ///     Filename of hdf5 dataset.
    ///
    /// Returns
    /// -------
    /// up4.Data
    ///     Data class.
    #[new]
    fn constructor(filename: &str) -> Self {
        let file = hdf5::File::open(filename).expect(&format!(
            "Unable to open file {}. Check if file exists.",
            filename
        ));
        let hdf5type: i32 = file
            .attr("hdf5_up4_type")
            .expect(&format!(
                "Can not find attribute \"hdf5_up4_type\" in file {}",
                filename
            ))
            .read_scalar()
            .expect(&format!(
                "Can not read scalar from attribute \"hdf5_up4_type\" in file {}",
                filename
            ));
        file.close().expect("Unable to close file");
        let data;
        if hdf5type == 0x1_i32 {
            data = PyData::from_tdata(filename)
        } else if hdf5type == 0x2_i32 {
            data = PyData::from_pdata(filename)
        } else {
            panic!("Unknown hdf5 type {}", hdf5type);
        }
        data
    }

    /// Create new instance of the up4.Data class. This method assumes a particle oriented hdf5 file.
    ///
    /// Parameters
    /// ----------
    /// filename : str
    ///     Filename of hdf5 dataset.
    ///
    /// Returns
    /// -------
    /// up4.Data
    ///     Data class
    #[staticmethod]
    fn from_pdata(filename: &str) -> Self {
        let pdata = PData::new(filename);
        let selector = ParticleSelector::default();
        PyData {
            data: Box::new(pdata),
            selector: Box::new(selector),
        }
    }

    /// Create new instance of the Data class. This method assumes a time oriented hdf5 file.
    ///
    /// Parameters
    /// ----------
    /// filename : str
    ///     Filename of hdf5 dataset.
    ///
    /// Returns
    /// -------
    /// up4.Data
    ///     Data class
    #[staticmethod]
    fn from_tdata(filename: &str) -> Self {
        let tdata = TData::new(filename);
        let selector = ParticleSelector::default();
        PyData {
            data: Box::new(tdata),
            selector: Box::new(selector),
        }
    }

    /// Calculate statistics of the dataset and print to terminal. Currently these are:
    /// * System dimensions.
    /// * Maximum time.
    /// * Number of particles.
    /// * Mean velocity.
    /// * Minimum and maximum velocity.
    fn stats<'py>(&self, _py: Python<'py>) {
        self.data.stats();
    }

    /// Return the dimensions of the system as a dictionary.
    /// The dictionary contains the following keys:
    /// * ``xmin``: Minimum x coordinate.
    /// * ``xmax``: Maximum x coordinate.
    /// * ``ymin``: Minimum y coordinate.
    /// * ``ymax``: Maximum y coordinate.
    /// * ``zmin``: Minimum z coordinate.
    /// * ``zmax``: Maximum z coordinate.
    fn dimensions<'py>(&self, py: Python<'py>) -> &'py pyo3::types::PyDict {
        let stats = self.data.global_stats();
        let dim = stats.dimensions();
        let key_vals: Vec<(&str, PyObject)> = vec![
            ("xmin", dim[[0, 0]].to_object(py)),
            ("xmax", dim[[0, 1]].to_object(py)),
            ("ymin", dim[[1, 0]].to_object(py)),
            ("ymax", dim[[1, 1]].to_object(py)),
            ("zmin", dim[[2, 0]].to_object(py)),
            ("zmax", dim[[2, 1]].to_object(py)),
        ];
        let dict = key_vals.into_py_dict(py);
        dict
    }

    /// Return the min position of the system as a array
    /// The array contains the following values at positions:
    /// * ``0``: Minimum x coordinate.
    /// * ``1``: Minimum y coordinate.
    /// * ``2``: Minimum z coordinate.
    fn min_position<'py>(&self, py: Python<'py>) -> &'py numpy::PyArray1<f64> {
        let stats = self.data.global_stats();
        let dim = stats.dimensions();
        let min_pos = dim.column(0).to_owned();
        min_pos.into_pyarray(py)
    }

    /// Return the max position of the system as a array
    /// The array contains the following values at positions:
    /// * ``0``: Maximum x coordinate.
    /// * ``1``: Maximum y coordinate.
    /// * ``2``: Maximum z coordinate.
    fn max_position<'py>(&self, py: Python<'py>) -> &'py numpy::PyArray1<f64> {
        let stats = self.data.global_stats();
        let dim = stats.dimensions();
        let max_pos = dim.column(1).to_owned();
        max_pos.into_pyarray(py)
    }

    /// Select the dataset between two different times.
    ///
    /// Parameters
    /// ----------
    /// min_time : float
    ///     Starting time for selection.
    /// max_time : float
    ///     End time for selection.
    fn set_time(&mut self, min_time: f64, max_time: f64) {
        if self.data.global_stats().max_time() < &min_time {
            panic!(
                "selected time range is out of range. Consider changing time range.\n\
            Max time of system: {}",
                self.data.global_stats().max_time()
            )
        }
        self.selector.set_time(min_time, max_time)
    }

    /// Return vector data as a vector field.
    ///
    /// Parameters
    /// ----------
    /// grid : up4.Grid
    ///     Grid class containing the grid layout.
    ///
    /// Returns
    /// -------
    /// up4.VectorGrid
    ///     VectorGrid class containing each vector component for each grid cell.
    fn vectorfield<'py>(&mut self, _py: Python<'py>, grid: &PyGrid) -> PyVecGrid {
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self.data.vectorfield(grid.grid.clone(), selector);
        PyVecGrid { grid }
    }

    /// Return the velocity data as a velocity field of their norms.
    ///
    /// Parameters
    /// ----------
    /// grid : up4.Grid
    ///      Grid class containing the grid layout.
    ///
    /// Returns
    /// -------
    /// up4.Grid
    ///     Grid class containing the velocity field.
    fn velocityfield<'py>(&mut self, _py: Python<'py>, grid: &PyGrid) -> PyGrid {
        print_debug!("Starting Vectorfield function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self.data.velocityfield(grid.grid.clone(), selector);

        PyGrid { grid: grid }
    }

    /// Return particle information over specified duration.
    ///
    /// Parameters
    /// ----------
    /// particle_id : int
    ///     Particle ID.
    /// timestep : tuple[int, int]
    ///     Start and end timesteps for particle selection.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     2D numpy array with shape (timestep[1] - timestep[0]) X 7. Where in each row is the current time, particle position and its velocity components.
    fn extract<'py>(
        &mut self,
        _py: Python<'py>,
        particle_id: usize,
        timestep: (usize, usize),
    ) -> &'py PyArray2<f64> {
        self.data.extract(particle_id, timestep).into_pyarray(_py)
    }

    /// Return the number density field.
    ///
    /// Parameters
    /// ----------
    /// grid : up4.Grid
    ///     Grid class containing the grid layout.
    ///
    /// Returns
    /// -------
    /// up4.Grid
    ///     Grid class containing the number field
    fn numberfield<'py>(&mut self, _py: Python<'py>, grid: &PyGrid) -> PyGrid {
        print_debug!("Starting Vectorfield function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self.data.numberfield(grid.grid.clone(), selector);

        PyGrid { grid: grid }
    }

    /// Return the mean velocity of all valid particles in the system.
    ///
    /// Returns
    /// -------
    /// float
    ///     Mean particle velocity
    fn mean_velocity<'py>(&mut self, _py: Python<'py>) -> f64 {
        print_debug!("Starting mean velocity calculation on dataset");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let mean_velocity = self.data.mean_velocity(selector);
        // return
        mean_velocity
    } //End mean_velocity

    /// Return the dispersion of the particles in the system.
    /// See Martin, T. W., J. P. K. Seville, and D. J. Parker. "A general method for quantifying dispersion in multiscale systems using trajectory analysis."
    ///
    /// parameters
    /// ----------
    /// grid : up4.Grid
    ///    Grid class containing the grid layout.
    /// time_for_dispersion : float
    ///   Time for which the dispersion is calculated.
    ///
    /// returns
    /// -------
    /// up4.Grid
    ///   Grid class containing the dispersion field.
    ///
    ///
    fn dispersion<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        time_for_dispersion: f64,
    ) -> PyGrid {
        print_debug!("Starting Dispersion function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self
            .data
            .dispersion(grid.grid.clone(), selector, time_for_dispersion);

        PyGrid { grid: grid }
    }
} // End PyData

#[pyproto]
impl pyo3::PyObjectProtocol for PyData {
    fn __str__(&self) -> PyResult<String> {
        let global_stats = self.data.global_stats();
        let time = global_stats.max_time();
        let dim = global_stats.dimensions();
        let part_num = global_stats.nparticles();
        let vel_mag = global_stats.velocity_mag();

        Ok(format!(
            "Dimensions of the system:\n\
            \t x {:.2}-->{:.2}\n\
            \t y {:.2}-->{:.2}\n\
            \t z {:.2}-->{:.2}\n\
            The max time of this set is : {:.2}\n\
            Number of Particles: {:.2}\n\
            Mean velocity of: {:.2} m/s\n\
            Minimum velocity {:.2} m/s \nMaximum Velocity {:.2} m/s\n",
            dim[[0, 0]],
            dim[[1, 0]],
            dim[[0, 1]],
            dim[[1, 1]],
            dim[[0, 2]],
            dim[[1, 2]],
            time,
            part_num,
            vel_mag[1usize],
            vel_mag[0usize],
            vel_mag[2usize]
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        let global_stats = self.data.global_stats();
        let time = global_stats.max_time();
        let dim = global_stats.dimensions();
        let part_num = global_stats.nparticles();
        let vel_mag = global_stats.velocity_mag();
        Ok(format!(
            "Dimensions of the system:\n\
            \t x {:.2}-->{:.2}\n\
            \t y {:.2}-->{:.2}\n\
            \t z {:.2}-->{:.2}\n\
            The max time of this set is : {:.2}\n\
            Number of Particles: {:.2}\n\
            Mean velocity of: {:.2} m/s\n\
            Minimum velocity {:.2} m/s \nMaximum Velocity {:.2} m/s\n",
            dim[[0, 0]],
            dim[[1, 0]],
            dim[[0, 1]],
            dim[[1, 1]],
            dim[[0, 2]],
            dim[[1, 2]],
            time,
            part_num,
            vel_mag[1usize],
            vel_mag[0usize],
            vel_mag[2usize]
        ))
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn upppp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyData>()?;
    m.add_class::<PyGrid>()?;
    m.add_class::<PyConverter>()?;
    m.add_class::<PyVectorPlotter>()?;
    m.add_class::<PyScalarPlotter>()?;
    m.add_class::<PyComparisonPlotter>()?;
    Ok(())
}
