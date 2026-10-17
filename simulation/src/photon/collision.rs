use std::f64::consts::PI;

use rand::RngExt;

use crate::get_cdf::cdf_to_theta;
use crate::mie::get_mu::BulkCoefficients;
use crate::photon::MuellerMatrix;
use crate::simulat_const::*;
use std::error::Error;
use vector::{Cross, Norm, Vec4};

impl super::Photon {
    pub(crate) fn collision_event(
        &mut self,
        rng: &mut impl RngExt,
        bul: &BulkCoefficients,
        mul_theta: &[MuellerMatrix],
        cdf: &[f64],
    ) -> Result<(), Box<dyn Error>> {
        let idx = cdf_to_theta(rng, cdf);
        let theta = (idx as f64 + 0.5) / FREQUENCY as f64 * PI;
        let mul_at_theta = mul_theta[idx];
        let zeta: f64 = rng.random();
        let psi = zeta * 2.0 * PI;
        let (sin_t, cos_t) = theta.sin_cos();
        let (sin_p, cos_p) = psi.sin_cos();
        self.direction = self.direction.normalize();
        self.last_plane_normal_v = self.last_plane_normal_v.normalize();

        let new_direction = self.direction * cos_t
            + (self.last_plane_normal_v.cross(self.direction)) * sin_t * cos_p
            + self.last_plane_normal_v * sin_t * sin_p;

        let new_plane_normal = self.last_plane_normal_v * cos_p
            - (self.last_plane_normal_v.cross(self.direction)) * sin_p;

        self.last_plane_normal_v = new_plane_normal.normalize();
        self.direction = new_direction.normalize();

        let cos_2p = cos_p * cos_p - sin_p * sin_p;
        let sin_2p = 2.0 * sin_p * cos_p;

        let r_psi = MuellerMatrix::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, cos_2p, sin_2p, 0.0),
            Vec4::new(0.0, -sin_2p, cos_2p, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        let scattered_stokes = mul_at_theta * (r_psi * self.stokes_vector);
        let intensity_ratio = scattered_stokes[0] / (mul_at_theta[0][0] * self.stokes_vector[0]);
        if !intensity_ratio.is_finite() || intensity_ratio <= 0.0 {
            return Err("invalid intensity ratio during polarized scattering".into());
        }
        self.w *= bul.mu_s / bul.mu_t * intensity_ratio;
        self.stokes_vector = scattered_stokes / scattered_stokes[0];
        self.scattered = true;
        Ok(())
    }
}
