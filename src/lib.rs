
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
extern crate numpy;
#[macro_use(s)]
extern crate ndarray;
use ndarray::prelude::*;
use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn ,PyReadonlyArray1, ToPyArray};



/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyAnalyser(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(vectorfield))?;

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
                    radius: f64     // include a radius, only available for sim-data
                    ) -> (&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>,&'py PyArrayDyn<f64>) {
    // Opening hdf5 file

    println!("Opening file {}", filename);
    let file =hdf5::File::open( filename ).expect("Error reading hdf5 file in rust");

    let timesteps = file.len();

    let array = file.dataset("dimensions").unwrap().read_2d::<f64>().unwrap();
    let min_array = array.slice(s![0,..]).to_owned();
    let max_array = array.slice(s![1,..]).to_owned();

        let dimensions = dimensions.as_array().to_owned();
        let cells : Array1<f64> = cells.as_array().to_owned();
        //let cells_int =
    //before going through timestep, implement:
    // dimension check?



    let cell_size: Array1<f64> = array![max_array[0usize]/(cells[0usize]), max_array[1usize]/(cells[1usize]), max_array[2usize]/(cells[2usize]) ];
    println!("{:?},{:?}",max_array,cells);
    //initiate needed 2d arrays:
    let mut v_x_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize,cells[0usize].floor() as usize ));
    let mut v_y_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[1usize].floor() as usize ));
    let mut v_z_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[0usize].floor() as usize ));

    //array to count how many particles found per cell
    let mut n_x_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize,cells[0usize].floor() as usize ));
    let mut n_y_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[1usize].floor() as usize ));
    let mut n_z_grid = ndarray::Array2::<f64>::zeros(( cells[2usize].floor() as usize ,cells[0usize].floor() as usize ));

    for timestep in 0..timesteps-1 {
        println!("hello start timestep");
        let name: String  = "timestep ".to_string() +  &timestep.to_string();
        println!("max time {:?}",timesteps);
        let group = file.group(&name).unwrap();
        let current_time = group.dataset("time").unwrap().read_raw::<f64>().unwrap()[0];
        // check if timestep is in the timeframe given
        if ( current_time < min_time){
            //println!("{} is smaller then {}",current_time, min_time);
            continue;
        }
        if ( current_time > max_time){
            //println!("{} is bigger then {}",current_time, max_time);
            continue;
        }
        //let dataset = group.dataset("position").expect( "error");
        let positions = group.dataset("position").expect( "error").read_2d::<f64>().unwrap();
        let velocitys = group.dataset("velocity").expect( "error").read_2d::<f64>().unwrap();
        let particles = positions.len()/3;
        // loop over all particles in this timestep, calculate the velocity vector and add it to the
        // vectorfield array
        for particle in (0..particles){
            println!("start particle {:?},{:?}",particle,particles);
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
            if position[0] > dimensions[0usize] ||
                position[1] > dimensions[1usize] ||
                position[2] > dimensions[2usize] {println!("This is out")}

            // find the cell indice where particle is right now

            let i :usize = ( x /  cell_size[0usize] ).floor() as usize;
            let j :usize = ( y /  cell_size[1usize] ).floor() as usize;
            let k :usize = ( z /  cell_size[2usize] ).floor() as usize;
            //println!("{:?},{:?},{:?},{:?},{:?},{:?}",i,j,k,x,y,z);
            v_x_grid[[k,i]] = v_x_grid[[k,i]] + vx;
            v_z_grid[[k,i]]= v_z_grid[[k,i]] + vz;
            v_y_grid[[k,j]] = v_y_grid[[k,j]] + vy;

            n_x_grid[[k,i]] = n_x_grid[[k,i]] + 1.0;
            n_z_grid[[k,i]]= n_z_grid[[k,i]] + 1.0;
            n_y_grid[[k,j]] = n_y_grid[[k,j]] + 1.0;
            println!("end particle");
        }
        println!("end timestep");
    }
    v_x_grid = v_x_grid / n_x_grid;
    v_y_grid = v_y_grid / n_y_grid;
    v_z_grid = v_z_grid / n_z_grid;

    let (sx,sy) = meshgrid(Array::range(0.0,cells[0usize]*cell_size[0usize],cell_size[0usize]),
                                                     Array::range(0.0,cells[2usize]*cell_size[2usize],cell_size[2usize]));


    println!("hello");
    file.close();
    return (v_x_grid.to_pyarray(_py).to_dyn(),
            v_y_grid.to_pyarray(_py).to_dyn(),
            v_z_grid.to_pyarray(_py).to_dyn(),
            sx.to_pyarray(_py).to_dyn(),
            sy.to_pyarray(_py).to_dyn()
    );


}


fn meshgrid(x: ndarray::Array1<f64>, y: ndarray::Array1<f64>)-> (ndarray::Array2<f64>,ndarray::Array2<f64>) {
    let mut xx = ndarray::Array2::<f64>::zeros(( x.len(), y.len() ));
    let mut yy = ndarray::Array2::<f64>::zeros(( x.len(), y.len() ));

    for idx in (0..x.len()) {
        for idy in (0..y.len()){
            xx[[idx,idy]] = x[idx];
            yy[[idx,idy]] = y[idx];

        }


    }
    return (xx, yy)

}
