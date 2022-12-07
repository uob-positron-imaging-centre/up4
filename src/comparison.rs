use crate::datamanager::{DataManager,Manager};
use crate::functions::Granular;
use crate::grid::GridFunctions3D;
use crate::particleselector::ParticleSelector;

use crate::types::*;

struct comparer {
    data: Box<dyn Manager + Send>,
    data2: Box<dyn Manager + Send>,
}
pub trait Comparison: Manager + Send {
    fn compare(&self, other: &Self)-> comparer{
        let stats = self.global_stats();
        let dim = stats.dimensions();

        let selector = ParticleSelector::default();
        comparer{
            data: Box::new(self),
            data2: Box::new(other),
        }
    };

    fn align(&self, other: &Self);
}
