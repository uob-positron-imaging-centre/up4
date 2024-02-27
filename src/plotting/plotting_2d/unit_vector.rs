use std::f64::consts::PI;

use crate::plotting_2d::Arrow;
use crate::utilities::maths::meshgrid;
use crate::{GridFunctions3D, VectorGrid};
use colorous::Gradient;
use derive_getters::Getters;
use itertools::izip;
use ndarray::{Array1, Array2, Zip};
use ndarray_stats::QuantileExt;
use plotly::common::{ColorScale, ColorScaleElement, Fill, Line};

use plotly::{color::NamedColor, common::Mode};
use plotly::{HeatMap, Scatter, Trace};

// TODO make sure that the arrow centres are in the centre of the cell
#[derive(Getters, Clone)]
pub struct UnitVectorPlot {
    x: Array1<f64>,
    y: Array1<f64>,
    u: Array2<f64>,
    v: Array2<f64>,
    norm: Array2<f64>,
    true_norm: Array2<f64>,
}

impl UnitVectorPlot {
    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> UnitVectorPlot {
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
        let mut u = grid.data[i].collapse(axis);
        let mut v = grid.data[j].collapse(axis);
        let mut true_norm = Array2::zeros(u.dim());
        Zip::from(&mut true_norm)
            .and(&u)
            .and(&v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });
        u /= &true_norm;
        v /= &true_norm;
        let norm = Array2::ones(u.dim());

        let dx = x[1] - x[0];
        let dy = y[1] - y[0];
        let plot = UnitVectorPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        };
        plot.bound_half_node(dx, dy)
    }

    pub fn from_vector_grid_single_plane(
        grid: VectorGrid,
        axis: usize,
        index: usize,
    ) -> UnitVectorPlot {
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
        let mut u = grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut v = grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut true_norm = Array2::zeros(u.dim());
        Zip::from(&mut true_norm)
            .and(&u)
            .and(&v)
            .for_each(|n, &u, &v| {
                *n = f64::hypot(u, v);
            });
        u /= &true_norm;
        v /= &true_norm;
        let norm = Array2::ones(u.dim());

        let dx = x[1] - x[0];
        let dy = y[1] - y[0];
        let plot = UnitVectorPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        };
        plot.bound_half_node(dx, dy)
    }

    // BUG arrowheads are not drawn properly in method from plotly.py
    // need to reconsider how the arrows are drawn to do this properly
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
            .and(&self.u)
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

    pub fn create_quiver_traces(
        &self,
        scale_ratio: f64,
        colourmap: Option<Gradient>,
    ) -> Vec<Box<dyn Trace>> {
        let arrows = self.create_arrows(scale_ratio);
        let mut traces = Vec::<Box<dyn Trace>>::with_capacity(arrows.len() + 1);
        for arrow in arrows {
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
            let trace = Scatter::new(xs, ys)
                .mode(Mode::Lines)
                .show_legend(false)
                .fill(Fill::ToSelf)
                .fill_color(NamedColor::Black)
                .show_legend(false)
                .line(Line::new().color(NamedColor::Black));
            traces.push(trace);
        }
        traces.push(self.create_unit_vector_background(colourmap));

        traces
    }

    fn create_unit_vector_background(
        &self,
        colourmap: Option<Gradient>,
    ) -> Box<HeatMap<f64, f64, f64>> {
        let (x, y) = meshgrid(self.x(), self.y());
        let cmap = self.get_colour_map(colourmap);
        // let mut z = Vec::with_capacity(self.true_norm.dim().0);
        // for row in self.true_norm.rows() {
        //     let mut inner_vec = Vec::with_capacity(row.len());
        //     for val in row {
        //         inner_vec.push(*val);
        //     }
        //     z.push(inner_vec);
        // }
        

        HeatMap::new(
            x.into_raw_vec(),
            y.into_raw_vec(),
            self.true_norm.to_owned().into_raw_vec(),
        )
        .color_scale(ColorScale::Vector(cmap))
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
                *sf = if n > max {
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
}
