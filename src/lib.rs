
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
extern crate numpy;
#[macro_use(s)]
extern crate ndarray;
use ndarray::prelude::*;
use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn ,PyReadonlyArray1, ToPyArray};



/// A Python module implemented in Rust.
#[pymodule]
fn rustAnalyser(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(vectorfield))?;
    m.add_wrapped(wrap_pyfunction!(alive))?;
    m.add_wrapped(wrap_pyfunction!(occupancy_plot1D))?;
    Ok(())
}

#[pyfunction]
    fn vectorfield<'py>(_py: Python<'py>,
                    filename: &str, //filename of hdf5 file
                    cells : PyReadonlyArray1<f64>,    //number of cells to store vec-data
                    min_time: f64,  //where to start the averaging
                    max_time: f64,  //where to end the averaging
                    dimensions: PyReadonlyArrayDyn<f64>,    // Region where to look at, rest ignored
                    norm_on: bool,  //normalise the size of the vectors
                    radius: PyReadonlyArray1<f64>,    // include a radius, only available for sim-data
                    particle_id: PyReadonlyArray1<i64>
                    ) -> (&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>) {
    // Opening hdf5 file
    let file =hdf5::File::open( filename ).expect("Error reading hdf5 file in rust");

    //read the number of timesteps inside this hdf5file
    let mut timesteps:u64 = timesteps(&file);

    let array = file.dataset("dimensions").unwrap().read_2d::<f64>().unwrap();
    let min_array = array.slice(s![0,..]).to_owned();
    let max_array = array.slice(s![1,..]).to_owned();

        let dimensions = dimensions.as_array().to_owned();
    let cells : Array1<f64> = cells.as_array().to_owned();
    let particle_id : Array1<i64> = particle_id.as_array().to_owned();
    let radius : Array1<f64> = radius.as_array().to_owned();
        //let cells_int =
    //before going through timestep, implement:
    // dimension check?



    let cell_size: Array1<f64> = array![(max_array[0usize]-min_array[0usize])/(cells[0usize]), (max_array[1usize]-min_array[1usize])/(cells[1usize]), (max_array[2usize]-min_array[2usize])/(cells[2usize]) ];

    //initiate needed 2d arrays:
    let mut v_x_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize,cells[0usize].floor() as usize ));
    let mut v_y_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[1usize].floor() as usize ));
    let mut v_z_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[0usize].floor() as usize ));

    //array to count how many particles found per cell
    let mut n_x_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize,cells[0usize].floor() as usize ));
    let mut n_y_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[1usize].floor() as usize ));
    let mut n_z_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[0usize].floor() as usize ));

    for timestep in 0..timesteps-1 {

        let name: String  = "timestep ".to_string() +  &timestep.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if ( current_time < min_time){
            continue;
        }
        if ( current_time > max_time){
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group.dataset("position").expect( "error").read_2d::<f64>().unwrap();
        let velocitys = group.dataset("velocity").expect( "error").read_2d::<f64>().unwrap();
        let ID = group.dataset("particleid").expect( "error").read_1d::<f64>().unwrap();
        let rad_array = group.dataset("radius").expect( "error").read_1d::<f64>().unwrap();
        let particles = positions.len()/3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in (0..particles){
            //check if this particle is fitting the criteria
            if !check_id(ID[particle] as usize ,&particle_id) {continue}
            if !check_radius(rad_array[particle] as f64 ,&radius) {continue}
            let position = positions.slice(s![particle,..]).to_owned();
            let velocity = velocitys.slice(s![particle,..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x : f64 = position[0usize] - min_array[0usize];
            let y : f64 = position[1usize] - min_array[1usize];
            let z : f64 = position[2usize] - min_array[2usize];

            //velocitys
            let vx : f64 = velocity[0usize];
            let vy : f64 = velocity[1usize];
            let vz : f64 = velocity[2usize];

            // check if the current particle position falls into the specified dimension
            // still needs to be done
            if position[0] > dimensions[0usize] ||
                position[1] > dimensions[1usize] ||
                position[2] > dimensions[2usize] {println!("This is out")}

            // find the cell indice where particle is right now

            let i :usize = ( x /  cell_size[0usize] ).floor() as usize;
            let j :usize = ( y /  cell_size[1usize] ).floor() as usize;
            let k :usize = ( z /  cell_size[2usize] ).floor() as usize;
            if i == cells[0usize] as usize || j ==cells[1usize] as usize|| k == cells[2usize] as usize {continue}


            v_x_grid[[k,i]] = v_x_grid[[k,i]] + vx;
            v_z_grid[[k,i]]= v_z_grid[[k,i]] + vz;
            v_y_grid[[k,j]] = v_y_grid[[k,j]] + vy;

            n_x_grid[[k,i]] = n_x_grid[[k,i]] + 1.0;
            n_z_grid[[k,i]]= n_z_grid[[k,i]] + 1.0;
            n_y_grid[[k,j]] = n_y_grid[[k,j]] + 1.0;
            
        }
    }

    v_x_grid = v_x_grid / n_x_grid;
    v_y_grid = v_y_grid / n_y_grid;
    v_z_grid = v_z_grid / n_z_grid;
    let (sx,sy) = meshgrid(Array::linspace(0.0,cells[0usize]*cell_size[0usize],cells[0usize] as usize ),
                                                     Array::linspace(0.0,cells[2usize]*cell_size[2usize],cells[2usize] as usize));
    if norm_on{
        let norm_arr = norm(&v_x_grid,&v_y_grid,&v_z_grid).to_owned();
        v_x_grid = v_x_grid / &norm_arr;
        v_y_grid = v_y_grid / &norm_arr;
        v_z_grid = v_z_grid / &norm_arr;
    }
    file.close();

    (v_x_grid.into_pyarray(_py).to_dyn(),
            v_y_grid.into_pyarray(_py).to_dyn(),
            v_z_grid.into_pyarray(_py).to_dyn(),
            sx.into_pyarray(_py).to_dyn(),
            sy.into_pyarray(_py).to_dyn()
            )


}




#[pyfunction]
fn alive(){
    //chekc if rust is "alive"
        println!("True");
}



#[pyfunction]
fn occupancy_plot1D<'py>(py: Python<'py>,
                         filename: &str,
                         radius: PyReadonlyArray1<f64>,
                         particle_id: PyReadonlyArray1<i64>,
                         clouds: bool,
                         axis:usize,
                         norm: bool,
                         min_time: f64,
                         cells: f64
                                    )-> (&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>){
    /*
    function to calculate the time averaged occupancy plot of a particle system


     */
    let file =hdf5::File::open( filename ).expect("Error reading hdf5 file in rust");
    let timesteps = timesteps(&file);

    let array_temp = file.dataset("dimensions").unwrap().read_2d::<f64>().unwrap();
    let min_array: Array1<f64> = array_temp.slice(s![0,..]).to_owned();
    let max_array:Array1<f64> = array_temp.slice(s![1,..]).to_owned();
    let cell_size: Array1<f64> = (&max_array-&min_array)/cells;
    let particle_id : Array1<i64> = particle_id.as_array().to_owned();
    let radius : Array1<f64> = radius.as_array().to_owned();
    let mut occu: Array1<f64> = Array1::zeros((cells as usize));
    let mut array: Array1<f64> = Array1::linspace(0.0, (&max_array[axis]-&min_array[axis]) , cells as usize);
    let dt = get_dt(&file);
    for timestep in 0..timesteps-1 {
        let name: String  = "timestep ".to_string() +  &timestep.to_string();
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if ( current_time < min_time){
            continue;
        }

        let positions = group.dataset("position").expect( "error").read_2d::<f64>().unwrap();
        let velocitys = group.dataset("velocity").expect( "error").read_2d::<f64>().unwrap();

        let rad_array = group.dataset("radius").expect( "error").read_1d::<f64>().unwrap();
        let particles = positions.len()/3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in (0..particles){
            //if !check_id(ID[particle] as usize ,&particle_id) {continue}
            if !check_radius(rad_array[particle] as f64 ,&radius) {continue}
            let position = positions.slice(s![particle,..]).to_owned();
            let velocity = velocitys.slice(s![particle,..]).to_owned();
            //reset the position. the lowest value should be at 0,0,0
            let x : f64 = position[axis] - min_array[axis];
            let vel: f64 = velocity[axis].abs();

            //find current cell location
            let cell_id = find_closest(&array,x);

            //calculate the time this particle spent in the cell
            let mut time_spent = cell_size[axis]/vel;
            if time_spent > dt {
                time_spent = dt;
            }
            occu[cell_id] = occu[cell_id] + time_spent;
        }

    }
    if norm {
        let mut xmax = 0;
        for x in 0..occu.len(){
            if occu[x] > occu[xmax]{xmax = x;}
        }
        let max_num = occu[xmax];
        for x in 0..occu.len(){
            occu[x] = occu[x] / max_num;
        }

    }
    file.close();
    (occu.to_pyarray(py).to_dyn(), array.to_pyarray(py).to_dyn())
}




fn check_id(id: usize, var: &Array1<i64>) -> bool {
    let mut ret_val = false;
    if (var[0usize] == -1 && var[1usize] == -1) { ret_val = true; }
    else {
        if id >= var[0usize] as usize && id <= var[1usize] as usize {
            ret_val = true;
        }

    }
    ret_val

}
fn check_radius(id: f64, var: &Array1<f64>) -> bool {
    let mut ret_val = false;
    //println!("{:?}",var[1usize]);
    if (var[0usize] == -1.0 && var[1usize] == -1.0 ) { ret_val = true; }
    else {
        if id >= var[0usize] && id < var[1usize]{
            ret_val = true;
        }

    }
    //println!("{:?}",ret_val);
    ret_val


}

fn meshgrid(x: ndarray::Array1<f64>, y: ndarray::Array1<f64>)-> (ndarray::Array2<f64>,ndarray::Array2<f64>) {
    let mut xx = ndarray::Array2::<f64>::zeros(( y.len(), x.len() ));
    let mut yy = ndarray::Array2::<f64>::zeros(( y.len(), x.len() ));

    for idx in (0..x.len()) {
        for idy in (0..y.len()){
            xx[[idy,idx]] = x[idx];
            yy[[idy,idx]] = y[idy];

        }


    }
    return (xx, yy)

}

fn norm(arr1: &Array2<f64>,arr2: &Array2<f64>,arr3:&Array2<f64>)-> Array2<f64>{
    let mut norm_array: Array2<f64> = Array2::zeros((arr1.shape()[0usize],arr1.shape()[1usize]));

    for idx in (0..norm_array.shape()[0usize]) {
        for idy in (0..norm_array.shape()[1usize]) {


            norm_array[[idx,idy]] =  (arr1[[idx,idy]].powf(2.0) + arr2[[idx,idy]].powf(2.0) + arr3[[idx,idy]].powf(2.0) ).sqrt()
        }
    }

    norm_array


}

fn find_closest(arr: &Array1<f64>, num: f64) -> usize{
    let mut id: usize = 0;

    let len_arr = arr.len();
    let mut smallest :f64 = std::f64::MAX;
    for x in 0..len_arr{
        if (arr[x]-num).abs() < smallest{
            smallest = (arr[x]-num).abs();
            id = x;

        }
    }
    return id
}

fn timesteps(file: &hdf5::File)->u64{
    let mut timesteps:u64 = 0 ;
    let vec = file.member_names().unwrap();
    for x in file.member_names().unwrap(){
        if x.contains("timestep") {
            timesteps += 1;

        }
    }
    timesteps
}


fn get_dt(file: &hdf5::File)->f64{
    let t1 = file.group("timestep 0").unwrap().dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
    let t2 = file.group("timestep 1").unwrap().dataset("time").unwrap().read_raw::<f64>().unwrap()[0];

    t2-t1

}