pub trait Indices: Sized {
    fn new() -> Self;
    fn max(&self) -> usize;
}
