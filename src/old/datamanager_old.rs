use std::collections::HashMap;

use crate::functions;
use ndarray::prelude::*;

mod MP_SIM;
mod SP_EXP;
mod utilities;
mod buffer;
use buffer::Buffer;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyType};
use pyo3::ffi;


pub trait Granular {
    fn vectorfield(&mut self)->Array1<f64> {
        println!("test");
        Array1::<f64>::zeros(1)
    }
}




pub trait DataManager {
    fn get<D>(&mut self, name: &str, timestep: usize) -> ArrayView<f64, D> where D: Dimension;

    fn position(&self, timestep: usize)->ArrayView2<f64>{
        self.get::<Ix2>("position", timestep)
    }

    fn velocity(&self, timestep: usize)->ArrayView2<f64>{
        self.get::<Ix2>("velocity", timestep)
    }

    fn radius(&self, timestep: usize) -> ArrayView1<f64>{
        self.get::<Ix1>("radius", timestep)
    }

    fn particleid<'r>(&'r self,timestep: usize)->ArrayView1<'r,f64>{
        self.get::<Ix1>("particleid", timestep)
    }

    fn clouds<'r>(&'r self,timestep: usize)->ArrayView1<'r,f64>{
        self.get::<Ix1>("ppcloud", timestep)
    }


}


#[pyclass]
pub struct PData{
    // A data managing system for HDF5 files in the
    // H5Part format or H5TimePart information
    file: hdf5::File,

    buffer: Vec<Timestep>,
    range: (usize, usize),
    usable: bool,
    updated: Vec<String>,
    pub buffersize: usize,
    buffersteps: usize,
}




impl Granular for PData {

}



impl Drop for PData {
    fn drop(&mut self){
        self.file.close();
    }
}





#[derive(Debug, Default)]
struct Timestep {
    position: Array2<f64>,
    velocity: Array2<f64>,
    radius: Array1<f64>,
    particleid: Array1<f64>,
    clouds: Array1<f64>,
}
