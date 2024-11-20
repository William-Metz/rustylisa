// simulation_runner.rs
use crate::data_point::DataPoint;
use crate::test_case::test_case::TestCase;
use tokio::runtime::Runtime;

use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use std::time::Duration;

pub struct SimulationRunner {
    pub test_cases: Vec<TestCase>,
    pub simulation_data: Arc<Mutex<Vec<Option<Vec<DataPoint>>>>>,
    pub simulations_running: Arc<AtomicUsize>,
    pub runtime: Arc<Runtime>,
    pub total_simulation_duration: Arc<Mutex<Option<Duration>>>,
}
