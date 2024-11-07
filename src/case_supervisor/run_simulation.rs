use crate::case_supervisor::case_supervisor::CaseSupervisor;
impl CaseSupervisor {
    pub fn run_simulation(&mut self) {
        for n in 0..self.case.n_steps {
            self.delta_tau = (n as f64) * self.wave.delta_tau_r; //update

            if !self.wave.did_step_ok(n) {
                println!("Coalescence");
                println!("{}",self.case.R);
                println!("{}", self.case.r);
                self.save_to_csv();
                break;
            }
        }
    }
}
