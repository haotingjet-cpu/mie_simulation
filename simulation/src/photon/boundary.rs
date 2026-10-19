use std::f64::consts::PI;

use crate::{
    photon::{self, Photon},
    simulat_const::*,
};
use rand::RngExt;
use vector::{Dot, Norm, Vec3};

/// * ### path_end_is_in_container : 判斷路徑終點是否仍在容器中
/// * ### is_tir :  判斷是否為全反射
/// * ### get_fresnel_r : 得出 fresnel 反射係數, 如果 is_tir 為 true (全反射)，反射率為 1
/// * ### is_reflection : 藉反射係數與隨機數隨機是否反射
/// * ### caculate_intersection : 確保路徑不完全在內部後計算, 會直接對位置和剩餘路徑做更新，所以沒回傳
/// * ### get_normal : 得出單位向量
/// * ### get_refraction_vec : 得出折射後向量, 回傳必須是單位向量
pub(crate) trait Container {
    fn path_end_is_in_container(start: &Vec3<f64>, path: &Vec3<f64>) -> bool;

    fn is_tir(theta_i_cos: f64) -> bool;

    fn get_fresnel_r(theta_i_cos: f64) -> f64;

    fn is_reflection(rng: &mut impl RngExt, theta_i_cos: f64) -> bool;

    fn caculate_intersection(start: &mut Vec3<f64>, path: &mut Vec3<f64>);

    fn get_normal(location: Vec3<f64>) -> Vec3<f64>;

    fn get_refraction_vec(path: Vec3<f64>, normal: Vec3<f64>) -> Vec3<f64>;

    fn log(
        theta_log: &mut [[f64; FREQUENCY_IDX]; 2],
        v: Vec3<f64>,
        photon: &mut Photon,
        _location: Option<Vec3<f64>>,
    );
}

/// # 外殼為無限延伸圓柱體
pub(crate) struct Cylinder;

impl Container for Cylinder {
    #[inline(always)]
    fn path_end_is_in_container(start: &Vec3<f64>, path: &Vec3<f64>) -> bool {
        let new = start + path;

        if new[0] * new[0] + new[1] * new[1] <= RADIUS_NM * RADIUS_NM {
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
        if Self::is_tir(theta_i_cos) {
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
    fn is_reflection(rng: &mut impl RngExt, theta_i_cos: f64) -> bool {
        let r = Self::get_fresnel_r(theta_i_cos);
        let zeta: f64 = rng.random();
        zeta < r
    }

    fn caculate_intersection(start: &mut Vec3<f64>, path: &mut Vec3<f64>) {
        let a = path[0] * path[0] + path[1] * path[1];

        let mins_b = -2.0 * (start[0] * path[0] + start[1] * path[1]);
        let c = start[0] * start[0] + start[1] * start[1] - RADIUS_NM * RADIUS_NM;

        let d = mins_b * mins_b - 4.0 * a * c;

        let d_sqrt = d.max(0.0).sqrt();

        let x1 = (mins_b + d_sqrt) / (2.0 * a);
        let x2 = (mins_b - d_sqrt) / (2.0 * a);

        let eps = 1e-9;
        let x = if x1 > eps {
            x1
        } else if x2 > eps {
            x2
        } else {
            0.0
        };
        let x = x.clamp(0.0, 1.0);

        *start = *start + *path * x;
        *path = *path * (1.0 - x);
    }

    fn get_normal(location: Vec3<f64>) -> Vec3<f64> {
        let r = (location[0] * location[0] + location[1] * location[1]).sqrt();
        Vec3::new(-location[0] / r, -location[1] / r, 0.0)
    }

    fn get_refraction_vec(path: Vec3<f64>, normal: Vec3<f64>) -> Vec3<f64> {
        let path = path / path.norm();

        let r = THETA_C_SIN_REC;
        let r_sq = THETA_C_SIN_2_REC;

        let dot = path.dot(normal);

        let (n, cos_theta) = if dot < 0.0 {
            (normal, -dot)
        } else {
            (normal * -1.0, dot)
        };

        let k = 1.0 - r_sq * (1.0 - cos_theta * cos_theta);
        let path_2 = path * r + n * (r * cos_theta - k.sqrt());

        // 回傳必然是單位向量
        path_2
    }

    fn log(
        theta_log: &mut [[f64; FREQUENCY_IDX]; 2],
        v: Vec3<f64>,
        photon: &mut Photon,
        _location: Option<Vec3<f64>>,
    ) {
        let theta = v[0].clamp(-1.0, 1.0).acos();
        let theta = ((theta / PI * FREQUENCY as f64).floor() as usize).min(FREQUENCY_IDX - 1);
        theta_log[photon.scattered as usize][theta] +=
            photon.w * photon::get_i(&photon.stokes_vector);
    }
}

/// 長方體容器
/// 長和寬皆為為 ` 2.0 * crate::simulat_const::RADIUS_NM`
/// 高度無限長
pub(crate) struct Cuboid;
