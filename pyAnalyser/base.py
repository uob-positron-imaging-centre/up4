#! /usr/bin/python3
import rustAnalyser as rust

import numpy as np
import os
import pickle
import h5py

inf = float('inf')
from .recovery import Center
from .plot import *
class Data():
    def __init__( self, filename, overwrite = False):
        # ckeck filename which format it has
        if filename.endswith( ".hdf5" ) or filename.endswith( "h5" ):
            # filename is a hdf5 file and can be used directly
            if os.path.exists(filename):
                self.filename = filename 
            else: 
                raise ValueError("Could not find HDF5 file. Wrong name/direction ?")
        elif filename.endswith( ".pickle" ):
            if os.path.exists(filename):
                self.transform_hdf5(filename,overwrite)
            else:
                raise ValueError("Could not fine pickle file. Wrong name/direction ?" )
            self.filename = filename.split( ".pickle" )[0] + ".hdf5"
        else:
            raise ValueError("File format not known. Check input or transform data to DEM-hdf5")
        
        self.recovery = Center(self.filename)
    
    def vectorfield(
        self,
        cells=[30.0,30.0,30.0],
        min_time =-inf, 
        max_time = inf,
        dimensions = [inf, inf, inf],
        norm = False, 
        return_data = False, 
        radius = 0.0,
        particle_id = 0,
        plot = True
    ):

        if isinstance(dimensions, (list, tuple, np.ndarray)):
            dimensions = np.asarray(dimensions, dtype=float)
        else:
            raise ValueError(f"dimensions should be a list/array not {type(dimensions)}")
            
        if isinstance(cells, (list, tuple, np.ndarray)):
            cells = np.asarray(cells, dtype=float)
        else:
            raise ValueError(f"Cells should be a list/array not {type(cells)}")
               
        if type(radius) == float or type(radius) == int:
            radius = np.asarray([radius, radius], dtype=float)
        elif isinstance(radius, (list, tuple, np.ndarray)):
            radius = np.asarray(radius, dtype=float)
        else:
            raise ValueError(f"radius should be a number or a list/array not {type(radius)}")
            
            
        if type(particle_id) == float or type(particle_id) == int:
            particle_id = np.asarray([int(particle_id), int(particle_id)], dtype=int)
        elif isinstance(particle_id, (list, tuple, np.ndarray)):
            particle_id = np.asarray(particle_id, dtype=int)
        else:
            raise ValueError(f"particle_id should be a number or a list/array not {type(particle_id)}")
      
            
        vx, vy, vz ,sx,sy = rust.vectorfield(self.filename,
                                            cells,
                                            min_time,
                                            max_time,
                                            dimensions,
                                            norm,
                                            radius,
                                            particle_id
                                            )
        self.recovery.add(np.asarray([vx, vy, vz ,sx,sy]))
        if plot:
            plot_vectorfield(sx,sy,vx,vz)
        return vx, vy, vz ,sx,sy
        
        
    def occupancyplot1D(
        self,
        cells=40,
        min_time =-inf,
        norm = False, 
        return_data = False, 
        radius = 0.0,
        particle_id = 0,
        clouds = False,
        axis=2,
        plot = True
    ):
 
        if type(cells) == float or type(cells) == int:
            cells = float(cells)
        else:
            raise ValueError(f"Cells should be a number not {type(cells)}")
               
        if type(radius) == float or type(radius) == int:
            radius = np.asarray([radius, radius], dtype=float)
        elif isinstance(radius, (list, tuple, np.ndarray)):
            radius = np.asarray(radius, dtype=float)
        else:
            raise ValueError(f"radius should be a number or a list/array not {type(radius)}")
            
            
        if type(particle_id) == float or type(particle_id) == int:
            particle_id = np.asarray([int(particle_id), int(particle_id)], dtype=int)
        elif isinstance(particle_id, (list, tuple, np.ndarray)):
            particle_id = np.asarray(particle_id, dtype=int)
        else:
            raise ValueError(f"particle_id should be a number or a list/array not {type(particle_id)}")
      
        if type(axis) == float or type (axis) == int:
            axis=int(axis)
        else:
            raise ValueError(f"Axis should be a number not {type(cells)}")
            
        occu,array = rust.occupancy_plot1D(self.filename,
                                            radius,
                                            particle_id,
                                            clouds,
                                            axis,
                                            norm,
                                            min_time,
                                            cells
                                         
                                            )
        if plot:
            plot_occu_1D(occu,array,axis)
        return occu, array
        
        
    def transform_hdf5(self, file, overwrite = False):
        """ pickle data to HDF5
        MUST be old DEM_analysis pickle format"""
        new_file = file.split(".pickle")[0]+".hdf5"
        if os.path.exists(file.split(".pickle")[0]+".hdf5") or os.path.exists(file.split(".pickle")[0]+".h5"):
            if not overwrite:
                input(f"WARNING!!! using equally named filename \"{new_file}\" instead of \"{file}\"\nToggle overwrite to True to overwrite ")
        data, min_array, max_array = pickle.load(open(file,"rb"))
        data = np.asarray(data)
        f = h5py.File(new_file,"w")
        f.create_dataset("dimensions", data = np.asarray([min_array,max_array]))
        for id,timestep in enumerate(data):
            time = timestep[0]
            particle_data  = timestep[1]
            group = f.create_group("timestep "+str(id) )
            
            p_data = np.asarray(particle_data,dtype=float)
            pos = p_data[:,0:3]
            vel = p_data[:,3:6]
            rad = p_data[:,9]
            cloud = p_data[:,8]
            spezies = p_data[:,6]
            particle_id =p_data[:,7]
            group.create_dataset("position",data = pos)
            group.create_dataset("velocity",data = vel)
            group.create_dataset("radius",data = rad)
            group.create_dataset("ppcloud",data = cloud)
            group.create_dataset("spezies",data = spezies)
            group.create_dataset("particleid",data = particle_id)
            group.create_dataset("time",data = time)
        
    def maketestfile():    
        
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
                rad = np.random.rand(particles)
                ids = np.arange(particles)
                for position in positions:
                    max_array[position > max_array] = position [position > max_array]
                    min_array[position < min_array] = position [position < min_array]

                grp = f.create_group(f"timestep {id}")
                grp.create_dataset("position",data = positions)
                grp.create_dataset("velocity",data = velocity)
                grp.create_dataset("particleid",data = ids)
                grp.create_dataset("radius",data = rad)
                grp.create_dataset("time", data = timestep)
        f.create_dataset("dimensions", data = np.asarray([min_array,max_array]))
        f.close()
        
    
     
