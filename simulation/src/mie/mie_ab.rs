use crate::mie::MieCoefficients;
use num_complex::Complex;
use std::error::Error;
pub fn mie_ab(m: Complex<f64>, x: f64) -> Result<MieCoefficients, Box<dyn Error>> {
    let mx = m * Complex::new(x, 0.0);
    let nmax = (2.0 + x + 4.0 * x.cbrt()).round();
    let nmx = (nmax.max(mx.norm()) + 16.0).round().max(nmax + 1.0);

    let nmax_len = nmax as usize;
    let nmx_int = nmx as usize;

    let mut px = vec![Complex::new(0.0, 0.0); nmax_len];
    let mut p1x = vec![Complex::new(0.0, 0.0); nmax_len];
    let mut chx = vec![Complex::new(0.0, 0.0); nmax_len];
    let mut ch1x = vec![Complex::new(0.0, 0.0); nmax_len];
    let mut dn = vec![Complex::new(0.0, 0.0); nmx_int];

    let (sin_x, cos_x) = x.sin_cos();
    p1x[0] = Complex::new(sin_x, 0.0);
    px[0] = Complex::new(sin_x / x - cos_x, 0.0);
    ch1x[0] = Complex::new(cos_x, 0.0);
    chx[0] = Complex::new(cos_x / x + sin_x, 0.0);

    for n in 1..nmax_len {
        let nf = (n + 1) as f64;
        let factor = (2.0 * nf - 1.0) / x;
        p1x[n] = px[n - 1];
        px[n] = factor * px[n - 1] - p1x[n - 1];
        ch1x[n] = chx[n - 1];
        chx[n] = factor * chx[n - 1] - ch1x[n - 1];
    }

    for i in (2..nmx_int).rev() {
        let tem = Complex::new(i as f64, 0.0) / mx;
        dn[i - 1] = tem - (Complex::new(1.0, 0.0) / (dn[i] + tem));
    }

    let mut an = Vec::with_capacity(nmax_len);
    let mut bn = Vec::with_capacity(nmax_len);

    let j_comp = Complex::new(0.0, 1.0);
    for i in 0..nmax_len {
        let n_f = (i + 1) as f64;
        let d = dn[i + 1];

        let p = px[i];
        let p1 = p1x[i];
        let ch = chx[i];
        let ch1 = ch1x[i];

        let gs = p - j_comp * ch;
        let gs1 = p1 - j_comp * ch1;
        let n_over_x = Complex::new(n_f / x, 0.0);

        let da = d / m + n_over_x;
        let db = d * m + n_over_x;

        an.push((da * p - p1) / (da * gs - gs1));
        bn.push((db * p - p1) / (db * gs - gs1));
    }

    Ok(MieCoefficients { an, bn })
}

fn low_frequency_mie_ab(m: Complex<f64>, x: f64) -> MieCoefficients {
    let m2 = m * m;
    let ll = (m * m - 1.0) / (m * m + 2.0);
    let x3 = x.powi(3);
    let x5 = x.powi(5);
    let x6 = x.powi(6);

    let a1 = (-Complex::new(0.0, 2.0) * x3 / 3.0) * ll
        - (Complex::new(0.0, 2.0) * x5 / 5.0) * ll * (m2 - 2.0) / (m2 + 2.0)
        + (4.0 * x6 / 9.0) * (ll.powi(2));

    let a2 = (-Complex::new(0.0, 1.0) * x5 / 15.0) * (m2 - 1.0) / (2.0 * m2 + 3.0);

    let b1 = (-Complex::new(0.0, 1.0) * x5 / 45.0) * (m2 - 1.0);
    let b2 = Complex::<f64>::ZERO;

    let an = vec![a1, a2];
    let bn = vec![b1, b2];

    let mie_coef = MieCoefficients { an, bn };

    return mie_coef;
}

pub(crate) fn auto_mie_ab(m: Complex<f64>, x: f64) -> Result<MieCoefficients, Box<dyn Error>> {
    if x < 0.5 {
        Ok(low_frequency_mie_ab(m, x))
    } else {
        mie_ab(m, x)
    }
}
