pub struct ParityContour {
    reference_data: Array2<f64>,
    comparison_data: Array2<f64>,
    x: Array2<f64>,
    y: Array2<f64>,
    delta: Array2<f64>,
}

impl ParityContour {
    pub fn from_vector_grids_depth_averaged(
        reference_grid: VectorGrid,
        comparison_grid: VectorGrid,
        axis: usize,
    ) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let (x, y) = meshgrid(x, y);

        let ref_u = reference_grid.data[i].collapse(axis);
        let ref_v = reference_grid.data[j].collapse(axis);
        let mut reference_data = Array2::zeros(ref_u.dim());
        Zip::from(&mut reference_data)
            .and(&ref_u)
            .and(&ref_v)
            .for_each(|d, &u, &v| {
                *d = f64::hypot(u, v);
            });

        let comp_u = comparison_grid.data[i].collapse(axis);
        let comp_v = comparison_grid.data[j].collapse(axis);
        let mut comparison_data = Array2::zeros(comp_u.dim());
        Zip::from(&mut comparison_data)
            .and(&comp_u)
            .and(&comp_v)
            .for_each(|d, &u, &v| {
                *d = f64::hypot(u, v);
            });

        let delta = &reference_data - &comparison_data;

        ParityContour {
            reference_data,
            comparison_data,
            x,
            y,
            delta,
        }
    }

    pub fn from_vector_grids_single_plane(
        reference_grid: VectorGrid,
        comparison_grid: VectorGrid,
        axis: usize,
        index: usize,
    ) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };
        let i = usize::from(axis == 0);
        let j = if axis == 0 || axis == 1 { 2 } else { 1 };
        let (x, y) = meshgrid(x, y);

        let ref_u = reference_grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let ref_v = reference_grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut reference_data = Array2::zeros(ref_u.dim());
        Zip::from(&mut reference_data)
            .and(&ref_u)
            .and(&ref_v)
            .for_each(|d, &u, &v| {
                *d = f64::hypot(u, v);
            });

        let comp_u = comparison_grid.data[i]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let comp_v = comparison_grid.data[j]
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);
        let mut comparison_data = Array2::zeros(comp_u.dim());
        Zip::from(&mut comparison_data)
            .and(&comp_u)
            .and(&comp_v)
            .for_each(|d, &u, &v| {
                *d = f64::hypot(u, v);
            });

        let delta = &reference_data - &comparison_data;

        ParityContour {
            reference_data,
            comparison_data,
            x,
            y,
            delta,
        }
    }

    pub fn from_grids_depth_averaged(
        reference_grid: Box<dyn GridFunctions3D>,
        comparison_grid: Box<dyn GridFunctions3D>,
        axis: usize,
    ) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };

        let (x, y) = meshgrid(x, y);

        let reference_data = reference_grid.collapse(axis);

        let comparison_data = comparison_grid.collapse(axis);

        let delta = &reference_data - &comparison_data;

        ParityContour {
            reference_data,
            comparison_data,
            x,
            y,
            delta,
        }
    }

    pub fn from_grids_single_plane(
        reference_grid: Box<dyn GridFunctions3D>,
        comparison_grid: Box<dyn GridFunctions3D>,
        axis: usize,
        index: usize,
    ) -> ParityContour {
        // select yz (0), xz (1) or xy (2) plane
        let x = if axis == 0 || axis == 1 {
            reference_grid.get_ypositions().to_owned()
        } else {
            reference_grid.get_xpositions().to_owned()
        };
        let y = if axis == 0 || axis == 1 {
            reference_grid.get_zpositions().to_owned()
        } else {
            reference_grid.get_ypositions().to_owned()
        };

        let (x, y) = meshgrid(x, y);

        let reference_data = reference_grid
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);

        let comparison_data = comparison_grid
            .get_data()
            .to_owned()
            .index_axis_move(ndarray::Axis(axis), index);

        let delta = &reference_data - &comparison_data;

        ParityContour {
            reference_data,
            comparison_data,
            x,
            y,
            delta,
        }
    }

    pub fn create_parity_contour(&self) -> Vec<Box<HeatMap<f64, f64, f64>>> {
        let heatmap = HeatMap::new(
            self.x.to_owned().into_raw_vec(),
            self.y.to_owned().into_raw_vec(),
            self.delta.to_owned().into_raw_vec(),
        );
        let traces = vec![heatmap];

        traces
    }

    pub fn delta_as_percent(self) -> Self {
        let delta = (&self.reference_data - &self.comparison_data) / &self.reference_data * 100.;

        ParityContour {
            reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x,
            y: self.y,
            delta,
        }
    }

    pub fn delta_as_difference(self) -> Self {
        let delta = &self.reference_data - &self.comparison_data;

        ParityContour {
            reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x,
            y: self.y,
            delta,
        }
    }

    pub fn delta_as_absolute_difference(self) -> Self {
        let delta_vec: Vec<f64> = (&self.reference_data - &self.comparison_data)
            .into_iter()
            .map(f64::abs)
            .collect();
        let delta: Array2<f64> =
            Array2::from_shape_vec(self.reference_data.raw_dim(), delta_vec).unwrap();
        ParityContour {
            reference_data: self.reference_data,
            comparison_data: self.comparison_data,
            x: self.x,
            y: self.y,
            delta,
        }
    }
}
