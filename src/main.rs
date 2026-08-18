mod get_cdf;
mod mie;
mod photon;
mod simulat_const;
mod vector;
use std::error::Error;

use crate::{
    get_cdf::get_cdf_fn,
    simulat_const::*,
    vector::{Vec3, Vec4},
};

fn main() -> Result<(), Box<dyn Error>> {
    let polyethylene = simulat_const::POLYETHYLENE;
    let bulkcoef = mie::get_mu::find_solution_mu_sta()?;
    println!("mu_t = {}", bulkcoef.mu_t);
    println!("mean free path = {}", 1.0 / bulkcoef.mu_t);
    let mul_theta = polyethylene.get_theta_vs_mueller_matrix()?;

    let cdf = get_cdf_fn(&mul_theta);
    println!("cdf: {:?}", cdf);

    let mut rng = rand::rng();
    let mut theta_log = [0.0; FREQUENCY_IDX as usize];

    for tim in 0..simulat_const::PHOTON_CONST {
        if tim % 10000 == 0 {
            println!("{}", tim)
        }
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
                None => {
                    break 'uni;
                }
            }
        }
    }
    println!("{:?}", polyethylene);
    for i in 0..FREQUENCY_IDX {
        theta_log[i] = theta_log[i].log10();
        print!(
            "|角度 {}: 值取 log 後{}| , ",
            (i as i64) * 180 / FREQUENCY,
            theta_log[i]
        );
    }
    Ok(())
}
