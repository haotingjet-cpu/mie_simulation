pub(crate) mod boundary;
pub(crate) mod collision;
pub(crate) mod path;

use crate::vector::{Matrix4, Vec3, Vec4};
use num_complex::Complex;
/// # StokesVector
/// * i: f64 -> Total intensity
/// * g: f64 -> Degree of linear polarization at 0° or 90°
/// * u: f64 -> Degree of linear polarization at +45° or -45°
/// * v: f64 -> Degree of circular polarization
pub(crate) type StokesVector = Vec4<f64>;
impl StokesVector {
    fn new_with_stocks(i: f64, q: f64, u: f64, v: f64) -> Self {
        Self::new(i, q, u, v)
    }

    fn get_i(&self) -> f64 {
        self[0]
    }
}

pub(crate) type MuellerMatrix = Matrix4<f64>;
impl MuellerMatrix {
    pub(crate) fn get_mueller_matrix_with_s1s2(s1: Complex<f64>, s2: Complex<f64>) -> Self {
        let s11 = (s1.norm_sqr() + s2.norm_sqr()) / 2.0;
        let s12 = (s2.norm_sqr() - s1.norm_sqr()) / 2.0;
        let s33 = (s1 * (s2.conj())).re;
        let s34 = (s2 * (s1.conj())).im;

        Self::new(
            Vec4::new(s11, s12, 0.0, 0.0),
            Vec4::new(s12, s11, 0.0, 0.0),
            Vec4::new(0.0, 0.0, s33, s34),
            Vec4::new(0.0, 0.0, -s34, s33),
        )
    }
}

pub(crate) type direc_vec = Vec3<f64>;

#[repr(align(32))]
pub(crate) struct Photon {
    pub(crate) status: StokesVector,
    pub(crate) direction: direc_vec,
    pub(crate) start_location: direc_vec,
    pub(crate) last_plane_normal_v: direc_vec,
    pub(crate) w: f64,
}
