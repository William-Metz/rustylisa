// src/case_supervisor/case_supervisor.rs
use crate::test_case::test_case::TestCase;

pub struct CaseSupervisor {
    pub case: TestCase,
    pub delta_tau_r: f64,
}

impl CaseSupervisor {
    // Constructor function to initialize the struct
    pub fn new(case: TestCase) -> CaseSupervisor {
        let delta_tau_r = case.deltaT / case.M;
        CaseSupervisor { case, delta_tau_r }
    }
}
