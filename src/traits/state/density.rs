use nalgebra::{RealField, Scalar};

use crate::traits::float::Float;

pub trait DensityState<T>
where
    T: RealField + Scalar + Float,
{
    fn density(&self) -> &T;
    fn density_mut(&mut self) -> &mut T;
}
