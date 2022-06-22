//! Parent Module for plotting functions

use colorous::Gradient;
use ndarray::{Array, Array1};
use plotly::{common::ColorScalePalette, Plot, Layout, layout::Axis};

/// Traits for 2D and 3D data where:
/// Type D is the array dimension
/// Type P is the plot object
pub trait VectorData<T, D, P> {
    fn scale_global(&mut self, scale_factor: f64); 
    fn scale_elementwise(&mut self, scale_array:Array<T, D>);
    fn bound_min(&mut self, min: &f64);
    fn bound_max(&mut self, max: &f64);
    fn bound_min_max(&mut self, min: &f64, max: &f64);
    fn bound_node(&mut self, dx: &f64);
    fn normalise_vectors(&mut self);
    fn normalise_colour(&self, colour_bounds: &Option<(f64, f64)>) -> (Array1<f64>, f64, f64);
    fn create_plotly_traces(&self, arrow_scale: Option<f64>, colour: Gradient, colour_bounds: Option<(f64, f64)>) -> Vec<Box<P>>;
    fn vector_plot(&self, traces: Vec<Box<P>>, layout: Layout, square: bool, axes: Vec<Option<Axis>>) -> Plot;
    fn auto_axis_range(&self, layout: Layout, axes: Vec<Axis>, dtick: f64) -> Layout;
}

/// Module for creating 2D quiver plots
pub mod vector2d;

/// Module for creating 3D cone plots
pub mod vector3d;

// Convenience functions that act on a plotly plot - for 3D we need to act on Scene instead
// Manually set x axis range
//pub fn axis_range_x(layout: &mut Layout, xaxis: Axis, xmin: f64, xmax:f64) {
//    layout.x_axis(xaxis.range(vec![xmin, xmax]));
//}

// Manually set y axis range
//pub fn axis_range_y(layout: &mut Layout, yaxis: Axis, ymin: f64, ymax:f64) {
//    layout.y_axis(yaxis.range(vec![ymin, ymax]));
//}

// Manually set z axis range
// pub fn axis_range_z(layout: &mut Layout, zaxis: Axis, zmin: f64, zmax:f64) {
//    layout.z_axis(zaxis.range(vec![zmin, zmax]));
// }

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