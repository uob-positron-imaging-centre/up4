#!/usr/bin/env python
# -*-coding:utf-8 -*-
# File    :   __init__.py
# Time    :   02/09/2022
# Author  :   Dominik Werner, Daniel Weston
# Version :   0.1.0
# Contact :   dxw963@bham.ac.uk, dtw545@bham.ac.uk
# Licence :   GNU v3.0

from plotly.io import from_json
import plotly
from upppp_rust import Plotter2D


def save_fig(self, fig: plotly.graph_objects.Figure, filename: str, dpi: int) -> None:
    """
    Save Plotly figure with specified dpi. Currently, this method assumes A4 format
    with 20mm borders.

    Parameters
    ----------
    fig : plotly.graph_objects.Figure
        Plotly figure to save.
    filename : str
        Filename for saved figure.
    dpi : int
        DPI value for image.
    """
    # plotly sets the width to 700 if not explicitly set
    width = fig.layout.width if fig.layout.width is not None else 700
    scale = (170 / 25.4) / (width / dpi)
    fig.write_image(filename, scale=scale)


class P2D(Plotter2D):
    def __init__(self, grid):
        Plotter2D._from_vector_grid(grid)

    def quiver_plot(self, axis: int) -> plotly.graph_objects.Figure:
        self._quiver_plot(axis)
        quiver_plot = self._create_plot()

        return quiver_plot

    def _create_plot(self) -> plotly.graph_objects.Figure:
        """
        Converts generated JSON string into Plotly figure.

        Returns
        -------
        plotly.graph_objects.Figure
            Plotly figure object.
        """
        fig = from_json(self.plotting_string)
        return fig
