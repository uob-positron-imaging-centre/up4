use ndarray;
use itertools;
use itertools::izip;
use ndarray::prelude::*;
use std::f64::consts::PI;
use plotly::common::{Fill, Line, Marker, Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{NamedColor, Plot, Scatter};
//define struct to contain raw data for plotting quivers
pub struct ArrowData {
    xdata: Array1<f64>,
    ydata: Array1<f64>,
    udata: Array1<f64>,
    vdata: Array1<f64>,
    xdata_end: Array1<f64>,
    ydata_end: Array1<f64>,
}

impl ArrowData {
//constructor for ArrowData struct
    pub fn new(x:Array2<f64>, y:Array2<f64>, u:Array2<f64>, v:Array2<f64>) -> ArrowData {
        ArrowData {
            xdata: ArrowData::flatten(&x),
            ydata: ArrowData::flatten(&y),
            udata: ArrowData::flatten(&ArrowData::scale(0.3,&u)), 
            vdata: ArrowData::flatten(&ArrowData::scale(0.3,&v)),
            xdata_end: ArrowData::flatten(&x) + ArrowData::flatten(&ArrowData::scale(0.3,&u)), 
            ydata_end: ArrowData::flatten(&y) + ArrowData::flatten(&ArrowData::scale(0.3,&v))
            }
        }
    pub fn flatten(arr:&Array2<f64>) -> Array1<f64>{
    //helper associated function for constructor - flattens a 2D array into a 1D array
        return arr.slice(s![0..arr.shape()[0], 0..arr.shape()[1]]) //create slice of all elements
                .iter() //create iterable
                .copied() //iterate through
                .collect::<Array1<f64>>() //collect into array
    }
    //helper associated function for constructor - scales u and v
    pub fn scale(scale: f64, arr:&Array2<f64>) ->  Array2<f64> {
        //default barb length is 30% of arrow length
        //TODO add different scaling modes
        return scale*arr
    }
    
}

pub fn quiver_barbs(data: &ArrowData) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    //create quiver barbs
    let mut barb_x = Vec::new();
    let mut barb_y = Vec::new();
    for (start, end) in izip!(&data.xdata, &data.xdata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_x.push(tup);    
    }
    for (start, end) in izip!(&data.ydata, &data.ydata_end) {
        let tup: (f64, f64) = (*start, *end);
        barb_y.push(tup);    
    }
    //println!("{:?}", barb_x);
    return (barb_x, barb_y)
}

pub fn gen_quiver_arrows(data: &ArrowData) -> (Vec<(f64, f64, f64, f64)>, Vec<(f64, f64, f64, f64)>) {
//gen the list of x and y values to plot arrows
    
    //default angle is pi/9
    const ANGLE: f64 = PI/9.0; //TODO add this as an optional argument

    //let default scale be 0.5
    const ARROW_SCALE: f64 = 0.5;
    
    //length is simply (x+u) - x = u etc
    //TODO add scaling options/ match what happens to the actual arrow length   
    let arrow_len: Array1<f64> = izip!(&data.vdata,&data.udata).map(|(u,v)| f64::hypot(*u,*v)).collect::<Array1<f64>>()*ARROW_SCALE; 
    
    // get barb angles
    let barb_ang: Array1<f64> = izip!(&data.vdata, &data.udata).map(|(v,u)| f64::atan2(*v,*u)).collect::<Array1<f64>>();
    
    //find angles for both lines of arrow
    let arrow_ang_1: Array1<f64> = &barb_ang + ANGLE;
    let arrow_ang_2: Array1<f64> = &barb_ang - ANGLE;
    
    //do some trig on these
    let sin_ang_1: Array1<f64> = arrow_ang_1.mapv(f64::sin);
    let cos_ang_1: Array1<f64> = arrow_ang_1.mapv(f64::cos);
    let sin_ang_2: Array1<f64> = arrow_ang_2.mapv(f64::sin);
    let cos_ang_2: Array1<f64> = arrow_ang_2.mapv(f64::cos);
    
    //find corresponding components
    let seg_1_x: Array1<f64> = &arrow_len * &cos_ang_1;
    let seg_1_y: Array1<f64> = &arrow_len * &sin_ang_1;
    let seg_2_x: Array1<f64> = &arrow_len * &cos_ang_2;
    let seg_2_y: Array1<f64> = &arrow_len * &sin_ang_2;
    
    //set coordinates of the arrow
    let point_1_x: Array1<f64> = &data.xdata_end - &seg_1_x;
    let point_1_y: Array1<f64> = &data.ydata_end - &seg_1_y;
    let point_2_x: Array1<f64> = &data.xdata_end - &seg_2_x;
    let point_2_y: Array1<f64> = &data.ydata_end - &seg_2_y;
    
    //finally, combine this data into something usable
    let mut arrow_x = Vec::new();
    let mut arrow_y = Vec::new();
    const NAN: f64 = f64::NAN;
    
    for (start, mid, end) in izip!(point_1_x, point_2_x, &data.xdata_end) {
        let tup: (f64, f64, f64, f64) = (start, NAN, mid, *end);
        arrow_x.push(tup);    
    }
    for (start, mid, end) in izip!(point_1_y, point_2_y, &data.ydata_end) {
        let tup: (f64, f64, f64, f64) = (start, NAN, mid, *end);
        arrow_y.push(tup);    
    }
    return (arrow_x, arrow_y)
}

pub fn plot_arrows(data: ArrowData, n: f64) {
     let pts: f64 = n;
     let (barb_x, barb_y) = quiver_barbs(&data);
     let (arrow_x, arrow_y) = gen_quiver_arrows(&data);
     let mut plot = Plot::new();
     plot.use_local_plotly();
     //janky unpacking
     for (x_line, y_line, x_head, y_head) in izip!(barb_x, barb_y, arrow_x, arrow_y) {
        //*_head.1 is the None which i could include if you don't like filled arrowheads...
        let xpl = vec![x_line.0, 
                       x_line.1, 
                       x_head.0,
                       //x_head.1,
                       x_head.2,
                       x_head.3];
                       
        let ypl = vec![y_line.0, 
                       y_line.1, 
                       y_head.0,
                       //y_head.1,
                       y_head.2,
                       y_head.3];

        let trace = Scatter::new(xpl, ypl)
                        .mode(Mode::Lines)
                        .show_legend(false)
                        .fill(Fill::ToSelf)
                        .fill_color(NamedColor::Blue)
                        .line(Line::new().color(NamedColor::Blue));
        plot.add_trace(trace);
     }      
      let layout = Layout::new()
        .title("oh wow it works!!!".into())
        .x_axis(Axis::new().title("Effort".into()).range(vec![0., 2.*PI+PI/pts]))
        .y_axis(Axis::new().title("Reward".into()).range(vec![0., 2.*PI+PI/pts]));
    plot.set_layout(layout);
    plot.show();
}

