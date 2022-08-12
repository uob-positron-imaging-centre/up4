use crate::print_debug;

use super::check_signals;
use indicatif::{ProgressBar, ProgressStyle};
use ndarray;
use polyfit_rs;
pub fn interpolate(data: ndarray::Array2<f64>) -> ndarray::Array2<f64> {
    // TODO implement different types of interpolation
    let time = data.slice(ndarray::s![0 as usize, ..]);
    let x = data.slice(ndarray::s![1 as usize, ..]);
    let y = data.slice(ndarray::s![2 as usize, ..]);
    let z = data.slice(ndarray::s![3 as usize, ..]);
    let maxtime = time[time.len() - 1];
    let timesteps = time.len();
    let dt = maxtime / timesteps as f64;
    // First timestep:
    let mut interp_data = ndarray::Array2::<f64>::zeros((timesteps, 4));
    // first timestep:
    interp_data[[0, 0]] = time[0];
    interp_data[[0, 1]] = x[0];
    interp_data[[0, 2]] = y[0];
    interp_data[[0, 3]] = z[0];
    // last timestep:
    interp_data[[timesteps - 1, 0]] = time[timesteps - 1];
    interp_data[[timesteps - 1, 1]] = x[timesteps - 1];
    interp_data[[timesteps - 1, 2]] = y[timesteps - 1];
    interp_data[[timesteps - 1, 3]] = z[timesteps - 1];

    // loop over whole dataset and figure out the location at each timestep
    for step in 1..timesteps - 1 {
        let time_new = interp_data[[step - 1, 0]] + dt;
        // find index in real data:
        // TODO OPTIMISE: This function is highly optimizable
        let real_step = argmin(time, time_new);
        let x_new = interpolate_step(
            data[[real_step - 1, 0]],
            data[[real_step + 1, 0]],
            time_new,
            data[[real_step - 1, 1]],
            data[[real_step + 1, 1]],
        );
        let y_new = interpolate_step(
            data[[real_step - 1, 0]],
            data[[real_step + 1, 0]],
            time_new,
            data[[real_step - 1, 2]],
            data[[real_step + 1, 2]],
        );
        let z_new = interpolate_step(
            data[[real_step - 1, 0]],
            data[[real_step + 1, 0]],
            time_new,
            data[[real_step - 1, 3]],
            data[[real_step + 1, 3]],
        );
        interp_data[[step, 0]] = time_new;
        interp_data[[step, 1]] = x_new;
        interp_data[[step, 2]] = y_new;
        interp_data[[step, 3]] = z_new;
    }
    data
}

pub fn velocity_polynom(
    data: ndarray::Array2<f64>,
    sampling_steps: usize,
    degree: usize,
) -> ndarray::Array2<f64> {
    if sampling_steps % 2 == 0 {
        panic!(
            "Sampling_steps must be a uneven number. not {}",
            sampling_steps
        )
    }
    let mut new_data =
        ndarray::Array2::<f64>::zeros((data.column(0).len() - (sampling_steps - 1), 7));
    let bar = ProgressBar::new(new_data.column(0).len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% {per_sec} ({eta})")
        .with_key("eta", |state| format!("Time left: {:.1}s", state.eta().as_secs_f64()))
        .with_key("per_sec", |state| format!("{:.1} steps/s", state.per_sec()))
        .progress_chars("#>-"));
    for id in (sampling_steps - 1) / 2..data.column(0).len() - (sampling_steps - 1) / 2 {
        let datasegment = data.slice(ndarray::s![
            id - (sampling_steps - 1) / 2..id + (sampling_steps - 1) / 2,
            ..
        ]);
        let time = datasegment.column(0).to_owned();
        let time = (&time - time[0]).to_vec();
        // fit a cur:?ve using polyfit-rs
        // do this for each dimension
        let param_x: Vec<f64> =
            polyfit_rs::polyfit_rs::polyfit(&time, &datasegment.column(1).to_vec(), degree)
                .expect("Error while fitting polynom to data in x-direction");
        let param_y: Vec<f64> =
            polyfit_rs::polyfit_rs::polyfit(&time, &datasegment.column(2).to_vec(), degree)
                .expect("Error while fitting polynom to data in y-direction");
        let param_z: Vec<f64> =
            polyfit_rs::polyfit_rs::polyfit(&time, &datasegment.column(3).to_vec(), degree)
                .expect("Error while fitting polynom to data in z-direction");

        // Calculation of the velocities
        // location for the velocities is the middlepoint of the polynom
        let vx: f64 = param_x
            .iter()
            .enumerate()
            .map(|(n, param)| {
                param * n as f64 * time[(sampling_steps - 1) / 2 + 1].powf(n as f64 - 1.)
            })
            .sum();
        let vy: f64 = param_y
            .iter()
            .enumerate()
            .map(|(n, param)| {
                param * n as f64 * time[(sampling_steps - 1) / 2 + 1].powf(n as f64 - 1.)
            })
            .sum();
        let vz: f64 = param_z
            .iter()
            .enumerate()
            .map(|(n, param)| {
                param * n as f64 * time[(sampling_steps - 1) / 2 + 1].powf(n as f64 - 1.)
            })
            .sum();
        new_data[[id - (sampling_steps - 1) / 2, 0]] = data[[id, 0]];
        new_data[[id - (sampling_steps - 1) / 2, 1]] = data[[id, 1]];
        new_data[[id - (sampling_steps - 1) / 2, 2]] = data[[id, 2]];
        new_data[[id - (sampling_steps - 1) / 2, 3]] = data[[id, 3]];
        new_data[[id - (sampling_steps - 1) / 2, 4]] = vx;
        new_data[[id - (sampling_steps - 1) / 2, 5]] = vy;
        new_data[[id - (sampling_steps - 1) / 2, 6]] = vz;
        print_debug!("{},{},{}", vx, vy, vz);
        print_debug!("{:?},{:?},{:?}", param_x, param_y, param_z);
        bar.inc(1);

        check_signals!();
    } // End loop over dataset
    bar.finish();
    new_data
}

fn interpolate_step(
    time_old: f64,
    time_new: f64,
    time_current: f64,
    pos_old: f64,
    pos_new: f64,
) -> f64 {
    let pos_current =
        pos_old + ((pos_new - pos_old) / (time_new - time_old)) * (time_current - time_old);
    pos_current
}

fn argmin(array: ndarray::ArrayView1<f64>, point: f64) -> usize {
    let mut argmin = usize::MAX;
    let mut min = f64::MAX;
    for (step, x) in array.iter().enumerate() {
        if (x - point).abs() < min {
            min = (x - point).abs();
            argmin = step;
        }
    }
    argmin
}
