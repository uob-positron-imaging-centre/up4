use numpy::{PyArrayDyn, IntoPyArray};
use pyo3::prelude::*;
use crate::{datamanager::{Manager, PData, TData}, base::{Selector, ParticleSelector, PyGrid}, print_debug};
///Data class
#[pyclass(name="Data")]
#[pyo3(text_signature = "(filename)")]
pub struct PyData {
    data: Box<dyn Manager + Send>,
    selector: Box<dyn Selector + Send>,
}
/// Class that holds the particle data for processing, if you have simulation data, you will *probably*
/// want to use ``from_pdata`` to instantiate this class as this handles a large number of particles. 
/// For experimental data, ``from_tdata`` is recommended. However, as the choice ultimately makes no 
/// difference to how you use this library, that choice is down to you!
/// 
/// Attributes
/// ----------
/// placeholder: 
///     idk lol
/// 
/// Methods
/// -------
/// from_pdata:
///     Constructor for Data using data that has large numbers of particles, typically from simulation. 
/// 
/// from_tdata:
///     Constructor for Data using data that has low numbers of particles, typically from experiment.
/// 
/// stats:
///     Calculate statistics
/// 
/// vectorfield: 
///     Convert the data into a vectorfield
/// 
/// mean_velocity_showcase: 
///     shows off mean velocity
/// 
/// mean_velocity:
///     Calculates mean velocities
#[pymethods]
impl PyData {
    #[new]
    fn constructor(filename: &str) -> Self {
        PyData::from_pdata(filename)
    }
    /// Constructor for ``Data`` that is structured to handle large numbers
    /// of particles. 
    /// 
    /// Parameters
    /// ----------
    /// filename: String
    ///     Path to the file to be analysed.
    #[staticmethod]
    fn from_pdata(filename: &str) -> Self {
        let pdata = PData::new(filename);
        let selector=ParticleSelector::default();
        PyData { data: Box::new(pdata) ,selector:Box::new(selector) }
    }
    /// Constructor for ``Data`` that is structured to handle small numbers
    /// of particles. 
    /// 
    /// Parameters
    /// ----------
    /// filename: String
    ///     Path to the file to be analysed.
    #[pyo3(name = "from_tdata", text_signature = "(filename, /)")]
    #[staticmethod]
    fn from_tdata(filename: &str) -> Self {
        let tdata = TData::new(filename);
        let selector=ParticleSelector::default();
        PyData { data: Box::new(tdata),selector:Box::new(selector) }
    }

    /// perform statistics
    /// 
    /// Parameters
    /// ----------
    /// um:
    ///     idk    
    fn stats<'py>(
        &self,
        _py: Python<'py>,
    ) {
        self.data.stats();
    }


    /// make a vector field
    /// 
    /// Parameters
    /// ----------
    /// um:
    ///     idk  
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

    /// mean velocity showcase
    /// 
    /// Parameters
    /// ----------
    /// um:
    ///     idk  
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

    /// mean velocity
    /// 
    /// Parameters
    /// ----------
    /// um:
    ///     idk  
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






