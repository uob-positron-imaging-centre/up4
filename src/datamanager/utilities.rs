use crate::types::*;
/***
pub fn dimensions(file: &hdf5::File)->(Array1<f64>,Array1<f64>){
        let array = file
        .dataset("dimensions")
        .expect(&format!(
            "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
            file.filename()
        ))
        .read_2d::<f64>()
        .expect(&format!(
            "Can not read data from \"dimensions\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            file.filename()
        ));
        let min_array = array.slice(s![0, ..]).to_owned();
        let max_array = array.slice(s![1, ..]).to_owned();
        (min_array, max_array)
    }


/// Version finder for HDF5 files in the H5part format
///
/// # Arguments
/// * 'file' -- A hdf5 filehandel
///
/// # Examples
/// ```
/// let file = hdf5::File::open("test.hdf5")
/// let version = find_version(&file)
/// ```
pub fn find_version(file: &hdf5::File)-> f64{
    let attr =  match file.attr("version"){
        Ok(attribute) => match attribute.read_scalar(){
            Ok(scalar) => scalar,
            Err(_) => panic!("Attribute \"version\" does not contain a scalar. File: {}",
                            file.filename()
                            )
        },
        Err(_) => panic!("Attribute \"version\" is not available in file {}. \
                          If you use a old version of HDF5 files, please include \
                          the version with the function TODO!", file.filename()),
    };

    attr

}

pub fn find_type(file: &hdf5::File)-> String{
    //"SP_SIM"--> 1
    //"MP_SIM"--> 2
    //"SP_EXP"--> 3
    //"MP_EXP"--> 4
    let type_: String =  match file.attr("type"){
        Ok(attribute) => match attribute.read_scalar::<hdf5::types::VarLenUnicode>(){
            Ok(scalar) => String::from(scalar.as_str()),
            Err(_) => panic!("Attribute \"version\" does not contain a scalar", )
        },
        Err(_) => panic!("Attribute \"version\" is not available in your file. \
                          If you use a old version of HDF5 files, please include \
                          the version with the function TODO!", ),
    };
    type_
}


*/

fn rotate_position(position: Position, angle: [f64; 3], anchor: [f64; 3]) -> Position {
    let mut new_position = [
        position[0] - anchor[0],
        position[1] - anchor[1],
        position[2] - anchor[2],
    ];
    for axis in 0..3 {
        let (sin, cos) = angle[axis].sin_cos();
        let (x, y, z) = (new_position[0], new_position[1], new_position[2]);
        match axis {
            0 => {
                new_position[1] = y * cos - z * sin;
                new_position[2] = y * sin + z * cos;
            }
            1 => {
                new_position[0] = x * cos + z * sin;
                new_position[2] = -x * sin + z * cos;
            }
            2 => {
                new_position[0] = x * cos - y * sin;
                new_position[1] = x * sin + y * cos;
            }
            _ => unreachable!(),
        }
    }
    [
        new_position[0] + anchor[0],
        new_position[1] + anchor[1],
        new_position[2] + anchor[2],
    ]
}

fn rotate_velocity(velocity: [f64; 3], angle: [f64; 3]) -> [f64; 3] {
    let mut new_velocity = [velocity[0], velocity[1], velocity[2]];
    for axis in 0..3 {
        let (sin, cos) = angle[axis].sin_cos();
        let (x, y, z) = (new_velocity[0], new_velocity[1], new_velocity[2]);
        match axis {
            0 => {
                new_velocity[1] = y * cos - z * sin;
                new_velocity[2] = y * sin + z * cos;
            }
            1 => {
                new_velocity[0] = x * cos + z * sin;
                new_velocity[2] = -x * sin + z * cos;
            }
            2 => {
                new_velocity[0] = x * cos - y * sin;
                new_velocity[1] = x * sin + y * cos;
            }
            _ => unreachable!(),
        }
    }
    [new_velocity[0], new_velocity[1], new_velocity[2]]
}
