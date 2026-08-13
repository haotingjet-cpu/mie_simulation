use num_complex::Complex;

#[derive(Debug)]
pub struct CrossSections {
    pub cext: f64,
    pub csca: f64,
    pub cabs: f64,
    pub cpr: f64,
    pub g: f64,
    pub cback: f64,
    pub cratio: f64,
}

#[derive(Debug)]
pub struct Efficiencies {
    pub qext: f64,
    pub qsca: f64,
    pub qabs: f64,
    pub qpr: f64,
    pub g: f64,
    pub qback: f64,
    pub qratio: f64,
}

pub enum IsCrossSections {
    CrossSection(CrossSections),
    Efficiencie(Efficiencies),
}

impl Efficiencies {
    pub(crate) fn to_cross_section(self, css: f64) -> CrossSections {
        let cross = CrossSections {
            cext: css * self.qext,
            csca: css * self.qsca,
            cabs: css * self.qabs,
            cpr: css * self.qpr,
            g: self.g,
            cback: css * self.qback,
            cratio: self.qratio,
        };

        cross
    }
}
