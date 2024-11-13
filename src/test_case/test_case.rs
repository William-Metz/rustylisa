// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use core::f64;
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct TestCase {
    // Fields from Case_Info
    pub uncertainties: [f64; 15],
    pub beta_: f64,
    pub psi: f64,
    pub lambda0: f64,
    pub rho_0: f64,
    pub theta_: f64,
    pub phi_: f64,
    pub theta_1: f64,
    pub phi_1: f64,
    pub theta_2: f64,
    pub phi_2: f64,
    pub M: f64,  // Mass of the system in solar masses
    pub GM: f64, // Gravitational constant * mass
    pub tau_c: f64,
    pub n_steps: u64,

    // Fields from TestCase
    pub delta: f64,
    pub t_0: f64,
    pub R: f64,
    pub r: f64,
    pub chi_10_x: Option<f64>,
    pub chi_10_y: Option<f64>,
    pub chi_10_z: Option<f64>,
    pub chi_20_x: Option<f64>,
    pub chi_20_y: Option<f64>,
    pub chi_20_z: Option<f64>,
    pub pn_order: i32,
    pub detectors: i32,
    pub delta_t: f64,
    pub duration: f64,
    pub chi1: f64,
    pub chi2: f64,
}
