use crate::vcalculator::vcalculator::VCalculator;
impl VCalculator {
    pub fn psi_orb_for_last_v(&self) -> f64 {
        let log_viv0 = (self.v / self.v0).ln();

        // Debug each sub-calculation
        let part1 = self.psi_c;
        let part2 = self.p0 / self.v5;
        let part3 = self.p2 * self.v2;
        let part4 = self.p3 * self.v3;
        let part5 = self.p4 * self.v4;
        let part6 = self.p5 * self.v5 * log_viv0;
        let part7 = (self.p6 + 15.0 * self.b6 * log_viv0) * self.v6;
        let part8 = self.p7 * self.v7;

        part1 + part2 * (1.0 + part3 + part4 + part5 + part6 + part7 + part8)
    }
}
