use crate::photon::Photon;
use crate::vector::Vec3;
use rand::RngExt;

impl Photon {
    fn get_mean_free_path(&self, rng: &mut impl RngExt, mu_t: f64) -> Vec3<f64> {
        let num: f64 = rng.random();
        let s = -num.ln() / mu_t;
        self.direction * s
    }

    pub(crate) fn move_photon(&mut self, mu_t: f64) {}
}
