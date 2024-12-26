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
    pub mod kernel;
    pub mod state;
    pub mod validator;
    pub use validator::DimensionValidator;
}

pub mod kernel {
    pub mod cubic_spline;
}

pub mod constants;
pub mod errors;
pub mod types;

pub use errors::SphError;
/* pub use ndarray_linalg::Norm; */
pub use traits::state::State;
pub use types::Float;
