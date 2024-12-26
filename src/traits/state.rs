use std::error::Error;

use ndarray::Array2;
use crate::Float;

pub trait State: Sized {
    fn new(particles: usize) -> Self;
    fn from_array(data: Array2<Float>) -> Result<Self, Box<dyn Error>>;
    fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> Float;
    // fn position(&self) -> ArrayView2<Self::T>;
    // fn position_mut(&mut self) -> ArrayViewMut2<Self::T>
}
