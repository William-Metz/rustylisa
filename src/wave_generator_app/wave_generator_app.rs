use crate::test_case::test_case::TestCase;
use std::sync::Arc;
use std::time::Instant;
use tokio;
use tokio::runtime::Runtime;

use crate::simulation_runner::simulation_runner::SimulationRunner;
use crate::wave_generator_app::view::View;

pub struct WaveGeneratorApp {
    pub runner: SimulationRunner, // Replace individual fields with a runner
    pub selected_tab: usize,
    pub current_view: View,
    pub simulations_start_time: Option<Instant>,
}

impl Default for WaveGeneratorApp {
    fn default() -> Self {
        let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
        let test_cases = vec![TestCase::new(
            100000.0, 0.1, 500.0, 10000000.0, 39.0, 24.0, 0.0, 5.0, 268.5, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0, 2, 50.0, 1.0,
        )];
        let runner = SimulationRunner::new(test_cases, runtime);
        Self {
            runner,
            selected_tab: 0,
            current_view: View::Input,
            simulations_start_time: None,
        }
    }
}
