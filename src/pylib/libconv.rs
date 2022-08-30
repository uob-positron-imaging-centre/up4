//! This file provides coupling of functions to convert data to HDF5 file format
//! Functions are taken from base::converter.rs
//!
use crate::converter::*;
use pyo3::prelude::*;

#[pyclass(name = "Converter")]
pub struct PyConverter {}

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
    #[args(
        columns = "vec![0,1,2,3]",
        delimiter = "\",\"",
        header = "true",
        comment = "\"#\"",
        vel = "false",
        interpolate = "false",
        radius = "0.0"
    )]
    #[staticmethod]
    fn csv(
        filename: &str,
        outname: &str,
        columns: Vec<i64>,
        delimiter: &str,
        header: bool,
        comment: &str,
        vel: bool,
        interpolate: bool,
        radius: f64,
    ) {
        csv_converter(
            filename,
            outname,
            columns,
            delimiter,
            header,
            comment,
            vel,
            interpolate,
            radius,
        )
    }
}
