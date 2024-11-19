#![allow(dead_code)]

pub mod interpolate {
    pub mod density;
    pub mod force_viscous;
}

pub mod systems {
    pub mod particle;
}

pub mod traits;

pub mod kernel;

pub mod state;
