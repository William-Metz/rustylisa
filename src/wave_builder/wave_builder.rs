// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use crate::spin_evolver::spin_evolver::SpinEvolverClass;
use core::f64;

#[derive(Clone, PartialEq, Debug)]
pub struct WaveBuilder {
    pub lota_dn: f64,
    pub beta_: f64,
    pub delta: f64,
    pub delta_tau_r: f64,
    pub delta_tau: f64,
    pub spin_evolver: SpinEvolverClass,
    pub tau_r_dn: f64,
    pub eta: f64,
    pub chiax_dn: f64,
    pub chiay_dn: f64,
    pub chiaz_dn: f64,
    pub chisx_dn: f64,
    pub chisy_dn: f64,
    pub chisz_dn: f64,
    pub psi_r_dn: f64,
    pub psi_r_dp: f64,
    pub psi_p: f64,
    pub alpha_dn: f64,
    pub pn_order: u64, //Remove later
    pub hp: f64,
    pub hx: f64,
    pub vdn: f64,
    pub w: [f64; 248],
    pub a: [f64; 248],
    pub cos_am_psi: [[f64; 6]; 6],
    pub cos_ap_psi: [[f64; 6]; 6],
    pub sin_am_psi: [[f64; 6]; 6],
    pub sin_ap_psi: [[f64; 6]; 6],
}
