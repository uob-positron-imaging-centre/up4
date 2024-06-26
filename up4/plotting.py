#!/usr/bin/env python
# -*-coding:utf-8 -*-
# File    :   __init__.py
# Time    :   02/09/2022
# Author  :   Dominik Werner, Daniel Weston
# Version :   0.1.0
# Contact :   dxw963@bham.ac.uk, dtw545@bham.ac.uk
# Licence :   GNU v3.0

from __future__ import annotations
from plotly.io import from_json
import plotly
from upppp_rust import RustPlotter2D, RustGrid, RustVectorGrid
from .grid import Grid


def save_fig(
    fig: plotly.graph_objects.Figure,
    filename: str,
    dpi: int = 600,
    border_width: int = 20,
    paper_width: int = 210,
) -> None:
    """
    Save Plotly figure with specified dpi.

    Parameters
    ----------
    fig : plotly.graph_objects.Figure
        Plotly figure to save.
    filename : str
        Filename for saved figure.
    dpi : int, optional
        DPI value for image, by default 600 dots per inch.
    border_width : int, optional
        Width of margins of document (in mm), by default 20mm
    paper_width : int, optional
        Width of paper (in mm), by default 210mm (A4)
    """
    # plotly sets the width to 700 if not explicitly set
    mm_to_inches = 25.4
    pixel_width = fig.layout.width if fig.layout.width is not None else 700
    actual_width = paper_width - (2 * border_width)
    scale = (actual_width / mm_to_inches) / (pixel_width / dpi)
    fig.write_image(filename, scale=scale)


class Plotter2D:
    def __init__(self, field: Grid):
        """
        Plotting class for 2D plots.

        Parameters
        ----------
        field : Grid
            An `up4.Grid` instance containing field data.
        """
        if isinstance(field, RustGrid):
            self._grid_type = "grid"
            self._plotter = RustPlotter2D._from_grid(field)
        elif isinstance(field, RustVectorGrid):
            self._grid_type = "vector_grid"
            self._plotter = RustPlotter2D._from_vector_grid(field)
        else:
            raise ValueError("Unknown grid type used.")

    def quiver_plot(
        self,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        scaling_mode: str = None,
        min_size: float = None,
        max_size: float = None,
        colour_map: str = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a quiver plot from the VectorGrid/Grid used to create
        this class instance.

        Parameters
        ----------
        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        scaling_mode : str, optional
            Method by which to scale vector arrows. Permitted values are:

            - "max": Arrows will be no longer than `max_size`
            - "min": Arrows will be no shorter than `min_size`
            - "minmax": Arrows will be no shorter than `min_size` and no longer than `max_size`
            - "half_node": Arrows will be no longer than half of the cell size.
            - "full_node": Arrows fill be no longer than the cell size.

            , by default None

        min_size : float, optional
            Minimum length of arrows, by default None

        max_size : float, optional
            Maximum length of arrows, by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Quiver plot.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.quiver_plot(axis, selection = "depth_average",
            scaling_mode = "half_node)
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "x/X [-]"
            ),
            yaxes = dict(
                title_text = "y/Y [-]"
            ),
        )
        >>> fig = plotter.quiver_plot(axis, selection = "depth_average",
            scaling_mode = "minmax", min_size = 0.01, max_size = 0.5, layout = layout)
        >>> fig.show()
        """
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
            scaling_mode=scaling_mode,
            min_size=min_size,
            max_size=max_size,
            colour_map=colour_map,
        )
        # Quiver plots are only available for VectorGrids
        if self._grid_type == "grid":
            raise ValueError(
                "Quiver plots are only available for `VectorGrid`s. "
                "Please use ensure that your input grid is of type `VectorGrid` "
                "method instead."
            )

        self._plotter._quiver_plot(
            axis,
            selection,
            index=index,
            scaling_mode=scaling_mode,
            min_size=min_size,
            max_size=max_size,
            colour_map=colour_map,
        )
        quiver_plot = self._create_plot(layout=layout, style=style)

        return quiver_plot

    # TODO keep an eye on this method to see if i need to mention anything about trace indices
    def unit_vector_plot(
        self,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        scaling_mode: str = None,
        min_size: float = None,
        max_size: float = None,
        colour_map: str = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a unit vector plot from the VectorGrid/Grid used to create
        this class instance.

        Parameters
        ----------
        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        scaling_mode : str, optional
            Method by which to scale vector arrows. Permitted values are:

            - "max": Arrows will be no longer than `scaling_args[0]`
            - "min": Arrows will be no shorter than `scaling_args[0]`
            - "minmax": Arrows will be no shorter than `scaling_args[0]` and no
                longer than `scaling_args[1]`
            - "half_node": Arrows will be no longer than half of the cell size.
            - "full_node": Arrows fill be no longer than the cell size.

            , by default None

        min_size : float, optional
            Minimum length of arrows, by default None

        max_size : float, optional
            Maximum length of arrows, by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Unit vector plot.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.unit_vector_plot(axis, selection = "depth_average",
            scaling_mode = "half_node)
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "x/X [-]"
            ),
            yaxes = dict(
                title_text = "y/Y [-]"
            ),
        )
        >>> fig = plotter.unit_vector_plot(axis, selection = "depth_average",
            scaling_mode = "minmax", scaling_args = [0.01, 0.5], layout = layout)
        >>> fig.show()
        """
        # Unit vector plots are only available for VectorGrids
        if self._grid_type == "grid":
            raise ValueError(
                "Unit vector plots are only available for `VectorGrid`s. "
                "Please use ensure that your input grid is of type `VectorGrid` "
                "method instead."
            )
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
            scaling_mode=scaling_mode,
            min_size=min_size,
            max_size=min_size,
            colour_map=colour_map,
        )
        self._plotter._unit_vector_plot(
            axis,
            selection,
            index=index,
            scaling_mode=scaling_mode,
            min_size=min_size,
            max_size=max_size,
            colour_map=colour_map,
        )
        unit_vector_plot = self._create_plot(style=style, layout=layout)

        return unit_vector_plot

    def scalar_map(
        self,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        colour_map: str = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a heatmap plot from the VectorGrid/Grid used to create
        this class instance.

        Parameters
        ----------
        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Heatmap plot.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.scalar_map(axis, selection = "depth_average")
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "x/X [-]"
            ),
            yaxes = dict(
                title_text = "y/Y [-]"
            ),
        )
        >>> fig = plotter.scalar_map(axis, selection = "depth_average",
           , layout = layout)
        >>> fig.show()
        """
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
            colour_map=colour_map,
        )
        self._plotter._scalar_map(
            self._grid_type,
            axis=axis,
            selection=selection,
            index=index,
            colour_map=colour_map,
        )
        scalar_map_plot = self._create_plot(style=style, layout=layout)
        return scalar_map_plot

    def scalar_contour(
        self,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        colour_map: str = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a contour plot from the VectorGrid/Grid used to create
        this class instance.

        Parameters
        ----------
        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Contour plot.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.scalar_map(axis, selection = "depth_average")
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "x/X [-]"
            ),
            yaxes = dict(
                title_text = "y/Y [-]"
            ),
        )
        >>> fig = plotter.scalar_map(axis, selection = "depth_average",
           , layout = layout)
        >>> fig.show()
        """
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
            colour_map=colour_map,
        )
        self._plotter._scalar_contour(
            self._grid_type,
            axis=axis,
            selection=selection,
            index=index,
            colour_map=colour_map,
        )
        scalar_contour_plot = self._create_plot(style=style, layout=layout)

        return scalar_contour_plot

    # TODO do i really need only a single plane or can i do this on the entire grid?
    def parity_plot(
        self,
        comparison_grid: Grid | RustVectorGrid,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a parity plot from the VectorGrid/Grid used to create
        this class instance and an additional VectorGrid/Grid to compare.

        Parameters
        ----------
        comparison_grid : Grid | VectorGrid
            Grid for comparison. Dimensions must be the same as the grid used to create
            this class instance.

        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Parity plot.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> comparison_data = up4.Data("path/to/another/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> grid_comparison = up4.Grid(data=comparison_data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> comparison_field = data.vectorfield(grid_comparison)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.parity_plot(comparison_field, axis, selection = "depth_average")
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> comparison_data = up4.Data("path/to/another/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> grid_comparison = up4.Grid(data=comparison_data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> comparison_field = data.vectorfield(grid_comparison)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "Measured values"
            ),
            yaxes = dict(
                title_text = "Predicted values"
            ),
        )
        >>> fig = plotter.parity_plot(comparison_field, axis, selection = "depth_average")
        >>> fig.show()
        """
        # TODO consider verifying shape matching at this layer
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
        )
        if self._grid_type == "vector_grid":
            self._plotter._parity_plot_from_vector_grid(
                comparison_grid, axis, selection, index
            )
        else:
            self._plotter._parity_plot_from_grid(
                comparison_grid, axis, selection, index
            )
        parity_plot = self._create_plot(style=style, layout=layout)

        return parity_plot

    def parity_map(
        self,
        comparison_grid: Grid | RustVectorGrid,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        colour_map: str = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a parity map from the VectorGrid/Grid used to create
        this class instance and an additional VectorGrid/Grid to compare.

        Parameters
        ----------
        comparison_grid : Grid | VectorGrid
            Grid for comparison. Dimensions must be the same as the grid used to create
            this class instance.

        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Parity map.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> comparison_data = up4.Data("path/to/another/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> grid_comparison = up4.Grid(data=comparison_data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> comparison_field = data.vectorfield(grid_comparison)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.parity_map(comparison_field, axis, selection = "depth_average")
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> comparison_data = up4.Data("path/to/another/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> grid_comparison = up4.Grid(data=comparison_data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> comparison_field = data.vectorfield(grid_comparison)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "Measured values"
            ),
            yaxes = dict(
                title_text = "Predicted values"
            ),
        )
        >>> fig = plotter.parity_map(comparison_field, axis, selection = "depth_average")
        >>> fig.show()
        """
        # TODO consider verifying shape matching at this layer
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
        )
        if self._grid_type == "vector_grid":
            self._plotter._parity_map_from_vector_grid(
                comparison_grid, axis, selection, index
            )
        else:
            self._plotter._parity_map_from_grid(comparison_grid, axis, selection, index)
        parity_map = self._create_plot(style=style, layout=layout)

        return parity_map

    def parity_contour(
        self,
        comparison_grid: Grid | RustVectorGrid,
        axis: int,
        selection: str = "depth_average",
        index: int = None,
        colour_map: str = None,
        layout: dict = None,
        style: dict = None,
    ) -> plotly.graph_objects.Figure:
        """
        Generate a parity contour from the VectorGrid/Grid used to create
        this class instance and an additional VectorGrid/Grid to compare.

        Parameters
        ----------
        comparison_grid : Grid | VectorGrid
            Grid for comparison. Dimensions must be the same as the grid used to create
            this class instance.

        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Parity contour.

        Examples
        --------

        Creating a simple plot:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> comparison_data = up4.Data("path/to/another/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> grid_comparison = up4.Grid(data=comparison_data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> comparison_field = data.vectorfield(grid_comparison)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> fig = plotter.parity_contour(comparison_field, axis, selection = "depth_average")
        >>> fig.show()

        Customising the layout:

        >>> import up4
        >>> data = up4.Data("path/to/hdf5/file")
        >>> comparison_data = up4.Data("path/to/another/hdf5/file")
        >>> grid_car = up4.Grid(data=data, num_cells=[20, 20, 20])
        >>> grid_comparison = up4.Grid(data=comparison_data, num_cells=[20, 20, 20])
        >>> vec_field = data.vectorfield(grid_car)
        >>> comparison_field = data.vectorfield(grid_comparison)
        >>> plotter = up4.Plotter2D(vec_field)
        >>> layout = dict(
            height = 800,
            width = 800,
            xaxes = dict(
                title_text = "Measured values"
            ),
            yaxes = dict(
                title_text = "Predicted values"
            ),
        )
        >>> fig = plotter.parity_contour(comparison_field, axis, selection = "depth_average")
        >>> fig.show()
        """
        # TODO consider verifying shape matching at this layer
        # Ensure that the Rust layer is passed valid parameters
        self._validate_input(
            axis,
            selection,
            index,
        )
        if self._grid_type == "vector_grid":
            self._plotter._parity_contour_from_vector_grid(
                comparison_grid, axis, selection, index
            )
        else:
            self._plotter._parity_contour_from_grid(
                comparison_grid, axis, selection, index
            )
        parity_contour = self._create_plot(style=style, layout=layout)

        return parity_contour

    def _validate_input(
        self,
        axis: int,
        selection: str,
        index: int,
        scaling_mode: str = None,
        min_size: float = None,
        max_size: float = None,
        colour_map: str = None,
    ) -> None:
        """
        Validate input before passing to Rust layer.

        Parameters
        ----------
        axis : int
            Axis which contains the perpendicular plane to be used. A value of 0 means
            that the yz plane is used, 1 for xz and 2 for yz. For a cylindrical grid,
            swap (x, y, z) for (r, theta, z) in these definitions.

        selection : str, optional
            Manner in which the plane is generated ("depth_average" or "plane"). Using
            "depth_average" will average values along the selected `axis` whereas
            "plane" simply selects the plane pointed to by the `index` value. Note that
            an `index` value is *mandatory* if using "plane" for `selection`. By default
            "depth_average".

        index : int, optional
            Location of plane along selected `axis` for `selection` = "plane",
            by default None

        scaling_mode : str, optional
            Method by which to scale vector arrows. Permitted values are:
            - "max": Arrows will be no longer than `scaling_args[0]`
            - "min": Arrows will be no shorter than `scaling_args[0]`
            - "minmax": Arrows will be no shorter than `scaling_args[0]` and no
                longer than `scaling_args[1]`
            - "half_node": Arrows will be no longer than half of the cell size.
            - "full_node": Arrows fill be no longer than the cell size.
            , by default None

        min_size : float, optional
            Minimum length of arrows, by default None

        max_size : float, optional
            Maximum length of arrows, by default None

        colour_map : str, optional
            Colourmap to use in the plot, case-insensitive. See the
            `colorous documentation <https://docs.rs/colorous/latest/colorous/>`
            for a list of valid colourmap values, by default None

        Raises
        ------
        ValueError
            Raised when invalid input is given.
        RuntimeWarning
            Raised when the number of elements in `scaling_args` is longer than needed.

        """
        valid_axis_values = [0, 1, 2]
        valid_selection_values = ["depth_average", "plane"]
        valid_scaling_mode_values = ["min", "max", "minmax", "half_node", "full_node"]
        valid_colour_map_values = [
            "turbo",
            "viridis",
            "inferno",
            "spectral",
            "magma",
            "plasma",
            "cividis",
            "warm",
            "cool",
            "cubehelix",
            "blue_green",
            "blue_purple",
            "green_blue",
            "orange_red",
            "purple_blue_green",
            "purple_blue",
            "purple_red",
            "red_purple",
            "yellow_green_blue",
            "yellow_green",
            "yellow_orange_brown",
            "yellow_orange_red",
            "blues",
            "greens",
            "greys",
            "oranges",
            "purples",
            "reds",
            "brown_green",
            "purple_green",
            "pink_green",
            "purple_orange",
            "red_blue",
            "red_grey",
            "red_yellow_blue",
            "red_yellow_green",
            "spectral",
        ]
        # Check for valid input
        if axis not in valid_axis_values:
            raise ValueError(
                f"Axis value: {axis} is not valid. Valid values are 0 (yz plane), 1 (xz"
                " plane) or 2 (xy plane) only."
            )
        if selection not in valid_selection_values:
            raise ValueError(
                f"Selection value: {selection} is not valid. Valid values are"
                ' "depth_average" or "plane".'
            )
        if selection == "plane" and index is None:
            raise ValueError(
                'Using selection: "plane" requires a value for `index` to be set'
            )
        if scaling_mode is not None and scaling_mode not in valid_scaling_mode_values:
            raise ValueError(
                f"Scaling mode value: {scaling_mode} is not valid. Valid values are"
                ' "min", "max", "minmax", "half_node", "full_node".'
            )
        if scaling_mode == "min" and min_size is None:
            raise ValueError(
                'Using scaling mode: "min" requires a value for `min_size` to be set'
            )
        if scaling_mode == "max" and max_size is None:
            raise ValueError(
                'Using scaling mode: "max" requires a value for `max_size` to be set'
            )
        if scaling_mode == "minmax" and (min_size is None or max_size is None):
            raise ValueError(
                'Using scaling mode: "minmax" requires values for `min_size` and '
                "`max_size` to be set"
            )
        if colour_map is not None and colour_map not in valid_colour_map_values:
            raise ValueError(
                f"Colourmap value: {colour_map} is not valid. Valid values are"
                f" {valid_colour_map_values}."
            )

    def _create_plot(self, layout: dict, style: dict) -> plotly.graph_objects.Figure:
        """
        Converts generated JSON string into Plotly figure.

        layout : dict, optional
            Dictionary with layout specifications for the plot. Nested dictionaries and
             plotly's "magic underscores" are supported. See the
             `plotly documentation <https://plotly.com/python/reference/layout/>` for a
             list of valid layout values, by default None

        style : dict, optional
            Dictionary with trace specifications for the plot. See the `plotly
            documentation <https://plotly.com/python/reference/scatter/>` for a
            list of valid layout values, by default None

        Returns
        -------
        plotly.graph_objects.Figure
            Plotly figure object.
        """

        fig = from_json(self._plotter.plotting_string)
        fig.plotly_update(style, layout)
        return fig
