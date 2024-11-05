// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/wave_builder/savedata.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use crate::constants::YEAR;
use crate::data_point::DataPoint;
use crate::wave_builder::wave_builder::WaveBuilder;
use std::f64::consts::PI;

impl WaveBuilder {
    pub fn save_data(&mut self, step: u64) {
        //        tau_r*Parameters.GM/Parameters.Year;
        let t: f64 = (step as f64) * self.spin_evolver.test_case.delta_t / YEAR;
        let omega: f64 = self.vdn.powi(3) / self.spin_evolver.test_case.GM;
        let torb: f64 = 2.0 * PI / omega;

        let data_point = DataPoint {
            time: t,
            hp: self.hp,
            hx: self.hx,
            torb,
            n_step: step,
        };
        self.spin_evolver.data.push(data_point);
    }
}
