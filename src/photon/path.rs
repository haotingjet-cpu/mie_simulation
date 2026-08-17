use super::boundary::path_end_is_in_container;
use crate::photon::{Photon, boundary};
use crate::simulat_const::FREQUENCY;
use crate::vector::{Dot, Norm, Vec3};
use rand::RngExt;

impl Photon {
    fn get_mean_free_path(&self, rng: &mut impl RngExt, mu_t: f64) -> Vec3<f64> {
        let num: f64 = rng.random();
        let s = -(1.0 - num).ln() / mu_t;
        self.direction * s
    }

    pub(crate) fn move_a_path(
        &mut self,
        theta_log: &mut [f64; FREQUENCY as usize],
        rng: &mut impl RngExt,
        mu_t: f64,
    ) -> Option<&mut Self> {
        let mut path = self.get_mean_free_path(rng, mu_t);
        loop {
            if path_end_is_in_container(&self.start_location, &path) {
                self.start_location = self.start_location + path;
                return Some(self);
            } else {
                boundary::caculate_intersection(&mut self.start_location, &mut path);
                let normal = boundary::get_normal(self.start_location);
                let theta_i_cos = -path.dot(&normal).abs() / path.norm();

                if boundary::is_reflection(rng, theta_i_cos) {
                    path = path - normal * 2.0 * (path.dot(&normal));
                } else {
                    let path_2 = boundary::get_refraction_vec(path, normal);
                    let theta = (path_2[0] * normal[0] + path_2[1] + normal[1])
                        .acos()
                        .to_degrees()
                        .round() as usize;
                    theta_log[theta] += self.status.get_i();
                    return None;
                }
            }
        }
    }
}
