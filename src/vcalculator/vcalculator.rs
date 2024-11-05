// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/vcalculator/vcalculator.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use std::f64::consts::PI;
#[derive(Clone, PartialEq, Debug)]
pub struct VCalculator {
    a0: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    a7: f64,
    b6: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    p0: f64,
    p2: f64,
    p3: f64,
    p4: f64,
    p5: f64,
    p6: f64,
    p7: f64,
    v: f64,
    v0: f64,
    v2: f64,
    v3: f64,
    v4: f64,
    v5: f64,
    v6: f64,
    v7: f64,
    beta3: f64,
    beta5: f64,
    beta6: f64,
    beta7: f64,
    zeta: f64,
    zeta0: f64,
    eta: f64,
    tau_c: f64,
    psi_c: f64,
}

impl VCalculator {
    pub fn new(the_tau_c: f64, delta: f64, chi1l: f64, chi2l: f64) -> Self {
        let eta = (1.0 - delta * delta) * 0.25;
        let onep_delta = 1.0 + delta;
        let onem_delta = 1.0 - delta;
        let tau_c = the_tau_c;
        let b6 = -1712.0 / 315.0;
        let gamma_e: f64 = 0.577215664901533;
        let beta3 = (113.0 / 48.0 * onep_delta * onep_delta + 25.0 / 4.0 * eta) * chi1l
            + (113.0 / 48.0 * onem_delta * onem_delta + 25.0 / 4.0 * eta) * chi2l;

        let beta5 = ((31319.0 / 4032.0 - 1159.0 / 96.0 * eta) * onep_delta * onep_delta
            + 809.0 / 84.0 * eta
            - 281.0 / 8.0 * eta * eta)
            * chi1l
            + ((31319.0 / 4032.0 - 1159.0 / 96.0 * eta) * onem_delta * onem_delta
                + 809.0 / 84.0 * eta
                - 281.0 / 8.0 * eta * eta)
                * chi2l;

        let beta6 = PI * (75.0 / 8.0 * onep_delta * onep_delta + 151.0 / 6.0 * eta) * chi1l * PI
            + (75.0 / 8.0 * onem_delta * onem_delta + 151.0 / 6.0 * eta) * chi2l * PI;
        let beta7 = ((130325.0 / 3024.0 - 796069.0 / 8064.0 * eta + 100019.0 / 3456.0 * eta * eta)
            * onep_delta
            * onep_delta
            + 1195759.0 / 18144.0 * eta
            - 257023.0 / 1008.0 * eta * eta
            + 2903.0 / 32.0 * eta * eta * eta)
            * chi1l
            + ((130325.0 / 3024.0 - 796069.0 / 8064.0 * eta + 100019.0 / 3456.0 * eta * eta)
                * onem_delta
                * onem_delta
                + 1195759.0 / 18144.0 * eta
                - 257023.0 / 1008.0 * eta * eta
                + 2903.0 / 32.0 * eta * eta * eta)
                * chi2l;

        let a0 = 96.0 / 5.0 * eta;
        let a2 = -743.0 / 336.0 - 11.0 / 4.0 * eta;
        let a3 = 4.0 * PI - beta3;
        let a4 = 34103.0 / 18144.0 + 13661.0 / 2016.0 * eta + 59.0 / 18.0 * eta * eta;
        let a5 = (-4159.0 / 672.0 + 189.0 / 8.0 * eta) * PI - beta5;
        let a6 = 16447322263.0 / 139708800.0 + 16.0 / 3.0 * PI * PI
            - 856.0 / 105.0 * 16.0_f64.ln()
            - 1712.0 / 105.0 * gamma_e
            - beta6
            + (451.0 / 48.0 * PI * PI - 56198689.0 / 217728.0) * eta
            + 541.0 / 896.0 * eta * eta
            - 5605.0 / 2592.0 * eta * eta * eta;
        let a7 = -4415.0 / 4032.0 * PI
            + 358675.0 / 6048.0 * PI * eta
            + 91495.0 / 1512.0 * PI * eta * eta
            - beta7;

        let c2 = -a2 / 6.0;
        let c3 = -a3 / 5.0;
        let c4 = -a4 / 4.0 + 5.0 / 24.0 * a2 * a2;
        let c5 = -a5 / 3.0 + 3.0 / 5.0 * a2 * a3;
        let c6 = -a6 / 2.0 - 3.0 / 4.0 * b6 + 23.0 / 24.0 * a4 * a2 + 12.0 / 25.0 * a3 * a3
            - 67.0 / 144.0 * a2 * a2 * a2;
        let c7 = -a7 + 2.0 * a5 * a2 + 2.0 * a4 * a3 - 3.0 * a3 * a2 * a2;

        let p0 = -3.0 / (5.0 * a0);
        let p2 = -5.0 / 3.0 * a2;
        let p3 = -5.0 / 2.0 * a3;
        let p4 = -5.0 * a4 + 5.0 * a2 * a2;
        let p5 = 5.0 * a5 - 10.0 * a3 * a2;
        let p6 = 5.0 * a6 - 15.0 * b6 - 10.0 * a4 * a2 - 5.0 * a3 * a3 - 5.0 * a2 * a2 * a2;
        let p7 = 5.0 / 2.0 * a7 - 5.0 * a5 * a2 - 5.0 * a4 * a3 + 15.0 / 2.0 * a3 * a2 * a2;

        let zeta0 = (5.0 / (256.0 * eta * tau_c)).powf(0.125);
        let zeta2 = zeta0 * zeta0;
        let zeta3 = zeta2 * zeta0;
        let zeta4 = zeta3 * zeta0;
        let zeta5 = zeta4 * zeta0;
        let zeta6 = zeta5 * zeta0;
        let zeta7 = zeta6 * zeta0;

        let v0 = zeta0
            * (1.0
                + c2 * zeta2
                + c3 * zeta3
                + c4 * zeta4
                + c5 * zeta5
                + (c6 - 3.0 / 2.0 * b6 * (zeta0).ln()) * zeta6
                + c7 * zeta7);

        let v = v0;
        let v2 = v * v;
        let v3 = v2 * v;
        let v4 = v3 * v;
        let v5 = v4 * v;
        let v6 = v5 * v;
        let v7 = v6 * v;

        let psi_c = -p0 / v5 * (1.0 + p2 * v2 + p3 * v3 + p4 * v4 + p6 * v6 + p7 * v7);

        VCalculator {
            a0,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            b6,
            c2,
            c3,
            c4,
            c5,
            c6,
            c7,
            p0,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            v,
            v0,
            v2,
            v3,
            v4,
            v5,
            v6,
            v7,
            beta3,
            beta5,
            beta6,
            beta7,
            zeta: 0.0,
            zeta0,
            eta,
            tau_c,
            psi_c,
        }
    }

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
