use super::traits::State;

use ndarray::{Array2, ArrayView2, ArrayView1, ArrayView0};
use crate::traits::Float;
use crate::{slice_state_2d, slice_state_2d_indexed, slice_state_1d, slice_state_0d};

struct Indices {
    pub position: (usize, usize),
    pub velocity: (usize, usize),
    pub mass: usize,
    pub density: usize,
    pub end: usize,
}

impl Indices {
    fn new(dim: usize) -> Self {
        Self {
            position: (0, dim),
            velocity: (dim, 2 * dim),
            mass: 2 * dim,
            density: 2 * dim + 1,
            end: 2 * dim + 2,
        }
    }
}

pub struct BasicState<T> {
    data: Array2<T>,
    indices: Indices,
}

impl<T> State for BasicState<T>
where
    T: Float,
{
    type T = T;

    fn new(dim: usize, particles: usize) -> Self {
        let indices = Indices::new(dim);
        let data = Array2::<Self::T>::zeros((indices.end, particles));
        Self { data, indices }
    }

    fn from_array(dim: usize, data: Array2<Self::T>) -> Result<Self, String> {
        let indices = Indices::new(dim);
        let (d, _) = data.dim();

        if d != indices.end {
            return Err(
                format!("The array provided does not have the correct shape to be used in a BasicState of dimension {dim}")
                    .to_string(),
            );
        }

        Ok(Self { data, indices })
    }

    fn positions(&self) -> ArrayView2<Self::T> {
        slice_state_2d!(self, position)
    }

    fn velocities(&self) -> ArrayView2<Self::T> {
        slice_state_2d!(self, velocity)
    }

    fn masses(&self) -> ArrayView1<Self::T> {
        slice_state_1d!(self, mass)
    }

    fn densities(&self) -> ArrayView1<Self::T> {
        slice_state_1d!(self, density)
    }

    fn position(&self, particle: usize) -> ArrayView1<Self::T> {
        slice_state_2d_indexed!(self, position, particle)
    }

    fn velocity(&self, particle: usize) -> ArrayView1<Self::T> {
        slice_state_2d_indexed!(self, velocity, particle)
    }

    fn mass(&self, particle: usize) -> ArrayView0<Self::T> {
        slice_state_0d!(self, mass, particle)
    }

    fn density(&self, particle: usize) -> ArrayView0<Self::T> {
        slice_state_0d!(self, density, particle)
    }
}
