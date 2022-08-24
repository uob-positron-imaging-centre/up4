extern crate ndarray;
use super::{CellId, Dim, GridFunctions3D, Position, ThreeD};
use crate::{print_debug, print_warning};
use derive_getters::Getters;
use ndarray::{prelude::*, RemoveAxis};
use ndarray_stats::QuantileExt;
use std::any::Any;
use std::cell::Cell;
use std::f32::consts::PI;
use std::ops::{Add, DivAssign, Sub};
// TODO
// - Currently the cylinder is always vertical, but this should be configurable.
#[derive(Getters, Clone)]
pub struct CylindricalGrid3D {
    cells: CellId,
    rpositions: Array1<[f64; 2]>, // radius min and max for cell, needed because non equal distance
    rmeanpositions: Array1<f64>,  // midpoint radius for cell
    opositions: Array1<f64>,      // omega (angle)
    zpositions: Array1<f64>,      // height
    center: [f64; 3],
    radius: f64,
    limits: ThreeD,
    data: Array3<f64>,
    weight: Array3<f64>,
    // attrs: HashMap<String, >,
}

impl CylindricalGrid3D {
    pub fn new(cells: [usize; 3], limit: Dim, mode: &str) -> Self {
        if mode != "volume" {
            panic!("CylindricalGrid3D currently only supports \"volume\" mode.");
        }
        print_debug!("Grid3D: Generating new grid");

        let lim = match limit {
            Dim::ThreeD(x) => x,
            _ => panic!("Grid3D got limits for other then three dimensions."),
        };
        // the distance beween two angles is constant
        let ocellsize = (2.0 * 3.14159) / cells[1] as f64;
        // height distance is also easy to calculate
        let zcellsize = (lim[2][1] - lim[2][0]) / cells[2] as f64;
        // The radial positions depend on the mode
        let mut rpositions = Array::from_elem(cells[0], [0.0, 0.0]);
        let mut opositions = Array::from_elem(cells[1], 0.);
        let mut zpositions = Array::from_elem(cells[2], 0.);

        // positions contain the boundary of the cell
        for cellidy in 0..cells[1] {
            opositions[cellidy as usize] = -3.14159 + cellidy as f64 * ocellsize + ocellsize;
        }

        for cellidz in 0..cells[2] {
            zpositions[cellidz as usize] = cellidz as f64 * zcellsize + zcellsize + lim[2][0];
        }
        // center in cartesian coords!!!
        let center = [
            (lim[0][0] + lim[0][1]) / 2.0,
            (lim[1][0] + lim[1][1]) / 2.0,
            (lim[2][0] + lim[2][1]) / 2.0,
        ];
        let outer_radius = {
            let x = if (lim[0][1] - center[0]).abs() > (lim[0][0] - center[0]).abs() {
                (lim[0][1] - center[0]).abs()
            } else {
                (lim[0][0] - center[0]).abs()
            };
            let y = if (lim[1][1] - center[1]).abs() > (lim[1][0] - center[1]).abs() {
                (lim[1][1] - center[1]).abs()
            } else {
                (lim[1][0] - center[1]).abs()
            };
            (x.powf(2.) + y.powf(2.)).sqrt()
        };

        let mut inner_radius = 0.0;
        for cellidx in 0..cells[0] {
            if mode == "volume" {
                // radial positions should be in a distance so that all cells have the same volume
                // volume of a cell is pi*h*(r_o**2-r_i**2)*alpha/360
                let new = (outer_radius * outer_radius / cells[0] as f64
                    + inner_radius * inner_radius)
                    .sqrt();
                rpositions[cellidx as usize] = [inner_radius, new];
                inner_radius = new;
            }
        }

        let lim = [
            [0.0, outer_radius],    // radius
            [-3.14159, 3.14159],    // omega
            [lim[2][0], lim[2][1]], // height4
        ];
        let rmeanpositions = rpositions
            .iter()
            .map(|x| (x[0] + x[1]) / 2.)
            .collect::<Array1<f64>>();
        CylindricalGrid3D {
            cells,
            rpositions,
            rmeanpositions,
            opositions,
            zpositions,
            center,
            radius: outer_radius,
            limits: lim,
            data: Array3::zeros(cells),
            weight: Array3::zeros(cells),
        }
    }
    pub fn to_cylindrical(&self, pos: Position) -> Position {
        let pos = [pos[0] - self.center[0], pos[1] - self.center[1], pos[2]];
        let r = (pos[0] * pos[0] + pos[1] * pos[1]).sqrt();
        let theta = pos[1].atan2(pos[0]);
        let z = pos[2];
        let new_pos = [r, theta, z];
        print_debug!("Cart to cyl: {:?}-->{:?}", pos, new_pos);
        new_pos
    }
}

impl std::fmt::Debug for CylindricalGrid3D {
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

impl std::fmt::Display for CylindricalGrid3D {
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
impl GridFunctions3D for CylindricalGrid3D {
    fn is_inside(&self, pos: Position) -> bool {
        let pos = self.to_cylindrical(pos);
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);
        let in_rad = pos[0] <= self.radius;
        let in_omega = pos[1] >= self.limits[1][0] && pos[1] <= self.limits[1][1];
        let in_height = pos[2] >= self.limits[2][0] && pos[2] <= self.limits[2][1];
        in_rad && in_omega && in_height
    }

    fn cell_id(&self, pos: Position) -> CellId {
        print_debug!("Grid3D: Checking if {:?} is in grid", pos);
        let pos = &self.to_cylindrical(pos);
        let posr = pos[0];
        let mut cell_idr = None;
        for (idx, radius) in self.rpositions.iter().enumerate() {
            if posr >= radius[0] && posr <= radius[1] {
                cell_idr = Some(idx)
            }
        }
        let cell_idr = cell_idr.expect("Unable to find radial cell id");
        let poso = pos[1];
        let cell_ido = (&self.opositions - poso)
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
        [cell_idr, cell_ido, cell_idz]
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
            let weight = weight.mapv(|x| if x.is_nan() { 0. } else { x });
            result = result + &data_arr * &weight;
            result_weight = result_weight + &weight;
        }
        result = result / &result_weight;
        result
    }

    fn get_xpositions(&self) -> &Array1<f64> {
        print_warning!("CylindricalGrid3D: get_xpositions is returning midpoint of cell!");
        &self.rmeanpositions
    }
    fn get_ypositions(&self) -> &Array1<f64> {
        &self.opositions
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
