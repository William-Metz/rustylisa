use crate::spin_evolver::spin_evolver::SpinEvolverClass ;
use std::f64::consts::PI;
use libm::atan2;

impl SpinEvolverClass {
    pub fn do_step_succeeded(&mut self) -> bool {
        // Check whether the step has been adjusted
        if self.delta_tau_hp >self.delta_tau_hf{
            self.adjust_the_past();  // if so, adjust the past values
        }
        // Calculate the current orbital angular momentum magnitude
        let v1: f64 = self.v_calc.v_at_time(self.tau_n);
        if v1 > 0.5 {
            return false; // if our speed is getting too high, bail out
        }
        let v2: f64 = v1*v1;
        let v3: f64 = v2*v1;
        let v4: f64 = v3*v1;
        let v5: f64 = v4*v1;
        let l_n: f64 = self.eta/v1*(1.0 + self.l_2*v2 + self.l_3*v3 + self.l_4*v4);

        // let components of: f64 spin rate of change;
        let omega_1: f64 = v5*(self.c_10 + self.c_12*v2 + self.c_14*v4)/l_n;
        let chi1x_dot_n: f64 = omega_1*(self.ell_y_n*self.chi1z_n - self.ell_z_n*self.chi1y_n);
        let chi1y_dot_n: f64 = omega_1*(self.ell_z_n*self.chi1x_n - self.ell_x_n*self.chi1z_n);
        let chi1z_dot_n: f64 = omega_1*(self.ell_x_n*self.chi1y_n - self.ell_y_p*self.chi1x_n);
        let omega_2: f64 = v5*(self.c_20 + self.c_22*v2 + self.c_24*v4)/l_n;
        let chi2x_dot_n: f64 = omega_2*(self.ell_y_n*self.chi2z_n - self.ell_z_n*self.chi2y_n);
        let chi2y_dot_n: f64 = omega_2*(self.ell_z_n*self.chi2x_n - self.ell_x_n*self.chi2z_n);
        let chi2z_dot_n: f64 = omega_2*(self.ell_x_n*self.chi2y_n - self.ell_y_n*self.chi2x_n);

        // Evolve the spins using an leapfrog step
        let two_delta_tau: f64 = 2.0*self.delta_tau_hf;
        let chi1x_f: f64 = self.chi1x_p + two_delta_tau*chi1x_dot_n;
        let chi1y_f: f64 = self.chi1y_p + two_delta_tau*chi1y_dot_n;
        let chi1z_f: f64 = self.chi1z_p + two_delta_tau*chi1z_dot_n;
        let chi2x_f: f64 = self.chi2x_p + two_delta_tau*chi2x_dot_n;
        let chi2y_f: f64 = self.chi2y_p + two_delta_tau*chi2y_dot_n;
        let chi2z_f: f64 = self.chi2z_p + two_delta_tau*chi2z_dot_n;

        // Evolve the orbital angular momentum using a leapfrog step
        let ell_dot_n: f64 = self.eta*(-1.0/v2 + self.l_2 + 2.0*self.l_2*v1 + 3.0*self.l_4*v2)*self.v_calc.v_dot_for_last_v()/l_n;
        let ell_x_dot_n: f64 = -self.op_delta2_i4*chi1x_dot_n - self.om_delta2_i4*chi2x_dot_n + ell_dot_n*self.ell_x_n;
        let ell_y_dot_n: f64 = -self.op_delta2_i4*chi1y_dot_n - self.om_delta2_i4*chi2y_dot_n + ell_dot_n*self.ell_y_n;
        let ell_z_dot_n: f64 = -self.op_delta2_i4*chi1z_dot_n - self.om_delta2_i4*chi2z_dot_n + ell_dot_n*self.ell_z_n;
        let ell_x_f: f64 = self.ell_x_p + two_delta_tau*ell_x_dot_n;
        let ell_y_f: f64 = self.ell_y_p + two_delta_tau*ell_y_dot_n;
        let ell_z_f: f64 = self.ell_z_p + two_delta_tau*ell_z_dot_n;

        // Evolve the precession phase
        let alpha_dot_n: f64 = (self.ell_y_n*ell_x_dot_n - self.ell_x_n*ell_y_dot_n)/(ell_x_dot_n*ell_x_dot_n + ell_y_dot_n*ell_y_dot_n);
        let psi_pr_f: f64 = self.psi_pr_p- two_delta_tau*alpha_dot_n*(self.ell_n).cos();

        // Check to see whether we have just crossed the 2nd/3rd quadrant line
        if ell_y_f < 0.0 && self.ell_y_n > 0.0 {
            if (ell_x_f*self.ell_y_n - self.ell_x_n*ell_y_f)/(self.ell_y_n-ell_y_f) < 0.0 {
                self.n_alpha_cycles = 1;
            }
        }
        else if ell_y_f > 0.0 && self.ell_y_n < 0.0{ 
            if (ell_x_f*self.ell_y_n - self.ell_x_n*ell_y_f)/(self.ell_y_n-ell_y_f) < 0.0 {
                self.n_alpha_cycles = -1
            }
        }

        // From here on, the future step just calculated becomes the present step
        // and the present step becomes the past step
        self.tau_p = self.tau_n;
        self.tau_n = self.tau_n + self.delta_tau_hf;

        self.chi1x_p = self.chi1x_n;
        self.chi1y_p = self.chi1y_n;
        self.chi1z_p = self.chi1z_n;
        self.chi1x_n = chi1x_f;
        self.chi1y_n = chi1y_f;
        self.chi1z_n = chi1z_f;

        self.chi2x_p = self.chi2x_n;
        self.chi2y_p = self.chi2y_n;
        self.chi2z_p = self.chi2z_n;
        self.chi2x_n = chi2x_f;
        self.chi2y_n = chi2y_f;
        self.chi2z_n = chi2z_f;

        self.ell_x_p = self.ell_x_n;
        self.ell_y_p = self.ell_y_n;
        self.ell_z_p = self.ell_z_n;
        self.ell_x_n = ell_x_f;
        self.ell_y_n = ell_y_f;
        self.ell_z_n = ell_z_f;

        self.psi_pr_p= self.psi_pr_n;
        self.psi_pr_n= psi_pr_f;


        self.alpha_p = self.alpha_n;
        self.alpha_n = atan2(self.ell_y_n,self.ell_x_n) + (self.n_alpha_cycles as f64)*2.0*PI;

        self.ell_p = self.ell_n;
        self.ell_n = atan2(self.ell_z_n,(self.ell_x_n*self.ell_x_n + self.ell_y_n*self.ell_y_n).sqrt());

        // Calculate the ideal next time step
        let s1dot: f64 = (chi1x_dot_n*chi1x_dot_n + chi1y_dot_n*chi1y_dot_n + chi1z_dot_n*chi1z_dot_n).sqrt();
        let s2dot: f64 = (chi2x_dot_n*chi2x_dot_n + chi2y_dot_n*chi2y_dot_n + chi2z_dot_n*chi2z_dot_n).sqrt();
        let delta_tau: f64 = f64::min(self.test_case.chi1/s1dot, self.test_case.chi2/s2dot);
        if delta_tau < self.delta_tau_hf {// if we need a smaller step
            if self.delta_tau_hp > self.delta_tau_hf{ 
                return false; // if we just did a smaller step, we are breaking down, so quit
            }
            else{
                self.delta_tau_hp = self.delta_tau_hf;// store the previous step
                self.delta_tau_hf =self.delta_tau_hf/2.0;  // reduce the next step size by two
            }
        }
        else{ // if we don't need a smaller step, repeat the current step
            self.delta_tau_hp = self.delta_tau_hf;
        }
        true
    }


}
