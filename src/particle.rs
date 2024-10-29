use crate::traits::float::Float;
use nalgebra::{Point, RealField, SVector, Scalar};

#[repr(C)]
pub struct Particle<T, const D: usize>
where
    T: Scalar + RealField + Float,
{
    id: usize,
    pub position: Point<T, D>,
    pub velocity: SVector<T, D>,
    pub mass: T,
}

impl<T, const D: usize> Particle<T, D>
where
    T: Scalar + RealField + Float,
{
    pub fn new(id: usize, position: Point<T, D>, velocity: SVector<T, D>, mass: T) -> Self {
        Self {
            id,
            position,
            velocity,
            mass,
        }
    }
}

impl<T, const D: usize> PartialEq for Particle<T, D>
where
    T: Scalar + RealField + Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
