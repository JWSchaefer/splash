use crate::traits::float::Float;
use crate::{particle::Particle, traits::kernel::Kernel};

use nalgebra::{RealField, Scalar, SVector};
use num::Zero;

pub fn force_viscous<T, K, const D: usize>(
    particle_i: &Particle<T, D>,
    particles: &Vec<Particle<T, D>>,
    kernel: K,
) -> SVector<T, D>
where
    K: Kernel<T, D>,
    T: Float + RealField + Scalar,
{
    let mut force_viscous = SVector::<T, D>::zero();
    for particle_j in particles {
        if particle_i == particle_j {
            continue;
        }

        let cov: T = kernel.apply(particle_j.position, particle_i.position);

        /* Do stuff here */
    }
    force_viscous
}
