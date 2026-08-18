mod get_cdf;
mod mie;
mod photon;
mod simulat_const;
mod vector;
use std::error::Error;

use crate::{
    get_cdf::get_cdf_fn,
    simulat_const::FREQUENCY_IDX,
    vector::{Vec3, Vec4},
};

fn main() -> Result<(), Box<dyn Error>> {
    let polyethylene = simulat_const::POLYETHYLENE;
    let bulkcoef = mie::get_mu::find_solution_mu_sta()?;

    let mul_theta = polyethylene.get_theta_vs_mueller_matrix()?;

    let cdf = get_cdf_fn(&mul_theta);

    let mut rng = rand::rng();
    let mut theta_log = [0.0; FREQUENCY_IDX as usize];

    for _ in 0..simulat_const::PHOTON_CONST {
        let mut photon = photon::Photon {
            status: Vec4::new(10.0, 0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
            start_location: Vec3::new(0.0, 0.0, 0.0),
            last_plane_normal_v: Vec3::new(0.0, 0.0, 1.0),
            w: 1.0,
        };
        'uni: loop {
            match photon.move_a_path(&mut theta_log, &mut rng, bulkcoef.mu_t) {
                Some(t) => {
                    t.collision_event(&mut rng, &bulkcoef, &mul_theta, &cdf);
                    if !(t.russion_roulette(&mut rng)) {
                        break 'uni;
                    };
                }
                None => break 'uni,
            }
        }
    }

    println!("{:?}", theta_log);
    Ok(())
}
