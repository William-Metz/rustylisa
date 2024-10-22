mod test_case;
mod wave_builder;
mod Case_Supervisor;
mod constants;
use crate::test_case::test_case::TestCase;
use crate::test_case::test_case_functions::print_test_case;
use crate::wave_builder::CalculateAmplitudes::CalculateAmplitudes;
use crate::wave_builder::CalculateWaveFactors::CalculateWaveFactors;
use crate::wave_builder::wave_builder::Wave_Builder;
use crate::Case_Supervisor::case_supervisor;
fn main(){

    println!("Hello World!");


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
    1.0,            // omega_
    None,           // chi_10_x
    Some(1.1),      // chi_10_y
    Some(1.2),      // chi_10_z
    Some(2.0),      // chi_20_x
    Some(2.1),      // chi_20_y
    Some(2.2),      // chi_20_z
    1.0,            // Pn0
    2,              // detectors
    50.0,           // deltaT
    1.0,            // duration
);
    let mut wave = Wave_Builder{
        lotaDN: 1.0, 
        beta_: 1.0, 
        delta: 1.0, 
        eta: 1.0, 
        pi: 1.0, 
        chiaxDN: 1.0, 
        chiayDN: 1.0, 
        chiazDN: 1.0, 
        chisxDN: 1.0, 
        chisyDN: 1.0, 
        chiszDN: 1.0, 
        AlphaDN: 1.0,
        PNOrder: 10,
        PsirDN: 1.0,
        W: [0.0;248],
        Cos_Am_Psi: [[0.0; 6]; 6],
        Cos_Ap_Psi: [[0.0; 6]; 6],
        Sin_Am_Psi: [[0.0; 6]; 6],
        Sin_Ap_Psi: [[0.0; 6]; 6],

    };
    for n in 0.. testcase1.NSteps{
        let mut tau_r = 0; //update
        if not DidDectorStepOK(n){
           print!("Colences");
           break;

        }



    }
    let A = CalculateAmplitudes(&wave);
    let B = CalculateWaveFactors(&mut wave);

    // Print all amplitudes
    for (i, &amplitude) in A.iter().enumerate() {
        println!("A[{}] = {}", i, amplitude);
    }
    for n in 1..10{
        println!("Case Number:{}", n);
        if true { // will be if it stepped properly
            print_test_case(&testcase1);
        }

    }
}
