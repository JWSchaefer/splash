use macros::State;

#[derive(State)]
struct MyState<const D: usize> {
    #[vector]
    position: f64,
    #[vector]
    velocity: f64,
    #[vector]
    acceleration: f64,
    #[scalar]
    mass: f64,
    #[scalar]
    id: u32,
}

fn main() {
    MyState::<2>::new();
}
