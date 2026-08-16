use crate::simulat_const::*;
use crate::vector::Norm;
use crate::vector::Vec3;
use rand::RngExt;

#[inline(always)]
pub(crate) fn path_end_is_in_container(start: &Vec3<f64>, path: &Vec3<f64>) -> bool {
    let new = start + path;
    let r = 1.0;

    if new[0] * new[0] + new[1] * new[1] <= r * r {
        return true;
    }
    false
}

#[inline]
fn is_tir(theta_i_cos: f64) -> bool {
    let sin_i_2 = 1.0 - theta_i_cos * theta_i_cos;
    sin_i_2 > THETA_C_SIN_2
}

#[inline]
fn get_fresnel_r(theta_i_cos: f64) -> f64 {
    if is_tir(theta_i_cos) {
        return 1.0;
    }

    let theta_t_cos = (1.0 - THETA_C_SIN_2_REC * (1.0 - theta_i_cos * theta_i_cos)).sqrt();

    let rs_sqrt =
        (theta_i_cos - THETA_C_SIN * theta_t_cos) / (theta_i_cos + THETA_C_SIN * theta_t_cos);
    let rs = rs_sqrt * rs_sqrt;

    let rp_sqrt =
        (THETA_C_SIN * theta_i_cos - theta_t_cos) / (THETA_C_SIN * theta_i_cos + theta_t_cos);

    let rp = rp_sqrt * rp_sqrt;

    let r = (rs + rp) * 0.5;

    r
}

#[inline]
pub(crate) fn is_reflection(rng: &mut impl RngExt, theta_i_cos: f64) -> bool {
    let r = get_fresnel_r(theta_i_cos);
    let zeta: f64 = rng.random();
    if zeta < r { true } else { false }
}

pub(crate) fn caculate_intersection(start: &mut Vec3<f64>, path: &mut Vec3<f64>) {
    let a = path[0] * path[0] + path[1] * path[1];
    if a < 1e-12 {
        return;
    }

    let mins_b = -2.0 * (start[0] * path[0] + start[1] * path[1]);
    let c = start[0] * start[0] + start[1] * start[1] - RADIUS * RADIUS;

    let d = mins_b * mins_b - 4.0 * a * c;

    let d_sqrt = d.max(0.0).sqrt();

    let x1 = (mins_b + d_sqrt) / (2.0 * a);

    let x = x1.clamp(0.0, 1.0);

    *start = *start + *path * x;
    *path = *path * (1.0 - x);
}

pub(crate) fn get_normal(location: Vec3<f64>) -> Vec3<f64> {
    location / location.norm()
}
