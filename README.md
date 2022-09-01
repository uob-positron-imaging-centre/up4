# up4: Universal Post-Processor for Particulate Processes
![example workflow](https://github.com/uob-positron-imaging-centre/up4/actions/workflows/main.yaml/badge.svg)

A fast flexible analysis toolset for particle data of all kinds.

Use it to compare interplatform particle results - either experimental
or synthetic data generated by different platforms like LIGGGHTS, MercuryDPM,
Barracuda and more.

up4 has a python API, which enables it to be easy to use for a wide range of
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
