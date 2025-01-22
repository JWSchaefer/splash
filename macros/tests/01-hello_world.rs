use macros::State;

#[derive(State)]
struct Test {
    #[scalar(position : f64)]
    #[vector(velocity : f64)]
    #[vector(acceleration : f64)]
    #[scalar(mass: f64)]
    #[scalar(id : u32)]
    other: u32,
}

fn main() {
    Test::new();
}
