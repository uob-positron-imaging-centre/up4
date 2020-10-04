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
    fig = None
    ):
    y=arr if axis == 2 else occu
    x=occu if axis == 2 else arr
    x_title = "Particle Volume Fraction [%]" if axis ==2 else "Width [m]"
    y_title =  "Height [m]" if axis ==2 else "Particle Volume Fraction [%]"
    if fig is None:
        fig = go.Figure()
    fig.add_trace(go.Scatter(x=x,y=y))
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
        
    fig.show()
    return fig
    
    
def plot_vectorfield(sx,sy,vx,vy,y_max = None,x_max = None,width=500, height=900):
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
        
    fig.show()
    return fig
    
    
    
def plot_image(img):
    import plotly.express as px
    fig = px.imshow((img), color_continuous_scale='gray')
    fig.show()
    
    
    
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
