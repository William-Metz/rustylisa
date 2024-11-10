// simulation_runner.rs
use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::data_point::DataPoint;
use crate::test_case::test_case::TestCase;
use tokio::runtime::Runtime;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

pub struct SimulationRunner {
    pub test_cases: Vec<TestCase>,
    pub simulation_data: Arc<Mutex<Vec<Option<Vec<DataPoint>>>>>,
    pub simulations_running: Arc<AtomicUsize>,
    pub runtime: Arc<Runtime>,
    pub total_simulation_duration: Arc<Mutex<Option<Duration>>>,
}

impl SimulationRunner {
    pub fn new(test_cases: Vec<TestCase>, runtime: Arc<Runtime>) -> Self {
        Self {
            test_cases: test_cases.clone(),
            simulation_data: Arc::new(Mutex::new(vec![None; test_cases.len()])),
            simulations_running: Arc::new(AtomicUsize::new(0)),
            runtime,
            total_simulation_duration: Arc::new(Mutex::new(None)),
        }
    }

    pub fn run_simulation(&self, case_index: usize) {
        let test_case = self.test_cases[case_index].clone();
        let simulation_data = Arc::clone(&self.simulation_data);
        let simulations_running = Arc::clone(&self.simulations_running);

        self.runtime.spawn(async move {
            let mut case_supervisor = CaseSupervisor::new(test_case);
            case_supervisor.run_simulation();

            let mut data = simulation_data.lock().unwrap();
            data[case_index] = Some(case_supervisor.wave.spin_evolver.data.clone());

            // Decrement the counter after each simulation completes
            simulations_running.fetch_sub(1, Ordering::SeqCst);
        });
    }

    pub fn run_all_simulations(&self) {
        // Record the start time
        let simulations_start_time = Instant::now();
        let simulations_running = Arc::clone(&self.simulations_running);
        let total_simulation_duration = Arc::clone(&self.total_simulation_duration);

        for i in 0..self.test_cases.len() {
            self.simulations_running.fetch_add(1, Ordering::SeqCst);
            self.run_simulation(i);
        }

        // Spawn a task to monitor when all simulations are complete
        self.runtime.spawn(async move {
            // Wait until all simulations are complete
            while simulations_running.load(Ordering::SeqCst) > 0 {
                tokio::task::yield_now().await; // Yield to avoid busy waiting
            }

            // Calculate and store the total duration once all are complete
            let duration = simulations_start_time.elapsed();
            *total_simulation_duration.lock().unwrap() = Some(duration);
        });
    }
}
