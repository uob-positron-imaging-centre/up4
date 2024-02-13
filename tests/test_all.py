#! /usr/bin/python3
# Author:   Dominik Werner
# File:     test.py
# Date:     20.11.21

"""
 Testing for uPPPP
 this skript tests given functions and returns its speed and
 if it failed or not
"""
import up4
import numpy as np
import os
from glob import glob
import pytest
from natsort import natsorted as sorted

location = os.path.dirname(os.path.abspath(__file__))
# add your function to this command list


@pytest.fixture
def data(request):
    """Returns a instance of pdata with the experiment test data in fixtures/"""
    if request.param == "exp":
        folder = os.path.join(location, "fixtures", "1p5u_HD1_glass.hdf5")
        return up4.Data(folder)
    elif request.param == "exp2":
        folder = os.path.join(location, "fixtures", "26mbq_day2.hdf5")
        return up4.Data(folder)
    elif request.param == "sim":
        folder = os.path.join(location, "fixtures", "drum.hdf5")
        return up4.Data(folder)


@pytest.fixture
def grid(request):
    """Returns a instance of pdata with the experiment test data in fixtures/"""
    if request.param == "cylidrical":
        return up4.Grid.cylindrical3d_from_data
    elif request.param == "cartesian":
        return up4.Grid.cartesian3d_from_data


class TestVtk:
    def test_generated(self):
        """Test if the hdf5 file is written and that it is readable"""
        if os.path.exists(os.path.join(location, "fixtures", "drum.hdf5")):
            os.remove(os.path.join(location, "fixtures", "drum.hdf5"))
        filenames = sorted([
            x
            for x in glob(os.path.join(location, "fixtures", "post", "drum*.vtk"))
            if not "bound" in x
        ])
        up4.Converter.vtk(
            filenames, 1e-5, os.path.join(location, "fixtures", "drum.hdf5")
        )
        try:
            up4.Data.from_tdata(os.path.join(location, "fixtures", "drum.hdf5"))
        except Exception as e:
            pytest.fail(e)

    def test_generated_folder(self):
        """Test if the hdf5 file is written and that it is readable"""
        if os.path.exists(os.path.join(location, "fixtures", "drum.hdf5")):
            os.remove(os.path.join(location, "fixtures", "drum.hdf5"))
        up4.Converter.vtk_from_folder(
            os.path.join(location, "fixtures", "post"),
            1e-5,
            os.path.join(location, "fixtures", "drum.hdf5"),
        )
        try:
            up4.Data.from_tdata(os.path.join(location, "fixtures", "drum.hdf5"))
        except Exception as e:
            pytest.fail(e)


class TestCSV:
    def test_one(self):
        """Test if the csv file is written and that it is readable"""
        file = os.path.join(location, "fixtures", "1p5u_HD1_glass.hdf5")
        if os.path.exists(file):
            os.remove(file)
        up4.Converter.csv(
            os.path.join(location, "fixtures", "csvs", "1p5u_HD1_glass.csv"),
            file,
            columns=[0, 1, 3, 2],
            delimiter=" ",
            comment="#",
            vel=True,
            interpolate=True,
        )
        try:
            up4.Data(file)
        except Exception as e:
            pytest.fail(e)

    def test_two(self):
        """Test if the csv file is written and that it is readable"""
        file = os.path.join(location, "fixtures", "26mbq_day2.hdf5")
        if os.path.exists(file):
            os.remove(file)
        up4.Converter.csv(
            os.path.join(location, "fixtures", "csvs", "26mbq_day2.csv"),
            file,
            columns=[0, 1, 3, 2],
            delimiter=" ",
            comment="#",
            vel=True,
            interpolate=True,
        )
        try:
            up4.Data(file)
        except Exception as e:
            pytest.fail(e)


@pytest.mark.parametrize("data", ["exp2", "sim"], indirect=True)
@pytest.mark.parametrize("grid", ["cylidrical", "cartesian"], indirect=True)
class TestFields:
    def test_velocityfield(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        field = data.velocityfield(grid)
        assert field.shape() == [10, 10, 10]

    def test_numberfield(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        field = data.numberfield(grid)
        assert field.shape() == [10, 10, 10]

    def test_vectorfield(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        field = data.vectorfield(grid)
        assert field.shape() == [10, 10, 10]

    def test_dispersion(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        dispersion, me = data.dispersion(grid, 0.2)
        assert dispersion.shape() == [10, 10, 10]

    def test_granular_temperature(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        field = data.granular_temperature(grid)
        assert field.shape() == [10, 10, 10]

    def test_occupancyfield(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        field = data.occupancyfield(grid)
        assert field.shape() == [10, 10, 10]

    def test_msd_field(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        field = data.msd_field(grid, 0.2)
        assert field.shape() == [10, 10, 10]


@pytest.mark.parametrize("grid", ["cylidrical", "cartesian"], indirect=True)
class TestGrid:
    def test_slice(self, grid):
        grid = grid(
            up4.Data.from_tdata(os.path.join(location, "fixtures", "drum.hdf5")),
            cells=[10, 9, 8],
        )
        slice_yz = grid.slice(0, 5)
        assert slice_yz.shape == (9, 8)
        slice_xz = grid.slice(1, 5)
        assert slice_xz.shape == (10, 8)
        slice_xy = grid.slice(2, 5)
        assert slice_xy.shape == (10, 9)

    def test_vector_slice(self, grid):
        data = up4.Data.from_tdata(os.path.join(location, "fixtures", "drum.hdf5"))
        grid = grid(data, cells=[10, 9, 8])
        vector_grid = data.vectorfield(grid)
        slice_yz_1, slice_yz_2, slice_yz_3 = vector_grid.slice(0, 5)
        assert slice_yz_1.shape == (9, 8)
        assert slice_yz_2.shape == (9, 8)
        assert slice_yz_3.shape == (9, 8)
        slice_xz_1, slice_xz_2, slice_xz_3 = vector_grid.slice(1, 5)
        assert slice_xz_1.shape == (10, 8)
        assert slice_xz_2.shape == (10, 8)
        assert slice_xz_3.shape == (10, 8)
        slice_xy_1, slice_xy_2, slice_xy_3 = vector_grid.slice(2, 5)
        assert slice_xy_1.shape == (10, 9)
        assert slice_xy_2.shape == (10, 9)
        assert slice_xy_3.shape == (10, 9)

    def test_grid_generation(self, grid):
        data = up4.Data.from_tdata(os.path.join(location, "fixtures", "drum.hdf5"))
        if grid.__name__.startswith("cartesian"):
            grid = up4.Grid(data, num_cells=[10, 9, 8])
            assert grid.shape() == [10, 9, 8]
            grid = up4.Grid(
                data, num_cells=[10, 9, 8], xlim=[0, 1], ylim=[0, 1], zlim=[0, 1]
            )
            assert grid.shape() == [10, 9, 8]
        else:
            grid = up4.Grid(data, num_cells=[10, 9, 8], grid_style="cylindrical")
            assert grid.shape() == [10, 9, 8]
            grid = up4.Grid(
                data,
                num_cells=[10, 9, 8],
                xlim=[0, 1],
                ylim=[0, 1],
                zlim=[0, 1],
                grid_style="cylindrical",
            )
            assert grid.shape() == [10, 9, 8]


@pytest.mark.parametrize("data", ["exp2", "sim"], indirect=True)
@pytest.mark.parametrize("grid", ["cylidrical", "cartesian"], indirect=True)
class TestFunctions:
    def histogram(self, data, grid):
        hist, bins = data.histogram(grid)
        assert len(hist) == 10
        assert len(bins) == 11


@pytest.mark.parametrize("data", ["sim"], indirect=True)
@pytest.mark.parametrize("grid", ["cylidrical", "cartesian"], indirect=True)
class TestMixing:
    def test_lacey(self, data, grid):
        grid = grid(data, cells=[10, 10, 10])
        time, mixing = data.lacey_mixing_index(grid, 1, 1)
        assert mixing[0] >= 0  # think of a better test here


@pytest.mark.parametrize("data", ["sim", "exp2"], indirect=True)
class TestConditional:
    def test_circulation(self, data):
        xmin, xmax = data.min_position()[0], data.max_position()[0]
        circ_time = data.circulation_time(position=(xmin + xmax) / 2, axis=0)
        assert np.nanmean(circ_time) > 0
