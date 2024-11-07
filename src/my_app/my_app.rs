use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::data_point::DataPoint;
use crate::test_case::test_case::TestCase;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use tokio;

use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
macro_rules! ui_label_drag {
    ($ui:ident, $struct:expr, $( $label:literal => $field:ident ),* $(,)?) => {
        $(
            $ui.label($label);
            $ui.add(egui::DragValue::new(&mut $struct.$field));
            $ui.end_row();
        )*
    };
}
#[derive(Clone)]
enum View {
    Input,
    Results,
}

pub struct MyApp {
    test_cases: Vec<TestCase>,
    simulation_data: Arc<Mutex<Vec<Option<Vec<DataPoint>>>>>, // Shared simulation data
    needs_simulation: Vec<bool>,
    selected_tab: usize,
    current_view: View,
    runtime: Arc<Runtime>, // Shared runtime
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            test_cases: vec![TestCase::new(
                100000.0, 0.1, 500.0, 10000000.0, 39.0, 24.0, 0.0, 5.0, 268.5, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0, 2, 50.0, 1.0,
            )],
            simulation_data: Arc::new(Mutex::new(vec![None])),
            needs_simulation: vec![false],
            selected_tab: 0,
            current_view: View::Input,
            runtime: Arc::new(Runtime::new().expect("Failed to create Tokio runtime")),
        }
    }
}

impl Clone for MyApp {
    fn clone(&self) -> Self {
        Self {
            test_cases: self.test_cases.clone(),
            simulation_data: Arc::clone(&self.simulation_data), // Share simulation data
            needs_simulation: self.needs_simulation.clone(),
            selected_tab: self.selected_tab,
            current_view: self.current_view.clone(),
            runtime: Arc::clone(&self.runtime),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut simulations_to_run = vec![];

        // Navigation buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Input Parameters").clicked() {
                    self.current_view = View::Input;
                }
                if ui.button("Simulation Results").clicked() {
                    self.current_view = View::Results;
                }
                if ui.button("Run All Simulations").clicked() {
                    simulations_to_run.extend(0..self.test_cases.len()); // Queue all simulations
                }
            });
        });

        // Display content based on the current view
        match self.current_view {
            View::Input => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Input Parameters");

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

                            if ui.button("Run Simulation").clicked() {
                                simulations_to_run.push(i); // Queue this simulation
                            }
                        });
                    }

                    if ui.button("Add New Test Case").clicked() {
                        self.test_cases.push(TestCase::new(
                            100000.0, 0.1, 500.0, 10000000.0, 39.0, 24.0, 0.0, 5.0, 268.5, 0.0,
                            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 2, 50.0, 1.0,
                        ));
                        let mut data = self.simulation_data.lock().unwrap();
                        data.push(None);
                        self.needs_simulation.push(false);
                    }
                });
            }
            View::Results => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Simulation Results");

                    // Tab for selecting the test case to display
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

                    // Access simulation data safely using the mutex
                    let data = self.simulation_data.lock().unwrap();
                    ui.heading(format!(
                        "Gravitational Wave Plot for Test Case {}",
                        self.selected_tab + 1
                    ));
                    if let Some(ref points) = data[self.selected_tab] {
                        let hp_points: PlotPoints =
                            points.iter().map(|point| [point.time, point.hp]).collect();
                        let hx_points: PlotPoints =
                            points.iter().map(|point| [point.time, point.hx]).collect();

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
            }
        }

        for i in simulations_to_run {
            let testcase = self.test_cases[i].clone();
            let simulation_data = Arc::clone(&self.simulation_data);
            let runtime = Arc::clone(&self.runtime);
            runtime.spawn(async move {
                let mut case_supervisor = CaseSupervisor::new(testcase);

                // Run the simulation
                case_supervisor.run_simulation();

                // Store the simulation data
                let mut data = simulation_data.lock().expect("Failed to lock mutex");
                data[i] = Some(case_supervisor.wave.spin_evolver.data.clone());
            });
        }
    }
}
