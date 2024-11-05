
// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/spin_evolver/spin_evolver.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use crate::data_point::DataPoint;
use crate::test_case::test_case::TestCase;
use crate::vcalculator::vcalculator::VCalculator;
use core::f64;
#[derive(Clone, PartialEq, Debug)]
pub struct SpinEvolverClass {
    pub c_10: f64,
    pub c_12: f64,
    pub c_14: f64,
    pub c_20: f64,
    pub c_22: f64,
    pub c_24: f64,
    pub l_2: f64,
    pub l_3: f64,
    pub l_4: f64,
    pub n_alpha_cycles: i32,
    pub om_delta2_i4: f64,
    pub op_delta2_i4: f64,
    pub test_case: TestCase,
    pub data: Vec<DataPoint>,
    pub s_ell_: f64,
    pub v_0: f64,
    pub v_calc: VCalculator,
    pub iota_n: f64,
    pub iota_p: f64,
    pub alpha_n: f64,
    pub alpha_p: f64,
    pub delta_tau_hf: f64,
    pub delta_tau_hp: f64,
    pub eta: f64,
    pub simga_ell_: f64,
    pub tau_n: f64,
    pub tau_p: f64,
    pub chi1x_n: f64,
    pub chi1x_p: f64,
    pub chi1y_n: f64,
    pub chi1y_p: f64,
    pub chi1z_n: f64,
    pub chi1z_p: f64,
    pub chi1ell_: f64,
    pub chi2x_n: f64,
    pub chi2x_p: f64,
    pub chi2y_n: f64,
    pub chi2y_p: f64,
    pub chi2z_n: f64,
    pub chi2z_p: f64,
    pub chi2ell_: f64,
    pub psi_pr_n: f64,
    pub psi_pr_p: f64,
    pub ell_n: f64,
    pub ell_p: f64,
    pub ell_x_n: f64,
    pub ell_x_p: f64,
    pub ell_y_n: f64,
    pub ell_y_p: f64,
    pub ell_z_n: f64,
    pub ell_z_p: f64,
}
