use super::{Dim,GridFunctions};
use derive_getters::Getters;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use crate::{print_debug};
use std::any::Any;
type position = (f64,f64,f64);
type polarcoordinates = (f64,f64,f64);
#[derive(Getters,Clone)]
pub struct Grid2DPolar{
    cells: Array1<usize>,
    xpositions: Array1<f64>,
    ypositions:Array1<f64>,
    xlim: (f64, f64),
    ylim: (f64, f64),
    // attrs: HashMap<String, >,
    polarvector: (f64, f64, f64),
    polarorigin: (f64, f64, f64)
}


impl  Grid2DPolar{
    pub fn new(cells:Array1<usize>,polarvector: (f64,f64,f64), polarorigin: (f64,f64,f64), limit: Dim)-> Self{
        print_debug!("Grid2D: Generating new grid");
        if cells.shape()[0] != 2 {
                panic!("Grid2D got wrong Arrayshape.\\
                    Array should only hold a single number.")
        }

        let (xlim, ylim) = match limit{
            Dim::TwoD(s,y)=> (s,y),
            _ => panic!("Grid2D got limits for other then two dimensions.")

        };
        if xlim.0 != 0.0{
            panic!("xlim for polar coordinates must be 0 any time")
        }
        print_debug!("find zell positions of polar 2D plane");
        let ycellsize = (ylim.1-ylim.0)/cells[1] as f64;
        let mut xpositions = Array::from_elem(cells[0], 0.);
        let mut ypositions = Array::from_elem(cells[1], ycellsize/2.);
        for cellidy in 0..cells[1]{
            ypositions[cellidy as usize ] = cellidy as f64 * ycellsize + ylim.0;
        }
        let cell_vol = ((ylim.1-ylim.0)* 2.0 * std::f64::consts::PI * xlim.1*xlim.1)/(cells[0]*cells[1]) as f64;
        let mut current_position = 0.0;
        for cellidx in 0..cells[0]{
            let xsize  = (cell_vol/(ycellsize * std::f64::consts::PI)+current_position*current_position).sqrt();
            xpositions[cellidx as usize ] = current_position + xsize/2.0;
            current_position+=xsize;
        }
        print_debug!(
            "Grid2D:\n\tCells: {:?} \n\txpositions: {:?}\\
             \n\typositions: {:?} \n\txlim: {:?} \n\tylim: {:?}",
            cells,xpositions,ypositions,xlim,ylim
        );

        Grid2DPolar{
            cells,
            xpositions,
            ypositions,
            xlim,
            ylim,
            polarvector,
            polarorigin
        }
    }

    pub fn data_array<T: Default+Clone>(&self)->Array2<T>{
        Array2::from_elem((self.cells[0] as usize,self.cells[1] as usize),T::default())
    }

    pub fn transfer_to_polar(self, position: Vec<f64>)-> polarcoordinates{
        // step 1 : Translate
        let px = position[0] - self.polarorigin.0;
        let py = position[1] - self.polarorigin.1;
        let newz = position[2] - self.polarorigin.2;
        //step 2 Rotate#
        let rotation_angle = (self.polarvector.0/self.polarvector.1).atan();
        let newx = px*rotation_angle.cos()+py*rotation_angle.sin();
        let newy = -px*rotation_angle.sin()+py*rotation_angle.cos();
        let r = (newx*newx+newy*newy).sqrt();
        let phi= newx.atan2(newy);
        (r, phi, newz)
    }
}

impl GridFunctions for Grid2DPolar{


    fn is_inside(&self, num: Vec<f64>)-> bool{
        print_debug!("Grid2D: Checking if {:?} is in grid", num);
        let posx = num[0];
        let posy = num[1];
        (posx -self.polarorigin.0).abs() < self.xlim.1 &&
        posy > self.ylim.0 && posy < self.ylim.1
    }

    fn cell_id(&self, num: Vec<f64>)-> Array1<usize>{
        let posx = (num[0] -self.polarorigin.0).abs();
        print_debug!("Checking array {:?} with position {:?}",&self.xpositions,posx);
        let cell_vol = ((self.ylim.1-self.ylim.0)* 2.0 * std::f64::consts::PI * self.xlim.1*self.xlim.1)/(self.cells[0]*self.cells[1]) as f64;
        let mut current_position = 0.0;
        let ycellsize = (self.ylim.1-self.ylim.0)/self.cells[1] as f64;
        let mut cell_idx = std::usize::MAX;
        for cellidx in 0..self.cells[0]{
            let xsize  = (cell_vol/(ycellsize * std::f64::consts::PI)+current_position*current_position).sqrt();
            if posx > current_position && posx < current_position + xsize{
                 cell_idx=cellidx;
            }
            current_position+=xsize;
        }
        if cell_idx > self.cells[0]{
            panic!("Particle is not in grid.")
        }
        print_debug!("Result: {}",cell_idx);
        let posy = num[1];
        print_debug!("Checking array {:?} with position {:?}",&self.ypositions,posy);
        let cell_idy = (&self.ypositions-posy)
                        .iter()
                        .map(|y| y.abs())
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
