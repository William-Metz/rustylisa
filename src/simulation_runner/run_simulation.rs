use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::simulation_runner::simulation_runner::SimulationRunner;
use std::sync::{atomic::Ordering, Arc};

impl SimulationRunner {
    pub fn run_simulation(&self, case_index: usize, savedata: bool) {
        let test_case = self.test_cases[case_index].clone();
        let simulation_data = Arc::clone(&self.simulation_data);
        let simulations_running = Arc::clone(&self.simulations_running);

        self.runtime.spawn(async move {
            let mut case_supervisor = CaseSupervisor::new(test_case);
            case_supervisor.run_simulation();

            if savedata {
                if let Err(e) = case_supervisor.save_results(case_index) {
                    eprintln!("Failed to save results for case {}: {}", case_index, e);
                }
            }
            let mut data = simulation_data.lock().unwrap();
            data[case_index] = Some(case_supervisor.wave.spin_evolver.data.clone());

            // Decrement the counter after each simulation completes
            simulations_running.fetch_sub(1, Ordering::SeqCst);
        });
    }
}
