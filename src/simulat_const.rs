use crate::mie;
use num_complex::Complex;
pub(crate) const POLYETHYLENE: mie::Particle = mie::Particle {
    m: Complex::new(1.4998, 0.0),
    diameter: 2.0,
    molarity: 0.1,
};

pub(crate) const RADIUS: f64 = 1.0;

pub(crate) const N_MEDIUM: f64 = 1.332;
pub(crate) const N_BOUNDARY: f64 = 1.45;
pub(crate) const WAVELENGH: f64 = 632.8;

pub(crate) const THETA_C_SIN: f64 = N_BOUNDARY / N_MEDIUM;
pub(crate) const THETA_C_SIN_2: f64 = THETA_C_SIN * THETA_C_SIN;
pub(crate) const THETA_C_SIN_2_REC: f64 = 1.0 / THETA_C_SIN_2;
