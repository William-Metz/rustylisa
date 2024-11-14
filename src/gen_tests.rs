use crate::test_case::test_case::TestCase;
use rand::prelude::*;
use rand_distr::Uniform;

pub fn generate_test_cases(num_cases: usize) -> Vec<TestCase> {
    let mut test_cases = Vec::with_capacity(num_cases);
    let mut rng = rand::thread_rng();
    // Mass distribution (log-uniform for wide range)

    let min_mass: f32 = 2.0;
    let max_mass: f32 = 1_000_000.0;
    let log_min_mass = min_mass.ln();
    let log_max_mass = max_mass.ln();
    let log_mass_dist = Uniform::new(log_min_mass, log_max_mass);

    // Delta distribution (-0.9 to 0.9)
    let delta_dist = Uniform::new(-0.9, 0.9);

    // t_0 distribution (10 to 10,000)
    let t0_dist = Uniform::new(10.0, 10_000.0);

    // Spin magnitude distribution (0 to 1)
    let spin_dist = Uniform::new(0.0, 1.0);

    // pn_order distribution (0 to 3)
    let pn_order_dist = Uniform::new_inclusive(0, 3);

    // Angles distribution (if uniform over [0, 2π])
    let angle_dist = Uniform::new(0.0, std::f64::consts::TAU);

    // R distribution (log-uniform for wide range)
    let r_dist = Uniform::new_inclusive(2, 1_000_0000); //look into

    for _ in 0..num_cases {
        // Sample parameters

        let log_M = log_mass_dist.sample(&mut rng);
        let M = log_M.exp();
        let delta = delta_dist.sample(&mut rng);
        let t_0 = t0_dist.sample(&mut rng);
        let R = r_dist.sample(&mut rng);

        let chi1 = spin_dist.sample(&mut rng);
        let chi2 = spin_dist.sample(&mut rng);

        let beta_ = angle_dist.sample(&mut rng);
        let psi = angle_dist.sample(&mut rng);
        let lambda0 = angle_dist.sample(&mut rng);
        let theta_ = angle_dist.sample(&mut rng);
        let phi_ = angle_dist.sample(&mut rng);
        let theta_1 = angle_dist.sample(&mut rng);
        let phi_1 = angle_dist.sample(&mut rng);
        let theta_2 = angle_dist.sample(&mut rng);
        let phi_2 = angle_dist.sample(&mut rng);

        let pn_order = pn_order_dist.sample(&mut rng);

        // Constants (define these or sample if variable)
        let rho_0 = 0.0; //get back to
        let detectors = 1; //get back to
        let delta_t = 50.0;
        let duration = 10.0;

        // Create TestCase
        let test_case = TestCase::new(
            M.into(),
            delta,
            t_0,
            R.into(),
            beta_,
            psi,
            lambda0,
            theta_,
            phi_,
            chi1,
            theta_1,
            phi_1,
            chi2,
            theta_2,
            phi_2,
            rho_0,
            pn_order,
            detectors,
            delta_t,
            duration,
        );

        test_cases.push(test_case);
    }
    test_cases
}
