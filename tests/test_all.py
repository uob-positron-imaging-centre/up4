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
# add your function to this command list

@pytest.fixture
def data(request):
    """ Returns a instance of pdata with the experiment test data in fixtures/"""
    if request.param == "exp":
        return up4.Data("tests/fixtures/1000_0b_L_surf.hdf5")
    elif request.param == "exp2":
        return up4.Data("tests/fixtures/3u_HD1_glass.hdf5")
    elif request.param == "sim":
        return up4.Data("tests/fixtures/drum.hdf5")

@pytest.fixture
def grid(request):
    """ Returns a instance of pdata with the experiment test data in fixtures/"""
    if request.param == "cylidrical":
        return up4.Grid.databound_cylindrical3d
    elif request.param == "cartesian":
        return up4.Grid.databound_cartesian3d






class TestVtk:
    def test_successfull_write(self):
        """Test if the hdf5 file is written"""
        if os.path.exists("tests/fixtures/vtk/drum.hdf5"):
            os.remove("tests/fixtures/vtk/drum.hdf5")
        filenames = sorted([x for x in glob("tests/fixtures/post/drum*.vtk") if not "bound" in x])
        up4.Converter.vtk(filenames, 1e-5, "tests/fixtures/drum.hdf5")
        assert os.path.exists("tests/fixtures/drum.hdf5") == True

    def test_successfull_generated(self):
        """Test if the hdf5 file is written and that it is readable"""
        if os.path.exists("tests/fixtures/vtk/drum.hdf5"):
            os.remove("tests/fixtures/vtk/drum.hdf5")
        filenames = sorted([x for x in glob("tests/fixtures/post/drum*.vtk") if not "bound" in x])
        up4.Converter.vtk(filenames, 1e-5, "tests/fixtures/drum.hdf5")
        try:
            up4.Data.from_tdata("tests/fixtures/drum.hdf5")
        except Exception as e:
            pytest.fail(e)

@pytest.mark.parametrize("data",["exp","exp2","sim"], indirect=True)
@pytest.mark.parametrize("grid",["cylidrical", "cartesian"], indirect=True)
class TestFields:

    def test_velocityfield(self,data,grid):
        grid = grid(data, cells = [10,10,10])
        field = data.velocityfield(grid)
        assert field.shape() == [10,10,10]

    def test_numberfield(self,data,grid):
        grid = grid(data, cells = [10,10,10])
        field = data.numberfield(grid)
        assert field.shape() == [10,10,10]

    def test_vectorfield(self,data,grid):
        grid = grid(data, cells = [10,10,10])
        field = data.vectorfield(grid)
        assert field.shape() == [10,10,10]


