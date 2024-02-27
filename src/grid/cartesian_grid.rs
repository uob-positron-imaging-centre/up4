extern crate ndarray;
use super::{CellId, Dim, GridFunctions3D, Position, ThreeD};
use crate::{print_debug, print_warning};
use crate::utilities::{nan_mean, nan_std};
use derive_getters::Getters;
use itertools::Itertools;
use ndarray::{prelude::*, RemoveAxis};
use ndarray_stats::QuantileExt;
use std::any::Any;

use anyhow::{anyhow, Result};
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
            xpositions[cellidx] = cellidx as f64 * xcellsize + xcellsize / 2.0 + lim[0][0];
        }
        for cellidy in 0..cells[1] {
            ypositions[cellidy] = cellidy as f64 * ycellsize + ycellsize / 2.0 + lim[1][0];
        }
        for cellidz in 0..cells[2] {
            zpositions[cellidz] = cellidz as f64 * zcellsize + zcellsize / 2.0 + lim[2][0];
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
            nan_mean(&self.data),
            nan_std(&self.data),
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
            nan_mean(&self.data),
            nan_std(&self.data),
            self.data.min_skipnan(),
            self.data.max_skipnan()
        )
    }
}

impl std::ops::Index<CellId> for CartesianGrid3D {
    type Output = f64;
    fn index(&self, index: CellId) -> &Self::Output {
        &self.data[index]
    }
}
impl GridFunctions3D for CartesianGrid3D {
    fn is_inside(&self, pos: Position) -> bool {
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);
        if pos[0].is_nan() || pos[1].is_nan() || pos[2].is_nan() {
            return false;
        }
        self.limits
            .iter()
            .zip(pos.iter())
            .map(|(lim, pos)| pos >= &lim[0] && pos <= &lim[1])
            .all(|value| value)
    }

    fn cell_id(&self, pos: Position) -> Result<CellId> {
        print_debug!(
            "pos: {:?}, limits: {:?}, cells: {:?}",
            pos,
            self.limits,
            self.cells
        );
        // check if point is inside grid
        if !self.is_inside(pos) {
            return Err(anyhow!(
                "Position {:?} is outside of grid limits {:?}",
                pos,
                self.limits
            ));
        }
        let cell_idx = ((pos[0] - self.limits[0][0]) / (self.limits[0][1] - self.limits[0][0])
            * (self.cells[0] - 1) as f64) as usize;
        let cell_idy = ((pos[1] - self.limits[1][0]) / (self.limits[1][1] - self.limits[1][0])
            * (self.cells[1] - 1) as f64) as usize;
        let cell_idz = ((pos[2] - self.limits[2][0]) / (self.limits[2][1] - self.limits[2][0])
            * (self.cells[2] - 1) as f64) as usize;

        Ok([cell_idx, cell_idy, cell_idz])
    }
    #[allow(dead_code, unused_variables, unreachable_code)]
    fn cell_ids_in_trajectory(
        &self,
        pos1: Position,
        pos2: Position,
    ) -> Result<(Vec<CellId>, Vec<f64>)> {
        // TODO finish that algorithm!!
        let (init_cell, end_cell) = (self.cell_id(pos1)?, self.cell_id(pos2)?);
        return Ok((vec![init_cell, end_cell], vec![0.5, 0.5]));

        unimplemented!("Not implemented for 3D grid yet.");
        // This code was written on a friday evening. so please look over it
        // might be improvable
        let cell_ids: Vec<[f64; 3]> = Vec::new();
        let weights: Vec<f64> = Vec::new();

        let (init_cell, end_cell) = (self.cell_id(pos1).unwrap(), self.cell_id(pos2).unwrap());
        if init_cell == end_cell {
            return Ok((vec![init_cell], vec![1.]));
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
            let cell_positions = if dim == 0 {
                self.xpositions()
            } else if dim == 1 {
                self.ypositions()
            } else {
                self.zpositions()
            };
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
            let cell_id = match self.cell_id([
                pos1[0] + (distance + size_x / 2.) * dx,
                pos1[1] + (distance + size_y / 2.) * dy,
                pos1[2] + (distance + size_z / 2.) * dz,
            ]) {
                Ok(cell_id) => cell_id,
                Err(_) => {
                    println!("Error in cell_id calculation");
                    continue;
                }
            };
            cell_ids.push(cell_id);
        }
        Ok((cell_ids, weights))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_to_cell(&mut self, cell_id: CellId, value: f64) {
        self.data[(cell_id[0], cell_id[1], cell_id[2])] += value;
        self.weight[(cell_id[0], cell_id[1], cell_id[2])] += 1.;
    }
    fn get_value(&self, pos: Position) -> f64 {
        let cell_id = match self.cell_id(pos) {
            Ok(cell_id) => cell_id,
            Err(_) => return f64::NAN,
        };
        self.data[(cell_id[0], cell_id[1], cell_id[2])]
    }

    fn add_value(&mut self, pos: Position, value: f64) {
        let cell_id = match self.cell_id(pos) {
            Ok(cell_id) => cell_id,
            Err(_) => return,
        };
        // add bounds check
        if cell_id[0] >= self.cells[0] || cell_id[1] >= self.cells[1] || cell_id[2] >= self.cells[2]
        {
            println!(
                "Error: cell_id out of bounds: CellId: {:?} with grid of size: {:?}",
                cell_id,
                self.cells()
            );
            return;
        }
        self.data[(cell_id[0], cell_id[1], cell_id[2])] += value;
        self.weight[(cell_id[0], cell_id[1], cell_id[2])] += 1.;
    }

    // Between two points of a trajectory, find all cells that are crossed and the fraction of the
    // distance that is in each cell
    fn add_trajectory_value(&mut self, pos1: Position, pos2: Position, value: f64) {
        // find all cells the particle has been in and their fraction of the whole distance
        let (cell_ids, fractions) = match self.cell_ids_in_trajectory(pos1, pos2) {
            Ok((cell_ids, fractions)) => (cell_ids, fractions),
            Err(_) => {
                print_warning!("Error in cell_ids_in_trajectory calculation, cell id not found");
                return;
            }
        };
        // add the value to the cells
        for (cell_id, fraction) in cell_ids.iter().zip(fractions.iter()) {
            self.data[(cell_id[0], cell_id[1], cell_id[2])] += value * fraction;
            self.weight[(cell_id[0], cell_id[1], cell_id[2])] += fraction;
        }
    }

    fn divide_by_array(&mut self, other: &Array3<f64>) {
        self.data = &self.data / other;
    }

    fn divide_by_scalar(&mut self, other: f64) {
        self.data = &self.data / other;
    }

    fn divide_by_weight(&mut self) {
        self.data = &self.data / &self.weight;
    }

    fn insert(&mut self, pos: Position, value: f64) {
        let cell_id = match self.cell_id(pos) {
            Ok(cell_id) => cell_id,
            Err(_) => return,
        };
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

    fn collapse_weight(&self, axis: usize) -> Array2<f64> {
        //check for Nans and     Infs and replace with 0
        let axis = Axis(axis);
        let mut result_weight: Array2<f64> = Array::zeros(self.data.raw_dim().remove_axis(axis));

        for weight in self.weight.axis_iter(axis) {
            result_weight += &weight;
        }
        result_weight
    }

    fn collapse_two(&self, axis1: usize, axis2: usize) -> Array1<f64> {
        //check for Nans and     Infs and replace with 0
        if axis1 == axis2 {
            panic!("axis1 and axis2 must be different");
        }
        let mut axis1 = axis1;
        let mut axis2 = axis2;
        if axis1 > axis2 {
            // swap axis1 and axis2
            std::mem::swap(&mut axis1, &mut axis2);
        }
        let axis1 = Axis(axis1);
        let axis2 = Axis(axis2 - 1); // we removed axis1 so axis2 is now one smaller
        let first_collaps = self.collapse(axis1.index());
        let first_collaps_weight = self.collapse_weight(axis1.index());
        let mut result: Array1<f64> = Array::zeros(first_collaps.raw_dim().remove_axis(axis2));
        let mut result_weight: Array1<f64> =
            Array::zeros(first_collaps.raw_dim().remove_axis(axis2));

        for (data_arr, weight) in first_collaps
            .axis_iter(axis2)
            .zip(first_collaps_weight.axis_iter(axis2))
        {
            // check for nans
            let data_arr = data_arr.mapv(|x| if x.is_nan() { 0. } else { x });
            result = result + &data_arr * &weight;
            result_weight += &weight;
        }
        result /= &result_weight;
        result
    }

    fn collapse_two_weight(&self, axis1: usize, axis2: usize) -> Array1<f64> {
        //check for Nans and     Infs and replace with 0
        if axis1 == axis2 {
            panic!("axis1 and axis2 must be different");
        }
        let mut axis1 = axis1;
        let mut axis2 = axis2;
        if axis1 > axis2 {
            // swap axis1 and axis2
            std::mem::swap(&mut axis1, &mut axis2);
        }
        let axis1 = Axis(axis1);
        let axis2 = Axis(axis2 - 1);
        let first_collaps_weight = self.collapse_weight(axis1.index());
        let mut result_weight: Array1<f64> =
            Array::zeros(first_collaps_weight.raw_dim().remove_axis(axis2));

        for weight in first_collaps_weight.axis_iter(axis2) {
            result_weight += &weight;
        }
        result_weight
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
    // mode 0: set all values above threshold to zero, mode 1: set all values above threshold to threshold mode 2: set all values above threshold to mean of surrounding all values
    fn outlier_removal(&mut self, threshold: f64, mode: usize) {
        if mode == 0 {
            self.data = self.data.mapv(|x| if x > threshold { 0. } else { x });
        } else if mode == 1 {
            self.data = self
                .data
                .mapv(|x| if x > threshold { threshold } else { x });
        } else if mode == 2 {
            let mut result: Array3<f64> = Array::zeros(self.data.raw_dim());
            let mut weight: Array3<f64> = Array::zeros(self.data.raw_dim());
            for (idx, data) in self.data.indexed_iter() {
                if *data > threshold {
                    let mut sum: f64 = 0.;
                    let mut weight_sum: f64 = 0.;
                    let mut counter_sum: f64 = 0.;
                    // now check every surrounding index, only one cell away from idx
                    // 26 idx must be checked
                    // yes.. i know what you are thinking
                    let indexes = [
                        (idx.0 - 1, idx.1 - 1, idx.2 - 1),
                        (idx.0 - 1, idx.1 - 1, idx.2),
                        (idx.0 - 1, idx.1 - 1, idx.2 + 1),
                        (idx.0 - 1, idx.1, idx.2 - 1),
                        (idx.0 - 1, idx.1, idx.2),
                        (idx.0 - 1, idx.1, idx.2 + 1),
                        (idx.0 - 1, idx.1 + 1, idx.2 - 1),
                        (idx.0 - 1, idx.1 + 1, idx.2),
                        (idx.0 - 1, idx.1 + 1, idx.2 + 1),
                        (idx.0, idx.1 - 1, idx.2 - 1),
                        (idx.0, idx.1 - 1, idx.2),
                        (idx.0, idx.1 - 1, idx.2 + 1),
                        (idx.0, idx.1, idx.2 - 1),
                        (idx.0, idx.1, idx.2 + 1),
                        (idx.0, idx.1 + 1, idx.2 - 1),
                        (idx.0, idx.1 + 1, idx.2),
                        (idx.0, idx.1 + 1, idx.2 + 1),
                        (idx.0 + 1, idx.1 - 1, idx.2 - 1),
                        (idx.0 + 1, idx.1 - 1, idx.2),
                        (idx.0 + 1, idx.1 - 1, idx.2 + 1),
                        (idx.0 + 1, idx.1, idx.2 - 1),
                        (idx.0 + 1, idx.1, idx.2),
                        (idx.0 + 1, idx.1, idx.2 + 1),
                        (idx.0 + 1, idx.1 + 1, idx.2 - 1),
                        (idx.0 + 1, idx.1 + 1, idx.2),
                        (idx.0 + 1, idx.1 + 1, idx.2 + 1),
                    ];

                    for index in &indexes {
                        if index.0 < self.data.shape()[0]
                            && index.1 < self.data.shape()[1]
                            && index.2 < self.data.shape()[2]
                        {
                            if self.data[[index.0, index.1, index.2]] > threshold {
                                continue;
                            }
                            sum += self.data[[index.0, index.1, index.2]];
                            weight_sum += self.weight[[index.0, index.1, index.2]];
                            counter_sum += 1.0;
                        }
                    }
                    result[idx] = sum / counter_sum;
                    weight[idx] = weight_sum / counter_sum;
                } else {
                    result[idx] = *data;
                    weight[idx] = self.weight[idx];
                }
            }
            self.data = result;
            self.weight = weight;
        } else {
            panic!(
                "Cartesian Grid: Mode {:?} not supported. Supported modes are:\n \
            0: set outliers to zero \n\
            1: set outliers to threshold \n\
            2: set outlier to mean of surrounding all cells
            ",
                mode
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_data() {
        let limit = Dim::ThreeD([[-2.0, 2.0], [-2.0, 2.0], [-2.0, 2.0]]);
        let n = 10;
        let cells = [n; 3];
        let mut grid = CartesianGrid3D::new(cells, limit);
        let data = Array3::<f64>::zeros((n, n, n));
        grid.set_data(data.clone());
        assert_eq!(grid.data, data);
    }

    #[test]
    fn test_set_weights() {
        let limit = Dim::ThreeD([[-2.0, 2.0], [-2.0, 2.0], [-2.0, 2.0]]);
        let n = 10;
        let cells = [n; 3];
        let mut grid = CartesianGrid3D::new(cells, limit);
        let weights = Array3::<f64>::zeros((n, n, n));
        grid.set_weights(weights.clone());
        assert_eq!(grid.weight, weights);
    }

    // TODO test outlier removal
}
