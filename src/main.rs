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

use rustylisa::gen_tests::generate_test_cases;
use rustylisa::simulation_runner::simulation_runner::SimulationRunner;
use rustylisa::wave_generator_app::wave_generator_app::WaveGeneratorApp;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::runtime::Runtime;
fn main() {
    /* let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));

    let test_cases = generate_test_cases(1); // Generate 1000 test cases

    let simulation_runner = SimulationRunner::new(test_cases, runtime.clone());
    let monitor_handle = simulation_runner.run_all_simulations(true);

    // Wait for the monitoring task to complete
    runtime.block_on(async {
        monitor_handle.await.expect("Monitoring task panicked");
    });

    println!("All simulations are complete.");    */
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gravitational Wave Visualizer",
        native_options,
        Box::new(|_cc| Box::new(WaveGeneratorApp::default())),
    )
    .expect("Failed to start eframe application");
}
