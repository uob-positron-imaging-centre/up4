//!
//! Grids implement ndarrays which hold data. This grid wrapper allows easy grid operations such as
//! finding a special cell

//TODO Rewrite grids to make dimensionality a generic instead of 3 structs

extern crate ndarray;
use crate::types::*;
use derive_getters::Getters;
use dyn_clone::{clone_trait_object, DynClone};
use ndarray::prelude::*;
use num_traits;

use std::any::Any;
pub mod cartesian_grid;
pub use cartesian_grid::CartesianGrid3D;
pub mod cylindrical_grid;
pub use cylindrical_grid::CylindricalGrid3D;
pub mod vector_grid;
pub use vector_grid::VectorGrid;

use anyhow::Result;
//mod grid2dpolar;
//pub use grid2dpolar::Grid2DPolar;
/// Provides a generic way to send ranges to nD-Grid struct.
///
/// Each Dimension is defined by n - tuples of n*2 numbers to define the range in each dimension
/// # Example
///
/// ```rust
/// use upppp_rust::Dim;
/// let range1d = Dim::OneD([[0.,10.]]);    //Define a range between zero and ten.
/// let range2d = Dim::TwoD([[0.,10.],[10.2,18.]]);
/// let range3d = Dim::ThreeD([[0.,10.],[1.0,2.0],[-5.1,18.2]]);
/// ```
///
#[derive(Clone)]
pub enum Dim {
    OneD(OneD),
    TwoD(TwoD),
    ThreeD(ThreeD),
}

//pub trait Grid: Debug {}
pub trait GridFunctions3D: DynClone + std::fmt::Display + std::fmt::Debug + Send {
    //get value at this cell id
    fn get_value(&self, pos: Position) -> f64;

    // add to the value at the same position
    fn add_value(&mut self, pos: Position, value: f64);

    // add values to the grid according to the trajectories between two points
    fn add_trajectory_value(&mut self, pos1: Position, pos2: Position, value: f64);

    // divide the whole array by another
    fn divide_by_array(&mut self, other: &Array3<f64>);

    // divide the whole array by another
    fn divide_by_scalar(&mut self, other: f64);

    // divide by the weights
    fn divide_by_weight(&mut self);

    // insert value in cell at his position
    fn insert(&mut self, pos: Position, value: f64);

    // Check if particle/ number is inside the overall dimensions
    fn is_inside(&self, pos: Position) -> bool;

    // Return cell ID of Data/Particle
    fn cell_id(&self, pos: Position) -> Result<CellId>;

    fn cell_ids_in_trajectory(
        &self,
        pos1: Position,
        pos2: Position,
    ) -> Result<(Vec<CellId>, Vec<f64>)>;

    // add a number to a cell given its cel id
    fn add_to_cell(&mut self, cell_id: CellId, value: f64);

    // Needed for python interface ( check that again, might be not needed)
    fn as_any(&self) -> &dyn Any;

    // return a new instance of grid with zeros
    fn new_zeros(&self) -> Box<dyn GridFunctions3D>;

    fn collapse(&self, axis: usize) -> Array2<f64>;

    fn collapse_weight(&self, axis: usize) -> Array2<f64>;

    fn collapse_two(&self, axis1: usize, axis2: usize) -> Array1<f64>;

    fn collapse_two_weight(&self, axis1: usize, axis2: usize) -> Array1<f64>;

    fn slice(&self, axis: usize, position: f64) -> Array2<f64>;

    fn slice_idx(&self, axis: usize, index: usize) -> Array2<f64>;
    //cellcenters

    // histogram
    // Need to write getters in here
    fn get_xpositions(&self) -> &Array1<f64>;
    fn get_ypositions(&self) -> &Array1<f64>;
    fn get_zpositions(&self) -> &Array1<f64>;
    fn get_limits(&self) -> &[[f64; 2]; 3];
    fn get_cells(&self) -> &CellId;
    fn get_data(&self) -> &Array3<f64>;
    fn get_weights(&self) -> &Array3<f64>;
    fn is_cylindrical(&self) -> bool;

    fn set_data(&mut self, data: Array3<f64>);

    fn set_weights(&mut self, weights: Array3<f64>);

    fn outlier_removal(&mut self, threshold: f64, mode: usize); // mode 0: set all values above threshold to zero, mode 1: set all values above threshold to threshold mode 2: set all values above threshold to mean of surrounding all values
}
clone_trait_object!(GridFunctions3D);

#[derive(Getters, Clone)]
pub struct Grid<F> {
    cells: Array1<usize>,
    positions: Array3<f64>,
    limits: Dim,
    data: Array3<F>,
}
impl<F> Grid<F> {
    fn new<Sh>(_shape: Sh) -> Self
    where
        Sh: ShapeBuilder + Clone,
        F: Clone + num_traits::identities::Zero,
    {
        Grid {
            cells: array![0, 0, 0],
            positions: Array3::<f64>::ones([10, 10, 10]),
            limits: Dim::OneD([[0.0, 0.0]]),
            data: Array3::<F>::zeros([10, 10, 10]),
        }
    }
}
