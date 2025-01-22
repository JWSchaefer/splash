use lazy_nd::lazy_nd;

#[lazy_nd(dim = D)]
struct Test<const D: usize> {
    #[scalar(position : f64)]
    #[vector(velocity : f64)]
    #[vector(acceleration : f64)]
    #[scalar(mass: f64)]
    #[scalar(id: u32)]
    other: u32,
}

fn main() {}
