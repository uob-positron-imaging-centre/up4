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

pub fn minmax2d(arr:&Array2<f64>)->(Array1<f64>, Array1<f64>){
    let mut min = arr.map_axis(Axis(0),|view| *view.iter().min_by_key(|value| value).unwrap());
    let mut max = arr.map_axis(Axis(0),|view| *view.iter().max().unwrap());

    (min,max)
}
