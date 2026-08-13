use std::error::Error;

pub(crate) fn mie_pi_tau(mu: f64, nmax: f64) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let nmax = nmax as usize;
    if nmax <= 2 {
        return Err("mie_pi_tau: nmax should bigger than 2".into());
    }
    let mut p = vec![0.0_f64; nmax];
    let mut t = vec![0.0; nmax];
    p[0] = 1.0;
    p[1] = 3.0 * mu;
    t[0] = mu;
    t[1] = 3.0 * (2.0 * mu * mu - 1.0);

    let mut p_last2 = p[0];
    let mut p_last1 = p[1];

    for n in (2..nmax) {
        let p_curr =
            ((2.0 * n as f64 + 1.0) * (mu * p_last1) - (n as f64 + 1.0) * p_last2) / n as f64;
        let t_curr = (n as f64 + 1.0) * mu * p_curr - (n as f64 + 2.0) * p_last1;

        p[n] = p_curr;
        t[n] = t_curr;

        p_last2 = p_last1;
        p_last1 = p_curr;
    }

    Ok((p, t))
}
