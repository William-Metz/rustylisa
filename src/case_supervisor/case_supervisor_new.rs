use crate::case_supervisor::case_supervisor::CaseSupervisor;
use crate::test_case::test_case::TestCase;
use crate::wave_builder::wave_builder::WaveBuilder;
impl CaseSupervisor {
    // Constructor function to initialize the struct
    pub fn new(case: TestCase) -> CaseSupervisor {
        //        let delta_tau_r = case.delta_t / case.GM;
        let delta_tau = 0.0;
        let wave = WaveBuilder::new(&case);
        CaseSupervisor {
            case,
            wave,
            delta_tau,
        }
    }
}
