.. _scalarplots:

*********************
Scalar Plots Tutorial
*********************

Scalar quantities calculated by ``up4`` can be visualised by the `up4.Plotter2D`
class. For ease of use, this class takes 3D grids as an input, and will 
appropriately depth-average or slice the data for you. The `axis` argument to
methods defined in this class refers to which axis the depth-averaging/ slicing
will happen along (cylindrical equivalents in brackets):

- 0: slice along x(r)-axis, the plane for the data is y-z (:math:`\theta`-z).
- 1: slice along y(:math:`\theta`)-axis, the plane for the data is x-z (r-z).
- 2: slice along z(z)-axis, the plane for the data is x-y (r-:math:`\theta`).

Available scalar plotting options are:

- `up4.Plotter2D.scalar_map`
- `up4.Plotter2D.scalar_contour`

It is possible to use either a 3D scalar field (see :ref:`tutorial <scalarfield>`) or a
3D vector field, e.g. the velocity field. In the latter case, ``up4`` will
calculate the magnitude of the vector quantity after depth-averaging or slicing.

In the proceeding sections, we will focus on the visualisation of scalar fields
from a rotating drum, that rotates about the x-axis. The data for this can be found in
the ``tests/fixtures`` directory of ``up4``. To avoid unnecessary code repitition,
each example assumes that above it is the following code block:

.. code-block:: python

    import up4
    drum_hdf5_file = '/path/to/hdf5/file'
    data = up4.Data(drum_hdf5_file)
    grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])

Plotting with a cylindrical grid happens in an identical way, but using a Cartesian
grid for this example maintains the shape for the drum's free surface so that each plot
is easier to understand.

Heat Map
========
We can visualise the depth-averaged velocity field, a vector quantity, as a scalar field 
using `up4.Plotter2D.scalar_map`:

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    fig = plotter.scalar_map(axis = axis, selection = "depth_average") # depth-average the vector grid 
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")
    fig.update_traces(
        colorbar=dict(
            title=dict(text="Velocity (m s<sup>-1</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

Alternatively, we could view just a slice of the y-z plane:

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    # select the y-z plane located at index 2, note that index is a required
    # argument for "plane" selection.
    fig = plotter.scalar_map(axis = axis, selection = "plane", index = 2)  
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")
    fig.update_traces(
        colorbar=dict(
            title=dict(text="Velocity (m s<sup>-1</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

Note that these two examples could be combined into one, as a `up4.Plotter2D` instance
can generate multiple figures based on the input grid provided to it since its plotting
methods return a `plotly.graph_objects.Figure` instance.

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 

    depth_fig = plotter.scalar_map(axis = axis, selection = "depth_average")  
    # from here, you can customise your figure however you like 
    depth_fig.update_layout(width=800, height=800)
    depth_fig.update_xaxes(title="y position (m)")
    depth_fig.update_yaxes(title="z position (m)")
    depth_fig.update_traces(
        colorbar=dict(
            title=dict(text="Velocity (m s<sup>-1</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

    # the same Plotter2D instance as above is used here to generate a different figure
    plane_fig = plotter.scalar_map(axis = axis, selection = "plane", index = 2)  
    # from here, you can customise your figure however you like 
    plane_fig.update_layout(width=800, height=800)
    plane_fig.update_xaxes(title="y position (m)")
    plane_fig.update_yaxes(title="z position (m)")
    plane_fig.update_traces(
        colorbar=dict(
            title=dict(text="Velocity (m s<sup>-1</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

Contour
=======
Instead of the velocity field, what if we wanted to understand the quality of mixing
in a granular process. We can do this by drawing contours of the granular temperature,
as regions with higher values are better mixed.

.. code-block:: python

    gran_temp_field = data.granular_temperature(grid_car) # granular temperature field
    plotter = up4.Plotter2D(gran_temp_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    fig = plotter.scalar_contour(axis = axis, selection = "depth_average")  
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")
    fig.update_traces(
        colorbar=dict(
            title=dict(text="Granular Temperature (m<sup>2</sup> s<sup>-2</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

As before, perhaps we may be interested in a specific plane:

.. code-block:: python

    gran_temp_field = data.granular_temperature(grid_car) # granular temperature field
    plotter = up4.Plotter2D(gran_temp_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    # select the y-z plane located at index 2, note that index is a required
    # argument for "plane" selection.
    fig = plotter.scalar_contour(axis = axis, selection = "plane", index = 2)  
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")
    fig.update_traces(
        colorbar=dict(
            title=dict(text="Granular Temperature (m<sup>2</sup> s<sup>-2</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

Note that the plots discussed in the tutorials for :ref:`vector plotting <vectorplots>`
and :ref:`data comparison <datacomparison>` can also be used on the dataset used in these
tutorials, with the same `up4.Plotter2D` instance!

Formatting
==========

The methods of `up4.Plotter2D` return `plotly.graph_objects.Figure` instances, so you can
customise your plots to the same level of detail as natively using plotly. Examples of
this are shown at the bottom of each code block, where the x- and y-axes have been given
labels, and the colourbar has been given a title. 

The choice to do this is deliberate as the plotly API in Python and Rust is *substantially*
different. In Python, it is a fully object-oriented approach with a myriad of optional
arguments, something that Rust cannot handle ergonomically. In Rust, the plotly API is
instead following a functional paradigm. Thus, the choice was made that ``up4`` will instead
expose the figure in a manner compatible with the language's API.

Finally, saving static plotly images to a required dpi is supported in ``up4``:
.. TODO include using dicts
.. code-block:: python
    
    # create a plot
    import up4
    drum_hdf5_file = '/path/to/hdf5/file'
    data = up4.Data(drum_hdf5_file)
    grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    fig = plotter.scalar_map(axis = axis, selection = "depth_average") # depth-average the vector grid 
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")
    fig.update_traces(
        colorbar=dict(
            title=dict(text="Velocity (m s<sup>-1</sup>)"),
        ),
        selector=dict(type="heatmap"),
    )

    # now, save it with a required dpi
    dpi = 600 # typical requirement for many journals
    up4.save_fig(
        fig = fig, # figure to save
        filename = "velocity_field.png" # location to save file to
        dpi = dpi, # image dpi
        border_width = 20, # width of paper border (in mm)
        paper_width = 210 # a4 paper width (in mm)
    )