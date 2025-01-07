use crate::traits::Indices;
use crate::types::UInt;
use crate::{errors::SplashError, types::Float};
use anyhow::Result;
use ndarray::Array2;

pub trait State<I: Indices, const D: usize>: Sized {
    fn new(particles: usize) -> Self;

    fn data(&self) -> &Array2<Float>;
    fn data_mut(&mut self) -> &mut Array2<Float>;

    fn indices(&self) -> &I;

    fn from_array(data: Array2<Float>) -> Result<Self, SplashError>;

    fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> Float;
}

macro_rules! construct_state_trait {
    ($state_trait_name:ident, $($name:ident($quantity_type:ident, $data_type:ty)),*) => {

        use paste::paste;

        paste! {
            pub trait [<$state_trait_name Indices>]<const D : usize> : Indices {
                $(
                    fn $name(&self) -> &std::ops::Range<usize>;
                )*
            }


            pub trait $state_trait_name<I : [<$state_trait_name Indices>]<D>, const D : usize> : State<I ,D> {
                $(
                    fn $name (&self) -> ndarray::ArrayView2<$data_type>;
                    fn [<$name _mut>] (&mut self) -> ndarray::ArrayViewMut2<$data_type>;
                )*
            }
        }
    };
}

construct_state_trait!(
    MinimalState,
    position(Vector, Float),
    velocity(Vector, Float),
    acceleration(Vector, Float),
    mass(Scalar, Float),
    id(Scalar, UInt)
);
