#![allow(non_snake_case)]
#![allow(clippy::module_inception)]

use rustylisa::gen_tests::create_baseline;
use rustylisa::wave_generator_app::wave_generator_app::WaveGeneratorApp;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "generate_tests" {
        if args.len() == 2 {
            eprintln!("Error: Too few arguments. Please specify number of tests to generate");
            std::process::exit(1);
        }
        match args[2].parse::<usize>() {
            Ok(num_tests) => {
                println!("Generating {} tests", num_tests);
                create_baseline(num_tests);
            }
            Err(_) => {
                eprintln!("Error: Invalid number of tests. Please provide a positive integer.");
                std::process::exit(1);
            }
        }
    } else {
        run_eframe_application();
    }
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
