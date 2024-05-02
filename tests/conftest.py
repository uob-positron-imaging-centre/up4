#!/usr/bin/env python
# -*-coding:utf-8 -*-
# File     :   conftest.py
# Time     :   02/05/2024
# Author   :   Daniel Weston
# Version  :   0.1.0
# Contact  :   dtw545@bham.ac.uk

from concurrent.futures import ThreadPoolExecutor as Executor
import requests
import os
import shutil

def download(url: str, folder: str, filenames: list[str], destination: str) -> None:
    """Download data from repository for tests

    Parameters
    ----------
    url : str
        Repository url.
    destination : str
        Destination folder.
    """
    destination = os.path.join(destination, folder)
    if not os.path.exists(destination):
        os.makedirs(destination)
    
    with Executor() as executor:
        # download files and set stream = True to avoid loading the content into memory
        futures = [executor.submit(requests.get, f"{url}/{folder}/{filename}", stream=True) for filename in filenames]
        for future, filename in zip(futures, filenames):
            with open(os.path.join(destination, filename), "wb") as file:
                file.write(future.result().content)

# Download files
def pytest_sessionstart(session):
    repo = "https://raw.githubusercontent.com/uob-positron-imaging-centre/up4-data/main"
    destination = os.path.join(os.path.dirname(__file__), "data")
    
    csv_folder = "csvs"
    csv_files = ["1p5u_HD1_glass.csv", "26mbq_day2.csv"]
    download(repo, csv_folder, csv_files , destination)

    starting_number = 1804000
    increment = 4000
    number_of_files = 11
    vtk_folder = "vtk/rotating-drum"
    vtk_files = [f"drum_{starting_number + increment * i}.vtk" for i in range(number_of_files)]
    download(repo, vtk_folder, vtk_files, destination)

    vtu_folder = "vtu/rotating-drum"
    vtu_files = [f"drum_{starting_number + increment * i}.vtu" for i in range(number_of_files)]
    download(repo, vtu_folder, vtu_files, destination)

def pytest_sessionfinish(session, exitstatus):
    destination = os.path.join(os.path.dirname(__file__), "data")
    shutil.rmtree(destination)