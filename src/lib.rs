#![allow(dead_code)]

pub mod interpolate {
    pub mod density;
    pub mod force_viscous;
}

pub mod kernels {
    pub mod cubic_spline;
}

pub mod traits {
    pub mod state {
        pub mod density;
    }
    pub mod float;
    /*  pub mod interpolator; */
    pub mod kernel;
}

pub mod particle;
