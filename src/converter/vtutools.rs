use std::path::PathBuf;

use regex::CaptureMatches;
use vtkio::model::*;

use crate::{print_debug, vtu};
pub fn get_field<T>(filename: &str, field: &str) -> Vec<T>
where
    T: vtkio::model::Scalar,
{
    let filepath = PathBuf::from(filename);
    let mut vtu_file =
        Vtk::import(filename).unwrap_or_else(|_| panic!("Failed to load file: {:?}", filename));
    vtu_file.load_all_pieces();
    let pieces = if let DataSet::UnstructuredGrid { pieces, .. } = vtu_file.data {
        pieces
    } else {
        panic!("UnstructuredGrid not found.  Wrong vtk data type");
    };

    print_debug!("Number of pieces = {}", pieces.len());
    print_debug!("File = {:?}", filename);

    let mut output_vector: Vec<T> = Vec::new();
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

    output_vector
}

pub fn get_positions<T>(filename: &str) -> Vec<T>
where
    T: vtkio::model::Scalar,
{
    let filepath = PathBuf::from(filename);
    let mut vtu_file =
        Vtk::import(filename).unwrap_or_else(|_| panic!("Failed to load file: {:?}", filename));
    vtu_file.load_all_pieces();
    let pieces = if let DataSet::UnstructuredGrid { pieces, .. } = vtu_file.data {
        pieces
    } else {
        panic!("UnstructuredGrid not found.  Wrong vtk data type");
    };

    print_debug!("Number of pieces = {}", pieces.len());
    print_debug!("File = {:?}", filename);

    let mut output_vector: Vec<T> = Vec::new();
    for raw_piece in pieces {
        let piece = raw_piece.load_piece_data(Some(&filepath)).unwrap();
        let mut points = piece.points.cast_into::<T>().expect("Failed cast");
        output_vector.append(&mut points)
    }

    output_vector
}
