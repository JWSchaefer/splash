use crate::Float;
#[cfg(feature = "f32")]
pub const PI: Float = std::f32::consts::PI;

#[cfg(feature = "f64")]
pub const PI: Float = std::f64::consts::PI;
