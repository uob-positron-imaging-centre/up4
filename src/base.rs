//!  Parent Module for smaller modules that provide more functionality
//!
//! Needed by `functions` module.

/// Module that implements nD grids and basic functionality on them.
pub mod grid;
pub use grid::*;

/// Module that implements the `ParticleSelector`, a struct deciding if a particle is valid or not 
pub mod particleselector;
pub use particleselector::*;
