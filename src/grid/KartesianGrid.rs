extern crate ndarray;
use super::{CellId, Dim, GridFunctions3D, Position, ThreeD};
use crate::{print_debug, print_warning};
use derive_getters::Getters;
use ndarray::{prelude::*, RemoveAxis};
use ndarray_stats::QuantileExt;
use std::any::Any;
use std::cell::Cell;
use std::ops::{Add, DivAssign, Sub};

#[derive(Getters, Clone)]
pub struct KartesianGrid3D {
    cells: CellId,
    xpositions: Array1<f64>,
    ypositions: Array1<f64>,
    zpositions: Array1<f64>,
    limits: ThreeD,
    data: Array3<f64>,
    weight: Array3<f64>,
    // attrs: HashMap<String, >,
}

impl KartesianGrid3D {
    pub fn new(cells: [usize; 3], limit: Dim) -> Self {
        print_debug!("Grid3D: Generating new grid");

        let lim = match limit {
            Dim::ThreeD(x) => x,
            _ => panic!("Grid3D got limits for other then three dimensions."),
        };
        let xcellsize = (lim[0][1] - lim[0][0]) / cells[0] as f64;
        let ycellsize = (lim[1][1] - lim[1][0]) / cells[1] as f64;
        let zcellsize = (lim[2][1] - lim[2][0]) / cells[2] as f64;
        let mut xpositions = Array::from_elem(cells[0], 0.);
        let mut ypositions = Array::from_elem(cells[1], 0.);
        let mut zpositions = Array::from_elem(cells[2], 0.);
        for cellidx in 0..cells[0] {
            xpositions[cellidx as usize] = cellidx as f64 * xcellsize + xcellsize / 2.0 + lim[0][0];
        }
        for cellidy in 0..cells[1] {
            ypositions[cellidy as usize] = cellidy as f64 * ycellsize + ycellsize / 2.0 + lim[1][0];
        }
        for cellidz in 0..cells[2] {
            zpositions[cellidz as usize] = cellidz as f64 * zcellsize + zcellsize / 2.0 + lim[2][0];
        }
        KartesianGrid3D {
            cells,
            xpositions,
            ypositions,
            zpositions,
            limits: lim,
            data: Array3::zeros(cells),
            weight: Array3::zeros(cells),
        }
    }
    /*
    pub fn into_py(&self) -> PyGrid {
        PyGrid {
            grid: Box::new(self.clone()),
        }
    }*/
}

impl std::fmt::Debug for KartesianGrid3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Grid3D: \n\tCells: {:?} \n\txlim: {:?} \
            \n\tylim: {:?} \n\tzlim: {:?}\n\tData information:\n\t\tMean: {:?}\
            \n\t\tStd: {:?}\n\t\tMin: {:?}\n\t\tMax: {:?}",
            self.cells,
            self.limits[0],
            self.limits[1],
            self.limits[2],
            self.data.mean().expect("Unable to calculate mean of data"),
            self.data.std(1.),
            self.data.min_skipnan(),
            self.data.max_skipnan()
        )
    }
}

impl std::fmt::Display for KartesianGrid3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Grid3D: \n\tCells: {:?} \n\txlim: {:?} \
            \n\tylim: {:?} \n\tzlim: {:?}\n\tData information:\n\t\tMean: {:?}\
            \n\t\tStd: {:?}\n\t\tMin: {:?}\n\t\tMax: {:?}",
            self.cells,
            self.limits[0],
            self.limits[1],
            self.limits[2],
            self.data.mean().expect("Unable to calculate mean of data"),
            self.data.std(1.),
            self.data.min_skipnan(),
            self.data.max_skipnan()
        )
    }
}
impl GridFunctions3D for KartesianGrid3D {
    fn is_inside(&self, pos: Position) -> bool {
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);
        self.limits
            .iter()
            .zip(pos.iter())
            .map(|(lim, pos)| pos > &lim[0] && pos < &lim[1])
            .all(|value| value)
    }
    /*
    fn cell_id(&self, pos: Position) -> CellId {
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);
        let posx = pos[0];
        let cell_idx = (&self.xpositions - posx)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri3D", pos));
        let posy = pos[1];
        let cell_idy = (&self.ypositions - posy)
            .iter()
            .map(|x| x.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri3D", pos));
        let posz = pos[2];
        let cell_idz = (&self.zpositions - posz)
            .iter()
            .map(|z| z.abs())
            .collect::<Array1<f64>>()
            .argmin()
            .expect(&format!("Can not find min of {:?} in Gri3D", pos));
        [cell_idx, cell_idy, cell_idz]
    }*/

    fn cell_id(&self, pos: Position) -> CellId {
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);

        let cell_idx = ((pos[0] - self.limits[0][0]) / (self.limits[0][1] - self.limits[0][0])
            * self.cells[0] as f64) as usize;
        let cell_idy = ((pos[1] - self.limits[1][0]) / (self.limits[1][1] - self.limits[1][0])
            * self.cells[1] as f64) as usize;
        let cell_idz = ((pos[2] - self.limits[2][0]) / (self.limits[2][1] - self.limits[2][0])
            * self.cells[2] as f64) as usize;

        [cell_idx, cell_idy, cell_idz]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_value(&self, pos: Position) -> f64 {
        let cell_id = self.cell_id(pos);
        self.data[(cell_id[0], cell_id[1], cell_id[2])]
    }

    fn add_value(&mut self, pos: Position, value: f64) {
        let cell_id = self.cell_id(pos);
        self.data[(cell_id[0], cell_id[1], cell_id[2])] += value;
        self.weight[(cell_id[0], cell_id[1], cell_id[2])] += 1.;
    }

    fn divide_by(&mut self, other: &Array3<f64>) {
        self.data = &self.data / other;
    }
    fn divide_by_weight(&mut self) {
        self.data = &self.data / &self.weight;
    }

    fn insert(&mut self, pos: Position, value: f64) {
        let cell_id = self.cell_id(pos);
        self.data[(cell_id[0], cell_id[1], cell_id[2])] = value;
    }
    fn new_zeros(&self) -> Box<dyn GridFunctions3D> {
        let mut grid = self.clone();
        grid.data = Array::zeros(self.cells);
        Box::new(grid)
    }

    fn collapse(&self, axis: usize) -> Array2<f64> {
        //check for Nans and     Infs and replace with 0
        let axis = Axis(axis);
        let mut result: Array2<f64> = Array::zeros(self.data.raw_dim().remove_axis(axis));
        let mut result_weight: Array2<f64> = Array::zeros(self.data.raw_dim().remove_axis(axis));

        for (data_arr, weight) in self.data.axis_iter(axis).zip(self.weight.axis_iter(axis)) {
            // check for nans
            let data_arr = data_arr.mapv(|x| if x.is_nan() { 0. } else { x });
            result = result + &data_arr * &weight;
            result_weight = result_weight + &weight;
        }
        result = result / &result_weight;
        result
    }

    fn get_xpositions(&self) -> &Array1<f64> {
        &self.xpositions
    }
    fn get_ypositions(&self) -> &Array1<f64> {
        &self.ypositions
    }
    fn get_zpositions(&self) -> &Array1<f64> {
        &self.zpositions
    }
    fn get_limits(&self) -> &ThreeD {
        &self.limits
    }
    fn get_cells(&self) -> &CellId {
        &self.cells
    }
    fn get_data(&self) -> &Array3<f64> {
        &self.data
    }
}
