use crate::case_supervisor::case_supervisor::CaseSupervisor;
use std::fs::File;

impl CaseSupervisor {
    pub fn save_parameters_to_json(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create file");
        serde_json::to_writer_pretty(file, &self.case).expect("Could not write parameters to JSON");
    }
}
