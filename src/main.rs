
#![allow(non_snake_case)]
#![allow(clippy::module_inception)]

use rustylisa::gen_tests::generate_test_cases;
use rustylisa::simulation_runner::simulation_runner::SimulationRunner;
use rustylisa::wave_generator_app::wave_generator_app::WaveGeneratorApp;
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "generate_tests" {
        println!("Generating tests");
        run_test_generation();
    } else {
        run_eframe_application();
    }
}

fn run_test_generation() {
    let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));

    let test_cases = generate_test_cases(5); // Generate test cases
    let simulation_runner = SimulationRunner::new(test_cases, runtime.clone());
    let monitor_handle = simulation_runner.run_all_simulations(false);

    // Wait for the monitoring task to complete
    runtime.block_on(async {
        monitor_handle.await.expect("Monitoring task panicked");
    });

    println!("All simulations are complete.");
}

fn run_eframe_application() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gravitational Wave Visualizer",
        native_options,
        Box::new(|_cc| Box::new(WaveGeneratorApp::default())),
    )
    .expect("Failed to start eframe application");
}

