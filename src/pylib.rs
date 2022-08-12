use pyo3::prelude::*;
extern crate ndarray;
extern crate plotly;
use crate::converter::*;
use crate::grid::*;
use crate::particleselector::*;
use crate::types::*;
use crate::{print_debug, print_warning};
pub mod libgrid;
use crate::datamanager::{Manager, PData, TData};

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

    #[args(norm_on = false, axis = 0)]
    fn velocityfield<'py>(
        &mut self,
        _py: Python<'py>,
        grid: &PyGrid,
        norm_on: bool, //normalise the size of the vectors
        axis: usize,
    ) -> PyGrid {
        print_debug!("Starting Vectorfield function");
        let selector: &ParticleSelector =
            match self.selector.as_any().downcast_ref::<ParticleSelector>() {
                Some(b) => b,
                None => panic!("Can not convert PyGrid to Grid1D as "),
            };
        let grid = self.data.velocityfield(grid.grid.clone(), selector);

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

#[pyclass(name = "Converter")]
struct PyConverter {}

#[pymethods]
impl PyConverter {
    #[args(filter = "r\"(\\d+).vtk\"")]
    #[staticmethod]
    fn vtk(
        filenames: Vec<&str>,
        timestep: f64,
        outname: &str,
        filter: &str, // example r"vtk_(\d+).vtk"
    ) {
        vtk(filenames, timestep, outname, filter);
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
