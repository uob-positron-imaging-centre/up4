.. _vectorplots:

*********************
Vector Plots Tutorial
*********************

Vector quantities calculated by ``up4`` can be visualised by the `up4.Plotter2D`
class. For ease of use, this class takes 3D grids as an input, and will 
appropriately depth-average or slice the data for you. The `axis` argument to
methods defined in this class refers to which axis the depth-averaging/ slicing
will happen along (cylindrical equivalents in brackets):

- 0: slice along x(r)-axis, the plane for the data is y-z (:math:`\theta`-z).
- 1: slice along y(:math:`\theta`)-axis, the plane for the data is x-z (r-z).
- 2: slice along z(z)-axis, the plane for the data is x-y (r-:math:`\theta`).

Available vector plotting options are:

- `up4.Plotter2D.quiver_plot`
- `up4.Plotter2D.unit_vector_plot`

In the proceeding sections, we will focus on the visualisation of vector fields
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

Quiver Plot
===========
We can visualise the depth-averaged velocity field, a vector quantity using 
`up4.Plotter2D.quiver_plot`:

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    fig = plotter.quiver_plot(axis = axis, selection = "depth_average") # depth-average the vector grid 
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")

Alternatively, we could view just a slice of the y-z plane:

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    # select the y-z plane located at index 2, note that index is a required
    # argument for "plane" selection.
    fig = plotter.quiver_plot(axis = axis, selection = "plane", index = 2)  
    # from here, you can customise your figure however you like 
    fig.update_layout(width=800, height=800)
    fig.update_xaxes(title="y position (m)")
    fig.update_yaxes(title="z position (m)")

Note that these two examples could be combined into one, as a `up4.Plotter2D` instance
can generate multiple figures based on the input grid provided to it since its plotting
methods return a `plotly.graph_objects.Figure` instance.

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 

    depth_fig = plotter.quiver_plot(axis = axis, selection = "depth_average")  
    # from here, you can customise your figure however you like 
    depth_fig.update_layout(width=800, height=800)
    depth_fig.update_xaxes(title="y position (m)")
    depth_fig.update_yaxes(title="z position (m)")

    # the same Plotter2D instance as above is used here to generate a different figure
    plane_fig = plotter.quiver_plot(axis = axis, selection = "plane", index = 2)  
    # from here, you can customise your figure however you like 
    plane_fig.update_layout(width=800, height=800)
    plane_fig.update_xaxes(title="y position (m)")
    plane_fig.update_yaxes(title="z position (m)")

Arrow Scaling
-------------
Not all vector fields were made equal, some contain a large range of values that will
naturally make the resulting plot hard to read due to the difference in scales present.
We can rescale the arrows to improve the graph readability. The scaling options provided
by ``up4`` as optional arguments to its vector plotting functionality are as follows:

- "min": arrows below the "min" value are rescaled to "min".
- "max": arrows above the "max" value are rescaled to "max".
- "minmax": applies both "min" and "max" methods.
- "half_node": constrains arrows to fit within an ellipse with semi-principal axes 0.5dx and 0.5dy. 
- "full_node": constrains arrows to fit within an ellipse with semi-principal axes dx and dy. 

For the latter 2 methods, dx and dy refer to the grid spacing in the selected plane. The
"half_node" scaling ensures that no arrows will overlap, but may make arrows look really
far apart. The "full_node" scaling will make the plot appear less sparse, but risks arrow
overlap, depending on the vector orientation.

These methods will alter the arrow lengths non-uniformly, but they will still be coloured
according to their actual magnitude. If a system exhibits a truly large range of values,
it may be preferable to use the unit vector plot functionality.

Let's see some examples of these in action, shall we?

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 

    # all arrows at least as long as 0.1 units
    min_fig = plotter.quiver_plot(axis = axis, selection = "depth_average", 
                                    scaling_mode = "min", scaling_args = [0.1])  
    # all arrows are no longer than 0.5 units
    max_fig = plotter.quiver_plot(axis = axis, selection = "depth_average", 
                                    scaling_mode = "min", scaling_args = [0.5]) 
    # all arrows are at least 0.1 units long, but no longer than 0.5 units
    minmax_fig = plotter.quiver_plot(axis = axis, selection = "depth_average", 
                                    scaling_mode = "min", scaling_args = [0.1, 0.5])
    # all arrows have lengths no longer than 0.5*sqrt(dx**2 + dy**2)
    half_node_fig = plotter.quiver_plot(axis = axis, selection = "depth_average", 
                                    scaling_mode = "half_node")
    # all arrows have lengths no longer than sqrt(dx**2 + dy**2)
    full_node_fig = plotter.quiver_plot(axis = axis, selection = "depth_average", 
                                    scaling_mode = "full_node")

Notice that `scaling_args` *must* be a list!

Unit Vector Plot
================
The unit vector plot is a version of the quiver plot that is well-suited to systems with
a range of scales present, for instance if velocity magnitudes lie between >2 orders of
magnitude; a difficult problem for a normal quiver plot. The key differences between these
two plots are:

- Unit vector plots all have arrows with the same length.
- The background of a unit vector plot is shaded by the vector magnitude.

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 

    depth_fig = plotter.unit_vector_plot(axis = axis, selection = "depth_average")  
    # from here, you can customise your figure however you like 
    depth_fig.update_layout(width=800, height=800)
    depth_fig.update_xaxes(title="y position (m)")
    depth_fig.update_yaxes(title="z position (m)")

As before, perhaps we may be interested in a specific plane:

.. code-block:: python

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 
    # the same Plotter2D instance as above is used here to generate a different figure
    plane_fig = plotter.quiver_plot(axis = axis, selection = "plane", index = 2)  
    # from here, you can customise your figure however you like 
    plane_fig.update_layout(width=800, height=800)
    plane_fig.update_xaxes(title="y position (m)")
    plane_fig.update_yaxes(title="z position (m)")

Note that the plots discussed in the tutorials for :ref:`scalar plotting <scalarplots>`
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

.. code-block:: python

    # create a plot
    import up4
    drum_hdf5_file = '/path/to/hdf5/file'
    data = up4.Data(drum_hdf5_file)
    grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])

    vel_field = data.vectorfield(grid_car) # velocity vectorfield
    plotter = up4.Plotter2D(vel_field) # create a Plotter2D instance
    axis = 0 # interested in the y-z plane 

    depth_fig = plotter.unit_vector_plot(axis = axis, selection = "depth_average")  
    # from here, you can customise your figure however you like 
    depth_fig.update_layout(width=800, height=800)
    depth_fig.update_xaxes(title="y position (m)")
    depth_fig.update_yaxes(title="z position (m)")

    # now, save it with a required dpi
    dpi = 600 # typical requirement for many journals
    up4.save_fig(
        fig = fig, # figure to save
        filename = "velocity_field.png" # location to save file to
        dpi = dpi, # image dpi
        border_width = 20, # width of paper border (in mm)
        paper_width = 210 # a4 paper width (in mm)
    )