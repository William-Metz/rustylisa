mod test_case;
mod data_point;
mod wave_builder;
mod spin_data;
mod vcalculator;
mod constants;
mod spin_evolver;
mod case_supervisor;
use crate::test_case::test_case::TestCase;
use crate::wave_builder::wave_builder::WaveBuilder;
use crate::case_supervisor::case_supervisor::CaseSupervisor;
use std::time::Instant;
fn main(){

    let start = Instant::now();
    let testcase1 = TestCase::new(
        [10.0; 15],      // Uncertainties
        39.0,           // beta (degrees)
        24.0,           // psi (degrees)
        0.0,            // lambda0
        1.0,            // rho_0
        5.0,            // theta_ (degrees)
        268.5,          // phi_ (degrees)
        1.0,            // theta_1 (example)
        1.0,            // phi1 (example)
        1.0,            // theta2 (example)
        1.0,            // phi2 (example)
        10000.0,         // M
        500.0,          // T0
        0.1,            // delta
        0.0,            // chi1
        0.0,            // chi2
        10000000.0,     // R
        268.5,            // omega_
        None,           // chi_10_x
        None,      // chi_10_y
        None,      // chi_10_z
        None,      // chi_20_x
        None,      // chi_20_y
        None,      // chi_20_z
        0.0,            // Pn0
        0,              // pn_order
        2,              // detectors
        50.0,           // deltaT
        1.0,            // duration
        );
    let mut new_case_super = CaseSupervisor::new(testcase1);
    new_case_super.run_simulation();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

}

impl WaveBuilder {
    pub fn print_differences(&self, other: &Self) {
        if self.lota_dn != other.lota_dn {
            println!("lotaDN differs: self = {:?}, other = {:?}", self.lota_dn, other.lota_dn);
        }
        if self.beta_ != other.beta_ {
            println!("beta_ differs: self = {:?}, other = {:?}", self.beta_, other.beta_);
        }
        if self.delta != other.delta {
            println!("delta differs: self = {:?}, other = {:?}", self.delta, other.delta);
        }
        if self.delta_tau_r != other.delta_tau_r {
            println!("delta_tau_r differs: self = {:?}, other = {:?}", self.delta_tau_r, other.delta_tau_r);
        }

        if self.tau_r_dn != other.tau_r_dn {
            println!("tau_rDN differs: self = {:?}, other = {:?}", self.tau_r_dn, other.tau_r_dn);
        }
        if self.eta != other.eta {
            println!("eta differs: self = {:?}, other = {:?}", self.eta, other.eta);
        }
        if self.chiax_dn != other.chiax_dn {
            println!("chiaxDN differs: self = {:?}, other = {:?}", self.chiax_dn, other.chiax_dn);
        }
        if self.chiay_dn != other.chiay_dn {
            println!("chiayDN differs: self = {:?}, other = {:?}", self.chiay_dn, other.chiay_dn);
        }
        if self.chiaz_dn != other.chiaz_dn {
            println!("chiazDN differs: self = {:?}, other = {:?}", self.chiaz_dn, other.chiaz_dn);
        }
        if self.chisx_dn != other.chisx_dn {
            println!("chisxDN differs: self = {:?}, other = {:?}", self.chisx_dn, other.chisx_dn);
        }
        if self.chisy_dn != other.chisy_dn {
            println!("chisyDN differs: self = {:?}, other = {:?}", self.chisy_dn, other.chisy_dn);
        }
        if self.chisz_dn != other.chisz_dn {
            println!("chiszDN differs: self = {:?}, other = {:?}", self.chisz_dn, other.chisz_dn);
        }
        if self.p_sir_dn != other.p_sir_dn {
            println!("PsirDN differs: self = {:?}, other = {:?}", self.p_sir_dn, other.p_sir_dn);
        }
        if self.p_sir_dp != other.p_sir_dp {
            println!("PsirDP differs: self = {:?}, other = {:?}", self.p_sir_dp, other.p_sir_dp);
        }
        if self.p_si_p != other.p_si_p {
            println!("PsiP differs: self = {:?}, other = {:?}", self.p_si_p, other.p_si_p);
        }
        if self.alpha_dn != other.alpha_dn {
            println!("AlphaDN differs: self = {:?}, other = {:?}", self.alpha_dn, other.alpha_dn);
        }
        if self.pn_order != other.pn_order {
            println!("PNOrder differs: self = {:?}, other = {:?}", self.pn_order, other.pn_order);
        }

        for (i, (self_w, other_w)) in self.w.iter().zip(&other.w).enumerate() {
            if self_w != other_w {
                println!("W[{}] differs: self = {:?}, other = {:?}", i, self_w, other_w);
            }
        }
        for (i, (self_cos_am, other_cos_am)) in self.cos_am_psi.iter().zip(&other.cos_am_psi).enumerate() {
            for (j, (self_val, other_val)) in self_cos_am.iter().zip(other_cos_am).enumerate() {
                if self_val != other_val {
                    println!("Cos_Am_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
        for (i, (self_cos_ap, other_cos_ap)) in self.cos_ap_psi.iter().zip(&other.cos_ap_psi).enumerate() {
            for (j, (self_val, other_val)) in self_cos_ap.iter().zip(other_cos_ap).enumerate() {
                if self_val != other_val {
                    println!("Cos_Ap_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
        for (i, (self_sin_am, other_sin_am)) in self.sin_am_psi.iter().zip(&other.sin_am_psi).enumerate() {
            for (j, (self_val, other_val)) in self_sin_am.iter().zip(other_sin_am).enumerate() {
                if self_val != other_val {
                    println!("Sin_Am_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
        for (i, (self_sin_ap, other_sin_ap)) in self.sin_ap_psi.iter().zip(&other.sin_ap_psi).enumerate() {
            for (j, (self_val, other_val)) in self_sin_ap.iter().zip(other_sin_ap).enumerate() {
                if self_val != other_val {
                    println!("Sin_Ap_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
    }
}
