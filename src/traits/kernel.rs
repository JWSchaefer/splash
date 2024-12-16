use ndarray::{Array2, ArrayView2};
use crate::traits::Float;

pub trait Kernel<T: Float> {
    fn new(h: T) -> Self;
    fn apply(&self, x1: ArrayView2<T>, x2: ArrayView2<T>) -> Array2<T>;
    fn apply_derivative(&self, x1: ArrayView2<T>, x2: ArrayView2<T>) -> Array2<T>;
}
