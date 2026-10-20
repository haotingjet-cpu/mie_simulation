use crate::{mie, photon};
use num_complex::Complex;
use vector::{Vec3, Vec4};

// ===== 模擬可修改填參數 ===============================
pub(crate) const POLYETHYLENE: mie::Particle = mie::Particle {
    m: Complex::new(1.4998, 0.0),
    diameter_nm: 1200.0,
    molarity_mol_nm3: 1.6605e-41, // mol/nm^3
};

pub(crate) const PHOTON: photon::Photon = photon::Photon {
    stokes_vector: Vec4::new(1.0, 0.0, 0.0, 0.0),
    direction: Vec3::new(1.0, 0.0, 0.0),
    start_location: Vec3::new(0.0, 0.0, 0.0),
    last_plane_normal_v: Vec3::new(0.0, 0.0, 1.0),
    w: 1.0,
    scattered: false,
};

pub(crate) const RADIUS_NM: f64 = 5e6;
pub(crate) const N_MEDIUM: f64 = 1.332;
pub(crate) const N_BOUNDARY: f64 = 1.45;
pub(crate) const WAVELENGH_NM: f64 = 632.8;

pub(crate) const LIMIT_W_LINE: f64 = 1e-3;
pub(crate) const M: f64 = 6.0;

pub(crate) const PHOTON_CONST: usize = 5000000000000;
pub(crate) const FREQUENCY: i64 = 180 * 5;
pub(crate) const CUT: usize = 100000;
pub(crate) const WORKER_COUNT: usize = 15;

pub(crate) const AVOGADRO: f64 = 6.02214076e23;
// ===== 加速計算用參數，由推倒來不可改 ===================
pub(crate) const THETA_C_SIN: f64 = N_BOUNDARY / N_MEDIUM;
pub(crate) const THETA_C_SIN_REC: f64 = THETA_C_SIN.recip();
pub(crate) const THETA_C_SIN_2: f64 = THETA_C_SIN * THETA_C_SIN;
pub(crate) const THETA_C_SIN_2_REC: f64 = THETA_C_SIN_2.recip();
pub(crate) const FREQUENCY_IDX: usize = FREQUENCY as usize;

// ===== 編譯期參數特性確保 =============================
const _: () = {
    if PHOTON_CONST == 0 {
        panic!()
    }

    if FREQUENCY % 180 != 0 {
        panic!();
    }

    if M.round() != M {
        panic!()
    }

    if CUT == 0 {
        panic!()
    }
};
