//! This file provides coupling of functions to convert data to HDF5 file format
//! Functions are taken from base::converter.rs
use crate::converter::{self, *};
use pyo3::prelude::*;

/// Convert particle data from a given format to HDF5 (H5Part-like).
///
/// Methods
/// -------
/// vtk:
///     Convert VTK file to a HDF5 file
///
/// vtk_from_folder:
///     Convert all VTK files in a folder to a HDF5 file
///
/// csv:
///     Convert CSV file to a HDF5 file
#[pyclass(name = "Converter")]
pub struct PyConverter {}

#[pymethods]
impl PyConverter {
    /// Convert VTK file to a HDF5 file.
    ///
    /// Parameters
    /// ----------
    /// filenames : List(str)
    ///     List of VTK files to convert, the list must be ordered by time and filenames must contain the timestep.
    ///
    /// timestep : float
    ///     Time between two timesteps.
    ///
    /// outname : str
    ///     Name of the output HDF5 file.
    ///
    /// filter : str, optional
    ///     Regex Filter to apply to the data in order to extract the time for each file, by default r"(\d+).vtk"
    ///
    #[pyo3(signature = (filenames, timestep, outname, filter = "(\\d+).vtk"))]
    #[staticmethod]
    fn vtk(
        filenames: Vec<&str>,
        timestep: f64,
        outname: &str,
        filter: &str, // example r"vtk_(\d+).vtk"
    ) {
        vtk(filenames, timestep, outname, filter);
    }

    /// Convert all VTK files in a folder to a HDF5 file.
    ///
    /// Parameters
    /// ----------
    /// folder : str
    ///     Path to the folder containing the VTK files
    ///     Folder must only contain one type of vtk files
    ///
    /// timestep : float
    ///     Time between two timesteps
    ///
    /// outname : str
    ///     Name of the output HDF5 file
    ///
    /// filter : str, optional
    ///     Regex Filter to apply to the data in order to extract the time for each file, by default r"(\d+).vtk"
    ///
    /// Returns
    /// -------
    /// None
    ///
    #[pyo3(signature = (folder, timestep, outname, filter = r"(\d+).vtk"))]
    #[staticmethod]
    fn vtk_from_folder(
        folder: &str,
        timestep: f64,
        outname: &str,
        filter: &str, // example r"vtk_(\d+).vtk"
    ) {
        vtk_from_folder(folder, timestep, outname, filter);
    }

    // TODO ensure this doesn't get accidentally swept up into doctests
    /// Convert CSV file to a HDF5 file.
    ///
    /// Parameters
    /// ----------
    /// filename : str
    ///    Path to the CSV file
    ///
    /// outname : str
    ///    Name of the output HDF5 file
    ///
    /// columns : List(int), optional
    ///     List of columns to convert containing t,x,y,z,(optional vx,vy,vz), by default [0,1,2,3]
    ///
    /// delimiter : str, optional
    ///     Pattern for separating numbers in a csv file, by default ','
    ///
    /// header : bool, optional
    ///     True if the CSV file contains a header, by default True
    ///
    /// comment : str, optional
    ///     Comment character to ignore, by default '#'
    ///
    /// vel : bool, optional
    ///     If true the velocity will be computed from the position using the savitzky-golay filter, by default False
    ///
    /// interpolate : bool, optional
    ///     If true the particle positions will be interpolated in order to have a constant timestep, by default False
    ///
    /// radius : float, optional
    ///     Particle radius, by default 0.0
    ///
    /// sampling_steps : int, optional
    ///    Number of sampling steps for the savitzky-golay filter to calculate the velocity, by default 9
    ///
    /// Returns
    /// -------
    /// None
    ///
    #[pyo3(signature = (
        filename,
        outname,
        columns = vec![0,1,2,3],
        delimiter = ",",
        header = true,
        comment = "#",
        vel = false,
        interpolate = false,
        radius = 0.0,
        sampling_steps = 9
    ))]
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
        sampling_steps: usize,
    ) {
        // check if sampling_steps is odd
        if sampling_steps % 2 == 0 {
            panic!("Sampling steps must be a odd number!");
        }
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
            sampling_steps,
        )
    }

    // TODO ensure this doesn't get accidentally swept up into doctests
    /// Convert CSV file containing multiple particles into a HDF5 file.
    ///
    /// There can be different ways how this is achieved, therefore the function
    /// takes an argument called `method` which can be one of the following:
    ///     
    /// - `chain`:   The particles are chained in the file, i.e. the first particle
    ///     is followed by the second, the second by the third, etc.
    ///     all particles are stored in one file
    /// - `id_line`: This algorithm sorts the particles by their id column and
    ///     their time column. The `columns` argument must contain the
    ///     id column as the first element.
    ///
    /// no other method is implemented yet. If you want to use another method, please
    /// contact the developers.
    ///
    /// Parameters
    /// ----------
    /// filename : str
    ///     Path to the CSV file
    ///
    /// outname : str
    ///     Name of the output HDF5 file
    ///
    /// columns : List(int), optional
    ///     List of columns to convert containing t,x,y,z, (optional vx,vy,vz), by default [0,1,2,3]
    ///
    /// header : bool, optional
    ///     True if the CSV file contains a header, by default True
    ///
    /// comment : str, optional
    ///     Comment character to ignore, by default "#"
    ///
    /// vel : bool, optional
    ///     If true the velocity will be computed from the position using the savitzky-golay filter, by default False
    ///
    /// interpolate : bool, optional
    ///     If true the particle positions will be interpolated in order to have a constant timestep, by default False
    ///
    /// radius : float, optional
    ///     Radius of the particle, by default 0.0
    ///
    /// method : str, optional
    ///     Method to use to convert the CSV file. Can be one of the following:
    ///
    /// - `chain`: The particles are chained in the file, i.e. the first particle
    ///              is followed by the second, the second by the third, etc.
    ///            all particles are stored in one file
    ///         -  id_line: This algorithm sorts the particles by their id column and
    ///            their time column. The `columns` argument must contain the
    ///            id column as the first element.
    ///
    ///     , by default chain
    ///
    //#[allow(unreachable_code, unused_variables)]
    #[pyo3(signature = (
        filename,
        outname,
        columns = vec![0,1,2,3],
        delimiter = "\",\"",
        header = true,
        comment = "\"#\"",
        vel = false,
        interpolate = false,
        radius = 0.0,
        method = "\"id_line\""
    ))]
    #[staticmethod]
    fn csv_multi(
        filename: &str,
        outname: &str,
        columns: Vec<i64>,
        delimiter: &str,
        header: bool,
        comment: &str,
        vel: bool,
        interpolate: bool,
        radius: f64,
        method: &str,
    ) -> PyResult<()> {
        //errors out immediately because of the function is not implemented
        //return Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
        //"Multi CSV reader is not implemented yet. This feature comes in future!",
        //));
        csv_multi_converter(
            filename,
            outname,
            columns,
            delimiter,
            header,
            comment,
            vel,
            interpolate,
            radius,
            method,
        );
        Ok(())
    }

    /// Convert CSV file containing multiple particles into a HDF5 file.
    ///
    /// There can be different ways how this is achieved, therefore the function
    /// takes an argument called `method` which can be one of the following:
    ///
    ///   - chain:   The particles are chained in the file, i.e. the first particle
    ///         is followed by the second, the second by the third, etc.
    ///         all particles are stored in one file.
    ///   - id_line: This algorithm sorts the particles by their id column and
    ///         their time column. The `columns` argument must contain the
    ///         id column as the first element.
    ///
    /// no other method is implemented yet. If you want to use another method, please
    /// contact the developers.
    ///
    /// Parameters
    /// ----------
    /// filenames : str
    ///     Path to the CSV files
    ///
    /// outname : str
    ///     Name of the output HDF5 file
    ///
    /// columns : List(int), optional
    ///     List of columns to convert containing pid,x,y,z,(optional vx,vy,vz)
    ///     Default: [0,1,2,3]
    ///
    /// header : bool, optional
    ///     True if the CSV file contains a header
    ///     Default: True
    ///
    /// comment : str, optional
    ///     Comment character to ignore
    ///     Default: "#"
    ///
    /// vel : bool, optional
    ///     If true the velocity will be computed from the position using the savitzky-golay filter
    ///     Default: False
    ///
    /// interpolate : bool, optional
    ///     If true the particle positions will be interpolated in order to have a constant timestep
    ///     Default: False
    ///
    /// radius : float, optional
    ///     Radius of the particle
    ///
    //#[allow(unreachable_code, unused_variables)]
    #[pyo3(signature = (
        filenames,
        outname,
        times,
        columns = vec![0,1,2,3],
        delimiter = ",",
        header = true,
        comment = "#",
        vel = false,
        interpolate = false,
        radius = 0.0
    ))]
    #[staticmethod]
    fn csv_multi_files(
        filenames: Vec<&str>,
        outname: &str,
        times: Vec<f64>,
        columns: Vec<i64>,
        delimiter: &str,
        header: bool,
        comment: &str,
        vel: bool,
        interpolate: bool,
        radius: f64,
    ) -> PyResult<()> {
        // errors out immediately because of the function is not implemented
        //return Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
        //    "Multi CSV reader is not implemented yet. This feature comes in future!",
        //));
        converter::csv_multi_file_time_step(
            filenames,
            outname,
            columns,
            times,
            delimiter,
            header,
            comment,
            vel,
            interpolate,
            radius,
        );
        Ok(())
    }
}
