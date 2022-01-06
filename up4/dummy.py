#!/usr/bin/env python
# -*-coding:utf-8 -*-
#File    :   test.py
#Time    :   2022/01/06 00:10:42
#Author  :   Dominik Werner, Daniel Weston
#Version :   0.1.0
#Contact :   d.wer2@gmx.de, danw2697@btinternet.com
#Licence :   GNU v3.0
#Desc    :   None
import numpy as np
import textwrap 

class Dummy:
    """
    This is a class defined within Python and not in rust

    Parameters
    ----------
    Um:
        idk
    
    Notes
    -----
    Don't have any

    Examples
    --------
    None
    """
    def __init__(self, msg):
        self.msg = msg
        
    def __repr__(self):
        # String representation of the class
        name = "Test"
        underline = "-" * len(name)

        # Custom printing of the .lines and .samples_indices arrays
        with np.printoptions(threshold = 5, edgeitems = 2):
            pixels_str = f"{textwrap.indent(str(self.pixels), '  ')}"

        # Pretty-printing extra attributes
        attrs_str = ""
        if self.attrs:
            items = []
            for k, v in self.attrs.items():
                s = f"  {k.__repr__()}: {v}"
                if len(s) > 75:
                    s = s[:72] + "..."
                items.append(s)
            attrs_str = "\n" + "\n".join(items) + "\n"

        # Return constructed string
        return (
            f"{name}\n{underline}\n"
            f"msg = {self.msg}\n"
            "}\n"
        )