use std::f64;

use crate::wave_builder::wave_builder::WaveBuilder;

impl WaveBuilder {
    pub fn did_step_ok(&mut self, step_num: u64) -> bool {
        if ((step_num as f64) + 2.0) * self.delta_tau > (self.spin_evolver.test_case.tau_c) {
            return false;
        }
        // Otherwise, check if we can get data from the spin evolver
        if let Some(spin_data) = self
            .spin_evolver
            .get_spin_data_at_time((step_num as f64) * self.delta_tau)
        {
            // We have data, so
            self.vdn = spin_data.v; // get the current speed
                                    // If our speed is half that of light, our approximations are breaking down, so bail out
            if self.vdn > 0.5 {
                return false;
            }

            // Otherwise, load the rest of the data coming from the spin evolver
            self.lota_dn = spin_data.iota;
            self.alpha_dn = spin_data.alpha;
            self.chiax_dn = spin_data.chi_ax;
            self.chiay_dn = spin_data.chi_ay;
            self.chiaz_dn = spin_data.chi_az;
            self.chisx_dn = spin_data.chi_sx;
            self.chisy_dn = spin_data.chi_sy;
            self.chisz_dn = spin_data.chi_sz;

            // Calculate the wave phase
            //let tau_rm:f64 = (step_num as f64- 0.5)*self.delta_tau_r;
            self.psi_r_dn = spin_data.psi;
            // do the following instead of the above if we want the data in the orbiting LISA frame
            // psi_rDN = psi_rDP + (1.0 + Parameters.Ve*Sin(Parameters.Θ)*Sin(Parameters.GMΩe*tau_rm - Parameters.Φ))*(spinData.psi_ - psi_P)
            self.psi_r_dp = self.psi_r_dn;
            self.psi_p = spin_data.psi;
            self.tau_r_dn = (step_num as f64) * self.delta_tau_r;

            // Calculate the wave
            self.calculate_amplitudes();
            self.calculate_wave_factors();
            self.sum_source_h(false);

            // Write out useful information for plotting (if this is not a case from a file)
            self.save_data(step_num);
            ////SaveDataForPlotting(tau_rDN)

            // We have completed the detector step successfully

            true
        } else {
            false
        }
    }
}
