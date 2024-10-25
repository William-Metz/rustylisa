mod test_case;
mod wave_builder;
mod spin_data;
mod vcalculator;
mod constants;
mod spin_evolver;
use crate::test_case::test_case::TestCase;
use crate::wave_builder::wave_builder::Wave_Builder;
fn main(){

    let testcase1 = TestCase::new(
        [0.0; 15],      // Uncertainties
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
        1000.0,         // M
        500.0,          // T0
        0.1,            // delta
        1.1,            // chi1
        1.2,            // chi2
        10000000.0,     // R
        268.5,            // omega_
        None,           // chi_10_x
        None,      // chi_10_y
        None,      // chi_10_z
        None,      // chi_20_x
        None,      // chi_20_y
        None,      // chi_20_z
        0.0,            // Pn0
        2,              // detectors
        50.0,           // deltaT
        1.0,            // duration
        );
    let mut wave = Wave_Builder::new(&testcase1);

    let mut last_wave = wave.clone();
    for n in 0.. testcase1.NSteps{
        let tau_r = wave.delta_tau_r; //update
        if last_wave == wave {
            println!("test");

        }
        else{
            last_wave.print_differences(&wave);

        }
        if ! wave.didStepOK(n,&testcase1){
            println!("Colences");
            break;

        }



    }

}

impl Wave_Builder {
    pub fn print_differences(&self, other: &Self) {
        if self.lotaDN != other.lotaDN {
            println!("lotaDN differs: self = {:?}, other = {:?}", self.lotaDN, other.lotaDN);
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

        if self.tau_rDN != other.tau_rDN {
            println!("tau_rDN differs: self = {:?}, other = {:?}", self.tau_rDN, other.tau_rDN);
        }
        if self.eta != other.eta {
            println!("eta differs: self = {:?}, other = {:?}", self.eta, other.eta);
        }
        if self.pi != other.pi {
            println!("pi differs: self = {:?}, other = {:?}", self.pi, other.pi);
        }
        if self.chiaxDN != other.chiaxDN {
            println!("chiaxDN differs: self = {:?}, other = {:?}", self.chiaxDN, other.chiaxDN);
        }
        if self.chiayDN != other.chiayDN {
            println!("chiayDN differs: self = {:?}, other = {:?}", self.chiayDN, other.chiayDN);
        }
        if self.chiazDN != other.chiazDN {
            println!("chiazDN differs: self = {:?}, other = {:?}", self.chiazDN, other.chiazDN);
        }
        if self.chisxDN != other.chisxDN {
            println!("chisxDN differs: self = {:?}, other = {:?}", self.chisxDN, other.chisxDN);
        }
        if self.chisyDN != other.chisyDN {
            println!("chisyDN differs: self = {:?}, other = {:?}", self.chisyDN, other.chisyDN);
        }
        if self.chiszDN != other.chiszDN {
            println!("chiszDN differs: self = {:?}, other = {:?}", self.chiszDN, other.chiszDN);
        }
        if self.PsirDN != other.PsirDN {
            println!("PsirDN differs: self = {:?}, other = {:?}", self.PsirDN, other.PsirDN);
        }
        if self.PsirDP != other.PsirDP {
            println!("PsirDP differs: self = {:?}, other = {:?}", self.PsirDP, other.PsirDP);
        }
        if self.PsiP != other.PsiP {
            println!("PsiP differs: self = {:?}, other = {:?}", self.PsiP, other.PsiP);
        }
        if self.AlphaDN != other.AlphaDN {
            println!("AlphaDN differs: self = {:?}, other = {:?}", self.AlphaDN, other.AlphaDN);
        }
        if self.PNOrder != other.PNOrder {
            println!("PNOrder differs: self = {:?}, other = {:?}", self.PNOrder, other.PNOrder);
        }
        for (i, (self_w, other_w)) in self.W.iter().zip(&other.W).enumerate() {
            if self_w != other_w {
                println!("W[{}] differs: self = {:?}, other = {:?}", i, self_w, other_w);
            }
        }
        for (i, (self_cos_am, other_cos_am)) in self.Cos_Am_Psi.iter().zip(&other.Cos_Am_Psi).enumerate() {
            for (j, (self_val, other_val)) in self_cos_am.iter().zip(other_cos_am).enumerate() {
                if self_val != other_val {
                    println!("Cos_Am_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
        for (i, (self_cos_ap, other_cos_ap)) in self.Cos_Ap_Psi.iter().zip(&other.Cos_Ap_Psi).enumerate() {
            for (j, (self_val, other_val)) in self_cos_ap.iter().zip(other_cos_ap).enumerate() {
                if self_val != other_val {
                    println!("Cos_Ap_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
        for (i, (self_sin_am, other_sin_am)) in self.Sin_Am_Psi.iter().zip(&other.Sin_Am_Psi).enumerate() {
            for (j, (self_val, other_val)) in self_sin_am.iter().zip(other_sin_am).enumerate() {
                if self_val != other_val {
                    println!("Sin_Am_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
        for (i, (self_sin_ap, other_sin_ap)) in self.Sin_Ap_Psi.iter().zip(&other.Sin_Ap_Psi).enumerate() {
            for (j, (self_val, other_val)) in self_sin_ap.iter().zip(other_sin_ap).enumerate() {
                if self_val != other_val {
                    println!("Sin_Ap_Psi[{}][{}] differs: self = {:?}, other = {:?}", i, j, self_val, other_val);
                }
            }
        }
    }
}
