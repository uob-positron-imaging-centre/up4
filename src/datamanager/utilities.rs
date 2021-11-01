use ndarray::prelude::*;

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
