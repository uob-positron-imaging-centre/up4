//! Timestep based HDF5 dataset manager
//!
//! Implementation of reading + buffering functions.

use super::{DataManager, GlobalStats, Manager, Timestep};
use itertools::Chunk;
use pyo3::prelude::*;
extern crate ndarray;
use crate::particleselector::Selector;
use crate::types::*;
use crate::ParticleSelector;
use crate::{print_debug, print_warning};
#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use ndarray::prelude::*;
use std::time::Instant;
const BUFFERSIZE: usize = 20000000;
#[pyclass]
pub struct TData {
    // A data managing system for HDF5 files in the
    // H5Part format or H5TimePart information
    file: hdf5::File,
    buffer: Vec<Timestep>,
    buffer_extra: Vec<Vec<Timestep>>,
    range: (usize, usize),
    range_extra: Vec<(usize, usize)>,
    pub buffersize: usize,
    pub buffersize_extra: Vec<usize>,
    single_data: Timestep,
    global_stats_: GlobalStats,
    rotation_is_set: bool,
    rotation: [f64; 3],
    rotation_center: [f64; 3],
    external_buffer_names: Vec<usize>,
}

impl TData {
    pub fn new(filename: &str) -> Self {
        #[cfg(feature = "blosc")]
        blosc_set_nthreads(8);
        print_debug!(
            "TData: Generating new instance with file: {}\
            and buffersize: {}",
            &filename,
            BUFFERSIZE
        );
        let file = hdf5::File::open(filename)
            .unwrap_or_else(|_| panic!("Can not read HDF5 file {}. ", &filename));
        let buffer = vec![Timestep::default(); BUFFERSIZE];
        let mut data = TData {
            file,
            buffer,
            buffer_extra: vec![],
            range: (0, 0),
            range_extra: vec![],
            buffersize: BUFFERSIZE,
            buffersize_extra: vec![],
            single_data: Timestep::default(),
            global_stats_: GlobalStats::default(),
            rotation_is_set: false,
            rotation: [0.0, 0.0, 0.0],
            rotation_center: [0.0, 0.0, 0.0],

            external_buffer_names: vec![],
        };
        print_debug!(
            "TData: Generation complete. First buffer update starting, buffer size: {}",
            data.buffer.len()
        );
        let global_stats = data.global_stats();
        data.global_stats_ = global_stats;
        data.update((0, data.buffersize));
        print_debug!("TData: Buffer updated. Returning initiated TData.");
        data
    }

    fn read_single(&self, timestep: usize) -> Timestep {
        print_debug!("TData: Reading a single timestep.");

        print_debug!("TData: Looping over all particles.");

        // for each particle find the Position at the
        let position = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find group \"timestep {}\" in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("position")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"position\" in file {}",
                    self.file.filename()
                )
            })
            .read_2d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read dataset \"position\" in file {}",
                    self.file.filename()
                )
            })
            .to_owned()
            .axis_iter(Axis(0))
            .map(|x| [x[0], x[1], x[2]])
            .collect::<Array1<Position>>();

        // for each particle find the velocuty at the
        let velocity = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find group \"timestep {}\" in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("velocity")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"velocity\" in file {}",
                    self.file.filename()
                )
            })
            .read_2d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read dataset \"velocity\" in file {}",
                    self.file.filename()
                )
            })
            .to_owned();
        //test

        //Radius
        let radius = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("radius")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                )
            })
            .read_1d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read data from \"radius\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                )
            })
            .to_owned();
        let density = match self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("density")
        {
            Ok(s) => s
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"density\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned(),
            Err(_) => {
                print_warning!(
                    "Can not find dataset \"density\" in file {} \
                    \n using ones instead ",
                    self.file.filename()
                );
                let mut arr = radius.clone();
                arr.fill(1.0);
                arr
            }
        };

        let clouds = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("ppcloud")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                )
            })
            .read_1d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read data from \"ppcloud\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                )
            })
            .to_owned();

        let time = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not fine timestep {} in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("time")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                )
            })
            .read_scalar::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                )
            })
            .to_owned();
        let particletype = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("particletype")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"particletype\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                )
            })
            .read_1d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read data from \"particletype\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                )
            })
            .to_owned();
        let particleid = self
            .file
            .group(&format!("timestep {}", timestep))
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                )
            })
            .dataset("id")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"id\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                )
            })
            .read_1d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read data from \"id\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                )
            })
            .to_owned();

        print_debug!("TData: Data read. Saving new timestap and return reference.");
        //let dt = Timestep::default();
        //self.single_data = dt;
        Timestep {
            time,
            position,
            velocity,
            radius,
            particleid,
            clouds,
            density,
            ptype: particletype,
        }
    }

    pub fn update(&mut self, mut range: (usize, usize)) {
        print_debug!("TData: Updating buffer");
        let timesteps = *self.global_stats_.timesteps();
        if range.1 >= timesteps {
            range.1 = timesteps;
        }
        self.buffersize = range.1 - range.0;

        print_debug!("TData: Looping over all particles:");
        for timestep in range.0..range.1 {
            // for each particle find the Position at the
            print_debug!("Timestep {}: Extracting position from HDF5.", timestep);
            let position = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find group \"timestep {}\" in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("position")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"position\" in file {}",
                        self.file.filename()
                    )
                })
                .read_2d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read dataset \"position\" in file {}",
                        self.file.filename()
                    )
                })
                .to_owned();
            let position = position
                .axis_iter(Axis(0))
                .map(|x| [x[0], x[1], x[2]])
                .collect::<Array1<Position>>();
            // for each particle find the velocuty at the
            let velocity = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find group \"timestep {}\" in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("velocity")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"velocity\" in file {}",
                        self.file.filename()
                    )
                })
                .read_2d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read dataset \"velocity\" in file {}",
                        self.file.filename()
                    )
                })
                .to_owned();

            //Radius
            let radius = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("radius")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"radius\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let density = match self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("density")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .unwrap_or_else(|_| {
                        panic!(
                            "Can not read data from \"density\" dataset. \
                            Data type or data format might be wrong. \
                            Check creation of HDF5 file  \"{:?}\"",
                            self.file.filename()
                        )
                    })
                    .to_owned(),
                Err(_) => {
                    print_warning!(
                        "Can not find dataset \"density\" in file {} \
                        \n using ones instead ",
                        self.file.filename()
                    );
                    let mut arr = radius.clone();
                    arr.fill(1.0);
                    arr
                }
            };

            let clouds = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("ppcloud")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"ppcloud\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();

            let time = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not fine timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("time")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_scalar::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"time\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let particleid = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("id")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"id\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"id\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let particletype = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("particletype")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"particletype\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"particletype\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();

            let dt = Timestep {
                time,
                position,
                velocity,
                radius,
                particleid,
                clouds,
                density,
                ptype: particletype,
            };
            print_debug!(
                "\tTimestep {}: Saving struct in buffer with len: {}.",
                timestep,
                self.buffer.len()
            );
            self.buffer[timestep - range.0] = dt;
        }
        self.range = range; //(range.0,range.1-1);
        print_debug!("TData: Finished buffer update")
    }

    pub fn update_extra(&mut self, mut range: (usize, usize), buffer_id: usize) {
        print_debug!("TData: Updating buffer");
        let timesteps = *self.global_stats_.timesteps();
        if range.1 >= timesteps {
            range.1 = timesteps;
        }
        self.buffersize_extra[buffer_id] = range.1 - range.0;

        print_debug!("TData: Looping over all particles:");
        for timestep in range.0..range.1 {
            // for each particle find the Position at the
            print_debug!("Timestep {}: Extracting position from HDF5.", timestep);
            let position = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find group \"timestep {}\" in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("position")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"position\" in file {}",
                        self.file.filename()
                    )
                })
                .read_2d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read dataset \"position\" in file {}",
                        self.file.filename()
                    )
                })
                .to_owned();
            let position = position
                .axis_iter(Axis(0))
                .map(|x| [x[0], x[1], x[2]])
                .collect::<Array1<Position>>();
            // for each particle find the velocuty at the
            let velocity = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find group \"timestep {}\" in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("velocity")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"velocity\" in file {}",
                        self.file.filename()
                    )
                })
                .read_2d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read dataset \"velocity\" in file {}",
                        self.file.filename()
                    )
                })
                .to_owned();

            //Radius
            let radius = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("radius")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"radius\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let density = match self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("density")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .unwrap_or_else(|_| {
                        panic!(
                            "Can not read data from \"density\" dataset. \
                            Data type or data format might be wrong. \
                            Check creation of HDF5 file  \"{:?}\"",
                            self.file.filename()
                        )
                    })
                    .to_owned(),
                Err(_) => {
                    print_warning!(
                        "Can not find dataset \"density\" in file {} \
                        \n using ones instead ",
                        self.file.filename()
                    );
                    let mut arr = radius.clone();
                    arr.fill(1.0);
                    arr
                }
            };

            let clouds = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("ppcloud")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"ppcloud\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();

            let time = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not fine timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("time")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_scalar::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"time\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let particleid = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("id")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"id\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"id\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let particletype = self
                .file
                .group(&format!("timestep {}", timestep))
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find timestep {} in file {}",
                        timestep,
                        self.file.filename()
                    )
                })
                .dataset("particletype")
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not find dataset \"particletype\" in HDF5 file \"{:?}\"",
                        self.file.filename()
                    )
                })
                .read_1d::<f64>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Can not read data from \"particletype\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    )
                })
                .to_owned();
            let dt = Timestep {
                time,
                position,
                velocity,
                radius,
                particleid,
                clouds,
                density,
                ptype: particletype,
            };
            print_debug!(
                "\tTimestep {}: Saving struct in buffer with len: {}.",
                timestep,
                self.buffer.len()
            );
            self.buffer_extra[buffer_id][timestep - range.0] = dt;
        }
        self.range_extra[buffer_id] = range; //(range.0,range.1-1);
        print_debug!("TData: Finished buffer update")
    }

    pub fn test(&mut self) {
        let now = Instant::now();
        let _time = self.global_stats_.max_time();
        let mut x = 0.;
        let selector = ParticleSelector::new(
            (f64::MIN, f64::MAX),
            vec![f64::MIN, f64::MAX],
            vec![f64::MIN, f64::MAX],
            vec![f64::MIN, f64::MAX],
            vec![usize::MIN, usize::MAX],
        );
        for timestep in 0..*self.global_stats_.timesteps() {
            let timestep_data = self.get_timestep(timestep);
            let positions: &Array1<Position> = timestep_data.position();
            let radius: &Array1<f64> = timestep_data.radius();
            let particleid: &Array1<f64> = timestep_data.particleid();
            let clouds: &Array1<f64> = timestep_data.clouds();
            let density: Array1<f64> = Array1::from_elem(clouds.len(), 1000.0);
            for (id, _position) in positions.outer_iter().enumerate() {
                if !selector.is_valid(radius[id], clouds[id], density[id], particleid[id] as usize)
                {
                    continue;
                }
                x += positions[0][0];
            }
        }
        println!("Elapsed time: {}", now.elapsed().as_millis());
        println!("x: {}", x);
    }
} //end impl

impl DataManager for TData {
    // return a array from a given dataset at a given timestep
    fn get_timestep(&mut self, timestep: usize) -> &Timestep {
        print_debug!("TData: Extracting timestep number {}", timestep);
        // If a timestep is requested that is higher then the current range
        // update buffer with new range
        print_debug!(
            "TData: Checking requested timestep {} in range {:?}",
            timestep,
            self.range
        );
        if timestep > self.range.1 - 1 {
            self.update((timestep, timestep + self.buffersize));
        } else if  timestep < self.range.0 {
            let chunk = (timestep as f64 / BUFFERSIZE as f64).floor() as usize;
            self.update((chunk * BUFFERSIZE, (chunk + 1) * BUFFERSIZE));
        }
        
        &self.buffer[timestep - self.range.0]
        
    }

    fn global_stats(&self) -> GlobalStats {
        let dimensions = self
            .file
            .attr("dimensions")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                )
            })
            .read_2d::<f64>()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read data from \"dimensions\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                )
            });
        let nparticles = self
            .file
            .attr("particle number")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"particle number\" in file {}",
                    self.file.filename()
                )
            })
            .read_scalar()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read scalar from attribute \"particle number\" in file {}",
                    self.file.filename()
                )
            });
        let timesteps = self
            .file
            .attr("timesteps")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"timesteps\" in file {}",
                    self.file.filename()
                )
            })
            .read_scalar()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read scalar from attribute \"timesteps\" in file {}",
                    self.file.filename()
                )
            });
        let time = self
            .file
            .attr("time")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"time\" in file {}",
                    self.file.filename()
                )
            })
            .read_raw()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read vector from attribute \"time\" in file {}",
                    self.file.filename()
                )
            });
        let sample_rate = self
            .file
            .attr("sample rate")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"sample rate\" in file {}",
                    self.file.filename()
                )
            })
            .read_scalar()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read scalar from attribute \"sample rate\" in file {}",
                    self.file.filename()
                )
            });
        let velocity = self
            .file
            .attr("velocity")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"velocity\" in file {}",
                    self.file.filename()
                )
            })
            .read_2d()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read scalar from attribute \"velocity\" in file {}",
                    self.file.filename()
                )
            });
        let velocity_mag = self
            .file
            .attr("velocity magnitude")
            .unwrap_or_else(|_| {
                panic!(
                    "Can not find attribute \"velocity magnitude\" in file {}",
                    self.file.filename()
                )
            })
            .read_1d()
            .unwrap_or_else(|_| {
                panic!(
                    "Can not read scalar from attribute \"velocity magnitude\" in file {}",
                    self.file.filename()
                )
            });
        let time_array = self
            .file
            .dataset("time array")
            .expect(
                "Can not find attribute \"time array\" in file, \
            You might have a old version of a HDF5 file. please include the \
            time as an array in attributes",
            )
            .read_1d()
            .expect("Can not read vector from attribute \"time array\" in file");
        GlobalStats {
            dimensions,
            nparticles,
            ntimesteps: timesteps,
            min_time: time[0],
            max_time: time[1],
            sample_rate,
            velocity,
            velocity_mag,
            time_array,
        }
    }
    fn stats(&self) {
        let global_stats = self.global_stats();
        let time = global_stats.max_time();
        let dim = global_stats.dimensions();
        println!("Dimensions of the system:: {:?}", dim);
        println!("The max time of this set is : {:?}", time);
        let part_num = global_stats.nparticles();
        println!("Number of Particles: {}", part_num);
        let vel_mag = global_stats.velocity_mag();
        println!("Mean velocity of: {} m/s", vel_mag[1usize]);
        println!(
            "Minimum velocity {} m/s \nMaximum Velocity {} m/s",
            vel_mag[0usize], vel_mag[2usize]
        );
    }

    fn get_timestep_unbuffered(&self, timestep: usize) -> Timestep {
        print_debug!("TData: Extracting timestep number {}", timestep);
        self.read_single(timestep)
    }
    /// setup a new buffer
    fn setup_buffer(&mut self, external_buffer_id: usize) {
        let range = (0usize, BUFFERSIZE);
        let buffer_id;
        if self.external_buffer_names.contains(&external_buffer_id) {
            // buffer is already know and will be reset
            buffer_id = self
                .external_buffer_names
                .iter()
                .position(|x| x == &external_buffer_id)
                .unwrap();
            self.range_extra[buffer_id] = range
        } else {
            // new buffer. Allocate new memory and load the first bit of data
            self.buffer_extra
                .push(vec![Timestep::default(); BUFFERSIZE]);
            buffer_id = self.buffer_extra.len() - 1;
            self.range_extra.push(range);
            self.buffersize_extra.push(BUFFERSIZE);
            self.external_buffer_names.push(external_buffer_id)
        }
        self.update_extra(range, buffer_id)
    }

    /// read from other buffer then the main one
    fn get_timestep_buffer(&mut self, timestep: usize, buffer_id: usize) -> &Timestep {
        print_debug!("PData: Extracting timestep number {}", timestep);
        // If a timestep is requested that is higher then the current range
        // update buffer with new range
        print_debug!(
            "PData: Checking requested timestep {} in range {:?}",
            timestep,
            self.range_extra[buffer_id]
        );
        if timestep > self.range_extra[buffer_id].1 - 1 {
            self.update_extra(
                (timestep, timestep + self.buffersize_extra[buffer_id]),
                buffer_id,
            );
        } else if self.range_extra[buffer_id].1 == self.global_stats_.ntimesteps
            && timestep < self.range_extra[buffer_id].0
        {
            self.update_extra((0, BUFFERSIZE), buffer_id);
        }
        // If a timestep below the current range is requested, update buffer
        if timestep < self.range.0 {
            self.update_extra(
                (timestep, timestep + self.buffersize_extra[buffer_id]),
                buffer_id,
            );
        }
        &self.buffer_extra[buffer_id][timestep - self.range_extra[buffer_id].0]
    }

    fn info(&self) -> Result<String, &'static str> {
        let global_stats = self.global_stats();
        let time = global_stats.max_time();
        let timesteps = global_stats.timesteps();
        let dim = global_stats.dimensions();
        let part_num = global_stats.nparticles();
        let vel_mag = global_stats.velocity_mag();
        let mut base_string = format!(
            "Time-Based HDF5 Dataset.\n\
            Dimensions of the system:\n\
            |   Min --> Max\t||   Diff\n\
            |x {:.2}-->{:.2}\t||  {:.2}\n\
            |y {:.2}-->{:.2}\t||  {:.2}\n\
            |z {:.2}-->{:.2}\t||  {:.2}\n\
            The max time of this set is : {:.2}\n\
            Number of Particles: {:.2}\n\
            Number of Timesteps: {:.2}\n\
            Mean velocity of: {:.2} m/s\n\
            Minimum velocity {:.2} m/s \nMaximum Velocity {:.2} m/s\n",
            dim[[0, 0]],
            dim[[1, 0]],
            dim[[1, 0]] - dim[[0, 0]],
            dim[[0, 1]],
            dim[[1, 1]],
            dim[[1, 1]] - dim[[0, 1]],
            dim[[0, 2]],
            dim[[1, 2]],
            dim[[1, 2]] - dim[[0, 2]],
            time,
            part_num,
            timesteps,
            vel_mag[1usize],
            vel_mag[0usize],
            vel_mag[2usize]
        );
        if self.rotation_is_set {
            base_string.push_str(&format!(
                "Rotation is set to: \n\
                \t x: {:.2} degree\n\
                \t y: {:.2} degree\n\
                \t z: {:.2} degree\n",
                self.rotation[0], self.rotation[1], self.rotation[2]
            ));

            base_string.push_str(&format!(
                "Rotation is performed around: \n\
                \t x: {:.2} degree\n\
                \t y: {:.2} degree\n\
                \t z: {:.2} degree\n",
                self.rotation_center[0], self.rotation_center[1], self.rotation_center[2]
            ))
        }
        Ok(base_string)
    }

    fn set_rotation_angle(&mut self, angle: f64, axis: f64) {
        self.rotation_is_set = true;
        self.rotation[axis as usize] = angle;
        // check if now all elements are zero
        if self.rotation.iter().all(|&x| x == 0.0) {
            self.rotation_is_set = false;
        }
    }

    fn set_rotation_anker(&mut self, point: [f64; 3]) {
        self.rotation_center = point;
    }
}

// Implement Manager which is just a sum of Granular + DataManager
impl Manager for TData {}
// TODO FUNCTIONS
// Write another class to get the posibility to buffer data from the pept experiment
//

// TODO PEPT/EXPERIMENTAL
// Manage timeframes: HOW?
// Solution1: define timestep in begining
// For return: interpolate between two points
// Solution2:
// for each new timestep call for the time
// to stuff accordingly
//

impl Drop for TData {
    fn drop(&mut self) {
        print_debug!("Killing TData.\nGoodbye :-)");
        //self.file.close();
    }
}
