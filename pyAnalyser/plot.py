import plotly.graph_objects as go

import plotly.figure_factory as ff

import numpy as np
def plot_occu_1D(
    occu,
    arr,
    axis,
    height = 900,
    width = 500,
    y_max = None,
    x_max = None,
    fig = None,
    name = "",
    plot = True
    ):
    y=arr if axis == 2 else occu
    x=occu if axis == 2 else arr
    x_title = "Particle Volume Fraction Normalized [-]" if axis ==2 else "Width [m]"
    y_title =  "Height [m]" if axis ==2 else "Particle Volume Fraction Normalized [-]"
    if fig is None:
        fig = go.Figure()
    fig.add_trace(go.Scatter(x=x,y=y, name = name))
    fig.update_layout(
            #autosize=False,
            width=width,
            height=height,
            xaxis_title= x_title,
            yaxis_title= y_title)
    if y_max is not None:
        fig.update_yaxes(range=[0.0, y_max])
    if x_max is not None:
        fig.update_xaxes(range=[0.0, x_max])

    if plot:
        fig.show()
    return fig


def plot_vectorfield(sx,sy,vx,vy,y_max = None,x_max = None,width=1000, height=1000, norm = False, plot = True):
    if norm:
        norm = np.sqrt( vx*vx+vy*vy)
        vx = vx/norm
        vy = vy/norm
    fig = ff.create_quiver(sx, sy, vx, vy,
                   scale=.004,
                   arrow_scale=.4,
                   name='quiver',
                   line_width=1)
    fig.update_layout(
        autosize=False,
        width=width,
        height=height,
        xaxis_title= "Width [m]",
        yaxis_title= "Height [m]")

    if y_max is not None:
        fig.update_yaxes(range=[0.0, y_max])
    if x_max is not None:
        fig.update_xaxes(range=[0.0, x_max])
    if plot:
        fig.show()
    return fig


def plot_image(img, plot = True):
    import plotly.express as px
    fig = px.imshow((img), color_continuous_scale='gray')
    if plot:
        fig.show()


def plot_heatmap(array, plot=True):
    array = np.flip(np.rot90(array,-1), axis =1)
    fig = go.Figure(data=go.Heatmap(
                    z=array))
    if plot:
        fig.show()
    return fig

def plot_velocity_distribution(
            vel_dist,
            num_axis_array,
            fig = True,
            width = 1000,
            height = 1000,
            plot = True
            ):

    # Axis Titles
    x_title = ' Velocity (m/s) '
    y_title = ' Number of Particles '

    # Plotting the Figure
    fig = go.Figure()
    fig.add_trace(go.Scatter(x=num_axis_array,y=vel_dist))
    fig.update_layout(
            width = width,
            height = height,
            xaxis_title = x_title,
            yaxis_title = y_title,
            )

    #fig.update_yaxes(type="log")
    if plot:
        fig.show()
    return fig

def plot_polynom( surface_poly, surface = None, fig=None, plot = True):
    if fig is  None:
        fig = go.Figure()
    x = np.linspace(surface_poly.domain[0], surface_poly.domain[1], 1000)
    fig.add_trace(go.Scatter(x = x, y = surface_poly(x)))
    if surface is not None:
        fig.add_trace(go.Scatter(x = surface[:, 0], y = surface[:, 1]))
    if plot:
        fig.show()
    return fig

def plot_MSD(msd, time, fig=None, plot = True,log = True):
    if fig is None:
        fig = go.Figure()
    fig.add_trace(go.Scatter(x = time, y = msd, mode="markers"))
    if log:
        fig.update_xaxes(type="log")
        fig.update_yaxes(type="log")
    fig.update_layout(
        xaxis_title="Time [s]",
        yaxis_title="Mean Squared Displacement [m²]"
    )
    if plot:
        fig.show()
    return fig

def plot_linear(
x = None,
y = None,
x_lable = "",
y_lable = "",
fig = None,
plot = True
):
    if fig is None:
        fig = go.Figure()
    fig.add_trace(go.Scatter(x = x, y = y))
    fig.update_layout(
        xaxis_title=x_lable,
        yaxis_title=y_lable
    )
    if plot:
        fig.show()
    return fig
