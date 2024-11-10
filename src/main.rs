// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/main.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
#![allow(non_snake_case)]
#![allow(clippy::module_inception)]

use rustylisa::wave_generator_app::wave_generator_app::WaveGeneratorApp;
fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gravitational Wave Visualizer",
        native_options,
        Box::new(|_cc| Box::new(WaveGeneratorApp::default())),
    )
    .expect("Failed to start eframe application");
}
