use ndarray::Array2;
use crate::errors::ArrayError;
use crate::traits::float::Float;
use crate::State;

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
    pub fn new(particles: usize) -> Self {
        Self {
            state: S::new(particles),
        }
    }
    pub fn from_array(data: Array2<<S as State>::T>) -> Result<Self, ArrayError> {
        Ok(Self {
            state: S::from_array(data)?,
        })
    }
    pub fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> S::T,
    {
        Self {
            state: S::from_shape_fn(particles, f),
        }
    }
}
