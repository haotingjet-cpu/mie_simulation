mod mie;
mod photon;
mod vector;
use std::error::Error;

use num_complex::Complex;

use crate::vector::{Vec3, Vec4};

fn main() -> Result<(), Box<dyn Error>> {
    let wavelength = 632.8;
    let polyethylene = mie::Particle {
        m: Complex::new(1.4998, 0.0),
        diameter: 2.0,
        molarity: 0.1,
    };
    let n_medium = 1.332;
    let bulkcoef = mie::get_mu::find_solution_mu_sta(&polyethylene, wavelength, n_medium)?;
    let theta_s1_s2 = polyethylene.get_half_round_s1s2(900, wavelength)?;
    let photon = photon::Photon {
        status: Vec4::new(10.0, 0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        start_location: Vec3::new(0.0, 0.0, 0.0),
    };
    todo!()
}
