use num_complex::Complex;
pub(crate) use std::f64::consts::PI;

pub(crate) mod get_mu;
pub(crate) mod little_func;
pub(crate) mod mie_ab;
pub(crate) mod mie_cd;
pub(crate) mod mie_pi_tau;
pub(crate) mod mie_q;
pub(crate) mod mie_s1_s2;
pub(crate) mod rayleigh;
pub(crate) mod struct_def;

pub(crate) const AVOGADRO: f64 = 6.02214076e23;

pub(crate) trait CIterator<T>: Iterator<Item = T> + Clone {}

impl<I, T> CIterator<T> for I where I: Iterator<Item = T> + Clone {}

pub(crate) struct MieCoefficients {
    pub(crate) an: Vec<Complex<f64>>,
    pub(crate) bn: Vec<Complex<f64>>,
}

pub(crate) struct Particle {
    pub(crate) m: Complex<f64>,
    pub(crate) diameter: f64,
    pub(crate) molarity: f64,
}
