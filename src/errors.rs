use std::fmt;

#[derive(Debug, Clone)]
pub enum ArrayError {
    Dimension,
    Function,
}

impl fmt::Display for ArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to construct state from the provided array")
    }
}
