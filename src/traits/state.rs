use ndarray::Array2;

use crate::errors::ArrayError;

pub trait State: Sized {
    type T;
    fn new(particles: usize) -> Self;
    fn from_array(data: Array2<Self::T>) -> Result<Self, ArrayError>;
    fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> Self::T;
}
