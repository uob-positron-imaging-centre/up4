
import numpy as np
from upppp_rust import Grid as rustGrid
from upppp_rust import Data

def calc_num_cells(cell_size,data,limits,xlim,ylim,zlim):
    if data is not None:
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
        num_cells = np.zeros(3)
        for i in range(3):
            num_cells[i] = int((dim[2*i+1]-dim[2*i])/cell_size[i])
        return num_cells
    elif limits is not None:
        num_cells = np.zeros(3)
        for i in range(3):
            num_cells[i] = int((limits[2*i+1]-limits[2*i])/cell_size[i])
        return np.array(num_cells, dtype=np.int64)


class Grid(rustGrid):
    def __new__(
        self,
        data = None,
        num_cells = [50, 50, 50],
        grid_style = "cartesian",
        cell_size = None,
        limits = None,
        xlim = None,
        ylim = None,
        zlim = None,
    ):
        # check types
        if not isinstance(limits,(list,np.ndarray)) and limits is not None:
            raise TypeError("limits must be a list or numpy array not a {}".format(type(limits)))
        elif limits is not None:
            limits = np.asarray(limits)
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
            if xlim[0] > xlim[1]:
                raise ValueError("xlim[0] must be smaller than xlim[1]")
        if not isinstance(ylim,(list,np.ndarray)) and ylim is not None:
            raise TypeError("ylim must be a list or numpy array")
        elif ylim is not None:
            ylim = np.asarray(ylim)
            if len(ylim) != 2:
                raise ValueError("ylim must have length 2")
            if ylim[0] > ylim[1]:
                raise ValueError("ylim[0] must be smaller than ylim[1]")
        if not isinstance(zlim,(list,np.ndarray)) and zlim is not None:
            raise TypeError("zlim must be a list or numpy array")
        elif zlim is not None:
            zlim = np.asarray(zlim)
            if len(zlim) != 2:
                raise ValueError("zlim must have length 2")
            if zlim[0] > zlim[1]:
                raise ValueError("zlim[0] must be smaller than zlim[1]")
        if not isinstance(data,Data) and data is not None:
            raise TypeError("data must be a up4-Data object")

        if not isinstance(grid_style,str) and grid_style is not None:
            raise TypeError("grid_style must be a string")
        if not grid_style in ["cartesian","cylindrical"]:
            raise ValueError("grid_style must be 'cartesian' or 'cylindrical'")

        if not isinstance(cell_size,(list, np.ndarray)) and cell_size is not None:
            raise TypeError("cell_size must be a list or numpy array")
        elif cell_size is not None:
            cell_size = np.asarray(cell_size)
            if len(cell_size) != 3:
                raise ValueError("cell_size must have length 3 containing the size of a single cell in x,y,z direction. Units are in file units")
            num_cells = calc_num_cells(cell_size,data,limits,xlim,ylim,zlim)
            print(num_cells)

        if data is None:
            # either limits or xlim,ylim,zlim must be given
            if limits is None and (xlim is None or ylim is None or zlim is None):
                raise ValueError("Either limits or xlim,ylim,zlim or a data object must be given")
            #but not both
            if limits is not None and (xlim is not None or ylim is not None or zlim is not None):
                raise ValueError("Either limits or xlim,ylim,zlim or a data object must be given")

        if grid_style == "cartesian":
            # data is given but no cell size or limits
            if data is not None and limits is None and xlim is  None and ylim is  None and zlim is  None:
                return self.cartesian3d_from_data(data, np.asarray(num_cells, dtype=np.int64))

            # data is given and cell size is given
            elif data is not None and limits is not None:
                if not any(i < 0 for i in limits):
                    return self.cartesian3d( np.asarray(num_cells), np.asarray(limits))
                else:
                    raise ValueError("limits must be positive")
            # data is given and at least one of xlim,ylim,zlim is not none
            elif data is not None and (xlim is not None or ylim is not None or zlim is not None):
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
                return self.cartesian3d( np.array(num_cells,dtype=np.int64), np.asarray(dim))
            elif data is None:
                if limits is not None:
                    if not any(i < 0 for i in limits):
                        return self.cartesian3d( np.asarray(num_cells), np.asarray(limits))
                    else:
                        raise ValueError("limits must be positive")
                elif xlim is not None and ylim is not None and zlim is not None:
                    dim = [xlim[0],xlim[1],ylim[0],ylim[1],zlim[0],zlim[1]]
                    return self.cartesian3d( np.asarray(num_cells), np.asarray(dim))
                else:
                    raise ValueError("Either limits or xlim,ylim,zlim must be given")
            else:
                raise ValueError("Something went wrong, you shouldn't be here. Contact developer")
        elif grid_style == "cylindrical":
            # data is given but no cell size or limits
            if data is not None and limits is None and xlim is  None and ylim is  None and zlim is  None:
                return self.cylindrical3d_from_data(data, np.asarray(num_cells))

            # data is given and cell size is given
            elif data is not None and limits is not None:
                if not any(i < 0 for i in limits):
                    return self.cylindrical3d( np.asarray(num_cells), np.asarray(limits))
                else:
                    raise ValueError("limits must be positive")
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
                if limits is not None:
                    if not any(i < 0 for i in limits):
                        return self.cylindrical3d( np.asarray(num_cells), np.asarray(limits))
                    else:
                        raise ValueError("limits must be positive")
                elif xlim is not None and ylim is not None and zlim is not None:
                    dim = [xlim[0],xlim[1],ylim[0],ylim[1],zlim[0],zlim[1]]
                    return self.cylindrical3d( np.asarray(num_cells), np.asarray(dim))
                else:
                    raise ValueError("Either limits or xlim,ylim,zlim must be given")
            else:
                raise ValueError("Something went wrong, you shouldn't be here. Contact developer")


