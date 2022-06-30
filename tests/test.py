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
def exp_data():
    """ Returns a instance of pdata with the experiment test data in fixtures/"""
    return up4.Data.from_pdata("fixtures/HSM_Glass_2l_250.hdf5")

@pytest.fixture
def sim_data():
    """ Returns a instance of tdata with the simulation test data in fixtures/"""
    return up4.Data.from_tdata("fixtures/drum.hdf5")


@pytest.mark.parametrize("data",[exp_data, sim_data])
def test_data(data):
    pass

class TestVtk:


    def test_successfull_write(self):
        """Test if the hdf5 file is written"""
        if os.path.exists("fixtures/vtk/drum.hdf5"):
            os.remove("fixtures/vtk/drum.hdf5")
        filenames = sorted([x for x in glob("fixtures/post/drum*.vtk") if not "bound" in x])
        up4.Converter.vtk(filenames, 1e-5, "fixtures/drum.hdf5")
        assert os.path.exists("fixtures/drum.hdf5") == True

    def test_successfull_generated(self):
        """Test if the hdf5 file is written and that it is readable"""
        if os.path.exists("fixtures/vtk/drum.hdf5"):
            os.remove("fixtures/vtk/drum.hdf5")
        filenames = sorted([x for x in glob("fixtures/post/drum*.vtk") if not "bound" in x])
        up4.Converter.vtk(filenames, 1e-5, "fixtures/drum.hdf5")
        try:
            up4.Data.from_tdata("fixtures/drum.hdf5")
        except Exception as e:
            pytest.fail(e)


"""
cmds=[]

def test_vtkio():
    # wrong filenames:
    filenames = glob("fixtures/post/drum*.vtk")
    try:
        p.Converter.vtk(filenames, 1e-5, "fixtures/drum.hdf5")
    except:
        print("Test failed successfully!")
    filenames = sorted([x for x in glob("fixtures/post/drum*.vtk") if not "bound" in x])
    #wrong filter:
    try:
        p.Converter.vtk(filenames, 1e-5, "fixtures/drum.hdf5",r"a(\d+).test")
    except:
        print("Test failed successfully!")
    #right filter:
    p.Converter.vtk(filenames, 1e-5, "fixtures/drum.hdf5")
    # test created dataset
    data = p.Data.from_tdata("fixtures/drum.hdf5")
    test_vectorfield(data)
#cmds.append(test_vtkio)

def test_vectorfield(data):
    grid = p.Grid.create2d(
            (20,25),        # Number of cells
            (-0.1,0.1),     # x-Dimensions
            (-0.1,0.1)      # y-Dimensions
        )
    vx,vy,px,py=data.vectorfield(grid)
    assert (vx.shape==(20,25)), "Wrong dimensions in vx-vectorfield"
    assert (vy.shape==(20,25)), "Wrong dimensions in vy-vectorfield"
    assert (px.shape==(20,25)), "Wrong dimensions in px-vectorfield"
    assert (py.shape==(20,25)), "Wrong dimensions in py-vectorfield"
cmds.append(test_vectorfield)

def test_mean_velocity(data):
    v = data.mean_velocity()
    assert type(v)==type(0.1), "Wrong return type"
    print(f"\t\tMean Velocity: {v}")
cmds.append(test_mean_velocity)

def test_mean_velocity_showcase(data):
    v = data.mean_velocity_showcase()
    print(f"\t\tMean Velocity: {v}")
    assert type(v)==type(0.1), "Wrong return type"
cmds.append(test_mean_velocity_showcase)



def test(data):
    result=[]
    for cmd in cmds:
        t = time.time()
        print(f"\tRunning{cmd.__name__}:")
        try:
            returndata=cmd(data)
        except KeyboardInterrupt:
            print("Ending program")
            raise KeyboardInterrupt
        except Exception as e:
            print((
            f"\tAn error occured while executing function {cmd.__name__}\n"
            f"ERROR: \n{e}"
            ))
            print(f"\t\t{cmd.__name__} failed after {time.time()-t:.04f} seconds")
            result.append([cmd.__name__,time.time()-t,"No"])
            continue
        print(f"\t\t{cmd.__name__} succeeded after {time.time()-t:.04f} seconds")
        result.append([cmd.__name__,time.time()-t,"Yes"])
    return result



if __name__=="__main__":
    test_vtkio()
    print("Testing Simulation Data")
    data = p.Data.from_tdata("fixtures/drum.hdf5")
    test(data)
    exit()
    print("\n\nTesting Experimental Data")
    data = p.Data.from_pdata("fixtures/HSM_Glass_2l_250.hdf5")
    test(data)
"""