.. _scalarfield:

**********************
Scalar Fields Tutorial
**********************

Scalar fields are a powerful tool to represent the state of a system. ``up4`` provides
you with basic tools to extract an visualize different quantities.

Available are:

- `up4.Data.velocityfield`
- `up4.Data.numberfield`
- `up4.Data.occupancyfield`
- `up4.Data.granular_temperature`


To generate a 2D field one first needs to generate a 3D grid:


.. code-block:: python

    import up4
    data = up4.Data("test_filename.hdf5")
    grid = up4.Grid(data, num_cells=[50,50,50])

here, we first load the data using our  `up4.Data` class, then generate a 3-D
grid using the  `up4.Grid` class constructor. Read the :ref:`tutorial <grid_ref>`
for grids to see a deeper insight into the generating of grids.

Afterwards generate a field containing the velocity data. The data contains
the mean velocity of all  particle passes through that each cell.

.. code-block:: python

    field = data.velocityfield(grid)
    print(field)

    """
    Grid3D:
        Cells: [50, 50, 50]
        xlim: [174.02695648134886, 372.21089969650967]
        ylim: [74.71161491723201, 203.37554460929758]
        zlim: [52.291522049547666, 239.25612145076877]
        Data information:
                Mean: NaN
                Std: NaN
                Min: 0.0021862203945679478
                Max: 0.40294214528818545
    """

`up4.Grid` also contains the weights of each cell. Meaning that it stores
the number of particles that crossed the cell. These weights are accessible using
the  `up4.Grid.weights_to_numpy` method.
`up4.Grid` implements a crude plotting function that allows fast but
nonflexible visualisation.
This plotting function generates a depth-averaged 2D `Plotly <https://plotly.com/>`_
figure of the field and is not editable. Simply provide the axis along which the
depth averaging should take place to the function:

.. code-block:: python

    field.plot(0) # plot x-axis


To accurately visualize 2D fields there are two possibilities: Slices and depth averaged representations.

Slices are simple 2D representations of the 3D field at a given position along a given axis:

.. code-block:: python

    slice_x_axis = field.slice(0, 20) # slice the 3D array at the 20st cell index along x-axis

alternatively, one can slice the field at a given position:

.. code-block:: python

    slice_x_axis = field.slice_pos(0, 101.5) # slice the 3D array at x = 101.5 mm


Depth-averaging is achieved by calculating the accurate mean along each axis.
The `up4.Grid.collapse` function uses its internal weights for each cell to accurately
determine the mean without loss of information.

.. code-block:: python

    averaged_y_axis = field.collapse(1) # depth average along the y-axis

one can also generate a 1D representation of the system by collapsing two dimensions:



.. code-block:: python

    averaged_y_z_axis = field.collapse_two(1,2) # depth averaging along x and y axis


