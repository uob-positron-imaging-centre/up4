use ndarray_stats::QuantileExt;
use plotly::color::NamedColor;
use plotly::common::Line;
use plotly::common::Marker;
use plotly::common::MarkerSymbol;
use plotly::common::Mode;
use plotly::Scatter;

use crate::GridFunctions3D;
use crate::VectorGrid;

// TODO we need to tell off users for unequal inputs
// BUG the plot is drawn wrong
pub struct ParityPlot {
    reference_data: Vec<f64>,
    comparison_data: Vec<f64>,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

impl ParityPlot {
    pub fn from_vector_grids(
        reference_grid: VectorGrid,
        comparison_grid: VectorGrid,
    ) -> ParityPlot {
        let capacity = reference_grid.get_data().len() * 3;
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64> = Vec::with_capacity(capacity);

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
        let xmax = if xmax_reference > xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference > ymax_comparison {
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

        ParityPlot {
            reference_data,
            comparison_data,
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn from_vector_grids_depth_averaged(
        reference_grid: VectorGrid,
        comparison_grid: VectorGrid,
        axis: usize,
    ) -> ParityPlot {
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
        let xmax = if xmax_reference > xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference > ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };

        let ref_u = reference_grid.data[i].collapse(axis);
        let ref_v = reference_grid.data[j].collapse(axis);

        let comp_u = comparison_grid.data[i].collapse(axis);
        let comp_v = comparison_grid.data[j].collapse(axis);

        let capacity = ref_u.len() + ref_v.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64> = Vec::with_capacity(capacity);

        for el in ref_u {
            reference_data.push(el);
        }
        for el in ref_v {
            reference_data.push(el);
        }

        for el in comp_u {
            comparison_data.push(el);
        }
        for el in comp_v {
            comparison_data.push(el);
        }

        ParityPlot {
            reference_data,
            comparison_data,
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn from_vector_grids_single_plane(
        reference_grid: VectorGrid,
        comparison_grid: VectorGrid,
        axis: usize,
        index: usize,
    ) -> ParityPlot {
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
        let xmax = if xmax_reference > xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference > ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };

        let ref_u = reference_grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let ref_v = reference_grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);

        let comp_u = comparison_grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let comp_v = comparison_grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);

        let capacity = ref_u.len() + ref_v.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64> = Vec::with_capacity(capacity);

        for el in ref_u {
            reference_data.push(el);
        }
        for el in ref_v {
            reference_data.push(el);
        }

        for el in comp_u {
            comparison_data.push(el);
        }
        for el in comp_v {
            comparison_data.push(el);
        }

        ParityPlot {
            reference_data,
            comparison_data,
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn from_grids(
        reference_grid: Box<dyn GridFunctions3D>,
        comparison_grid: Box<dyn GridFunctions3D>,
    ) -> ParityPlot {
        let capacity = reference_grid.get_data().len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64> = Vec::with_capacity(capacity);

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
        let xmax = if xmax_reference > xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference > ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        for el in reference_grid.get_data() {
            reference_data.push(*el);
        }

        for el in comparison_grid.get_data() {
            comparison_data.push(*el);
        }

        ParityPlot {
            reference_data,
            comparison_data,
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn from_grids_depth_averaged(
        reference_grid: Box<dyn GridFunctions3D>,
        comparison_grid: Box<dyn GridFunctions3D>,
        axis: usize,
    ) -> ParityPlot {
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
        let xmax = if xmax_reference > xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference > ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        let collapsed_ref = reference_grid.collapse(axis);
        let collapsed_comp = comparison_grid.collapse(axis);

        let capacity = collapsed_ref.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64> = Vec::with_capacity(capacity);

        for el in collapsed_ref {
            reference_data.push(el);
        }

        for el in collapsed_comp {
            comparison_data.push(el)
        }

        ParityPlot {
            reference_data,
            comparison_data,
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn from_grids_single_plane(
        reference_grid: Box<dyn GridFunctions3D>,
        comparison_grid: Box<dyn GridFunctions3D>,
        axis: usize,
        index: usize,
    ) -> ParityPlot {
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
        let xmax = if xmax_reference > xmax_comparison {
            xmax_reference
        } else {
            xmax_comparison
        };
        let ymin = if ymin_reference < ymin_comparison {
            ymin_reference
        } else {
            ymin_comparison
        };
        let ymax = if ymax_reference > ymax_comparison {
            ymax_reference
        } else {
            ymax_comparison
        };

        let collapsed_ref = reference_grid
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let collapsed_comp = comparison_grid
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);

        let capacity = collapsed_ref.len();
        let mut reference_data: Vec<f64> = Vec::with_capacity(capacity);
        let mut comparison_data: Vec<f64> = Vec::with_capacity(capacity);

        for el in collapsed_ref {
            reference_data.push(el);
        }

        for el in collapsed_comp {
            comparison_data.push(el);
        }

        ParityPlot {
            reference_data,
            comparison_data,
            xmin,
            xmax,
            ymin,
            ymax,
        }
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

// TODO tests
#[cfg(test)]
mod test {

    use super::*;

    // Helper functions

    // Tests
}
