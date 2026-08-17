use rand::RngExt;

use crate::get_cdf::{cdf_to_theta, get_cdf_fn};
use crate::mie::get_mu::BulkCoefficients;
use crate::photon::MuellerMatrix;
use crate::simulat_const::*;

impl super::Photon {
    pub(crate) fn collision_event(
        &mut self,
        rng: &mut impl RngExt,
        bul: &BulkCoefficients,
        mul_theta: &[MuellerMatrix; FREQUENCY_IDX],
        cdf: &[f64; FREQUENCY_IDX],
    ) {
        self.w = self.w * bul.mu_s / bul.mu_t;
        let idx = cdf_to_theta(rng, cdf);
        let mul_at_theta = mul_theta[idx];
    }
}
