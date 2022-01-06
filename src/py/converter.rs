use pyo3::prelude::*;
use crate::base;

#[pyclass(name="Converter")]
pub struct PyConverter{}


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