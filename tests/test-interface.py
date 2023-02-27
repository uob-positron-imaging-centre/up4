from pathlib import Path
import up4
from natsort import natsorted

path = Path(Path(__file__).parent, "fixtures/post")
files = [str(file) for file in path.glob("*.vtk") if "boundingBox" not in str(file)]
files = natsorted(files)

up4.Converter.vtk(
    files,  # Sorted list of filenames
    1e-5,  # timestep of the simulation
    str(Path(path, "output.hdf5")),  # filename to write
    r"(\d+).vtk",  # regex to extract the timestep from the filename
)

data = up4.Data(str(Path(path, "output.hdf5")))
print(data)
grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])

vec_field = data.vectorfield(grid_car)

plotter = up4.Plotter2D(vec_field)

fig = plotter.quiver_plot(1, selection="depth_average", scaling_mode="full_node")
fig.show()

fig = plotter.quiver_plot(1, selection="plane", index=1, scaling_mode="full_node")
fig.show()

# TODO see what squaring the plot does to the arrows
fig = plotter.unit_vector_plot(1, selection="depth_average")
fig.update_yaxes(
    scaleanchor="x",
    scaleratio=1,
)
fig.show()

fig = plotter.unit_vector_plot(1, selection="plane", index=1)
fig.update_yaxes(
    scaleanchor="x",
    scaleratio=1,
)
fig.show()

fig = plotter.scalar_map(
    1,
    selection="depth_average",
)
fig.update_yaxes(
    scaleanchor="x",
    scaleratio=1,
)
fig.show()

fig = plotter.scalar_map(1, selection="plane", index=1)
fig.update_yaxes(
    scaleanchor="x",
    scaleratio=1,
)
fig.show()

fig = plotter.parity_plot(vec_field, 1)
fig.show()
