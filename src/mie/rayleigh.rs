use crate::mie;
use crate::mie::little_func::find_x;
use crate::mie::struct_def::{self, Efficiencies};
use num_complex::Complex;
use std::error::Error;

pub(crate) fn rayleigh_mie_q(
    particle: &mie::Particle,
    wavelength: f64,
    nmedium: f64,
) -> Result<Efficiencies, Box<dyn Error>> {
    let m = particle.m / nmedium;
    let wavelength = wavelength / nmedium;

    let x = find_x(particle.diameter, wavelength)?;

    let ll = (m.powi(2) - Complex::new(1.0, 0.0)) / (m.powi(2) + Complex::new(2.0, 0.0));
    let ll_abs_sq = ll.norm_sqr();
    let qsca = 8.0 * ll_abs_sq * (x.powi(4)) / 3.0;
    let qabs = 4.0 * x * ll.im;

    let efficiencies = struct_def::Efficiencies {
        qext: qsca + qabs,
        qsca,
        qabs,
        qpr: qsca + qabs,
        g: 0.0,
        qback: 1.5 * qsca,
        qratio: 1.5,
    };

    Ok(efficiencies)
}
