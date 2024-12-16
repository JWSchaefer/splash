pub mod states {
    pub mod basic_state;
    pub mod state;
}

pub mod interpolate {
    pub mod density;
    pub mod force_viscous;
}

pub mod systems {
    pub mod particle;
}

pub mod traits {
    pub mod float;
    pub mod kernel;
    pub mod state;

    pub use float::Float;
}

pub mod kernel {
    pub mod cubic_spline;
}

pub mod errors;

pub use traits::state::State;
