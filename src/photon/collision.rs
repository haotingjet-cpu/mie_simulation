use crate::mie::get_mu::BulkCoefficients;

impl super::Photon {
    pub(crate) fn collision_event(&mut self, bul: &BulkCoefficients) {
        self.w = self.w * bul.mu_s / bul.mu_t;
    }
}
