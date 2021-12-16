use ndarray::prelude::*;
pub fn minmax(arr: &Array1<f64>) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for value in arr.iter() {
        if value > &max {
            max = value.clone();
        }
        if value < &min {
            min = value.clone();
        }
    }
    (min, max)
}

/// Flattens a 2D array into a 1D array. 
fn flatten_2d(arr:&Array2<f64>) -> Array1<f64>{
    return arr.slice(s![0..arr.shape()[0], 0..arr.shape()[1]]) //create slice of all elements
            .iter() //create iterable
            .copied() //iterate through
            .collect::<Array1<f64>>() //collect into array
}

/// Flattens a 3D array into a 1D array. 
fn flatten_3d(arr:&Array3<f64>) -> Array1<f64>{
    return arr.slice(s![0..arr.shape()[0], 0..arr.shape()[1], 0..arr.shape()[2]]) //create slice of all elements
            .iter() //create iterable
            .copied() //iterate through
            .collect::<Array1<f64>>() //collect into array
}