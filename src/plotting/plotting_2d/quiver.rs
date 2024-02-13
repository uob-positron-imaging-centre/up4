use crate::plotting::plotting_2d::Arrow;
use crate::utilities::maths::meshgrid;
use crate::{grid::VectorGrid, GridFunctions3D};
use colorous::Gradient;
use derive_getters::Getters;
use itertools::izip;
use ndarray::{Array1, Array2, Zip};
use ndarray_stats::QuantileExt;
use plotly::common::{
    ColorBar, ColorScale, ColorScaleElement, Fill, Line, Marker, Mode,
};
use plotly::{Scatter, Trace};
use std::f64::consts::PI;
#[derive(Getters, Clone, Debug)]
pub struct QuiverPlot {
    x: Array1<f64>,
    y: Array1<f64>,
    u: Array2<f64>,
    v: Array2<f64>,
    norm: Array2<f64>,
    true_norm: Array2<f64>,
}

impl QuiverPlot {
    pub fn new(
        x: Array1<f64>,
        y: Array1<f64>,
        u: Array2<f64>,
        v: Array2<f64>,
        norm: Array2<f64>,
        true_norm: Array2<f64>,
    ) -> QuiverPlot {
        QuiverPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        }
    }

    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> QuiverPlot {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
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
        let mut norm = Array2::zeros(u.dim());
        Zip::from(&mut norm).and(&u).and(&v).for_each(|n, &u, &v| {
            *n = f64::hypot(u, v);
        });
        let true_norm = norm.clone();

        QuiverPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        }
    }

    pub fn from_vector_grid_single_plane(
        grid: VectorGrid,
        axis: usize,
        index: usize,
    ) -> QuiverPlot {
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
        let mut norm = Array2::zeros(u.dim());
        Zip::from(&mut norm).and(&u).and(&v).for_each(|n, &u, &v| {
            *n = f64::hypot(u, v);
        });
        let true_norm = norm.clone();

        QuiverPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        }
    }

    // TODO investigate if there are means to querying the plot's aspect ratio at runtime, or if we can precompute the aspect ratio
    // knowing the relative size of the legend
    fn create_arrows(&self, scale_ratio: f64) -> Vec<Arrow> {
        // angle between arrows
        const ANGLE: f64 = PI / 9.0;
        // default scale in plotly's quiver plot is 0.3
        const SCALE_FACTOR: f64 = 0.3;
        let arrow_length = &self.norm * SCALE_FACTOR;
        let u = &self.u * scale_ratio;
        let mut barb_angles = Array2::zeros(self.u.dim());
        Zip::from(&mut barb_angles)
            .and(&self.v)
            .and(&u)
            .for_each(|a, &v, &u| {
                *a = f64::atan2(v, u);
            });
        let (x, y) = meshgrid(self.x(), self.y());

        // find angles for either side of arrow
        let arrow_angle_1: Array2<f64> = &barb_angles + ANGLE;
        let arrow_angle_2: Array2<f64> = &barb_angles - ANGLE;

        //find angles for both sides of arrow point
        let sin_angle_1: Array2<f64> = arrow_angle_1.mapv(f64::sin);
        let cos_angle_1: Array2<f64> = arrow_angle_1.mapv(f64::cos);
        let sin_angle_2: Array2<f64> = arrow_angle_2.mapv(f64::sin);
        let cos_angle_2: Array2<f64> = arrow_angle_2.mapv(f64::cos);

        //find corresponding components
        let seg_1_x: Array2<f64> = &arrow_length * &cos_angle_1;
        let seg_1_y: Array2<f64> = &arrow_length * &sin_angle_1;
        let seg_2_x: Array2<f64> = &arrow_length * &cos_angle_2;
        let seg_2_y: Array2<f64> = &arrow_length * &sin_angle_2;

        let end_x: Array2<f64> = &x + &u;
        let end_y: Array2<f64> = &y + &self.v;

        //set coordinates of the arrow
        let point_1_x: Array2<f64> = &end_x - seg_1_x;
        let point_1_y: Array2<f64> = &end_y - seg_1_y;
        let point_2_x: Array2<f64> = &end_x - seg_2_x;
        let point_2_y: Array2<f64> = &end_y - seg_2_y;

        let arrows: Vec<Arrow> =
            izip!(x, y, end_x, end_y, point_1_x, point_1_y, point_2_x, point_2_y)
                .map(|(x, y, e_x, e_y, p1x, p1y, p2x, p2y)| {
                    Arrow::new((x, y), (e_x, e_y), (p1x, p1y), (p2x, p2y))
                })
                .collect();

        arrows
    }

    // BUG colourbar does not appear
    // TODO ensure that creating many traces works okay for users
    pub fn create_quiver_traces(
        &self,
        scale_ratio: f64,
        colourmap: Option<Gradient>,
    ) -> Vec<Box<dyn Trace>> {
        let arrows = self.create_arrows(scale_ratio);
        let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(arrows.len() + 1);
        let cmap_values = self.normalise_colour();
        for (arrow, cmap_value) in izip!(arrows, &cmap_values) {
            let xs = vec![
                arrow.base.0,
                arrow.tip.0,
                arrow.left_point.0,
                arrow.right_point.0,
                arrow.tip.0,
            ];
            let ys = vec![
                arrow.base.1,
                arrow.tip.1,
                arrow.left_point.1,
                arrow.right_point.1,
                arrow.tip.1,
            ];
            let colour = match colourmap {
                Some(colourmap) => format!("#{:x}", colourmap.eval_continuous(*cmap_value)),
                None => String::from("Black"),
            };
            let trace = Scatter::new(xs, ys)
                .mode(Mode::Lines)
                .show_legend(false)
                .fill(Fill::ToSelf)
                .fill_color(colour.clone())
                .show_legend(false)
                .line(Line::new().color(colour));

            traces.push(trace);
        }

        // Create an invisible marker to get the colorbar to appear - use the same map as above
        let cmap = self.get_colour_map(colourmap);
        let show = colourmap.is_some(); // only show if colourmap is provided - else we just have black arrows
        let invisible_marker = Scatter::new(vec![self.x[0]], vec![self.y[0]])
            .mode(Mode::Markers)
            .marker(
                Marker::new()
                    .cmin(*self.true_norm().min_skipnan())
                    .cmax(*self.true_norm().max_skipnan())
                    .color_bar(ColorBar::new())
                    .color_scale(ColorScale::Vector(cmap))
                    .show_scale(show)
                    .size(1),
            )
            .show_legend(false);
        traces.push(invisible_marker);

        traces
    }

    pub fn scale_global(mut self, scale_factor: f64) -> Self {
        self.u *= scale_factor;
        self.v *= scale_factor;
        Zip::from(&mut self.norm)
            .and(&self.u)
            .and(&self.v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });

        self
    }

    pub fn scale_elementwise(mut self, scale_factor: Array2<f64>) -> Self {
        self.u *= &scale_factor;
        self.v *= &scale_factor;
        Zip::from(&mut self.norm)
            .and(&self.u)
            .and(&self.v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });

        self
    }

    pub fn bound_min(self, min: f64) -> Self {
        let mut scale_factor: Array2<f64> = Array2::zeros(self.u.dim());
        Zip::from(&mut scale_factor)
            .and(&self.norm)
            .for_each(|sf, &n| {
                *sf = if n < min { min / n } else { 1. };
            });

        self.scale_elementwise(scale_factor)
    }

    pub fn bound_max(self, max: f64) -> Self {
        let mut scale_factor: Array2<f64> = Array2::zeros(self.u.dim());
        Zip::from(&mut scale_factor)
            .and(&self.norm)
            .for_each(|sf, &n| {
                *sf = if n > max { max / n } else { 1. };
            });

        self.scale_elementwise(scale_factor)
    }

    pub fn bound_min_max(self, min: f64, max: f64) -> Self {
        let mut scale_factor: Array2<f64> = Array2::zeros(self.u.dim());
        Zip::from(&mut scale_factor)
            .and(&self.norm)
            .for_each(|sf, &n| {
                *sf = if n > min {
                    max / n
                } else if n < min {
                    min / n
                } else {
                    1.
                };
            });

        self.scale_elementwise(scale_factor)
    }

    pub fn bound_half_node(mut self, dx: f64, dy: f64) -> Self {
        let largest_norm = *self.norm().max_skipnan();
        self.u *= 0.5 * dx / largest_norm;
        self.v *= 0.5 * dy / largest_norm;
        Zip::from(&mut self.norm)
            .and(&self.u)
            .and(&self.v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });

        self
    }

    pub fn bound_full_node(mut self, dx: f64, dy: f64) -> Self {
        let largest_norm = *self.norm().max_skipnan();
        self.u *= dx / largest_norm;
        self.v *= dy / largest_norm;
        Zip::from(&mut self.norm)
            .and(&self.u)
            .and(&self.v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });

        self
    }

    fn normalise_colour(&self) -> Array2<f64> {
        let min = *self.true_norm.min_skipnan();
        let max = *self.true_norm.max_skipnan();

        (&self.true_norm - min) / (max - min)
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

#[cfg(test)]
mod test {
    use ndarray::Axis;

    use super::*;
    use crate::plotting_test_support::*;
    // correct construction
    #[test]
    fn quiver_from_vector_grid_depth_averaged() {
        let grid = vector_grid();

        for axis in 0..=2 {
            let quiver_plot = QuiverPlot::from_vector_grid_depth_averaged(grid.clone(), axis);

            let i = usize::from(axis == 0);
            let j = if axis == 0 || axis == 1 { 2 } else { 1 };
            // i and j cannot be equal to the axis we are averaging along
            assert!(i != axis);
            assert!(j != axis);
            let test_u = grid.data[i].collapse(axis);
            let test_v = grid.data[j].collapse(axis);
            let delta_u = &test_u - &quiver_plot.u;
            let delta_v = &test_v - &quiver_plot.v;
            // check all elements of delta_u are less than a certain tolerance
            delta_u.iter().for_each(|&u| assert!(u.abs() < 1e-10));
            delta_v.iter().for_each(|&v| assert!(v.abs() < 1e-10));
        }
    }

    // TODO make these tests less bad
    #[test]
    fn quiver_from_vector_grid_single_plane() {
        let grid = vector_grid();
        for axis in 0..=2 {
            let i = usize::from(axis == 0);
            let j = if axis == 0 || axis == 1 { 2 } else { 1 };
            for index in 0..grid.data[i].get_data().len_of(Axis(axis)) {
                let quiver_plot =
                    QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
                let test_u = grid.data[i].get_data().index_axis(Axis(axis), index);
                let test_v = grid.data[j].get_data().index_axis(Axis(axis), index);
                let delta_u = &test_u - &quiver_plot.u;
                let delta_v = &test_v - &quiver_plot.v;
                // check all elements of delta_u are less than a certain tolerance
                delta_u.iter().for_each(|&x| assert!(x.abs() < 1e-10));
                delta_v.iter().for_each(|&x| assert!(x.abs() < 1e-10));
            }
        }
    }

    #[test]
    fn min_scaling() {
        let grid = vector_grid();
        let axis = 1;
        let index = 0;
        let quiver_plot = QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
        let scale_factor = 0.1;
        let quiver_plot = quiver_plot.bound_min(scale_factor);
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let mut test_u = &quiver_plot.u / &grid.data[i].get_data().index_axis(Axis(axis), index);
        let mut test_v = &quiver_plot.v / &grid.data[j].get_data().index_axis(Axis(axis), index);
        // Here we define 0/0 as scale_factor because the scaling factor is essentially not applied to zero values
        test_u.mapv_inplace(|x| if x.is_nan() { scale_factor } else { x });
        test_v.mapv_inplace(|x| if x.is_nan() { scale_factor } else { x });
        // Check that u and v are greater than or equal to the scale factor
        test_u
            .iter()
            .for_each(|&u| assert!((u.abs() - scale_factor) >= -1e-10));
        test_v
            .iter()
            .for_each(|&v| assert!((v.abs() - scale_factor) >= -1e-10));
        // Check that the norms are greater than or equal to the scale factor
        quiver_plot
            .norm
            .iter()
            .for_each(|&n| assert!(n >= scale_factor));
    }
    #[test]
    fn max_scaling() {
        let grid = vector_grid();
        let axis = 1;
        let index = 0;
        let quiver = QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
        let scale_factor = 0.5;
        let quiver = quiver.bound_max(scale_factor);
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let mut test_u = &quiver.u / &grid.data[i].get_data().index_axis(Axis(axis), index);
        let mut test_v = &quiver.v / &grid.data[j].get_data().index_axis(Axis(axis), index);
        // Here we define 0/0 as scale_factor because the scaling factor is essentially not applied to zero values
        test_u.mapv_inplace(|x| if x.is_nan() { scale_factor } else { x });
        test_v.mapv_inplace(|x| if x.is_nan() { scale_factor } else { x });
        // Check that u and v are at less than or equal to the scale factor
        test_u
            .iter()
            .for_each(|&u| assert!((u.abs() - scale_factor) <= 1e-10));
        test_v
            .iter()
            .for_each(|&v| assert!((v.abs() - scale_factor) <= 1e-10));
        // Check that the norms are less than or equal to the scale factor
        quiver.norm.iter().for_each(|&n| assert!(n <= scale_factor));
    }
    #[test]
    fn min_max_scaling() {
        let grid = vector_grid();
        let axis = 1;
        let index = 0;
        let quiver = QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
        let max_size = 0.5;
        let min_size = 0.1;
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let quiver = quiver.bound_min_max(min_size, max_size);
        let mut test_u = &quiver.u / &grid.data[i].get_data().index_axis(Axis(axis), index);
        let mut test_v = &quiver.v / &grid.data[j].get_data().index_axis(Axis(axis), index);
        // Here we define 0/0 as min_size because the scaling factor is essentially not applied to zero values
        test_u.mapv_inplace(|x| if x.is_nan() { min_size } else { x });
        test_v.mapv_inplace(|x| if x.is_nan() { min_size } else { x });
        // Check that u and v are less than or equal to the scale factor
        test_u
            .iter()
            .for_each(|&u| assert!((u.abs() - max_size) <= 1e-10));
        test_v
            .iter()
            .for_each(|&v| assert!((v.abs() - max_size) <= 1e-10));
        // Also check that u and v are greater than or equal to the scale factor
        test_u
            .iter()
            .for_each(|&u| assert!((u.abs() - min_size) >= -1e-10));
        test_v
            .iter()
            .for_each(|&v| assert!((v.abs() - min_size) >= -1e-10));
        // Check that the norms are less than or equal to the scale factor
        quiver.norm.iter().for_each(|&n| assert!(n <= max_size));
        // Also check that they are greater than or equal to the scale factor
        quiver.norm.iter().for_each(|&n| assert!(n >= min_size));
    }
    #[test]
    fn half_node_scaling() {
        let grid = vector_grid();
        let axis = 1;
        let index = 0;
        let quiver = QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
        let dx = 0.5;
        let dy = 0.5;
        let quiver = quiver.bound_half_node(dx, dy);
        let largest_norm = *quiver.norm().max_skipnan();
        let mut test_u = &quiver.u * 0.5 * dx / largest_norm;
        let mut test_v = &quiver.v * 0.5 * dy / largest_norm;
        // Here we define 0/0 as 0.5 * dx / largest_norm because the scaling factor is essentially not applied to zero values
        test_u.mapv_inplace(|x| {
            if x.is_nan() {
                0.5 * dx / largest_norm
            } else {
                x
            }
        });
        test_v.mapv_inplace(|x| {
            if x.is_nan() {
                0.5 * dy / largest_norm
            } else {
                x
            }
        });
        // Check that u and v are less than or equal to the scale factor
        test_u
            .iter()
            .for_each(|&u| assert!((u.abs() - 0.5 * dx / largest_norm) <= 1e-10));
        test_v
            .iter()
            .for_each(|&v| assert!((v.abs() - 0.5 * dy / largest_norm) <= 1e-10));
        // Check that the norms are less than or equal to the scale factor
        quiver
            .norm
            .iter()
            .for_each(|&n| assert!(n <= 0.5 * dx / largest_norm));
    }
    #[test]
    fn full_node_scaling() {
        let grid = vector_grid();
        let axis = 1;
        let index = 0;
        let quiver = QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
        let dx = 0.5;
        let dy = 0.5;
        let quiver = quiver.bound_full_node(dx, dy);
        let largest_norm = *quiver.norm().max_skipnan();
        let mut test_u = &quiver.u * dx / largest_norm;
        let mut test_v = &quiver.v * dy / largest_norm;
        // Here we define 0/0 as dx / largest_norm because the scaling factor is essentially not applied to zero values
        test_u.mapv_inplace(|x| if x.is_nan() { dx / largest_norm } else { x });
        test_v.mapv_inplace(|x| if x.is_nan() { dy / largest_norm } else { x });
        // Check that u and v are less than or equal to the scale factor
        test_u
            .iter()
            .for_each(|&u| assert!((u.abs() - dx / largest_norm) <= 1e-10));
        test_v
            .iter()
            .for_each(|&v| assert!((v.abs() - dy / largest_norm) <= 1e-10));
        // Check that the norms are less than or equal to the scale factor
        quiver
            .norm
            .iter()
            .for_each(|&n| assert!(n <= dx / largest_norm));
    }
    #[test]
    fn norm_correctness() {
        // check for each axis and index
        let grid = vector_grid();
        for axis in 0..=2 {
            for index in 0..grid.data[0].get_data().len_of(Axis(axis)) {
                let quiver_plot =
                    QuiverPlot::from_vector_grid_single_plane(grid.clone(), axis, index);
                let mut norm = Array2::zeros(quiver_plot.u.dim());
                Zip::from(&mut norm)
                    .and(&quiver_plot.u)
                    .and(&quiver_plot.v)
                    .for_each(|n, &u, &v| {
                        *n = f64::hypot(u, v);
                    });
                let delta = &norm - &quiver_plot.norm;
                delta.iter().for_each(|&x| assert!(x.abs() < 1e-10));
            }
        }

        // now with depth-averaging
        for axis in 0..=2 {
            let quiver_plot = QuiverPlot::from_vector_grid_depth_averaged(grid.clone(), axis);
            let mut norm = Array2::zeros(quiver_plot.u.dim());
            Zip::from(&mut norm)
                .and(&quiver_plot.u)
                .and(&quiver_plot.v)
                .for_each(|n, &u, &v| {
                    *n = f64::hypot(u, v);
                });
            let delta = &norm - &quiver_plot.norm;
            delta.iter().for_each(|&x| assert!(x.abs() < 1e-10));
        }
    }
    #[test]
    fn colour_range() {}
    #[test]
    fn trace_creation() {}
}
