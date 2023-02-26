use crate::{print_debug, setup_bar};

use super::check_signals;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::{Itertools, TakeWhileRef};
use ndarray::{self, ArrayView1};
use polyfit_rs;
pub mod velocity_paralell;
pub fn interpolate(
    data: ndarray::Array2<f64>,
    max_time: f64,
    steps: usize,
) -> ndarray::Array2<f64> {
    // TODO implement different types of interpolation
    //let bar = ProgressBar::new(data.column(0).len() as u64);
    /*bar.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} Interpolation [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% {per_sec} ({eta})")
    .with_key("eta", |state| format!("Time left: {:.1}s", state.eta().as_secs_f64()))
    .with_key("per_sec", |state| format!("{:.1} steps/s", state.per_sec()))
    .progress_chars("#>-"));*/
    //let bar = setup_bar!("Interpolate", data.column(0).len());
    if !is_sorted(&data.column(0)) {
        panic!("Data is not sorted by time. Please sort the data before interpolation.")
    }
    let time: ArrayView1<f64> = data.slice(ndarray::s![.., 0_usize]);
    let x: ArrayView1<f64> = data.slice(ndarray::s![.., 1_usize]);
    let y: ArrayView1<f64> = data.slice(ndarray::s![.., 2_usize]);
    let z: ArrayView1<f64> = data.slice(ndarray::s![.., 3_usize]);

    let maxtime = max_time;
    let timesteps = steps;
    let dt = maxtime / timesteps as f64;
    // First timestep:
    let mut interp_data = ndarray::Array2::<f64>::zeros((steps, 4));
    // first timestep:
    interp_data[[0, 0]] = time[0];
    interp_data[[0, 1]] = x[0];
    interp_data[[0, 2]] = y[0];
    interp_data[[0, 3]] = z[0];
    // last timestep:
    interp_data[[timesteps - 1, 0]] = time[time.len() - 1];
    interp_data[[timesteps - 1, 1]] = x[time.len() - 1];
    interp_data[[timesteps - 1, 2]] = y[time.len() - 1];
    interp_data[[timesteps - 1, 3]] = z[time.len() - 1];
    // loop over whole dataset and figure out the location at each timestep
    let mut real_step = 1;
    for step in 1..timesteps - 1 {
        let time_new = step as f64 * dt;
        // find the next indx in the real data which may be the old one
        real_step = {
            // if the temporal distance between new time and old time is smaller
            // then distance between new time and next time return old timestep
            print_debug!("Real_step:{} time_len {}", real_step, time.len());
            if real_step < time.len() - 1 {
                let old_time = time[real_step];
                let new_time = time[real_step + 1];
                print_debug!("Old time: {} New time: {}", old_time, new_time);
                if new_time < old_time {
                    real_step += 1;
                } else if (time_new - old_time).abs() < (time_new - new_time).abs() {
                } else {
                    real_step += 1;
                    if real_step < time.len() - 1 {
                        let old_time = time[real_step];
                        let new_time = time[real_step + 1];

                        if (time_new - old_time).abs() < (time_new - new_time).abs() {
                        } else {
                            real_step += 1;
                        }
                    }
                }
            }
            real_step
        };
        print_debug!("Interpolate:  step: {}, real_step: {}, time_new: {}, time[real_step]: {}, time[real_step+1]: {}", step, real_step, time_new, time[real_step], time[real_step+1]);

        if real_step >= data.shape()[0] - 1 {
            interp_data[[step, 0]] = time_new;
            interp_data[[step, 1]] = f64::NAN;
            interp_data[[step, 2]] = f64::NAN;
            interp_data[[step, 3]] = f64::NAN;
        } else {
            if data[[real_step - 1, 0]] == data[[real_step + 1, 0]] {
                interp_data[[step, 0]] = time_new;
                interp_data[[step, 1]] = f64::NAN;
                interp_data[[step, 2]] = f64::NAN;
                interp_data[[step, 3]] = f64::NAN;
                continue;
            }
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
        if step % 20000 == 0 {
            //bar.inc(20000);
            check_signals!();
        }
    }
    //bar.finish();
    interp_data
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

    //let bar = setup_bar!("Velocity Calc", new_data.column(0).len());
    for id in (sampling_steps - 1) / 2..data.column(0).len() - (sampling_steps - 1) / 2 {
        //println!("ID: {}", id);
        //println!("sampling_steps: {}", sampling_steps);
        //println!("{}", id - (sampling_steps - 1) / 2);
        //println!("{}", id + (sampling_steps - 1) / 2);
        let datasegment = data.slice(ndarray::s![
            id - (sampling_steps - 1) / 2..id + (sampling_steps - 1) / 2 + 1,
            ..
        ]);
        let time = datasegment.column(0).to_owned();
        let time = (&time - time[0]).to_vec();
        //println!("Time len: {}", time.len());
        //println!("Time: {:?}", time);

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
        print_debug!("Vel: {},{},{}", vx, vy, vz);
        print_debug!(
            "T: {:?} Pos:{:?},{:?},{:?}",
            data[[id, 0]],
            data[[id, 1]],
            data[[id, 2]],
            data[[id, 3]]
        );
        if id % 2000 == 0 {
            //bar.inc(2000);
            check_signals!();
        }
    } // End loop over dataset
      //bar.finish();
    new_data
}

fn interpolate_step(
    time_old: f64,
    time_new: f64,
    time_current: f64,
    pos_old: f64,
    pos_new: f64,
) -> f64 {
    
    pos_old + ((pos_new - pos_old) / (time_new - time_old)) * (time_current - time_old)
}

// ndarray sorting by elements in another array
pub fn sort_by_array<T>(data: Vec<T>, idx: &Vec<usize>) -> Vec<T>
where
    T: std::cmp::PartialOrd + Clone + Copy,
{
    print_debug!("      Sorting data");

    let mut array_sorted = Vec::new();
    if data.len() == idx.len() {
        for i in 0..idx.len() {
            array_sorted.push(data[idx[i]]);
        }
    } else {
        for i in 0..idx.len() {
            array_sorted.push(data[3 * idx[i]]);
            array_sorted.push(data[3 * idx[i] + 1]);
            array_sorted.push(data[3 * idx[i] + 2]);
        }
    }

    print_debug!("      Done");
    array_sorted
}

pub fn make_sortlist(particle_ids: &Vec<u64>) -> Vec<usize> {
    // make vec with length of particle_ids
    // thanks to that stackoverflow....
    let mut indices = (0..particle_ids.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &particle_ids[i]);
    indices
}

pub fn sort_by_column<T>(data: ndarray::Array2<T>, column: usize) -> ndarray::Array2<T>
where
    T: std::cmp::PartialOrd + Clone + Copy + num_traits::Zero,
{
    let mut indices = (0..data.column(0).len()).collect::<Vec<_>>();
    let mut new_data = ndarray::Array2::<T>::zeros((data.shape()[0], data.shape()[1]));
    indices.sort_by(|a, b| {
        let one = data[[*a, column]];
        let two = data[[*b, column]];
        one.partial_cmp(&two).unwrap()
    });
    for (i, index) in indices.into_iter().enumerate() {
        new_data.row_mut(i).assign(&data.row(index));
    }
    
    new_data
}

pub fn is_sorted<T>(data: &ndarray::ArrayView1<T>) -> bool
where
    T: std::cmp::PartialOrd + Clone + Copy,
{
    data.into_iter().tuple_windows().all(|(a, b)| a <= b)
}

#[cfg(feature = "blosc")]
use hdf5::filters::{blosc_get_nthreads, blosc_set_nthreads};
//// Make a builder for blosc datasets in hdf5 so we dont have to change stuff all the time

macro_rules! make_dataset_builder {
    ($group:expr) => {{
        let builder = $group.new_dataset_builder();
        #[cfg(feature = "blosc")]
        let builder = builder.blosc_zstd(BLOSC_COMPRESSION, BLOSC_SHUFFLE);
        // return builder
        builder
    }};
}

pub(crate) use make_dataset_builder;
