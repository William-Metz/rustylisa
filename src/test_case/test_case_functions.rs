// test_case_functions.rs
use crate::test_case::test_case::TestCase;

pub fn print_test_case(testcase: &TestCase) {
    println!("M:{}", testcase.M);
    println!("\u{03B4}:{}", testcase.delta);
    println!("\u{2080}:{}", testcase.t_0);
    println!("R:{}", testcase.R);
    println!("\u{03b2}:{}", testcase.beta_);
    println!("\u{03c8}:{}", testcase.psi);
    println!("\u{03BB}\u{2080}:{}", testcase.lambda0);
    println!("\u{0398}:{}", testcase.theta_);
    println!("\u{03A6}:{}", testcase.phi_);
    println!("\u{03A9}:{}", testcase.omega_);
    println!("\u{03A7}\u{00B9}\u{2070}x:{}", testcase.chi_10_x.map_or("None".to_string(), |v| v.to_string()));
    println!("\u{03A7}\u{00B9}\u{2070}y:{}", testcase.chi_10_y.map_or("None".to_string(), |v| v.to_string()));
    println!("\u{03A7}\u{00B9}\u{2070}z:{}", testcase.chi_10_z.map_or("None".to_string(), |v| v.to_string()));
    println!("\u{03A7}\u{00B2}\u{2070}x:{}", testcase.chi_20_x.map_or("None".to_string(), |v| v.to_string()));
    println!("\u{03A7}\u{00B2}\u{2070}y:{}", testcase.chi_20_y.map_or("None".to_string(), |v| v.to_string()));
    println!("\u{03A7}\u{00B2}\u{2070}z:{}", testcase.chi_20_z.map_or("None".to_string(), |v| v.to_string()));
    println!("\u{03C1}\u{2080}:{}", testcase.p_n0);
    println!("detectors:{}", testcase.detectors);
    println!("\u{0394}T:{}", testcase.delta_t);
    println!("Duration:{}", testcase.duration);
}
