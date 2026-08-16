use super::boundary::path_end_is_in_container;
use crate::photon::{Photon, boundary};
use crate::simulat_const::FREQUENCY;
use crate::vector::{Dot, Norm, Vec3};
use rand::RngExt;

impl Photon {
    fn get_mean_free_path(&self, rng: &mut impl RngExt, mu_t: f64) -> Vec3<f64> {
        let num: f64 = rng.random();
        let s = -num.ln() / mu_t;
        self.direction * s
    }

    pub(crate) fn move_a_path(&mut self, theta_log: Vec<f64>, rng: &mut impl RngExt, mu_t: f64) {
        let mut path = self.get_mean_free_path(rng, mu_t);
        loop {
            if path_end_is_in_container(&self.start_location, &path) {
                self.start_location = self.start_location + path;
                return;
            } else {
                boundary::caculate_intersection(&mut self.start_location, &mut path);
                let theta_i_cos =
                    self.start_location.dot(&path) / (self.start_location.norm() * path.norm());
                let normal = boundary::get_normal(self.start_location);

                if boundary::is_reflection(rng, theta_i_cos) {
                    path = path * (-1.0) + normal * 2.0 * (path.dot(&normal));
                } else {
                    todo!("這邊要處理折射還有紀錄，但還沒寫完");
                    return;
                }
            }
        }
    }
}
