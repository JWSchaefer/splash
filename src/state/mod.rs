mod basic {
    pub mod state;
    pub mod traits;
}

pub mod macros;

pub use macros::state_macros;
pub use basic::{traits::State, state::BasicState};
