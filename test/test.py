import uPPPP as p
import time
import numpy as np



def test(data):
    cmds=[p.Data.vectorfield]
    result=[]
    for cmd in cmds:
        t = time.time()
        print(f"Testing {cmd.__name__}")
        try:
            result=cmd(data)
        except KeyboardInterrupt:
            print("Ending program")
            raise KeyboardInterrupt
        except Exception as e:
            with open(f"ERROR_{cmd.__name__}_{time.time()}.txt","w") as f:
                f.write("uPPPP Error:\n"+str(e))
            print(f"{cmd.__name__} failed after {time.time()-t} seconds")
            result.append([cmd.__name__,time.time()-t,"No"])
            continue
        print(f"{cmd.__name__} took {time.time()-t} seconds")
        result.append([cmd.__name__,time.time()-t,"Yes"])
    return result
    

    
if __name__=="__main__":
    print("Testing Simulation Data")
    data = p.Data.from_tdata("drum.hdf5")
    test(data)
    
    print("Testing Experimental Data")
    data = p.Data.from_pdata("HSM_Glass_2l_250.hdf5")
    test(data)
 
 
 
 
