use macros::State;

#[derive(State)]
struct Test<const D: usize> {
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
    live: bool,
}

fn main() {
    Test::<2>::new();
}
