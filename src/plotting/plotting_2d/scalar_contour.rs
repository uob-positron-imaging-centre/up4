use crate::GridFunctions3D;
use crate::VectorGrid;
use colorous::Gradient;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::Zip;
use ndarray_stats::QuantileExt;
use plotly::common::ColorScale;
use plotly::common::ColorScaleElement;
use plotly::Contour;

pub struct ScalarContour {
    x: Array1<f64>,
    y: Array1<f64>,
    data: Vec<Vec<f64>>,
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl ScalarContour {
    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> ScalarContour {
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
        let mut data_arr = Array2::zeros(u.dim());
        Zip::from(&mut data_arr)
            .and(&u)
            .and(&v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });
        let min = *data_arr.min_skipnan();
        let max = *data_arr.max_skipnan();
        let mut data = Vec::with_capacity(data_arr.dim().0);
        for col in data_arr.columns() {
            data.push(col.to_vec());
        }

        ScalarContour {
            x,
            y,
            data,
            min,
            max,
        }
    }

    pub fn from_vector_grid_single_plane(
        grid: VectorGrid,
        axis: usize,
        index: usize,
    ) -> ScalarContour {
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
        let mut data_arr = Array2::zeros(u.dim());
        Zip::from(&mut data_arr)
            .and(&u)
            .and(&v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });
        let min = *data_arr.min_skipnan();
        let max = *data_arr.max_skipnan();

        let mut data = Vec::with_capacity(data_arr.dim().0);
        for col in data_arr.columns() {
            data.push(col.to_vec());
        }

        ScalarContour {
            x,
            y,
            data,
            min,
            max,
        }
    }

    pub fn from_grid_depth_averaged(grid: Box<dyn GridFunctions3D>, axis: usize) -> ScalarContour {
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
        let data_arr = grid.collapse(axis);
        let min = *data_arr.min_skipnan();
        let max = *data_arr.max_skipnan();
        let mut data = Vec::with_capacity(data_arr.dim().0);
        for col in data_arr.columns() {
            data.push(col.to_vec());
        }

        ScalarContour {
            x,
            y,
            data,
            min,
            max,
        }
    }

    pub fn from_grid_single_plane(
        grid: Box<dyn GridFunctions3D>,
        axis: usize,
        index: usize,
    ) -> ScalarContour {
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
        let data_arr = grid
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let min = *data_arr.min_skipnan();
        let max = *data_arr.max_skipnan();

        let mut data = Vec::with_capacity(data_arr.dim().0);
        for col in data_arr.columns() {
            data.push(col.to_vec());
        }

        ScalarContour {
            x,
            y,
            data,
            min,
            max,
        }
    }

    pub fn create_scalar_contour(
        &self,
        colourmap: Option<Gradient>,
    ) -> Vec<Box<Contour<Vec<f64>>>> {
        let cmap = self.get_colour_map(colourmap);
        let contour = Contour::new(
            self.x.to_owned().into_raw_vec(),
            self.y.to_owned().into_raw_vec(),
            self.data.to_owned(),
        )
        .color_scale(ColorScale::Vector(cmap));
        let trace = vec![contour];

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
