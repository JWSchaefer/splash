use ndarray::ArrayView1;
use anyhow::Result;
use crate::Float;

pub trait Kernel: Sized {
    fn new(h: Float) -> Result<Self>;
    fn apply(
        &self,
        x1: ArrayView1<Float>,
        x2: ArrayView1<Float>,
    ) -> Result<Float>;
    fn apply_derivative(
        &self,
        x1: ArrayView1<Float>,
        x2: ArrayView1<Float>,
    ) -> Result<Float>;
}
