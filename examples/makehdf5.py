#! /usr/bin/python3

import h5py

f = h5py.File("mytestfile.hdf5","w")

import numpy as np
particles = 100
timesteps = 100

t_max = 10

times = np.linspace(0,t_max,timesteps)
max_array = np.asarray([float("-inf"),float("-inf"),float("-inf")])
min_array = np.asarray([float("inf"),float("inf"),float("inf")])
for id, timestep in enumerate(times):

        velocity =np.random.rand(particles,3)
        positions = np.random.rand(particles,3)
        for position in positions:
            max_array[position > max_array] = position [position > max_array]
            min_array[position < min_array] = position [position < min_array]

        grp = f.create_group(f"timestep {id}")
        grp.create_dataset("position",data = positions)
        grp.create_dataset("velocity",data = velocity)
        grp.create_dataset("time", data = timestep)
f.create_dataset("dimensions", data = np.asarray([min_array,max_array]))


