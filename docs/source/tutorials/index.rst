*********
Tutorials
*********
The usual workflow in up4 consists of 4 steps: Converting, Generating grids, Analysis, and Plotting or short CGAP.
This tutorial will go through each element in CGAP and explain what it does and how to use it.

.. toctree::
   :maxdepth: 1

   scalar_fields
   grid_tutorial
   converting
   mixing
   conditional_analysis



Data analysis
----------------

Data analysis is done by methods of the :code:`up4.Data` class. The most important methods are:

- :code:`up4.Data.vectorfield`: Calculates the velocity vector field of the data
- :code:`up4.Data.velocityfield`: Calculates the magnitude velocity field of the data
- :code:`up4.Data.numberfield`: Calculates the number field of the data


.. code-block:: python

    import up4
    data = up4.Data("path/to/data.hdf5")
    grid_car = up4.Grid.cartesian_from_data(data, cells =[20,20,20])
    velocity_field = data.velocityfield(grid_car)


all functions can be called with any type of grid.
The velocity field in this example can also be calculated in cylindrical coordinates:

.. code-block:: python

    import up4
    data = up4.Data("path/to/data.hdf5")
    grid_cyl = up4.Grid.cylindrical_from_data(data, cells =[20,20,20])
    velocity_field = data.velocityfield(grid_cyl)

Plotting
--------

Plotting is done by one of the classes in :code:`up4.plotting`. These are:

- :code:`up4.plotting.VectorPlotter`: Calculates the velocity vector field of the data
- :code:`up4.plotting.ScalarPlotter`: Calculates the magnitude velocity field of the data
- :code:`up4.plotting.ComparisonPlotter`: Calculates the number field of the data

If we reuse the example from above, plotting can be done by adding the following lines:

.. code-block:: python

    from up4.plotting import VectorPlotter, ScalarPlotter
    dpi = 600 # nice quality image saving
    axis = 1 # look along y-axis
    index = 4
    vector_field = data.vector_field(grid) # for vector plotting

    # plot vector field
    vec_field_plotter = VectorPlotter(vector_field)
    vec_field_plotter.unit_vector_plot(vector_field)
    vec_fig = vec_field_plotter.plot()

Which, if used on the `tests/fixtures/drum.hdf5` file, looks a little like this:

.. image:: ../_static/unitv.png



