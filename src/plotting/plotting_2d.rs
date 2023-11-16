//! Submodule for handling 2D plots.
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

pub mod parity_contour;
pub mod parity_map;
pub mod parity_plot;
pub mod quiver;
pub mod scalar_contour;
pub mod scalar_map;
pub mod unit_vector;
