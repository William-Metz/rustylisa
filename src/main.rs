#![allow(non_snake_case)]
#![allow(clippy::module_inception)]
mod case_supervisor;
mod constants;
mod data_point;
mod spin_data;
mod spin_evolver;
mod test_case;
mod vcalculator;
mod wave_builder;

use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::data_point::DataPoint;
use crate::test_case::test_case::TestCase;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gravitational Wave Visualizer",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
    .expect("Failed to start eframe application");
}

struct MyApp {
    test_case: TestCase,

    // Simulation data
    simulation_data: Option<Vec<DataPoint>>,

    // Flag to indicate if the simulation needs to be run
    needs_simulation: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            test_case: TestCase::new(),
            simulation_data: None,
            needs_simulation: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Input Parameters");

            egui::Grid::new("param_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    // Beta
                    ui.label("beta:");
                    ui.add(egui::DragValue::new(&mut self.test_case.beta_));
                    ui.end_row();

                    // Psi
                    ui.label("psi:");
                    ui.add(egui::DragValue::new(&mut self.test_case.psi));
                    ui.end_row();

                    // Lambda0
                    ui.label("lambda0:");
                    ui.add(egui::DragValue::new(&mut self.test_case.lambda0));
                    ui.end_row();

                    // ... Add other parameters as needed

                    // Duration
                    ui.label("duration:");
                    ui.add(egui::DragValue::new(&mut self.test_case.duration));
                    ui.end_row();
                });

            // Run Simulation button
            if ui.button("Run Simulation").clicked() {
                self.needs_simulation = true;
            }

            ui.separator();

            // Plot the simulation data if available
            if let Some(data) = &self.simulation_data {
                ui.heading("Gravitational Wave Plot");

                let hp_points: PlotPoints =
                    data.iter().map(|point| [point.time, point.hp]).collect();

                let hx_points: PlotPoints =
                    data.iter().map(|point| [point.time, point.hx]).collect();

                let hp_line = Line::new(hp_points).name("HP");
                let hx_line = Line::new(hx_points).name("HX");

                Plot::new("wave_plot")
                    .view_aspect(2.0)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(hp_line);
                        plot_ui.line(hx_line);
                    });
            }
        });

        // Run the simulation if needed
        if self.needs_simulation {
            self.run_simulation();
            self.needs_simulation = false;
        }
    }
}

impl MyApp {
    fn run_simulation(&mut self) {
        use std::time::Instant;

        let start = Instant::now();

        // Clone the test case to use in simulation
        let testcase = self.test_case.clone();

        let mut case_supervisor = CaseSupervisor::new(testcase);
        case_supervisor.run_simulation();

        let duration = start.elapsed();
        println!("Simulation Time elapsed: {:?}", duration);

        // Store the simulation data
        self.simulation_data = Some(case_supervisor.wave.spin_evolver.data.clone());
    }
}
