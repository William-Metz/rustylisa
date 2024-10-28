use std::f64;

use crate::test_case::test_case::TestCase;
use crate::spin_evolver::spin_evolver::SpinEvolverClass;
use crate::spin_data::spin_data::SpinData;
use crate::wave_builder::wave_builder::Wave_Builder;

impl Wave_Builder{
    pub fn didStepOK(&mut self, step_num: u8, test_case: &TestCase) -> bool{  


        if ((step_num as f64)+2.0)*test_case.deltaT > (test_case.tau_c){
            return false
        }
        // Otherwise, check if we can get data from the spin evolver
        if let Some(spinData) = self.Spin_Evolver.GetSpinDataAtTime(test_case, (step_num as f64)*test_case.deltaT){
            // We have data, so
            let VDN = spinData.v; // get the current speed
                                  // If our speed is half that of light, our approximations are breaking down, so bail out
            if VDN > 0.5{

                return false;

            }

            // Otherwise, load the rest of the data coming from the spin evolver
            self.lotaDN = spinData.iota;
            println!("iota:{}",spinData.iota);
            self.AlphaDN = spinData.alpha;
            self.chiaxDN = spinData.chi_ax;
            self.chiayDN = spinData.chi_ay;
            self.chiazDN = spinData.chi_az;
            self.chisxDN = spinData.chi_sx;
            self.chisyDN = spinData.chi_sy;
            self.chiszDN = spinData.chi_sz;

            // Calculate the wave phase
            let tau_rm:f64 = (step_num as f64- 0.5)*self.delta_tau_r;
            self.PsirDN = spinData.psi;
            // do the following instead of the above if we want the data in the orbiting LISA frame
            // psi_rDN = psi_rDP + (1.0 + Parameters.Ve*Sin(Parameters.Θ)*Sin(Parameters.GMΩe*tau_rm - Parameters.Φ))*(spinData.psi_ - psi_P)
            self.PsirDP = self.PsirDN;
            self.PsiP = spinData.psi;
            self.tau_rDN = (step_num as f64)*self.delta_tau_r;

            // Calculate the wave
            _ = self.CalculateAmplitudes();
            _ = self.CalculateWaveFactors();
            // SumSourceH(W);

            // Write out useful information for plotting (if this is not a case from a file)
            ////SaveDataForPlotting(tau_rDN)

            // We have completed the detector step successfully

            return true;
        }
        else {
            return false;
        }
    }
}

