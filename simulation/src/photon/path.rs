use super::boundary::path_end_is_in_container;
use crate::photon::{self, Photon, boundary};
use crate::simulat_const::{FREQUENCY, FREQUENCY_IDX, RADIUS_NM};
use rand::RngExt;
use std::f64::consts::PI;
use vector::{Dot, Norm, Vec3};

impl Photon {
    fn get_mean_free_path(&self, rng: &mut impl RngExt, mu_t: f64) -> Vec3<f64> {
        let num: f64 = rng.random();
        let s = -(1.0 - num).ln() / mu_t;
        self.direction * s
    }

    pub(crate) fn move_a_path(
        &mut self,
        theta_log: &mut [[f64; FREQUENCY_IDX]; 2],
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
                let incoming_direction = self.direction.normalize();
                let theta_i_cos = incoming_direction.dot(normal).abs().clamp(0.0, 1.0);
                self.start_location = self.start_location + normal * (RADIUS_NM * 1e-9);

                if boundary::is_reflection(rng, theta_i_cos) {
                    path = path - normal * 2.0 * (path.dot(normal));
                    self.direction =
                        incoming_direction - normal * 2.0 * incoming_direction.dot(normal);

                    let n = self.last_plane_normal_v;
                    self.last_plane_normal_v =
                        (n - self.direction * n.dot(self.direction)).normalize();
                } else {
                    let path_2 = boundary::get_refraction_vec(incoming_direction, normal);
                    let theta = path_2[0].clamp(-1.0, 1.0).acos();
                    let theta =
                        ((theta / PI * FREQUENCY as f64).floor() as usize).min(FREQUENCY_IDX - 1);
                    theta_log[self.scattered as usize][theta] +=
                        self.w * photon::get_i(&self.stokes_vector);
                    return None;
                }
            }
        }
    }
}
