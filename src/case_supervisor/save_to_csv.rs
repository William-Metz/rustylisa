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
}
