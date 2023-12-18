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

#[cfg(test)]
mod test {

    use std::f64::consts;

    use ndarray::{Array1, Array3};

    use crate::{
        utilities::maths::meshgrid3d,
        CartesianGrid3D, Dim, GridFunctions3D, VectorGrid,
    };


    // Helper functions
    fn vector_grid() -> VectorGrid {
        let limit = Dim::ThreeD([[-2.0, 2.0], [-2.0, 2.0], [-2.0, 2.0]]);
        let n = 10;
        let cells = [n; 3];

        const PI: f64 = consts::PI;
        let x = Array1::range(
            0.,
            2. * PI + PI / ((n - 1) as f64),
            2. * PI / ((n - 1) as f64),
        );
        let y = Array1::range(
            0.,
            2. * PI + PI / ((n - 1) as f64),
            2. * PI / ((n - 1) as f64),
        );
        let z = Array1::range(-1.0, 1.0 + 1.0 / ((n - 1) as f64), 2.0 / ((n - 1) as f64));
        println!("{:?}", x.shape());
        println!("{:?}", y.shape());
        println!("{:?}", z.shape());
        let (xx, yy, _) = meshgrid3d(&x, &y, &z);
        let u = &xx.mapv(f64::sin) * &yy.mapv(f64::cos);
        let v = -&yy.mapv(f64::sin) * &xx.mapv(f64::cos);
        let w = Array3::ones(cells) * 2.0;
        let mut ugrid = CartesianGrid3D::new(cells, limit.clone());
        let mut vgrid = CartesianGrid3D::new(cells, limit.clone());
        let mut wgrid = CartesianGrid3D::new(cells, limit);
        println!("{:?}", u.shape());
        println!("{:?}", v.shape());
        println!("{:?}", w.shape());
        ugrid.set_data(u);
        vgrid.set_data(v);
        wgrid.set_data(w);

        let mut grid = VectorGrid::new(Box::new(ugrid));
        grid.data[1] = Box::new(vgrid);
        grid.data[2] = Box::new(wgrid);

        println!("{:?}", grid);

        grid
    }
    // fn direct_construction() -> QuiverPlot {}

    // fn vector_grid() -> QuiverPlot {}

    // Tests

    // correct construction
    #[test]
    fn quiver_from_vector_grid() {
        let grid = vector_grid();
    }

    // arrows created properly

    // norm is adjusted properly

    // scaling correctness

    // colour range is determined properly

    //

    // traces are created properly
}
