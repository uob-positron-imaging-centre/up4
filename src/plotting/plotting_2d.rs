//! Submodule for handling 2D plots.
mod parity_contour;
mod parity_map;
mod parity_plot;
mod quiver;
mod scalar_contour;
mod scalar_map;
mod unit_vector;


/// Internal representation of arrow data
struct Arrow {
    base: (f64, f64),
    tip: (f64, f64),
    left_point: (f64, f64),
    right_point: (f64, f64),
}

impl Arrow {
    fn new(
        base: (f64, f64),
        tip: (f64, f64),
        left_point: (f64, f64),
        right_point: (f64, f64),
    ) -> Arrow {
        Arrow {
            base,
            tip,
            left_point,
            right_point,
        }
    }
}
