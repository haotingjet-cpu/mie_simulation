use crate::mie::AVOGADRO;
use crate::mie::{self, PI};
use crate::simulat_const::*;
use std::error::Error;

pub(crate) struct BulkCoefficients {
    pub mu_s: f64, // 體積散射係數
    pub mu_t: f64, // 體積消光係數
    pub mu_a: f64, // 體積吸收係數
}

pub(crate) fn get_sigm_sta() -> Result<(f64, f64, f64), Box<dyn Error>> {
    let r = POLYETHYLENE.diameter / 2.0;
    let effi = super::mie_q::auto_mie_q(None)?;
    let area = PI * r * r;
    let sigma_s = area * effi.qsca;
    let sigma_t = area * effi.qext;
    let sigma_a = sigma_t - sigma_s;
    Ok((sigma_s, sigma_t, sigma_a))
}

pub(crate) fn find_solution_mu_sta() -> Result<BulkCoefficients, Box<dyn Error>> {
    let sigm_1 = get_sigm_sta()?;
    let mu_s = (POLYETHYLENE.molarity * sigm_1.0) * AVOGADRO;
    let mu_t = (POLYETHYLENE.molarity * sigm_1.1) * AVOGADRO;
    let mu_a = (POLYETHYLENE.molarity * sigm_1.2) * AVOGADRO;
    Ok(BulkCoefficients { mu_s, mu_t, mu_a })
}
