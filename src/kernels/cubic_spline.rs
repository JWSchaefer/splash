use crate::traits::float::Float;
use crate::traits::kernel::Kernel;
use nalgebra::{Point, RealField, Scalar};

pub struct CubicSpline<T, const D: usize>
where
    T: Scalar + RealField + Float,
{
    pub h: T,
    scale_factor: T,
}

macro_rules! impl_cubic_spline {
    ($($t:ty),*) => {
        $(

            impl<const D: usize> CubicSpline<$t, D> {
                fn get_scale_factor(h: $t) -> Result<$t, String> {
                    let pi : $t = <$t>::PI;
                    match D {
                        1 => Ok(4.0 / (3.0 * h)),
                        2 => Ok(40.0 / (pi * 7.0 * h.powi(2))),
                        3 => Ok(8.0 / (pi * h.powi(3))),
                        _ => Err(format!(
                            "Scale factor for {} dimensions is undefined. Valid values of D are 1, 2, or 3.",
                            D
                        )),
                    }
                }
            }

            impl<const D: usize> Kernel<$t, D> for CubicSpline<$t, D> {
                fn new(h: $t) -> Self {

                    let scale_factor = match Self::get_scale_factor(h){
                        Ok(scale_factor) => scale_factor,
                        Err(msg) => panic!("{msg}"),
                    };

                    Self { h, scale_factor }
                }

                fn apply(&self, x1: Point<$t, D>, x2: Point<$t, D>) -> $t {
                    let q: $t = (x1 - x2).norm() / self.h;

                    return self.scale_factor *
                    {
                        if q <= 0.5 {
                            1. + 6.0 * (q.powi(3) - q.powi(2))
                        } else if q <= 1.0 {
                            2.0 * ((1.0 - q).powi(3))
                        } else {
                            0.0
                        }
                    }

                }


                fn apply_derivative(&self, x1: Point<$t, D>, x2: Point<$t, D>) -> $t {

                    let q: $t = (x1 - x2).norm() / self.h;

                    return self.scale_factor *
                    {
                        if q <= 0.5 {
                            6. * ((3. * q.powi(2)) - (2. * q))
                        } else if q <= 1.0 {
                            -6. * (1. - q).powi(2)
                        } else {
                            0.0
                        }
                    }

                }

            }

        )*
    }
}
impl_cubic_spline!(f32, f64);
