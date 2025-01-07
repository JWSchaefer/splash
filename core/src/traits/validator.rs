use crate::SphError::{self, ShapeError};
use ndarray::{ArrayView, Dimension};

use crate::Float;

pub trait DimensionValidator {
    fn validate(&self, expected_dim: &[usize]) -> Result<(), SphError>;
}

impl<'a, D: Dimension> DimensionValidator for ArrayView<'a, Float, D> {
    fn validate(&self, expected_dim: &[usize]) -> Result<(), SphError> {
        match self.shape() == expected_dim {
            true => Ok(()),
            false => Err(ShapeError {
                expected: expected_dim.to_vec(),
                actual: self.shape().to_vec(),
            }),
        }
    }
}
