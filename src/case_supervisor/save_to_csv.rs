use crate::case_supervisor::case_supervisor::CaseSupervisor;
use std::fs::File;
use std::io::{BufWriter, Write};

use serde_json::to_writer_pretty;

impl CaseSupervisor {
    pub fn save_to_json(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create file");
        to_writer_pretty(file, &self.wave.spin_evolver.data).expect("Could not write data to JSON");
    }
    pub fn save_results(&self, case_index: usize) -> std::io::Result<()> {
        let results_dir = format!("results/test_cases/case_{}/", case_index);
        std::fs::create_dir_all(&results_dir)?;

        // Save simulation data to JSON
        let data_path = format!("{}data.json", results_dir);
        self.save_to_json(&data_path);

        // Save parameters to JSON
        let params_path = format!("{}parameters.json", results_dir);
        let params_file = File::create(params_path)?;
        serde_json::to_writer_pretty(params_file, &self.case)?;

        Ok(())
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
}
