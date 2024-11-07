// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/test_case/new_test_case.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use crate::constants::{G, HUBBLECONSTANT, YEAR};
use crate::test_case::test_case::TestCase;
use core::f64;
use std::f64::consts::PI;

impl TestCase {
    pub fn new(
        M: f64,
        delta: f64,
        t_0: f64,
        R: f64,
        beta_: f64,
        psi: f64,
        lambda0: f64,
        theta_: f64,
        phi_: f64,
        chi1: f64,
        theta_1: f64,
        phi_1: f64,
        chi2: f64,
        theta_2: f64,
        phi_2: f64,
        rho_0: f64,
        pn_order: i32,
        detectors: i32,
        delta_t: f64,
        duration: f64,
    ) -> Self {
        let mut case = Self {
            //inputs
            M,
            delta,
            t_0,
            R,
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
            // To Be Calculated
            uncertainties: [0.0; 15],
            chi_10_x: None,
            chi_10_y: None,
            chi_10_z: None,
            chi_20_x: None,
            chi_20_y: None,
            chi_20_z: None,
            tau_c: 0.0,
            GM: 0.0,
            n_steps: 0,
        };
        // Convert angles from degrees to radians
        let radians_from_degrees = PI / 180.0;
        case.beta_ *= radians_from_degrees;
        case.psi *= radians_from_degrees;
        case.lambda0 *= radians_from_degrees;
        case.rho_0 *= radians_from_degrees;
        case.theta_ *= radians_from_degrees;
        case.phi_ *= radians_from_degrees;
        case.theta_1 *= radians_from_degrees;
        case.phi_1 *= radians_from_degrees;
        case.theta_2 *= radians_from_degrees;
        case.phi_2 *= radians_from_degrees;
        case.R *= YEAR;

        // Use imported constants G and GM_OMEGA_E
        case.GM = G * case.M; // G is the gravitational constant from the constants module
        let z: f64 = case.R * HUBBLECONSTANT;

        // Calculate v0 using imported G constant
        let v0 = ((case.GM * 2.0 * PI * (1.0 + z)) / case.t_0).powf(1.0 / 3.0);

        // Calculate η (eta)
        let eta = 0.25 * (1.0 - case.delta * case.delta);

        // Calculate Σℓ (Sigma ℓ)
        let sigma_l = 0.5
            * ((1.0 - case.delta) * case.chi2 * case.theta_2.cos()
                - (1.0 + case.delta) * case.chi1 * case.theta_1.cos());

        // Calculate Sℓ (S ℓ)
        let s_l = 0.25
            * ((1.0 + case.delta) * (1.0 + case.delta) * case.chi1 * case.theta_1.cos()
                + (1.0 - case.delta) * (1.0 - case.delta) * case.chi2 * case.theta_2.cos());

        // Calculate T2, T3, and T4
        let t2 = 32.0 / 3.0 * (743.0 / 2688.0 + 11.0 / 32.0 * eta);
        let t3 =
            64.0 / 3.0 * (47.0 / 40.0 * s_l + case.delta * 15.0 / 32.0 * sigma_l - 3.0 / 10.0 * PI);
        let t4 = 64.0 * (743.0 / 2688.0 + 11.0 / 32.0 * eta).powf(2.0)
            - 128.0 / 9.0
                * (1855099.0 / 14450688.0 + 56975.0 / 258048.0 * eta - 371.0 / 2048.0 * eta * eta);

        // Calculate τc (tau_c)

        case.tau_c = 5.0 / (256.0 * eta * v0.powf(8.0))
            * (1.0 + t2 * v0.powf(2.0) + t3 * v0.powf(3.0) + t4 * v0.powf(4.0));
        //Caclutate run duratation
        case.n_steps = (case.duration * YEAR / case.delta_t).round() as u64;

        //let mut v_c = VCalculator::new(case.tau_c,case.delta, 0.0, 0.0);
        //let v1 = v_c.v_at_time(0.0);
        case
    }
}
