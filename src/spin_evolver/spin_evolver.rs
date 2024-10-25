use core::f64;
use crate::vcalculator::vcalculator::VCalculator;
use crate::test_case::test_case::TestCase;
use crate::spin_data::spin_data::SpinData;
use std::f64::consts::PI;
use libm::atan2;
#[derive(Clone,PartialEq,Debug)]
pub struct SpinEvolverClass {
    C10: f64,
    C12: f64,
    C14: f64,
    C20: f64,
    C22: f64,
    C24: f64,
    L2: f64,
    L3: f64,
    L4: f64,
    NalphaCycles: i32,
    Omdelta2I4: f64,
    Opdelta2I4: f64,
    P: TestCase,
    S_ell_: f64,
    V0: f64,
    VCalc: VCalculator,
    iota_N: f64,
    iota_P: f64,
    alphaN: f64,
    alphaP: f64,
    delta_tau_hF: f64,
    delta_tau_h_P: f64,
    nu: f64,
    simga_ell_: f64,
    tauN: f64,
    tauP: f64,
    chi1xN: f64,
    chi1xP: f64,
    chi1yN: f64,
    chi1yP: f64,
    chi1zN: f64,
    chi1zP: f64,
    chi1ell_: f64,
    chi2xN: f64,
    chi2xP: f64,
    chi2yN: f64,
    chi2yP: f64,
    chi2zN: f64,
    chi2zP: f64,
    chi2ell_: f64,
    psi_prN: f64,
    psi_prP: f64,
    ell_xN: f64,
    ell_xP: f64,
    ell_yN: f64,
    ell_yP: f64,
    ell_zN: f64,
    ell_zP: f64,
}

impl SpinEvolverClass {
    pub fn new(c_info: &TestCase) -> Self {
        let mut spin_evolver = Self {
            C10: 0.0,
            C12: 0.0,
            C14: 0.0,
            C20: 0.0,
            C22: 0.0,
            C24: 0.0,
            L2: 0.0,
            L3: 0.0,
            L4: 0.0,
            NalphaCycles: 0,
            Omdelta2I4: 0.0,
            Opdelta2I4: 0.0,
            P: c_info.clone(),
            S_ell_: 0.0,
            V0: 0.0,
            VCalc: VCalculator::new(0.0, 0.0, 0.0, 0.0),
            iota_N: 0.0,
            iota_P: 0.0,
            alphaN: 0.0,
            alphaP: 0.0,
            delta_tau_hF: 0.0,
            delta_tau_h_P: 0.0,
            nu: 0.0,
            simga_ell_: 0.0,
            tauN: 0.0,
            tauP: 0.0,
            chi1xN: 0.0,
            chi1xP: 0.0,
            chi1yN: 0.0,
            chi1yP: 0.0,
            chi1zN: 0.0,
            chi1zP: 0.0,
            chi1ell_: 0.0,
            chi2xN: 0.0,
            chi2xP: 0.0,
            chi2yN: 0.0,
            chi2yP: 0.0,
            chi2zN: 0.0,
            chi2zP: 0.0,
            chi2ell_: 0.0,
            psi_prN: 0.0,
            psi_prP: 0.0,
            ell_xN: 0.0,
            ell_xP: 0.0,
            ell_yN: 0.0,
            ell_yP: 0.0,
            ell_zN: 0.0,
            ell_zP: 0.0,
        };

        if c_info.chi1 == 0.0 && c_info.chi2 == 0.0 {
            // No spins, initialize VCalc and get V0 value
            spin_evolver.VCalc = VCalculator::new(c_info.tau_c, c_info.delta, 0.0, 0.0);
            spin_evolver.V0 = spin_evolver.VCalc.v_at_time(0.0);
        } else {
            // Initialize with spins, set up the spin coefficients and other calculations
            // Start with basic setup
            let nu = 0.25*(1.0 - c_info.delta*c_info.delta);
            let nu2: f64 = nu*nu;
            let nu3: f64 = nu2*nu;

            // Calculate spin evolution coefficients
            spin_evolver.C10 = 0.75*(1.0 - c_info.delta) + 0.5*nu;
            spin_evolver.C20 = 0.75*(1.0 + c_info.delta) + 0.5*nu;
            spin_evolver.C12 = 9.0/16.0 + 5.0/4.0*nu - nu2/24.0 + c_info.delta*(-9.0/16.0 + 5.0/8.0*nu);
            spin_evolver.C22 = 9.0/16.0 + 5.0/4.0*nu - nu2/24.0 - c_info.delta*(-9.0/16.0 + 5.0/8.0*nu);
            spin_evolver.C14 = 27.0/32.0 + 3.0/16.0*nu - 105.0/32.0*nu2 - nu3/48.0 + c_info.delta*(-27.0/32.0 + 39.0/8.0*nu -5.0/32.0*nu2);
            spin_evolver.C24 = 27./32.0 + 3.0/16.0*nu - 105.0/32.0*nu2 - nu3/48.0 - c_info.delta*(-27.0/32.0 + 39.0/8.0*nu -5.0/32.0*nu2);

            // Calculate orbital AM magnitude coefficients
            spin_evolver.Opdelta2I4 = 0.25*(1.0 + c_info.delta)*(1.0 + c_info.delta);
            spin_evolver.Omdelta2I4 = 0.25*(1.0 - c_info.delta)*(1.0 - c_info.delta);

            spin_evolver.chi1ell_ = c_info.chi1*((c_info.theta_1).cos());
            spin_evolver.chi2ell_ = c_info.chi2*((c_info.Theta2).cos());
            spin_evolver.S_ell_ = spin_evolver.Opdelta2I4*spin_evolver.chi1ell_ + spin_evolver.Omdelta2I4*spin_evolver.chi2ell_;
            spin_evolver.simga_ell_ = 0.5*(1.0 - c_info.delta)*spin_evolver.chi2ell_ - 0.5*(1.0 + c_info.delta)*spin_evolver.chi1ell_;
            spin_evolver.L2 = 3.0/2.0 + nu/6.0;
            spin_evolver.L3 = -35.0/6.0*spin_evolver.S_ell_-5.0/2.0*c_info.delta*spin_evolver.simga_ell_;
            spin_evolver.L4 = 27.0/8.0 - 19.0/8.0*nu + nu2/24.0;

            // Initialize the velocity calculator
            //VCalc = New VCalculatorClass(P.tauc, c_info.delta, chi1ell_, chi2ell_)

            // Calculate the initial orbital angular momentum magnitude
            spin_evolver.V0 = spin_evolver.VCalc.v_at_time(0.0);
            let mut v2: f64 = spin_evolver.V0*spin_evolver.V0;
            let mut v3: f64 = v2*spin_evolver.V0;
            let mut v4: f64 = v3*spin_evolver.V0;
            let mut v5: f64 = v4*spin_evolver.V0;
            let mut L0: f64 = nu/spin_evolver.V0*(1.0 + spin_evolver.L2*v2 + spin_evolver.L3*v3 + spin_evolver.L4*v4);

            // Calculate spin components in the precessing frame
            let chi1xL: f64 = spin_evolver.P.chi1*(spin_evolver.P.theta_1).sin()*(spin_evolver.P.Phi1).cos();
            let chi1yL: f64 = spin_evolver.P.chi1*(spin_evolver.P.theta_1).sin()*(spin_evolver.P.Phi1).sin();
            let chi1zL: f64 = spin_evolver.chi1ell_;
            let chi2xL: f64 = spin_evolver.P.chi2*(spin_evolver.P.Theta2).sin()*(spin_evolver.P.Phi2).cos();
            let chi2yL: f64 = spin_evolver.P.chi2*(spin_evolver.P.Theta2).sin()*(spin_evolver.P.Phi2).sin();
            let chi2zL: f64 = spin_evolver.chi2ell_;

            // Calculate total angular momentum components in the precessing frame
            let j0x: f64 = spin_evolver.Opdelta2I4*chi1xL + spin_evolver.Omdelta2I4*chi2xL;
            let j0y: f64 = spin_evolver.Opdelta2I4*chi1yL + spin_evolver.Omdelta2I4*chi2yL;
            let j0z: f64 = spin_evolver.Opdelta2I4*chi1zL + spin_evolver.Omdelta2I4*chi2zL + L0;

            // Calculate rotation matrix
            let θ0: f64 = atan2(j0z, (j0x*j0x+j0y*j0y).sqrt());
            let φ0: f64 = atan2(j0y, j0x);
            let rxx: f64 = (θ0).cos()*(φ0).cos();
            let rxy: f64 = (θ0).cos()*(φ0).sin();
            let rxz: f64 = -(θ0).sin();
            let ryx: f64 = -(φ0).sin();
            let ryy: f64 = (φ0).cos();
            let ryz: f64 = 0.0;
            let rzx: f64 = (θ0).sin()*(φ0).cos();
            let rzy: f64 = (θ0).sin()*(φ0).sin();
            let rzz: f64 = (θ0).cos();

            // Calculate initial values for the spin vector components
            spin_evolver.chi1xP = rxx*chi1xL + rxy*chi1yL  + rxz*chi1zL;
            spin_evolver.chi1yP = ryx*chi1xL + ryy*chi1yL  + ryz*chi1zL;
            spin_evolver.chi1zP = rzx*chi1xL + rzy*chi1yL  + rzz*chi1zL;
            spin_evolver.chi2xP = rxx*chi2xL + rxy*chi2yL  + rxz*chi2zL;
            spin_evolver.chi2yP = ryx*chi2xL + ryy*chi2yL  + ryz*chi2zL;
            spin_evolver.chi2zP = rzx*chi2xL + rzy*chi2yL  + rzz*chi2zL;

            // Calculate intial values for the orbital angular momentum components and angles
            spin_evolver.ell_xP = rxz*L0;
            spin_evolver.ell_yP = ryz*L0;
            spin_evolver.ell_zP = rzz*L0;
            spin_evolver.alphaP = atan2(spin_evolver.ell_yP,spin_evolver.ell_xP);
            spin_evolver.iota_P = atan2(spin_evolver.ell_zP,(spin_evolver.ell_xP*spin_evolver.ell_xP + spin_evolver.ell_yP*spin_evolver.ell_yP).sqrt());

            // Calculate components of the initial spin rate of change
            let mut omega_1: f64 = v5*(spin_evolver.C10 + spin_evolver.C12*v2 + spin_evolver.C14*v4)/L0;
            let chi1xDotP: f64 = omega_1*(spin_evolver.ell_yP*spin_evolver.chi1zP - spin_evolver.ell_zP*spin_evolver.chi1yP);
            let chi1yDotP: f64 = omega_1*(spin_evolver.ell_zP*spin_evolver.chi1xP - spin_evolver.ell_xP*spin_evolver.chi1zP);
            let chi1zDotP: f64 = omega_1*(spin_evolver.ell_xP*spin_evolver.chi1yP - spin_evolver.ell_yP*spin_evolver.chi1xP);
            let mut omega_2: f64 = v5*(spin_evolver.C20 + spin_evolver.C22*v2 + spin_evolver.C24*v4)/L0;
            let chi2xDotP: f64 = omega_2*(spin_evolver.ell_yP*spin_evolver.chi2zP - spin_evolver.ell_zP*spin_evolver.chi2yP);
            let chi2yDotP: f64 = omega_2*(spin_evolver.ell_zP*spin_evolver.chi2xP - spin_evolver.ell_xP*spin_evolver.chi2zP);
            let chi2zDotP: f64 = omega_2*(spin_evolver.ell_xP*spin_evolver.chi2yP - spin_evolver.ell_yP*spin_evolver.chi2xP);

            // Calculate the first time step to be half the step that would
            // take 628 steps for the fastest spin to precess once
            let s1dot: f64 = (chi1xDotP*chi1xDotP + chi1yDotP*chi1yDotP + chi1zDotP*chi1zDotP).sqrt();
            let s2dot: f64 = (chi2xDotP*chi2xDotP + chi2yDotP*chi2yDotP + chi2zDotP*chi2zDotP).sqrt();
            if s1dot == 0.0 {
                spin_evolver.delta_tau_h_P = spin_evolver.P.chi2/s2dot}
            else if s2dot == 0.0 {
                spin_evolver.delta_tau_h_P = spin_evolver.P.chi1/s1dot;
            }
            else{
                spin_evolver.delta_tau_h_P = 0.5*f64::min(spin_evolver.P.chi1/s1dot, spin_evolver.P.chi2/s2dot);
            }
            spin_evolver.delta_tau_hF = spin_evolver.delta_tau_h_P;

            // Evolve the spins using an Euler step
            spin_evolver.chi1xN = spin_evolver.chi1xP + spin_evolver.delta_tau_h_P*chi1xDotP;
            spin_evolver.chi1yN = spin_evolver.chi1yP + spin_evolver.delta_tau_h_P*chi1yDotP;
            spin_evolver.chi1zN = spin_evolver.chi1zP + spin_evolver.delta_tau_h_P*chi1zDotP;
            spin_evolver.chi2xN = spin_evolver.chi2xP + spin_evolver.delta_tau_h_P*chi2xDotP;
            spin_evolver.chi2yN = spin_evolver.chi2yP + spin_evolver.delta_tau_h_P*chi2yDotP;
            spin_evolver.chi2zN = spin_evolver.chi2zP + spin_evolver.delta_tau_h_P*chi2zDotP;

            // Evolve the orbital angular momentum using an Euler step
            let ell_DotP: f64 = nu*(-1.0/v2 + spin_evolver.L2 + 2.0*spin_evolver.L2*spin_evolver.V0 + 3.0*spin_evolver.L4*v2)*spin_evolver.VCalc.v_dot_for_last_v()/L0;
            let ell_xDotP: f64 = -spin_evolver.Opdelta2I4*chi1xDotP - spin_evolver.Omdelta2I4*chi2xDotP + ell_DotP*spin_evolver.ell_xP;
            let ell_yDotP: f64 = -spin_evolver.Opdelta2I4*chi1yDotP - spin_evolver.Omdelta2I4*chi2yDotP + ell_DotP*spin_evolver.ell_yP;
            let ell_zDotP: f64 = -spin_evolver.Opdelta2I4*chi1zDotP - spin_evolver.Omdelta2I4*chi2zDotP + ell_DotP*spin_evolver.ell_zP;
            spin_evolver.ell_xN = spin_evolver.ell_xP + spin_evolver.delta_tau_h_P*ell_xDotP;
            spin_evolver.ell_yN = spin_evolver.ell_yP + spin_evolver.delta_tau_h_P*ell_yDotP;
            spin_evolver.ell_zN = spin_evolver.ell_zP + spin_evolver.delta_tau_h_P*ell_zDotP;

            // To get a more precise estimate of the future values, iterate the calculation
            // Calculate the orbital angular momentum magnitude at the first time step
            let v1: f64 = spin_evolver.VCalc.v_at_time(spin_evolver.delta_tau_h_P);
            v2 = v1*v1;
            v3 = v2*v1;
            v4 = v3*v1;
            v5 = v4*v1;
            L0  = nu/v1*(1.0 + spin_evolver.L2*v2 + spin_evolver.L3*v3 + spin_evolver.L4*v4);

            // Calculate components of the future spin rate of change
            omega_1 = v5*(spin_evolver.C10 + spin_evolver.C12*v2 + spin_evolver.C14*v4)/L0;
            let chi1xDotN: f64 = omega_1*(spin_evolver.ell_yN*spin_evolver.chi1zN - spin_evolver.ell_zN*spin_evolver.chi1yN);
            let chi1yDotN: f64 = omega_1*(spin_evolver.ell_zN*spin_evolver.chi1xN - spin_evolver.ell_xN*spin_evolver.chi1zN);
            let chi1zDotN: f64 = omega_1*(spin_evolver.ell_xN*spin_evolver.chi1yN - spin_evolver.ell_yN*spin_evolver.chi1xN);
            omega_2 = v5*(spin_evolver.C20 + spin_evolver.C22*v2 + spin_evolver.C24*v4)/L0;
            let chi2xDotN: f64 = omega_2*(spin_evolver.ell_yN*spin_evolver.chi2zN - spin_evolver.ell_zN*spin_evolver.chi2yN);
            let chi2yDotN: f64 = omega_2*(spin_evolver.ell_zN*spin_evolver.chi2xN - spin_evolver.ell_xN*spin_evolver.chi2zN);
            let chi2zDotN: f64 = omega_2*(spin_evolver.ell_xN*spin_evolver.chi2yN - spin_evolver.ell_yN*spin_evolver.chi2xN);

            // Evolve the spins using a more correct step
            spin_evolver.chi1xN = spin_evolver.chi1xP + 0.5*spin_evolver.delta_tau_h_P*(chi1xDotP + chi1xDotN);
            spin_evolver.chi1yN = spin_evolver.chi1yP + 0.5*spin_evolver.delta_tau_h_P*(chi1yDotP + chi1yDotN);
            spin_evolver.chi1zN = spin_evolver.chi1zP + 0.5*spin_evolver.delta_tau_h_P*(chi1zDotP + chi1zDotN);
            spin_evolver.chi2xN = spin_evolver.chi2xP + 0.5*spin_evolver.delta_tau_h_P*(chi2xDotP + chi2xDotN);
            spin_evolver.chi2yN = spin_evolver.chi2yP + 0.5*spin_evolver.delta_tau_h_P*(chi2yDotP + chi2yDotN);
            spin_evolver.chi2zN = spin_evolver.chi2zP + 0.5*spin_evolver.delta_tau_h_P*(chi2zDotP + chi2zDotN);

            // Evolve the orbital angular momentum using a more correct step
            let ell_DotN: f64 = nu*(-1.0/v2 + spin_evolver.L2 + 2.0*spin_evolver.L2*spin_evolver.P.v0 + 3.0*spin_evolver.L4*v2)*spin_evolver.VCalc.v_dot_for_last_v()/L0;
            let ell_xDotN: f64 = -spin_evolver.Opdelta2I4*chi1xDotN - spin_evolver.Omdelta2I4*chi2xDotN + ell_DotP*spin_evolver.ell_xN;
            let ell_yDotN: f64 = -spin_evolver.Opdelta2I4*chi1yDotN - spin_evolver.Omdelta2I4*chi2yDotN + ell_DotP*spin_evolver.ell_yN;
            let ell_zDotN: f64 = -spin_evolver.Opdelta2I4*chi1zDotN - spin_evolver.Omdelta2I4*chi2zDotN + ell_DotP*spin_evolver.ell_zN;
            spin_evolver.ell_xN = spin_evolver.ell_xP + 0.5*spin_evolver.delta_tau_h_P*(ell_xDotN + ell_xDotP);
            spin_evolver.ell_yN = spin_evolver.ell_yP + 0.5*spin_evolver.delta_tau_h_P*(ell_yDotN + ell_yDotP);
            spin_evolver.ell_zN = spin_evolver.ell_zP + 0.5*spin_evolver.delta_tau_h_P*(ell_zDotN + ell_zDotP);
            spin_evolver.alphaN = atan2(spin_evolver.ell_yN,spin_evolver.ell_xN);
            spin_evolver.iota_N = atan2(spin_evolver.ell_zN,(spin_evolver.ell_xN*spin_evolver.ell_xN + spin_evolver.ell_yN*spin_evolver.ell_yN).sqrt());
            // Initialize the precession phase
            let alphaDotP: f64 = (spin_evolver.ell_yP*ell_xDotP - spin_evolver.ell_xP*ell_yDotP)/(ell_xDotP*ell_xDotP + ell_yDotP*ell_yDotP);
            let alphaDotN: f64 = (spin_evolver.ell_yN*ell_xDotN - spin_evolver.ell_xN*ell_yDotN)/(ell_xDotN*ell_xDotN + ell_yDotN*ell_yDotN);
            spin_evolver.psi_prP = 0.0;
            spin_evolver.psi_prN = -0.5*spin_evolver.delta_tau_h_P*(alphaDotP*(spin_evolver.iota_P).cos() + alphaDotN*(spin_evolver.iota_N).cos());

            // Check to see whether we have crossed the 2nd/3rd quadrant line
            if spin_evolver.ell_yN < 0.0 && spin_evolver.ell_yP > 0.0 {
                if (spin_evolver.ell_xN*spin_evolver.ell_yP - spin_evolver.ell_xP*spin_evolver.ell_yN)/(spin_evolver.ell_yP-spin_evolver.ell_yN) < 0.0 {
                    spin_evolver.NalphaCycles = 1;
                }
            }
            else if spin_evolver.ell_yN > 0.0 && spin_evolver.ell_yP < 0.0 {
                if (spin_evolver.ell_xN*spin_evolver.ell_yP - spin_evolver.ell_xP*spin_evolver.ell_yN)/(spin_evolver.ell_yP-spin_evolver.ell_yN) < 0.0{
                    spin_evolver.NalphaCycles = -1;
                }
            }

            else{
                spin_evolver.NalphaCycles = 0;
            }
            spin_evolver.alphaN = spin_evolver.alphaN + (spin_evolver.NalphaCycles as f64)*2.0*PI;

            // Finally, update the times
            spin_evolver.tauP = 0.0;
            spin_evolver.tauN = spin_evolver.delta_tau_h_P;
        }

        spin_evolver
    }
    // Placeholder for the actaul step function
    pub fn do_step_succeeded(&mut self) -> bool {
        // Placeholder for step success logic
        true
    }



    pub fn GetSpinDataAtTime(&mut self, p: &TestCase,tau: f64) -> Option<SpinData> {
        if p.chi1 == 0.0 && p.chi2 == 0.0 {
            // If we have no spins
            let mut data = SpinData::new();
            data.iota = 1.0;
            data.alpha = PI;
            data.chi_ax = 0.0;
            data.chi_ay = 0.0;
            data.chi_az = 0.0;
            data.chi_sx = 0.0;
            data.chi_sy = 0.0;
            data.chi_sz = 0.0;

            let v_for_tau =self.VCalc.v_at_time(tau);
            data.v = v_for_tau;
            data.psi = p.lambda0 +self.VCalc.psi_orb_for_last_v() - 6.0 * v_for_tau.powi(3) * (v_for_tau / self.V0).ln();
            return Some(data);
        } else {
            // If we have at least one nonzero spin, then we need to evolve
            // Cycle through steps until we get beyond the requested time
            while tau > self.tauN {
                if !self.do_step_succeeded() {
                    return None;
                }
            }

            // Interpolate data to pass on to the rest of the program
            let f_n = (tau - self.tauP) / self.delta_tau_h_P;
            let f_p = 1.0 - f_n;

            let mut data = SpinData::new();
            data.iota = f_n * self.iota_N + f_p * self.iota_P;
            data.alpha = f_n * self.alphaN + f_p * self.alphaP;
            data.chi_ax = 0.5 * (f_n * (self.chi1xN- self.chi2xN) + f_p * (self.chi1xP- self.chi2xP));
            data.chi_ay = 0.5 * (f_n * (self.chi1yN- self.chi2yN) + f_p * (self.chi1yP - self.chi2yP));
            data.chi_az = 0.5 * (f_n * (self.chi1zN - self.chi2zN) + f_p * (self.chi1zP - self.chi2zP));
            data.chi_sx = 0.5 * (f_n * (self.chi1xN + self.chi2xN) + f_p * (self.chi1xP + self.chi2xP));
            data.chi_sy = 0.5 * (f_n * (self.chi1yN + self.chi2yN) + f_p * (self.chi1yP + self.chi2yP));
            data.chi_sz = 0.5 * (f_n * (self.chi1zN + self.chi2zN) + f_p * (self.chi1zP + self.chi2zP));

            // Calculate phase at the current time
            let v_for_tau =self.VCalc.v_at_time(tau);
            data.v = v_for_tau;
            data.psi = p.lambda0 +self.VCalc.psi_orb_for_last_v() + f_n * self.psi_prN + f_p * self.psi_prP- 6.0 * v_for_tau.powi(3) * (v_for_tau / self.V0).ln();
            return Some(data);
        }
    }

}
