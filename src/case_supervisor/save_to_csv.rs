use crate::case_supervisor::case_supervisor::CaseSupervisor;
use std::fs::File;
use std::io::{BufWriter, Write};

impl CaseSupervisor {
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
