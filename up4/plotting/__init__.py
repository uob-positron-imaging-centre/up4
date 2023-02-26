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


def save_fig(
    self,
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


class P2D:
    def __init__(self, grid):
        self.plotter = Plotter2D._from_vector_grid(grid)

    def quiver_plot(
        self,
        axis: int,
        selection=None,
        index=None,
        scaling_mode=None,
        scaling_args=None,
    ) -> plotly.graph_objects.Figure:
        self.plotter._quiver_plot(axis, selection, index, scaling_mode, scaling_args)
        quiver_plot = self._create_plot()

        return quiver_plot

    def unit_vector_plot(
        self,
        axis: int,
        selection=None,
        index=None,
        scaling_mode=None,
        scaling_args=None,
    ) -> plotly.graph_objects.Figure:
        self.plotter._unit_vector_plot(
            axis, selection, index, scaling_mode, scaling_args
        )
        unit_vector_plot = self._create_plot()

        return unit_vector_plot

    def _create_plot(self) -> plotly.graph_objects.Figure:
        """
        Converts generated JSON string into Plotly figure.

        Returns
        -------
        plotly.graph_objects.Figure
            Plotly figure object.
        """
        fig = from_json(self.plotter.plotting_string)
        return fig
