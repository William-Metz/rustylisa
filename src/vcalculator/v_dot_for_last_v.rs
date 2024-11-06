use crate::vcalculator::vcalculator::VCalculator;
impl VCalculator {
    pub fn v_dot_for_last_v(&self) -> f64 {
        self.a0 / 3.0
            * self.v7
            * self.v2
            * (1.0
                + self.a2 * self.v2
                + self.a3 * self.v3
                + self.a4 * self.v4
                + self.a5 * self.v5
                + (self.a6 + self.b6 * self.v.ln()) * self.v6
                + self.a7 * self.v7)
    }
}
