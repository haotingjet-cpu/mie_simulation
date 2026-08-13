use crate::mie::{self, CIterator, little_func};
use complex_bessel::{besselj, bessely};
use num_complex::Complex;
use std::error::Error;

pub(crate) fn mie_cd(
    m: Complex<f64>,
    x: f64,
) -> Result<(Vec<Complex<f64>>, Vec<Complex<f64>>), Box<dyn Error>> {
    let mx = m * Complex::new(x, 0.0);

    let (nmax, nmx) = calculate_expansion_limits(mx, x);

    let n = little_func::arange_iter(1, nmax as usize + 1)?;

    let nu = n.clone().map(|x| x + 0.5);

    let cnn = calculate_cnn(nmx, nmax, mx)?;

    let jnx = compute_jnx(nu.clone(), x)?;

    let jnmx = compute_jnmx(nu.clone(), mx);

    let yx = compute_yx(nu, x)?;

    let hx = jnx
        .iter()
        .zip(yx.iter())
        .map(|(&jnx, &yx)| jnx + Complex::new(0.0, 1.0) * yx);

    let b1x = std::iter::once(Complex::new(x.sin() / x, 0.0))
        .chain(jnx.clone().into_iter().take(nmax as usize - 1));

    let y1x = std::iter::once(Complex::new(-x.cos() / x, 0.0))
        .chain(yx.clone().into_iter().take(nmax as usize - 1));

    let hn1x = b1x
        .clone()
        .zip(y1x)
        .map(|(b1x, y1x)| b1x + Complex::new(0.0, 1.0) * y1x);

    let ax = b1x
        .zip(n.clone().zip(jnx.iter()))
        .map(move |(b1x, (n, jnx))| x * b1x - n * jnx);

    let ahx = hn1x
        .zip(n.zip(hx.clone()))
        .map(move |(hn1x, (n, hx))| Complex::new(x, 0.0) * hn1x - n * hx);

    let numerator = jnx
        .iter()
        .zip(ahx.clone().zip(hx.clone().zip(ax)))
        .map(|(&jnx, (ahx, (hx, ax)))| jnx * ahx - hx * ax);

    let c_denominator = ahx
        .clone()
        .zip(hx.clone().zip(cnn.clone()))
        .map(|(ahx, (hx, cnn))| ahx - hx * cnn);
    let d_denominator = ahx
        .zip(hx.zip(cnn))
        .map(move |(ahx, (hx, cnn))| m * m * ahx - hx * cnn);

    let cn = jnmx
        .clone()
        .zip(numerator.clone().zip(c_denominator))
        .map(|(j, (n, c))| j.map(|j| j * n / c))
        .collect::<Result<Vec<Complex<f64>>, _>>()?;

    let dn = jnmx
        .zip(numerator.zip(d_denominator))
        .map(|(j, (n, d))| j.map(|j| j * m * n / d))
        .collect::<Result<Vec<Complex<f64>>, _>>()?;

    Ok((cn, dn))
}

fn calculate_expansion_limits(mx: Complex<f64>, x: f64) -> (f64, f64) {
    let nmax = (2.0 + x + 4.0 * (x.powf(1.0 / 3.0))).round();
    let nmx = (nmax.max(mx.norm()) + 16.0).round().max(nmax + 1.0);
    (nmax, nmx)
}

fn calculate_cnn(
    nmx: f64,
    nmax: f64,
    mx: Complex<f64>,
) -> Result<impl CIterator<Complex<f64>>, Box<dyn Error>> {
    let mut cnx = vec![Complex::<f64>::ZERO; nmx as usize];

    for j in little_func::arange_iter(2, nmx as usize + 1)?.rev() {
        cnx[j as usize - 2] =
            Complex::new(j, 0.0) - mx * mx / (cnx[j as usize - 1] + Complex::new(j, 0.0));
    }

    let cnn = cnx.into_iter().take(nmax as usize);
    Ok(cnn)
}

fn compute_jnx<I>(nu: I, x: f64) -> Result<Vec<Complex<f64>>, Box<dyn Error>>
where
    I: CIterator<f64>,
{
    let jnx = nu
        .map(|nu| besselj(nu, Complex::new(x, 0.0)).map(|bj| bj * (mie::PI / (2.0 * x)).sqrt()))
        .collect::<Result<Vec<Complex<f64>>, _>>()?;

    Ok(jnx)
}

fn compute_jnmx<I>(
    nu: I,
    mx: Complex<f64>,
) -> impl CIterator<Result<Complex<f64>, complex_bessel::Error>>
where
    I: CIterator<f64>,
{
    let jnmx = nu.map(move |nu| besselj(nu, mx).map(|bj| (2.0 * mx / mie::PI).sqrt() / bj));

    jnmx
}

fn compute_yx<I>(nu: I, x: f64) -> Result<Vec<Complex<f64>>, Box<dyn Error>>
where
    I: CIterator<f64>,
{
    let yx: Vec<Complex<f64>> = nu
        .map(|nu| bessely(nu, Complex::new(x, 0.0)).map(|by| by * (mie::PI / (2.0 * x)).sqrt()))
        .collect::<Result<Vec<Complex<f64>>, _>>()?;
    Ok(yx)
}
