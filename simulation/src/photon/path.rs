use super::boundary::path_end_is_in_container;
use crate::photon::{self, Photon, boundary};
use crate::simulat_const::{FREQUENCY, FREQUENCY_IDX};
use rand::RngExt;
use vector::{Dot, Norm, Vec3};

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
                self.start_location = self.start_location * 0.999;
                let normal = boundary::get_normal(self.start_location);
                let theta_i_cos = path.dot(normal).abs() / path.norm();

                if boundary::is_reflection(rng, theta_i_cos) {
                    path = path - normal * 2.0 * (path.dot(normal));
                    self.direction = path.normalize();

                    let n = self.last_plane_normal_v;
                    self.last_plane_normal_v =
                        (n - self.direction * n.dot(self.direction)).normalize();
                } else {
                    let path_2 = boundary::get_refraction_vec(path, normal);
                    let theta = Vec3::new(path_2[0], path_2[1], 0.0)
                        .dot(Vec3::new(1.0, path_2[1], 0.0))
                        .acos();
                    let theta = ((theta / crate::mie::PI * (FREQUENCY as f64)).round() as usize)
                        .min(FREQUENCY_IDX - 1);
                    theta_log[theta] += photon::get_i(&self.stokes_vector);
                    return None;
                }
            }
        }
    }
}
