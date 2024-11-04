use crate::test_case::test_case::TestCase;
use crate::{constants::{YEAR, G, HUBBLECONSTANT}, vcalculator::vcalculator::VCalculator}; 
use std::f64::consts::PI;


impl TestCase{
    pub fn new(
        uncertainties: [f64; 15],
        beta: f64,
        psi: f64,
        lambda0: f64,
        rho0: f64,
        theta: f64,
        phi: f64,
        theta1: f64,
        phi1: f64,
        theta2: f64,
        phi2: f64,
        m: f64,          // Mass of the system passed in as an argument
        t0: f64,
        delta: f64,
        chi1: f64,
        chi2: f64,
        mut r: f64,
        omega_: f64,
        chi_10_x: Option<f64>,
        chi_10_y: Option<f64>,
        chi_10_z: Option<f64>,
        chi_20_x: Option<f64>,
        chi_20_y: Option<f64>,
        chi_20_z: Option<f64>,
        pn0: f64,
        pn_order: i32,
        detectors: i32,
        delta_t: f64,
        duration: f64,
    ) -> TestCase{

        // Convert angles from degrees to radians
        let radians_from_degrees = PI / 180.0;
        let beta = radians_from_degrees * beta;
        let psi = radians_from_degrees * psi;
        let lambda0 = radians_from_degrees * lambda0;
        let rho0 = radians_from_degrees * rho0;
        let theta = radians_from_degrees * theta;
        let phi = radians_from_degrees * phi;
        let theta1 = radians_from_degrees * theta1;
        let phi1 = radians_from_degrees * phi1;
        let theta2 = radians_from_degrees * theta2;
        let phi2 = radians_from_degrees * phi2;
        r = r*YEAR;

        // Use imported constants G and GM_OMEGA_E
        let gm = G * m; // G is the gravitational constant from the constants module
        let z: f64 = r*HUBBLECONSTANT; 

        // Calculate v0 using imported G constant
        let v0 = ((gm * 2.0 * PI * (1.0 + z)) / t0).powf(1.0 / 3.0);

        // Calculate η (eta)
        let eta = 0.25 * (1.0 - delta * delta);

        // Calculate Σℓ (Sigma ℓ)
        let sigma_l = 0.5 * ((1.0 - delta) * chi2 * theta2.cos() - (1.0 + delta) * chi1 * theta1.cos());

        // Calculate Sℓ (S ℓ)
        let s_l = 0.25 * ((1.0 + delta) * (1.0 + delta) * chi1 * theta1.cos() + (1.0 - delta) * (1.0 - delta) * chi2 * theta2.cos());

        // Calculate T2, T3, and T4
        let t2 = 32.0 / 3.0 * (743.0 / 2688.0 + 11.0 / 32.0 * eta);
        let t3 = 64.0 / 3.0 * (47.0 / 40.0 * s_l + delta * 15.0 / 32.0 * sigma_l - 3.0 / 10.0 * PI);
        let t4 = 64.0 * (743.0 / 2688.0 + 11.0 / 32.0 * eta).powf(2.0)
            - 128.0 / 9.0 * (1855099.0 / 14450688.0 + 56975.0 / 258048.0 * eta - 371.0 / 2048.0 * eta * eta);

        // Calculate τc (tau_c)
        
        let tau_c = 5.0 / (256.0 * eta * v0.powf(8.0)) * (1.0 + t2 * v0.powf(2.0) + t3 * v0.powf(3.0) + t4 * v0.powf(4.0));
        //Caclutate run duratation
        let nsteps= (duration*YEAR/delta_t).round() as u64;

        let mut v_c = VCalculator::new(tau_c, delta, 0.0, 0.0); 
        let v1 = v_c.v_at_time(0.0); 

        // Return the struct
        TestCase{
            uncertainties,
            beta_: beta,
            psi: psi,
            lambda0: lambda0,
            rho_0: rho0,
            theta_: theta,
            phi_: phi,
            theta_1: theta1,
            phi_1: phi1,
            theta_2: theta2,
            phi_2: phi2,
            M: m,           // Store the mass
            GM: gm,         // Store GM (G * M)
            tau_c,
            n_steps: nsteps,
            // Fields from TestCase
            delta,
            t_0: t0,
            R: r,
            omega_,
            chi_10_x,
            chi_10_y,
            chi_10_z,
            chi_20_x,
            chi_20_y,
            chi_20_z,
            p_n0: pn0,
            pn_order: pn_order,
            detectors,
            delta_t,
            duration,
            chi1,
            chi2,

        }
    }
}

