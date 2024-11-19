use num_traits::Zero;
pub trait Float: Copy + std::ops::Add<Output = Self> + Zero {
    const PI: Self;

    // Convert from f64
    fn from_f64(value: f64) -> Self;

    // Convert from f32
    fn from_f32(value: f32) -> Self;
}

// Implement Float for f32
impl Float for f32 {
    const PI: f32 = std::f32::consts::PI;

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn from_f32(value: f32) -> Self {
        value
    }
}

// Implement Float for f64
impl Float for f64 {
    const PI: f64 = std::f64::consts::PI;

    fn from_f64(value: f64) -> Self {
        value
    }

    fn from_f32(value: f32) -> Self {
        value as f64
    }
}
