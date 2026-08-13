use crate::mie::{MieCoefficients, PI, Particle, little_func, mie_ab, mie_pi_tau};
use num_complex::Complex;
use std::error::Error;

pub(crate) fn mies1s2_one_theta(
    m: Complex<f64>,
    x: f64,
    mu: f64,
    mie_coef: Option<&MieCoefficients>,
) -> Result<(Complex<f64>, Complex<f64>), Box<dyn Error>> {
    let nmax = (2.0 + x + 4.0 * x.cbrt()).round().max(3.0);
    let mie_coef = match mie_coef {
        Some(coef) => coef,
        None => &mie_ab::auto_mie_ab(m, x)?,
    };
    let (an, bn) = (&mie_coef.an, &mie_coef.bn);

    let (pin, taun) = mie_pi_tau::mie_pi_tau(mu, nmax)?;

    if an.len() != bn.len() || pin.len() != taun.len() {
        return Err("mies1s2_one_theta: an/bn 或 pin/taun 長度不對稱".into());
    }
    if pin.len() < an.len() {
        return Err(
            "mies1s2_one_theta: angular arrays (pin) 長度小於 Mie coefficients (an)".into(),
        );
    }

    let mut s1_sum = Complex::new(0.0, 0.0);
    let mut s2_sum = Complex::new(0.0, 0.0);

    let len = an.len();

    assert!(len <= bn.len());
    assert!(len <= pin.len());
    assert!(len <= taun.len());

    for i in 0..len {
        let n_f = (i + 1) as f64;
        let n2 = (2.0 * n_f + 1.0) / (n_f * (n_f + 1.0));

        let a = an[i];
        let b = bn[i];
        let pi = pin[i];
        let tau = taun[i];

        s1_sum += (a * pi + b * tau) * n2;
        s2_sum += (a * tau + b * pi) * n2;
    }

    Ok((s1_sum, s2_sum))
}
