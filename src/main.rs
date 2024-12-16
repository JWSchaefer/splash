use sph::systems::particle::ParticleSystem;
use sph::states::basic_state::BasicState3;

// use nalgebra::{Point2, Vector2};
// use sph::interpolate::density::{self, density};
// use sph::kernels::cubic_spline::CubicSpline;
// use sph::particle::{self, Particle};
// use sph::traits::kernel::Kernel;

// use rand::Rng;

// pub fn main() {
//     let x1 = Point::<f32, 2>::new(0., 0.);
//     let x2 = Point::<f32, 2>::new(-1.0, 0.0);
//     let kernel = CubicSpline::<f32, 2>::new(1.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
//
//     let x1 = Point::<f64, 2>::new(0., 0.);
//     let x2 = Point::<f64, 2>::new(-1.0, 0.0);
//     let kernel = CubicSpline::<f64, 2>::new(2.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
//
//     let x1 = Point::<f32, 3>::new(0., 0., 0.0);
//     let x2 = Point::<f32, 3>::new(-1.0, 0.0, 0.);
//     let kernel = CubicSpline::<f32, 3>::new(1.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
//
//     let x1 = Point::<f64, 3>::new(0., 0., 0.);
//     let x2 = Point::<f64, 3>::new(-1.0, 0.0, 0.);
//     let kernel = CubicSpline::<f64, 3>::new(2.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
// }

pub fn main() {
    fn calc(inp: (usize, usize)) -> f32 {
        let (i, j) = inp;
        let (i, j): (f32, f32) = (i as f32, j as f32);
        i + j / 10.0
    }

    let system = ParticleSystem::<BasicState3<f32>>::from_shape_fn(15, calc);

    let positions = system.state.position();

    let (m, n) = positions.dim();
    println!("Positions:\t{m} x {n}");
    println!("{positions}");

    let velocities = system.state.velocity();
    let (m, n) = velocities.dim();
    println!("Velocities:\t{m} x {n}");
    println!("{velocities}");

    let masses = system.state.mass();
    let m = masses.dim();
    println!("Masses:\t\t{m:?}");
    println!("{masses}");

    let densities = system.state.density();
    let m = densities.dim();
    println!("Densities:\t{m:?}");
    println!("{densities}");

    let spins = system.state.spin();
    let m = densities.dim();
    println!("Spins:\t{m:?}");
    println!("{spins}");
}
