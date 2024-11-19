use ndarray::{Array2, ArrayView0, ArrayView1, ArrayView2};
pub trait State: Sized {
    type T;
    fn new(dim: usize, particles: usize) -> Self;
    fn from_array(dim: usize, data: Array2<Self::T>) -> Result<Self, String>;

    fn positions(&self) -> ArrayView2<Self::T>;
    fn velocities(&self) -> ArrayView2<Self::T>;
    fn masses(&self) -> ArrayView1<Self::T>;
    fn densities(&self) -> ArrayView1<Self::T>;

    fn position(&self, particle: usize) -> ArrayView1<Self::T>;
    fn velocity(&self, particle: usize) -> ArrayView1<Self::T>;
    fn mass(&self, particle: usize) -> ArrayView0<Self::T>;
    fn density(&self, particle: usize) -> ArrayView0<Self::T>;
}
