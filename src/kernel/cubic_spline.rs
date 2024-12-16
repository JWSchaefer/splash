use ndarray::{Array2, ArrayView2};
use crate::traits::Float;

struct CubicSpline<T>
where
    T: Float,
{
    h: T,
}

impl<T> Kernel<T> for CubicSpline<T>
where
    T: Float,
{
    fn new(h: T) -> Self;
    fn apply(&self, x1: ArrayView2<T>, x2: ArrayView2<T>) -> Array2<T>;
    fn apply_derivative(&self, x1: ArrayView2<T>, x2: ArrayView2<T>) -> Array2<T>;
}
