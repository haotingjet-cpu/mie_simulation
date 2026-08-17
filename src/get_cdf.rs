use crate::photon::MuellerMatrix;
use rand::RngExt;

const FREQUENCY: usize = crate::simulat_const::FREQUENCY as usize;

pub(crate) fn get_cdf_fn(mulmat: &[MuellerMatrix; FREQUENCY]) -> [f64; FREQUENCY] {
    let mut cdf = [0.0; FREQUENCY];
    let mut sum = 0.0;

    for i in 0..FREQUENCY {
        let s1 = mulmat[i][0][0]; // s11
        sum += s1;
        cdf[i] = sum;
    }

    if sum <= 0.0 {
        let uniform_prob = 1.0 / (FREQUENCY as f64);
        for i in 0..FREQUENCY {
            cdf[i] = (i + 1) as f64 * uniform_prob;
        }
        return cdf;
    }

    let t = sum.recip();
    for i in 0..FREQUENCY {
        cdf[i] *= t;
    }

    cdf[FREQUENCY - 1] = 1.0;
    cdf
}

pub(crate) fn cdf_to_theta(rng: &mut impl RngExt, cdf: &[f64; FREQUENCY]) -> usize {
    let zeta: f64 = 1.0 - rng.random::<f64>();

    match cdf.binary_search_by(|probe| probe.partial_cmp(&zeta).unwrap()) {
        Ok(index) => index,
        Err(index) => {
            if index >= FREQUENCY {
                FREQUENCY - 1
            } else {
                index
            }
        }
    }
}
