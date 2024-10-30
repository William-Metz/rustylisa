mod test_case;
mod data_point;
mod wave_builder;
mod spin_data;
mod vcalculator;
mod constants;
mod spin_evolver;
mod case_supervisor;
mod plotting;
use crate::test_case::test_case::TestCase;
use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::plotting::plotting::plot_data;
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
    let mut data = new_case_super.wave.spin_evolver.data.clone();
    plot_data(&data).unwrap();

}

