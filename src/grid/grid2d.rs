extern crate ndarray;
use crate::print_debug;
use derive_getters::Getters;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use std::any::Any;

use super::{Dim, GridFunctions, TwoD};
#[derive(Getters, Clone)]
pub struct Grid2D {
    cells: Array1<usize>,
    xpositions: Array1<f64>,
    ypositions: Array1<f64>,
    limits: TwoD,
    // attrs: HashMap<String, >,
}

impl Grid2D {
    pub fn new(cells: Array1<usize>, limit: Dim) -> Self {
        print_debug!("Grid2D: Generating new grid");
        if cells.shape()[0] != 2 {
            panic!(
                "Grid2D got wrong Arrayshape.\\
                    Array should only hold a single number."
            )
        }

        let lim = match limit {
            Dim::TwoD(s) => (s),
            _ => panic!("Grid2D got limits for other then two dimensions."),
        };
        let xcellsize = (lim[0][1] - lim[0][0]) / cells[0] as f64;
        let ycellsize = (lim[1][1] - lim[1][0]) / cells[1] as f64;
        let mut xpositions = Array::from_elem(cells[0], 0.);
        let mut ypositions = Array::from_elem(cells[1], 0.);
        for cellidx in 0..cells[0] {
            xpositions[cellidx as usize] = cellidx as f64 * xcellsize + lim[0][0];
        }
        for cellidy in 0..cells[1] {
            ypositions[cellidy as usize] = cellidy as f64 * ycellsize + lim[1][0];
        }
        print_debug!(
            "Grid2D:\n\tCells: {:?} \n\txpositions: {:?}\\
             \n\typositions: {:?} \n\txlim: {:?} \n\tylim: {:?}",
            cells,
            xpositions,
            ypositions,
            xlim,
            ylim
        );

        Grid2D {
            cells,
            xpositions,
            ypositions,
            limits: lim,
        }
    }

    pub fn data_array<T: Default + Clone>(&self) -> Array2<T> {
        Array2::from_elem(
            (self.cells[0] as usize, self.cells[1] as usize),
            T::default(),
        )
    }
}

impl GridFunctions for Grid2D {
    fn is_inside(&self, num: Vec<f64>) -> bool {
        print_debug!("Grid2D: Checking if {:?} is in grid", num);
        let posx = num[0];
        let posy = num[1];
        posx > self.limits[0][0]
            && posx < self.limits[0][1]
            && posy > self.limits[1][0]
            && posy < self.limits[1][1]
    }

    fn cell_id(&self, num: Vec<f64>) -> Array1<usize> {
        let posx = num[0];
        print_debug!(
            "Checking array {:?} with position {:?}",
            &self.xpositions,
            posx
        );
        let cell_idx = (&self.xpositions - posx)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri2D", num));
        print_debug!("Result: {}", cell_idx);
        let posy = num[1];
        print_debug!(
            "Checking array {:?} with position {:?}",
            &self.ypositions,
            posy
        );
        let cell_idy = (&self.ypositions - posy)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri2D", num));
        print_debug!("Result: {}", cell_idy);
        array![cell_idx, cell_idy]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
