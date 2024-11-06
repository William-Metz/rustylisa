use crate::vcalculator::vcalculator::VCalculator;
impl VCalculator{
    pub fn v_at_time(&mut self, tau: f64) -> f64 {
        let zeta = (5.0 / (256.0 * self.eta * (self.tau_c - tau))).powf(0.125);
        let zeta2 = zeta * zeta;
        let zeta3 = zeta2 * zeta;
        let zeta4 = zeta3 * zeta;
        let zeta5 = zeta4 * zeta;
        let zeta6 = zeta5 * zeta;
        let zeta7 = zeta6 * zeta;

        self.v = zeta
            * (1.0
                + self.c2 * zeta2
                + self.c3 * zeta3
                + self.c4 * zeta4
                + self.c5 * zeta5
                + (self.c6 - 1.5 * self.b6 * zeta.ln()) * zeta6
                + self.c7 * zeta7);
        self.v2 = self.v.powi(2);
        self.v3 = self.v.powi(3);
        self.v4 = self.v.powi(4);
        self.v5 = self.v.powi(5);
        self.v6 = self.v.powi(6);
        self.v7 = self.v.powi(7);
        self.v
    }

}
