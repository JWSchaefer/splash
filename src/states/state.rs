#[macro_export]
macro_rules! construct_state {
        ($state_name:ident, $($name:ident($value:expr)),*) => {



        #[derive(Debug)]
        struct Indices {
            $(
                $name: std::ops::Range<usize>,
            )*
            max : usize
        }

        use lazy_static;
        use std::sync::Mutex;

        lazy_static::lazy_static! {
            static ref CURRENT_INDEX: Mutex<usize> = Mutex::new(0);
        }

        impl Indices {
            fn new() -> Self {
                let mut index = CURRENT_INDEX.lock().unwrap();
                Indices {
                    $(
                        $name: {
                            if $value > 1 {
                                let start = *index;
                                let end = start + $value;
                                *index = end;
                                start..end
                            } else {
                                let idx = *index;
                                *index += 1;
                                idx..idx+1
                            }
                        },
                    )*
                max : *index - 1
                }
            }
        }


        #[derive(Debug)]
        pub struct $state_name {
            indicies : Indices,
            data : ndarray::Array2<crate::Float>,
        }

        paste::paste! {

        impl $state_name   {

            $(

                pub fn $name(&self) -> ndarray::ArrayView2<crate::Float> {
                    let range : std::ops::Range<usize> = self.indicies.$name.clone();
                    self.data.slice(ndarray::s![range, ..])
                }

                pub fn [<$name _mut>](&mut self) -> ndarray::ArrayViewMut2<crate::Float> {
                    let range : std::ops::Range<usize> = self.indicies.$name.clone();
                    self.data.slice_mut(ndarray::s![range, ..])
                }

            )*
            }
        }

        impl crate::State for $state_name

        {



            fn new(particles: usize) -> Self {
                let indicies = Indices::new();
                let data = ndarray::Array2::<crate::Float>::zeros((indicies.max, particles));
                Self { indicies, data }
            }

            fn from_array(data: ndarray::Array2<crate::Float>) -> Result<Self, Box<dyn std::error::Error>> {
                let indicies = Indices::new();
                let (dim, _) = &data.dim();

                if dim - 1 != indicies.max {
                    return Err(Box::new(crate::SphError::TestError))
                }

                println!("Dim:\t{dim:?}");
                Ok(Self { indicies , data })
            }

            fn from_shape_fn<F>(particles: usize, f: F) -> Self
            where
                F: FnMut((usize, usize)) -> crate::Float
            {
                let indicies = Indices::new();
                let data = ndarray::Array2::<crate::Float>::from_shape_fn((indicies.max+1, particles), f);

            Self { indicies, data }

            }
        }

    };
}
