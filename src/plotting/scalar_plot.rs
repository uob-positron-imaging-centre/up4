

use ndarray::{Array1, Array2, Array3};
use plotly::{HeatMap, Trace, Plot, Layout};
use crate::{GridFunctions3D, component_data_selector};
use crate::utilities::maths::meshgrid;
pub struct ScalarPlotter {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    zdata: Array1<f64>,
    scalar_data: Array3<f64>
}

impl ScalarPlotter {
    pub fn new(grid: Box<dyn GridFunctions3D>) -> ScalarPlotter {
        let xdata: Array1<f64> = grid.get_xpositions().to_owned();
        let ydata: Array1<f64> = grid.get_ypositions().to_owned();
        let zdata: Array1<f64> = grid.get_zpositions().to_owned();
        let scalar_data: Array3<f64> = grid.get_data().to_owned();
        return ScalarPlotter { 
            xdata: xdata, 
            ydata: ydata, 
            zdata: zdata, 
            scalar_data: scalar_data, 
        }
    }

    // TODO heatmap wrapping
    pub fn scalar_map_plot(&self, axis: usize, index: usize) -> Vec<Box<HeatMap<f64, f64, f64>>> {
        let (xaxis, yaxis) = self.axis_selector(axis);
        let (xaxis, yaxis) = meshgrid(xaxis, yaxis);
        let plot_data = component_data_selector(self.scalar_data.to_owned(), axis, index);
        let heatmap = HeatMap::new(xaxis.into_raw_vec(), yaxis.into_raw_vec(), plot_data.into_raw_vec());
        let trace = vec![heatmap];
        return trace
    }
    // TODO contour wrapping
    pub fn scalar_contour_plot(&self, axis: usize, index: usize)  {
        
    }

    //FIXME doc
    fn axis_selector(&self, axis: usize) -> (Array1<f64>, Array1<f64>) {
        match axis {
            // yz view
            0 => {
                let xcomponent = self.ydata.to_owned();
                let ycomponent = self.zdata.to_owned();
                return (xcomponent, ycomponent)
            }
            // xz view
            1 => {
                let xcomponent = self.xdata.to_owned();
                let ycomponent = self.zdata.to_owned();
                return (xcomponent, ycomponent)
            }
            // xy view
            2 => {
                let xcomponent = self.xdata.to_owned();
                let ycomponent = self.ydata.to_owned();
                return (xcomponent, ycomponent)
            }
            // panic
            _ => panic!("axis value must be either 0, 1 or 2!")
        };
    }

    // FIXME documentation
    pub fn plot(&self, traces: Vec<Box<dyn Trace>>, layout: Layout, show: bool) -> Plot {
        let mut plot: Plot = Plot::new();
        //use local render version
        plot.use_local_plotly();
        for trace in traces{
            plot.add_trace(trace);
        }
        plot.set_layout(layout);
        if show{
            plot.show();
        }
        return plot
    }
}

// TODO slice plots
// e.g. in an axis plot slices of the grid every n indices

// TODO 3 panelled projection plot - xy, xz, yz depth-averaged quantities

// TODO see if slice plots lend themselves well to this format too

// TODO interactive slice plots (draggable)