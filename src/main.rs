use anyhow::Result;
use sph::kernel::cubic_spline::CubicSpline;
use sph::states::basic_state::BasicState2;
use sph::systems::particle::ParticleSystem;
use sph::traits::kernel::Kernel;
use sph::Float;

pub fn main() -> Result<()> {
    fn calc(inp: (usize, usize)) -> Float {
        let (i, j) = inp;
        let (_i, j) = (i as Float, j as Float);
        j / 100.0
    }

    let n_particles: usize = 400;

    let system =
        ParticleSystem::<BasicState2>::from_shape_fn(n_particles, calc);

    let positions = system.state.position();

    let kernel = CubicSpline::<2>::new(1.0)?;

    for i in 0..n_particles {
        /*         let _ = kernel.apply(positions.column(11), positions.column(i))?; */
        let _ = kernel
            .apply_derivative(positions.column(11), positions.column(i))?;
    }

    Ok(())
}

// pub fn main() -> Result<()> {
//     fn calc(inp: (usize, usize)) -> Float {
//         let (i, j) = inp;
//         let (i, j) = (i as Float, j as Float);
//         i + j / 10.0
//     }
//
//     let system = ParticleSystem::<BasicState3>::from_shape_fn(15, calc);
//
//     let positions = system.state.position();
//
//     // let (m, n) = positions.dim();
//     // println!("Positions:\t{m} x {n}");
//     // println!("{positions}");
//
//     // let velocities = system.state.velocity();
//     // let (m, n) = velocities.dim();
//     // println!("Velocities:\t{m} x {n}");
//     // println!("{velocities}");
//
//     // let masses = system.state.mass();
//     // let m = masses.dim();
//     // println!("Masses:\t\t{m:?}");
//     // println!("{masses}");
//
//     // let densities = system.state.density();
//     // let m = densities.dim();
//     // println!("Densities:\t{m:?}");
//     // println!("{densities}");
//
//     // let spins = system.state.spin();
//     // let m = densities.dim();
//     // println!("Spins:\t{m:?}");
//     // println!("{spins}");
// }
