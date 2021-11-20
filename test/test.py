#! /usr/bin/python
# Author:   Dominik Werner
# File:     test.py
# Date:     20.11.21

###############
# Testing suit for uPPPP
# this skript tests given functions and returns its speed and
# if it failed or not

import uPPPP as p
import time
import numpy as np


# add your function to this command list
cmds=[]

################## Vectorfield test #################
def test_vectorfield(data):
    grid = p.Grid.create2d(
            (20,25),        # Number of cells
            (-0.1,0.1),     # x-Dimensions
            (-0.1,0.1)      # y-Dimensions
        )
    vx,vy,px,py=data.vectorfield(grid)
    print(py.shape)
    assert(vx.shape==(20,25)), "Wrong dimensions in vx-vectorfield"
    assert(vy.shape==(20,25)), "Wrong dimensions in vy-vectorfield"
    assert(px.shape==(20,25)), "Wrong dimensions in px-vectorfield"
    assert(py.shape==(20,25)), "Wrong dimensions in py-vectorfield"
cmds.append(test_vectorfield)
##################### End ###############################


def run_test(data):
    result=[]
    for cmd in cmds:
        t = time.time()
        print(f"Testing {cmd.__name__}")
        try:
            returndata=cmd(data)
        except KeyboardInterrupt:
            print("Ending program")
            raise KeyboardInterrupt
        except Exception as e:
            print((
            f"An error occured while executing function {cmd.__name__}\n"
            f"ERROR: \n{e}"
            ))
            with open(f"ERROR_{cmd.__name__}_{time.time()}.txt","w") as f:
                f.write("uPPPP Error:\n"+str(e))
            print(f"{cmd.__name__} failed after {time.time()-t:.04f} seconds")
            result.append([cmd.__name__,time.time()-t,"No"])
            continue
        print(f"{cmd.__name__} succeeded after {time.time()-t:.04f} seconds")
        result.append([cmd.__name__,time.time()-t,"Yes"])
    return result



if __name__=="__main__":
    print("Testing Simulation Data")
    data = p.Data.from_tdata("drum.hdf5")
    run_test(data)

    print("\n\nTesting Experimental Data")
    data = p.Data.from_pdata("HSM_Glass_2l_250.hdf5")
    run_test(data)
