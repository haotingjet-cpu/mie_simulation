use crate::{simulat_const::*, vector::Vec3};
use rand::RngExt;

#[inline(always)]
pub(crate) fn is_in_container_fast(start: Vec3<f64>, path: Vec3<f64>) -> bool {
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

/*    let a = path[0] * path[0] + path[1] * path[1];
if a < 1e-12 {
    return (false,start);
}

let mins_b = -2.0 * (start[0] * path[0] + start[1] * path[1]);
let c = start[0] * start[0] + start[1] * start[1] - r * r;

let d = mins_b * mins_b - 4.0 * a * c;

if d < 0.0 {
    return (false,new);
}

let d_sqrt = d.sqrt();

let x1 = (mins_b + d_sqrt) / (2.0 * a);

let x = x1.clamp(0.0, 1.0);

(false,start + path * x)
*/
