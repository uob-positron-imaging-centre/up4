extern crate ndarray;

use crate::{print_debug};
use std::any::Any;

pub trait Selector {
    fn is_valid(&self, radius: f64, clouds: f64, density: f64, particleid: usize)->bool;
    fn timestep_valid(&self, timestep:f64)-> bool;
    fn as_any(&self)-> &dyn Any;
}

pub struct ParticleSelector{
    time: (f64,f64),
    radius: Vec<f64>,
    clouds: Vec<f64>,
    density:Vec<f64>,
    particleid: Vec<usize>,

}
impl Default for ParticleSelector{
    fn default()->Self{
        ParticleSelector{
            time: (f64::MIN,f64::MAX),
            radius: vec![-1.],
            clouds: vec![-1.],
            density: vec![-1.],
            particleid: vec![0],
        }
    }
}
impl ParticleSelector{
    pub fn new(
        time: (f64,f64),
        radius: Vec<f64>,
        clouds: Vec<f64>,
        density:Vec<f64>,
        particleid: Vec<usize>,
    )-> Self{
        ParticleSelector{
            time,
            radius,
            clouds,
            density,
            particleid,
        }
    }

    fn is_radius_valid(&self,radius: f64)->bool{
        // if first element is -1 one does not want to check this
        if self.radius[0] == -1. {
            return true
        }
        if self.radius.len() == 1{
            return radius == self.radius[0]

        }else if self.radius.len() == 2 {
            return radius  >= self.radius[0] && radius <= self.radius[1]

        }else{
            return self.radius.contains(&radius)
        }

    }
    fn is_cloud_valid(&self,cloud: f64)->bool{
        // if first element is -1 one does not want to check this
        if self.clouds[0] == -1. {
            return true
        }
        if self.clouds.len() == 1{
            return cloud == self.clouds[0]

        }else if self.radius.len() == 2 {
            return cloud  >= self.clouds[0] && cloud <= self.clouds[1]

        }else{
            return self.clouds.contains(&cloud)
        }
    }
    fn is_density_valid(&self,density: f64)->bool{
        // if first element is -1 one does not want to check this
        if self.density[0] == -1. {
            return true
        }
        if self.density.len() == 1{
            return density == self.density[0]

        }else if self.radius.len() == 2 {
            return density  >= self.density[0] && density <= self.radius[1]

        }else{
            return self.density.contains(&density)
        }
    }
    fn is_particleid_valid(&self,particleid: usize)->bool{
        // if first element is 0 one does not want to check this
        // BUG Does not allow to choose particle id nr 0 as a particle.
        if self.particleid[0] == 0 {
            return true
        }
        if self.particleid.len() == 1{
            return particleid == self.particleid[0]

        }else if self.particleid.len() == 2 {
            return particleid  >= self.particleid[0] && particleid <= self.particleid[1]

        }else{
            return self.particleid.contains(&particleid)
        }
    }
}

impl Selector for ParticleSelector{
    fn is_valid(&self, radius: f64, clouds: f64, density: f64, particleid: usize)->bool{
        print_debug!("ParticleSelector: Checking if particle is valid.");
        if !self.is_radius_valid(radius){
            return false
        }else if !self.is_cloud_valid(clouds){
            return false
        }else if !self.is_density_valid(density){
            return false
        }else if !self.is_particleid_valid(particleid){
            return false
        };
        return true
    }

    fn timestep_valid(&self, time:f64)-> bool{
        (time >= self.time.0 && time <= self.time.1)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
