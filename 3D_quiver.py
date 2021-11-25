import plotly.graph_objects as go
import numpy as np
fig = go.Figure(data=go.Cone(x=[1], y=[1], z=[1], u=[1], v=[1], w=[0]))

fig.update_layout(scene_camera_eye=dict(x=-0.76, y=1.8, z=0.92))
json = fig.to_json()
print(json)
fig.show()