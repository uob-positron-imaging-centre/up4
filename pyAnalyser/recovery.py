



import time
import h5py
import numpy as np
 
class Center():
    def __init__(self,filename, max_ = 10):
        self.filename = filename
        with h5py.File(filename,"r+") as f:
            if "recovery" in f:
                self._max_save = f["recovery"].attrs["max"]
            else:
                grp = f.create_group("recovery")
                grp.attrs["max"] = max_ 
                self._max_save = f["recovery"].attrs["max"]
   
    def add(self, array):
        # add a new array in recovery list
        # Delete the oldest if necessery

        with h5py.File(self.filename,"r+") as f:
            grp = f["recovery"]
            keys = grp.keys()
            if len(keys) >= self._max_save:
                names = list(keys)
                times =[grp[x].attrs["time"] for x in names]
                oldest = np.argmax(times)
                del grp[ names[oldest] ]
            time.sleep(1)
            t= time.localtime()
            name = time.strftime("%d_%m_%Y_time_%H_%M_%S",t)
            data = grp.create_dataset(name, data = array)
            data.attrs["time"] = t
            
    def iter(self):
        with h5py.File(self.filename,"r+") as f:
            grp = f["recovery"]
            keys = list(grp.keys())
            for name in keys:
                yield list(grp[name])
            
