mod get_cdf;
mod mie;
mod photon;
mod simulat_const;
use std::error::Error;
use std::io::{self, Write};
use std::time;
use vector::{Vec3, Vec4};

use crate::{get_cdf::get_cdf_fn, simulat_const::*};

fn main() -> Result<(), Box<dyn Error>> {
    let polyethylene = simulat_const::POLYETHYLENE;
    let bulkcoef = mie::get_mu::find_solution_mu_sta()?;
    println!("mu_t = {}", bulkcoef.mu_t);
    println!("mean free path = {}", 1.0 / bulkcoef.mu_t);
    let mul_theta = polyethylene.get_theta_vs_mueller_matrix()?;

    let cdf = get_cdf_fn(&mul_theta);

    let mut rng = rand::rng();
    let mut theta_log = [[0.0; FREQUENCY_IDX]; 2];

    let now = time::Instant::now();

    for tim in 0..simulat_const::PHOTON_CONST {
        if tim % (simulat_const::PHOTON_CONST / 10000) == 0 {
            update_progress(tim, now)?;
        }
        let mut photon = new_photon();

        loop {
            match photon.move_a_path(&mut theta_log, &mut rng, bulkcoef.mu_t) {
                Some(t) => {
                    t.collision_event(&mut rng, &bulkcoef, &mul_theta, &cdf)?;
                    if !t.alive_check(&mut rng) {
                        break;
                    };
                }
                None => {
                    break;
                }
            }
        }
    }
    println!();
    println!("{:?}", now.elapsed());
    println!("{:?}", polyethylene);
    let ballistic = to_intensity_per_sr(&theta_log[0], PHOTON_CONST);
    let scattered = to_intensity_per_sr(&theta_log[1], PHOTON_CONST);
    draw::draw(&ballistic, &scattered)?;
    Ok(())
}

fn to_intensity_per_sr(counts: &[f64], n_photons: usize) -> Vec<f64> {
    let d = std::f64::consts::PI / counts.len() as f64;
    counts
        .iter()
        .enumerate()
        .map(|(i, &count)| {
            let omega =
                2.0 * std::f64::consts::PI * ((i as f64 * d).cos() - ((i + 1) as f64 * d).cos());
            count / (n_photons as f64 * omega)
        })
        .collect()
}

fn update_progress(tim: usize, started_at: time::Instant) -> io::Result<()> {
    let elapsed = started_at.elapsed();
    let eta = estimate_remaining_time(tim, PHOTON_CONST, elapsed)
        .map(format_duration)
        .unwrap_or_else(|| "estimating".to_owned());
    print!(
        "\r[{}:{}] | [{}{}] | [{:.1} %] | ETA {}",
        simulat_const::PHOTON_CONST,
        tim,
        "#".repeat(tim * 100 / PHOTON_CONST),
        ".".repeat(100 - tim * 100 / PHOTON_CONST - 1),
        tim as f32 * 100.0 / simulat_const::PHOTON_CONST as f32,
        eta,
    );
    io::stdout().flush()
}

fn estimate_remaining_time(
    completed: usize,
    total: usize,
    elapsed: time::Duration,
) -> Option<time::Duration> {
    if completed == 0 || total == 0 {
        return None;
    }

    let remaining = total.saturating_sub(completed);
    let seconds_per_item = elapsed.as_secs_f64() / completed as f64;
    Some(time::Duration::from_secs_f64(
        seconds_per_item * remaining as f64,
    ))
}

fn format_duration(duration: time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn new_photon() -> photon::Photon {
    photon::Photon {
        stokes_vector: Vec4::new(1.0, 0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        start_location: Vec3::new(0.0, 0.0, 0.0),
        last_plane_normal_v: Vec3::new(0.0, 0.0, 1.0),
        w: 1.0,
        scattered: false,
    }
}

#[cfg(test)]
mod tests {
    use super::{estimate_remaining_time, format_duration, to_intensity_per_sr};
    use std::f64::consts::PI;
    use std::time::Duration;

    #[test]
    fn solid_angle_normalization_recovers_constant_intensity() {
        let n_photons = 100;
        let bin_count = 20;
        let step = PI / bin_count as f64;
        let intensity = 2.5;
        let counts: Vec<f64> = (0..bin_count)
            .map(|i| {
                let solid_angle =
                    2.0 * PI * ((i as f64 * step).cos() - ((i + 1) as f64 * step).cos());
                n_photons as f64 * solid_angle * intensity
            })
            .collect();

        let normalized = to_intensity_per_sr(&counts, n_photons);
        assert!(
            normalized
                .iter()
                .all(|value| (value - intensity).abs() < 1e-12)
        );
    }

    #[test]
    fn remaining_time_uses_average_photon_rate() {
        assert_eq!(
            estimate_remaining_time(25, 100, Duration::from_secs(10)),
            Some(Duration::from_secs(30))
        );
        assert_eq!(
            estimate_remaining_time(0, 100, Duration::from_secs(10)),
            None
        );
    }

    #[test]
    fn duration_is_formatted_as_hours_minutes_seconds() {
        assert_eq!(format_duration(Duration::from_secs(3661)), "01:01:01");
    }
}
