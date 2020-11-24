#! /usr/bin/env/python3
# Author Dominik Werner
# Date: 18.Sep.2020

import pyAnalyser as pA

filename = "particle_data.pickle"
# Check the input data, if neceserry convert to HDF5 file

data = pA.Data(filename)

# Calculate a velocity vectorfield and plot the arrwos normalised
data.vectorfield(cells=[30,30,50], norm= True, plot = True)

# Calculate a occupancy plot and plot it normalised
data.occupancyplot1D(cells=100, norm = True, plot = True)


