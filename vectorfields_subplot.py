from plotly.subplots import make_subplots
import plotly.graph_objects as go
import up4
from up4.plotting import VectorPlotter
import glob
import numpy as np
import plotly.figure_factory as ff
# we make 2D vectorfields


def make_vectorfield(data, cx=40, cy=40, cz=30, dimensions=None, mask=None, plot=False):
    print(data)
    data = up4.Data(file)

    grid = up4.Grid.cartesian3d_from_data(data,cells=[cx,cy,cz])

    # This generates a vectorgrid. not a normal grid!!
    field = data.vectorfield(grid)
    unitv_plotter = VectorPlotter(field)
    unitv_plotter.unit_vector_plot(0,5)
    fig = unitv_plotter.plot()


    fig.update_layout(
            width = 1200,
            height = 1000,
            template = "plotly_white", # A lot different templates available
            font = dict(
                size= 30,
                family = "Computer Modern"
            )
        )
    fig.update_yaxes(
        scaleanchor = "x",
        scaleratio = 1,
    )




    fig['data'][0]['line']['color'] = "black"

    fig.update_layout(
        autosize=False,
        width=1000,
        height=1000,
        paper_bgcolor="white",  # 'rgba(0,0,0,0)',
        plot_bgcolor="white",  # 'rgba(0,0,0,0)',
        yaxis=dict(
            # range=[0,0.35],
            domain=[0.13, 0.99],
            # anchor="x2",
            # overlaying="y2",
            title=r"$\Large{\text{Height [m]}}$",
            side="left",
            tickcolor="black",
            ticks="outside",
            tickwidth=6,
            ticklen=20,
            mirror=True,
            showline=True, linewidth=6, linecolor="black", gridcolor="rgba(0,0,0,0.3)", gridwidth=1, zerolinecolor="black", zerolinewidth=2),
        xaxis=dict(
            # range=[0,0.095],
            title_standoff=0,
            title=r"$\Large{\text{Width [m]}}$",
            tickcolor="black",
            ticks="outside",
            tickwidth=6,
            ticklen=20,
            showline=True, linewidth=6, linecolor="black", gridcolor="rgba(0,0,0,0.3)", gridwidth=1, zerolinecolor="black", zerolinewidth=2,
        ),
        font=dict(
            #family="Courier New, monospace",
            size=20,
            # color="RebeccaPurple"
        ),
        legend=dict(
            yanchor="top",
            y=0.95,
            xanchor="right",
            x=0.20,
            bgcolor="white", bordercolor="Black",
            borderwidth=2
        ),
    )
    #fig.show()
    return fig


subplot_title_on = False
axis_scaling = False
files = np.asarray([
    #["10_10_1.5_r.hdf5", 1.0, 1.5],
    #["10_10_2.5_r.hdf5", 1.0, 2.5],
    #["10_10_2_r.hdf5", 1.0, 2.0],
    ["15_10_1.5.hdf5", 0.6666666666666666, 1.5],
    ["15_10_2.hdf5", 0.6666666666666666, 2.0],
    ["15_10_2.5.hdf5", 0.6666666666666666, 2.5],
    ["15_15_1.5.hdf5", 1.0, 1.5],
    ["15_15_2.hdf5", 1.0, 2.0],
    ["15_15_2.5.hdf5", 1.0, 2.5],
    ["15_20_1.5.hdf5", 1.3333333333333333, 1.5],
    ["15_20_2.5.hdf5", 1.3333333333333333, 2.5],
    ["15_20_2.hdf5", 1.3333333333333333, 2.0],
    ["15_30_1.5.hdf5", 2.0, 1.5],
    ["10_20_2.hdf5", 2.0, 2.0],
    ["15_30_2.5.hdf5", 2.0, 2.5],
    #["10_20_2.5_r.hdf5", 2.0, 2.5],
    #["10_20_2.5.hdf5", 2.0, 2.5],
    ["20_10_2.5.hdf5", 0.5, 2.5],
    #["20_20_2.5.hdf5", 1.0, 2.5],
    #["20_40_1.5.hdf5", 2.0, 1.5],
    ["20_10_1.5.hdf5", 0.5, 1.5],
    ["20_10_2.hdf5", 0.5, 2.0],
    #["20_20_1.5.hdf5", 1.0, 1.5],
    #["20_20_2.hdf5", 1.0, 2.0],
    #["20_40_2.hdf5", 2.0, 2.0],
])
h_dratio = []
flow = []
for file in files:
    file = file[0]
    dia = float(file.split("_")[0])
    height = float(file.split("_")[1])
    flow_rate = float(file.split("_")[2].split(".hdf5")[0])
    print(f"[\"{file}\", {height/dia}, {flow_rate}],")
    h_dratio.append(height/dia)
    flow.append(flow_rate)

cols = len(np.unique(h_dratio))
rows = len(np.unique(flow))
fig = make_subplots(rows=rows, cols=cols,
                    vertical_spacing=0.05, horizontal_spacing=0.05,)
col = 0
row = 0
plot_id = 0
for unique_flow in np.unique(flow)[::-1]:
    row += 1
    col = 0
    for unique_ratio in sorted(np.unique(h_dratio)):
        plot_id += 1
        col += 1
        idx = np.where((np.array(flow) == unique_flow) &
                       (np.array(h_dratio) == unique_ratio))
        file = np.array(files)[idx][0][0]
        data = up4.Data(file)
        print(file)
        vec_fig = make_vectorfield(data, cx=10, cy=30, cz=30)
        # add all data to the figure:
        for i in range(0, len(vec_fig.data)):
            fig.add_trace(vec_fig.data[i], row=row, col=col)

        title = f"Flow rate: {unique_flow}, Height/Width: {unique_ratio}"
        if subplot_title_on:
            fig.layout.annotations[plot_id-1]["text"] = title


fig.update_layout(template="plotly_white", width=2000, height=2000)

for key in fig.layout.to_plotly_json().keys():
    if key.startswith("yaxis"):
        fig.layout[key]["showticklabels"] = False
        num = key.split("yaxis")[1]
        if axis_scaling:
            fig.layout[key]["scaleanchor"] = "x"+str(num)
            fig.layout[key]["scaleratio"] = 1
    if key.startswith("xaxis"):
        fig.layout[key]["showticklabels"] = False
fig.show()