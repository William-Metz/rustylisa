use crate::test_case::test_case::TestCase;
use crate::wave_generator_app::view::View;
use crate::wave_generator_app::wave_generator_app::WaveGeneratorApp;
use csv::{ReaderBuilder, Trim};
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use rfd::FileDialog;
use std::fs::File;

macro_rules! ui_label_drag {
    ($ui:ident, $struct:expr, $( $label:literal => $field:ident ),* $(,)?) => {
        $(
            $ui.label($label);
            $ui.add(egui::DragValue::new(&mut $struct.$field));
            $ui.end_row();
        )*
    };
}

impl eframe::App for WaveGeneratorApp {
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
                    self.runner.run_all_simulations(); // Call to run all simulations
                }
            });
        });

        // Display content based on the current view
        match self.current_view {
            View::Input => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Input Parameters");

                    for (i, test_case) in self.runner.test_cases.iter_mut().enumerate() {
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
                        self.runner.test_cases.push(TestCase::new(
                                100000.0, 0.1, 500.0, 10000000.0, 39.0, 24.0, 0.0, 5.0, 268.5, 0.0,
                                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 2, 50.0, 1.0,
                        ));
                        let mut data = self.runner.simulation_data.lock().unwrap();
                        data.push(None);
                    }
                    if ui.button("Import Test Cases from CSV").clicked() {
                        if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                            if let Ok(file) = File::open(&path) {
                                let mut reader = ReaderBuilder::new()
                                    .has_headers(false)
                                    .trim(Trim::All) // Trim whitespace from all fields
                                    .from_reader(file);

                                for (line_num, result) in reader.records().enumerate() {
                                    match result {
                                        Ok(record) => {
                                            if record.len() != 20 {
                                                eprintln!(
                                                    "Invalid number of fields in line {}: expected 20, got {}",
                                                    line_num + 1,
                                                    record.len()
                                                );
                                                continue;
                                            }

                                            // Parse fields into appropriate types
                                            let parse_field = |s: &str| -> Result<f64, _> { s.parse::<f64>() };
                                            let parse_field_i32 = |s: &str| -> Result<i32, _> { s.parse::<i32>() };

                                            match (
                                                parse_field(&record[0]),
                                                parse_field(&record[1]),
                                                parse_field(&record[2]),
                                                parse_field(&record[3]),
                                                parse_field(&record[4]),
                                                parse_field(&record[5]),
                                                parse_field(&record[6]),
                                                parse_field(&record[7]),
                                                parse_field(&record[8]),
                                                parse_field(&record[9]),
                                                parse_field(&record[10]),
                                                parse_field(&record[11]),
                                                parse_field(&record[12]),
                                                parse_field(&record[13]),
                                                parse_field(&record[14]),
                                                parse_field(&record[15]),
                                                parse_field_i32(&record[16]),
                                                parse_field_i32(&record[17]),
                                                parse_field(&record[18]),
                                                parse_field(&record[19]),
                                                ) {
                                                    (
                                                        Ok(M),
                                                        Ok(delta),
                                                        Ok(t_0),
                                                        Ok(R),
                                                        Ok(beta_),
                                                        Ok(psi),
                                                        Ok(lambda0),
                                                        Ok(theta_),
                                                        Ok(phi_),
                                                        Ok(chi1),
                                                        Ok(theta_1),
                                                        Ok(phi_1),
                                                        Ok(chi2),
                                                        Ok(theta_2),
                                                        Ok(phi_2),
                                                        Ok(rho_0),
                                                        Ok(pn_order),
                                                        Ok(detectors),
                                                        Ok(delta_t),
                                                        Ok(duration),
                                                        ) => {
                                                            let test_case = TestCase::new(
                                                                M, delta, t_0, R, beta_, psi, lambda0, theta_, phi_, chi1,
                                                                theta_1, phi_1, chi2, theta_2, phi_2, rho_0, pn_order,
                                                                detectors, delta_t, duration,
                                                            );

                                                            self.runner.test_cases.push(test_case);
                                                            let mut data = self.runner.simulation_data.lock().unwrap();
                                                            data.push(None);
                                                        }
                                                    _ => {
                                                        eprintln!(
                                                            "Failed to parse line {}: {}",
                                                            line_num + 1,
                                                            record.iter().collect::<Vec<_>>().join(",")
                                                        );
                                                    }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error reading CSV record at line {}: {}", line_num + 1, e);
                                        }
                                    }
                                }
                            } else {
                                eprintln!("Failed to open file: {:?}", path);
                            }
                        }
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
                            for i in 0..self.runner.test_cases.len() {
                                ui.selectable_value(
                                    &mut self.selected_tab,
                                    i,
                                    format!("Test Case {}", i + 1),
                                );
                            }
                        });

                    ui.separator();

                    // Access simulation data safely using the mutex
                    let data = self.runner.simulation_data.lock().unwrap();
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

        if let Some(duration) = *self.runner.total_simulation_duration.lock().unwrap() {
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.label(format!("All simulations completed in {:.2?}", duration));
            });
        }

        ctx.request_repaint();
    }
}
