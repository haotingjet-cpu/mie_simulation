use crate::{
    photon::{self, MuellerMatrix},
    simulat_const::{FREQUENCY, N_MEDIUM, WAVELENGH_NM},
};
use num_complex::Complex;
use std::error::Error;
pub(crate) use std::f64::consts::PI;

pub(crate) mod get_mu;
pub(crate) mod little_func;
pub(crate) mod mie_ab;
pub(crate) mod mie_pi_tau;
pub(crate) mod mie_q;
pub(crate) mod mie_s1_s2;
pub(crate) mod rayleigh;
pub(crate) mod struct_def;

pub(crate) struct MieCoefficients {
    pub(crate) an: Vec<Complex<f64>>,
    pub(crate) bn: Vec<Complex<f64>>,
}

#[derive(Debug)]
pub(crate) struct Particle {
    pub(crate) m: Complex<f64>,
    pub(crate) diameter_nm: f64,
    pub(crate) molarity_mol_nm3: f64,
}

pub(crate) fn mie_parameters(particle: &Particle) -> Result<(Complex<f64>, f64), Box<dyn Error>> {
    let relative_index = particle.m / N_MEDIUM;
    let wavelength_medium_nm = WAVELENGH_NM / N_MEDIUM;
    let size_parameter = little_func::find_x(particle.diameter_nm, wavelength_medium_nm)?;
    Ok((relative_index, size_parameter))
}

impl Particle {
    pub(crate) fn get_theta_vs_mueller_matrix(
        &self,
    ) -> Result<Vec<MuellerMatrix>, Box<dyn std::error::Error>> {
        if FREQUENCY <= 1 {
            return Err("get_half_round_s1s2: frequency should bigger than 1".into());
        };
        let (m, x) = mie_parameters(self)?;

        let mie_coef = Some(&mie_ab::auto_mie_ab(m, x)?);

        let d = PI / FREQUENCY as f64;

        let matrices: Vec<MuellerMatrix> = (0..FREQUENCY)
            .map(|i| (((i as f64) + 0.5) * d).cos())
            .map(|mu| self::mie_s1_s2::mies1s2_one_theta(m, x, mu, mie_coef))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(s1, s2)| photon::get_mueller_matrix_with_s1s2(s1, s2))
            .collect();
        Ok(matrices)
    }
}
