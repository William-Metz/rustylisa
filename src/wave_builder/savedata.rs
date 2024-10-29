use crate::wave_builder::wave_builder::WaveBuilder;
use crate::constants::YEAR;
use std::f64::consts::PI;


use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Serialize)]
pub struct DataPoint {
    time: f64,
    hp: f64,
    hx: f64,
    torb: f64,
}

impl WaveBuilder{
    pub fn save_to_csv(& self, data_point: &DataPoint ) {
        let file = File::create("data.csv").expect("Could not create file");
        let mut writer = BufWriter::new(file);

        // Write headers if the file is empty (you can check if the file exists)
        writeln!(writer, "Time,HP,HX,Torb,NSteps").expect("Could not write headers");

        // Write the data point
        writeln!(writer, "{},{},{},{},{}", data_point.time, data_point.hp, data_point.hx, data_point.torb, self.spin_evolver.test_case.n_steps)
            .expect("Could not write data point");
    }

    pub fn save_data(& self){
        //        tau_r*Parameters.GM/Parameters.Year;
        let t: f64 = self.spin_evolver.test_case.GM/YEAR;
        let omega: f64 = self.VDN*self.VDN*self.VDN/self.spin_evolver.test_case.GM;
        let torb: f64 = 2.0*PI/omega;

        let data_point = DataPoint {
            time: t,
            hp: self.HP,
            hx: self.HX,
            torb: torb,
        };
        println!("test");

        self.save_to_csv(&data_point);
    }
}

