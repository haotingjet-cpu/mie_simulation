mod get_cdf;
mod mie;
mod photon;
mod simulat_const;
mod vector;
use std::error::Error;

use crate::{
    get_cdf::get_cfd_fn,
    simulat_const::FREQUENCY,
    vector::{Vec3, Vec4},
};
use rand::RngExt;

fn main() -> Result<(), Box<dyn Error>> {
    let polyethylene = simulat_const::POLYETHYLENE;
    let bulkcoef = mie::get_mu::find_solution_mu_sta()?;

    let mul_theta = polyethylene.get_theta_vs_mueller_matrix()?;

    let _cfd = get_cfd_fn(&mul_theta);

    let mut rng = rand::rng();
    let mut theta_log = [0.0; FREQUENCY as usize];

    for _ in 0..5 {
        let mut photon = photon::Photon {
            status: Vec4::new(10.0, 0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
            start_location: Vec3::new(0.0, 0.0, 0.0),
            last_plane_normal_v: Vec3::new(0.0, 0.0, 1.0),
            w: 1.0,
        };
        'uni: loop {
            match photon.move_a_path(&mut theta_log, &mut rng, bulkcoef.mu_t) {
                Some(t) => t.collision_event(&bulkcoef),
                None => break 'uni,
            }
        }
    }

    todo!()
}
