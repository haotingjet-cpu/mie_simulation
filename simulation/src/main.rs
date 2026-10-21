mod get_cdf;
mod mie;
mod photon;
mod simulat_const;
use std::error::Error;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{thread, time};

use crate::{get_cdf::get_cdf_fn, simulat_const::*};

type Histogram = [[f64; FREQUENCY_IDX]; 2];

fn main() -> Result<(), Box<dyn Error>> {
    write::draw_from_hdf5("result/2000/l.hdf5")?;
    return Ok(());
    let show_progress = !no_progress_requested(std::env::args().skip(1));
    let polyethylene = POLYETHYLENE;
    let bulkcoef = mie::get_mu::find_solution_mu_sta()?;

    println!("mu_t : {}", bulkcoef.mu_t);
    println!("自由路徑長 : {}", 1.0 / bulkcoef.mu_t);
    println!("光子數 : {}", PHOTON_CONST);
    println!("執行緒數量 : {}", WORKER_COUNT);
    println!("每個 CHUNK : {}", CUT);

    let mul_theta = polyethylene.get_theta_vs_mueller_matrix()?;

    let cdf = get_cdf_fn(&mul_theta);

    let now = time::Instant::now();
    let completed = AtomicUsize::new(0);
    let next_chunk = AtomicUsize::new(0);
    let worker_failed = AtomicBool::new(false);
    let theta_log = thread::scope(|scope| -> Result<Histogram, Box<dyn Error>> {
        let mut handles = Vec::with_capacity(WORKER_COUNT);

        for _ in 0..WORKER_COUNT {
            let completed = &completed;
            let next_chunk = &next_chunk;
            let worker_failed = &worker_failed;
            let bulkcoef = &bulkcoef;
            let mul_theta = &mul_theta;
            let cdf = &cdf;
            handles.push(scope.spawn(move || -> Result<Histogram, &'static str> {
                let mut rng = rand::rng();
                let mut theta_log = [[0.0; FREQUENCY_IDX]; 2];

                loop {
                    if worker_failed.load(Ordering::Relaxed) {
                        break;
                    }
                    let chunk_id = next_chunk.fetch_add(1, Ordering::Relaxed);
                    let Some(photons_in_chunk) = photons_in_chunk(chunk_id, PHOTON_CONST, CUT)
                    else {
                        break;
                    };
                    for _ in 0..photons_in_chunk {
                        if let Err(error) =
                            PHOTON.simulate(&mut rng, &mut theta_log, bulkcoef, mul_theta, cdf)
                        {
                            worker_failed.store(true, Ordering::Relaxed);
                            return Err(error);
                        }
                    }
                    completed.fetch_add(photons_in_chunk, Ordering::Relaxed);
                }
                Ok(theta_log)
            }));
        }

        let progress_interval = PHOTON_CONST.div_ceil(1000).max(1);
        let mut last_progress = 0;
        loop {
            let current = completed.load(Ordering::Relaxed).min(PHOTON_CONST);
            if show_progress
                && (current.saturating_sub(last_progress) >= progress_interval
                    || current == PHOTON_CONST)
            {
                update_progress(current, now)?;
                last_progress = current;
            }
            if current == PHOTON_CONST
                || worker_failed.load(Ordering::Relaxed)
                || handles.iter().all(|handle| handle.is_finished())
            {
                break;
            }
            thread::sleep(time::Duration::from_millis(1));
        }

        let mut theta_log = [[0.0; FREQUENCY_IDX]; 2];
        let mut worker_error = None;
        for handle in handles {
            match handle.join() {
                Ok(Ok(worker_log)) => add(&mut theta_log, worker_log),
                Ok(Err(error)) => {
                    worker_error.get_or_insert_with(|| io::Error::other(error));
                }
                Err(_) => {
                    worker_error
                        .get_or_insert_with(|| io::Error::other("simulation worker panicked"));
                }
            }
        }
        if let Some(error) = worker_error {
            return Err(error.into());
        }
        Ok(theta_log)
    })?;

    println!();

    println!("總耗時 : {:?}", now.elapsed());
    println!("{:?}", polyethylene);
    let ballistic = to_intensity_per_sr(&theta_log[0], PHOTON_CONST);
    let scattered = to_intensity_per_sr(&theta_log[1], PHOTON_CONST);

    if let Err(e) = write::storage_hdf5(&ballistic, &scattered) {
        println!("hdf5 儲存失敗");
        if show_progress {
            println!("{e:?}");
        }
    }

    let word = write::draw(&ballistic, &scattered, "latest_result.png")?;

    if show_progress {
        println!("{}", word);
    }
    Ok(())
}

fn add(final_log: &mut Histogram, log: Histogram) {
    for (final_category, log_category) in final_log.iter_mut().zip(log) {
        for (final_bin, log_bin) in final_category.iter_mut().zip(log_category) {
            *final_bin += log_bin;
        }
    }
}

fn photons_in_chunk(chunk_id: usize, total: usize, chunk_size: usize) -> Option<usize> {
    if chunk_size == 0 {
        return None;
    }
    let start = chunk_id.checked_mul(chunk_size)?;
    (start < total).then(|| chunk_size.min(total - start))
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
    let filled = (tim.saturating_mul(50) / PHOTON_CONST).min(50);
    print!(
        "\r[{}:{}] | [{}{}] | [{:.1} %] | [{}]",
        simulat_const::PHOTON_CONST,
        tim,
        "#".repeat(filled),
        ".".repeat(50 - filled),
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

#[cfg(test)]
mod tests {
    use super::{
        Histogram, add, estimate_remaining_time, format_duration, no_progress_requested,
        photons_in_chunk, to_intensity_per_sr,
    };
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

    #[test]
    fn histogram_merge_adds_every_category_and_bin() {
        let mut total = [[0.0; super::FREQUENCY_IDX]; 2];
        let mut worker = [[0.0; super::FREQUENCY_IDX]; 2];
        worker[0][0] = 1.0;
        worker[0][super::FREQUENCY_IDX - 1] = 2.0;
        worker[1][0] = 3.0;
        worker[1][super::FREQUENCY_IDX - 1] = 4.0;

        add(&mut total, worker);

        assert_eq!(total[0][0], 1.0);
        assert_eq!(total[0][super::FREQUENCY_IDX - 1], 2.0);
        assert_eq!(total[1][0], 3.0);
        assert_eq!(total[1][super::FREQUENCY_IDX - 1], 4.0);
        let _: Histogram = total;
    }

    #[test]
    fn chunk_sizing_covers_exact_and_partial_final_chunks() {
        let chunk_sizes: Vec<_> = (0..4)
            .filter_map(|id| photons_in_chunk(id, 23, 10))
            .collect();
        assert_eq!(chunk_sizes, [10, 10, 3]);
        assert_eq!(chunk_sizes.iter().sum::<usize>(), 23);
        assert_eq!(photons_in_chunk(0, 10, 0), None);
    }

    #[test]
    fn no_progress_flag_is_detected_without_matching_other_arguments() {
        assert!(no_progress_requested(["simulation", "--no_progress"]));
        assert!(!no_progress_requested(["simulation", "--no-progress"]));
        assert!(!no_progress_requested(["simulation"]));
    }
}

fn no_progress_requested(args: impl IntoIterator<Item = impl AsRef<str>>) -> bool {
    args.into_iter().any(|arg| arg.as_ref() == "--no_progress")
}
