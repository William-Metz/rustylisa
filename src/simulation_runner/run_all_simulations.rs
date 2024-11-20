use crate::simulation_runner::simulation_runner::SimulationRunner;
use std::sync::{atomic::Ordering, Arc};
use std::time::Instant;

impl SimulationRunner {
    pub fn run_all_simulations(&self, savedata: bool) -> tokio::task::JoinHandle<()> {
        // Record the start time
        let simulations_start_time = Instant::now();
        let simulations_running = Arc::clone(&self.simulations_running);
        let total_simulation_duration = Arc::clone(&self.total_simulation_duration);

        for i in 0..self.test_cases.len() {
            self.simulations_running.fetch_add(1, Ordering::SeqCst);
            self.run_simulation(i, savedata);
        }

        // Return a handle to the monitoring task
        self.runtime.spawn(async move {
            // Wait until all simulations are complete
            while simulations_running.load(Ordering::SeqCst) > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }

            // Calculate and store the total duration once all are complete
            let duration = simulations_start_time.elapsed();
            *total_simulation_duration.lock().unwrap() = Some(duration);
        })
    }
}
