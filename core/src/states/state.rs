use crate::traits::state::{Indices, MinimalState, QuantityType};

macro_rules! define_state {
    ($state_name : ident, $($trait_name : ident($($quantity_name:ident($quantity_type:expr)),*)),*) => {
        use paste::paste;

        use lazy_static;
        use std::sync::Mutex;

        lazy_static::lazy_static! {
            static ref CURRENT_INDEX: Mutex<usize> = Mutex::new(0);
        }

        paste!{
            #[derive(Debug)]
            struct [<$state_name Indices>]<const D : usize> {
                $(
                    $(
                        $quantity_name : std::ops::Range<usize>,
                    )*
                    max : usize
                )*
            }

            // TODO - Finish new() method
            impl<const D:usize> crate::traits::state::Indices for [<$state_name Indices>]<D> {
                fn new() -> Self {
                    Self {
                        $(
                            $(
                                $quantity_name : match $quantity_type {
                                    QuantityType::Vector => {0..1},
                                    QuantityType::Scalar => {1..2}
                                },
                            )*
                        )*
                        max : 4
                    }
                }

                fn max(&self) -> usize {
                    self.max
                }
            }


            $(
                impl<const D : usize> crate::traits::state::[<$trait_name Indices>]<D> for [<$state_name Indices>]<D> {
                    $(
                        fn $quantity_name(&self) -> &std::ops::Range<usize>{
                            &self.$quantity_name
                        }
                    )*
                }
            )*


            #[derive(Debug)]
            struct $state_name<const D: usize> {
                data: ndarray::Array2<crate::Float>,
                indices: [<$state_name Indices>]<D>,
            }



            impl<const D :usize> crate::traits::state::State for $state_name<D> {

                type I = [<$state_name Indices>]<D>;

                fn new(particles: usize) -> Self {
                    let indices = [<$state_name Indices>]::<D>::new();
                    let data = ndarray::Array2::<crate::Float>::zeros((indices.max, particles));
                    Self { indices, data }
                }

                fn data(&self) -> &ndarray::Array2<crate::Float> {
                    &self.data
                }

                fn data_mut(&mut self) -> &mut ndarray::Array2<crate::Float> {
                    &mut self.data
                }

                fn from_array(data: ndarray::Array2<crate::Float>) -> Result<Self, crate::SphError> {
                    let indices = [<$state_name Indices>]::<D>::new();
                    let (dim, _) = &data.dim();

                    if dim - 1 != indices.max {
                        return Err(crate::SphError::TestError)
                    }

                    println!("Dim:\t{dim:?}");
                    Ok(Self { indices , data })
                }

                fn from_shape_fn<F>(particles: usize, f: F) -> Self
                where
                    F: FnMut((usize, usize)) -> crate::Float
                {
                    let indices = [<$state_name Indices>]::<D>::new();
                    let data = ndarray::Array2::<crate::Float>::from_shape_fn((indices.max+1, particles), f);

                    Self { indices, data }

                }

                fn indices(&self) -> &Self::I {
                    &self.indices
                }

            }

            $(
                impl<const D : usize> $trait_name<D> for $state_name{}
            )*



        }
    };
}

define_state! {
    MyState,
    MinimalState(
        position(QuantityType::Vector),
        velocity(QuantityType::Vector),
        density(QuantityType::Scalar)
    )
}
