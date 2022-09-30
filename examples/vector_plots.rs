use upppp_rust::VectorGrid;
use plotly::common::ColorBar;
use plotly::common::{ColorScale::Palette, ColorScalePalette};
use plotly::layout::{Axis};
use plotly::{Plot, Layout};
use plotly::Trace;

use upppp_rust::{CartesianGrid3D, GridFunctions3D};
use upppp_rust::plotting::vector_plot::VectorPlotter;
use std::f64::consts;
use upppp_rust::grid::Dim;

fn unit_vector_2d(){
    /* 
    This example creates 4 vortices from the equations
        u(x, y) =  sin(x)*cos(y)
        v(x, y) = -sin(y)*cos(x)
    where u and v are components of a vector at positions x and y.

    In this example we see how to create the VectorData2D struct and 
    visualise it as a quiver plot, viewing the arrows as unit vectors
    and using a heatmap to colour the background based on the vector norms.
    */
    
    const PI: f64 = consts::PI;
    const PTS: usize = 30; //number of points
    // create values for vortices
    // create the grids for the data
    let limit: Dim = Dim::ThreeD([[0., 2.*PI], [0., 2.*PI], [0., 0.]]);
    let cells: [usize; 3] = [PTS, PTS, 1];
    let mut xgrid: Box<dyn GridFunctions3D> = Box::new(CartesianGrid3D::new(cells, limit));
    let mut ygrid: Box<dyn GridFunctions3D> = xgrid.clone();
    let mut zgrid: Box<dyn GridFunctions3D> = xgrid.clone();
    // assign values based on their positions
    let posx = xgrid.get_xpositions().to_owned();
    let posy = xgrid.get_ypositions().to_owned();
    let posz = xgrid.get_zpositions().to_owned();
    for i in 0..PTS {
        for j in 0..PTS {
            let xvalue = posx[i].sin()*posy[j].cos();
            let yvalue = -posy[j].sin()*posx[i].cos();
            xgrid.insert([posx[i], posy[j], posz[0]], xvalue);
            ygrid.insert([posx[i], posy[j], posz[0]], yvalue);
            zgrid.insert([posx[i], posy[j], posz[0]], 0.);
        }
    }
    // create VectorGrid struct
    let mut grid: VectorGrid = VectorGrid::new(xgrid);
    grid.data[1] = ygrid;
    grid.data[2] = zgrid;
    // create VectorData2D struct
    let mut arrows: VectorPlotter = VectorPlotter::new(grid);
    // define properties for plot
    let uniform: bool = true;
    let arrow_scale: Option<f64> = None; // default scaling
    // create arrow traces
    let axis: usize = 2;
    let index: usize = 0;
    let mut traces: Vec<Box<dyn Trace>> = Vec::new();
    let scatter_traces = arrows.create_unit_vector_traces(arrow_scale, uniform, axis, index);
    // set layout
    // FIXME layout settings
    let layout: Layout = Layout::new()
                        .width(800)      
                        .height(800)
                        //.auto_size(true)         
                        .title("2D vortex plot".into());
    let square: bool = true;
    let xaxis: Option<Axis> = Some(Axis::new().title("x position".into()));
    let yaxis: Option<Axis> = Some(Axis::new().title("y position".into()));
    let axes = vec![xaxis, yaxis];
    let smoothing = None;
    let (heatmap, layout) = arrows.create_unit_vector_background(layout, square, axes, smoothing, axis, index);
    // adjust the heatmap colourbar and colourscale
    let color_scale = Palette(ColorScalePalette::Viridis);
    let color_bar = ColorBar::new().title("Vector magnitude".into()); 
    for scatter in scatter_traces {
        traces.push(scatter);
        //traces.push(background);
    }
    traces.push(heatmap.color_scale(color_scale).zmax(1.).zmin(0.).color_bar(color_bar));
    let show: bool = true;
    let plot: Plot = arrows.plot(traces, layout, show);
    
}

fn main() {

   unit_vector_2d();
}
