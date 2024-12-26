use crate::{Float, SphError};
use anyhow::Result;
use ndarray::Array2;

pub trait State: Sized {
    fn new(particles: usize) -> Self;
    fn from_array(data: Array2<Float>) -> Result<Self, SphError>;
    fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> Float;
}
