use ndarray_linalg::Scalar;

pub trait Float:
    Copy
    + Clone
    + num_traits::Float
    + std::ops::Mul
    + Clone
    + std::convert::Into<f64>
    + Scalar
{
    const PI: Self;
}

// Implement Float for f32
impl Float for f32 {
    const PI: f32 = std::f32::consts::PI;
}

// Implement Float for f64
impl Float for f64 {
    const PI: f64 = std::f64::consts::PI;
}
