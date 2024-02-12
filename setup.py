#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# File   : setup.py
# License: GNU v3.0
# Author : Dominik Werner
# Date   : 08.09.2020


import sys
from setuptools.command.install import install
from setuptools import setup, find_packages
import numpy as np
import os

debug = os.getenv("UPPP_DEBUG")
if debug == "true":
    features = ["uPPPP_debug", "python"]
else:
    features = ["python"]

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding


setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

with open("README.md", "r") as f:
    long_description = f.read()

setup(
    name="up4",
    version="0.1.0",
    author="Dominik Werner <dxw963@bham.ac.uk>",
    classifiers=[
        "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)",
        "Development Status :: 2 - Pre-Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
        "Topic :: Scientific/Engineering",
        "Topic :: Scientific/Engineering :: Physics",
    ],
    url="https://github.com/D-werner-bham/pyAnalyse",
    description="analysing toolset for particle data",
    long_description=long_description,
    long_description_content_type="text/markdown",
    packages=find_packages(exclude=["tests", "*.tests", "*.tests.*", "tests.*"]),
    package_data={"up4": ["py.typed"]},
    rust_extensions=[
        RustExtension("upppp_rust", features=features, binding=Binding.PyO3)
    ],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)
