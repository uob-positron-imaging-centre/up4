use crate::{print_debug, setup_bar};

use super::check_signals;
use indicatif::{ProgressBar, ProgressStyle};
use ndarray::{self};
use polyfit_rs;
use rayon::prelude::*;

pub fn velocity_polynom_parallel(
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

    //let bar = setup_bar!("Velocity Calc", data.column(0).len());
    let x: Vec<ndarray::Array2<f64>> = data
        .windows((7, 7))
        .into_iter()
        .map(|x| x.to_owned().into_shape((7, 7)).unwrap())
        .collect();
    let new_data: Vec<ndarray::Array1<f64>> = x
        .par_iter()
        .enumerate()
        .map(|(id, datasegment)| {
            let mut result_array = ndarray::Array1::<f64>::zeros(7);
            //let datasegment = data.slice(ndarray::s![
            //   id - (sampling_steps - 1) / 2..id + (sampling_steps - 1) / 2,
            //       ..
            //  ]);
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
            result_array[0] = datasegment[[(sampling_steps - 1) / 2, 0]];
            result_array[1] = datasegment[[(sampling_steps - 1) / 2, 1]];
            result_array[2] = datasegment[[(sampling_steps - 1) / 2, 2]];
            result_array[3] = datasegment[[(sampling_steps - 1) / 2, 3]];
            result_array[4] = vx;
            result_array[5] = vy;
            result_array[6] = vz;

            print_debug!("{},{},{}", vx, vy, vz);
            print_debug!("{:?},{:?},{:?}", param_x, param_y, param_z);
            if id % 10000 == 0 {
                //bar.inc(10000);
                check_signals!();
            }
            result_array
        })
        .collect(); // End loop over dataset

    //bar.finish();
    to_array2(new_data).unwrap()
}

fn to_array2<T: Copy>(
    source: Vec<ndarray::Array1<T>>,
) -> Result<ndarray::Array2<T>, impl std::error::Error> {
    let width = source.len();
    let flattened: ndarray::Array1<T> = source.into_iter().flat_map(|row| row.to_vec()).collect();
    let height = flattened.len() / width;
    flattened.into_shape((width, height))
}
