use pyo3::prelude::*;
extern crate ndarray;
extern crate plotly;
use crate::converter::*;
use crate::grid::*;
use crate::particleselector::*;
use crate::types::*;
use crate::{print_debug, print_warning};
pub mod libconv;
pub mod libgrid;
use crate::datamanager::{Manager, PData, TData};

use libconv::*;
use libgrid::*;
#[pyclass(name = "Data")]
struct PyData {
    data: Box<dyn Manager + Send>,
    selector: Box<dyn Selector + Send>,
}

#[pymethods]
impl PyData {
    #[new]
    fn constructor(filename: &str) -> Self {
        PyData::from_pdata(filename)
    }

    #[staticmethod]
    fn from_pdata(filename: &str) -> Self {
        let pdata = PData::new(filename);
        let selector = ParticleSelector::default();
        PyData {
            data: Box::new(pdata),
            selector: Box::new(selector),
        }
    }

    #[staticmethod]
    fn from_tdata(filename: &str) -> Self {
        let tdata = TData::new(filename);
        let selector = ParticleSelector::default();
        PyData {
            data: Box::new(tdata),
            selector: Box::new(selector),
        }
    }

    fn stats<'py>(&self, _py: Python<'py>) {
        self.data.stats();
    }

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

    fn mean_velocity_showcase<'py>(&mut self, _py: Python<'py>) -> f64 {
        print_debug!("Starting mean velocity calculation on dataset ");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let mean_velocity = self.data.mean_velocity_showcase(selector);
        // return
        mean_velocity
    } //End mean_velocity

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
} // ENd PyData

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
    Ok(())
}
