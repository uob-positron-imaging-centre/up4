use crate::datamanager::Manager;
use crate::grid::{self, *};
use crate::particleselector::ParticleSelector;
use itertools::Itertools;
use ndarray::prelude::*;

pub struct Comparer {
    data: Box<dyn Manager + Send>,
    grid: Box<dyn GridFunctions3D>,
    data2: Box<dyn Manager + Send>,
    grid2: Box<dyn GridFunctions3D>,
    aligned: bool,
}
pub trait Comparison {
    fn compare(
        &self,
        data: Box<dyn Manager + Send>,
        data2: Box<dyn Manager + Send>,
        grid: Box<dyn GridFunctions3D>,
    ) -> Comparer {
        Comparer {
            data: data,
            grid: grid.clone(),
            data2: data2,
            grid2: grid.clone(),
            aligned: false,
        }
    }
    fn align(&mut self);
}

impl Comparison for Comparer {
    fn align(&mut self) {
        let quantile_threshold = 0.01;
        let steps = 100;
        // we need to generate grids such that the data oerlaps perfect and we also may need to set rotation of datasets
        // first we need to find out which type grid is
        let gridtype;
        if self.grid.as_any().is::<CartesianGrid3D>() {
            gridtype = "cartesian";
        } else if self.grid.as_any().is::<CylindricalGrid3D>() {
            gridtype = "cylindrical";
        } else {
            panic!("Grid type not supported");
        }
        let selector = ParticleSelector::default();
        // make grid the first time, include all particles
        let stats1 = self.data.global_stats();
        let dim1 = stats1.dimensions();
        let size: [f64; 3] = [
            dim1[[1, 0]] - dim1[[0, 0]],
            dim1[[1, 1]] - dim1[[0, 1]],
            dim1[[1, 2]] - dim1[[0, 2]],
        ];
        let stats2 = self.data2.global_stats();
        let dim2 = stats2.dimensions();
        let size2: [f64; 3] = [
            dim2[[1, 0]] - dim2[[0, 0]],
            dim2[[1, 1]] - dim2[[0, 1]],
            dim2[[1, 2]] - dim2[[0, 2]],
        ];
        // check if the dimensions are roughly 800 times larger than the other, then the unit is different
        let set2_adjustment = if size[0] / size2[0] > 800.0 {
            1000.0
        } else if size2[0] / size[0] > 800.0 {
            1.0 / 1000.0
        } else {
            1.0
        };
        // move the grid a maximum of the half of the size of the grid in each dimension
        // Actually, first i need to make sure thta the cell sizes of each grid are exactly the same size,
        // Therefore i need to make sure that the grid is the same size in each dimension
        // which is done by defining the dimensions by the max dimensions
        let max_x = f64::max(size[0], size2[0] * set2_adjustment);
        let max_y = f64::max(size[1], size2[1] * set2_adjustment);
        let max_z = f64::max(size[2], size2[2] * set2_adjustment);
        let _system_size = [max_x, max_y, max_z];
        // now each individual dimension can be defined:
        let dim1 = Array2::from_shape_vec(
            (2, 3),
            vec![
                dim1[[0, 0]],
                dim1[[0, 1]],
                dim1[[0, 2]],
                dim1[[0, 0]] + max_x,
                dim1[[0, 1]] + max_y,
                dim1[[0, 2]] + max_z,
            ],
        )
        .unwrap();
        let dim2 = Array2::from_shape_vec(
            (2, 3),
            vec![
                dim2[[0, 0]],
                dim2[[0, 1]],
                dim2[[0, 2]],
                dim2[[0, 0]] + max_x * set2_adjustment,
                dim2[[0, 1]] + max_y * set2_adjustment,
                dim2[[0, 2]] + max_z * set2_adjustment,
            ],
        )
        .unwrap();

        let move_x = (size[0] / 2.0) / steps as f64;
        let move_y = (size[1] / 2.0) / steps as f64;
        let move_z = (size[2] / 2.0) / steps as f64;
        let mut old_diff = f64::INFINITY;
        let mut best_offset: [f64; 3] = [0.0, 0.0, 0.0];
        for xoffset in (-steps / 2..steps / 2).map(|x| x as f64 * move_x) {
            for yoffset in (-steps / 2..steps / 2).map(|x| x as f64 * move_y) {
                for zoffset in (-steps / 2..steps / 2).map(|x| x as f64 * move_z) {
                    // For each offset, keep the second mesh steady and move the first grid
                    let mut dim1_ = dim1.clone();
                    dim1_[[0, 0]] += xoffset;
                    dim1_[[0, 1]] += yoffset;
                    dim1_[[0, 2]] += zoffset;
                    dim1_[[1, 0]] += xoffset;
                    dim1_[[1, 1]] += yoffset;
                    dim1_[[1, 2]] += zoffset;

                    let grid1 = make_grid(&dim1_, gridtype, [100, 100, 100]);
                    let grid2 = make_grid(&dim2, gridtype, [100, 100, 100]);
                    // Generate a field with the number of particles in each cell
                    let field1 = self.data.numberfield(grid1, &selector);
                    let field2 = self.data2.numberfield(grid2, &selector);
                    // threshold the field at 1% low
                    let binary_field1 = binary_threshold(field1.get_data(), quantile_threshold);
                    let binary_field2 = binary_threshold(field2.get_data(), quantile_threshold);

                    let diff = (binary_field1 - binary_field2)
                        .mapv(|x| x.abs())
                        .iter()
                        .sum::<f64>();
                    if diff < old_diff {
                        old_diff = diff;
                        best_offset = [xoffset, yoffset, zoffset]
                    }
                    println!("{} {} {}", best_offset[0], best_offset[1], best_offset[2]);
                    println!("{}", diff);
                }
            }
        }
    }
}

fn binary_threshold(field: &Array3<f64>, quantile: f64) -> Array3<f64> {
    if quantile > 1.0 || quantile < 0.0 {
        panic!("Quantile must be between 0 and 1");
    }
    let threshold = field
        .iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .filter(|x| **x != 0.0)
        .nth((field.len() as f64 * quantile) as usize)
        .unwrap();
    field.mapv(|x| if x > *threshold { 1.0 } else { 0.0 })
}

fn make_grid(dim: &Array2<f64>, gridtype: &str, cells: [usize; 3]) -> Box<dyn GridFunctions3D> {
    let grid: Box<dyn GridFunctions3D>;
    if gridtype == "cartesian" {
        grid = Box::new(CartesianGrid3D::new(
            [cells[0], cells[1], cells[2]],
            grid::Dim::ThreeD([
                [dim[[0, 0]], dim[[1, 0]]],
                [dim[[0, 1]], dim[[1, 1]]],
                [dim[[0, 2]], dim[[1, 2]]],
            ]),
        ))
    } else if gridtype == "cylindrical" {
        grid = Box::new(CylindricalGrid3D::new(
            [cells[0], cells[1], cells[2]],
            grid::Dim::ThreeD([
                [dim[[0, 0]], dim[[1, 0]]],
                [dim[[0, 1]], dim[[1, 1]]],
                [dim[[0, 2]], dim[[1, 2]]],
            ]),
            "volume",
        ))
    } else {
        panic!("Grid type not supported");
    }
    return grid;
}

impl Comparer {
    fn velocityfield(&mut self) -> (Array1<f64>, Array1<f64>) {
        let field1 = self
            .data
            .velocityfield(self.grid.clone(), &ParticleSelector::default());
        let field2 = self
            .data2
            .velocityfield(self.grid2.clone(), &ParticleSelector::default());
        (
            field1.get_data().iter().map(|x| *x).collect(),
            field2.get_data().iter().map(|x| *x).collect(),
        )
    }
}
