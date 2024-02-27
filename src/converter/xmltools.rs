use std::path::PathBuf;

use regex::CaptureMatches;
use vtkio::model::*;

use crate::{print_debug, VTKType};
// TODO watch out for vtkio updates concerning raw encoding for appended binary data
pub fn get_field<T>(filename: &str, field: &str, vtk_type: &VTKType) -> Vec<T>
where
    T: vtkio::model::Scalar,
{
    let filepath = PathBuf::from(filename);
    let mut file = match Vtk::import(filename) {
        Ok(file) => file,
        Err(e) => panic!("Failed to load file: {:?}", e),
    };

    file.load_all_pieces()
        .unwrap_or_else(|_| panic!("{} load failed", field));
    let mut output_vector = Vec::new();
    match vtk_type {
        VTKType::PolyData => {
            if let DataSet::PolyData { pieces, .. } = file.data {
                // find_attribute::<T, PolyDataPiece>(pieces, field, filepath)
                // let mut output_vector: Vec<T> = Vec::new();
                for raw_piece in pieces {
                    let piece = raw_piece.load_piece_data(Some(&filepath)).unwrap();
                    let attributes = &piece.data.point;
                    // TODO add escape hatch when attribute is found
                    for attribute in attributes {
                        if attribute.name() == field {
                            if let Attribute::DataArray(data_array) = attribute {
                                print_debug!("{:?}, {:?}", attribute.name(), field);

                                let mut data = data_array
                                    .data
                                    .clone()
                                    .cast_into::<T>()
                                    .expect("Failed cast");
                                output_vector.append(&mut data)
                            }
                        }
                    }
                }
            } else {
                panic!("PolyData not found.  Wrong vtk data type");
            }
        }
        VTKType::UnstructuredGrid => {
            if let DataSet::UnstructuredGrid { pieces, .. } = file.data {
                // find_attribute::<T, UnstructuredGridPiece>(pieces, field, filepath)
                // let mut output_vector: Vec<T> = Vec::new();
                for raw_piece in pieces {
                    let piece = raw_piece.load_piece_data(Some(&filepath)).unwrap();
                    let attributes = &piece.data.point;
                    // TODO add escape hatch when attribute is found
                    for attribute in attributes {
                        if attribute.name() == field {
                            if let Attribute::DataArray(data_array) = attribute {
                                print_debug!("{:?}, {:?}", attribute.name(), field);

                                let mut data = data_array
                                    .data
                                    .clone()
                                    .cast_into::<T>()
                                    .expect("Failed cast");
                                output_vector.append(&mut data)
                            }
                        }
                    }
                }
            } else {
                panic!("UnstructuredGrid not found.  Wrong vtk data type");
            }
        }
    };

    output_vector
}

pub fn get_positions<T>(filename: &str, vtk_type: &VTKType) -> Vec<T>
where
    T: vtkio::model::Scalar,
{
    let filepath = PathBuf::from(filename);
    let mut file = match Vtk::import(filename) {
        Ok(file) => file,
        Err(e) => panic!("Failed to load file: {:?}", e),
    };

    file.load_all_pieces()
        .unwrap_or_else(|_| panic!("Positions load failed"));
    let mut output_vector = Vec::new();
    match vtk_type {
        VTKType::PolyData => {
            if let DataSet::PolyData { pieces, .. } = file.data {
                // find_attribute::<T, PolyDataPiece>(pieces, field, filepath)
                // let mut output_vector: Vec<T> = Vec::new();
                for raw_piece in pieces {
                    let piece = raw_piece.load_piece_data(Some(&filepath)).unwrap();
                    let mut points = piece.points.cast_into::<T>().expect("Failed cast");
                    output_vector.append(&mut points)
                }
            } else {
                panic!("PolyData not found.  Wrong vtk data type");
            }
        }
        VTKType::UnstructuredGrid => {
            if let DataSet::UnstructuredGrid { pieces, .. } = file.data {
                // find_attribute::<T, UnstructuredGridPiece>(pieces, field, filepath)
                // let mut output_vector: Vec<T> = Vec::new();
                for raw_piece in pieces {
                    let piece = raw_piece.load_piece_data(Some(&filepath)).unwrap();
                    let mut points = piece.points.cast_into::<T>().expect("Failed cast");
                    output_vector.append(&mut points)
                }
            } else {
                panic!("UnstructuredGrid not found.  Wrong vtk data type");
            }
        }
    };

    // vtu_file.load_all_pieces().expect("Positions load failed");
    // let pieces = if let DataSet::UnstructuredGrid { pieces, .. } = vtu_file.data {
    //     pieces
    // } else {
    //     panic!("UnstructuredGrid not found.  Wrong vtk data type");
    // };

    // print_debug!("Number of pieces = {}", pieces.len());
    // print_debug!("File = {:?}", filename);

    // let mut output_vector: Vec<T> = Vec::new();
    // for raw_piece in pieces {
    //     let piece = raw_piece.load_piece_data(Some(&filepath)).unwrap();
    //     let mut points = piece.points.cast_into::<T>().expect("Failed cast");
    //     output_vector.append(&mut points)
    // }

    output_vector
}
