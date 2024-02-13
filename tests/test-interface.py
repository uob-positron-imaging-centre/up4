from pathlib import Path
import up4

path = Path(Path(__file__).parent, "fixtures/csvs")
up4.Converter.csv(
    str(Path(path, "flat_hd1_90lmin_f40_nEv200.csv")),
    outname=str(Path(path, "output.hdf5")),
    vel=True,
    delimiter=",",
    header = False,
)

data = up4.Data(str(Path(path, "output.hdf5")))
print(data)


size = 800
axes = [0, 1, 2]
xlabel = ["y", "x", "x"]
ylabel = ["z", "z", "y"]
grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
vec_field = data.vectorfield(grid_car)
plotter = up4.Plotter2D(vec_field)
for (axis, dimx, dimy) in zip(axes, xlabel, ylabel):
    fig = plotter.unit_vector_plot(axis, selection="depth_average")
    fig.update_layout(width=size, height=size)
    fig.update_xaxes(title=f"{dimx} position (mm)")
    fig.update_yaxes(title=f"{dimy} position (mm)")
    fig.update_traces(
        colorbar=dict(
            title=dict(text="Velocity (m s<sup>-1</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )
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

