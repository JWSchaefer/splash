use crate::constants::PI;
use crate::traits::kernel::Kernel;
use crate::traits::DimensionValidator;
use crate::Float;
use crate::{errors::SphError::DimensionError, SphError};
use anyhow::{Context, Result};
use ndarray::ArrayView1;
use ndarray_linalg::{Norm, Scalar};

pub struct CubicSpline<const D: usize> {
    h: Float,
    norm: Float,
}

impl<const D: usize> CubicSpline<D> {
    fn get_norm(h: Float) -> Result<Float, SphError> {
        let result = match D {
            1 => 4.0 / (3.0 * h),
            2 => 40.0 / (7.0 * PI * h.pow(2.0)),
            3 => 8.0 / (PI * h.pow(3.0)),
            _ => return Err(DimensionError { dim: D }),
        };

        Ok(result)
    }
}

impl<const D: usize> Kernel for CubicSpline<D> {
    fn new(h: Float) -> Result<Self> {
        let norm = Self::get_norm(h)
            .context("Cubic Spline kernel is only defined for D <= 3")?;
        Ok(Self { h, norm })
    }
    fn apply(
        &self,
        x1: ArrayView1<Float>,
        x2: ArrayView1<Float>,
    ) -> Result<Float> {
        x1.validate(&[D])
            .context("Dimension of x1 does not match the kernel dimension")?;
        x2.validate(&[D])
            .context("Dimension of x2 does not match the kernel dimension")?;

        let q = (&x1 - &x2).norm() / self.h;

        let w = self.norm
            * match q {
                q if q >= 0.0 && q <= 0.5 => {
                    6.0 * (q.powi(3) - q.powi(2)) + 1.0
                }
                q if q > 0.5 && q <= 1.0 => 2.0 * (1.0 - q).powi(3),
                _ => 0.0,
            };

        println!(
            "|x1- x2| = {:2.4} -> q = {q:2.4} -> w = {w:2.4} ",
            (&x1 - &x2).norm()
        );

        Ok(w)
    }
    fn apply_derivative(
        &self,
        x1: ArrayView1<Float>,
        x2: ArrayView1<Float>,
    ) -> Result<Float> {
        x1.validate(&[D])
            .context("Dimension of x1 does not match the kernel dimension")?;
        x2.validate(&[D])
            .context("Dimension of x2 does not match the kernel dimension")?;

        let q = (&x1 - &x2).norm() / self.h;

        let w = self.norm
            * self.h
            * match q {
                q if q >= 0.0 && q <= 0.5 => {
                    6.0 * ((3.0 * q.powi(2)) - (2.0 * q))
                }
                q if q > 0.5 && q <= 1.0 => -6.0 * (1.0 - q).powi(2),
                _ => 0.0,
            };

        Ok(w)
    }
}
