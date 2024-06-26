//! Create Python bindings for crate.

use ndarray::Array0;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
extern crate ndarray;
extern crate plotly;
use crate::particleselector::*;

use crate::print_debug;
use numpy::{IntoPyArray, PyArray2};
pub mod libcomp;
pub mod libconv;
pub mod libgrid;
pub mod libplot;
use crate::datamanager::{Manager, PData, TData};

use libconv::*;
use libgrid::*;
use libplot::*;


// pyO3 helper:
#[derive(FromPyObject)]
enum SliceIntOrVec<'a> {
    Slice(&'a pyo3::types::PySlice),
    Int(isize),
    Vec(Vec<isize>),
}

/// Class that holds the particle data for processing.
///
/// If you have simulation data, you will *probably*
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
///     Return the dispersion of all valid particles in the system for a given time
///
/// histogram:
///     Return the histogram of all valid particles in the system for a given time
///
/// granular_temperature:
///     Return the granular temperature of all valid particles in the system for a given time
///
/// lacey_mixing_index:
///     Return the lacey mixing index for the whole system over a given time
///
/// circulation_time:
///     Return the circulation time for the whole system, returns all times as one large array
///
/// circulation_time_two_boundary:
///     Return the circulation time for the whole system determined using two bounaries, returns all times as one large array
///
/// concentration_field:
///     Return the concentration field for the whole system
///
/// homogenity_index:
///     Return the homogenity index for the whole system, defiuned by two particle species
///
/// msd_field:
///     Return the mean square displacement field for the whole system
///
/// msd:
///     Return the mean square displacement for the whole system over time
///
#[pyclass(name = "Data")]
struct PyData {
    data: Box<dyn Manager + Send>,
    selector: Box<dyn Selector + Send>,
}

#[pymethods]
impl PyData {
    /// Create new instance of up4.Data class. Time or particle oriented formats are parsed automatically.
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
        let file = hdf5::File::open(filename)
            .unwrap_or_else(|_| panic!("Unable to open file {}. Check if file exists.", filename));
        let hdf5type: i32 = file
            .attr("hdf5_up4_type")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"hdf5_up4_type\" in file {}",
                    filename
                )
            })
            .read_scalar()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read scalar from attribute \"hdf5_up4_type\" in file {}",
                    filename
                )
            });
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
            ("xmax", dim[[1, 0]].to_object(py)),
            ("ymin", dim[[0, 1]].to_object(py)),
            ("ymax", dim[[1, 1]].to_object(py)),
            ("zmin", dim[[0, 2]].to_object(py)),
            ("zmax", dim[[1, 2]].to_object(py)),
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
        let min_pos = dim.row(0).to_owned();
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
        let max_pos = dim.row(1).to_owned();
        max_pos.into_pyarray(py)
    }

    /// Number of particles in the system.
    fn nparticles(&self) -> usize {
        *self.data.global_stats().nparticles()
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

    /// Return the time array of the dataset.
    ///
    /// Returns
    /// -------
    ///
    /// Array of time values.
    ///
    fn time<'py>(&self, py: Python<'py>) -> &'py numpy::PyArray1<f64> {
        self.data
            .global_stats()
            .time_array()
            .to_owned()
            .into_pyarray(py)
    }

    /// Return velocity data as a vector field.
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
    #[pyo3(signature = (grid, mode = "absolute", min_velocity = -1000000.0, max_velocity = 1000000.0))]
    fn velocityfield<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        mode: &str,
        min_velocity: f64,
        max_velocity: f64,
    ) -> PyGrid {
        print_debug!("Starting Vectorfield function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self.data.velocityfield(
            grid.grid.clone(),
            selector,
            mode,
            min_velocity,
            max_velocity,
        );

        PyGrid { grid }
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

        PyGrid { grid }
    }

    /// Return the occupancy field.
    ///
    /// Parameters
    /// ----------
    /// grid : up4.Grid
    ///     Grid class containing the grid layout.
    ///
    /// min_vel : float, optional
    ///    Minimum velocity to be considered as occupied.
    ///
    /// Returns
    /// -------
    /// up4.Grid
    ///     Grid class containing the number field
    #[pyo3(signature=(grid, min_vel = 0.0))]
    fn occupancyfield<'py>(&mut self, _py: Python<'py>, grid: &PyGrid, min_vel: f64) -> PyGrid {
        print_debug!("Starting Vectorfield function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self
            .data
            .occupancyfield(grid.grid.clone(), selector, min_vel);

        PyGrid { grid }
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

        // return
        self.data.mean_velocity(selector)
    } //End mean_velocity

    /// Return the dispersion of the particles in the system.
    ///
    /// See [1]_ for further details.
    ///
    /// Parameters
    /// ----------
    /// grid : up4.Grid
    ///    Grid class containing the grid layout.
    /// time_for_dispersion : float
    ///   Time for which the dispersion is calculated.
    ///
    /// Returns
    /// -------
    /// up4.Grid
    ///   Grid class containing the dispersion field.
    ///
    /// float
    ///   Mixing efficiency
    ///
    /// References
    /// ----------
    /// .. [1] Martin, T. W., J. P. K. Seville, and D. J. Parker. "A general method for quantifying dispersion in multiscale systems using trajectory analysis."
    fn dispersion<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        time_for_dispersion: f64,
    ) -> (PyGrid, f64) {
        print_debug!("Starting Dispersion function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let (grid, mixing_efficiency) =
            self.data
                .dispersion(grid.grid.clone(), selector, time_for_dispersion);

        (PyGrid { grid }, mixing_efficiency)
    }

    /// Calculate a histogram of a specific property in a region of the system.
    ///
    /// The histogram is calculated for all particles that are valid according to the particleselector.
    /// The histogram is calculated for the region defined by the grid.
    ///
    /// Parameters
    /// ----------
    ///
    /// grid : pygrid
    ///     The grid that defines the region of the system.
    ///
    /// property : str
    ///     The property that is used to calculate the histogram, the following properties are available:
    ///     - 'velocity'
    ///
    /// bins : int
    ///     The number of bins in the histogram.
    ///
    /// Returns
    /// -------
    /// histogram : numpy.ndarray
    ///     The histogram.
    ///
    /// bin_edges : numpy.ndarray
    ///     The bin edges.
    ///
    #[pyo3(signature = (grid, property = "velocity", bins = 100, limit = 0.0))]
    fn histogram<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        property: &str,
        bins: usize,
        limit: f64,
    ) -> (&'py numpy::PyArray1<f64>, &'py numpy::PyArray1<f64>) {
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let (histogram, bin_edges) =
            self.data
                .histogram(grid.grid.clone(), selector, property, limit, bins);
        (histogram.into_pyarray(_py), bin_edges.into_pyarray(_py))
    }

    /// Calculate the granular temperature of the system.
    /// The granular temperature is defined as the mean fluctuating velocity of the particles.
    ///
    /// Parameters
    /// ----------
    ///
    /// grid : PyGrid
    ///     The grid that defines the region of the system.
    ///
    /// mode : str
    ///    The mode in which the granular temperature is calculated.
    ///    The following modes are available:
    ///
    ///    - 'xyz'
    ///    - 'x'
    ///    - 'y'
    ///    - 'z'
    ///
    ///    , by default 'xyz'
    ///
    /// Returns
    /// -------
    ///
    /// PyGrid
    ///     The granular temperature of the system.
    #[pyo3(signature = (grid, mode = "xyz"))]
    fn granular_temperature<'py>(&mut self, _py: Python<'py>, grid: &PyGrid, mode: &str) -> PyGrid {
        print_debug!("Starting Granular Temperature function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self
            .data
            .granular_temperature_field(grid.grid.clone(), selector, mode);

        PyGrid { grid }
    }

    /// Calculate the Lacey mixing index for two particle types in a region of the system for a time period.
    ///
    /// Parameters
    /// ----------
    /// grid : PyGrid
    ///    The grid that defines the region of the system.
    ///
    /// type_a : int
    ///     The particle type of the first particle.
    ///
    /// type_b : int
    ///     The particle type of the second particle.
    ///
    /// Returns
    /// -------
    /// time : numpy.ndarray
    ///    The time of the mixing index.
    ///
    /// mixing_index : numpy.ndarray
    ///   The mixing index.
    #[pyo3(signature = (grid, type_a = 0, type_b = 1, threshold = 10))]
    fn lacey_mixing_index<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        type_a: usize,
        type_b: usize,
        threshold: usize,
    ) -> (&'py numpy::PyArray1<f64>, &'py numpy::PyArray1<f64>) {
        print_debug!("Starting Lacey Mixing Index function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let (time, mixing_index) =
            self.data
                .lacey_mixing(grid.grid.clone(), selector, type_a, type_b, threshold);

        (time.into_pyarray(_py), mixing_index.into_pyarray(_py))
    }

    /// Calculate the circulation time of a particle in a system.
    /// This algorithm works by setting a single boundary, which the particle has to cross 3 times.
    ///
    /// Parameters
    /// ----------
    /// position : float
    ///     The position of the boundary.
    ///
    /// axis : int
    ///     The axis along which the boundary is set. Default 0.
    ///
    /// Returns
    /// -------
    /// circulation_time : list
    ///     The circulation time of the particle.
    #[pyo3(signature = (position, axis = 0))]
    fn circulation_time<'py>(&mut self, _py: Python<'py>, position: f64, axis: usize) -> Vec<f64> {
        print_debug!("Starting Circulation Time function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };

        self.data.circulation_time(selector, axis, position)
    }

    /// Calculate the circulation time of a particle in a system.
    /// This algorithm works by setting two boundaries, which the particle has to cross 5 times in total.
    ///
    /// Parameters
    /// ----------
    /// position : tuple[float, float]
    ///     The positions of the boundaries.
    ///
    /// axis : int
    ///     The axis along which the boundary is set. Default 0.
    ///
    /// Returns
    /// -------
    /// list[tuple]
    ///     The start and end times for each circulation
    ///
    // TODO: also return the poarticle ID that did the circulation
    #[pyo3(signature = (position, axis = 0))]
    fn circulation_time_two_boundary<'py>(&mut self, _py: Python<'py>, position: (f64,f64), axis: usize) -> Vec<(f64,f64)> {
        print_debug!("Starting Circulation Time function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };

        self.data.circulation_time_two_boundary(selector, axis, position)
    }

    /// Calculate the concentration field of the system
    /// The concentration field is calculated by counting the number of particles of type a and b in a region of the system.
    /// The concentration is calculated by n_a / (n_a + n_b).
    ///
    /// Parameters
    /// ----------
    /// grid : PyGrid
    ///    The grid that defines the region of the system.
    ///
    /// type_a : int
    ///    The particle type of the first particle.
    ///
    /// type_b : int
    ///   The particle type of the second particle.
    ///
    /// Returns
    /// -------
    /// concentration_field : PyGrid
    ///   The concentration field of the system.
    #[pyo3(signature = (grid, type_a = 1, type_b = 2))]
    fn concentration_field<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        type_a: usize,
        type_b: usize,
    ) -> PyGrid {
        print_debug!("Starting Concentration Field function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self
            .data
            .concentration_field(grid.grid.clone(), selector, type_a, type_b);

        PyGrid { grid }
    }

    /// Calculate the homogenity index.
    ///
    /// Parameters
    /// ----------
    ///
    /// grid : PyGrid
    ///   The grid that defines the region of the system.
    #[pyo3(signature = (grid, min_vel = 0.0))]
    fn homogenity_index<'py>(&mut self, _py: Python<'py>, grid: &PyGrid, min_vel: f64) -> f64 {
        print_debug!("Starting Homogenity Index function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        self.data
            .homogenity_index(grid.grid.clone(), selector, min_vel)
    }

    /// Calculate the mean squared displacement field of the system.
    /// The mean squared displacement field is calculated by calculating the mean squared displacement of a particle in a region of the system.
    /// Each cell element represents the mean squared displacement of all particles in that cell.
    ///
    /// Parameters
    /// ----------
    /// grid : PyGrid
    ///   The grid that defines the region of the system.
    ///
    /// Returns
    /// -------
    /// msd_field : PyGrid
    ///  The mean squared displacement field of the system.
    #[pyo3(signature = (grid,time))]
    fn msd_field<'py>(&mut self, _py: Python<'py>, grid: &PyGrid, time: f64) -> PyGrid {
        print_debug!("Starting MSD Field function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self.data.msd_field(grid.grid.clone(), selector, time);

        PyGrid { grid }
    }

    /// Calculate the mean squared displacement of a particle in a system.
    ///
    /// Parameters
    /// ----------
    /// grid : PyGrid
    ///   The grid that defines the region of the system.
    ///
    /// min_time : float
    ///   The minimum time to start calculating the MSD.
    ///
    /// max_time : float
    ///   The maximum time to stop calculating the MSD.
    ///
    /// Returns
    /// -------
    /// msd : ndarray
    ///   The mean squared displacement of the particle.
    ///
    /// time : ndarray
    ///   The time at which the MSD was calculated.
    #[pyo3(signature = (grid, min_time = 0.0, max_time = 0.0, steps = 100))]
    fn msd<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        min_time: f64,
        max_time: f64,
        steps: usize,
    ) -> (&'py numpy::PyArray1<f64>, &'py numpy::PyArray1<f64>) {
        print_debug!("Starting MSD function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let (msd, time) = self
            .data
            .msd(grid.grid.clone(), selector, min_time, max_time, steps);

        (msd.into_pyarray(_py), time.into_pyarray(_py))
    }

    /// set the rotation of the system
    /// The rotation is set by rotating the system around a axis specified by the user.
    ///
    /// Parameters
    /// ----------
    /// anker : list
    ///     A point around which the system is rotated.
    ///
    /// angle : list
    ///     The angle of rotation defined in 3 rotations around the x, y and z axis.
    ///     The rotation is defined in radians.
    ///
    /// Returns
    /// -------
    /// None
    // #[pyo3(signature = (anker, angle))]
    // fn set_rotation(&mut self, anker: Vec<f64>, angle: Vec<f64>) {
    //     print_debug!("Starting set_rotation function");

    //     if anker.len() != 3 {
    //         panic!("The anker must be a list of 3 elements");
    //     }
    //     if angle.len() != 3 {
    //         panic!("The angle must be a list of 3 elements");
    //     }
    //     for i in 0..3 {
    //         self.data.set_rotation_angle(angle[i], i);
    //     }

    //     self.data.set_rotation_anker([anker[0], anker[1], anker[2]]);
    // }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.data.info().expect("Could not get info"))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.data.info().expect("Could not get info"))
    }

    fn __getitem__(&mut self, idx: SliceIntOrVec, py: Python) -> PyResult<PyObject> {
        match idx{
            SliceIntOrVec::Int(index) =>{
            let _idx;
            if index < 0 {
                _idx = *self.data.global_stats().timesteps() as isize + index;
            } else {
                _idx = index;
            }
            // Single index
            let timestep = self.data.get_timestep(_idx as usize);
            // make it an array
            let position = timestep.position().to_owned();
            let velocity = timestep.velocity().to_owned();
            // concatenate the arrays: position has 3 
            let mut result = ndarray::Array2::<f64>::zeros(
                    (position.len(), 7) 
                );
            let particle_id = timestep.particleid().to_owned();
            // concatenate the arrays: position has 3 
            for i in 0..position.len() {
                let id = particle_id[i];
                let p_x  = position[i][0];
                let p_y  = position[i][1];
                let p_z  = position[i][2];
                let v_x:f64  = velocity[[i,0]];
                let v_y:f64  = velocity[[i,1]];
                let v_z:f64  = velocity[[i,2]];
                let x  = ndarray::array![id as f64, p_x, p_y, p_z, v_x, v_y, v_z];
                result.row_mut(i).assign(&x);
            }

            Ok(result.into_pyarray(py).to_object(py))
        },
        SliceIntOrVec::Vec(indices) =>{
            // Fancy indexing with a list of indices
            // return multiple timesteps as a 3D array
            let mut result = ndarray::Array3::<f64>::zeros(
                [indices.len(), *self.data.global_stats().nparticles(), 7]
            );
            for (i, index) in indices.iter().enumerate() {
                let  _idx;
                if index < &0 {
                    _idx = *self.data.global_stats().timesteps() as isize + index;
                } else {
                    _idx = *index;
                }
                let timestep = self.data.get_timestep(_idx as usize);
                // make it an array
                let position = timestep.position().to_owned();
                let velocity = timestep.velocity().to_owned();
                let particle_id = timestep.particleid().to_owned();
                // concatenate the arrays: position has 3 
                for j in 0..position.len() {
                    let id = particle_id[j];
                    let p_x:f64  = position[j][0];
                    let p_y:f64  = position[j][1];
                    let p_z:f64  = position[j][2];
                    let v_x:f64  = velocity[[j,0]];
                    let v_y:f64  = velocity[[j,1]];
                    let v_z:f64  = velocity[[j,2]];
                    let x  = ndarray::array![id as f64, p_x, p_y, p_z, v_x, v_y, v_z];
                    result.slice_mut(ndarray::s![i, j, ..]).assign(&x);
                }
            }
            Ok(result.into_pyarray(py).to_object(py))
            // next: SLICE
        },
        SliceIntOrVec::Slice(slice) =>{
            let indices = slice.indices(*self.data.global_stats().timesteps() as i64).expect(
                "Could not get indices"
            );
            let start = indices.start;
            let stop = indices.stop;
            let step = indices.step;
            let mut result = ndarray::Array3::<f64>::zeros(
                [((stop - start) / step) as usize, *self.data.global_stats().nparticles(), 7]
            );
            for (i, index) in (start..stop).step_by(step as usize).enumerate() {
                let timestep = self.data.get_timestep(index as usize);
                // make it an array
                let position = timestep.position().to_owned();
                let velocity = timestep.velocity().to_owned();
                let particle_id = timestep.particleid().to_owned();
                // concatenate the arrays: position has 3 
                for j in 0..position.len() {
                    let id = particle_id[j];
                    let p_x:f64  = position[j][0];
                    let p_y:f64  = position[j][1];
                    let p_z:f64  = position[j][2];
                    let v_x:f64  = velocity[[j,0]];
                    let v_y:f64  = velocity[[j,1]];
                    let v_z:f64  = velocity[[j,2]];
                    let x  = ndarray::array![id as f64, p_x, p_y, p_z, v_x, v_y, v_z];
                    result.slice_mut(ndarray::s![i, j, ..]).assign(&x);
                }
            }
            Ok(result.into_pyarray(py).to_object(py))
        },
        _ => Err(pyo3::exceptions::PyIndexError::new_err("Index must be an integer or a list of integers"))
        }
    }
    
    fn __len__(&self) -> usize {
        self.data.global_stats().timesteps().to_owned()
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
    m.add_class::<PyPlotter2D>()?;
    m.add_class::<PyVecGrid>()?;
    Ok(())
}
