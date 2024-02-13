use crate::utilities::maths::meshgrid;
use crate::GridFunctions3D;
use crate::VectorGrid;
use colorous::Gradient;
use derive_getters::Getters;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::Zip;
use plotly::common::ColorScale;
use plotly::common::ColorScaleElement;
use plotly::HeatMap;

#[derive(Getters, Clone)]
pub struct ScalarMap {
    x: Array1<f64>,
    y: Array1<f64>,
    data: Array2<f64>,
}
// TODO add a note about plotly and needing to use transpose to handle the axes properly
impl ScalarMap {
    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let u = grid.data[i].collapse(axis);
        let v = grid.data[j].collapse(axis);
        let mut data = Array2::zeros(u.dim());
        Zip::from(&mut data).and(&u).and(&v).for_each(|n, &u, &v| {
            *n = f64::hypot(u, v);
        });

        ScalarMap { x, y, data }
    }

    pub fn from_vector_grid_single_plane(grid: VectorGrid, axis: usize, index: usize) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let u = grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let v = grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut data = Array2::zeros(u.dim());
        Zip::from(&mut data).and(&u).and(&v).for_each(|n, &u, &v| {
            *n = f64::hypot(u, v);
        });

        ScalarMap { x, y, data }
    }

    pub fn from_grid_depth_averaged(grid: Box<dyn GridFunctions3D>, axis: usize) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let data = grid.collapse(axis);

        ScalarMap { x, y, data }
    }

    pub fn from_grid_single_plane(
        grid: Box<dyn GridFunctions3D>,
        axis: usize,
        index: usize,
    ) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let data = grid.slice_idx(axis, index);
        ScalarMap { x, y, data }
    }

    pub fn create_scalar_map(
        &self,
        colourmap: Option<Gradient>,
    ) -> Vec<Box<HeatMap<f64, f64, f64>>> {
        let cmap = self.get_colour_map(colourmap);

        let (x, y) = meshgrid(self.x(), self.y());
        let heatmap = HeatMap::new(
            x.into_raw_vec(),
            y.into_raw_vec(),
            self.data.to_owned().into_raw_vec(),
        ).color_scale(ColorScale::Vector(cmap));
        let trace = vec![heatmap];
        trace
    }

    fn get_colour_map(&self, colourmap: Option<Gradient>) -> Vec<ColorScaleElement> {
        let n = 10;
        let mut colour_vector = Vec::with_capacity(n);
        let gradient = match colourmap {
            Some(colourmap) => colourmap,
            None => colorous::PLASMA,
        };
        for i in 0..=n {
            let frac = (i as f64) / (n as f64);
            let element = ColorScaleElement(frac, format!("#{:x}", gradient.eval_continuous(frac)));
            colour_vector.push(element);
        }

        colour_vector
    }
}
