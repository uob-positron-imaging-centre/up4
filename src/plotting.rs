//! Parent Module for plotting functions

use colorous::Gradient;
use dyn_clone::{DynClone, clone_trait_object};
use ndarray::{Array1, Array2, Array3};
use crate::GridFunctions3D;

use self::vector_plot::VectorPlotter;

//FIXME doc
pub mod vector_plot;
/// Module for handling 2D scalar data
pub mod scalar_plot;
/// Module handling plots comparing 2 datasets
pub mod comparison_plot;

// FIXME doc
pub fn axis_selector(grid: Box<dyn GridFunctions3D>, axis: usize) -> (Array1<f64>, Array1<f64>) {
    match axis {
        // yz view
        0 => {
            let xaxis = grid.get_ypositions().to_owned();
            let yaxis = grid.get_zpositions().to_owned();
            return (xaxis, yaxis)
        }
        // xz view
        1 => {
            let xaxis = grid.get_xpositions().to_owned();
            let yaxis = grid.get_zpositions().to_owned();
            return (xaxis, yaxis)
        }
        // xy view
        2 => {
            let xaxis = grid.get_xpositions().to_owned();
            let yaxis = grid.get_ypositions().to_owned();
            return (xaxis, yaxis)
        }
        // panic
        _ => panic!("axis value must be either 0, 1 or 2!")
    };
}

// FIXME doc
pub fn data_selector(grid: Box<dyn GridFunctions3D>, axis: usize, index: usize) -> Array2<f64> {
    let selected_data: Array2<f64> = grid.get_data().index_axis(ndarray::Axis(axis), index).into_owned();
    return selected_data
}

//FIXME doc
pub fn component_data_selector(data: Array3<f64>, axis: usize, index: usize) -> Array2<f64> {
    let selected_data: Array2<f64> = data.index_axis(ndarray::Axis(axis), index).into_owned();
    return selected_data
}
/*
// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    /*
    Some quick shortcuts for reference

    mark each test function with #[test]
    ignore test function with #[ignore]
    if a test *should* cause a panic, use #[should_panic(expected = panic-message)]
    */

    // Helper functions
    fn create_array()

    // Tests
    
    // 2D 
    #[test]
    fn create_arrow_data()

    #[test]
    #[should_panic]
    fn uneven_array_input()
        // try it with uneven y then u then v

    // 3D
}
*/