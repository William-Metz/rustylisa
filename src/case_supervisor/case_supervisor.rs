use crate::test_case::test_case::TestCase;
use crate::wave_builder::wave_builder::WaveBuilder;

pub struct CaseSupervisor {
    pub case: TestCase,
    pub wave: WaveBuilder,
    pub delta_tau: f64,
}
