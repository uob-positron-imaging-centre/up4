use std::collections::HashMap;
use ndarray::prelude::*;
pub struct Buffer{
     data: HashMap<String,ArrayD<f64>>,
     range: Array1<isize>,
     usable: bool,
     updated: Vec<String>,
     pub buffersize: usize,
     buffersteps: usize,
}


impl Buffer {
    /// Creates a new buffer.
    pub fn new(buffersize: usize) -> Buffer {
        Buffer {
            data: HashMap::new(),
            range: array![-(buffersize as isize),-1].mapv(|x| x as isize),
            usable: false,
            updated: Vec::<String>::new(),
            buffersize: buffersize,
            buffersteps:0,
          }
    }

    // Create a test buffer filled with some data
    pub fn test() -> Buffer {
        let mut x = HashMap::new();
        x.insert("test data".to_string(), Array2::<f64>::zeros((10,10)).into_dyn());
        x.insert("test data2".to_string(), Array3::<f64>::zeros((10,10,3)).into_dyn());
        let mut range = array![0,10].mapv(|x| x as isize);
        Buffer {
            data: x,
            range: range,
            usable: true,
            updated: Vec::<String>::new(),
            buffersize:10,
            buffersteps: 0,
        }
    }

    // check if a buffer has a given timestep
    pub fn has(&self,timestep: u64)->bool{
        if (self.range[0] <= timestep as isize) & (self.range[1] >= timestep as isize){
            return true
        }
        else{
            return false
        }
    }

    // return a array from a given dataset at a given timestep
    pub fn get<D>(&self, name: &str, timestep: u64)->ArrayView<f64,D>
        where D: Dimension
        {

        if !self.usable{
            panic!("This buffer reader is not updated and may contain wrong information!")
        }

        if timestep as isize > self.range[self.range.len() -1]{
            panic!("Buffer got access request to timestemp which is not loaded.")
        }

        let arr = self.data.get(name).unwrap();
        let result = if arr.ndim() == 2{
             arr.slice(s![timestep  as usize -self.range[0] as usize,..]).into_dyn()
        } else if arr.ndim() == 3{
             arr.slice(s![timestep as usize  -self.range[0] as usize ,..,..]).into_dyn()
        }else{
            panic!("Dimension wrong")
        };

        result.into_dimensionality::<D>()
        .expect(&format!("BufferError"))
    }

    pub fn range(&self)->&Array1<isize>{
        &self.range
    }
    // To
    pub fn update(&mut self, name: &str, array: ArrayD<f64>, range: Array1<isize>){
        // Check if the global update started
        // Update time range of new update or check if new update is
        // compatible with new temp
        // Flag buffer as "unreadable" if global update started
        if self.updated.len() >=1{
            if range != self.range{
                panic!("Buffer got data that is not in sync with other data")
            }
        }else if self.updated.len() == 0{
            // sticking a new array inside
            if range != self.range{
                self.range = range;
                self.usable = false;
            }

        }
        // Save the name of the updated Data
        // Multiple updates of one data possible
        // without interfereing or bugging global
        // process
        if !self.updated.iter().any(|i| i==name){
            self.updated.push(name.to_string());
        }
        // save the data in current map-section
        // or insert new map-section
        if self.data.contains_key(name){
            *self.data.get_mut(name).unwrap() = array;
        }else{
            self.data.insert(name.to_string(),array);
        }
        // check if global update is finished
        if self.data.keys().len() == self.updated.len() as usize{
            self.usable = true;
            self.buffersteps += 1;
        }
        // refresh names if global update finished
        // must be here if someone updates a set without
        // changing the timespace (whyever...)

        if self.usable{
            self.updated = Vec::<String>::new();
        }
    }

}
