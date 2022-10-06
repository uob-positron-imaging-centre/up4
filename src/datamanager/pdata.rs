//! Particle based HDF5 dataset manager
//!
//! Implementation of reading + buffering functions.

use super::{DataManager, GlobalStats, Manager, Timestep};
extern crate ndarray;
use crate::particleselector::Selector;
use crate::types::*;
use crate::ParticleSelector;
use crate::{print_debug, print_warning};
use ndarray::prelude::*;
use std::time::Instant;
const BUFFERSIZE: usize = 2000000;

//#[pyclass]
pub struct PData {
    // A data managing system for HDF5 files in the
    // H5Part format or H5TimePart information
    file: hdf5::File,
    buffer: Vec<Timestep>, // Replace with Option(Timestep, None)
    range: (usize, usize),
    pub buffersize: usize,
    single_data: Timestep,
    global_stats_: GlobalStats,
}

impl PData {
    ///new conste
    pub fn new(filename: &str) -> Self {
        print_debug!(
            "PData: Generating new instance with file: {}\
            and buffersize: {}",
            &filename,
            BUFFERSIZE
        );
        let file =
            hdf5::File::open(filename).unwrap_or_else(|_| panic!("Can not read HDF5 file {}. ", &filename));
        let buffer = vec![Timestep::default(); BUFFERSIZE];
        let mut data = PData {
            file,
            buffer,
            range: (0, 0),
            buffersize: BUFFERSIZE,
            single_data: Timestep::default(),
            global_stats_: GlobalStats::default(),
        };
        print_debug!(
            "PData: Generation complete. First buffer update starting, buffer size: {}",
            data.buffer.len()
        );

        let global_stats = data.global_stats();
        data.global_stats_ = global_stats;
        data.update((0, data.buffersize));

        print_debug!("PData: Buffer updated. Returning initiated PData.");
        data
    }
    #[inline(never)]
    fn read_single(&mut self, timestep: usize) -> &Timestep {
        print_debug!("PData: Reading a single timestep: {}", timestep);

        let particles = *self.global_stats_.nparticles();
        let mut position: Array1<Position> = Array1::from_elem(particles, [0.0, 0.0, 0.0]);
        let mut velocity: Array2<f64> = Array2::<f64>::zeros((particles, 3));
        let mut radius: Array1<f64> = Array1::<f64>::zeros(particles);
        let mut density: Array1<f64> = Array1::<f64>::zeros(particles);
        let mut particleid: Array1<f64> = Array1::<f64>::zeros(particles);
        let mut time: Array1<f64> = Array1::<f64>::zeros(particles);
        let clouds: Array1<f64> = Array1::<f64>::ones(particles);
        print_debug!("PData: Looping over all particles.");
        for particle_id in 0..particles {
            print_debug!("\tParticle {}: Extracting position from HDF5.", particle_id);
            let part_pos = self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find group \"particle {}\" in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("position")
                .unwrap_or_else(|_| panic!("Can not find dataset \"position\" in file {}",
                    self.file.filename()))
                .read_2d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read dataset \"position\" in file {}",
                    self.file.filename()))
                .slice(s![timestep, ..])
                .to_owned()
                .into_shape((particles, 3))
                .unwrap_or_else(|_| panic!("Can not shape dataset \"position\" in file {} into a ({}, 3) array",
                    self.file.filename(),
                    particles))
                .axis_iter(Axis(0))
                .map(|x| [x[0], x[1], x[2]])
                .collect::<Array1<Position>>();
            // iter over elements in 2D array. make  it a 1D array of static arrays with 3 elements.

            print_debug!("\tParticle {}: Extracting velocity from HDF5.", particle_id);
            let part_vel = self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find group \"particle {}\" in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("velocity")
                .unwrap_or_else(|_| panic!("Can not find dataset \"velocity\" in file {}",
                    self.file.filename()))
                .read_2d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read dataset \"velocity\" in file {}",
                    self.file.filename()))
                .slice(s![timestep, ..])
                .to_owned();

            //Radius
            print_debug!("\tParticle {}: Extracting radius from HDF5.", particle_id);
            let part_rad = self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not fine particle {} in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("radius")
                .unwrap_or_else(|_| panic!("Can not find dataset \"radius\" in HDF5 file \"{:?}\"",
                    self.file.filename()))
                .read_1d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read data from \"radius\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()))
                .slice(s![timestep])
                .to_owned();
            print_debug!("\tParticle {}: Extracting density from HDF5.", particle_id);
            let part_density = match self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find particle {} in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("density")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .unwrap_or_else(|_| panic!("Can not read data from \"density\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()))
                    .slice(s![timestep])
                    .to_owned(),
                Err(_s) => {
                    print_warning!(
                        "\tParticle {}: Could not find density in HDF5 file {}. \
                        Using one instead.",
                        particle_id,
                        self.file.filename()
                    );
                    let mut arr = part_rad.clone();
                    arr.fill(1.0);
                    arr
                }
            };
            print_debug!("\tParticle {}: Extracting time from HDF5.", particle_id);
            let part_time = self
                .file
                .group(&format!("particle {}", 0))
                .unwrap_or_else(|_| panic!("Can not fine particle {} in file {}",
                    0,
                    self.file.filename()))
                .dataset("time")
                .unwrap_or_else(|_| panic!("Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                    self.file.filename()))
                .read_1d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read data from \"time\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()))
                .slice(s![timestep])
                .to_owned();
            let mut part_id: Array1<f64> = Array1::<f64>::zeros(1);
            part_id.fill(particle_id as f64);

            position.slice_mut(s![particle_id]).assign(&part_pos);
            velocity.slice_mut(s![particle_id, ..]).assign(&part_vel);
            radius.slice_mut(s![particle_id]).assign(&part_rad);
            density.slice_mut(s![particle_id]).assign(&part_density);
            time.slice_mut(s![particle_id]).assign(&part_time);
            particleid.slice_mut(s![particle_id]).assign(&part_id);
        }

        let dt = Timestep {
            time: time[0],
            position,
            velocity,
            radius,
            particleid,
            clouds,
            density,
        };
        print_debug!("PData: Data read. Saving new timestap and return reference.");
        //let dt = Timestep::default();
        self.single_data = dt;
        &self.single_data
    }

    pub fn reset_buffer() {
        //TODO
    }
    pub fn test(&mut self) {
        let now = Instant::now();
        let _time = self.global_stats_.max_time();
        let mut v = 0.;
        let mut n = 0;
        let selector = ParticleSelector::new(
            (f64::MIN, f64::MAX),
            vec![f64::MIN, f64::MAX],
            vec![f64::MIN, f64::MAX],
            vec![f64::MIN, f64::MAX],
            vec![usize::MIN, usize::MAX],
        );
        for timestep in 0..*self.global_stats_.timesteps() {
            let timestep_data = self.get_timestep(timestep);
            let _positions: &Array1<Position> = timestep_data.position();
            let velocity: &Array2<f64> = timestep_data.velocity();
            let radius: &Array1<f64> = timestep_data.radius();
            let particleid: &Array1<f64> = timestep_data.particleid();
            let clouds: &Array1<f64> = timestep_data.clouds();
            let density: Array1<f64> = Array1::from_elem(clouds.len(), 1000.0);
            for (id, vel) in velocity.outer_iter().enumerate() {
                if !selector.is_valid(radius[id], clouds[id], density[id], particleid[id] as usize)
                {
                    continue;
                }
                if vel[0_usize].is_nan() {
                    continue;
                }
                v += vel[0_usize].abs();
                n += 1;
            }
        }
        println!("Elapsed time: {}", now.elapsed().as_millis());
        println!("vel: {}", v / n as f64);
        println!(
            "Vel from global STats: {}",
            self.global_stats_.velocity()[[0, 1]]
        )
    }

    #[inline(never)]
    pub fn update(&mut self, mut range: (usize, usize)) {
        print_debug!("PData: Updating buffer");
        let timesteps = *self.global_stats_.timesteps();
        if range.1 >= timesteps {
            range.1 = timesteps;
        }
        self.buffersize = range.1 - range.0;
        let particles = *self.global_stats_.nparticles();
        let mut position: Array2<Position> =
            Array2::<Position>::from_elem((self.buffersize, particles), [0.0, 0.0, 0.0]);
        let mut velocity: Array3<f64> = Array3::<f64>::zeros((self.buffersize, particles, 3));
        let mut radius: Array2<f64> = Array2::<f64>::zeros((self.buffersize, particles));
        let mut density: Array2<f64> = Array2::<f64>::zeros((self.buffersize, particles));
        let mut particleid: Array2<f64> = Array2::<f64>::zeros((self.buffersize, particles));
        let mut time: Array2<f64> = Array2::<f64>::zeros((self.buffersize, particles));
        let clouds: Array2<f64> = Array2::<f64>::ones((self.buffersize, particles));
        print_debug!("PData: Looping over all particles:");
        for particle_id in 0..particles {
            // for each particle find the Position at the
            print_debug!("\tParticle {}: Extracting position from HDF5.", particle_id);
            let part_pos = self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find group \"particle {}\" in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("position")
                .unwrap_or_else(|_| panic!("Can not find dataset \"position\" in file {}",
                    self.file.filename()))
                .read_2d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read dataset \"position\" in file {}",
                    self.file.filename()))
                .slice(s![range.0..range.1, ..])
                .to_owned()
                .into_shape((range.1-range.0, 3))
                .unwrap_or_else(|_| panic!("Can not shape dataset \"position\" in file {} into a ({}, 3) array, range: {:?}",
                    self.file.filename(),
                    range.1-range.0,
                    range))
                .axis_iter(Axis(0))
                .map(|x| [x[0], x[1], x[2]])
                .collect::<Array1<Position>>();

            // for each particle find the velocuty at the
            print_debug!("\tParticle {}: Extracting velocity from HDF5.", particle_id);
            let part_vel = self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find group \"particle {}\" in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("velocity")
                .unwrap_or_else(|_| panic!("Can not find dataset \"velocity\" in file {}",
                    self.file.filename()))
                .read_2d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read dataset \"velocity\" in file {}",
                    self.file.filename()))
                .slice(s![range.0..range.1, ..])
                .to_owned();

            //Radius
            print_debug!("\tParticle {}: Extracting radius from HDF5.", particle_id);
            let part_rad = match self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find particle {} in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("radius")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .unwrap_or_else(|_| panic!("Can not read data from \"radius\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()))
                    .slice(s![range.0..range.1])
                    .to_owned(),
                Err(_s) => {
                    print_warning!(
                        "\tParticle {}: Could not find radius in HDF5 file {}. \
                        Using zeros instead.",
                        particle_id,
                        self.file.filename()
                    );
                    Array1::<f64>::zeros(part_vel.len() / 3)
                }
            };
            print_debug!("\tParticle {}: Extracting density from HDF5.", particle_id);
            let part_density = match self
                .file
                .group(&format!("particle {}", particle_id))
                .unwrap_or_else(|_| panic!("Can not find particle {} in file {}",
                    particle_id,
                    self.file.filename()))
                .dataset("density")
            {
                Ok(s) => s
                    .read_1d::<f64>()
                    .unwrap_or_else(|_| panic!("Can not read data from \"density\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                        self.file.filename()))
                    .slice(s![range.0..range.1])
                    .to_owned(),
                Err(_s) => {
                    print_warning!(
                        "\tParticle {}: Could not find density in HDF5 file {}. \
                        Using one instead.",
                        particle_id,
                        self.file.filename()
                    );
                    Array1::<f64>::zeros(part_vel.len() / 3)
                }
            };

            print_debug!("\tParticle {}: Extracting time from HDF5.", particle_id);
            let part_time = self
                .file
                .group(&format!("particle {}", 0))
                .unwrap_or_else(|_| panic!("Can not fine particle {} in file {}",
                    0,
                    self.file.filename()))
                .dataset("time")
                .unwrap_or_else(|_| panic!("Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                    self.file.filename()))
                .read_1d::<f64>()
                .unwrap_or_else(|_| panic!("Can not read data from \"time\" dataset. \
                    Data type or data format might be wrong. \
                    Check creation of HDF5 file  \"{:?}\"",
                    self.file.filename()))
                .slice(s![range.0..range.1])
                .to_owned();

            print_debug!("\tParticle {}: generating particle ID array.", particle_id);
            let mut part_id: Array1<f64> = Array1::<f64>::zeros(self.buffersize);
            part_id.fill(particle_id as f64);

            print_debug!(
                "\tParticle {}: Inserting all information in Arrays.",
                particle_id
            );
            position.slice_mut(s![.., particle_id]).assign(&part_pos);
            velocity
                .slice_mut(s![.., particle_id, ..])
                .assign(&part_vel);
            radius.slice_mut(s![.., particle_id]).assign(&part_rad);
            density.slice_mut(s![.., particle_id]).assign(&part_density);
            time.slice_mut(s![.., particle_id]).assign(&part_time);
            particleid.slice_mut(s![.., particle_id]).assign(&part_id);
            print_debug!("\tParticle {}: Finish.", particle_id);
        }
        //let mut  clouds = particleid.clone();
        //clouds.fill(1.);

        print_debug!("PData: Generating Timestep structs.");
        for timestep in 0..range.1 - range.0 {
            let dt = Timestep {
                time: time[[timestep, 0]], //.slice(s![timestep,..]),
                position: position.slice(s![timestep, ..]).to_owned(),
                velocity: velocity.slice(s![timestep, .., ..]).to_owned(),
                radius: radius.slice(s![timestep, ..]).to_owned(),
                particleid: particleid.slice(s![timestep, ..]).to_owned(),
                clouds: clouds.slice(s![timestep, ..]).to_owned(),
                density: density.slice(s![timestep, ..]).to_owned(),
            };

            self.buffer[timestep] = dt;
        }
        self.range = range; //(range.0,range.1-1);
        print_debug!("PData: Finished buffer update")
    }
} //end impl

impl DataManager for PData {
    // return a array from a given dataset at a given timestep
    fn get_timestep(&mut self, timestep: usize) -> &Timestep {
        print_debug!("PData: Extracting timestep number {}", timestep);
        // If a timestep is requested that is higher then the current range
        // update buffer with new range
        print_debug!(
            "PData: Checking requested timestep {} in range {:?}",
            timestep,
            self.range
        );
        if timestep > self.range.1 - 1 {
            self.update((timestep, timestep + self.buffersize));
        } else if &self.range.1 == &self.global_stats_.ntimesteps && timestep < self.range.0 {
            self.update((0, BUFFERSIZE));
        }
        // If a timestep below the current range is requested, read in single timestep
        if timestep < self.range.0 {
            self.read_single(timestep)
        } else {
            // use mem::swap to swap it with a option(None)
            &self.buffer[timestep - self.range.0]
        }
    }

    fn global_stats(&self) -> GlobalStats {
        let dimensions = self
            .file
            .attr("dimensions")
            .unwrap_or_else(|_| panic!("Can not find dataset \"dimensions\" in HDF5 file \"{:?}\"",
                self.file.filename()))
            .read_2d::<f64>()
            .unwrap_or_else(|_| panic!("Can not read data from \"dimensions\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                self.file.filename()));
        let nparticles = self
            .file
            .attr("particle number")
            .unwrap_or_else(|_| panic!("Can not find attribute \"particle number\" in file {}",
                self.file.filename()))
            .read_scalar()
            .unwrap_or_else(|_| panic!("Can not read scalar from attribute \"particle number\" in file {}",
                self.file.filename()));
        let timesteps = self
            .file
            .attr("timesteps")
            .unwrap_or_else(|_| panic!("Can not find attribute \"timesteps\" in file {}",
                self.file.filename()))
            .read_scalar()
            .unwrap_or_else(|_| panic!("Can not read scalar from attribute \"timesteps\" in file {}",
                self.file.filename()));
        let time = self
            .file
            .attr("time")
            .unwrap_or_else(|_| panic!("Can not find attribute \"time\" in file {}",
                self.file.filename()))
            .read_raw()
            .unwrap_or_else(|_| panic!("Can not read vector from attribute \"time\" in file {}",
                self.file.filename()));
        let sample_rate = self
            .file
            .attr("sample rate")
            .unwrap_or_else(|_| panic!("Can not find attribute \"sample rate\" in file {}",
                self.file.filename()))
            .read_scalar()
            .unwrap_or_else(|_| panic!("Can not read scalar from attribute \"sample rate\" in file {}",
                self.file.filename()));
        let velocity = self
            .file
            .attr("velocity")
            .unwrap_or_else(|_| panic!("Can not find attribute \"velocity\" in file {}",
                self.file.filename()))
            .read_2d()
            .unwrap_or_else(|_| panic!("Can not read scalar from attribute \"velocity\" in file {}",
                self.file.filename()));
        let velocity_mag = self
            .file
            .attr("velocity magnitude")
            .unwrap_or_else(|_| panic!("Can not find attribute \"velocity magnitude\" in file {}",
                self.file.filename()))
            .read_1d()
            .unwrap_or_else(|_| panic!("Can not read scalar from attribute \"velocity magnitude\" in file {}",
                self.file.filename()));
        GlobalStats {
            dimensions,
            nparticles,
            ntimesteps: timesteps,
            min_time: time[0],
            max_time: time[1],
            sample_rate,
            velocity,
            velocity_mag,
        }
    }
    fn stats(&self) {
        let time = self.global_stats_.max_time();
        let dim = self.global_stats_.dimensions();
        println!("Dimensions of the system:: {:?}", dim);
        println!("The max time of this set is : {:?}", time);
        let part_num = self.global_stats_.nparticles();
        println!("Number of Particles: {}", part_num);
        let vel_mag = self.global_stats_.velocity_mag();
        println!("Mean velocity of: {} m/s", vel_mag[1usize]);
        println!(
            "Minimum velocity {} m/s \nMaximum Velocity {} m/s",
            vel_mag[0usize], vel_mag[2usize]
        );
    }
}

// Implement Manager which is just a sum of Granular + DataManager
impl Manager for PData {}

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

impl Drop for PData {
    fn drop(&mut self) {
        print_debug!("Killing PData.\nGoodbye :-)");
        //self.file.close();
    }
}
