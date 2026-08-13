use crate::mie;
use crate::mie::little_func::{self, find_x};
use crate::mie::mie_ab;
use crate::mie::rayleigh;
use crate::mie::struct_def::{self, Efficiencies};
use num_complex::Complex;
use std::error::Error;

fn mie_q(
    particle: &mie::Particle,
    wavelength: f64,
    nmedium: Option<f64>,
    rayleigh_thresh: Option<f64>,
) -> Result<Efficiencies, Box<dyn Error>> {
    let nmedium = nmedium.unwrap_or(1.00027316);
    let rayleigh_thresh = rayleigh_thresh.unwrap_or(0.05);
    let x = find_x(particle.diameter, wavelength)?;

    if x <= rayleigh_thresh {
        return rayleigh::rayleigh_mie_q(particle, wavelength, Some(nmedium));
    }

    let nmax = (2.0 + x + 4.0 * (x.powf(1.0 / 3.0))).round() as usize;
    let n = little_func::arange(1, nmax + 1)?;

    let mut n1 = vec![0.0; n.len()];
    let mut n2 = n1.clone();
    let mut n3 = n1.clone();
    n1.iter_mut()
        .zip(n.iter())
        .for_each(|(n1_elem, &n)| *n1_elem = n * 2.0 + 1.0);
    n2.iter_mut()
        .zip(n.iter())
        .for_each(|(n2_elem, &n)| *n2_elem = n * (n + 2.0) / (n + 1.0));
    n3.iter_mut()
        .zip(n1.iter().zip(n.iter()))
        .for_each(|(n3_elem, (&n1, &n))| *n3_elem = n1 / (n * (n + 1.0)));

    let x2 = x.powi(2);

    let mie_coef = mie_ab::mie_ab(particle.m, x)?;

    let (an, bn) = (&mie_coef.an, &mie_coef.bn);

    let qext = (2.0 / x2)
        * n1.iter()
            .zip(an.iter().zip(bn.iter()))
            .map(|(&n1_v, (an_c, bn_c))| n1_v * (an_c.re + bn_c.re))
            .sum::<f64>();

    let qsca = (2.0 / x2)
        * n1.iter()
            .zip(an.iter().zip(bn.iter()))
            .map(|(&n1_v, (an_c, bn_c))| n1_v * (an_c.norm_sqr() + bn_c.norm_sqr()))
            .sum::<f64>();

    let qabs = qext - qsca;

    let g = calculate_g(&an, &bn, &n2, &n3, nmax, qsca, x2);

    let qpr = qext - qsca * g;

    let qback = calculate_qback(&an, &bn, &n1, nmax, x2);

    let qratio = qback / qsca;

    let effi = struct_def::Efficiencies {
        qabs,
        qext,
        qsca,
        qpr,
        g,
        qback,
        qratio,
    };

    // let css = little_func::PI * (diameter / 2.0).powi(2);
    Ok(effi)
}

pub fn auto_mie_q(
    particle: &mie::Particle,
    wavelength: f64,
    n_medium: Option<f64>,
    crossover: Option<f64>,
) -> Result<Efficiencies, Box<dyn Error>> {
    let n_medium = n_medium.unwrap_or(1.0);
    let crossover = crossover.unwrap_or(0.01);

    let wavelengh_eff = wavelength / n_medium;
    let x_eff = little_func::find_x(particle.diameter, wavelengh_eff)?;

    if x_eff < crossover {
        return rayleigh::rayleigh_mie_q(&particle, wavelength, Some(n_medium));
    } else {
        return mie_q(&particle, wavelength, Some(n_medium), None);
    }
}

fn calculate_g(
    an: &[Complex<f64>],
    bn: &[Complex<f64>],
    n2: &[f64],
    n3: &[f64],
    nmax: usize,
    qsca: f64,
    x2: f64,
) -> f64 {
    let slice_and_pad = |get_part: fn(&Complex<f64>) -> f64, arr: &[Complex<f64>]| -> Vec<f64> {
        let mut padded = vec![0.0; nmax];
        for i in 1..nmax {
            if let Some(c) = arr.get(i) {
                padded[i - 1] = get_part(c);
            }
        }
        padded
    };

    let g1_0 = slice_and_pad(|c| c.re, an); // an.real
    let g1_1 = slice_and_pad(|c| c.im, an); // an.imag
    let g1_2 = slice_and_pad(|c| c.re, bn); // bn.real
    let g1_3 = slice_and_pad(|c| c.im, bn); // bn.imag

    let total_sum: f64 = an
        .iter()
        .zip(bn.iter())
        .zip(n2.iter())
        .zip(n3.iter())
        .enumerate()
        .take(nmax)
        .map(|(i, (((an_c, bn_c), &n2_v), &n3_v))| {
            let ar = an_c.re;
            let ai = an_c.im;
            let br = bn_c.re;
            let bi = bn_c.im;

            let term1 = n2_v * (ar * g1_0[i] + ai * g1_1[i] + br * g1_2[i] + bi * g1_3[i]);
            let term2 = n3_v * (ar * br + ai * bi);
            term1 + term2
        })
        .sum();

    (4.0 / (qsca * x2)) * total_sum
}

fn calculate_qback(
    an: &[Complex<f64>],
    bn: &[Complex<f64>],
    n1: &[f64],
    nmax: usize,
    x2: f64,
) -> f64 {
    let sum_complex: Complex<f64> = an
        .iter()
        .zip(bn.iter())
        .zip(n1.iter())
        .enumerate()
        .take(nmax)
        .fold(Complex::new(0.0, 0.0), |acc, (i, ((an_c, bn_c), &n1_v))| {
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };

            let term = (an_c - bn_c) * (n1_v * sign);

            acc + term
        });

    let abs_sqr = sum_complex.norm_sqr();

    (1.0 / x2) * abs_sqr
}
