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
    m.add_wrapped(wrap_pyfunction!(mean_angular_velocity))?;
    m.add_wrapped(wrap_pyfunction!(timesteps))?;
    m.add_wrapped(wrap_pyfunction!(occupancy_plot1d))?;
    m.add_wrapped(wrap_pyfunction!(occupancy_plot1d_pept))?;
    m.add_wrapped(wrap_pyfunction!(velocity_plot1d))?;
    m.add_wrapped(wrap_pyfunction!(surface_polynom))?;
    m.add_wrapped(wrap_pyfunction!(granular_temperature))?;
    m.add_wrapped(wrap_pyfunction!(mean_velocity))?;
    m.add_wrapped(wrap_pyfunction!(dispersion))?;
    m.add_wrapped(wrap_pyfunction!(dispersion_pept))?;
    m.add_wrapped(wrap_pyfunction!(surface_velocity))?;
    m.add_wrapped(wrap_pyfunction!(rotational_velocity_distribution))?;
    m.add_wrapped(wrap_pyfunction!(velocity_distribution))?;
    m.add_wrapped(wrap_pyfunction!(force_distribution))?;
    m.add_wrapped(wrap_pyfunction!(mean_squared_displacement))?;
    m.add_wrapped(wrap_pyfunction!(mean_squared_displacement_pept))?;
    m.add_wrapped(wrap_pyfunction!(power_draw))?;
    m.add_wrapped(wrap_pyfunction!(mean_surface_velocity))?;
    m.add_wrapped(wrap_pyfunction!(circulation_time_boundary))?;
    m.add_wrapped(wrap_pyfunction!(circulation_time_boundary_pept))?;
    m.add_wrapped(wrap_pyfunction!(shear_rate))?;
    m.add_wrapped(wrap_pyfunction!(granular_temperature_2d))?;
    m.add_wrapped(wrap_pyfunction!(trajectory))?;
    m.add_wrapped(wrap_pyfunction!(trajectory_pept))?;
    m.add_wrapped(wrap_pyfunction!(velocity_pept))?;
    m.add_wrapped(wrap_pyfunction!(time))?;
    m.add_wrapped(wrap_pyfunction!(time_pept))?;
    m.add_wrapped(wrap_pyfunction!(jumps))?;
    m.add_wrapped(wrap_pyfunction!(occupancyfield))?;
    m.add_wrapped(wrap_pyfunction!(velocityfield))?;
    m.add_wrapped(wrap_pyfunction!(numberfield))?;
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
fn timesteps(filename:&str)->usize {
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

#[pyfunction]
fn occupancy_plot1d_pept<'py>(
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
    let (image, arr) = functions::occupancy_plot1d_pept(
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

#[pyfunction]
fn velocity_plot1d<'py>(
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
    let (image, arr) = functions::velocity_plot1d(
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
///     dispersionion: PyArray1<f64>
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
    dimensions:PyReadonlyArray2<f64>,
    cells: PyReadonlyArray1<i64>,
) -> (&'py PyArrayDyn<f64>,f64) {
    let (dispersion_cells, mixing_efficiency) = functions::dispersion(
        file,
        timestep.as_array().to_owned(),
        delta_t,
        mesh_size.as_array().to_owned(),
        dimensions.as_array().to_owned(),
        cells.as_array().to_owned(),
    );

    // return dispersion
    (dispersion_cells.into_pyarray(_py).to_dyn(), mixing_efficiency)

}

#[pyfunction]
fn dispersion_pept<'py>(
    _py: Python<'py>,
    file: &str,
    delta_t: f64,
    mesh_size: PyReadonlyArray1<f64>,
    dimensions:PyReadonlyArray2<f64>,
    cells: PyReadonlyArray1<i64>,
    max_error: f64
) -> (&'py PyArrayDyn<f64>,f64) {

    let (dispersion_cells, mixing_efficiency) = functions::dispersion_pept(
        file,
        delta_t,
        mesh_size.as_array().to_owned(),
        dimensions.as_array().to_owned(),
        cells.as_array().to_owned(),
        max_error,
    );

    // return dispersion
    (dispersion_cells.into_pyarray(_py).to_dyn(), mixing_efficiency)

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

#[pyfunction]
fn mean_angular_velocity<'py>(
    py: Python<'py>,
    filename: &str,
    min_time: f64,

) -> (f64) {
    let mean_velocity = functions::mean_angular_velocity(filename, min_time);
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

/// [Force Distribution Function]:
/// Calculate teh velocity distribution in your system
#[pyfunction]
fn force_distribution<'py>(
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
    let (party_id, force_dist, num_axis_array) = functions::force_distribution(
        filename,
        bins,
        min_time,
        max_time,
    );
    (
        party_id,
        force_dist.into_pyarray(_py).to_dyn(),
        num_axis_array.into_pyarray(_py).to_dyn(),
    )
}

/// [Velocity Distribution Function]:
/// Calculate teh velocity distribution in your system
#[pyfunction]
fn rotational_velocity_distribution<'py>(
    _py: Python<'py>,
    filename: &str, // which contains vel & pos data
    bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
    rot_speed: f64,  // rotation of the drum in rpm
    drum_center: PyReadonlyArray1<f64>, // center of the drum
    timestep: usize
) -> (
    Vec<Vec<i64>>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
) {
    let (party_id, vel_dist, num_axis_array) = functions::rotational_velocity_distribution(
        filename,
        bins,
        min_time,
        max_time,
        rot_speed,
        drum_center.as_array().to_owned(),
        timestep
    );
    (
        party_id,
        vel_dist.into_pyarray(_py).to_dyn(),
        num_axis_array.into_pyarray(_py).to_dyn(),
    )
}

#[pyfunction]
fn mean_surface_velocity(
    filename: &str, // which contains vel & pos data
    mut bins: i64,  // bins can be defined by the user, but the default value is */10 the amount
    min_time: f64,  // where to start the averaging
    max_time: f64,  // where to end the averaging
    rot_speed: f64,  // rotation of the drum in rpm
    drum_center: PyReadonlyArray1<f64>, // center of the drum

)->f64{
    functions::mean_surface_velocity(
        filename,
        bins,
        min_time,
        max_time,
        rot_speed,
        drum_center.as_array().to_owned()
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
fn mean_squared_displacement_pept<'py>(
    _py: Python<'py>,
    filename: &str,
    start_timestep: usize,
    max_msd: f64,
    bins: usize,
)->(&'py PyArrayDyn<f64>, &'py PyArrayDyn<f64>) {
    let ( MSD, time ) = functions::mean_squared_displacement_pept(filename, start_timestep,max_msd,bins);
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


#[pyfunction]
fn circulation_time_boundary<'py>(_py: Python<'py>, filename:&str, boundary:PyReadonlyArray1<f64>, axis: usize,min_time:f64)->(Vec<f64>, Vec<f64>, Vec<f64>) {
    functions::circulation_time_boundary(filename,boundary.as_array().to_owned(),axis,min_time)

}
#[pyfunction]
fn circulation_time_boundary_pept<'py>(_py: Python<'py>, filename:&str, boundary:PyReadonlyArray1<f64>, axis: usize)->(Vec<f64>, Vec<f64>, Vec<f64>) {
    functions::circulation_time_boundary_pept(filename,boundary.as_array().to_owned(),axis)

}


#[pyfunction]
fn shear_rate<'py>(_py: Python<'py>,
    filename: &str,          //filename of hdf5 file
    cells: PyReadonlyArray1<f64>,      //number of cells to store vec-data
    min_time: f64,           //where to start the averaging
    max_time: f64,           //where to end the averaging
    dimensions: PyReadonlyArray2<f64>, // Region where to look at, rest ignored
    radius: PyReadonlyArray1<f64>,     // include a radius
    particle_id: PyReadonlyArray1<i64>,
    axis: usize,
    line_offset: f64,
    line_gradient: f64,
) -> (
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<f64>,
    &'py PyArrayDyn<i64>,
    &'py PyArrayDyn<i64>,
    f64
) {
    let (px,py,sx,sy,x,y,tau) =functions::shear_rate(
        filename,
        cells.as_array().to_owned(),
        min_time,
        max_time,
        dimensions.as_array().to_owned(),
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
        axis,
        line_offset,
        line_gradient
    );
    (px.into_pyarray(_py).to_dyn(),
    py.into_pyarray(_py).to_dyn(),
    sx.into_pyarray(_py).to_dyn(),
    sy.into_pyarray(_py).to_dyn(),
    x.into_pyarray(_py).to_dyn(),
    y.into_pyarray(_py).to_dyn(),
    tau
)
}


#[pyfunction]
fn granular_temperature_2d<'py>(
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

    let (temp, ngrid, sx, sy) = functions::granular_temperature_2d(
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
    temp.into_pyarray(_py).to_dyn(),
    ngrid.into_pyarray(_py).to_dyn(),
    sx.into_pyarray(_py).to_dyn(),
    sy.into_pyarray(_py).to_dyn(),
)
}

#[pyfunction]
fn trajectory<'py>(
    _py: Python<'py>,
    filename: &str,          //filename of hdf5 file
    ID: usize,      //number of cells to store vec-data
)-> &'py PyArrayDyn<f64>{
functions::trajectory(filename, ID).into_pyarray(_py).to_dyn()

}

#[pyfunction]
fn trajectory_pept<'py>(
    _py: Python<'py>,
    filename: &str,          //filename of hdf5 file
)-> &'py PyArrayDyn<f64>{
functions::trajectory_pept(filename).into_pyarray(_py).to_dyn()

}

#[pyfunction]
fn velocity_pept<'py>(
    _py: Python<'py>,
    filename: &str,          //filename of hdf5 file
)-> &'py PyArrayDyn<f64>{
functions::velocity_pept(filename).into_pyarray(_py).to_dyn()

}

#[pyfunction]
fn time<'py>(
    _py: Python<'py>,
    filename: &str,          //filename of hdf5 file
)-> &'py PyArrayDyn<f64>{
functions::time(filename).into_pyarray(_py).to_dyn()

}

#[pyfunction]
fn time_pept<'py>(
    _py: Python<'py>,
    filename: &str,          //filename of hdf5 file
)-> &'py PyArrayDyn<f64>{
functions::time_pept(filename).into_pyarray(_py).to_dyn()

}

#[pyfunction]
fn jumps<'py>(
    _py: Python<'py>,
    filename: &str,          //filename of hdf5 file
    min_velocity: f64,
    min_distance: f64,
)-> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>){
functions::jumps(filename, min_velocity, min_distance)

}


/// [Vectorfield Function]:
/// Calculate the velocity Vectorfiels of your System
#[pyfunction]
fn occupancyfield<'py>(
    _py: Python<'py>,
    filename: &str,                      //filename of hdf5 file
    cells: PyReadonlyArray1<f64>,        //number of cells to store vec-data
    min_time: f64,                       //where to start the averaging
    max_time: f64,                       //where to end the averaging
    dimensions: PyReadonlyArray2<f64>, // Region where to look at, rest ignored
    radius: PyReadonlyArray1<f64>,       // include a radius, only available for sim-data
    particle_id: PyReadonlyArray1<i64>,
    axis: usize,
) ->
    (&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>)
     {

    let (sx,sy,field) = functions::fields::occupancyfield(
        filename,
        cells.as_array().to_owned(),
        min_time,
        max_time,
        dimensions.as_array().to_owned(),
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
        axis,
    );

    (sx.into_pyarray(_py).to_dyn(),sy.into_pyarray(_py).to_dyn(),field.into_pyarray(_py).to_dyn())
}

/// [Vectorfield Function]:
/// Calculate the velocity Vectorfiels of your System
#[pyfunction]
fn velocityfield<'py>(
    _py: Python<'py>,
    filename: &str,                      //filename of hdf5 file
    cells: PyReadonlyArray1<f64>,        //number of cells to store vec-data
    min_time: f64,                       //where to start the averaging
    max_time: f64,                       //where to end the averaging
    dimensions: PyReadonlyArray2<f64>, // Region where to look at, rest ignored
    radius: PyReadonlyArray1<f64>,       // include a radius, only available for sim-data
    particle_id: PyReadonlyArray1<i64>,
    axis: usize,
) ->
    (&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>)
     {

    let (sx,sy,field) = functions::fields::velocityfield(
        filename,
        cells.as_array().to_owned(),
        min_time,
        max_time,
        dimensions.as_array().to_owned(),
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
        axis,
    );

    (sx.into_pyarray(_py).to_dyn(),sy.into_pyarray(_py).to_dyn(),field.into_pyarray(_py).to_dyn())
}

/// [Vectorfield Function]:
/// Calculate the velocity Vectorfiels of your System
#[pyfunction]
fn numberfield<'py>(
    _py: Python<'py>,
    filename: &str,                      //filename of hdf5 file
    cells: PyReadonlyArray1<f64>,        //number of cells to store vec-data
    min_time: f64,                       //where to start the averaging
    max_time: f64,                       //where to end the averaging
    dimensions: PyReadonlyArray2<f64>, // Region where to look at, rest ignored
    radius: PyReadonlyArray1<f64>,       // include a radius, only available for sim-data
    particle_id: PyReadonlyArray1<i64>,
    axis: usize,
) ->
    (&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>)
     {

    let (sx,sy,field) = functions::fields::numberfield(
        filename,
        cells.as_array().to_owned(),
        min_time,
        max_time,
        dimensions.as_array().to_owned(),
        radius.as_array().to_owned(),
        particle_id.as_array().to_owned(),
        axis,
    );

    (sx.into_pyarray(_py).to_dyn(),sy.into_pyarray(_py).to_dyn(),field.into_pyarray(_py).to_dyn())
}
