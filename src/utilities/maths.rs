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
