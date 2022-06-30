extern crate ndarray;
use crate::print_debug;
use derive_getters::Getters;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use std::any::Any;

use super::{Dim, GridFunctions, ThreeD};

#[derive(Getters, Clone)]
pub struct Grid3D {
    cells: Array1<usize>,
    xpositions: Array1<f64>,
    ypositions: Array1<f64>,
    zpositions: Array1<f64>,
    limits: ThreeD,
    // attrs: HashMap<String, >,
}

impl Grid3D {
    pub fn new(cells: Array1<usize>, limit: Dim) -> Self {
        print_debug!("Grid3D: Generating new grid");
        if cells.shape()[0] != 3 {
            panic!(
                "Grid1D got wrong Arrayshape.\\
                    Array should only hold a single number."
            )
        }

        let lim = match limit {
            Dim::ThreeD(x) => x,
            _ => panic!("Grid3D got limits for other then three dimensions."),
        };
        let xcellsize = (lim[0][1] - lim[0][0]) / cells[0] as f64;
        let ycellsize = (lim[1][1] - lim[1][0]) / cells[1] as f64;
        let zcellsize = (lim[2][1] - lim[2][0]) / cells[2] as f64;
        let mut xpositions = Array::from_elem(cells[0], 0.);
        let mut ypositions = Array::from_elem(cells[1], 0.);
        let mut zpositions = Array::from_elem(cells[1], 0.);
        for cellidx in 0..cells[0] {
            xpositions[cellidx as usize] = cellidx as f64 * xcellsize + xcellsize;
        }
        for cellidy in 0..cells[1] {
            ypositions[cellidy as usize] = cellidy as f64 * ycellsize + ycellsize;
        }
        for cellidz in 0..cells[2] {
            zpositions[cellidz as usize] = cellidz as f64 * zcellsize + zcellsize;
        }
        print_debug!(
            "Grid3D:\n\tCells: {:?} \n\txpositions: {:?} \n\t\\
            ypositions: {:?} \n\tzpositions: {:?} \n\txlim: {:?} \\
            \n\tylim: {:?} \n\tzlim: {:?}",
            cells,
            xpositions,
            ypositions,
            zpositions,
            xlim,
            ylim,
            zlim,
        );

        Grid3D {
            cells,
            xpositions,
            ypositions,
            zpositions,
            limits: lim,
        }
    }
    /*
    pub fn into_py(&self) -> PyGrid {
        PyGrid {
            grid: Box::new(self.clone()),
        }
    }*/
}

impl GridFunctions for Grid3D {
    fn is_inside(&self, particle_position: Vec<f64>) -> bool {
        print_debug!("Grid3D: Checking if {:?} is in grid", num);
        self.limits
            .iter()
            .zip(particle_position.iter())
            .map(|(lim, pos)| pos > &lim[0] && pos < &lim[1])
            .all(|value| value)
    }

    fn cell_id(&self, num: Vec<f64>) -> Array1<usize> {
        print_debug!("Grid3D: Checking if {:?} is in grid", num);
        let posx = num[0];
        let cell_idx = (&self.xpositions - posx)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri3D", num));
        let posy = num[1];
        let cell_idy = (&self.ypositions - posy)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri3D", num));
        let posz = num[2];
        let cell_idz = (&self.zpositions - posz)
            .iter()
            .map(|z| z.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri3D", num));
        array![cell_idx, cell_idy, cell_idz]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
