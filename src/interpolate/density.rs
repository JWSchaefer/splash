// use crate::traits::float::Float;
// use crate::{Particle, traits::kernel::Kernel};
//
// use nalgebra::{RealField, Scalar};
// //
// // pub fn density<T, K, const D: usize>(
// //     particle_i: &Particle<T, D>,
// //     particles: &Vec<Particle<T, D>>,
// //     kernel: K,
// // ) -> T
// // where
// //     K: Kernel<T, D>,
// //     T: Float + RealField + Scalar,
// // {
// //     let mut density: T = <T as Float>::from_f64(0.);
// //
// //     for particle_j in particles {
// //         if particle_i == particle_j {
// //             continue;
// //         }
// //
// //         let cov: T = kernel.apply(particle_j.position, particle_i.position);
// //
// //         density += particle_j.mass * cov;
// //     }
// //
// //     density
// // }
