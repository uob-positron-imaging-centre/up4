//!
//! Grids implement ndarrays which hold data. This grid wrapper allows easy grid operations such as
//! finding a special cell

//TODO Rewrite grids to make dimensionality a generic instead of 3 structs

extern crate ndarray;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use crate::{GlobalStats,print_debug};
use numpy::PyArray1;use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArray1,PyReadonlyArray2, ToPyArray};
use std::fmt::Debug;
use std::any::Any;
use derive_getters::Getters;
use dyn_clone::{clone_trait_object, DynClone};
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
pub enum Dim {
    OneD((f64, f64)),
    TwoD((f64, f64), (f64, f64)),
    ThreeD((f64, f64), (f64, f64), (f64, f64)),
}
pub trait Grid: Debug{

}
pub trait GridFunctions: DynClone{

    // Clone instance with possibly new data type
    // fn new_data<B: Default + Clone>(&self)->Self<B>;

    // Check if particle/ number is inside the overall dimensions
    fn is_inside(&self,num: Vec<f64>) -> bool;

    // Return cell ID of Data/Particle
    fn cell_id(&self,num: Vec<f64>) -> Array1<usize>;

    // Check if boundaries are bigger then the system
    //fn check_boundaries(&self, stats: GlobalStats) -> bool;

    // Adjust maximum boundaries to the Global max
    //fn adjust_boundaries(&self, stats:  GlobalStats);
    fn as_any(&self)-> &dyn Any;
}
clone_trait_object!(GridFunctions);

/// One dimensional grid that allowes storage of `cells[0]` datapoints of type `<T: Clone + Debug>`
/// in the equally spaced range of `xlim`, such as velocity distributions.
/// ```rust
/// let grid = Grid1D::new(
///             arr![10.],              // 10 cells
///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
///             0.0,                    // Initiate with zeros
/// };
/// ```
#[derive(Getters,Clone)]
pub struct Grid1D{
    cells: Array1<usize>,
    positions: Array1<f64>, // midpoint of each cell
    xlim: (f64, f64),
    // attrs: HashMap<String, >,
}

impl Grid1D{
    /// Struct constructor.
    /// ```rust
    /// let grid = Grid1D::new(
    ///             arr![10.],              // 10 cells
    ///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
    ///             0.0,                    // Initiate with zeros
    /// };
    /// ```
    fn new(cells:Array1<usize>, limit: Dim)-> Self{
        print_debug!("Grid1D: Generating new grid");
        if cells.shape()[0] != 1 {
                panic!("Grid1D got wrong Cell-shape.\\
                    Array should only hold a single number. Not {:?}",cells.shape())
        }
        let xlim = match limit{
            Dim::OneD(s)=> s,
            _ => panic!("Grid1D got limits for other then one dimension.")

        };
        let cellsize = (xlim.1-xlim.0)/cells[0] as f64;
        let mut positions = Array1::<f64>::zeros(cells[0]);
        for cellid in 0..cells[0] as usize{
            positions[cellid] = cellid as f64 * cellsize +cellsize
        }
        print_debug!(
            "Grid1D:\n\tCells: {:?} \n\tpositions: {:?} \n\tlim: {:?}",
            cells,positions,xlim
        );

        Grid1D{
            cells,
            positions,
            xlim,
        }
    }

    fn data_array<T: Default+Clone>(&self)->Array1<T>{
        Array1::from_elem(self.cells[0] as usize,T::default())
    }
}

impl GridFunctions for Grid1D{
    /// Check if a position is inside cell and return a bool
    /// ```rust
    /// let grid = Grid1D::new(
    ///             arr![10],              // 10 cells
    ///             Dim::OneD((0.,100.)),   // dimensions range from 0 to 100
    ///             0.0,                    // Initiate with zeros
    /// };
    /// assert_eq!(grid.is_inside(12.5), true);
    /// ```
    fn is_inside(&self, num: Vec<f64>)-> bool{
        print_debug!("Grid1D: Checking if {:?} is in grid", num);
        let pos = num[0];
        pos > self.xlim.0 && pos < self.xlim.1
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
    fn cell_id(&self, num: Vec<f64>)-> Array1<usize>{
        print_debug!("Grid1D: Checking if {:?} is in grid", num);
        let pos = num[0];
        let cell_id = (&self.positions-pos)
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

#[derive(Getters,Clone)]
pub struct Grid2D{
    cells: Array1<usize>,
    xpositions: Array1<f64>,
    ypositions:Array1<f64>,
    xlim: (f64, f64),
    ylim: (f64, f64),
    // attrs: HashMap<String, >,
}

impl  Grid2D{
    pub fn new(cells:Array1<usize>, limit: Dim)-> Self{
        print_debug!("Grid2D: Generating new grid");
        if cells.shape()[0] != 2 {
                panic!("Grid2D got wrong Arrayshape.\\
                    Array should only hold a single number.")
        }

        let (xlim, ylim) = match limit{
            Dim::TwoD(s,y)=> (s,y),
            _ => panic!("Grid2D got limits for other then two dimensions.")

        };
        let xcellsize = (xlim.1-xlim.0)/cells[0] as f64;
        let ycellsize = (ylim.1-ylim.0)/cells[1] as f64;
        let mut xpositions = Array::from_elem(cells[0], 0.);
        let mut ypositions = Array::from_elem(cells[1], 0.);
        for cellidx in 0..cells[0]{
            xpositions[cellidx as usize ] = cellidx as f64 * xcellsize + xlim.0;
        }
        for cellidy in 0..cells[1]{
            ypositions[cellidy as usize ] = cellidy as f64 * ycellsize + ylim.0;
        }
        print_debug!(
            "Grid2D:\n\tCells: {:?} \n\txpositions: {:?}\\
             \n\typositions: {:?} \n\txlim: {:?} \n\tylim: {:?}",
            cells,xpositions,ypositions,xlim,ylim
        );

        Grid2D{
            cells,
            xpositions,
            ypositions,
            xlim,
            ylim,
        }
    }

    pub fn data_array<T: Default+Clone>(&self)->Array2<T>{
        Array2::from_elem((self.cells[0] as usize,self.cells[1] as usize),T::default())
    }
}

impl GridFunctions for Grid2D{


    fn is_inside(&self, num: Vec<f64>)-> bool{
        print_debug!("Grid2D: Checking if {:?} is in grid", num);
        let posx = num[0];
        let posy = num[1];
        posx > self.xlim.0 && posx < self.xlim.1 &&
        posy > self.ylim.0 && posy < self.ylim.1
    }

    fn cell_id(&self, num: Vec<f64>)-> Array1<usize>{
        let posx = num[0];
        print_debug!("Checking array {:?} with position {:?}",&self.xpositions,posx);
        let cell_idx = (&self.xpositions-posx)
                        .iter()
                        .map(|x| x.abs())
                        .collect::<Array1<f64>>()
                        .argmin()
                        .expect(&format!("Can not find min of {:?} in Gri2D",num));
        print_debug!("Result: {}",cell_idx);
        let posy = num[1];
        print_debug!("Checking array {:?} with position {:?}",&self.ypositions,posy);
        let cell_idy = (&self.ypositions-posy)
                        .iter()
                        .map(|x| x.abs())
                        .collect::<Array1<f64>>()
                        .argmin()
                        .expect(&format!("Can not find min of {:?} in Gri2D",num));
        print_debug!("Result: {}",cell_idy);
        array![cell_idx,cell_idy]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Getters,Clone)]
pub struct Grid3D{
    cells: Array1<usize>,
    xpositions: Array1<f64>,
    ypositions:Array1<f64>,
    zpositions:Array1<f64>,
    xlim: (f64, f64),
    ylim: (f64, f64),
    zlim: (f64, f64),
    // attrs: HashMap<String, >,
}

impl  Grid3D{
    fn new(cells:Array1<usize>, limit: Dim)-> Self{
        print_debug!("Grid3D: Generating new grid");
        if cells.shape()[0] != 3 {
                panic!("Grid1D got wrong Arrayshape.\\
                    Array should only hold a single number.")
        }

        let (xlim, ylim, zlim) = match limit{
            Dim::ThreeD(s,y,z)=> (s,y, z),
            _ => panic!("Grid3D got limits for other then three dimensions.")

        };
        let xcellsize = (xlim.1-xlim.0)/cells[0] as f64;
        let ycellsize = (ylim.1-ylim.0)/cells[1] as f64;
        let zcellsize = (zlim.1-zlim.0)/cells[2] as f64;
        let mut xpositions = Array::from_elem(cells[0], 0.);
        let mut ypositions = Array::from_elem(cells[1], 0.);
        let mut zpositions = Array::from_elem(cells[1], 0.);
        for cellidx in 0..cells[0]{
            xpositions[cellidx as usize ] = cellidx as f64 * xcellsize + xcellsize;
        }
        for cellidy in 0..cells[1]{
            ypositions[cellidy as usize ] = cellidy as f64 * ycellsize + ycellsize;
        }
        for cellidz in 0..cells[2]{
            zpositions[cellidz as usize ] = cellidz as f64 * zcellsize + zcellsize;
        }
        print_debug!(
            "Grid3D:\n\tCells: {:?} \n\txpositions: {:?} \n\t\\
            ypositions: {:?} \n\tzpositions: {:?} \n\txlim: {:?} \\
            \n\tylim: {:?} \n\tzlim: {:?}",
            cells,xpositions,ypositions,zpositions,xlim,ylim,zlim,
        );

        Grid3D{
            cells,
            xpositions,
            ypositions,
            zpositions,
            xlim,
            ylim,
            zlim,
        }
    }

    pub fn into_py(&self)->PyGrid{
        PyGrid{
            grid: Box::new(self.clone())
        }
    }


}

impl GridFunctions for Grid3D{
    fn is_inside(&self, num: Vec<f64>)-> bool{
        print_debug!("Grid3D: Checking if {:?} is in grid", num);
        let posx = num[0];
        let posy = num[1];
        let posz = num[2];
        posx > self.xlim.0 && posx < self.xlim.1 &&
        posy > self.ylim.0 && posy < self.ylim.1 &&
        posz > self.zlim.0 && posz < self.zlim.1
    }

    fn cell_id(&self, num: Vec<f64>)-> Array1<usize>{
        print_debug!("Grid3D: Checking if {:?} is in grid", num);
        let posx = num[0];
        let cell_idx = (&self.xpositions-posx)
                        .iter()
                        .map(|x| x.abs())
                        .collect::<Array1<f64>>()
                        .argmin()
                        .expect(&format!("Can not find min of {:?} in Gri3D",num));
        let posy = num[1];
        let cell_idy = (&self.ypositions-posy)
                        .iter()
                        .map(|x| x.abs())
                        .collect::<Array1<f64>>()
                        .argmin()
                        .expect(&format!("Can not find min of {:?} in Gri3D",num));
        let posz = num[2];
        let cell_idz = (&self.zpositions-posz)
                        .iter()
                        .map(|z| z.abs())
                        .collect::<Array1<f64>>()
                        .argmin()
                        .expect(&format!("Can not find min of {:?} in Gri3D",num));
        array![cell_idx,cell_idy]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }


}




#[pyclass(name="Grid")]
#[derive(Clone)]
pub struct PyGrid  {
        grid: Box<dyn GridFunctions+Send>
}

impl PyGrid{
    pub fn to_grid1d(&self)->Grid1D{
        let grid1d: Grid1D = match self.grid.as_any().downcast_ref::<Grid1D>(){
            Some(b) => b.clone(),
            None => panic!("Can not convert PyGrid to Grid1D ")
        };
        grid1d
    }

    pub fn to_grid2d(&self)->Grid2D{
        let grid2d: Grid2D = match self.grid.as_any().downcast_ref::<Grid2D>(){
            Some(b) => b.clone(),
            None => panic!("Can not convert PyGrid to Grid2D ")
        };
        grid2d
    }

    pub fn to_grid3d(&self)->Grid3D{
        let grid3d: Grid3D = match self.grid.as_any().downcast_ref::<Grid3D>(){
            Some(b) => b.clone(),
            None => panic!("Can not convert PyGrid to Grid3D ")
        };
        grid3d
    }
}

#[pymethods]
impl PyGrid{
    #[new]
    fn constructor()->PyGrid{
        Self{
            grid: Box::new(Grid1D::new(Array1::zeros(1),Dim::OneD((0.,1.))))
        }
    }

    #[staticmethod]
    fn create1d(cells: usize, xlim: (f64,f64))->PyGrid{
        Self{
            grid: Box::new(Grid1D::new(array![cells],Dim::OneD(xlim)))
        }
    }

    #[staticmethod]
    fn create2d(cells: (usize,usize), xlim: (f64,f64),ylim: (f64,f64))->PyGrid{
        Self{
            grid: Box::new(Grid2D::new(array![cells.0,cells.1],Dim::TwoD(xlim,ylim)))
        }
    }

    #[staticmethod]
    fn create3d(cells: (usize,usize,usize), xlim: (f64,f64),ylim: (f64,f64),zlim: (f64,f64))->PyGrid{
        Self{
            grid: Box::new(Grid3D::new(
                array![cells.0,cells.1,cells.2],
                Dim::ThreeD(xlim,ylim,zlim)
            ))
        }
    }

}
