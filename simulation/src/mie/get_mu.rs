use crate::mie::PI;
use crate::simulat_const::{AVOGADRO, POLYETHYLENE};
use std::error::Error;

pub(crate) struct BulkCoefficients {
    pub mu_s: f64,  // 體積散射係數
    pub mu_t: f64,  // 體積消光係數
    pub _mu_a: f64, // 體積吸收係數
}

fn validate_particle_volume_fraction(
    particle_density_per_nm3: f64,
    diameter_nm: f64,
) -> Result<(), Box<dyn Error>> {
    let particle_volume_fraction = particle_density_per_nm3 * (PI / 6.0) * diameter_nm.powi(3);
    if !particle_volume_fraction.is_finite() || particle_volume_fraction > 1.0 {
        return Err(format!(
            "particle volume fraction ({particle_volume_fraction:e}) exceeds the physically possible limit of 1"
        )
        .into());
    }
    Ok(())
}

pub(crate) fn get_sigm_sta() -> Result<(f64, f64, f64), Box<dyn Error>> {
    let r = POLYETHYLENE.diameter_nm / 2.0;
    let effi = super::mie_q::auto_mie_q(None)?;
    let area = PI * r * r;
    let sigma_s = area * effi.qsca;
    let sigma_t = area * effi.qext;
    let sigma_a = sigma_t - sigma_s;
    Ok((sigma_s, sigma_t, sigma_a))
}

pub(crate) fn find_solution_mu_sta() -> Result<BulkCoefficients, Box<dyn Error>> {
    let particle_density_per_nm3 = POLYETHYLENE.molarity_mol_nm3 * AVOGADRO;
    validate_particle_volume_fraction(particle_density_per_nm3, POLYETHYLENE.diameter_nm)?;
    let sigm_1 = get_sigm_sta()?;
    let mu_s = particle_density_per_nm3 * sigm_1.0;
    let mu_t = particle_density_per_nm3 * sigm_1.1;
    let mu_a = particle_density_per_nm3 * sigm_1.2;
    Ok(BulkCoefficients {
        mu_s,
        mu_t,
        _mu_a: mu_a,
    })
}

#[cfg(test)]
mod tests {
    use super::validate_particle_volume_fraction;

    #[test]
    fn rejects_impossible_particle_volume_fraction() {
        let error = match validate_particle_volume_fraction(1.0, 1200.0) {
            Ok(()) => panic!("impossible particle volume fraction should be rejected"),
            Err(error) => error.to_string(),
        };

        assert!(error.contains("particle volume fraction"));
    }
}
