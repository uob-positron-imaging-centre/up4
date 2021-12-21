use plotly::common::{Mode, ColorBar, ColorScale, ColorScalePalette, Line, Marker, Fill, ColorScaleElement};
use plotly::{Plot, Scatter};
use ndarray::prelude::*;
use upppp_rust::{plotting::vector2d, utilities::maths::meshgrid};
use vector2d::{BoundMode, ScaleMode};
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
                                    .cauto(true)
                                    
 
                                )
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
                                    .cauto(true)
                                    

                                )
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
    // generate 4 vortices and plot
    const PI: f64 = consts::PI;
    const PTS: f64 = 60.; //number of points
    let valx: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let valy: Array1<f64> = Array::range(0.,2.*PI+PI/PTS,2.*PI/PTS);
    let (x, y) = meshgrid(valx,valy);                       
    let u: Array2<f64> = &x.mapv(f64::sin)*&y.mapv(f64::cos);
    let v: Array2<f64> = -&y.mapv(f64::sin)*&x.mapv(f64::cos);
    let arrows = vector2d::ArrowData::new(x,y,u,v,ScaleMode::Default);
    let arrow_scale: Option<f64> = None;
    let mode: BoundMode = BoundMode::None;
    let colour = colorous::VIRIDIS;
    let palette = ColorScalePalette::Viridis;
    let colour_bounds = None; //Some((0.3, 0.5));
    let traces = vector2d::trace_arrows_plotly(arrows,arrow_scale, mode, colour, palette, colour_bounds);
    
    let layout = plotly::layout::Layout::new()
                    .title("Quiver plot".into());
    let plot = vector2d::plot(traces, layout, true); 
    plot.show();
}

fn main() {
   //simple_color_scatter();
   vector_2d();
}
