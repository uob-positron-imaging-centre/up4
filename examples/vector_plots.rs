use colorous::Gradient;
// This should appear in PR
use plotly::common::{Mode, ColorBar, ColorScale, ColorScalePalette, Line, Marker, Fill, ColorScaleElement};
use plotly::layout::Margin;
use plotly::{Plot, Scatter, Layout};
use ndarray::prelude::*;
use upppp_rust::plotting::VectorData;
use upppp_rust::plotting::vector2d::VectorData2D;
use upppp_rust::{plotting::vector2d, utilities::maths::meshgrid};
use std::f64::consts;

fn simple_color_scatter() {

    let mut plot = Plot::new();
    let gradient = colorous::VIRIDIS;
    let x = vec![0.,1.,0.5,1.,1.];
    let y = vec![0.,1.,1.,0.5,1.];
    let c = 0.5; //pretend norm
    let s = format!("#{:x}",gradient.eval_continuous(c));
    let trace = Scatter::new(x,y)
                        .mode(Mode::Lines)
                        .fill(Fill::ToSelf)
                        .fill_color(&s)
                        .line(Line::new()
                                    //.color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                                    .color(s)
                                    .cauto(true)) 
                                    .show_legend(false);
    plot.add_trace(trace);
    let x = vec![0.,-2.,-1.,-2.,-2.];
    let y = vec![0.,1.,1.,0.5,1.];
    let c = 1.;
    let s = format!("#{:x}",gradient.eval_continuous(c));
    let trace = Scatter::new(x,y)
                        .mode(Mode::Lines)
                        .fill(Fill::ToSelf)
                        .fill_color(&s)
                        .line(Line::new()
                        //.color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                        .color(s)
                        .cauto(true))
                        .show_legend(false);
    plot.add_trace(trace);
    let x = vec![0.,0.];
    let y = vec![0.5,1.];
    let size: usize = 1;
    let trace = Scatter::new(x,y)
                        .mode(Mode::Markers)
                        .marker(Marker::new()
                                    .cmin(0.) //min norm
                                    .cmax(1.) //max norm
                                    .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                                    //.color(Rgb::new(0,0,0))
                                    .color_bar(ColorBar::new())
                        .size(size))
                        .show_legend(false);
    plot.add_trace(trace);
    plot.use_local_plotly();
    plot.show();
}

fn vector_2d(){
    /* 
    This example creates 4 vortices from the equations
        u(x, y) =  sin(x)*cos(y)
        v(x, y) = -sin(y)*cos(x)
    where u and v are components of a vector at positions x and y.

    In this example we see how to create the VectorData2D struct and 
    visualise it as a quiver plot
    */
     
    const PI: f64 = consts::PI;
    const PTS: f64 = 20.; //number of points
    
    // create values for vortices
    let x: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let y: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let (x, y) = meshgrid(x, y);                       
    let u: Array2<f64> = &x.mapv(f64::sin)*&y.mapv(f64::cos);
    let v: Array2<f64> = -&y.mapv(f64::sin)*&x.mapv(f64::cos);

    // create VectorData2D struct
    let arrows: VectorData2D = vector2d::VectorData2D::new(x,y,u,v);

    // define properties for plot
    let arrow_scale: Option<f64> = None; // default scaling
    let colour: Gradient = colorous::VIRIDIS; // set colourmap
    let colour_bounds: Option<(f64, f64)> = None; //Some((0.3, 0.5));
    let traces: Vec<Box<Scatter<f64, f64>>> = arrows.create_plotly_traces(arrow_scale, colour, colour_bounds);
    let mut layout = Layout::new()
                        .width(1000)                
                        .title("Quiver plot".into());
    let square: bool = true;
    let mut axes = Vec::new();
    axes.resize_with(2, || None);
    let mut plot: Plot = arrows.vector_plot(traces, layout, square, axes);
    plot.show();
}

fn main() {
   //simple_color_scatter();
   vector_2d();
}
