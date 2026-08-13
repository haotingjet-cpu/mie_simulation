mod mie;
mod photon;
mod vector;
use num_complex::Complex;

fn main() {
    let wavelength = 632.8;
    let polyethylene = mie::Particle {
        m: Complex::new(1.4998, 0.0),
        diameter: 2.0,
        molarity: 0.1,
    };
}
