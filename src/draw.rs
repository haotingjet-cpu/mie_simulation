use crate::simulat_const::SCALE;
use plotters::prelude::*;

pub(crate) fn draw(v: Vec<f64>, r_min: f64, r_max: f64) -> Result<(), Box<dyn std::error::Error>> {
    let width = 1000 * SCALE;
    let height = 1000 * SCALE;

    let root =
        BitMapBackend::new("mie_scattering_high_dpi.png", (width, height)).into_drawing_area();
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

    let logs =
        crate::mie::little_func::arange(r_min.round() as usize, r_max.round() as usize, Some(0.5))?;
    for &log_val in &logs {
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

    let lengh = v.len();
    let unpolarized_data: Vec<(f64, f64)> = v
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let angle = (i as f64 / lengh as f64) * 180.0;

            let mut log_val = x.log10();
            if log_val < r_min || log_val.is_nan() {
                log_val = r_min;
            }

            polar_to_xy(angle, log_val)
        })
        .collect();

    // 【高解析度調整】7. 主曲線粗細等比例放大 (2 -> 6)
    let line_style = ShapeStyle {
        color: BLACK.to_rgba(),
        filled: false,
        stroke_width: 1 * SCALE,
    };

    chart
        .draw_series(LineSeries::new(unpolarized_data, line_style.clone()))?
        .label("Unpolarized")
        .legend(move |(x, y)| {
            PathElement::new(vec![(x, y), (x + 20 * SCALE as i32, y)], line_style.clone())
        });

    chart
        .configure_series_labels()
        .background_style(WHITE.filled())
        .border_style(&BLACK)
        .label_font(("sans-serif", 14 * SCALE).into_font())
        .margin(20 * SCALE)
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;

    root.present()?;
    println!("高解析度圖片已成功生成於 mie_scattering_high_dpi.png！");
    Ok(())
}
