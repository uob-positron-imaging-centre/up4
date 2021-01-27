#! /usr/bin/env/python3

import h5py
import numpy as np
from vtk import vtkDataSetReader
from vtk.util.numpy_support import vtk_to_numpy
from natsort import natsorted, ns
import os
from glob import glob
from tqdm import tqdm
import pickle


def pept(filename, header=16):
    data = np.genfromtxt(filename, skip_header=header)
    dump = 2
    while True:
        filename2 = filename + f"_0{dump}"
        print(f"looking for file: \"{filename2}\"")
        print(os.path.exists(filename2))
        if os.path.exists(filename2):
            print(f"Appending data {filename} with {filename2}")
            data2 = np.genfromtxt(filename2, skip_header=header)
            data = np.concatenate((data, data2))
            dump += 1
        else:
            break
    file = h5py.File(filename.split("a01")[0]+"hdf5", "w")
    data[1::] /= 1000
    data[0] /= 1000000
    inf = float("inf")
    min_val = np.asarray([inf, inf, inf])
    max_val = np.asarray([-inf, -inf, -inf])
    ###
    # needed variables
    velocitys = []
    positions = []
    time = []
    vel_calculation_steps = 4
    for id, timestep in enumerate(data):
        if id < vel_calculation_steps or \
           id > len(data) - vel_calculation_steps - 1:
            continue

        time.append(timestep[0])

        x = timestep[1]
        y = timestep[3]
        z = timestep[2]
        position = np.asarray([x, y, z])

        vx = (-data[id + vel_calculation_steps][1] + data[id - vel_calculation_steps][1])/(data[id- vel_calculation_steps ][0]-data[id+ vel_calculation_steps ][0])
        vy = (-data[id + vel_calculation_steps][3] + data[id - vel_calculation_steps][3])/(data[id- vel_calculation_steps ][0]-data[id+ vel_calculation_steps ][0])
        vz = (-data[id + vel_calculation_steps][2] + data[id - vel_calculation_steps][2])/(data[id- vel_calculation_steps ][0]-data[id+ vel_calculation_steps ][0])
        velocity = np.asarray([vx, vy, vz])
        positions.append(position)
        velocitys.append(velocity)

        min_mask = position < min_val
        max_mask = position > max_val
        min_val[min_mask] = position[min_mask]
        max_val[max_mask] = position[max_mask]

    cur_step = 0

    print(np.asarray(positions).shape)
    print(np.asarray(positions[0:1]).shape)

    print(np.asarray(positions[0]).shape)
    grp = file.create_group("timestep "+str(cur_step))
    grp.create_dataset("time", data=time[0])
    grp.create_dataset("position", data=np.asarray(positions))
    grp.create_dataset("velocity", data=np.asarray(velocitys))
    grp.create_dataset("radius", data=np.asarray([0]))
    grp.create_dataset("ppcloud", data=np.asarray([0]))
    grp.create_dataset("spezies", data=np.asarray([0]))
    grp.create_dataset("particleid", data=np.asarray([0]))

    grp = file.create_group("timestep "+str(1))
    grp.create_dataset("time", data=time[10])

    file.create_dataset("dimensions", data=np.asarray([min_val, max_val]))


def barracuda(file, overwrite=False):
        new_file = file.split(".pickle")[0]+".hdf5"
        if os.path.exists(file.split(".pickle")[0]+".hdf5") or \
        os.path.exists(file.split(".pickle")[0]+".h5"):
            if not overwrite:
                print(f"WARNING!!! using equally named filename \"{new_file}\" instead of \"{file}\"\nInclude \"overwrite = True \" to overwrite data ")
                return None
        data, min_array, max_array = pickle.load(open(file,"rb"))
        data = np.asarray(data, dtype=object)
        f = h5py.File(new_file,"w")
        f.create_dataset("dimensions", data=np.asarray([min_array, max_array]))
        for id,timestep in enumerate(data):
            time = timestep[0]
            particle_data  = timestep[1]
            group = f.create_group( "timestep "+str(id) )

            p_data = np.asarray(particle_data,dtype=float)
            pos = p_data[:,0:3]
            vel = p_data[:,3:6]
            rad = p_data[:,9]
            cloud = p_data[:,8]
            spezies = p_data[:,6]
            particle_id =p_data[:,7]
            try:
                dens = p_data[:,10]
                group.create_dataset("density", data=dens)
            except:
                pass
            group.create_dataset("position", data=pos)
            group.create_dataset("velocity", data=vel)
            group.create_dataset("radius", data=rad)
            group.create_dataset("ppcloud", data=cloud)
            group.create_dataset("spezies", data=spezies)
            group.create_dataset("particleid", data=particle_id)
            group.create_dataset("time", data=time)

def liggghts(files, dt=1):
    # filetype check.
    # we accept the path to files or a list of files
    if type(files) == type([]):
        files = [f for f in files if f.endswith("vtk")]
        if len(files) < 1:
            raise ValueError(("Files supplyed don't have the accepted "
                "VTK format!"))
    elif isinstance(files,str):
        files = [x for x in  glob(files+"*.vtk") if not "bound" in x]
        if len(files) < 1:
            raise ValueError("No files found in this directory")
    filenames = natsorted(files,key=lambda y: y.lower())
    inf=float("inf")
    min_val=np.asarray([inf,inf,inf])
    max_val=np.asarray([-inf,-inf,-inf])
    timestep = 0
    with h5py.File("_".join(filenames[0].split("_")[0:-1])+".hdf5","w") as file:
        for id,filename in tqdm(enumerate(filenames)):
            ##### read the ligghts file
            reader = vtkDataSetReader()
            reader.SetFileName(filename)
            reader.ReadAllScalarsOn()
            reader.ReadAllVectorsOn()
            reader.Update()
            data = reader.GetOutput()
            p = data.GetPointData()
            # if there is no data, continue
            if not p.HasArray("v"):
                continue
            #create group for current timestep
            grp = file.create_group("timestep "+str(timestep))
            timestep+=1
            time = int(filename.split(".vtk")[0].split("_")[-1]) * dt
            position = vtk_to_numpy(data.GetPoints().GetData())
            velocity = vtk_to_numpy(p.GetArray("v"))
            ID = vtk_to_numpy(p.GetArray("id"))
            radius = vtk_to_numpy(p.GetArray("radius"))
            type_ = vtk_to_numpy(p.GetArray("type"))
            if p.HasArray("f"):
                force = vtk_to_numpy(p.GetArray("f"))
                grp.create_dataset("force", data = force )

            # create datasets for each variables
            grp.create_dataset("time", data = time)
            grp.create_dataset("position", data = position)
            grp.create_dataset("velocity", data = velocity )
            grp.create_dataset("radius", data = radius )
            grp.create_dataset("ppcloud", data = np.zeros(len(type_)) )
            grp.create_dataset("spezies", data = type_ )
            grp.create_dataset("particleid", data = ID )



            #checking the min and ma position
            pos_min = np.min(position,axis = 0)
            pos_max = np.max(position,axis = 0)
            min_ = pos_min < min_val
            max_ = pos_max > max_val
            min_val[min_] = pos_min[min_]
            max_val[max_] = pos_max[max_]
        file.create_dataset("dimensions", data = np.asarray([min_val,max_val]))
        file.close()
