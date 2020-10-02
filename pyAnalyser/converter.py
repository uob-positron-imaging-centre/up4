#! /usr/bin/env/python3

import h5py
import numpy as np
from vtk import vtkDataSetReader
from vtk.util.numpy_support import vtk_to_numpy
from natsort import natsorted, ns


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

def Liggghts(filenames,dt = 1):
    filenames = natsorted(filenames,key=lambda y: y.lower()) 
    inf=float("inf")
    min_val=np.asarray([inf,inf,inf])
    max_val=np.asarray([-inf,-inf,-inf])
    file = h5py.File("_".join(filenames[0].split("_")[0:-1])+".hdf5","w")
    for id,filename in enumerate(filenames):
        #create group for current timestep
        grp = file.create_group("timestep "+str(id))
        ##### read the ligghts file
        reader = vtkDataSetReader()
        reader.SetFileName(filename)
        reader.ReadAllScalarsOn() 
        reader.ReadAllVectorsOn() 
        reader.Update()
        data = reader.GetOutput()
        p = data.GetPointData()
        
        time = int(filename.split(".vtk")[0].split("_")[-1]) * dt
        position = vtk_to_numpy(data.GetPoints().GetData())
        velocity = vtk_to_numpy(p.GetArray("v")) 
        ID = vtk_to_numpy(p.GetArray("id"))
        radius = vtk_to_numpy(p.GetArray("radius"))
        type_ = vtk_to_numpy(p.GetArray("type"))
        
        
        # create datasets for each variables
        grp.create_dataset("time", data = time)
        grp.create_dataset("position",data = position) 
        grp.create_dataset("velocity",data = velocity )
        grp.create_dataset("radius",data = radius )
        grp.create_dataset("ppcloud",data = np.zeros(len(type_)) )
        grp.create_dataset("spezies",data = type_ )
        grp.create_dataset("particleid",data = ID )
        
        
        #checking the min and ma position
        pos_min = np.min(position,axis = 0)
        pos_max = np.max(position,axis = 0)
        min_ = pos_min < min_val
        max_ = pos_max > max_val
        min_val[min_] = pos_min[min_] 
        max_val[max_] = pos_max[max_] 
    file.create_dataset("dimensions", data = np.asarray([min_val,max_val]))  
    file.close()
        
