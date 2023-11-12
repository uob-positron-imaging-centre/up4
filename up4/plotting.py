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
from upppp_rust import RustPlotter2D, RustGrid
from .grid import Grid


def save_fig(
    fig: plotly.graph_objects.Figure,
    filename: str,
    dpi: int,
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
    dpi : int
        DPI value for image.
    border_width : int, optional
        Width of margins of document (in mm), by default 20mm
    paper_width : int, optional
        Width of paper (in mm), by default 210mm (A4)
    """
    # plotly sets the width to 700 if not explicitly set
    pixel_width = fig.layout.width if fig.layout.width is not None else 700
    actual_width = paper_width - (2 * border_width)
    scale = (actual_width / 25.4) / (pixel_width / dpi)
    fig.write_image(filename, scale=scale)


class Plotter2D:
    def __init__(self, grid):
        if isinstance(grid, RustGrid):
            self._grid_type = "grid"
            self._plotter = RustPlotter2D._from_grid(grid)
        else:  # TODO be more thorough to please the german
            self._grid_type = "vector_grid"
            self._plotter = RustPlotter2D._from_vector_grid(grid)
    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def quiver_plot(
        self,
        axis: int,
        selection="depth_average",
        index=None,
        scaling_mode="none",
        scaling_args=None,
    ) -> plotly.graph_objects.Figure:
        self._plotter._quiver_plot(axis, selection, index, scaling_mode, scaling_args)
        quiver_plot = self._create_plot()

        return quiver_plot
    
    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def unit_vector_plot(
        self,
        axis: int,
        selection="depth_average",
        index=None,
        scaling_mode="none",
        scaling_args=None,
    ) -> plotly.graph_objects.Figure:
        self._plotter._unit_vector_plot(
            axis, selection, index, scaling_mode, scaling_args
        )
        unit_vector_plot = self._create_plot()

        return unit_vector_plot

    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def scalar_map(
        self,
        axis: int,
        selection="depth_average",
        index=None,
    ) -> plotly.graph_objects.Figure:
        self._plotter._scalar_map(self._grid_type, axis, selection, index)
        scalar_map_plot = self._create_plot()

        return scalar_map_plot

    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def scalar_contour(
        self,
        axis: int,
        selection="depth_average",
        index=None,
    ) -> plotly.graph_objects.Figure:
        self._plotter._scalar_contour(self._grid_type, axis, selection, index)
        scalar_contour_plot = self._create_plot()

        return scalar_contour_plot

    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def parity_plot(
        self, comparison_grid: Grid, axis: int, selection="depth_average", index=None
    ) -> plotly.graph_objects.Figure:
        if self._grid_type == "vector_grid":
            self._plotter._parity_plot_from_vector_grid(
                comparison_grid, axis, selection, index
            )
        else:
            self._plotter._parity_plot_from_grid(
                comparison_grid, axis, selection, index
            )
        parity_plot = self._create_plot()

        return parity_plot

    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def parity_map(
        self, comparison_grid: Grid, axis: int, selection="depth_average", index=None
    ) -> plotly.graph_objects.Figure:
        if self._grid_type == "vector_grid":
            self._plotter._parity_map_from_vector_grid(
                comparison_grid, axis, selection, index
            )
        else:
            self._plotter._parity_map_from_grid(comparison_grid, axis, selection, index)
        parity_map = self._create_plot()

        return parity_map

    # TODO add docstrings
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def parity_contour(
        self, comparison_grid: Grid, axis: int, selection="depth_average", index=None
    ) -> plotly.graph_objects.Figure:
        if self._grid_type == "vector_grid":
            self._plotter._parity_contour_from_vector_grid(
                comparison_grid, axis, selection, index
            )
        else:
            self._plotter._parity_contour_from_grid(
                comparison_grid, axis, selection, index
            )
        parity_contour = self._create_plot()

        return parity_contour

    # TODO consider forcing axes to be equal
    # TODO enable kwargs/ dict of figure properties to be passed to plotly
    def _create_plot(self) -> plotly.graph_objects.Figure:
        """
        Converts generated JSON string into Plotly figure.

        Returns
        -------
        plotly.graph_objects.Figure
            Plotly figure object.
        """
        fig = from_json(self._plotter.plotting_string)
        return fig
