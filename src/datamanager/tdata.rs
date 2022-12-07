//! Timestep based HDF5 dataset manager
//!
//! Implementation of reading + buffering functions.

use super::{DataManager, GlobalStats, Manager, Timestep};
use pyo3::prelude::*;
extern crate ndarray;
use crate::particleselector::Selector;

use crate::types::*;
use crate::ParticleSelector;
use crate::{print_debug, print_warning};
use ndarray::prelude::*;
use std::time::Instant;
const BUFFERSIZE: usize = 100;
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
}

impl TData {
    pub fn new(filename: &str) -> Self {
        print_debug!(
            "TData: Generating new instance with file: {}\
            and buffersize: {}",
            &filename,
            BUFFERSIZE
        );
        let file =
            hdf5::File::open(filename).expect(&format!("Can not read HDF5 file {}. ", &filename));
        let buffer = vec![Timestep::default(); BUFFERSIZE];
        let mut data = TData {
            file: file,
            buffer: buffer,
            buffer_extra: vec![],
            range: (0, 0),
            range_extra: vec![],
            buffersize: BUFFERSIZE,
            buffersize_extra: vec![],
            single_data: Timestep::default(),
            global_stats_: GlobalStats::default(),
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
            .expect(&format!(
                "Can not find group \"timestep {}\" in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("position")
            .expect(&format!(
                "Can not find dataset \"position\" in file {}",
                self.file.filename()
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read dataset \"position\" in file {}",
                self.file.filename()
            ))
            .to_owned()
            .axis_iter(Axis(0))
            .map(|x| [x[0], x[1], x[2]])
            .collect::<Array1<Position>>();

        // for each particle find the velocuty at the
        let velocity = self
            .file
            .group(&format!("timestep {}", timestep))
            .expect(&format!(
                "Can not find group \"timestep {}\" in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("velocity")
            .expect(&format!(
                "Can not find dataset \"velocity\" in file {}",
                self.file.filename()
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read dataset \"velocity\" in file {}",
                self.file.filename()
            ))
            .to_owned();

        //Radius
        let radius = self
            .file
            .group(&format!("timestep {}", timestep))
            .expect(&format!(
                "Can not find timestep {} in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("radius")
            .expect(&format!(
                "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                self.file.filename()
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"radius\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()
            ))
            .to_owned();
        let density = match self
            .file
            .group(&format!("timestep {}", timestep))
            .expect(&format!(
                "Can not find timestep {} in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("density")
        {
            Ok(s) => s
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"density\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
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
            .expect(&format!(
                "Can not find timestep {} in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("ppcloud")
            .expect(&format!(
                "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                self.file.filename()
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"ppcloud\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()
            ))
            .to_owned();

        let time = self
            .file
            .group(&format!("timestep {}", timestep))
            .expect(&format!(
                "Can not fine timestep {} in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                self.file.filename()
            ))
            .read_scalar::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()
            ))
            .to_owned();
        let particletype = self
            .file
            .group(&format!("timestep {}", timestep))
            .expect(&format!(
                "Can not find timestep {} in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("particletype")
            .expect(&format!(
                "Can not find dataset \"particletype\" in HDF5 file \"{:?}\"",
                self.file.filename()
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"particletype\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()
            ))
            .to_owned();
        let particleid = self
            .file
            .group(&format!("timestep {}", timestep))
            .expect(&format!(
                "Can not find timestep {} in file {}",
                timestep,
                self.file.filename()
            ))
            .dataset("id")
            .expect(&format!(
                "Can not find dataset \"id\" in HDF5 file \"{:?}\"",
                self.file.filename()
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"id\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()
            ))
            .to_owned();

        let dt = Timestep {
            time: time,
            position: position,
            velocity: velocity,
            radius: radius,
            particleid: particleid,
            clouds: clouds,
            density: density,
            ptype: particletype,
        };
        print_debug!("TData: Data read. Saving new timestap and return reference.");
        //let dt = Timestep::default();
        //self.single_data = dt;
        dt
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
                .expect(&format!(
                    "Can not find group \"timestep {}\" in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("position")
                .expect(&format!(
                    "Can not find dataset \"position\" in file {}",
                    self.file.filename()
                ))
                .read_2d::<f64>()
                .expect(&format!(
                    "Can not read dataset \"position\" in file {}",
                    self.file.filename()
                ))
                .to_owned();
            let position = position
                .axis_iter(Axis(0))
                .map(|x| [x[0], x[1], x[2]])
                .collect::<Array1<Position>>();
            // for each particle find the velocuty at the
            let velocity = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find group \"timestep {}\" in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("velocity")
                .expect(&format!(
                    "Can not find dataset \"velocity\" in file {}",
                    self.file.filename()
                ))
                .read_2d::<f64>()
                .expect(&format!(
                    "Can not read dataset \"velocity\" in file {}",
                    self.file.filename()
                ))
                .to_owned();

            //Radius
            let radius = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("radius")
                .expect(&format!(
                    "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"radius\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let density = match self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("density")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .expect(&format!(
                        "Can not read data from \"density\" dataset. \
                            Data type or data format might be wrong. \
                            Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    ))
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
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("ppcloud")
                .expect(&format!(
                    "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"ppcloud\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();

            let time = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not fine timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("time")
                .expect(&format!(
                    "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_scalar::<f64>()
                .expect(&format!(
                    "Can not read data from \"time\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let particleid = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("id")
                .expect(&format!(
                    "Can not find dataset \"id\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"id\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let particletype = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("particletype")
                .expect(&format!(
                    "Can not find dataset \"particletype\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"particletype\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();

            let dt = Timestep {
                time: time,
                position: position,
                velocity: velocity,
                radius: radius,
                particleid: particleid,
                clouds: clouds,
                density: density,
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
                .expect(&format!(
                    "Can not find group \"timestep {}\" in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("position")
                .expect(&format!(
                    "Can not find dataset \"position\" in file {}",
                    self.file.filename()
                ))
                .read_2d::<f64>()
                .expect(&format!(
                    "Can not read dataset \"position\" in file {}",
                    self.file.filename()
                ))
                .to_owned();
            let position = position
                .axis_iter(Axis(0))
                .map(|x| [x[0], x[1], x[2]])
                .collect::<Array1<Position>>();
            // for each particle find the velocuty at the
            let velocity = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find group \"timestep {}\" in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("velocity")
                .expect(&format!(
                    "Can not find dataset \"velocity\" in file {}",
                    self.file.filename()
                ))
                .read_2d::<f64>()
                .expect(&format!(
                    "Can not read dataset \"velocity\" in file {}",
                    self.file.filename()
                ))
                .to_owned();

            //Radius
            let radius = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("radius")
                .expect(&format!(
                    "Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"radius\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let density = match self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("density")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .expect(&format!(
                        "Can not read data from \"density\" dataset. \
                            Data type or data format might be wrong. \
                            Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()
                    ))
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
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("ppcloud")
                .expect(&format!(
                    "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"ppcloud\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();

            let time = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not fine timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("time")
                .expect(&format!(
                    "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_scalar::<f64>()
                .expect(&format!(
                    "Can not read data from \"time\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let particleid = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("id")
                .expect(&format!(
                    "Can not find dataset \"id\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"id\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let particletype = self
                .file
                .group(&format!("timestep {}", timestep))
                .expect(&format!(
                    "Can not find timestep {} in file {}",
                    timestep,
                    self.file.filename()
                ))
                .dataset("particletype")
                .expect(&format!(
                    "Can not find dataset \"particletype\" in HDF5 file \"{:?}\"",
                    self.file.filename()
                ))
                .read_1d::<f64>()
                .expect(&format!(
                    "Can not read data from \"particletype\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()
                ))
                .to_owned();
            let dt = Timestep {
                time: time,
                position: position,
                velocity: velocity,
                radius: radius,
                particleid: particleid,
                clouds: clouds,
                density: density,
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
                x = x + positions[0][0];
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
        } else if &self.range.1 == self.global_stats_.timesteps() && timestep < self.range.0 {
            self.update((0, BUFFERSIZE));
        }
        // If a timestep below the current range is requested, read in single timestep
        if timestep < self.range.0 {
            self.single_data = self.read_single(timestep);
            &self.single_data
        } else {
            // use mem::swap to swap it with a option(None)
            &self.buffer[timestep - self.range.0]
        }
    }

    fn global_stats(&self) -> GlobalStats {
        let dimensions = self
            .file
            .attr("dimensions")
            .expect(&format!(
                "Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
                self.file.filename()
            ))
            .read_2d::<f64>()
            .expect(&format!(
                "Can not read data from \"dimensions\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()
            ));
        let nparticles = self
            .file
            .attr("particle number")
            .expect(&format!(
                "Can not find attribute \"particle number\" in file {}",
                self.file.filename()
            ))
            .read_scalar()
            .expect(&format!(
                "Can not read scalar from attribute \"particle number\" in file {}",
                self.file.filename()
            ));
        let timesteps = self
            .file
            .attr("timesteps")
            .expect(&format!(
                "Can not find attribute \"timesteps\" in file {}",
                self.file.filename()
            ))
            .read_scalar()
            .expect(&format!(
                "Can not read scalar from attribute \"timesteps\" in file {}",
                self.file.filename()
            ));
        let time = self
            .file
            .attr("time")
            .expect(&format!(
                "Can not find attribute \"time\" in file {}",
                self.file.filename()
            ))
            .read_raw()
            .expect(&format!(
                "Can not read vector from attribute \"time\" in file {}",
                self.file.filename()
            ));
        let sample_rate = self
            .file
            .attr("sample rate")
            .expect(&format!(
                "Can not find attribute \"sample rate\" in file {}",
                self.file.filename()
            ))
            .read_scalar()
            .expect(&format!(
                "Can not read scalar from attribute \"sample rate\" in file {}",
                self.file.filename()
            ));
        let velocity = self
            .file
            .attr("velocity")
            .expect(&format!(
                "Can not find attribute \"velocity\" in file {}",
                self.file.filename()
            ))
            .read_2d()
            .expect(&format!(
                "Can not read scalar from attribute \"velocity\" in file {}",
                self.file.filename()
            ));
        let velocity_mag = self
            .file
            .attr("velocity magnitude")
            .expect(&format!(
                "Can not find attribute \"velocity magnitude\" in file {}",
                self.file.filename()
            ))
            .read_1d()
            .expect(&format!(
                "Can not read scalar from attribute \"velocity magnitude\" in file {}",
                self.file.filename()
            ));
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
            dimensions: dimensions,
            nparticles: nparticles,
            ntimesteps: timesteps,
            min_time: time[0],
            max_time: time[1],
            sample_rate: sample_rate,
            velocity: velocity,
            velocity_mag: velocity_mag,
            time_array: time_array,
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
    fn setup_buffer(&mut self) {
        self.buffer_extra
            .push(vec![Timestep::default(); BUFFERSIZE]);
        let buffer_id = self.buffer_extra.len() - 1;
        let range = (0, BUFFERSIZE);
        self.range_extra.push(range);
        self.buffersize_extra.push(BUFFERSIZE);
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
            self.update_extra((timestep, timestep + self.buffersize), buffer_id);
        } else if &self.range.1 == &self.global_stats_.ntimesteps && timestep < self.range.0 {
            self.update_extra((0, BUFFERSIZE), buffer_id);
        }
        // If a timestep below the current range is requested, update buffer
        if timestep < self.range.0 {
            self.update_extra((timestep, timestep + self.buffersize), buffer_id);
        }
        &self.buffer[timestep - self.range.0]
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
