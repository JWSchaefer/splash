use std::f64;

use nalgebra::{Point2, Vector2};
use sph::interpolate::density::{self, density};
use sph::kernels::cubic_spline::CubicSpline;
use sph::particle::{self, Particle};
use sph::traits::kernel::Kernel;

use rand::Rng;

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
    let mut rng = rand::thread_rng();
    let mut particles = Vec::<Particle<f64, 2>>::new();

    let mut kernels = Vec::<CubicSpline<f64, 2>>::new();

    for h in [0.1, 0.2, 0.25, 0.4, 0.5, 0.8, 1., 1.6, 2., 3.2, 4., 4.5, 5.] {
        kernels.push(CubicSpline::<f64, 2>::new(h));
    }

    let num_particles: usize = 1_000;

    for i in 0..num_particles {
        particles.push(Particle::<f64, 2>::new(
            i,
            Point2::new(rng.gen::<f64>() * 10., rng.gen::<f64>() * 10.),
            Vector2::new(0., 0.),
            1.0,
        ));
    }

    for kernel in kernels {
        let h = &kernel.h.clone();

        let density = density(&particles, kernel);

        for _density in density {
            println!("{_density}");
        }
    }
}
