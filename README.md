# up4: Universal Post-Processor for Particulate Processes

A fast flexible analysis toolset for particle data of all kinds.

Use it to compare interplatform particle results - either experimental
or synthetic data generated by different platforms like LIGGGHTS, MercuryDPM,
Barracuda and more.

up4 has a python surface, which enables it to be easy to use for a wide range of
scientists, but the core of it is implemented in RUST.


## Installation

Download or clone this repository and run:

    python3 -m pip install .

this could take some time as it compiles the rust code on your machine.

### Dependencies

you will need a rust compiler, see:

https://www.rust-lang.org/tools/install

Rust dependencys will be installed automatically before compilation


## To Do
### Tests
- test failiures. make exceptations work
- make paths work also when pytested from different folder
- new, better and smaller test datasets