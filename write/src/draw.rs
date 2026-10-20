use std::{
    error::Error,
    path::{Path, PathBuf},
};

use plotters::prelude::*;

const SCALE: u32 = 10;
pub fn draw(
    ballistic: &[f64],
    scattered: &[f64],
    path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if ballistic.len() != scattered.len() || ballistic.is_empty() {
        return Err(
            "ballistic and scattered distributions must have equal, non-zero lengths".into(),
        );
    }

    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for &x in ballistic.iter().chain(scattered).filter(|&&x| x > 0.0) {
        let log_x = x.log10();
        lo = lo.min(log_x);
        hi = hi.max(log_x);
    }
    if !lo.is_finite() || !hi.is_finite() {
        return Err("cannot draw distributions without positive intensity values".into());
    }
    let (mut r_min, mut r_max) = (lo.floor(), hi.ceil());
    if r_min >= r_max {
        r_min -= 0.5;
        r_max += 0.5;
    }

    let output = &path;
    let width = 1000 * SCALE;
    let height = 1000 * SCALE;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let root = root.margin(50 * SCALE, 50 * SCALE, 50 * SCALE, 50 * SCALE);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Mie Scattering Intensity (Log10)",
            ("sans-serif", 30 * SCALE).into_font(),
        )
        .build_cartesian_2d(-1.2..1.2, -1.2..1.2)?;

    let polar_to_xy = |angle_deg: f64, log_i: f64| -> (f64, f64) {
        let r = (log_i - r_min) / (r_max - r_min);
        let r = r.max(0.0);
        let rad = (90.0 - angle_deg).to_radians();
        (r * rad.cos(), r * rad.sin())
    };

    let grid_color = RGBColor(211, 211, 211);

    let n_steps = ((r_max - r_min) / 0.5).round() as usize;
    for k in 0..=n_steps {
        let log_val = r_min + k as f64 * 0.5;
        let circle_points: Vec<(f64, f64)> =
            (0..=360).map(|a| polar_to_xy(a as f64, log_val)).collect();

        let grid_style = ShapeStyle {
            color: grid_color.to_rgba(),
            filled: false,
            stroke_width: 2,
        };

        chart.draw_series(std::iter::once(PathElement::new(circle_points, grid_style)))?;

        chart.draw_series(std::iter::once(Text::new(
            format!("{:.1}", log_val),
            polar_to_xy(45.0, log_val),
            ("sans-serif", 15 * SCALE).into_font(),
        )))?;
    }

    let angles = vec![0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0];
    for &angle in &angles {
        let start = polar_to_xy(angle, r_min);
        let end = polar_to_xy(angle, r_max);

        let grid_style = ShapeStyle {
            color: grid_color.to_rgba(),
            filled: false,
            stroke_width: 2,
        };

        chart.draw_series(std::iter::once(PathElement::new(
            vec![start, end],
            grid_style,
        )))?;

        let label_pos = polar_to_xy(angle, r_max + 0.4);
        chart.draw_series(std::iter::once(Text::new(
            format!("{}°", angle as i32),
            label_pos,
            ("sans-serif", 16 * SCALE).into_font(),
        )))?;
    }

    let to_polar_points = |values: &[f64]| {
        values
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let angle = (i as f64 + 0.5) / values.len() as f64 * 180.0;
                let log_val = if x > 0.0 {
                    x.log10().clamp(r_min, r_max)
                } else {
                    r_min
                };
                polar_to_xy(angle, log_val)
            })
            .collect::<Vec<_>>()
    };

    for (label, values, color) in [
        ("Unscattered", ballistic, BLUE),
        ("Scattered", scattered, BLACK),
    ] {
        let line_style = ShapeStyle {
            color: color.to_rgba(),
            filled: false,
            stroke_width: SCALE,
        };
        chart
            .draw_series(LineSeries::new(to_polar_points(values), line_style.clone()))?
            .label(label)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20 * SCALE as i32, y)], line_style.clone())
            });
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.filled())
        .border_style(&BLACK)
        .label_font(("sans-serif", 14 * SCALE).into_font())
        .margin(20 * SCALE)
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;

    root.present()?;
    let word = format!("圖片已儲存在 {}", output);
    Ok(word)
}

pub fn draw_from_hdf5(path: impl AsRef<Path> + Clone) -> Result<(), Box<dyn Error>> {
    let file = hdf5::File::open(path.clone())?;

    let scattered: Vec<f64> = file.dataset("scattered")?.read_raw()?;
    let ballistic: Vec<f64> = file.dataset("ballistic")?.read_raw()?;

    let output = convert_extension(path);
    let output = output.to_str().unwrap();

    draw(&ballistic, &scattered, output)?;
    Ok(())
}

fn convert_extension(path: impl AsRef<Path>) -> PathBuf {
    // 1. with_extension 會自動替換原本的副檔名（hdf5 變成 png）
    path.as_ref().with_extension("png")
}
