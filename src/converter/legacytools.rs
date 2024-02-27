use vtkio::model::*;
pub fn get_field<T>(filename: &str, field: &str) -> Vec<T>
where
    T: vtkio::model::Scalar,
{
    let vtk_file =
        Vtk::import(filename).unwrap_or_else(|_| panic!("Failed to load file: {:?}", filename));
    let pieces = if let DataSet::PolyData { pieces, .. } = vtk_file.data {
        pieces
    } else {
        panic!("Wrong vtk data type");
    };

    // If piece is already inline, this just returns a piece data clone.
    let piece = pieces[0]
        .load_piece_data(None)
        .expect("Failed to load piece data");

    let attribute = &piece.data.point[0];

    if let Attribute::Field { data_array, .. } = attribute {
        data_array
            .iter()
            .find(|&DataArrayBase { name, .. }| name == field)
            .unwrap_or_else(|| panic!("Failed to find {} field", field))
            .data
            .clone()
            .cast_into::<T>()
            .expect("Failed cast")
    } else {
        panic!("No field attribute found");
    }
}

pub fn get_positions<T>(filename: &str) -> Vec<T>
where
    T: vtkio::model::Scalar,
{
    let vtk_file =
        Vtk::import(filename).unwrap_or_else(|_| panic!("Failed to load file: {:?}", filename));
    let pieces = if let DataSet::PolyData { pieces, .. } = &vtk_file.data {
        pieces
    } else {
        panic!("Wrong vtk data type");
    };
    // If piece is already inline, this just returns a piece data clone.
    let piece = pieces[0]
        .load_piece_data(None)
        .expect("Failed to load piece data");

    let attribute = &piece.points;
    attribute.clone().cast_into::<T>().expect("Failed cast")
}
