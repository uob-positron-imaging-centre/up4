#ignore this, this is to prototype stuff in python so i can see what i need
#from plotly.rs
"""
import plotly.graph_objects as go
import numpy as np


x = [1]
y = [1]
z = [1]
x, y, z = np.meshgrid(np.arange(-0.8,1,0.2),np.arange(-0.8,1,0.2),np.arange(-0.8,1,0.2))
u = [2]
v = [2.5]
w = [1]
norm = np.sqrt(u[0]*u[0]+v[0]*v[0]+w[0]*w[0])
xs = x
ys = y
zs = z
print([xs[0],ys[0],zs[0]])
fig = go.Figure(data=(go.Cone(x=x, y=y, z=z, u=u, v=v, w=w, sizemode='absolute', anchor = "tip"),go.Scatter3d(x=xs,y=ys,z=zs,mode='markers')))

fig.update_layout(scene_camera_eye=dict(x=-0.76, y=1.8, z=0.92))

fig.show()
"""


import plotly.figure_factory as ff
import plotly.graph_objects as go
import numpy as np
n = 10
tmp = np.arange(0,2*np.pi+np.pi/n,np.pi/n)
x, y = np.meshgrid(tmp,tmp)
u = np.sin(x)*np.cos(y)
v = -np.sin(y)*np.cos(x)
fig = ff.create_quiver(x,y,u,v)

#fig.update_layout(
#    width = 1000,
#    height = 1000,
#)
fig.update_xaxes(
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