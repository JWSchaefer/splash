#[macro_use]
pub mod state_macros {
    #[macro_export]
    macro_rules! slice_state_2d {
        ($self:ident, $field:ident) => {{
            let (start, end) = $self.indices.$field;
            $self.data.slice(ndarray::s![start..end, ..])
        }};
    }
    #[macro_export]
    macro_rules! slice_state_2d_indexed {
        ($self:ident, $field:ident, $index:expr) => {{
            let (start, end) = $self.indices.$field;
            $self.data.slice(ndarray::s![start..end, $index])
        }};
    }
    #[macro_export]
    macro_rules! slice_state_1d {
        ($self:ident, $field:ident) => {{
            let index = $self.indices.$field;
            $self
                .data
                .slice(ndarray::s![index, ..])
                .into_dimensionality::<ndarray::Ix1>()
                .unwrap()
        }};
    }
    #[macro_export]
    macro_rules! slice_state_0d {
        ($self:ident, $field:ident, $particle:expr) => {{
            let index = $self.indices.$field;
            $self
                .data
                .slice(ndarray::s![index, $particle])
                .into_dimensionality::<ndarray::Ix0>()
                .unwrap()
        }};
    }
}
