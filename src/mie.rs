use crate::{
    photon::MuellerMatrix,
    simulat_const::{FREQUENCY, WAVELENGH},
};
use num_complex::Complex;
pub(crate) use std::f64::consts::PI;

pub(crate) mod get_mu;
pub(crate) mod little_func;
pub(crate) mod mie_ab;
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

impl Particle {
    pub(crate) fn get_theta_vs_mueller_matrix(
        &self,
    ) -> Result<Vec<MuellerMatrix>, Box<dyn std::error::Error>> {
        if FREQUENCY <= 1 {
            return Err("get_half_round_s1s2: frequency should bigger than 1".into());
        };
        let x = little_func::find_x(self.diameter, WAVELENGH)?;

        let mie_coef = Some(&mie_ab::auto_mie_ab(self.m, x)?);

        let d = PI / (FREQUENCY as f64 - 1.0);

        let x = (0..FREQUENCY)
            .map(|i| ((i as f64) * d).cos())
            .map(|mu| self::mie_s1_s2::mies1s2_one_theta(self.m, x, mu, mie_coef).unwrap())
            .map(|(s1, s2)| MuellerMatrix::get_mueller_matrix_with_s1s2(s1, s2))
            .collect();
        Ok(x)
    }
}
