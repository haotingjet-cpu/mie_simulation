mod draw;
mod get_cdf;
mod mie;
mod photon;
mod simulat_const;
mod vector;
use std::error::Error;
use std::io::{self, Write};
use std::time;

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

    let mut rng = rand::rng();
    let mut theta_log = [0.0; FREQUENCY_IDX as usize];

    let now = time::Instant::now();

    for tim in 0..simulat_const::PHOTON_CONST {
        if tim % (simulat_const::PHOTON_CONST / 1000) == 0 {
            print!(
                "\r[{}:{}] | [{}{}] | [{:.1} %]",
                simulat_const::PHOTON_CONST,
                tim,
                "#".repeat(tim * 100 / PHOTON_CONST),
                ".".repeat(100 - tim * 100 / PHOTON_CONST - 1),
                tim as f32 * 100.0 / simulat_const::PHOTON_CONST as f32
            );
            io::stdout().flush()?;
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
    println!();
    println!("{:?}", now.elapsed());
    println!("{:?}", polyethylene);
    let theta_log: Vec<f64> = theta_log.into();
    let mut r_max = -1e200;
    let mut r_min = 1e200;
    theta_log.iter().for_each(|&x| {
        let x = x.log10();
        if x < r_min {
            r_min = x.round()
        } else if x > r_max {
            r_max = x.round() + 1.0
        }
    });
    println!("{}, {}", r_min, r_max);
    draw::draw(theta_log, r_min, r_max)?;
    Ok(())
}
