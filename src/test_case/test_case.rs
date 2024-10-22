use core::f64;

// src/test_case/test_case.rs 

use crate::constants::{YEAR, G, VE, G_OMEGA_E}; // Importing the constants
use std::{f64::consts::PI};

pub struct TestCase{
    // Fields from Case_Info
    pub Uncertainties: [f64; 15],
    pub beta_: f64,
    pub psi: f64,
    pub lambda0: f64,
    pub rho_0: f64,
    pub theta_: f64,
    pub phi_: f64,
    pub theta_1: f64,
    pub Phi1: f64,
    pub Theta2: f64,
    pub Phi2: f64,
    pub v0: f64,
    pub M: f64,         // Mass of the system in solar masses
    pub GM: f64,        // Gravitational constant * mass
    pub Simga_el: f64,
    pub S_el: f64,
    pub T2: f64,
    pub T3: f64,
    pub T4: f64,
    pub tau_c: f64,
    pub vC: f64,
    pub v1: f64,
    pub NSteps: u8,

    // Fields from TestCase
    pub delta: f64,
    pub T0: f64,
    pub R: f64,
    pub omega_: f64,
    pub chi_10_x: Option<f64>,
    pub chi_10_y: Option<f64>,
    pub chi_10_z: Option<f64>,
    pub chi_20_x: Option<f64>,
    pub chi_20_y: Option<f64>,
    pub chi_20_z: Option<f64>,
    pub Pn0: f64,
    pub detectors: i32,
    pub deltaT: f64,
    pub duration: f64,
}

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
        r: f64,
        omega_: f64,
        chi_10_x: Option<f64>,
        chi_10_y: Option<f64>,
        chi_10_z: Option<f64>,
        chi_20_x: Option<f64>,
        chi_20_y: Option<f64>,
        chi_20_z: Option<f64>,
        pn0: f64,
        detectors: i32,
        deltaT: f64,
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

        // Use imported constants G and GM_OMEGA_E
        let gm = G * m; // G is the gravitational constant from the constants module
        let z: f64 = 1.0; // Placeholder for Z(R), needs a Universe class or method

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
        let nsteps= (duration*YEAR/deltaT).round() as u8;

        // Placeholder for vC and v1, you may need to implement a class or function to calculate these
        let vC = 0.0; // Placeholder for vC calculation
        let v1 = 0.0; // Placeholder for v1 calculation

        // Return the struct
        TestCase{
            Uncertainties: uncertainties,
            beta_: beta,
            psi: psi,
            lambda0: lambda0,
            rho_0: rho0,
            theta_: theta,
            phi_: phi,
            theta_1: theta1,
            Phi1: phi1,
            Theta2: theta2,
            Phi2: phi2,
            v0,
            M: m,           // Store the mass
            GM: gm,         // Store GM (G * M)
            Simga_el: sigma_l,
            S_el: s_l,
            T2: t2,
            T3: t3,
            T4: t4,
            tau_c,
            vC,
            v1,
            NSteps: nsteps,
            // Fields from TestCase
            delta,
            T0: t0,
            R: r,
            omega_,
            chi_10_x,
            chi_10_y,
            chi_10_z,
            chi_20_x,
            chi_20_y,
            chi_20_z,
            Pn0: pn0,
            detectors,
            deltaT,
            duration,
        }
    }
}

