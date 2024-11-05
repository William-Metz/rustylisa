// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/spin_data/spin_data.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
#[derive(Debug)]
pub struct SpinData {
    pub v: f64,
    pub iota: f64,
    pub alpha: f64,
    pub chi_ax: f64,
    pub chi_ay: f64,
    pub chi_az: f64,
    pub chi_sx: f64,
    pub chi_sy: f64,
    pub chi_sz: f64,
    pub psi: f64,
}

impl SpinData {
    pub fn new() -> Self {
        SpinData {
            v: 0.0,
            iota: 0.0,
            alpha: 0.0,
            chi_ax: 0.0,
            chi_ay: 0.0,
            chi_az: 0.0,
            chi_sx: 0.0,
            chi_sy: 0.0,
            chi_sz: 0.0,
            psi: 0.0,
        }
    }
}
