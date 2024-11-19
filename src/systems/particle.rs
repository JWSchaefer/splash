use ndarray::Array2;
use crate::traits::float::Float;
use crate::state::State;

#[repr(C)]
pub struct ParticleSystem<S>
where
    S: State,
{
    pub state: S,
}

impl<S> ParticleSystem<S>
where
    S: State,
    S::T: Float,
{
    pub fn new(dim: usize, particles: usize) -> Self {
        Self {
            state: S::new(dim, particles),
        }
    }
    pub fn from_array(dim: usize, data: Array2<<S as State>::T>) -> Self {
        Self {
            state: S::from_array(dim, data).unwrap(),
        }
    }
}
