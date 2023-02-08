import up4
from up4.plotting import VectorPlotter
import glob

from natsort import natsorted
files = natsorted(glob.glob("data/*.vtk"))
files = glob.glob('/home/dan/LIGGGHTS/LIGGGHTS-PUBLIC/examples/LIGGGHTS/Tutorials_public/movingMeshGran/post/*.vtk')
files = [f for f in files if not "boundingBox" in f]
files = natsorted(files)
up4.Converter.vtk(
    files,                 # Sorted list of filenames
    1e-5,                  # timestep of the simulation
    'schulze.hdf5',         # filename to write
    #r"(\d+).vtk",         # regex to extract the timestep from the filename
)

filename = "schulze.hdf5"


data = up4.Data(filename)
grid = up4.Grid.cartesian3d_from_data(data,cells=[10,10,7])
# This generates a vectorgrid. not a normal grid!!
field = data.vectorfield(grid)


unitv_plotter = VectorPlotter(field)
unitv_plotter.unit_vector_plot(2, 3)
unitv_fig = unitv_plotter.plot()
unitv_fig.update_xaxes(title = dict(text = "x position (m)"))
unitv_fig.update_yaxes(title = dict(text = "z position (m)"))
unitv_fig.update_layout(title = "Velocity vector field xz projection")
unitv_fig.update_traces(
    colorbar = dict(
        title = dict(
            text = r"Magnitude $ms^{-1}$"
        ),
    ),
    #zmin = 0.,
    #zmax = 0.05,
    selector=dict(type='heatmap'))
#print(unitv_fig)
unitv_fig.show()