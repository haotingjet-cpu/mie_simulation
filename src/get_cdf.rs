use crate::photon::MuellerMatrix;
use rand::RngExt;

const FREQUENCY: usize = crate::simulat_const::FREQUENCY as usize;
pub(crate) fn get_cfd_fn(mulmat: &[MuellerMatrix; FREQUENCY]) -> [f64; FREQUENCY] {
    let mut cdf = [0.0; FREQUENCY];
    let mut sum = 0.0;
    for i in 0..FREQUENCY {
        let s1 = mulmat[i][0][0]; // s11
        cdf[i] = s1;
        sum += s1;
    }

    if sum > 0.0 {
        let t = sum.recip();

        let mut running_sum = 0.0;
        for i in 0..FREQUENCY {
            running_sum += cdf[i] * t;
            cdf[i] = running_sum;
        }
    }
    cdf
}

pub(crate) fn cfd_to_theta(rng: &mut impl RngExt, cdf: &[f64; FREQUENCY]) -> usize {
    let mut zeta: f64 = 1.0 - rng.random::<f64>();
    for i in 0..FREQUENCY {
        if zeta <= 0.0 {
            return i;
        }

        zeta -= cdf[i];
    }
    panic!("cfd 輸入或設定錯誤")
}
