mod datamanager;
//use std::time::{Duration, Instant};
mod base;

pub mod functions;
pub mod utilities;

use datamanager::pdata::PData;
use datamanager::tdata::TData;
mod quiver_funcs;

/*
fn main() {

    println!("Welcome to uPPPP!\nTesting current version...!\nTesting PData");
    let now = Instant::now();
    let mut pdata = PData::new("TEST/HSM_Glass_2l_250.hdf5");
    pdata.stats();

    pdata.test();
    println!("End time: {}", now.elapsed().as_millis());
    println!("Check completed. No errors found in PData!\n\nChecking TData");//lol
    let now = Instant::now();
    let mut tdata = TData::new("TEST/drum.hdf5");
    tdata.stats();

    tdata.test();
    println!("End time: {}", now.elapsed().as_millis());
    println!("Check completed. No errors found in TData!");


} */

fn main() {
    let mut pdata = PData::new("tests/fixtures/test_real_FB.hdf5");
    pdata.test();
    let mut tdata = TData::new("tests/fixtures/drum.hdf5");
    tdata.test();
}
