extern crate ndarray;
use super::{CellId, GridFunctions3D, Position};
use crate::{grid, CylindricalGrid3D};
use derive_getters::Getters;
use ndarray::prelude::*;
use std::any::Any;

use anyhow::Result;
#[derive(Getters, Clone)]
pub struct VectorGrid {
    pub data: [Box<dyn grid::GridFunctions3D>; 3],
    cyl_grid: CylindricalGrid3D,
}

impl VectorGrid {
    pub fn new(grid: Box<dyn GridFunctions3D>) -> Self {
        let cyl_grid = if grid.is_cylindrical() {
            grid
                .as_any()
                .downcast_ref::<CylindricalGrid3D>()
                .unwrap()
                .clone()
        } else {
            CylindricalGrid3D::default()
        };
        VectorGrid {
            data: [grid.clone(), grid.clone(), grid.clone()],
            cyl_grid,
        }
    }

    pub fn velocity_calculation(&self, pos: Position, vel: Array1<f64>) -> [f64; 3] {
        let mut vel = [vel[0], vel[1], vel[2]];
        if !self.data[0].is_cylindrical() {
            // it is cartesian, so just return the velocity
            return [vel[0], vel[1], vel[2]];
        } else {
            // downcast a data grid to a cylindrical grid
            let cyl_pos = self.cyl_grid.to_cylindrical(pos);
            let vel_radial = (pos[0] * vel[0] + pos[1] * vel[1]) / cyl_pos[0];
            let vel_omega =
                (pos[0] * vel[1] - pos[1] * vel[0]) / (pos[0] * pos[0] + pos[1] * pos[1]);
            let vel_z = vel[2];
            vel = [vel_radial, vel_omega, vel_z];
        }
        vel
    }
}

impl std::fmt::Debug for VectorGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Grid3D: \n\tCells: {:?} \n\txlim: {:?} \
            \n\tylim: {:?} \n\tzlim: {:?}\n\tData information:\n\t\tMean\
            \n\t\tDim 1: {:?}\n\t\tDim 2: {:?}\n\t\tDim 3: {:?}",
            self.data[0].get_cells(),
            self.data[0].get_limits()[0],
            self.data[0].get_limits()[1],
            self.data[0].get_limits()[2],
            self.data[0]
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
            self.data[1]
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
            self.data[2]
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
        )
    }
}

impl std::fmt::Display for VectorGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Grid3D: \n\tCells: {:?} \n\txlim: {:?} \
            \n\tylim: {:?} \n\tzlim: {:?}\n\tData information:\n\t\tMean:\
            \n\t\tDim 1: {:?}\n\t\tDim 2: {:?}\n\t\tDim 3: {:?}",
            self.data[0].get_cells(),
            self.data[0].get_limits()[0],
            self.data[0].get_limits()[1],
            self.data[0].get_limits()[2],
            self.data[0]
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
            self.data[1]
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
            self.data[2]
                .get_data()
                .mean()
                .expect("Unable to calculate mean of data"),
        )
    }
}

impl GridFunctions3D for VectorGrid {
    fn get_value(&self, _pos: Position) -> f64 {
        unimplemented!()
    }

    // add to the value at the same position
    fn add_value(&mut self, pos: Position, value: f64) {
        self.data[0].add_value(pos, value);
        self.data[1].add_value(pos, value);
        self.data[2].add_value(pos, value);
    }

    // divide the whole array by another
    fn divide_by_array(&mut self, other: &Array3<f64>) {
        self.data[0].divide_by_array(other);
        self.data[1].divide_by_array(other);
        self.data[2].divide_by_array(other);
    }
    fn divide_by_scalar(&mut self, other: f64) {
        self.data[0].divide_by_scalar(other);
        self.data[1].divide_by_scalar(other);
        self.data[2].divide_by_scalar(other);
    }

    // divide by the weights
    fn divide_by_weight(&mut self) {
        self.data[0].divide_by_weight();
        self.data[1].divide_by_weight();
        self.data[2].divide_by_weight();
    }

    // insert value in cell at his position
    fn insert(&mut self, pos: Position, value: f64) {
        self.data[0].insert(pos, value);
        self.data[1].insert(pos, value);
        self.data[2].insert(pos, value);
    }

    // Check if particle/ number is inside the overall dimensions
    fn is_inside(&self, pos: Position) -> bool {
        self.data[0].is_inside(pos)
    }

    // Return cell ID of Data/Particle
    fn cell_id(&self, pos: Position) -> Result<CellId> {
        self.data[0].cell_id(pos)
    }

    fn cell_ids_in_trajectory(
        &self,
        pos1: Position,
        pos2: Position,
    ) -> Result<(Vec<CellId>, Vec<f64>)> {
        self.data[0].cell_ids_in_trajectory(pos1, pos2)
    }
    // Needed for python interface ( check that again, might be not needed)
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_to_cell(&mut self, cell_id: CellId, value: f64) {
        self.data[0].add_to_cell(cell_id, value);
        self.data[1].add_to_cell(cell_id, value);
        self.data[2].add_to_cell(cell_id, value);
    }

    fn add_trajectory_value(&mut self, pos1: Position, pos2: Position, value: f64) {
        self.data[0].add_trajectory_value(pos1, pos2, value);
        self.data[1].add_trajectory_value(pos1, pos2, value);
        self.data[2].add_trajectory_value(pos1, pos2, value);
    }
    // return a new instance of grid with zeros
    fn new_zeros(&self) -> Box<dyn GridFunctions3D> {
        Box::new(VectorGrid::new(self.data[0].new_zeros()))
    }

    fn collapse(&self, _axis: usize) -> Array2<f64> {
        unimplemented!()
    }

    fn collapse_two(&self, axis1: usize, axis2: usize) -> Array1<f64> {
        self.data[0].collapse_two(axis1, axis2)
    }

    fn collapse_two_weight(&self, axis1: usize, axis2: usize) -> Array1<f64> {
        self.data[0].collapse_two_weight(axis1, axis2)
    }

    fn collapse_weight(&self, axis: usize) -> Array2<f64> {
        self.data[0].collapse_weight(axis)
    }

    fn slice(&self, _axis: usize, _position: f64) -> Array2<f64> {
        unimplemented!()
    }

    fn slice_idx(&self, _axis: usize, _cell_id: usize) -> Array2<f64> {
        // find the length of theplane in each direction with triangulation
        unimplemented!()
    }
    //cellcenters

    // Need to write getters in here
    fn get_xpositions(&self) -> &Array1<f64> {
        self.data[0].get_xpositions()
    }
    fn get_ypositions(&self) -> &Array1<f64> {
        self.data[0].get_ypositions()
    }
    fn get_zpositions(&self) -> &Array1<f64> {
        self.data[0].get_zpositions()
    }
    fn get_limits(&self) -> &[[f64; 2]; 3] {
        self.data[0].get_limits()
    }
    fn get_cells(&self) -> &CellId {
        self.data[0].get_cells()
    }
    fn get_data(&self) -> &Array3<f64> {
        self.data[0].get_data()
    }
    fn get_weights(&self) -> &Array3<f64> {
        self.data[0].get_weights()
    }
    fn is_cylindrical(&self) -> bool {
        self.data[0].is_cylindrical()
    }

    #[allow(unused_variables)]
    fn set_data(&mut self, data: Array3<f64>) {
        unimplemented!("Not implemented for VectorGrid")
    }

    fn set_weights(&mut self, weights: Array3<f64>) {
        self.data[0].set_weights(weights.clone());
        self.data[1].set_weights(weights.clone());
        self.data[2].set_weights(weights);
    }

    fn outlier_removal(&mut self, threshold: f64, mode: usize) {
        self.data[0].outlier_removal(threshold, mode);
        self.data[1].outlier_removal(threshold, mode);
        self.data[2].outlier_removal(threshold, mode);
    }
}
