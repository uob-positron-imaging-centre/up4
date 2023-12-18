//! Module of common maths functions exposed to users
use ndarray::prelude::*;
pub fn minmax(arr: &Array1<f64>) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for value in arr.iter() {
        if value > &max {
            max = *value;
        }
        if value < &min {
            min = *value;
        }
    }
    (min, max)
}

pub fn meshgrid(
    x: &ndarray::Array1<f64>,
    y: &ndarray::Array1<f64>,
) -> (ndarray::Array2<f64>, ndarray::Array2<f64>) {
    let mut xx = ndarray::Array2::<f64>::zeros((x.len(), y.len()));
    let mut yy = ndarray::Array2::<f64>::zeros((x.len(), y.len()));

    for idx in 0..x.len() {
        for idy in 0..y.len() {
            xx[[idx, idy]] = x[idx];
            yy[[idx, idy]] = y[idy];
        }
    }
    (xx, yy)
}

pub fn meshgrid3d(
    x: &ndarray::Array1<f64>,
    y: &ndarray::Array1<f64>,
    z: &ndarray::Array1<f64>,
) -> (
    ndarray::Array3<f64>,
    ndarray::Array3<f64>,
    ndarray::Array3<f64>,
) {
    let mut xx = ndarray::Array3::<f64>::zeros((x.len(), y.len(), z.len()));
    let mut yy = ndarray::Array3::<f64>::zeros((x.len(), y.len(), z.len()));
    let mut zz = ndarray::Array3::<f64>::zeros((x.len(), y.len(), z.len()));
    for idx in 0..x.len() {
        for idy in 0..y.len() {
            for idz in 0..z.len() {
                xx[[idx, idy, idz]] = x[idx];
                yy[[idx, idy, idz]] = y[idy];
                zz[[idx, idy, idz]] = z[idz];
            }
        }
    }
    (xx, yy, zz)
}

/// Flattens a 2D array into a 1D array.
pub fn flatten_2d(arr: &Array2<f64>) -> Array1<f64> {
    return arr
        .slice(s![0..arr.shape()[0], 0..arr.shape()[1]]) //create slice of all elements
        .iter() //create iterable
        .copied() //iterate through
        .collect::<Array1<f64>>(); //collect into array
}

/// Flattens a 3D array into a 1D array.
pub fn flatten_3d(arr: &Array3<f64>) -> Array1<f64> {
    return arr
        .slice(s![0..arr.shape()[0], 0..arr.shape()[1], 0..arr.shape()[2]]) //create slice of all elements
        .iter() //create iterable
        .copied() //iterate through
        .collect::<Array1<f64>>(); //collect into array
}
