#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# File   : setup.py
# License: GNU v3.0
# Author : Dominik Werner
# Date   : 08.09.2020



import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension



setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

with open("README.md", "r") as f:
    long_description = f.read()

setup(
    name="pyAnalyser",
    version="0.1.0",
    author = (
        "Dominik Werner <d.wer2@gmx.de>"
    ),
    classifiers=[
        "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)",
        "Development Status :: 2 - Pre-Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
        "Topic :: Scientific/Engineering",
        "Topic :: Scientific/Engineering :: Physics"
    ],
    url = "https://github.com/D-werner-bham/pyAnalyse",
    description = "analysing toolset for particle data",
    long_description = long_description,
    long_description_content_type = "text/markdown",
    packages=["pyAnalyser"],
    rust_extensions=[RustExtension("rustAnalyser")],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)
