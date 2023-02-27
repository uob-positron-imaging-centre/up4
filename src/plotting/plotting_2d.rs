//! Submodule for handling 2D plots.

use derive_getters::Getters;
use itertools::izip;
use ndarray::{Array1, Array2, Zip};
use ndarray_stats::QuantileExt;
use plotly::{
    common::{ColorBar, ColorScale, ColorScalePalette, Fill, Line, Marker, Mode, MarkerSymbol},
    Scatter, HeatMap, heat_map::Smoothing, color::NamedColor, Contour, Trace
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
    fn new(base: (f64, f64), tip: (f64, f64), left_point: (f64, f64), right_point: (f64, f64)) -> Arrow {

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
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        }  else {
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

        let end_x: Array2<f64> = &x + &self.u;
        let end_y: Array2<f64> = &y + &self.v;

        //set coordinates of the arrow
        let point_1_x: Array2<f64> = &end_x - seg_1_x;
        let point_1_y: Array2<f64> = &end_y - seg_1_y;
        let point_2_x: Array2<f64> = &end_x - seg_2_x;
        let point_2_y: Array2<f64> = &end_y - seg_2_y;

        let arrows: Vec<Arrow> =
            izip!(x, y, end_x, end_y, point_1_x, point_1_y, point_2_x, point_2_y)
                .map(|(x, y, e_x, e_y, p1x, p1y, p2x, p2y)| {
                    Arrow::new(
                        (x, y),
                        (e_x, e_y),
                        (p1x, p1y),
                        (p2x, p2y),
                    )
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
        let mut u = grid.data[i].collapse(axis);
        let mut v = grid.data[j].collapse(axis);
        let mut norm = Array2::zeros(u.dim());
        Zip::from(&mut norm).and(&u).and(&v).for_each(|n, &u, &v| {
            *n = f64::hypot(u, v);
        });
        let true_norm = norm.clone();

        u /= &norm;
        v /= &norm;
        let dx = x[1] - x[0];
        let dy = y[1] - y[0];
        UnitVectorPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        }.bound_half_node(dx, dy)
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
        }  else {
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
        let mut u = grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut v = grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut norm = Array2::zeros(u.dim());
        Zip::from(&mut norm).and(&u).and(&v).for_each(|n, &u, &v| {
            *n = f64::hypot(u, v);
        });
        let true_norm = norm.clone();
        
        u /= &norm;
        v /= &norm;
        let dx = x[1] - x[0];
        let dy = y[1] - y[0];
        UnitVectorPlot {
            x,
            y,
            u,
            v,
            norm,
            true_norm,
        }.bound_half_node(dx, dy)
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

        let end_x: Array2<f64> = &x + &self.u;
        let end_y: Array2<f64> = &y + &self.v;

        //set coordinates of the arrow
        let point_1_x: Array2<f64> = &end_x - seg_1_x;
        let point_1_y: Array2<f64> = &end_y - seg_1_y;
        let point_2_x: Array2<f64> = &end_x - seg_2_x;
        let point_2_y: Array2<f64> = &end_y - seg_2_y;

        let arrows: Vec<Arrow> =
            izip!(x, y, end_x, end_y, point_1_x, point_1_y, point_2_x, point_2_y)
                .map(|(x, y, e_x, e_y, p1x, p1y, p2x, p2y)| {
                    Arrow::new(
                        (x, y),
                        (e_x, e_y),
                        (p1x, p1y),
                        (p2x, p2y),
                    )
                })
                .collect();

        arrows
    }

    pub fn create_quiver_traces(&self) -> Vec<Box<dyn Trace>> {
        let arrows = self.create_arrows();
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
        traces.push(self.create_unit_vector_background());

        traces
    }

    fn create_unit_vector_background(&self) -> Box<HeatMap<f64, f64, f64>> {
        let (x, y) = meshgrid(self.x.to_owned(), self.y.to_owned());
        let heatmap = HeatMap::new(
            x.into_raw_vec(),
            y.into_raw_vec(),
            self.true_norm.to_owned().into_raw_vec()
        )
        .zsmooth(Smoothing::False)
        .zmin(*self.true_norm.min_skipnan())
        .zmax(*self.true_norm.min_skipnan());

        heatmap
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

pub struct ScalarMap {
    x: Array1<f64>,
    y: Array1<f64>,
    data: Array2<f64>,
}

impl ScalarMap {
    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> ScalarMap {
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
        let mut data = Array2::zeros(u.dim());
        Zip::from(&mut data).and(&u).and(&v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        ScalarMap {x,y,data}

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
        }  else {
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
        let u = grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let v = grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut data = Array2::zeros(u.dim());
        Zip::from(&mut data).and(&u).and(&v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        ScalarMap { x, y, data}
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
    }  else {
        grid.get_ypositions().to_owned()
    };        
    let data = grid.collapse(axis);

    ScalarMap { x, y, data }
    }

    pub fn from_grid_single_plane(grid: Box<dyn GridFunctions3D>, axis: usize, index: usize) -> ScalarMap {
    // select yz (0), xz (1) or xy (2) plane
    let x = if axis == 0 {
        grid.get_ypositions().to_owned()
    } else {
        grid.get_xpositions().to_owned()
    };
    let y = if axis == 0 || axis == 1 {
        grid.get_zpositions().to_owned()
    }  else {
        grid.get_ypositions().to_owned()
    };   
    let data = grid
    .get_data()
    .to_owned()
    .index_axis_move(ndarray::Axis(axis), index);

    ScalarMap { x, y, data }
    }

    pub fn create_scalar_map(&self) -> Vec<Box<HeatMap<f64, f64, f64>>> {
        let (x, y) = meshgrid(self.x.to_owned(), self.y.to_owned());
        let heatmap = HeatMap::new(
            x.into_raw_vec(),
            y.into_raw_vec(),
            self.data.to_owned().into_raw_vec(),
        );
        let trace = vec![heatmap];

        trace
    }
}

pub struct ScalarContour {
    x: Array1<f64>,
    y: Array1<f64>,
    data: Array2<f64>,
}

impl ScalarContour {
    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> ScalarContour {
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
        let mut data = Array2::zeros(u.dim());
        Zip::from(&mut data).and(&u).and(&v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        ScalarContour {x,y,data}

    }

    pub fn from_vector_grid_single_plane(grid: VectorGrid, axis: usize, index: usize) -> ScalarContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        }  else {
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
        let u = grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let v = grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut data = Array2::zeros(u.dim());
        Zip::from(&mut data).and(&u).and(&v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        ScalarContour { x, y, data}
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
    }  else {
        grid.get_ypositions().to_owned()
    };        
    let data = grid.collapse(axis);

    ScalarContour { x, y, data }
    }

    pub fn from_grid_single_plane(grid: Box<dyn GridFunctions3D>, axis: usize, index: usize) -> ScalarContour {
    // select yz (0), xz (1) or xy (2) plane
    let x = if axis == 0 {
        grid.get_ypositions().to_owned()
    } else {
        grid.get_xpositions().to_owned()
    };
    let y = if axis == 0 || axis == 1 {
        grid.get_zpositions().to_owned()
    }  else {
        grid.get_ypositions().to_owned()
    };   
    let data = grid
    .get_data()
    .to_owned()
    .index_axis_move(ndarray::Axis(axis), index);

    ScalarContour { x, y, data }
    }

    pub fn create_scalar_contour(&self) -> Vec<Box<Contour<f64, f64, f64>>> {
        let (x, y) = meshgrid(self.x.to_owned(), self.y.to_owned());
        let contour = Contour::new(
            x.into_raw_vec(),
            y.into_raw_vec(),
            self.data.to_owned().into_raw_vec(),
        );
        let trace = vec![contour];

        trace
    }
}

// TODO we need to tell off users for unequal inputs
pub struct ParityPlot {
    reference_data: Vec<f64>,
    comparison_data: Vec<f64>,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64
}

impl ParityPlot {
    pub fn from_vector_grids(reference_grid: VectorGrid, comparison_grid: VectorGrid) -> ParityPlot {
        let capacity = reference_grid.get_data().len() * 3;
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64>= Vec::with_capacity(capacity);
        
        let xmin_reference = *reference_grid.get_data().min_skipnan();
        let xmax_reference = *reference_grid.get_data().max_skipnan();
        let xmin_comparison = *comparison_grid.get_data().min_skipnan();
        let xmax_comparison = *comparison_grid.get_data().max_skipnan();
        let ymin_reference = *reference_grid.get_data().min_skipnan();
        let ymax_reference = *reference_grid.get_data().max_skipnan();
        let ymin_comparison = *comparison_grid.get_data().min_skipnan();
        let ymax_comparison = *comparison_grid.get_data().max_skipnan();

        let xmin = if xmin_reference < xmin_comparison {
            xmin_reference
        } else {
            xmin_comparison
        };
        let xmax = if xmax_reference < xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference < ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        for i in 0..=2 {
            for el in reference_grid.data[i].get_data() {
                reference_data.push(*el);
            }
            for el in comparison_grid.data[i].get_data() {
                comparison_data.push(*el);
            }
        }

        ParityPlot { reference_data, comparison_data, xmin, xmax, ymin, ymax }
    }   

    pub fn from_vector_grids_depth_averaged(reference_grid: VectorGrid, comparison_grid: VectorGrid, axis: usize) -> ParityPlot {      
        let xmin_reference = *reference_grid.get_data().min_skipnan();
        let xmax_reference = *reference_grid.get_data().max_skipnan();
        let xmin_comparison = *comparison_grid.get_data().min_skipnan();
        let xmax_comparison = *comparison_grid.get_data().max_skipnan();
        let ymin_reference = *reference_grid.get_data().min_skipnan();
        let ymax_reference = *reference_grid.get_data().max_skipnan();
        let ymin_comparison = *comparison_grid.get_data().min_skipnan();
        let ymax_comparison = *comparison_grid.get_data().max_skipnan();

        let xmin = if xmin_reference < xmin_comparison {
            xmin_reference
        } else {
            xmin_comparison
        };
        let xmax = if xmax_reference < xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference < ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
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

        let ref_u = reference_grid.data[i].collapse(axis);
        let ref_v = reference_grid.data[j].collapse(axis);

        let comp_u = comparison_grid.data[i].collapse(axis);
        let comp_v = comparison_grid.data[j].collapse(axis);

        let capacity = ref_u.len() + ref_v.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64>= Vec::with_capacity(capacity);

        for el in ref_u{
            reference_data.push(el);
        }
        for el in ref_v{
            reference_data.push(el);
        }

        for el in comp_u{
            comparison_data.push(el);
        }
        for el in comp_v{
            comparison_data.push(el);
        }

        ParityPlot { reference_data, comparison_data, xmin, xmax, ymin, ymax }
    }

    pub fn from_vector_grids_single_plane(reference_grid: VectorGrid, comparison_grid: VectorGrid, axis: usize, index: usize) -> ParityPlot {
        let xmin_reference = *reference_grid.get_data().min_skipnan();
        let xmax_reference = *reference_grid.get_data().max_skipnan();
        let xmin_comparison = *comparison_grid.get_data().min_skipnan();
        let xmax_comparison = *comparison_grid.get_data().max_skipnan();
        let ymin_reference = *reference_grid.get_data().min_skipnan();
        let ymax_reference = *reference_grid.get_data().max_skipnan();
        let ymin_comparison = *comparison_grid.get_data().min_skipnan();
        let ymax_comparison = *comparison_grid.get_data().max_skipnan();

        let xmin = if xmin_reference < xmin_comparison {
            xmin_reference
        } else {
            xmin_comparison
        };
        let xmax = if xmax_reference < xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference < ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
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

        let ref_u = reference_grid.data[i].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let ref_v = reference_grid.data[j].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let comp_u = comparison_grid.data[i].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let comp_v = comparison_grid.data[j].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let capacity = ref_u.len() + ref_v.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64>= Vec::with_capacity(capacity);

        for el in ref_u{
            reference_data.push(el);
        }
        for el in ref_v{
            reference_data.push(el);
        }

        for el in comp_u{
            comparison_data.push(el);
        }
        for el in comp_v{
            comparison_data.push(el);
        }

        ParityPlot { reference_data, comparison_data, xmin, xmax, ymin, ymax }

    }

    pub fn from_grids(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D> ) -> ParityPlot {
        let capacity = reference_grid.get_data().len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64>= Vec::with_capacity(capacity);
        
        let xmin_reference = *reference_grid.get_data().min_skipnan();
        let xmax_reference = *reference_grid.get_data().max_skipnan();
        let xmin_comparison = *comparison_grid.get_data().min_skipnan();
        let xmax_comparison = *comparison_grid.get_data().max_skipnan();
        let ymin_reference = *reference_grid.get_data().min_skipnan();
        let ymax_reference = *reference_grid.get_data().max_skipnan();
        let ymin_comparison = *comparison_grid.get_data().min_skipnan();
        let ymax_comparison = *comparison_grid.get_data().max_skipnan();

        let xmin = if xmin_reference < xmin_comparison {
            xmin_reference
        } else {
            xmin_comparison
        };
        let xmax = if xmax_reference < xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference < ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        for el in reference_grid.get_data(){
            reference_data.push(*el);
        }


        for el in comparison_grid.get_data(){
            comparison_data.push(*el);
        }        

        ParityPlot { reference_data, comparison_data, xmin, xmax, ymin, ymax }

    }

    pub fn from_grids_depth_averaged(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D>, axis: usize ) -> ParityPlot {       
        let xmin_reference = *reference_grid.get_data().min_skipnan();
        let xmax_reference = *reference_grid.get_data().max_skipnan();
        let xmin_comparison = *comparison_grid.get_data().min_skipnan();
        let xmax_comparison = *comparison_grid.get_data().max_skipnan();
        let ymin_reference = *reference_grid.get_data().min_skipnan();
        let ymax_reference = *reference_grid.get_data().max_skipnan();
        let ymin_comparison = *comparison_grid.get_data().min_skipnan();
        let ymax_comparison = *comparison_grid.get_data().max_skipnan();

        let xmin = if xmin_reference < xmin_comparison {
            xmin_reference
        } else {
            xmin_comparison
        };
        let xmax = if xmax_reference < xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference < ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        let collapsed_ref = reference_grid.collapse(axis);
        let collapsed_comp = comparison_grid.collapse(axis);

        let capacity = collapsed_ref.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64>= Vec::with_capacity(capacity);

        for el in collapsed_ref {
            reference_data.push(el);
        }
        
        for el in collapsed_comp {
            comparison_data.push(el)
        }

        ParityPlot { reference_data, comparison_data, xmin, xmax, ymin, ymax }

    }

    pub fn from_grids_single_plane(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D> , axis: usize, index: usize) -> ParityPlot {
        let xmin_reference = *reference_grid.get_data().min_skipnan();
        let xmax_reference = *reference_grid.get_data().max_skipnan();
        let xmin_comparison = *comparison_grid.get_data().min_skipnan();
        let xmax_comparison = *comparison_grid.get_data().max_skipnan();
        let ymin_reference = *reference_grid.get_data().min_skipnan();
        let ymax_reference = *reference_grid.get_data().max_skipnan();
        let ymin_comparison = *comparison_grid.get_data().min_skipnan();
        let ymax_comparison = *comparison_grid.get_data().max_skipnan();

        let xmin = if xmin_reference < xmin_comparison {
            xmin_reference
        } else {
            xmin_comparison
        };
        let xmax = if xmax_reference < xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference < ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        let collapsed_ref = reference_grid.get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let collapsed_comp = comparison_grid.get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let capacity = collapsed_ref.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64>= Vec::with_capacity(capacity);

        for el in collapsed_ref {
            reference_data.push(el);
        }

        for el in collapsed_comp {
            comparison_data.push(el);
        }

        ParityPlot { reference_data, comparison_data, xmin, xmax, ymin, ymax }


    }

    pub fn create_parity_scatter(&self) -> Vec<Box<Scatter<f64, f64>>> {
        let parity_line = Scatter::new(vec![self.xmin, self.xmax], vec![self.ymin, self.ymax])
            .mode(Mode::Lines)
            .show_legend(false)
            .line(Line::new().color(NamedColor::Black));

        let parity_scatter = Scatter::new(
            self.reference_data.to_owned(),
            self.comparison_data.to_owned(),
        )
        .mode(Mode::Markers)
        .marker(Marker::new().symbol(MarkerSymbol::Cross))
        .show_legend(false);

        let traces = vec![parity_line, parity_scatter];
        
        traces
    }
}

pub struct ParityMap {
    reference_data: Array2<f64>,
    comparison_data: Array2<f64>,
    x: Array2<f64>,
    y: Array2<f64>,
    delta: Array2<f64>
}

impl ParityMap {
    pub fn from_vector_grids_depth_averaged(reference_grid: VectorGrid, comparison_grid: VectorGrid, axis: usize) -> ParityMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
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
        let (x, y) = meshgrid(x, y);
        
        let ref_u = reference_grid.data[i].collapse(axis);
        let ref_v = reference_grid.data[j].collapse(axis);
        let mut reference_data = Array2::zeros(ref_u.dim());
        Zip::from(&mut reference_data).and(&ref_u).and(&ref_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let comp_u = comparison_grid.data[i].collapse(axis);
        let comp_v = comparison_grid.data[j].collapse(axis);
        let mut comparison_data = Array2::zeros(comp_u.dim());
        Zip::from(&mut comparison_data).and(&comp_u).and(&comp_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let delta = &reference_data - &comparison_data;

        ParityMap { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
    }

    pub fn from_vector_grids_single_plane(reference_grid: VectorGrid, comparison_grid: VectorGrid, axis: usize, index: usize) -> ParityMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
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
        let (x, y) = meshgrid(x, y);
        
        let ref_u = reference_grid.data[i].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let ref_v = reference_grid.data[j].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let mut reference_data = Array2::zeros(ref_u.dim());
        Zip::from(&mut reference_data).and(&ref_u).and(&ref_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let comp_u = comparison_grid.data[i].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let comp_v = comparison_grid.data[j].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let mut comparison_data = Array2::zeros(comp_u.dim());
        Zip::from(&mut comparison_data).and(&comp_u).and(&comp_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let delta = &reference_data - &comparison_data;

        ParityMap { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
        }
    
    pub fn from_grids_depth_averaged(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D>, axis: usize) -> ParityMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };

        let (x, y) = meshgrid(x, y);
        
        let reference_data = reference_grid.collapse(axis);

        let comparison_data = comparison_grid.collapse(axis);

        let delta = &reference_data - &comparison_data;

        ParityMap { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
    }

    pub fn from_grids_single_plane(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D>, axis: usize, index: usize) -> ParityMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };

        let (x, y) = meshgrid(x, y);
        
        let reference_data = reference_grid.get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let comparison_data = comparison_grid.get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let delta = &reference_data - &comparison_data;

        ParityMap { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
    }

    pub fn create_parity_map(&self) -> Vec<Box<HeatMap<f64, f64, f64>>> {

        let heatmap = HeatMap::new(
            self.x.to_owned().into_raw_vec(),
            self.y.to_owned().into_raw_vec(),
            self.delta.to_owned().into_raw_vec()
        );
        let traces = vec![heatmap];

        traces
    }

    pub fn delta_as_percent(self)-> Self {
        let delta = (&self.reference_data - &self.comparison_data) / &self.reference_data * 100.;

        ParityMap { reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x, 
            y: self.y, 
            delta}
    }

    pub fn delta_as_difference(self) -> Self {
        let delta = &self.reference_data - &self.comparison_data;

        ParityMap { reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x, 
            y: self.y, 
            delta}
    }

    pub fn delta_as_absolute_difference(self) -> Self {
        let delta_vec: Vec<f64> = (&self.reference_data - &self.comparison_data).into_iter().map(f64::abs).collect();
        let delta: Array2<f64> = Array2::from_shape_vec(self.reference_data.raw_dim(), delta_vec).unwrap();
        ParityMap { reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x, 
            y: self.y, 
            delta}
    }
}

pub struct ParityContour {
    reference_data: Array2<f64>,
    comparison_data: Array2<f64>,
    x: Array2<f64>,
    y: Array2<f64>,
    delta: Array2<f64>
}

impl ParityContour {
    pub fn from_vector_grids_depth_averaged(reference_grid: VectorGrid, comparison_grid: VectorGrid, axis: usize) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
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
        let (x, y) = meshgrid(x, y);
        
        let ref_u = reference_grid.data[i].collapse(axis);
        let ref_v = reference_grid.data[j].collapse(axis);
        let mut reference_data = Array2::zeros(ref_u.dim());
        Zip::from(&mut reference_data).and(&ref_u).and(&ref_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let comp_u = comparison_grid.data[i].collapse(axis);
        let comp_v = comparison_grid.data[j].collapse(axis);
        let mut comparison_data = Array2::zeros(comp_u.dim());
        Zip::from(&mut comparison_data).and(&comp_u).and(&comp_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let delta = &reference_data - &comparison_data;

        ParityContour { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
    }

    pub fn from_vector_grids_single_plane(reference_grid: VectorGrid, comparison_grid: VectorGrid, axis: usize, index: usize) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
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
        let (x, y) = meshgrid(x, y);
        
        let ref_u = reference_grid.data[i].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let ref_v = reference_grid.data[j].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let mut reference_data = Array2::zeros(ref_u.dim());
        Zip::from(&mut reference_data).and(&ref_u).and(&ref_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let comp_u = comparison_grid.data[i].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let comp_v = comparison_grid.data[j].get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);
        let mut comparison_data = Array2::zeros(comp_u.dim());
        Zip::from(&mut comparison_data).and(&comp_u).and(&comp_v).for_each(|d, &u, &v| {
            *d = f64::hypot(u, v);
        });

        let delta = &reference_data - &comparison_data;

        ParityContour { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
        }
    
    pub fn from_grids_depth_averaged(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D>, axis: usize) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };

        let (x, y) = meshgrid(x, y);
        
        let reference_data = reference_grid.collapse(axis);

        let comparison_data = comparison_grid.collapse(axis);

        let delta = &reference_data - &comparison_data;

        ParityContour { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
    }

    pub fn from_grids_single_plane(reference_grid: Box<dyn GridFunctions3D>, comparison_grid: Box<dyn GridFunctions3D>, axis: usize, index: usize) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };

        let (x, y) = meshgrid(x, y);
        
        let reference_data = reference_grid.get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let comparison_data = comparison_grid.get_data()
        .to_owned()
        .index_axis_move(ndarray::Axis(axis), index);

        let delta = &reference_data - &comparison_data;

        ParityContour { reference_data, 
            comparison_data, 
            x, 
            y, 
            delta }
    }

    pub fn create_parity_contour(&self) -> Vec<Box<HeatMap<f64, f64, f64>>> {

        let heatmap = HeatMap::new(
            self.x.to_owned().into_raw_vec(),
            self.y.to_owned().into_raw_vec(),
            self.delta.to_owned().into_raw_vec()
        );
        let traces = vec![heatmap];

        traces
    }

    pub fn delta_as_percent(self)-> Self {
        let delta = (&self.reference_data - &self.comparison_data) / &self.reference_data * 100.;

        ParityContour { reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x, 
            y: self.y, 
            delta}
    }

    pub fn delta_as_difference(self) -> Self {
        let delta = &self.reference_data - &self.comparison_data;

        ParityContour { reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x, 
            y: self.y, 
            delta}
    }

    pub fn delta_as_absolute_difference(self) -> Self {
        let delta_vec: Vec<f64> = (&self.reference_data - &self.comparison_data).into_iter().map(f64::abs).collect();
        let delta: Array2<f64> = Array2::from_shape_vec(self.reference_data.raw_dim(), delta_vec).unwrap();
        ParityContour { reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x, 
            y: self.y, 
            delta}
    }
}