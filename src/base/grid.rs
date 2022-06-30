//!
//! Grids implement ndarrays which hold data. This grid wrapper allows easy grid operations such as
//! finding a special cell

//TODO Rewrite grids to make dimensionality a generic instead of 3 structs

extern crate ndarray;
use crate::print_debug;
use derive_getters::Getters;
use dyn_clone::{clone_trait_object, DynClone};
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use num_traits;
use pyo3::prelude::*;
use std::any::Any;

pub mod grid1d;
pub mod grid2d;
pub mod grid3d;
pub use grid1d::Grid1D;
pub use grid2d::Grid2D;
pub use grid3d::Grid3D;

type OneD = [[f64; 2]; 1];
type TwoD = [[f64; 2]; 2];
type ThreeD = [[f64; 2]; 3];
//mod grid2dpolar;
//pub use grid2dpolar::Grid2DPolar;
/// Provides a generic way to send ranges to nD-Grid struct.
///
/// Each Dimenion is defined by n - tuples of n*2 numbers to define the range in each dimension
/// # Example
///
/// ```rust
/// let range1d = Dim::OneD((0.,10.));    //Define a range between zero and ten.
/// let range2d = Dim::TwoD((0.,10.),(10.2,18.));
/// let range3d = Dim::ThreeD((0.,10.),(1.0,2.0),(-5.1,18.2));
/// ```
///
#[derive(Clone)]
pub enum Dim {
    OneD(OneD),
    TwoD(TwoD),
    ThreeD(ThreeD),
}

pub enum GridType {
    Grid1D(Grid1D),
    Grid2D(Grid2D),
    Grid3D(Grid3D),
}

//pub trait Grid: Debug {}
pub trait GridFunctions: DynClone {
    // Clone instance with possibly new data type
    // fn new_data<B: Default + Clone>(&self)->Self<B>;

    // Check if particle/ number is inside the overall dimensions
    fn is_inside(&self, num: Vec<f64>) -> bool;

    // Return cell ID of Data/Particle
    fn cell_id(&self, num: Vec<f64>) -> Array1<usize>;

    // Check if boundaries are bigger then the system
    //fn check_boundaries(&self, stats: GlobalStats) -> bool;

    // Adjust maximum boundaries to the Global max
    //fn adjust_boundaries(&self, stats:  GlobalStats);
    fn as_any(&self) -> &dyn Any;

    //collaps
    //slice
    // cellcenters
}
clone_trait_object!(GridFunctions);

#[derive(Getters, Clone)]
pub struct Grid<F, D>
where
    D: Dimension,
{
    dimension: D,
    cells: Array1<usize>,
    positions: Array<f64, D>,
    limits: Dim,
    data: Array<F, D>,
}
impl<D: Dimension, F> Grid<F, D> {
    fn new<Sh>(dim: D, shape: Sh) -> Self
    where
        Sh: ShapeBuilder<Dim = D> + Clone,
        F: Clone + num_traits::identities::Zero,
    {
        println!("{}", dim.ndim());
        Grid {
            dimension: dim,
            cells: array![0, 0, 0],
            positions: Array::<f64, D>::ones(shape.clone()),
            limits: Dim::OneD([[0.0, 0.0]]),
            data: Array::<F, D>::zeros(shape),
        }
    }
}

impl<F: 'static + Clone, D: 'static + Dimension> GridFunctions for Grid<F, D> {
    fn is_inside(&self, particle_position: Vec<f64>) -> bool {
        print_debug!("Grid1D: Checking if {:?} is in grid", num);
        match self.limits {
            Dim::OneD(limit) => limit
                .iter()
                .zip(particle_position.iter())
                .map(|(lim, pos)| pos > &lim[0] && pos < &lim[1])
                .all(|value| value),
            Dim::TwoD(limit) => limit
                .iter()
                .zip(particle_position.iter())
                .map(|(lim, pos)| pos > &lim[0] && pos < &lim[1])
                .all(|value| value),
            Dim::ThreeD(limit) => limit
                .iter()
                .zip(particle_position.iter())
                .map(|(lim, pos)| pos > &lim[0] && pos < &lim[1])
                .all(|value| value),
        }
    }

    fn cell_id(&self, position: Vec<f64>) -> Array1<usize> {
        print_debug!("Grid1D: Checking if {:?} is in grid", num);
        if &position.len() != &self.dimension.ndim() {
            panic!(
                "GridDimensionError: The supplied position has the wrong \
            number of entities for this grid. Can not find Cell ID."
            )
        }
        position
            .iter()
            .enumerate()
            .map(|(dim, particle_pos)| {
                let b = &self
                    .positions
                    .iter()
                    .map(|cell_pos| cell_pos - particle_pos)
                    .collect::<Array1<f64>>()
                    .argmin()
                    .expect("Unable to find Cell ID of Particle on Grid");
                *b
            })
            .collect::<Array1<usize>>()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[pyclass(name = "Grid")]
#[derive(Clone)]
pub struct PyGrid {
    pub grid: Box<dyn GridFunctions + Send>,
}

#[pymethods]
impl PyGrid {
    #[new]
    fn constructor() -> PyGrid {
        Self {
            grid: Box::new(Grid1D::new(Array1::zeros(1), Dim::OneD([[0., 1.]]))),
        }
    }

    #[staticmethod]
    fn create1d(cells: usize, xlim: OneD) -> PyGrid {
        Self {
            grid: Box::new(Grid1D::new(array![cells], Dim::OneD(xlim))),
        }
    }

    #[staticmethod]
    fn create2d(cells: (usize, usize), xlim: (f64, f64), ylim: (f64, f64)) -> PyGrid {
        Self {
            grid: Box::new(Grid2D::new(
                array![cells.0, cells.1],
                Dim::TwoD([[xlim.0, xlim.1], [ylim.0, ylim.1]]),
            )),
        }
    }

    #[staticmethod]
    fn create3d(
        cells: (usize, usize, usize),
        xlim: (f64, f64),
        ylim: (f64, f64),
        zlim: (f64, f64),
    ) -> PyGrid {
        Self {
            grid: Box::new(Grid3D::new(
                array![cells.0, cells.1, cells.2],
                Dim::ThreeD([[xlim.0, xlim.1], [ylim.0, ylim.1], [zlim.0, zlim.1]]),
            )),
        }
    }
}
