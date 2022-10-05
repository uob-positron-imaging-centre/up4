# up4: Universal Post-Processor for Particulate Processes
![example workflow](https://github.com/uob-positron-imaging-centre/up4/actions/workflows/main.yaml/badge.svg)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
![docs](https://github.com/uob-positron-imaging-centre/up4/actions/workflows/pages.yaml/badge.svg)

A fast *and* flexible analysis toolset for particle data of all kinds
=====================================================================

Use it to compare inter-platform particle results - either experimental
or synthetic data generated by different platforms like [LIGGGHTS](https://www.cfdem.com/liggghtsr-open-source-discrete-element-method-particle-simulation-code), [MercuryDPM](https://www.mercurydpm.org/),
[Barracuda](https://cpfd-software.com/) and more.

`up4` has a python API, which enables it to be easy to use for a wide range of
scientists, but the core of it is implemented in [Rust](https://www.rust-lang.org/).

## Installation

### __1. HDF5__

One of the main requirements is the HDF5 library. To install the library we recommend using conda

```bash
conda install -c conda-forge hdf5==1.10.4
```

additionally, we recommend installing to export the following environment variables:

```bash
export HDF5_DIR="$CONDA_PREFIX"
```

for more information see  the [hdf5-rust](https://github.com/aldanor/hdf5-rust) instructions

### __2. Rust__

you will need a rust compiler, see:

<https://www.rust-lang.org/tools/install>

Rust dependencies will be installed automatically DURING compilation

### __3. Python__

Download or clone this repository and run:

```bash
    python3 -m pip install -r requirements.txt
```

after the requirements are installed, you can install the package with:

```bash
    python3 -m pip install .
```

this could take some time as it compiles the rust code on your machine.

To ensure that the installation was successful, you can run tests with [pytest](https://docs.pytest.org/en/7.1.x/getting-started.html):

```bash
    python3 -m pytest tests/test_all.py
```


# To DO / Dev notes
### Tests

- test failiures. make exceptations work
- make paths work also when pytested from different folder
- new, better and smaller test datasets
- tets converter::convertertools:{interpolate, velocity}

## Ideas

- save_constraints --> save the current particle selector constraints in the hdf5 file if file contains constraints, load them in Selector, however warn the user while loading, also make a reverse function to set selector to default state ( data.selector_default or similar)
- Align grids for comparison. Use some smart metric to align two grids and make diffs/or Hanquiao plots
- Statistics, is my data good enough? (how many traj do i need per cell to get good data)
- Data manipulation --> Rotate, translate system(with backup)
-

## Bugs

- bug in interpolator for csv files? there are zeros in 3u_HD1_glass

## Documentation
Extensive effort has gone into documenting *every* trait, function, struct, class and method in this library, so regardless of using this library in Rust *or* Python, there are no magic undocumented blocks of code. The Rust API reference can be found here 
## TODO add link 
and the Python API reference here 
## TODO add link. 
## TODO if possible to link between the 2 then mention that
## Help and Support
We recommend you check out our tutorials.
## TODO link
If your issue is not suitably resolved there, please check the [issues](https://github.com/uob-positron-imaging-centre/up4/issues) page on our GitHub. Finally, if no solution is available there, feel free to [open an issue](https://github.com/uob-positron-imaging-centre/up4/issues/new); the authors will attempt to respond as soon as possible.
## Performance
The core of this library is implemented in [Rust](https://www.rust-lang.org/), a modern systems language that natively boasts high performance and strong memory & thread safety. The [pyo3](https://github.com/PyO3/pyo3) Rust library is used to export the Rust backend as a Python library, so Rust knowledge *is not* a prerequisite for using `up4`. We deliberately chose a Rust-Python mix to ensure that the computationally intensive parts are *fast but readable* (rather than using an arcane mix of Cython and Numba) whilst maintaining an easy to use interface, which is where Python excels.
## Contributing
The `up4` library is *not* a one-man project; it is continually being worked on directly *and* indirectly by a dedicated group of granular materials researchers who are both tired of writing the *same* Python boilerplate to analyse experiments *and* waiting for said code to execute. If you are reading this, then it is likely that you feel at least one of these frustrations too! 

If you want to contribute to this project, there are many ways to help:
  - [Open an issue](https://github.com/uob-positron-imaging-centre/up4/issues/new)
  - Write a tutorial that fills in any gaps in the [`up4` documentation]() to help future users.
  - Share relevant algorithms/ functions that improve the post-processing process, so that others can benefit. If these come from published work, then please add a reference to this work in the code documentation.
  - Add your code directly to the repo and [open a pull request](https://github.com/uob-positron-imaging-centre/up4/compare).

## Citing
If you use this library in your research, you are kindly asked to cite:

> [Paper after publication]

## Licensing
## TODO add licence details

### Project to-do list

- [ ] HDF5 converter
  - [x] VTK -> HDF5
  - [ ] CSV -> HDF5
  - [ ] TXT -> HDF5
  - [ ] XLSX -> HDF5
- [ ] Plotting utilities
  - [ ] Vector plots
    - [ ] 2D
    - [ ] 3D
  - [ ] Heatmaps
  - [ ] Contour plots
  - [ ] Slice plots
  - [ ] Distributions
  - [ ] Parity plots
  - [ ] Recipes for above plots
- [ ] Vectorfield functions
  - [ ] Grid
    - [ ] Euler grid
    - [ ] Lagrange grid
- [ ] Equipment specific functions (or recipes?)
  - [ ] Rotating drum
  - [ ] Mills
  - [ ] Fluidised beds
- [ ] Generic particle functions (see old upppp)
  - [ ] Granular temperature distribution
  - [ ] Velocity distribution
- [ ] Documentation
  - [ ] Agree standards
  - [ ] Python
    - [ ] Configure Sphinx
    - [ ] Link rust docs (if possible)
  - [ ] Rust
- [ ] DEM contact list functions
  - [ ] Energy dissipated
  - [ ] Collision modes

## State of `up4`
### Plotting

- Rust
  - Limited ability to easily customise plots compared to the Python API. This is actively being worked on.
- Python
  - Export of created JSON strings to `Plotly.py` means that all tools are available to edit plot layout.
