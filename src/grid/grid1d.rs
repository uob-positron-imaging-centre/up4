extern crate ndarray;
use crate::print_debug;
use derive_getters::Getters;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use std::any::Any;

use super::{Dim, GridFunctions, OneD};

/// One dimensional grid that allowes storage of `cells[0]` datapoints of type `<T: Clone + Debug>`
/// in the equally spaced range of `xlim`, such as velocity distributions.
/// ```rust
/// let grid = Grid1D::new(
///             arr![10.],              // 10 cells
///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
///             0.0,                    // Initiate with zeros
/// };
/// ```
#[derive(Getters, Clone)]
pub struct Grid1D {
    cells: Array1<usize>,
    positions: Array1<f64>, // midpoint of each cell
    limits: OneD,
    // attrs: HashMap<String, >,
}

impl Grid1D {
    /// Struct constructor.
    /// ```rust
    /// let grid = Grid1D::new(
    ///             arr![10.],              // 10 cells
    ///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
    ///             0.0,                    // Initiate with zeros
    /// };
    /// ```
    pub fn new(cells: Array1<usize>, limit: Dim) -> Self {
        print_debug!("Grid1D: Generating new grid");
        if cells.shape()[0] != 1 {
            panic!(
                "Grid1D got wrong Cell-shape.\\
                    Array should only hold a single number. Not {:?}",
                cells.shape()
            )
        }
        let xlim = match limit {
            Dim::OneD(s) => s,
            _ => panic!("Grid1D got limits for other then one dimension."),
        };
        let cellsize = (xlim[0][1] - xlim[0][0]) / cells[0] as f64;
        let mut positions = Array1::<f64>::zeros(cells[0]);
        for cellid in 0..cells[0] as usize {
            positions[cellid] = cellid as f64 * cellsize + cellsize
        }
        print_debug!(
            "Grid1D:\n\tCells: {:?} \n\tpositions: {:?} \n\tlim: {:?}",
            cells,
            positions,
            xlim
        );

        Grid1D {
            cells,
            positions,
            limits: xlim,
        }
    }

    fn data_array<T: Default + Clone>(&self) -> Array1<T> {
        Array1::from_elem(self.cells[0] as usize, T::default())
    }
}

impl GridFunctions for Grid1D {
    /// Check if a position is inside cell and return a bool
    /// ```rust
    /// let grid = Grid1D::new(
    ///             arr![10],              // 10 cells
    ///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
    ///             0.0,                    // Initiate with zeros
    /// };
    /// assert_eq!(grid.is_inside(12.5), true);
    /// ```
    fn is_inside(&self, num: Vec<f64>) -> bool {
        print_debug!("Grid1D: Checking if {:?} is in grid", num);
        let pos = num[0];
        pos > self.limits[0][0] && pos < self.limits[0][1]
    }
    /// Return cell id of cell that holds a position
    /// ```rust
    /// let grid = Grid1D::new(
    ///             arr![10.],              // 10 cells
    ///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
    ///             0.0,                    // Initiate with zeros
    /// }
    /// let cellid = grid.cell_id()
    /// ```
    fn cell_id(&self, num: Vec<f64>) -> Array1<usize> {
        print_debug!("Grid1D: Checking if {:?} is in grid", num);
        let pos = num[0];
        let cell_id = (&self.positions - pos)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect("lol");
        array![cell_id]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
