use core::f64;
use crate::vcalculator::vcalculator::VCalculator;
use crate::test_case::test_case::TestCase;
use crate::spin_data::spin_data::SpinData;
use std::f64::consts::PI;
use libm::atan2;
#[derive(Clone,PartialEq,Debug)]
pub struct SpinEvolverClass {
    c_10: f64,
    c_12: f64,
    c_14: f64,
    c_20: f64,
    c_22: f64,
    c_24: f64,
    l_2: f64,
    l_3: f64,
    l_4: f64,
    n_alpha_cycles: i32,
    om_delta2_i4: f64,
    op_delta2_i4: f64,
    test_case: TestCase,
    s_ell_: f64,
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
    ell_N: f64,
    ell_P: f64,
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
            c_10: 0.0,
            c_12: 0.0,
            c_14: 0.0,
            c_20: 0.0,
            c_22: 0.0,
            c_24: 0.0,
            l_2: 0.0,
            l_3: 0.0,
            l_4: 0.0,
            n_alpha_cycles: 0,
            om_delta2_i4: 0.0,
            op_delta2_i4: 0.0,
            test_case: c_info.clone(),
            s_ell_: 0.0,
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
            ell_N: 0.0,
            ell_P: 0.0,
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
            spin_evolver.c_10 = 0.75*(1.0 - c_info.delta) + 0.5*nu;
            spin_evolver.c_20 = 0.75*(1.0 + c_info.delta) + 0.5*nu;
            spin_evolver.c_12 = 9.0/16.0 + 5.0/4.0*nu - nu2/24.0 + c_info.delta*(-9.0/16.0 + 5.0/8.0*nu);
            spin_evolver.c_22 = 9.0/16.0 + 5.0/4.0*nu - nu2/24.0 - c_info.delta*(-9.0/16.0 + 5.0/8.0*nu);
            spin_evolver.c_14 = 27.0/32.0 + 3.0/16.0*nu - 105.0/32.0*nu2 - nu3/48.0 + c_info.delta*(-27.0/32.0 + 39.0/8.0*nu -5.0/32.0*nu2);
            spin_evolver.c_24 = 27./32.0 + 3.0/16.0*nu - 105.0/32.0*nu2 - nu3/48.0 - c_info.delta*(-27.0/32.0 + 39.0/8.0*nu -5.0/32.0*nu2);

            // Calculate orbital AM magnitude coefficients
            spin_evolver.op_delta2_i4 = 0.25*(1.0 + c_info.delta)*(1.0 + c_info.delta);
            spin_evolver.om_delta2_i4 = 0.25*(1.0 - c_info.delta)*(1.0 - c_info.delta);

            spin_evolver.chi1ell_ = c_info.chi1*((c_info.theta_1).cos());
            spin_evolver.chi2ell_ = c_info.chi2*((c_info.theta_2).cos());
            spin_evolver.s_ell_ = spin_evolver.op_delta2_i4*spin_evolver.chi1ell_ + spin_evolver.om_delta2_i4*spin_evolver.chi2ell_;
            spin_evolver.simga_ell_ = 0.5*(1.0 - c_info.delta)*spin_evolver.chi2ell_ - 0.5*(1.0 + c_info.delta)*spin_evolver.chi1ell_;
            spin_evolver.l_2 = 3.0/2.0 + nu/6.0;
            spin_evolver.l_3 = -35.0/6.0*spin_evolver.s_ell_-5.0/2.0*c_info.delta*spin_evolver.simga_ell_;
            spin_evolver.l_4 = 27.0/8.0 - 19.0/8.0*nu + nu2/24.0;

            // Initialize the velocity calculator
            //VCalc = New VCalculatorClass(P.tauc, c_info.delta, chi1ell_, chi2ell_)

            // Calculate the initial orbital angular momentum magnitude
            spin_evolver.V0 = spin_evolver.VCalc.v_at_time(0.0);
            let mut v2: f64 = spin_evolver.V0*spin_evolver.V0;
            let mut v3: f64 = v2*spin_evolver.V0;
            let mut v4: f64 = v3*spin_evolver.V0;
            let mut v5: f64 = v4*spin_evolver.V0;
            let mut L0: f64 = nu/spin_evolver.V0*(1.0 + spin_evolver.l_2*v2 + spin_evolver.l_3*v3 + spin_evolver.l_4*v4);

            // Calculate spin components in the precessing frame
            let chi1xL: f64 = spin_evolver.test_case.chi1*(spin_evolver.test_case.theta_1).sin()*(spin_evolver.test_case.phi_1).cos();
            let chi1yL: f64 = spin_evolver.test_case.chi1*(spin_evolver.test_case.theta_1).sin()*(spin_evolver.test_case.phi_1).sin();
            let chi1zL: f64 = spin_evolver.chi1ell_;
            let chi2xL: f64 = spin_evolver.test_case.chi2*(spin_evolver.test_case.theta_2).sin()*(spin_evolver.test_case.phi_2).cos();
            let chi2yL: f64 = spin_evolver.test_case.chi2*(spin_evolver.test_case.theta_2).sin()*(spin_evolver.test_case.phi_2).sin();
            let chi2zL: f64 = spin_evolver.chi2ell_;

            // Calculate total angular momentum components in the precessing frame
            let j0x: f64 = spin_evolver.op_delta2_i4*chi1xL + spin_evolver.om_delta2_i4*chi2xL;
            let j0y: f64 = spin_evolver.op_delta2_i4*chi1yL + spin_evolver.om_delta2_i4*chi2yL;
            let j0z: f64 = spin_evolver.op_delta2_i4*chi1zL + spin_evolver.om_delta2_i4*chi2zL + L0;

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
            let mut omega_1: f64 = v5*(spin_evolver.c_10 + spin_evolver.c_12*v2 + spin_evolver.c_14*v4)/L0;
            let chi1xDotP: f64 = omega_1*(spin_evolver.ell_yP*spin_evolver.chi1zP - spin_evolver.ell_zP*spin_evolver.chi1yP);
            let chi1yDotP: f64 = omega_1*(spin_evolver.ell_zP*spin_evolver.chi1xP - spin_evolver.ell_xP*spin_evolver.chi1zP);
            let chi1zDotP: f64 = omega_1*(spin_evolver.ell_xP*spin_evolver.chi1yP - spin_evolver.ell_yP*spin_evolver.chi1xP);
            let mut omega_2: f64 = v5*(spin_evolver.c_20 + spin_evolver.c_22*v2 + spin_evolver.c_24*v4)/L0;
            let chi2xDotP: f64 = omega_2*(spin_evolver.ell_yP*spin_evolver.chi2zP - spin_evolver.ell_zP*spin_evolver.chi2yP);
            let chi2yDotP: f64 = omega_2*(spin_evolver.ell_zP*spin_evolver.chi2xP - spin_evolver.ell_xP*spin_evolver.chi2zP);
            let chi2zDotP: f64 = omega_2*(spin_evolver.ell_xP*spin_evolver.chi2yP - spin_evolver.ell_yP*spin_evolver.chi2xP);

            // Calculate the first time step to be half the step that would
            // take 628 steps for the fastest spin to precess once
            let s1dot: f64 = (chi1xDotP*chi1xDotP + chi1yDotP*chi1yDotP + chi1zDotP*chi1zDotP).sqrt();
            let s2dot: f64 = (chi2xDotP*chi2xDotP + chi2yDotP*chi2yDotP + chi2zDotP*chi2zDotP).sqrt();
            if s1dot == 0.0 {
                spin_evolver.delta_tau_h_P = spin_evolver.test_case.chi2/s2dot}
            else if s2dot == 0.0 {
                spin_evolver.delta_tau_h_P = spin_evolver.test_case.chi1/s1dot;
            }
            else{
                spin_evolver.delta_tau_h_P = 0.5*f64::min(spin_evolver.test_case.chi1/s1dot, spin_evolver.test_case.chi2/s2dot);
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
            let ell_DotP: f64 = nu*(-1.0/v2 + spin_evolver.l_2 + 2.0*spin_evolver.l_2*spin_evolver.V0 + 3.0*spin_evolver.l_4*v2)*spin_evolver.VCalc.v_dot_for_last_v()/L0;
            let ell_xDotP: f64 = -spin_evolver.op_delta2_i4*chi1xDotP - spin_evolver.om_delta2_i4*chi2xDotP + ell_DotP*spin_evolver.ell_xP;
            let ell_yDotP: f64 = -spin_evolver.op_delta2_i4*chi1yDotP - spin_evolver.om_delta2_i4*chi2yDotP + ell_DotP*spin_evolver.ell_yP;
            let ell_zDotP: f64 = -spin_evolver.op_delta2_i4*chi1zDotP - spin_evolver.om_delta2_i4*chi2zDotP + ell_DotP*spin_evolver.ell_zP;
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
            L0  = nu/v1*(1.0 + spin_evolver.l_2*v2 + spin_evolver.l_3*v3 + spin_evolver.l_4*v4);

            // Calculate components of the future spin rate of change
            omega_1 = v5*(spin_evolver.c_10 + spin_evolver.c_12*v2 + spin_evolver.c_14*v4)/L0;
            let chi1xDotN: f64 = omega_1*(spin_evolver.ell_yN*spin_evolver.chi1zN - spin_evolver.ell_zN*spin_evolver.chi1yN);
            let chi1yDotN: f64 = omega_1*(spin_evolver.ell_zN*spin_evolver.chi1xN - spin_evolver.ell_xN*spin_evolver.chi1zN);
            let chi1zDotN: f64 = omega_1*(spin_evolver.ell_xN*spin_evolver.chi1yN - spin_evolver.ell_yN*spin_evolver.chi1xN);
            omega_2 = v5*(spin_evolver.c_20 + spin_evolver.c_22*v2 + spin_evolver.c_24*v4)/L0;
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
            let ell_DotN: f64 = nu*(-1.0/v2 + spin_evolver.l_2 + 2.0*spin_evolver.l_2*spin_evolver.test_case.v0 + 3.0*spin_evolver.l_4*v2)*spin_evolver.VCalc.v_dot_for_last_v()/L0;
            let ell_xDotN: f64 = -spin_evolver.op_delta2_i4*chi1xDotN - spin_evolver.om_delta2_i4*chi2xDotN + ell_DotP*spin_evolver.ell_xN;
            let ell_yDotN: f64 = -spin_evolver.op_delta2_i4*chi1yDotN - spin_evolver.om_delta2_i4*chi2yDotN + ell_DotP*spin_evolver.ell_yN;
            let ell_zDotN: f64 = -spin_evolver.op_delta2_i4*chi1zDotN - spin_evolver.om_delta2_i4*chi2zDotN + ell_DotP*spin_evolver.ell_zN;
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
                    spin_evolver.n_alpha_cycles = 1;
                }
            }
            else if spin_evolver.ell_yN > 0.0 && spin_evolver.ell_yP < 0.0 {
                if (spin_evolver.ell_xN*spin_evolver.ell_yP - spin_evolver.ell_xP*spin_evolver.ell_yN)/(spin_evolver.ell_yP-spin_evolver.ell_yN) < 0.0{
                    spin_evolver.n_alpha_cycles = -1;
                }
            }

            else{
                spin_evolver.n_alpha_cycles = 0;
            }
            spin_evolver.alphaN = spin_evolver.alphaN + (spin_evolver.n_alpha_cycles as f64)*2.0*PI;

            // Finally, update the times
            spin_evolver.tauP = 0.0;
            spin_evolver.tauN = spin_evolver.delta_tau_h_P;
        }

        spin_evolver
    }
    pub fn AdjustThePast(&mut self){


        self.chi1xP = 0.5*(self.chi1xN + self.chi1xP);
        self.chi1yP = 0.5*(self.chi1yN + self.chi1yP);
        self.chi1zP = 0.5*(self.chi1zN + self.chi1zP);

        self.ell_xP = 0.5*(self.ell_xN + self.ell_xP);
        self.ell_yP = 0.5*(self.ell_yN + self.ell_yP);
        self.ell_zP = 0.5*(self.ell_zN + self.ell_zP);

        self.psi_prP= 0.5*(self.psi_prN+ self.psi_prP );


    }
    // Placeholder for the actaul step function
    pub fn do_step_succeeded(&mut self) -> bool {
        // Placeholder for step success logic
        // Check whether the step has been adjusted
        if self.delta_tau_h_P >self.delta_tau_hF{
            self.AdjustThePast();  // if so, adjust the past values
        }
        // Calculate the current orbital angular momentum magnitude
        let v1: f64 = self.VCalc.v_at_time(self.tauN);
        if v1 > 0.5 {
            return false; // if our speed is getting too high, bail out
        }
        let v2: f64 = v1*v1;
        let v3: f64 = v2*v1;
        let v4: f64 = v3*v1;
        let v5: f64 = v4*v1;
        let LN: f64 = self.nu/v1*(1.0 + self.l_2*v2 + self.l_3*v3 + self.l_4*v4);

        // let components of: f64 spin rate of change;
        let Omega_1: f64 = v5*(self.c_10 + self.c_12*v2 + self.c_14*v4)/LN;
        let chi1xDotN: f64 = Omega_1*(self.ell_yN*self.chi1zN - self.ell_zN*self.chi1yN);
        let chi1yDotN: f64 = Omega_1*(self.ell_zN*self.chi1xN - self.ell_xN*self.chi1zN);
        let chi1zDotN: f64 = Omega_1*(self.ell_xN*self.chi1yN - self.ell_yP*self.chi1xN);
        let Omega_2: f64 = v5*(self.c_20 + self.c_22*v2 + self.c_24*v4)/LN;
        let chi2xDotN: f64 = Omega_2*(self.ell_yN*self.chi2zN - self.ell_zN*self.chi2yN);
        let chi2yDotN: f64 = Omega_2*(self.ell_zN*self.chi2xN - self.ell_xN*self.chi2zN);
        let chi2zDotN: f64 = Omega_2*(self.ell_xN*self.chi2yN - self.ell_yN*self.chi2xN);

        // Evolve the spins using an leapfrog step
        let two_Delta_tau: f64 = 2.0*self.delta_tau_hF;
        let chi1xF: f64 = self.chi1xP + two_Delta_tau*chi1xDotN;
        let chi1yF: f64 = self.chi1yP + two_Delta_tau*chi1yDotN;
        let chi1zF: f64 = self.chi1zP + two_Delta_tau*chi1zDotN;
        let chi2xF: f64 = self.chi2xP + two_Delta_tau*chi2xDotN;
        let chi2yF: f64 = self.chi2yP + two_Delta_tau*chi2yDotN;
        let chi2zF: f64 = self.chi2zP + two_Delta_tau*chi2zDotN;

        // Evolve the orbital angular momentum using a leapfrog step
        let ell_DotN: f64 = self.nu*(-1.0/v2 + self.l_2 + 2.0*self.l_2*v1 + 3.0*self.l_4*v2)*self.VCalc.v_dot_for_last_v()/LN;
        let ell_xDotN: f64 = -self.op_delta2_i4*chi1xDotN - self.om_delta2_i4*chi2xDotN + ell_DotN*self.ell_xN;
        let ell_yDotN: f64 = -self.op_delta2_i4*chi1yDotN - self.om_delta2_i4*chi2yDotN + ell_DotN*self.ell_yN;
        let ell_zDotN: f64 = -self.op_delta2_i4*chi1zDotN - self.om_delta2_i4*chi2zDotN + ell_DotN*self.ell_zN;
        let ell_xF: f64 = self.ell_xP + two_Delta_tau*ell_xDotN;
        let ell_yF: f64 = self.ell_yP + two_Delta_tau*ell_yDotN;
        let ell_zF: f64 = self.ell_zP + two_Delta_tau*ell_zDotN;

        // Evolve the precession phase
        let alphaDotN: f64 = (self.ell_yN*ell_xDotN - self.ell_xN*ell_yDotN)/(ell_xDotN*ell_xDotN + ell_yDotN*ell_yDotN);
        let psi_prF: f64 = self.psi_prP- two_Delta_tau*alphaDotN*(self.ell_N).cos();

        // Check to see whether we have just crossed the 2nd/3rd quadrant line
        if ell_yF < 0.0 && self.ell_yN > 0.0 {
            if (ell_xF*self.ell_yN - self.ell_xN*ell_yF)/(self.ell_yN-ell_yF) < 0.0 {
                self.n_alpha_cycles = 1;
            }
        }
        else if ell_yF > 0.0 && self.ell_yN < 0.0{ 
                if (ell_xF*self.ell_yN - self.ell_xN*ell_yF)/(self.ell_yN-ell_yF) < 0.0 {
                    self.n_alpha_cycles = -1
            }
        }

        // From here on, the future step just calculated becomes the present step
        // and the present step becomes the past step
        self.tauP = self.tauN;
        self.tauN = self.tauN + self.delta_tau_hF;

        self.chi1xP = self.chi1xN;
        self.chi1yP = self.chi1yN;
        self.chi1zP = self.chi1zN;
        self.chi1xN = chi1xF;
        self.chi1yN = chi1yF;
        self.chi1zN = chi1zF;

        self.chi2xP = self.chi2xN;
        self.chi2yP = self.chi2yN;
        self.chi2zP = self.chi2zN;
        self.chi2xN = chi2xF;
        self.chi2yN = chi2yF;
        self.chi2zN = chi2zF;

        self.ell_xP = self.ell_xN;
        self.ell_yP = self.ell_yN;
        self.ell_zP = self.ell_zN;
        self.ell_xN = ell_xF;
        self.ell_yN = ell_yF;
        self.ell_zN = ell_zF;

        self.psi_prP= self.psi_prN;
        self.psi_prN= psi_prF;

    
        self.alphaP = self.alphaN;
        self.alphaN = atan2(self.ell_yN,self.ell_xN) + (self.n_alpha_cycles as f64)*2.0*PI;

        self.ell_P = self.ell_N;
        self.ell_N = atan2(self.ell_zN,(self.ell_xN*self.ell_xN + self.ell_yN*self.ell_yN).sqrt());

        // Calculate the ideal next time step
        let s1dot: f64 = (chi1xDotN*chi1xDotN + chi1yDotN*chi1yDotN + chi1zDotN*chi1zDotN).sqrt();
        let s2dot: f64 = (chi2xDotN*chi2xDotN + chi2yDotN*chi2yDotN + chi2zDotN*chi2zDotN).sqrt();
        let Delta_tau: f64 = f64::min((self.test_case.chi1/s1dot), self.test_case.chi2/s2dot);
        if Delta_tau < self.delta_tau_hF {// if we need a smaller step
            if self.delta_tau_h_P > self.delta_tau_hF{ 
                return false; // if we just did a smaller step, we are breaking down, so quit
            }
            else{
                self.delta_tau_h_P = self.delta_tau_hF;// store the previous step
                self.delta_tau_hF =self.delta_tau_hF/2.0;  // reduce the next step size by two
            }
        }
        else{ // if we don't need a smaller step, repeat the current step
            self.delta_tau_h_P = self.delta_tau_hF;
        }
        true
    }



pub fn GetSpinDataAtTime(&mut self, p: &TestCase,tau: f64) -> Option<SpinData> {
    println!("{}",p.chi1);
    if p.chi1 == 0.0 && p.chi2 == 0.0 {
        // If we have no spins
        let mut data = SpinData::new();
        println!("tset");
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
