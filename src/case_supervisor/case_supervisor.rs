// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/case_supervisor/case_supervisor.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use crate::test_case::test_case::TestCase;
use crate::wave_builder::wave_builder::WaveBuilder;

pub struct CaseSupervisor {
    pub case: TestCase,
    pub wave: WaveBuilder,
    pub delta_tau: f64,
}


