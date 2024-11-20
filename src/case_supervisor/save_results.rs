use crate::case_supervisor::case_supervisor::CaseSupervisor;

impl CaseSupervisor {
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
