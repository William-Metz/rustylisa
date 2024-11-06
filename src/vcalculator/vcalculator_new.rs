use crate::vcalculator::vcalculator::VCalculator;
use std::f64::consts::PI;

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
}
