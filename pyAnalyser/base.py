#! /usr/bin/python3
import rustAnalyser as rust

import numpy as np
from numpy.polynomial import Polynomial
import os
import h5py
from .recovery import Center
from .plot import *
from . import converter
inf = float('inf')

class Data():
    def __init__(self, filename, overwrite=False):
        # ckeck filename which format it has
        if filename.endswith(".hdf5") or filename.endswith("h5"):
            # filename is a hdf5 file and can be used directly
            if os.path.exists(filename):
                self.filename = filename
            else:
                raise ValueError("Could not find HDF5 file. Wrong name/direction ?")
        elif filename.endswith(".pickle"):
            if os.path.exists(filename):
                converter.barracuda(filename, overwrite)
            else:
                raise ValueError("Could not fine pickle file. Wrong name/direction ?")
            self.filename = filename.split(".pickle")[0] + ".hdf5"
        else:
            raise ValueError("File format not known. Check input or transform data to DEM-hdf5")

        self.recovery = Center(self.filename)

    def vectorfield(
        self,
        cells=[30.0,30.0,30.0],
        min_time =-inf,
        max_time = inf,
        dimensions = [[-inf, -inf, -inf], [inf, inf, inf]],
        norm = False,
        return_data = False,
        radius = -1.0,
        particle_id = -1,
        plot = True,
        width = 1000,
        height = 1000,
        axis = 0,
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


        vx, vz ,sx,sy = rust.vectorfield(self.filename,
                                            cells,
                                            min_time,
                                            max_time,
                                            dimensions,
                                            norm,
                                            radius,
                                            particle_id,
                                            axis,
                                            )
        self.recovery.add(np.asarray([vx, vz, sx, sy]))
        if plot:
            plot_vectorfield(sx,sy,vx,vz,width = width, height = height)
        return vx, vz, sx, sy


    def occupancyplot1d(
        self,
        cells=40,
        min_time =-inf,
        norm = False,
        return_data = False,
        radius = -1.0,
        particle_id = -1,
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

        occu,array = rust.occupancy_plot1d(self.filename,
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

    def gran_temp(self,min_time = -inf, plot = True):
        min_time = float(min_time)
        gt = rust.granular_temperature(self.filename, min_time)
        if plot:
            plot_linear(y=gt,x_lable="Timestep", y_lable="Granular Temperature")
        return gt
    def mean_velocity(self,min_time=0):
        return rust.mean_velocity(self.filename, float(min_time))

    def mean_angular_velocity(self,min_time=0):
        print("lol")
        return rust.mean_angular_velocity(self.filename, float(min_time))



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

    def surface_polynom(
        self,
        axis = 0,
        step = 0,

        steps_per_image = 10,
        threshold = 1.0,
        return_data = False,
        plot = True,
        width = 500,
        height = 900
    ):
        fig = None

        image,cell_len = rust.surface_polynom(self.filename,
                                            axis,
                                            step,
                                            steps_per_image,
                                            threshold
                                            )

        #plot_image (image)
        poly_surface, surface = self.extract_surface( image, cell_len)
        if plot:
            fig = plot_polynom(poly_surface, surface,fig, plot = True)
        return poly_surface

    def velocity_distribution(
            self,
            bins = -1,
            min_time = -inf,
            max_time = inf,
            plot = True,
            width = 1000,
            height = 1000
            ):

            if type(bins) == float:
                raise ValueError(f"Bins should be a integer not {type(bins)}")

            party_id, vel_dist, num_axis_array = rust.velocity_distribution(
                    self.filename,
                    bins = bins,
                    min_time = min_time,
                    max_time = max_time
                    )

            #self.recovery.add([party_id, vel_dist, num_axis_array])

            if plot:
                plot_velocity_distribution(vel_dist,
                                           num_axis_array,
                                           width = width,
                                           height = height)
            return vel_dist, num_axis_array
    def rotational_velocity_distribution(
            self,
            timestep,
            bins = -1,
            min_time = -inf,
            max_time = inf,
            rotation_speed = 45,
            drum_center =[0.0, 0., 0.],
            plot = True,
            width = 1000,
            height = 1000
            ):

            if type(bins) == float:
                raise ValueError(f"Bins should be a integer not {type(bins)}")

            party_id, vel_dist, num_axis_array = rust.rotational_velocity_distribution(
                    self.filename,
                    bins = bins,
                    min_time = min_time,
                    max_time = max_time,
                    rot_speed = rotation_speed,
                    drum_center = np.asarray(drum_center),
                    timestep=timestep
                    )

            #self.recovery.add([party_id, vel_dist, num_axis_array])

            if plot:
                plot_velocity_distribution(vel_dist,
                                           num_axis_array,
                                           width = width,
                                           height = height)
            return vel_dist, num_axis_array

    def mean_surface_velocity(
    self,
    bins = -1,
    min_time = -inf,
    max_time = inf,
    rotation_speed = 45,
    drum_center =[0.0, 0., 0.],):
        rust.mean_surface_velocity(self.filename,
        bins = bins,
        min_time = min_time,
        max_time = max_time,
        rot_speed = rotation_speed,
        drum_center = np.asarray(drum_center))

    def extract_surface(self,occupancy, cell_length):
        '''Extract the surface from a 2D occupancy plot (which is given as cells)
        and return a fitted 3rd order polynomial with the zeroth order coefficient
        set to 0.

        Parameters
        ----------
        occupancy: (M, N) numpy.ndarray
            Input occupancy plot - a 2D numpy array where 0 represents an empty
            cell, and a 1 represents the surface.

        cell_length: float
            The length of a single cell in the x- and y-dimensions.

        Returns
        -------
        numpy.polynomial.Polynomial
            The fitted 3rd order Polynomial, **with the 0th order coefficient set
            to zero**.
        '''

        # Extract the coordinates of the non-zero elements in `occupancy` and
        # multiply them by the cell length to get x and y points
        surface = np.argwhere(occupancy > 0.5)
        surface = surface * cell_length

        # Sort the points by the x coordinates, then y coordinates
        surface = surface[
            np.lexsort(
                (surface[:, 1], surface[:, 0])
            )
        ]

        # Take the highest point on the surface and clip its sides
        max_idx = surface[:, 1].argmax()

        surface_clipped = np.delete(surface, np.s_[:max_idx], axis = 0)
        surface_clipped = np.delete(surface_clipped, np.s_[-max_idx:], axis = 0)

        # Fit a 3rd order polynomial to the clipped surface
        surface_poly = Polynomial.fit(
            surface_clipped[:, 0],
            surface_clipped[:, 1],
            3
        )

        # Set the zeroth-order coefficient to 0 to "lower" the surface
        #surface_poly.coef[0] = 0

        return surface_poly, surface

    def dispersion(
        self,
        timestep=[0, 100 ],
        dt=10,
        mesh_size=[0.0,0.0,0.0],
        cells = [10,10,10],
        plot = True
    ):
            mesh_size = np.asarray(mesh_size)
            cells = np.asarray(cells)
            timestep = np.asarray(timestep,dtype=np.uintp)
            list, mixing_efficiency = rust.dispersion(
                self.filename,
                timestep,
                dt,
                mesh_size,
                cells,
                )
            if plot:
                plot_heatmap(np.asarray(list)[:,0,:])
            return list, mixing_efficiency

    def mean_squared_displacement(
        self,
        start_timestep = 0,
        plot = True
        ):
            start_timestep = int(start_timestep)

            msd_array, time_array = rust.mean_squared_displacement(
                self.filename,
                start_timestep
                )
            if plot:
                plot_MSD(msd_array,time_array)
            return msd_array, time_array

    def number_of_timesteps(self):
        return rust.timesteps(self.filename)

    def power_draw(self, min_time = 0, plot = True):
        power = rust.power_draw(self.filename, min_time)
        if plot:
            plot_linear(
            y = power,
            x_lable = "timestep",
            y_lable = "Power [W]"
            )
        return power
