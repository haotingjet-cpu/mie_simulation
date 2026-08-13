use crate::mie::AVOGADRO;
use crate::mie::{self, PI};
use std::error::Error;

pub(crate) struct BulkCoefficients {
    pub mu_s: f64, // 體積散射係數
    pub mu_t: f64, // 體積消光係數
    pub mu_a: f64, // 體積吸收係數
}

pub(crate) fn get_sigm_sta(
    particle: &mie::Particle,
    wavelength: f64,
    n_medium: Option<f64>,
) -> Result<(f64, f64, f64), Box<dyn Error>> {
    let r = particle.diameter / 2.0;
    let effi = super::mie_q::auto_mie_q(particle, wavelength, n_medium, None)?;
    let area = PI * r * r;
    let sigma_s = area * effi.qsca;
    let sigma_t = area * effi.qext;
    let sigma_a = sigma_t - sigma_s;
    Ok((sigma_s, sigma_t, sigma_a))
}

pub(crate) fn find_solution_musta(
    particle1: &mie::Particle,
    particle2: &mie::Particle,
    wavelength: f64,
    n_medium: f64,
) -> Result<BulkCoefficients, Box<dyn Error>> {
    let sigm_1 = get_sigm_sta(particle1, wavelength, Some(n_medium))?;
    let sigm_2 = get_sigm_sta(particle2, wavelength, Some(n_medium))?;
    let mu_s = (particle1.molarity * sigm_1.0 + particle2.molarity * sigm_2.0) * AVOGADRO;
    let mu_t = (particle1.molarity * sigm_1.1 + particle2.molarity * sigm_2.1) * AVOGADRO;
    let mu_a = (particle1.molarity * sigm_1.2 + particle2.molarity * sigm_2.2) * AVOGADRO;
    Ok(BulkCoefficients { mu_s, mu_t, mu_a })
}
