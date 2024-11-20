use crate::simulation_runner::simulation_runner::SimulationRunner;
use crate::test_case::test_case::TestCase;
use tokio::runtime::Runtime;

use std::sync::{atomic::AtomicUsize, Arc, Mutex};

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
}
