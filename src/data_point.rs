// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/data_point.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct DataPoint {
    pub time: f64,
    pub hp: f64,
    pub hx: f64,
    pub torb: f64,
    pub n_step: u64,
}
