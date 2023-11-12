pub struct ScalarMap {
    x: Array1<f64>,
    y: Array1<f64>,
    data: Vec<Vec<f64>>,
}

impl ScalarMap {
    pub fn from_vector_grid_depth_averaged(grid: VectorGrid, axis: usize) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let u = grid.data[i].collapse(axis);
        let v = grid.data[j].collapse(axis);
        let mut data = Vec::with_capacity(u.dim().0);
        for (u_row, v_row) in izip!(u.axis_iter(ndarray::Axis(0)), v.axis_iter(ndarray::Axis(0))) {
            let mut inner_vec = Vec::with_capacity(u.dim().1);
            Zip::from(&mut inner_vec).and(&u_row).and(&v_row).for_each(|d, &u, &v|{
                *d = f64::hypot(u, v);
            });
            data.push(inner_vec);
        }

        ScalarMap { x, y, data }
    }

    pub fn from_vector_grid_single_plane(grid: VectorGrid, axis: usize, index: usize) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let u = grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let v = grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut data = Vec::with_capacity(u.dim().0);
        for (u_row, v_row) in izip!(u.axis_iter(ndarray::Axis(0)), v.axis_iter(ndarray::Axis(0))) {
            let mut inner_vec = Vec::with_capacity(u.dim().1);
            Zip::from(&mut inner_vec).and(&u_row).and(&v_row).for_each(|d, &u, &v|{
                *d = f64::hypot(u, v);
            });
            data.push(inner_vec);
        }

        ScalarMap { x, y, data }
    }

    pub fn from_grid_depth_averaged(grid: Box<dyn GridFunctions3D>, axis: usize) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let data_arr = grid.collapse(axis);
        let mut data: Vec<Vec<f64>> = Vec::with_capacity(data_arr.dim().0);
        for row in data_arr.rows() {
            let inner_vec = row.to_vec();
            data.push(inner_vec);
        }
        ScalarMap { x, y, data }
    }

    pub fn from_grid_single_plane(
        grid: Box<dyn GridFunctions3D>,
        axis: usize,
        index: usize,
    ) -> ScalarMap {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 {
            grid.get_ypositions().to_owned()
        } else {
            grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            grid.get_zpositions().to_owned()
        } else {
            grid.get_ypositions().to_owned()
        };
        let data_arr = grid.collapse(axis);
        let mut data: Vec<Vec<f64>> = Vec::with_capacity(data_arr.dim().0);
        for row in data_arr.rows() {
            let inner_vec = row.to_vec();
            data.push(inner_vec);
        }

        ScalarMap { x, y, data }
    }

    pub fn create_scalar_map(&self) -> Vec<Box<HeatMap<f64, f64, Vec<f64>>>> {
        let (x, y) = meshgrid(self.x.to_owned(), self.y.to_owned());
        let heatmap = HeatMap::new(
            x.into_raw_vec(),
            y.into_raw_vec(),
            self.data.to_owned(),
        );
        let trace = vec![heatmap];

        trace
    }
}