#ignore this, this is to prototype stuff in python so i can see what i need
#from plotly.rs



import plotly.figure_factory as ff
import plotly.graph_objects as go
import numpy as np
x, y = np.meshgrid(np.arange(0,2*np.pi,np.pi/10),np.arange(0,2*np.pi,np.pi/10))
u = np.sin(x)*np.cos(y)
v = -np.sin(y)*np.cos(x)

fig = go.Figure(data=go.Cone(x=[1], y=[1], z=[1], u=[1], v=[1], w=[0]))

#fig.update_layout(
#    width = 1000,
#    height = 1000,
#)
fig.update_xaxes(
    #range=[-1,4],  # sets*the range of xaxis
    constrain="domain",  # meanwhile compresses*the xaxis by decreanp.sing its "domain"
)
fig.update_yaxes(
    scaleanchor = "x",
    scaleratio = 1,
)
# thus, we need to add the height, width, scale anchor and ratio to the serialise commands in plotly.rs

fig.show()
json = fig.to_json()
print(json)