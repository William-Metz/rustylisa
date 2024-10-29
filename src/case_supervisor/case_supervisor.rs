// src/case_supervisor/case_supervisor.rs
use crate::test_case::test_case::TestCase;
use crate::wave_builder::wave_builder::WaveBuilder;
use crate::constants::YEAR;
pub struct CaseSupervisor {
    pub case: TestCase,
    pub wave: WaveBuilder,
    pub delta_tau: f64,
}

impl CaseSupervisor {
    // Constructor function to initialize the struct
    pub fn new(case: TestCase) -> CaseSupervisor {
        let delta_tau_r = case.delta_t / case.GM;
        println!("{}",delta_tau_r);
        let delta_tau = 0.0; 
        let wave = WaveBuilder::new(&case);
        CaseSupervisor { case, wave, delta_tau}

    }
    pub fn run_simulation(mut self){


        for n in 0.. self.case.n_steps{
            self.delta_tau = (n as f64)*self.wave.delta_tau_r; //update
                                                               //            println!("{}", (self.delta_tau*self.case.GM/YEAR));

            if ! self.wave.did_step_ok(n,&self.case){
                println!("{}", (self.delta_tau*self.case.GM/YEAR));

                println!("Colences");
                break;

            }

            //   wave.print_differences(&last_wave);



        }


    }
}
