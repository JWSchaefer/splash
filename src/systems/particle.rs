use ndarray::Array2;
use crate::Float;
use std::error::Error;
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
{
    pub fn new(particles: usize) -> Self {
        Self {
            state: S::new(particles),
        }
    }
    pub fn from_array(data: Array2<Float>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            state: S::from_array(data)?,
        })
    }
    pub fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> Float,
    {
        Self {
            state: S::from_shape_fn(particles, f),
        }
    }
}
