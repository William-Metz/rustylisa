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
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CaseSupervisor {
    pub case: TestCase,
    pub wave: WaveBuilder,
    pub delta_tau: f64,
}

impl CaseSupervisor {
    // Constructor function to initialize the struct
    pub fn new(case: TestCase) -> CaseSupervisor {
        //        let delta_tau_r = case.delta_t / case.GM;
        let delta_tau = 0.0;
        let wave = WaveBuilder::new(&case);
        CaseSupervisor {
            case,
            wave,
            delta_tau,
        }
    }
    pub fn save_to_csv(&self) {
        let file = File::create("data.csv").expect("Could not create file");
        let mut writer = BufWriter::new(file);

        // Write headers if the file is empty
        writeln!(writer, "Time,HP,HX,Torb,NSteps").expect("Could not write headers");

        // Write all data points
        for data_point in self.wave.spin_evolver.data.iter() {
            writeln!(
                writer,
                "{},{},{},{},{}",
                data_point.time, data_point.hp, data_point.hx, data_point.torb, data_point.n_step
            )
            .expect("Could not write data point");
        }
    }
    pub fn run_simulation(&mut self) {
        for n in 0..self.case.n_steps {
            self.delta_tau = (n as f64) * self.wave.delta_tau_r; //update

            if !self.wave.did_step_ok(n) {
                println!("Colences");
                self.save_to_csv();
                break;
            }

            //   wave.print_differences(&last_wave);
        }
    }
}
