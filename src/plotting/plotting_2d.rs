//! Submodule for handling 2D plots.

use derive_getters::Getters;
use itertools::izip;
use ndarray::{Array1, Array2, Zip};
use ndarray_stats::QuantileExt;
use plotly::{
    common::{ColorBar, ColorScale, ColorScalePalette, Fill, Line, Marker, Mode},
    Scatter,
};
use std::f64::consts::PI;

use crate::{utilities::maths::meshgrid, GridFunctions3D, VectorGrid};

/// Internal representation of arrow data
struct Arrow {
    base: (f64, f64),
    tip: (f64, f64),
    left_point: (f64, f64),
    right_point: (f64, f64),
}

impl Arrow {
    fn new(x: f64, y: f64, u: f64, v: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> Arrow {
        let base = (x, y);
        let tip = (x + u, y + v);
        let left_point = (p1x, p1y);
        let right_point = (p2x, p2y);

        Arrow {
            base,
            tip,
            left_point,
            right_point,
        }
    }
}

#[derive(Getters, Clone)]
pub struct QuiverPlot {
    x: Array1<f64>,
    y: Array1<f64>,
    u: Array2<f64>,
    v: Array2<f64>,
    norm: Array2<f64>,
    true_norm: Array2<f64>,
}

// TODO have both a depth-average constructor and a per plane one
impl QuiverPlot {
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
        let i = if axis == 0 {
            1
        } else {
            0
        };
        let j = if axis == 0 || axis == 1 {
            2
        } else {
            1
        };
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
        } else if axis == 1 {
            grid.get_xpositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 {
            grid.get_zpositions().to_owned()
        } else if axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let i = if axis == 0 {
            1
        } else if axis == 1 {
            0
        } else {
            0
        };
        let j = if axis == 0 {
            2
        } else if axis == 1 {
            2
        } else {
            1
        };
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

    // BUG arrowheads are not drawn properly in method from plotly.py
    // need to reconsider how the arrows are drawn to do this properly
    fn create_arrows(&self) -> Vec<Arrow> {
        // angle between arrows
        const ANGLE: f64 = PI / 9.0;
        // default scale in plotly's quiver plot is 0.3
        const SCALE_FACTOR: f64 = 0.3;
        let arrow_length = &self.norm * SCALE_FACTOR;
        let mut barb_angles = Array2::zeros(self.u.dim());
        Zip::from(&mut barb_angles)
            .and(&self.v)
            .and(&self.u)
            .for_each(|a, &v, &u| {
                *a = f64::atan2(v, u);
            });
        let (x, y) = meshgrid(self.x.to_owned(), self.y.to_owned());
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

        //set coordinates of the arrow
        let point_1_x: Array2<f64> = &x + &self.u - seg_1_x;
        let point_1_y: Array2<f64> = &y + &self.v - seg_1_y;
        let point_2_x: Array2<f64> = &x + &self.u - seg_2_x;
        let point_2_y: Array2<f64> = &y + &self.v - seg_2_y;

        let arrows: Vec<Arrow> =
            izip!(&x, &y, &self.u, &self.v, point_1_x, point_1_y, point_2_x, point_2_y)
                .map(|(x, y, u, v, p1x, p1y, p2x, p2y)| {
                    Arrow::new(*x, *y, *u, *v, p1x, p1y, p2x, p2y)
                })
                .collect();

        arrows
    }

    pub fn create_quiver_traces(&self) -> Vec<Box<Scatter<f64, f64>>> {
        let arrows = self.create_arrows();
        let mut traces = Vec::with_capacity(arrows.len() + 1);
        let colours = self.normalise_colour();
        for (arrow, colour) in izip!(arrows, &colours) {
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
            // TODO replace colours with our own custom implementation
            let colourmap = colorous::VIRIDIS;
            let colour = format!("#{:x}", colourmap.eval_continuous(*colour));
            let trace = Scatter::new(xs, ys)
                .mode(Mode::Lines)
                .show_legend(false)
                .fill(Fill::ToSelf)
                .fill_color(colour.clone())
                .show_legend(false)
                .line(Line::new().color(colour));
            traces.push(trace);
        }

        //create an invisible marker to get the colorbar to appear - use the same map as above
        let invisible_marker = Scatter::new(vec![self.x[0]], vec![self.y[0]])
            .mode(Mode::Markers)
            .marker(
                Marker::new()
                    .cmin(*colours.min_skipnan())
                    .cmax(*colours.max_skipnan())
                    .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                    .color_bar(ColorBar::new())
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
}

// TODO make sure that the arrow centres are in the centre of the cell
pub struct UnitVectorPlot {
    x: Array1<f64>,
    y: Array1<f64>,
    u: Array2<f64>,
    v: Array2<f64>,
    norm: Array2<f64>,
}
