extern crate ndarray;
use super::{CellId, Dim, GridFunctions3D, Position, ThreeD};
use crate::print_debug;
use derive_getters::Getters;
use itertools::Itertools;
use ndarray::{prelude::*, RemoveAxis};
use ndarray_stats::QuantileExt;
use std::any::Any;

#[derive(Getters, Clone)]
pub struct CartesianGrid3D {
    cells: CellId,
    xpositions: Array1<f64>,
    ypositions: Array1<f64>,
    zpositions: Array1<f64>,
    limits: ThreeD,
    data: Array3<f64>,
    weight: Array3<f64>,
    // attrs: HashMap<String, >,
}

impl CartesianGrid3D {
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
        CartesianGrid3D {
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

impl std::fmt::Debug for CartesianGrid3D {
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

impl std::fmt::Display for CartesianGrid3D {
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
impl GridFunctions3D for CartesianGrid3D {
    fn is_inside(&self, pos: Position) -> bool {
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);
        self.limits
            .iter()
            .zip(pos.iter())
            .map(|(lim, pos)| pos > &lim[0] && pos < &lim[1])
            .all(|value| value)
    }

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
    #[allow(dead_code, unused_variables, unreachable_code)]
    fn cell_ids_in_trajectory(&self, pos1: Position, pos2: Position) -> (Vec<CellId>, Vec<f64>) {
        unimplemented!("Not implemented for 3D grid yet.");
        // This code was written on a friday evening. so please look over it
        // might be improvable
        let cell_ids = Vec::new();
        let weights = Vec::new();
        // check if both points are inside the grid
        if !self.is_inside(pos1) || !self.is_inside(pos2) {
            return (cell_ids, weights);
        }
        let (init_cell, end_cell) = (self.cell_id(pos1), self.cell_id(pos2));
        if init_cell == end_cell {
            return (vec![init_cell], vec![1.]);
        }
        let mut vec_distances = Vec::new();
        let mut vec_cell_ids = Vec::new();
        for dim in 0..3 {
            let pos_start = pos1[dim];
            let pos_end = pos2[dim];
            let cell_start = init_cell[dim];
            let cell_end = end_cell[dim];
            let cell_size = (self.limits[dim][1] - self.limits[dim][0]) / self.cells[dim] as f64;
            let cells_traversed = (cell_end as f64 - cell_start as f64).abs() as usize;
            let direction = if cell_end > cell_start { 1 } else { -1 };
            // get a vec with locations where a cell is crossed
            let mut distance = Vec::new();
            let cell_positions = self.xpositions();
            distance.push(
                cell_size / 2.0 + direction as f64 * (pos_start - cell_positions[cell_start]),
            );
            for cell in 1..cells_traversed - 1 {
                distance.push(cell_size);
            }
            distance
                .push(cell_size / 2.0 + direction as f64 * (pos_end - cell_positions[cell_end]));
            vec_distances.push(distance);
            vec_cell_ids.push(cell_ids);
        }
        let x_distances = vec_distances[0].clone();
        let y_distances = vec_distances[1].clone();
        let z_distances = vec_distances[2].clone();
        let dx = pos2[0] - pos1[0];
        let dy = pos2[1] - pos1[1];
        let dz = pos2[2] - pos1[2];
        // transform distances to absolute distances and add them in a vector
        // use the gradient dx dy and dz to calculate the distance traveled
        let mut distances = Vec::new();
        for x in x_distances {
            let t = x / dx;
            distances.push((x * x + (dy * t) * (dy * t) + (dz * t) * (dz * t)).sqrt());
        }
        for y in y_distances {
            let t = y / dy;
            distances.push((y * y + (dx * t) * (dx * t) + (dz * t) * (dz * t)).sqrt());
        }
        for z in z_distances {
            let t = z / dz;
            distances.push((z * z + (dy * t) * (dy * t) + (dx * t) * (dx * t)).sqrt());
        }
        // sort the distances and the cell ids
        let mut distances = distances
            .iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .map(f64::clone)
            .collect::<Vec<f64>>();
        let whole_distance = distances.iter().sum::<f64>();
        let mut weights = distances
            .iter()
            .map(|x| x / whole_distance)
            .collect::<Vec<f64>>();
        let mut cell_ids = Vec::new();
        let size_x = (self.limits[0][1] - self.limits[0][0]) / self.cells[0] as f64;
        let size_y = (self.limits[1][1] - self.limits[1][0]) / self.cells[1] as f64;
        let size_z = (self.limits[2][1] - self.limits[2][0]) / self.cells[2] as f64;
        cell_ids.push(init_cell);
        for distance in distances {
            let cell_id = self.cell_id([
                pos1[0] + (distance + size_x / 2.) * dx,
                pos1[1] + (distance + size_y / 2.) * dy,
                pos1[2] + (distance + size_z / 2.) * dz,
            ]);
            cell_ids.push(cell_id);
        }
        (cell_ids, weights)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_to_cell(&mut self, cell_id: CellId, value: f64) {
        self.data[(cell_id[0], cell_id[1], cell_id[2])] += value;
        self.weight[(cell_id[0], cell_id[1], cell_id[2])] += 1.;
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

    fn add_trajectory_value(&mut self, pos1: Position, pos2: Position, value: f64) {
        // find all cells the particle has been in and their fraction of the whole distance
        let (cell_ids, fractions) = self.cell_ids_in_trajectory(pos1, pos2);
        // add the value to the cells
        for (cell_id, fraction) in cell_ids.iter().zip(fractions.iter()) {
            self.data[(cell_id[0], cell_id[1], cell_id[2])] += value * fraction;
            self.weight[(cell_id[0], cell_id[1], cell_id[2])] += fraction;
        }
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
            result_weight += &weight;
        }
        result /= &result_weight;
        result
    }

    fn slice(&self, axis: usize, position: f64) -> Array2<f64> {
        // find the length of theplane in each direction with triangulation
        let cell_id = ((position - self.limits[axis][0])
            / (self.limits[axis][1] - self.limits[axis][0])
            * self.cells[axis] as f64) as usize;

        if axis == 0 {
            self.data.slice(s![cell_id, .., ..]).to_owned()
        } else if axis == 1 {
            self.data.slice(s![.., cell_id, ..]).to_owned()
        } else if axis == 2 {
            self.data.slice(s![.., .., cell_id]).to_owned()
        } else {
            panic!("Cartesian Grid: Axis {:?} not supported in 3D array", axis);
        }
    }

    fn slice_idx(&self, axis: usize, cell_id: usize) -> Array2<f64> {
        // find the length of theplane in each direction with triangulation
        if axis == 0 {
            self.data.slice(s![cell_id, .., ..]).to_owned()
        } else if axis == 1 {
            self.data.slice(s![.., cell_id, ..]).to_owned()
        } else if axis == 2 {
            self.data.slice(s![.., .., cell_id]).to_owned()
        } else {
            panic!("Cartesian Grid: Axis {:?} not supported in 3D array", axis);
        }
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
    fn get_weights(&self) -> &Array3<f64> {
        &self.weight
    }
    fn is_cylindrical(&self) -> bool {
        false
    }

    fn set_data(&mut self, data: Array3<f64>) {
        if self.data.shape() == data.shape() {
            self.data = data;
        } else {
            panic!("Cartesian Grid: Data shape does not match grid shape");
        }
    }
    fn set_weights(&mut self, weights: Array3<f64>) {
        if self.weight.shape() == weights.shape() {
            self.weight = weights;
        } else {
            panic!("Cartesian Grid: Weight shape does not match grid shape");
        }
    }
}
