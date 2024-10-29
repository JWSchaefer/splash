use nalgebra::{Point, Scalar};

pub trait Kernel<T: Scalar, const D: usize> {
    fn new(h: T) -> Self;
    fn apply(&self, x1: Point<T, D>, x2: Point<T, D>) -> T;
    fn apply_derivative(&self, x1: Point<T, D>, x2: Point<T, D>) -> T;
}
