from pathlib import Path
import up4
from natsort import natsorted
import plotly.graph_objects as go

path = Path(Path(__file__).parent, "fixtures/post")
files = [str(file) for file in path.glob("*.vtk") if "boundingBox" not in str(file)]
files = natsorted(files)

# up4.Converter.vtk(
#     files,  # Sorted list of filenames
#     1e-5,  # timestep of the simulation
#     str(Path(path, "output.hdf5")),  # filename to write
#     r"(\d+).vtk",  # regex to extract the timestep from the filename
# )
path = Path(Path(__file__).parent, "fixtures/csvs")
up4.Converter.csv(
    str(Path(path, "1p5u_HD1_glass.csv")),
    outname=str(Path(path, "output.hdf5")),
    vel=True,
    delimiter=" ",
)

data = up4.Data(str(Path(path, "output.hdf5")))


print(data)
axis = 2
grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])

vec_field = data.vectorfield(grid_car)
numberfield = data.granular_temperature(grid_car)
plotter = up4.Plotter2D(vec_field)

# fig = plotter.quiver_plot(axis, selection="depth_average", scaling_mode="full_node")
# fig.update_layout(width=600, height=600)
# fig.show()

# fig = plotter.quiver_plot(axis, selection="plane", index=1, scaling_mode="full_node")
# fig.update_layout(width=600, height=600)
# fig.show()

# TODO see what squaring the plot does to the arrows
fig = plotter.unit_vector_plot(axis, selection="dominik")
fig.update_layout(width=600, height=600)
fig.update_xaxes(title="x position (mm)")
fig.update_yaxes(title="y position (mm)")
fig.update_traces(
    colorbar=dict(
        title=dict(text="<u<sup>2</sup>> J kg<sup>-1</sup>"),
    ),
    selector=dict(type="heatmap"),
)
print(fig)
fig.show()

# fig = plotter.unit_vector_plot(axis, selection="plane", index=1)
# fig.update_layout(width=600, height=600)

# fig.show()

# fig = plotter.scalar_map(
#     1,
#     selection="depth_average",
# )
# fig.update_layout(width=600, height=600)

# fig.show()

# fig = plotter.scalar_map(axis, selection="plane", index=1)
# fig.update_layout(width=600, height=600)
# fig.show()

# fig = plotter.scalar_contour(
#     1,
#     selection="depth_average",
# )
# fig.update_layout(width=600, height=600)

# fig.show()

# fig = plotter.scalar_contour(axis, selection="plane", index=1)
# fig.update_layout(width=600, height=600)

# fig.show()


# fig = plotter.parity_plot(vec_field, 1)
# fig.show()

z = data.velocityfield(grid_car).to_numpy()
print(z)
fig = go.Figure(data=go.Heatmap(z=z))
# fig.show()
