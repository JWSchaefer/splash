use crate::{Float, SphError};
use anyhow::Result;
use ndarray::Array2;

pub enum QuantityType {
    Scalar,
    Vector,
}

pub trait Indices: Sized {
    fn new() -> Self;
    fn max(&self) -> usize;
}

pub trait State<I: Indices>: Sized {
    fn new(particles: usize) -> Self;

    fn data(&self) -> &Array2<Float>;
    fn data_mut(&mut self) -> &mut Array2<Float>;

    fn indices(&self) -> &I;

    fn from_array(data: Array2<Float>) -> Result<Self, SphError>;

    fn from_shape_fn<F>(particles: usize, f: F) -> Self
    where
        F: FnMut((usize, usize)) -> Float;
}

macro_rules! construct_state_trait {
    ($state_trait_name:ident, $($name:ident($quantity_type:expr)),*) => {

        use paste::paste;

        paste! {
            pub trait [<$state_trait_name Indices>]<const D : usize> : Indices {
                $(
                    fn $name(&self) -> &std::ops::Range<usize>;
                )*
            }


            pub trait $state_trait_name<I : [<$state_trait_name Indices>]<D>, const D : usize> : State<I> {
                $(
                    fn $name (
                        &self
                    ) -> ndarray::ArrayView2<Float> {
                        let indices : &I = self.indices();
                        let range : std::ops::Range<usize> = indices.$name().clone();
                        self.data().slice(ndarray::s![range, ..])
                    }

                    fn [<$name _mut>] (
                        &mut self
                    ) -> ndarray::ArrayViewMut2<Float> {
                        let indices : &I = self.indices();
                        let range : std::ops::Range<usize> = indices.$name().clone();
                        self.data_mut().slice_mut(ndarray::s![range, ..])
                    }
                )*
            }
        }
    };
}

construct_state_trait!(
    MinimalState,
    position(QuantityType::Vector),
    velocity(QuantityType::Vector),
    density(QuantityType::Scalar)
);
