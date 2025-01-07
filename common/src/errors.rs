use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum SplashError {
    #[error("Incorrect array shape: expected {:?}, got {:?}", .expected, .actual)]
    ShapeError {
        expected: Vec<usize>,
        actual: Vec<usize>,
    },

    #[error("Ivalid dimension {:?}", .dim)]
    DimensionError { dim: usize },

    #[error(
        "This error has not yet been implemented and is for development only"
    )]
    TestError,
}

impl Debug for SplashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
