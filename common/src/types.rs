#[cfg(feature = "32")]
pub type Float = f32;
#[cfg(feature = "32")]
pub type Int = i32;
#[cfg(feature = "32")]
pub type UInt = u32;

#[cfg(feature = "64")]
pub type Float = f64;
#[cfg(feature = "64")]
pub type Int = i64;
#[cfg(feature = "64")]
pub type UInt = u64;
