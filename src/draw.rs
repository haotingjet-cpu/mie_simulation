use plotters::prelude::*;

pub(crate) fn draw(v: Vec<f64>, r_min: f64, r_max: f64) -> Result<(), Box<dyn std::error::Error>> {
    // 【高解析度調整】1. 放大畫布像素尺寸 (1000 -> 3000)
    let scale = 3;
    let width = 1000 * scale;
    let height = 1000 * scale;

    let root =
        BitMapBackend::new("mie_scattering_high_dpi.png", (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // 【高解析度調整】2. 邊距也要等比例放大 (50 -> 150)
    let root = root.margin(50 * scale, 50 * scale, 50 * scale, 50 * scale);

    // 建立直角座標系
    let mut chart = ChartBuilder::on(&root)
        // 【高解析度調整】3. 標題字體大小放大 (30 -> 90)
        .caption(
            "Mie Scattering Intensity (Log10)",
            ("sans-serif", 30 * scale).into_font(),
        )
        .build_cartesian_2d(-1.2..1.2, -1.2..1.2)?;

    let polar_to_xy = |angle_deg: f64, log_i: f64| -> (f64, f64) {
        let r = (log_i - r_min) / (r_max - r_min);
        let r = r.max(0.0);
        let rad = (90.0 - angle_deg).to_radians();
        (r * rad.cos(), r * rad.sin())
    };

    let grid_color = RGBColor(211, 211, 211);

    // 繪製同心圓網格
    let logs = crate::mie::little_func::arange(r_min.round() as usize, r_max.round() as usize + 1)?;
    for &log_val in &logs {
        let circle_points: Vec<(f64, f64)> =
            (0..=360).map(|a| polar_to_xy(a as f64, log_val)).collect();

        // 【高解析度調整】4. 網格線條粗細稍微加粗 (預設 1 -> 2)
        let grid_style = ShapeStyle {
            color: grid_color.to_rgba(),
            filled: false,
            stroke_width: 2,
        };

        chart.draw_series(std::iter::once(PathElement::new(circle_points, grid_style)))?;

        // 【高解析度調整】5. 刻度字體大小放大 (15 -> 45)
        chart.draw_series(std::iter::once(Text::new(
            format!("{}", log_val as i32),
            polar_to_xy(45.0, log_val),
            ("sans-serif", 15 * scale).into_font(),
        )))?;
    }

    // 繪製放射狀角度線
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

        // 【高解析度調整】6. 角度標籤字體大小放大 (16 -> 48)
        let label_pos = polar_to_xy(angle, r_max + 0.4);
        chart.draw_series(std::iter::once(Text::new(
            format!("{}°", angle as i32),
            label_pos,
            ("sans-serif", 16 * scale).into_font(),
        )))?;
    }

    // 繪製散射曲線數據
    // let unpolarized_data: Vec<(f64, f64)> = (0..=180)
    //     .map(|deg| {
    //         let angle = deg as f64;
    //         let intensity = -5.5 + 1.2 * (angle * 0.1).cos() + 0.4 * (angle * 0.4).sin();
    //         polar_to_xy(angle, intensity)
    //     })
    //     .collect();
    let lengh = v.len();
    let unpolarized_data: Vec<(f64, f64)> = v
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            // 【修正核心】先轉成 f64 避免整數除法變成 0
            let angle = (i as f64 / lengh as f64) * 180.0;

            // 安全防護：取 log10，並避免小於我們的圖表下限 r_min (-8.0)
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
        stroke_width: 2 * scale,
    };

    chart
        .draw_series(LineSeries::new(unpolarized_data, line_style.clone()))?
        .label("Unpolarized")
        // 【高解析度調整】8. 圖例中的線條長度與粗細也同步放大 (x + 20 -> x + 60)
        .legend(move |(x, y)| {
            PathElement::new(vec![(x, y), (x + 20 * scale as i32, y)], line_style.clone())
        });

    // 配置圖例
    chart
        .configure_series_labels()
        .background_style(WHITE.filled())
        .border_style(&BLACK)
        // 【高解析度調整】9. 圖例字體、間距與外框粗細放大
        .label_font(("sans-serif", 14 * scale).into_font())
        .margin(20 * scale)
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;

    root.present()?;
    println!("高解析度圖片已成功生成於 mie_scattering_high_dpi.png！");
    Ok(())
}
