mod mie;
mod photon;
mod simulat_const;
mod vector;
use std::error::Error;

use num_complex::Complex;

use crate::vector::{Vec3, Vec4};

fn main() -> Result<(), Box<dyn Error>> {
    let polyethylene = simulat_const::POLYETHYLENE;
    let bulkcoef = mie::get_mu::find_solution_mu_sta()?;
    let photon = photon::Photon {
        status: Vec4::new(10.0, 0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        start_location: Vec3::new(0.0, 0.0, 0.0),
        last_plane_normal_v: Vec3::new(0.0, 0.0, 1.0),
        w: 0.0,
    };

    let mul_theta = polyethylene.get_theta_vs_mueller_matrix()?;
    todo!()
}
