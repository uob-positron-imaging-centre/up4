
import numpy as np
from upppp_rust import Grid as rustGrid
from upppp_rust import Data


class Grid(rustGrid):
    def __new__(
        self,
        num_cells = [50, 50, 50],
        grid_style = "cartesian",
        cell_size = None,
        xlim = None,
        ylim = None,
        zlim = None,
        data = None,
    ):
        # check types
        if not isinstance(cell_size,(list,np.ndarray)) and cell_size is not None:
            raise TypeError("cell_size must be a list or numpy array not a {}".format(type(cell_size)))
        elif cell_size is not None:
            cell_size = np.asarray(cell_size)
        if not isinstance(num_cells,(list,np.ndarray)) and num_cells is not None:
            raise TypeError("num_cells must be a list or numpy array not a {}".format(type(num_cells)))
        elif num_cells is not None:
            num_cells = np.asarray(num_cells)
            if len(num_cells) != 3:
                raise ValueError("num_cells must have length 3")
        if not isinstance(xlim,(list,np.ndarray)) and xlim is not None:
            raise TypeError("xlim must be a list or numpy array")
        elif xlim is not None:
            xlim = np.asarray(xlim)
            if len(xlim) != 2:
                raise ValueError("xlim must have length 2")
        if not isinstance(ylim,(list,np.ndarray)) and ylim is not None:
            raise TypeError("ylim must be a list or numpy array")
        elif ylim is not None:
            ylim = np.asarray(ylim)
            if len(ylim) != 2:
                raise ValueError("ylim must have length 2")
        if not isinstance(zlim,(list,np.ndarray)) and zlim is not None:
            raise TypeError("zlim must be a list or numpy array")
        elif zlim is not None:
            zlim = np.asarray(zlim)
            if len(zlim) != 2:
                raise ValueError("zlim must have length 2")
        if not isinstance(data,Data) and data is not None:
            raise TypeError("data must be a up4-Data object")

        if not isinstance(grid_style,str) and grid_style is not None:
            raise TypeError("grid_style must be a string")
        if not grid_style in ["cartesian","cylindrical"]:
            raise ValueError("grid_style must be 'cartesian' or 'cylindrical'")

        if data is None:
            # either cell_size or xlim,ylim,zlim must be given
            if cell_size is None and (xlim is None or ylim is None or zlim is None):
                raise ValueError("Either cell_size or xlim,ylim,zlim or a data object must be given")
            #but not both
            if cell_size is not None and (xlim is not None or ylim is not None or zlim is not None):
                raise ValueError("Either cell_size or xlim,ylim,zlim or a data object must be given")

        if grid_style == "cartesian":
            # data is given but no cell size or limits
            if data is not None and cell_size is None and xlim is  None and ylim is  None and zlim is  None:
                return self.cartesian3d_from_data(data, np.asarray(num_cells))

            # data is given and cell size is given
            elif data is not None and cell_size is not None:
                if not any(i < 0 for i in cell_size):
                    return self.cartesian3d( np.asarray(num_cells), np.asarray(cell_size))
                else:
                    raise ValueError("cell_size must be positive")
            # data is given and at least one of xlim,ylim,zlim is not none
            elif data is not None and xlim is not None and ylim is not None and zlim is not None:
                xmin,ymin,zmin = data.min_position()
                xmax,ymax,zmax = data.max_position()
                if xlim is not None:
                    xmin = xlim[0]
                    xmax = xlim[1]
                if ylim is not None:
                    ymin = ylim[0]
                    ymax = ylim[1]
                if zlim is not None:
                    zmin = zlim[0]
                    zmax = zlim[1]
                dim = [xmin,xmax,ymin,ymax,zmin,zmax]
                return self.cartesian3d( np.asarray(num_cells), np.asarray(dim))
            elif data is None:
                if cell_size is not None:
                    if not any(i < 0 for i in cell_size):
                        return self.cartesian3d( np.asarray(num_cells), np.asarray(cell_size))
                    else:
                        raise ValueError("cell_size must be positive")
                elif xlim is not None and ylim is not None and zlim is not None:
                    dim = [xlim[0],xlim[1],ylim[0],ylim[1],zlim[0],zlim[1]]
                    return self.cartesian3d( np.asarray(num_cells), np.asarray(dim))
                else:
                    raise ValueError("Either cell_size or xlim,ylim,zlim must be given")

        elif grid_style == "cylindrical":
            # data is given but no cell size or limits
            if data is not None and cell_size is None and xlim is  None and ylim is  None and zlim is  None:
                return self.cylindrical3d_from_data(data, np.asarray(num_cells))

            # data is given and cell size is given
            elif data is not None and cell_size is not None:
                if not any(i < 0 for i in cell_size):
                    return self.cylindrical3d( np.asarray(num_cells), np.asarray(cell_size))
                else:
                    raise ValueError("cell_size must be positive")
            # data is given and at least one of xlim,ylim,zlim is not none
            elif data is not None and xlim is not None and ylim is not None and zlim is not None:
                xmin,ymin,zmin = data.min_position()
                xmax,ymax,zmax = data.max_position()
                if xlim is not None:
                    xmin = xlim[0]
                    xmax = xlim[1]
                if ylim is not None:
                    ymin = ylim[0]
                    ymax = ylim[1]
                if zlim is not None:
                    zmin = zlim[0]
                    zmax = zlim[1]
                dim = [xmin,xmax,ymin,ymax,zmin,zmax]
                return self.cylindrical3d( np.asarray(num_cells), np.asarray(dim))
            elif data is None:
                if cell_size is not None:
                    if not any(i < 0 for i in cell_size):
                        return self.cylindrical3d( np.asarray(num_cells), np.asarray(cell_size))
                    else:
                        raise ValueError("cell_size must be positive")
                elif xlim is not None and ylim is not None and zlim is not None:
                    dim = [xlim[0],xlim[1],ylim[0],ylim[1],zlim[0],zlim[1]]
                    return self.cylindrical3d( np.asarray(num_cells), np.asarray(dim))
                else:
                    raise ValueError("Either cell_size or xlim,ylim,zlim must be given")



