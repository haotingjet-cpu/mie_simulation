pub(crate) mod boundary;
pub(crate) mod collision;
pub(crate) mod path;
use vector::{Matrix4, Vec3, Vec4};

use crate::simulat_const::{LIMIT_W_LINE, M};
use num_complex::Complex;
/// # StokesVector
/// * i: f64 -> Total intensity
/// * g: f64 -> Degree of linear polarization at 0° or 90°
/// * u: f64 -> Degree of linear polarization at +45° or -45°
/// * v: f64 -> Degree of circular polarization
pub(crate) type StokesVector = Vec4<f64>;

fn get_i(v: &StokesVector) -> f64 {
    v[0]
}
pub(crate) type MuellerMatrix = Matrix4<f64>;

pub(crate) fn get_mueller_matrix_with_s1s2(s1: Complex<f64>, s2: Complex<f64>) -> MuellerMatrix {
    let s11 = (s1.norm_sqr() + s2.norm_sqr()) / 2.0;
    let s12 = (s2.norm_sqr() - s1.norm_sqr()) / 2.0;
    let s33 = (s1 * (s2.conj())).re;
    let s34 = (s2 * (s1.conj())).im;

    MuellerMatrix::new(
        Vec4::new(s11, s12, 0.0, 0.0),
        Vec4::new(s12, s11, 0.0, 0.0),
        Vec4::new(0.0, 0.0, s33, s34),
        Vec4::new(0.0, 0.0, -s34, s33),
    )
}

pub(crate) type DirecVec = Vec3<f64>;

#[repr(align(32))]
pub(crate) struct Photon {
    pub(crate) stokes_vector: StokesVector,
    pub(crate) direction: DirecVec,
    pub(crate) start_location: DirecVec,
    pub(crate) last_plane_normal_v: DirecVec,
    pub(crate) w: f64,
    pub(crate) scattered: bool,
}

impl Photon {
    pub(crate) fn alive_check(&mut self, rng: &mut impl rand::RngExt) -> bool {
        // 判斷 W 是否低於死亡線
        if self.w <= LIMIT_W_LINE {
            // 進入輪盤賭
            let zeta: f64 = rng.random();
            if zeta < M.recip() {
                self.w = self.w * M;
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}
