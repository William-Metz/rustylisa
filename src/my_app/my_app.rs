use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::data_point::DataPoint;
use crate::test_case::test_case::TestCase;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

macro_rules! ui_label_drag {
    ($ui:ident, $struct:expr, $( $label:literal => $field:ident ),* $(,)?) => {
        $(
            $ui.label($label);
            $ui.add(egui::DragValue::new(&mut $struct.$field));
            $ui.end_row();
        )*
    };
}
// Now, your `MyApp` struct and `update` implementation can use `ui_label_drag!`.

pub struct MyApp {
    test_cases: Vec<TestCase>,

    // Simulation data
    simulation_data: Vec<Option<Vec<DataPoint>>>, // Store simulation data per test case

    needs_simulation: Vec<bool>, // Track if each test case needs simulation

    selected_tab: usize, // Track the selected test case
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            test_cases: vec![TestCase::new()], // Start with one test case
            simulation_data: vec![None],
            needs_simulation: vec![false],
            selected_tab: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Input Parameters");

            // Display each test case with collapsible input sections
            for (i, test_case) in self.test_cases.iter_mut().enumerate() {
                ui.collapsing(format!("Test Case {}", i + 1), |ui| {
                    egui::Grid::new(format!("param_grid_{}", i))
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .show(ui, |ui| {
                            ui_label_drag!(ui, test_case,
                                "M:" => M,
                                "delta:" => delta,
                                "t_0:" => t_0,
                                "R:" => R,
                                "beta:" => beta_,
                                "psi:" => psi,
                                "lambda0:" => lambda0,
                                "theta:" => theta_,
                                "phi:" => phi_,
                                "chi1:" => chi1,
                                "theta_1:" => theta_1,
                                "phi_1:" => phi_1,
                                "chi2:" => chi2,
                                "theta_2:" => theta_2,
                                "phi_2:" => phi_2,
                                "rho_0:" => rho_0,
                                "pn_order:" => pn_order,
                                "detectors:" => detectors,
                                "delta_t:" => delta_t,
                                "duration:" => duration,
                            );
                        });

                    // Button to run simulation for this specific test case
                    if ui.button("Run Simulation").clicked() {
                        self.needs_simulation[i] = true;
                    }
                });
            }

            // Add a new test case
            if ui.button("Add New Test Case").clicked() {
                self.test_cases.push(TestCase::new());
                self.simulation_data.push(None);
                self.needs_simulation.push(false);
            }

            ui.separator();

            // Tab selection using ComboBox
            ui.label("Select Test Case for Plot:");
            egui::ComboBox::from_label("Test Case")
                .selected_text(format!("Test Case {}", self.selected_tab + 1))
                .show_ui(ui, |ui| {
                    for i in 0..self.test_cases.len() {
                        ui.selectable_value(
                            &mut self.selected_tab,
                            i,
                            format!("Test Case {}", i + 1),
                        );
                    }
                });

            ui.separator();

            // Display plot for the selected test case
            ui.heading(format!(
                "Gravitational Wave Plot for Test Case {}",
                self.selected_tab + 1
            ));
            if let Some(data) = &self.simulation_data[self.selected_tab] {
                let hp_points: PlotPoints =
                    data.iter().map(|point| [point.time, point.hp]).collect();
                let hx_points: PlotPoints =
                    data.iter().map(|point| [point.time, point.hx]).collect();

                let hp_line = Line::new(hp_points).name("HP");
                let hx_line = Line::new(hx_points).name("HX");

                Plot::new(format!("wave_plot_{}", self.selected_tab))
                    .view_aspect(2.0)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(hp_line);
                        plot_ui.line(hx_line);
                    });
            } else {
                ui.label("No simulation data available. Please run the simulation.");
            }
        });

        // Run simulations
        let mut indices_to_simulate = vec![];
        for (i, needs_sim) in self.needs_simulation.iter().enumerate() {
            if *needs_sim {
                indices_to_simulate.push(i);
            }
        }

        for &i in &indices_to_simulate {
            self.run_simulation(i);
            self.needs_simulation[i] = false;
        }
    }
}

impl MyApp {
    fn run_simulation(&mut self, index: usize) {
        use std::time::Instant;

        let start = Instant::now();

        let testcase = self.test_cases[index].clone();
        let mut case_supervisor = CaseSupervisor::new(testcase);
        case_supervisor.run_simulation();

        let duration = start.elapsed();
        println!("Simulation Time elapsed: {:?}", duration);

        // Store the simulation data
        self.simulation_data[index] = Some(case_supervisor.wave.spin_evolver.data.clone());
    }
}
