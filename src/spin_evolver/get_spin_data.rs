use crate::spin_evolver::spin_evolver::SpinEvolverClass ;
use std::f64::consts::PI;
use crate::spin_data::spin_data::SpinData;

impl SpinEvolverClass {


pub fn get_spin_data_at_time(&mut self, tau: f64) -> Option<SpinData> {

    if self.test_case.chi1 == 0.0 && self.test_case.chi2 == 0.0 {

        // If we have no spins
        let mut data = SpinData::new();
        data.iota = 1.0;
        data.alpha = PI;
        data.chi_ax = 0.0;
        data.chi_ay = 0.0;
        data.chi_az = 0.0;
        data.chi_sx = 0.0;
        data.chi_sy = 0.0;
        data.chi_sz = 0.0;
        let v_for_tau =self.v_calc.v_at_time(tau);
        data.v = v_for_tau;
        let tmp = self.v_calc.psi_orb_for_last_v();
        data.psi = self.test_case.lambda0 + tmp - (6.0 * v_for_tau.powi(3) * (v_for_tau / self.v_0).ln());
        Some(data)
    } else {

        // If we have at least one nonzero spin, then we need to evolve
        // Cycle through steps until we get beyond the requested time
        while tau > self.tau_n {
            if !self.do_step_succeeded() {
                return None;
            }
        }

        // Interpolate data to pass on to the rest of the program
        let f_n = (tau - self.tau_p) / self.delta_tau_hp;
        let f_p = 1.0 - f_n;

        let mut data = SpinData::new();
        data.iota = f_n * self.iota_n + f_p * self.iota_p;
        data.alpha = f_n * self.alpha_n + f_p * self.alpha_p;
        data.chi_ax = 0.5 * (f_n * (self.chi1x_n- self.chi2x_n) + f_p * (self.chi1x_p- self.chi2x_p));
        data.chi_ay = 0.5 * (f_n * (self.chi1y_n- self.chi2y_n) + f_p * (self.chi1y_p - self.chi2y_p));
        data.chi_az = 0.5 * (f_n * (self.chi1z_n - self.chi2z_n) + f_p * (self.chi1z_p - self.chi2z_p));
        data.chi_sx = 0.5 * (f_n * (self.chi1x_n + self.chi2x_n) + f_p * (self.chi1x_p + self.chi2x_p));
        data.chi_sy = 0.5 * (f_n * (self.chi1y_n + self.chi2y_n) + f_p * (self.chi1y_p + self.chi2y_p));
        data.chi_sz = 0.5 * (f_n * (self.chi1z_n + self.chi2z_n) + f_p * (self.chi1z_p + self.chi2z_p));

        // Calculate phase at the current time
        let v_for_tau =self.v_calc.v_at_time(tau);
        data.v = v_for_tau;
        data.psi = self.test_case.lambda0 +self.v_calc.psi_orb_for_last_v() + f_n * self.psi_pr_n + f_p * self.psi_pr_p- 6.0 * v_for_tau.powi(3) * (v_for_tau / self.v_0).ln();
        Some(data)
    }

}

}
