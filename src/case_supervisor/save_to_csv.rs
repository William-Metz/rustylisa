use crate::case_supervisor::case_supervisor::CaseSupervisor;
use std::fs::File;

impl CaseSupervisor {
    pub fn save_to_csv(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create file");
        let mut wtr = csv::Writer::from_writer(file);
        for data_point in &self.wave.spin_evolver.data {
            wtr.serialize(data_point)
                .expect("Could not write data to CSV");
        }
        wtr.flush().expect("Could not flush CSV writer");
    }

    pub fn save_parameters_to_json(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create file");
        serde_json::to_writer_pretty(file, &self.case).expect("Could not write parameters to JSON");
    }

    pub fn save_results(&self, case_index: usize) -> std::io::Result<()> {
        let results_dir = format!("results/test_cases/case_{}/", case_index);
        std::fs::create_dir_all(&results_dir)?;

        // Save simulation data to CSV
        let data_path = format!("{}data.csv", results_dir);
        self.save_to_csv(&data_path);

        // Save parameters to JSON
        let params_path = format!("{}parameters.json", results_dir);
        self.save_parameters_to_json(&params_path);

        Ok(())
    }
}
