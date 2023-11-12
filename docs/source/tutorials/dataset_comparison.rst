.. _datacomparison:

***************************
Dataset Comparison Tutorial
***************************

In the final part of visualising data with ``up4`` is comparing datasets. The whole point
of ``up4`` was to provide a transparent and uniform way of processing data, so we should
obviously be able to compare different datasets, right?!?

Available comparison plotting options are:

- `up4.Plotter2D.parity_plot`
- `up4.Plotter2D.parity_map`
- `up4.Plotter2D.parity_contour`

.. warning::
    All dataset plotting functionality assumes that the grids supplied have equal numbers
    of cells and spacing. Failure to ensure this will make the comparisons meaningless!

The terminology used within this plotting functionality refer to "reference" and "comparison"
data. The former refers to data that is passed to an `up4.Plotter2D` instance, and the
latter refers to data that you want to, *ahem* compare, your data to.

Parity Plot
-----------

We can perform a comparison of each and every cell's value and see, for instance, if the
values predicted by a simulation match experimental data:

.. code-block:: python

    import up4

Parity Map
----------

A parity plot sacrifices location information, whilst you can see if two datasets are
similar, you can't see *where* they disagree. A parity map retains location information
for the selected plane.

Parity Contour
--------------