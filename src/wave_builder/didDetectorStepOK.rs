use std::f64;

use crate::test_case::test_case::TestCase;
use crate::spin_evolver::spin_evolver::SpinEvolverClass;
use crate::spin_data::spin_data::SpinData;
use crate::wave_builder::wave_builder::WaveBuilder;

impl WaveBuilder{
    pub fn did_step_ok(&mut self, step_num: u64, test_case: &TestCase) -> bool{  
         let curr: f64 = ((step_num as f64)+2.0)*test_case.delta_t;


        if ((step_num as f64)+2.0)*test_case.delta_t > (test_case.tau_c){
            return false
        }
        // Otherwise, check if we can get data from the spin evolver
        if let Some(spinData) = self.spin_evolver.get_spin_data_at_time(test_case, (step_num as f64)*test_case.delta_t){
            // We have data, so
            self.VDN = spinData.v; // get the current speed
                                  // If our speed is half that of light, our approximations are breaking down, so bail out
            if self.VDN > 0.5{

                return false;

            }

            // Otherwise, load the rest of the data coming from the spin evolver
            self.lota_dn = spinData.iota;
            self.alpha_dn = spinData.alpha;
            self.chiax_dn = spinData.chi_ax;
            self.chiay_dn = spinData.chi_ay;
            self.chiaz_dn = spinData.chi_az;
            self.chisx_dn = spinData.chi_sx;
            self.chisy_dn = spinData.chi_sy;
            self.chisz_dn = spinData.chi_sz;

            // Calculate the wave phase
            let tau_rm:f64 = (step_num as f64- 0.5)*self.delta_tau_r;
            self.p_sir_dn = spinData.psi;
            // do the following instead of the above if we want the data in the orbiting LISA frame
            // psi_rDN = psi_rDP + (1.0 + Parameters.Ve*Sin(Parameters.Θ)*Sin(Parameters.GMΩe*tau_rm - Parameters.Φ))*(spinData.psi_ - psi_P)
            self.p_sir_dp = self.p_sir_dn;
            self.p_si_p = spinData.psi;
            self.tau_r_dn = (step_num as f64)*self.delta_tau_r;

            // Calculate the wave
            self.calculate_amplitudes();
            self.CalculateWaveFactors();
            self.sum_source_h(false);

            // Write out useful information for plotting (if this is not a case from a file)
            self.save_data();
            ////SaveDataForPlotting(tau_rDN)

            // We have completed the detector step successfully

            return true;
        }
        else {
            return false;
        }
    }
}

