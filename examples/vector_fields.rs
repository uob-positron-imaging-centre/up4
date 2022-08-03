use std::env;

use upppp_rust::base::{CartesianGrid, GridTraits};



// my way
fn dan_grid(){
    // 1D
    let cells1: Vec<usize> = vec![10];
    let limits1: Vec<f64> = vec![0., 2.];
    let grid1: CartesianGrid = CartesianGrid::from_1d(cells1, limits1);
    println!("{:?}", grid1);
    
    // 1D
    let cells2: Vec<usize> = vec![10, 10];
    let limits2: Vec<f64> = vec![0., 2., 0., 3.];
    let grid2: CartesianGrid = CartesianGrid::from_2d(cells2, limits2);
    println!("{:?}", grid2);

    // 1D
    let cells3: Vec<usize> = vec![10, 10, 10];
    let limits3: Vec<f64> = vec![0., 2., 0., 3., 0., 4.];
    let grid3: CartesianGrid = CartesianGrid::from_3d(cells3, limits3);
    println!("{:?}", grid3);
}

// dom's way
fn dom_grid(){

}

// check for feature parity
fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    //dom_grid();
    dan_grid();
}