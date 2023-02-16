from pathlib import Path
import up4
from glob import glob
from natsort import natsorted

path = glob(Path(Path(__file__).parent, "post"))
files = [file for file in path.glob("*.vtk") if "boundingBox" not in file]
files = natsorted(files)

up4.Converter.vtk(
    files,                 # Sorted list of filenames
    1e-5,                  # timestep of the simulation
    'output.hdf5',         # filename to write
    r"(\d+).vtk",          # regex to extract the timestep from the filename
)

data = up4.Data('output.hdf5')
print(data)
grid_car = up4.Grid(
    data = data,
    num_cells = [20, 20, 20]
)

vec_field = data.vector_field(grid_car)

plotter = up4.plotting.P2D(vec_field)

fig = plotter.quiver_plot(0)
fig.show()