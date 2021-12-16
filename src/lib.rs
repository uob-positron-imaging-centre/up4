//! The `uppp` crate provides an tool for post processing particle based data.
//!
//! in `uppp` we provide different structs allowing the accessing, processing and visualisation
//! of quantities such as the velocity vectorfield, mean-squared-displacement or
//! dispersion.
//!
//! It allows you to handle different data types via the `DataManager` trait.
//! The currently implemented datatype consists of a [HDF5](https://github.com/aldanor/hdf5-rust)
//! based data structure similar as [H5Part](https://dav.lbl.gov/archive/Research/AcceleratorSAPP/)
//! with some modifications according to the type of data: Experimental or simulational.
//! Hence we implement two structs that implement the `DataManager` trait: `PData` and `TData`:
//!
//! - **`PData`**: Particle based saving of data, for experimental data with many timestep
//!    but few particles such as [PEPT](https://www.birmingham.ac.uk/research/activity/physics/particle-nuclear/positron-imaging-centre/positron-emission-particle-tracking-pept/pept-overview.aspx)
//! - **`TData`**: A timestep based saving of data, for simulational data from different engines such
//!    as [LIGGGHTS](https://www.cfdem.com/liggghtsr-open-source-discrete-element-method-particle-simulation-code)




use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};
extern crate ndarray;
extern crate plotly;
use plotly::common::Mode;
use plotly::{Plot, Scatter};

mod functions;
pub mod datamanager;
pub mod base;
pub mod utilities;
use datamanager::{GlobalStats,Manager,TData,PData};
use base::{Grid,Selector,ParticleSelector, PyGrid};
pub mod plotting;



#[pyclass(name="Data")]
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
        let selector=ParticleSelector::default();
        PyData { data: Box::new(pdata) ,selector:Box::new(selector) }
    }

    #[staticmethod]
    fn from_tdata(filename: &str) -> Self {
        let tdata = TData::new(filename);
        let selector=ParticleSelector::default();
        PyData { data: Box::new(tdata),selector:Box::new(selector) }
    }

    fn stats<'py>(
        &self,
        _py: Python<'py>,
    ) {
        self.data.stats();
    }



    #[args(norm_on=false, axis=0)]
    fn vectorfield<'py>(
        &mut self,
        _py: Python<'py>,
        grid: PyGrid,
        norm_on: bool,                       //normalise the size of the vectors
        axis: usize,
    ) -> (
        &'py PyArrayDyn<f64>,
        &'py PyArrayDyn<f64>,
        &'py PyArrayDyn<f64>,
        &'py PyArrayDyn<f64>,
    ) {
        print_debug!("Starting Vectorfield function");
        let selector: &ParticleSelector = match self.selector.as_any().downcast_ref::<ParticleSelector>(){
            Some(b) => b,
            None => panic!("Can not convert PyGrid to Grid1D as ")
        };
        let (vx, vy, sx, sy) = self.data.vectorfield(
            grid.to_grid2d(),
            selector,
            norm_on,
            axis
        );
        (
            vx.into_pyarray(_py).to_dyn(),
            vy.into_pyarray(_py).to_dyn(),
            sx.into_pyarray(_py).to_dyn(),
            sy.into_pyarray(_py).to_dyn(),
        )
    }//End vectorfield

    fn mean_velocity_showcase<'py>(
        &mut self,
        _py: Python<'py>,
    ) -> f64
     {
        print_debug!("Starting mean velocity calculation on dataset ");
        let selector: &ParticleSelector = match self.selector.as_any().downcast_ref::<ParticleSelector>(){
            Some(b) => b,
            None => panic!("Can not convert PyGrid to Grid1D as ")
        };
        let mean_velocity = self.data.mean_velocity_showcase(
            selector
        );
        // return
        mean_velocity
    }//End mean_velocity

    fn mean_velocity<'py>(
        &mut self,
        _py: Python<'py>,
    ) -> f64 {
        print_debug!("Starting mean velocity calculation on dataset");
        let selector: &ParticleSelector = match self.selector.as_any().downcast_ref::<ParticleSelector>(){
            Some(b) => b,
            None => panic!("Can not convert PyGrid to Grid1D as ")
        };
        let mean_velocity = self.data.mean_velocity(
            selector
        );
        // return
        mean_velocity
    }//End mean_velocity


}// ENd PyData


#[pyclass(name="Converter")]
struct PyConverter{}


#[pymethods]
impl PyConverter{
    #[args(filter="r\"(\\d+).vtk\"")]
    #[staticmethod]
    fn vtk(
        filenames: Vec<&str>,
        timestep: f64,
        outname: &str,
        filter: &str, // example r"vtk_(\d+).vtk"
    ){
        base::vtk(filenames,timestep,outname,filter);
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

