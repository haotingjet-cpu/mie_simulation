use super::boundary::is_in_container;
use crate::photon::{Photon, path};
use crate::vector::Vec3;
use rand::RngExt;

impl Photon {
    fn get_mean_free_path(&self, rng: &mut impl RngExt, mu_t: f64) -> Vec3<f64> {
        let num: f64 = rng.random();
        let s = -num.ln() / mu_t;
        self.direction * s
    }

    pub(crate) fn move_photon(&mut self, rng: &mut impl RngExt, mu_t: f64) {
        let path = self.get_mean_free_path(rng, mu_t);
        if is_in_container(&self.start_location, &path) {
            self.start_location = self.start_location + path;
        } else {
        }
    }
}
