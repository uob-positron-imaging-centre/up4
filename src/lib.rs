mod functions;
//mod functions_gpu;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
extern crate numpy;
extern crate ndarray;
extern crate ndarray_linalg;
use ndarray::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArray1,PyReadonlyArray2, PyReadonlyArrayDyn, ToPyArray};


/// Directory Module
/// The directory is responsible for connecting the python wrapper to rust
/// [!] If a new function is added, it must be included in this Directory Module.
#[pymodule]
fn rustAnalyser(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(vectorfield))?;
    m.add_wrapped(wrap_pyfunction!(timesteps))?;
    m.add_wrapped(wrap_pyfunction!(occupancy_plot1d))?;
    m.add_wrapped(wrap_pyfunction!(surface_polynom))?;
    m.add_wrapped(wrap_pyfunction!(granular_temperature))?;
    m.add_wrapped(wrap_pyfunction!(mean_velocity))?;
    m.add_wrapped(wrap_pyfunction!(dispersion))?;
    m.add_wrapped(wrap_pyfunction!(surface_velocity))?;
    m.add_wrapped(wrap_pyfunction!(velocity_distribution))?;
    m.add_wrapped(wrap_pyfunction!(mean_squared_displacement))?;
    m.add_wrapped(wrap_pyfunction!(power_draw))?;
    Ok(())
}
/// [Vectorfield Function]:
/// Calculate the velocity Vectorfiels of your System
#[pyfunction]
fn vectorfield<'py>(
    _py: Python<'py>,
    filename: &str,                      //filename of hdf5 file
    cells: PyReadonlyArray1<f64>,        //number of cells to store vec-data
    min_time: f64,                       //where to start the averaging
    max_time: f64,                       //where to end the averaging
    dimensions: PyReadonlyArray2<f64>, // Region where to look at, rest ignored
    norm_on: bool,                       //normalise the size of the vectors
    radius: PyReadonlyArray1<f64>,       // include a radius, only available for sim-data
    particle_id: PyReadonlyArray1<i64>,
    axis: usize,
) -> (
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
) {

    let (vx, vy, sx, sy) = functions::vectorfield(
        filename,
        cells.as_array().to_owned(),
        min_time,
        max_time,
        dimensions.as_array().to_owned(),
        norm_on,
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
        axis,
    );
    (
    vx.into_pyarray(_py).to_dyn(),
    vy.into_pyarray(_py).to_dyn(),
    sx.into_pyarray(_py).to_dyn(),
    sy.into_pyarray(_py).to_dyn(),
)
}


#[pyfunction]
fn timesteps(filename:&str)->(usize) {
    //get the number of timesteps
    let file = hdf5::File::open(filename).expect("Error reading hdf5 file in rust");
    let t = functions::timesteps(&file);
    file.close();
    // return
    t as usize
}

#[pyfunction]
fn occupancy_plot1d<'py>(
    py: Python<'py>,
    filename: &str,
    radius: PyReadonlyArray1<f64>,
    particle_id: PyReadonlyArray1<i64>,
    clouds: bool,
    axis: usize,
    norm: bool,
    min_time: f64,
    cells: f64,
) -> (&'py PyArrayDyn<f64>, &'py PyArrayDyn<f64>) {
    /*
    function to calculate the time averaged occupancy plot of a particle system


     */
    let (image, arr) = functions::occupancy_plot1d(
        filename,
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
        clouds,
        axis,
        norm,
        min_time,
        cells,
    );
    (
        image.into_pyarray(py).to_dyn(),
        arr.into_pyarray(py).to_dyn(),
    )
}
///Granular Temperature Calculator
/// This function creates a 1D Array containing the overall
/// Granular temperature of the system for each timestep
///
/// Input parameters:
///     Filename: Filename to the Hdf5 file Type: string
///
/// Outputs
///     PythonArray of size (1xN)
///
/// '''
/// let file = "test.hdf5"
/// let Array = granular_temperature(file)
/// '''
#[pyfunction]
fn granular_temperature<'py>(_py: Python<'py>, filename: &str,min_time: f64) -> &'py PyArrayDyn<f64> {
    /*
    function to calculate the granular temperature of a system for a amount of timesteps

     */
    let result = functions::granular_temperature(
        filename,
        min_time
    );
    result.to_pyarray(_py).to_dyn()
}

#[pyfunction]
fn surface_velocity<'py>(
    _py: Python<'py>,
    filename: &str,                      //filename of hdf5 file
    cells: PyReadonlyArray1<f64>,        //number of cells to store vec-data
    min_time: f64,                       //where to start the averaging
    max_time: f64,                       //where to end the averaging
    dimensions: PyReadonlyArray1<f64>, // Region where to look at, rest ignored
    radius: PyReadonlyArray1<f64>,       // include a radius, only available for sim-data
    particle_id: PyReadonlyArray1<i64>,
) -> (
    &'py PyArrayDyn<f64>
) {
    let x = functions::surface_velocity(
        filename,
        cells.as_array().to_owned(),
        min_time,
        max_time,
        dimensions.as_array().to_owned(),
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
    );


        x.into_pyarray(_py).to_dyn()
}

///Function to calculate the disperson of a system in a given point
/// Output:
///     Dispersion: PyArray1<f64>
/// Input:
///     delta_t: timestep difference for the dispersion calculation
///     time: timestep where to start the calculation
///     position: current position where to calculate the dispersion
///     delta_position: cubic cell around "position"
///
/// Errors:
///     If delta_t + time is bigger then the max timesteps
///
#[pyfunction]
fn dispersion<'py>(
    _py: Python<'py>,
    file: &str,
    timestep: PyReadonlyArray1<usize>,
    delta_t: usize,
    mesh_size: PyReadonlyArray1<f64>,
    cells: PyReadonlyArray1<i64>,
) -> Vec<Vec<Vec<f64>>> {
    let dispersion_cells = functions::dispersion(
        file,
        timestep.as_array().to_owned(),
        delta_t,
        mesh_size.as_array().to_owned(),
        cells.as_array().to_owned(),
    );

    // return dispersion
    dispersion_cells

}

#[pyfunction]
fn mean_velocity<'py>(
    py: Python<'py>,
    filename: &str,
    min_time: f64,

) -> (f64) {
    let mean_velocity = functions::mean_velocity(filename, min_time);
    mean_velocity
}

/// [Velocity Distribution Function]:
/// Calculate teh velocity distribution in your system
#[pyfunction]
fn velocity_distribution<'py>(
    _py: Python<'py>,
    filename: &str, // which contains vel & pos data
    mut bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
) -> (
    Vec<Vec<i64>>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
) {
    let (party_id, vel_dist, num_axis_array) = functions::velocity_distribution(
        filename,
        bins,
        min_time,
        max_time,
    );
    (
        party_id,
        vel_dist.into_pyarray(_py).to_dyn(),
        num_axis_array.into_pyarray(_py).to_dyn(),
    )
}

#[pyfunction]
fn surface_polynom<'py>(
    _py: Python<'py>,
    filename: &str,
    axis: usize,
    timestep: u64,
    mut ntimesteps: u64,
    threshold: f64,
) -> (&'py PyArrayDyn<f64>, f64) {
    /*
    function to calculate the time averaged occupancy plot of a particle system
     */
     let (image, arr) = functions::surface_polynom(
         filename,
         axis,
         timestep,
         ntimesteps,
         threshold,
     );

    (
        image.into_pyarray(_py).to_dyn(),
        arr
    )
}

#[pyfunction]
fn mean_squared_displacement<'py>(
    _py: Python<'py>,
    filename: &str,
    start_timestep: usize,
)->(&'py PyArrayDyn<f64>, &'py PyArrayDyn<f64>) {
    let ( MSD, time ) = functions::mean_squared_displacement(filename, start_timestep);
    // return to Python
    (
        MSD.into_pyarray(_py).to_dyn(),
        time.into_pyarray(_py).to_dyn(),
    )
}

#[pyfunction]
fn power_draw<'py>(_py: Python<'py>, filename:&str, min_time:f64)->&'py PyArrayDyn<f64> {
    functions::power_draw(filename,min_time).into_pyarray(_py).to_dyn()

}
