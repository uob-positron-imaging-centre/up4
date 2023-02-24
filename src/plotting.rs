//! Module that handles plotting of particle data.

use crate::GridFunctions3D;
use ndarray::{Array1, Array2, Array3};
use plotly::{Layout, Plot, Trace};

pub mod colourmap;
pub mod comparison_plot;
pub mod plotting_2d;
pub mod scalar_plot;
pub mod vector_plot;

/// Take traces and plot them
pub fn plot(traces: Vec<Box<dyn Trace>>, layout: Layout) -> Plot {
    let mut plot: Plot = Plot::new();
    //use local render version
    plot.use_local_plotly();
    for trace in traces {
        plot.add_trace(trace);
    }
    plot.set_layout(layout);

    plot
}
/// Return plane positions perpendicular to the provided axis.
pub fn axis_selector(grid: Box<dyn GridFunctions3D>, axis: usize) -> (Array1<f64>, Array1<f64>) {
    match axis {
        // yz view
        0 => {
            let xaxis = grid.get_ypositions().to_owned();
            let yaxis = grid.get_zpositions().to_owned();
            (xaxis, yaxis)
        }
        // xz view
        1 => {
            let xaxis = grid.get_xpositions().to_owned();
            let yaxis = grid.get_zpositions().to_owned();
            (xaxis, yaxis)
        }
        // xy view
        2 => {
            let xaxis = grid.get_xpositions().to_owned();
            let yaxis = grid.get_ypositions().to_owned();
            (xaxis, yaxis)
        }
        // panic
        _ => panic!("axis value must be either 0, 1 or 2!"),
    }
}

/// Return data in plane perpendicular to provided axis, at the provided index.
pub fn data_selector(grid: Box<dyn GridFunctions3D>, axis: usize, index: usize) -> Array2<f64> {
    let selected_data: Array2<f64> = grid
        .get_data()
        .index_axis(ndarray::Axis(axis), index)
        .into_owned();
    selected_data
}

/// Return data in plane perpendicular to provided axis, at the provided index.
pub fn component_data_selector(data: Array3<f64>, axis: usize, index: usize) -> Array2<f64> {
    let selected_data: Array2<f64> = data.index_axis(ndarray::Axis(axis), index).into_owned();
    selected_data
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
