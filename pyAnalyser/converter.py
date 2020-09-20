#! /usr/bin/env/python3

import h5py
import numpy as np


class Liggghts():
    def __init__(self):
        pass
        
    def read_custom(self):
        pass
        
        
class mercuryDPM():
    def __init__(self):
        pass
        
def PEPT(filename):
    data = np.genfromtxt(filename,skip_header=16)
    file = h5py.File(filename.split("a01")[0]+"hdf5","w")
    data[1::] /= 1000
    data[0]/=1000000
    inf=float("inf")
    min_val=[inf,inf,inf]
    max_val=[-inf,-inf,-inf]
    ###
    #needed variables
    cur_step = -1
    
    
    for id,timestep in enumerate(data):
        if id == 0 or id == len(data)-1:
            continue
        cur_step +=1
        grp = file.create_group("timestep "+str(cur_step))
        
        time = timestep[0]
        grp.create_dataset("time", data = time)
        
        x = timestep[1]
        y = timestep[3]
        z = timestep[2]
        position = np.asarray([[x,y,z]])
        positions= [x,y,z]

        vx = (-data[id+1][1] + data[id-1][1])/(data[id-1][0]-data[id+1][0])
        vy = (-data[id+1][3] + data[id-1][3])/(data[id-1][0]-data[id+1][0])
        vz = (-data[id+1][2] + data[id-1][2])/(data[id-1][0]-data[id+1][0])
        velocity = np.asarray([[vx,vy,vz]])
        
        grp.create_dataset("position",data = position)
        grp.create_dataset("velocity",data = velocity)
        grp.create_dataset("radius",data = np.asarray([0]))
        grp.create_dataset("ppcloud",data = np.asarray([0]))
        grp.create_dataset("spezies",data = np.asarray([0]))
        grp.create_dataset("particleid",data = np.asarray([0]))
        
        for i in range(0,3):
            min_val[i] = positions[i] if positions[i] < min_val[i] else min_val[i]
            max_val[i] = positions[i] if positions[i] > max_val[i] else max_val[i]
            
            
            
    file.create_dataset("dimensions", data = np.asarray([min_val,max_val]))
