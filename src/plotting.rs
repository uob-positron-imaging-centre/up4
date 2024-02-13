//! Module that handles plotting of particle data.

use plotly::{Layout, Plot, Trace};

pub mod plotting_2d;
pub use plotting_2d::*;

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
