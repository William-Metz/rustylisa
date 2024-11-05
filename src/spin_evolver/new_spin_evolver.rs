use crate::spin_evolver::spin_evolver::SpinEvolverClass;
use core::f64;
use crate::vcalculator::vcalculator::VCalculator;
use crate::test_case::test_case::TestCase;
use std::f64::consts::PI;
use libm::atan2;
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
            data: Vec::new(),
            test_case: c_info.clone(),
            s_ell_: 0.0,
            v_0: 0.0,
            v_calc: VCalculator::new(c_info.tau_c, c_info.delta, 0.0, 0.0),
            iota_n: 0.0,
            iota_p: 0.0,
            alpha_n: 0.0,
            alpha_p: 0.0,
            delta_tau_hf: 0.0,
            delta_tau_hp: 0.0,
            eta: 0.0,
            simga_ell_: 0.0,
            tau_n: 0.0,
            tau_p: 0.0,
            chi1x_n: 0.0,
            chi1x_p: 0.0,
            chi1y_n: 0.0,
            chi1y_p: 0.0,
            chi1z_n: 0.0,
            chi1z_p: 0.0,
            chi1ell_: 0.0,
            chi2x_n: 0.0,
            chi2x_p: 0.0,
            chi2y_n: 0.0,
            chi2y_p: 0.0,
            chi2z_n: 0.0,
            chi2z_p: 0.0,
            chi2ell_: 0.0,
            psi_pr_n: 0.0,
            psi_pr_p: 0.0,
            ell_n: 0.0,
            ell_p: 0.0,
            ell_x_n: 0.0,
            ell_x_p: 0.0,
            ell_y_n: 0.0,
            ell_y_p: 0.0,
            ell_z_n: 0.0,
            ell_z_p: 0.0,
        };

        if c_info.chi1 == 0.0 && c_info.chi2 == 0.0 {
            // No spins, initialize VCalc and get V0 value
            spin_evolver.v_0 = spin_evolver.v_calc.v_at_time(0.0);
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
            spin_evolver.v_calc = VCalculator::new(c_info.tau_c, c_info.delta, spin_evolver.chi1ell_, spin_evolver.chi2ell_);

            // Calculate the initial orbital angular momentum magnitude
            spin_evolver.v_0 = spin_evolver.v_calc.v_at_time(0.0);
            let mut v2: f64 = spin_evolver.v_0*spin_evolver.v_0;
            let mut v3: f64 = v2*spin_evolver.v_0;
            let mut v4: f64 = v3*spin_evolver.v_0;
            let mut v5: f64 = v4*spin_evolver.v_0;
            let mut l_0: f64 = nu/spin_evolver.v_0*(1.0 + spin_evolver.l_2*v2 + spin_evolver.l_3*v3 + spin_evolver.l_4*v4);

            // Calculate spin components in the precessing frame
            let chi1x_l: f64 = spin_evolver.test_case.chi1*(spin_evolver.test_case.theta_1).sin()*(spin_evolver.test_case.phi_1).cos();
            let chi1y_l: f64 = spin_evolver.test_case.chi1*(spin_evolver.test_case.theta_1).sin()*(spin_evolver.test_case.phi_1).sin();
            let chi1z_l: f64 = spin_evolver.chi1ell_;
            let chi2x_l: f64 = spin_evolver.test_case.chi2*(spin_evolver.test_case.theta_2).sin()*(spin_evolver.test_case.phi_2).cos();
            let chi2y_l: f64 = spin_evolver.test_case.chi2*(spin_evolver.test_case.theta_2).sin()*(spin_evolver.test_case.phi_2).sin();
            let chi2z_l: f64 = spin_evolver.chi2ell_;

            // Calculate total angular momentum components in the precessing frame
            let j0x: f64 = spin_evolver.op_delta2_i4*chi1x_l + spin_evolver.om_delta2_i4*chi2x_l;
            let j0y: f64 = spin_evolver.op_delta2_i4*chi1y_l + spin_evolver.om_delta2_i4*chi2y_l;
            let j0z: f64 = spin_evolver.op_delta2_i4*chi1z_l + spin_evolver.om_delta2_i4*chi2z_l + l_0;

            // Calculate rotation matrix
            let theta_0: f64 = atan2(j0z, (j0x*j0x+j0y*j0y).sqrt());
            let phi_0: f64 = atan2(j0y, j0x);
            let rxx: f64 = (theta_0).cos()*(phi_0).cos();
            let rxy: f64 = (theta_0).cos()*(phi_0).sin();
            let rxz: f64 = -(theta_0).sin();
            let ryx: f64 = -(phi_0).sin();
            let ryy: f64 = (phi_0).cos();
            let ryz: f64 = 0.0;
            let rzx: f64 = (theta_0).sin()*(phi_0).cos();
            let rzy: f64 = (theta_0).sin()*(phi_0).sin();
            let rzz: f64 = (theta_0).cos();

            // Calculate initial values for the spin vector components
            spin_evolver.chi1x_p = rxx*chi1x_l + rxy*chi1y_l  + rxz*chi1z_l;
            spin_evolver.chi1y_p = ryx*chi1x_l + ryy*chi1y_l  + ryz*chi1z_l;
            spin_evolver.chi1z_p = rzx*chi1x_l + rzy*chi1y_l  + rzz*chi1z_l;
            spin_evolver.chi2x_p = rxx*chi2x_l + rxy*chi2y_l  + rxz*chi2z_l;
            spin_evolver.chi2y_p = ryx*chi2x_l + ryy*chi2y_l  + ryz*chi2z_l;
            spin_evolver.chi2z_p = rzx*chi2x_l + rzy*chi2y_l  + rzz*chi2z_l;

            // Calculate intial values for the orbital angular momentum components and angles
            spin_evolver.ell_x_p = rxz*l_0;
            spin_evolver.ell_y_p = ryz*l_0;
            spin_evolver.ell_z_p = rzz*l_0;
            spin_evolver.alpha_p = atan2(spin_evolver.ell_y_p,spin_evolver.ell_x_p);
            spin_evolver.iota_p = atan2(spin_evolver.ell_z_p,(spin_evolver.ell_x_p*spin_evolver.ell_x_p + spin_evolver.ell_y_p*spin_evolver.ell_y_p).sqrt());

            // Calculate components of the initial spin rate of change
            let mut omega_1: f64 = v5*(spin_evolver.c_10 + spin_evolver.c_12*v2 + spin_evolver.c_14*v4)/l_0;
            let chi1x_dot_p: f64 = omega_1*(spin_evolver.ell_y_p*spin_evolver.chi1z_p - spin_evolver.ell_z_p*spin_evolver.chi1y_p);
            let chi1y_dot_p: f64 = omega_1*(spin_evolver.ell_z_p*spin_evolver.chi1x_p - spin_evolver.ell_x_p*spin_evolver.chi1z_p);
            let chi1z_dot_p: f64 = omega_1*(spin_evolver.ell_x_p*spin_evolver.chi1y_p - spin_evolver.ell_y_p*spin_evolver.chi1x_p);
            let mut omega_2: f64 = v5*(spin_evolver.c_20 + spin_evolver.c_22*v2 + spin_evolver.c_24*v4)/l_0;
            let chi2x_dot_p: f64 = omega_2*(spin_evolver.ell_y_p*spin_evolver.chi2z_p - spin_evolver.ell_z_p*spin_evolver.chi2y_p);
            let chi2y_dot_p: f64 = omega_2*(spin_evolver.ell_z_p*spin_evolver.chi2x_p - spin_evolver.ell_x_p*spin_evolver.chi2z_p);
            let chi2z_dot_p: f64 = omega_2*(spin_evolver.ell_x_p*spin_evolver.chi2y_p - spin_evolver.ell_y_p*spin_evolver.chi2x_p);

            // Calculate the first time step to be half the step that would
            // take 628 steps for the fastest spin to precess once
            let s1dot: f64 = (chi1x_dot_p*chi1x_dot_p + chi1y_dot_p*chi1y_dot_p + chi1z_dot_p*chi1z_dot_p).sqrt();
            let s2dot: f64 = (chi2x_dot_p*chi2x_dot_p + chi2y_dot_p*chi2y_dot_p + chi2z_dot_p*chi2z_dot_p).sqrt();
            if s1dot == 0.0 {
                spin_evolver.delta_tau_hp = spin_evolver.test_case.chi2/s2dot}
            else if s2dot == 0.0 {
                spin_evolver.delta_tau_hp = spin_evolver.test_case.chi1/s1dot;
            }
            else{
                spin_evolver.delta_tau_hp = 0.5*f64::min(spin_evolver.test_case.chi1/s1dot, spin_evolver.test_case.chi2/s2dot);
            }
            spin_evolver.delta_tau_hf = spin_evolver.delta_tau_hp;

            // Evolve the spins using an Euler step
            spin_evolver.chi1x_n = spin_evolver.chi1x_p + spin_evolver.delta_tau_hp*chi1x_dot_p;
            spin_evolver.chi1y_n = spin_evolver.chi1y_p + spin_evolver.delta_tau_hp*chi1y_dot_p;
            spin_evolver.chi1z_n = spin_evolver.chi1z_p + spin_evolver.delta_tau_hp*chi1z_dot_p;
            spin_evolver.chi2x_n = spin_evolver.chi2x_p + spin_evolver.delta_tau_hp*chi2x_dot_p;
            spin_evolver.chi2y_n = spin_evolver.chi2y_p + spin_evolver.delta_tau_hp*chi2y_dot_p;
            spin_evolver.chi2z_n = spin_evolver.chi2z_p + spin_evolver.delta_tau_hp*chi2z_dot_p;

            // Evolve the orbital angular momentum using an Euler step
            let ell_dot_p: f64 = nu*(-1.0/v2 + spin_evolver.l_2 + 2.0*spin_evolver.l_2*spin_evolver.v_0 + 3.0*spin_evolver.l_4*v2)*spin_evolver.v_calc.v_dot_for_last_v()/l_0;
            let ell_x_dot_p: f64 = -spin_evolver.op_delta2_i4*chi1x_dot_p - spin_evolver.om_delta2_i4*chi2x_dot_p + ell_dot_p*spin_evolver.ell_x_p;
            let ell_y_dot_p: f64 = -spin_evolver.op_delta2_i4*chi1y_dot_p - spin_evolver.om_delta2_i4*chi2y_dot_p + ell_dot_p*spin_evolver.ell_y_p;
            let ell_z_dot_p: f64 = -spin_evolver.op_delta2_i4*chi1z_dot_p - spin_evolver.om_delta2_i4*chi2z_dot_p + ell_dot_p*spin_evolver.ell_z_p;
            spin_evolver.ell_x_n = spin_evolver.ell_x_p + spin_evolver.delta_tau_hp*ell_x_dot_p;
            spin_evolver.ell_y_n = spin_evolver.ell_y_p + spin_evolver.delta_tau_hp*ell_y_dot_p;
            spin_evolver.ell_z_n = spin_evolver.ell_z_p + spin_evolver.delta_tau_hp*ell_z_dot_p;

            // To get a more precise estimate of the future values, iterate the calculation
            // Calculate the orbital angular momentum magnitude at the first time step
            let v1: f64 = spin_evolver.v_calc.v_at_time(spin_evolver.delta_tau_hp);
            v2 = v1*v1;
            v3 = v2*v1;
            v4 = v3*v1;
            v5 = v4*v1;
            l_0  = nu/v1*(1.0 + spin_evolver.l_2*v2 + spin_evolver.l_3*v3 + spin_evolver.l_4*v4);

            // Calculate components of the future spin rate of change
            omega_1 = v5*(spin_evolver.c_10 + spin_evolver.c_12*v2 + spin_evolver.c_14*v4)/l_0;
            let chi1x_dot_n: f64 = omega_1*(spin_evolver.ell_y_n*spin_evolver.chi1z_n - spin_evolver.ell_z_n*spin_evolver.chi1y_n);
            let chi1y_dot_n: f64 = omega_1*(spin_evolver.ell_z_n*spin_evolver.chi1x_n - spin_evolver.ell_x_n*spin_evolver.chi1z_n);
            let chi1z_dot_n: f64 = omega_1*(spin_evolver.ell_x_n*spin_evolver.chi1y_n - spin_evolver.ell_y_n*spin_evolver.chi1x_n);
            omega_2 = v5*(spin_evolver.c_20 + spin_evolver.c_22*v2 + spin_evolver.c_24*v4)/l_0;
            let chi2x_dot_n: f64 = omega_2*(spin_evolver.ell_y_n*spin_evolver.chi2z_n - spin_evolver.ell_z_n*spin_evolver.chi2y_n);
            let chi2y_dot_n: f64 = omega_2*(spin_evolver.ell_z_n*spin_evolver.chi2x_n - spin_evolver.ell_x_n*spin_evolver.chi2z_n);
            let chi2z_dot_n: f64 = omega_2*(spin_evolver.ell_x_n*spin_evolver.chi2y_n - spin_evolver.ell_y_n*spin_evolver.chi2x_n);

            // Evolve the spins using a more correct step
            spin_evolver.chi1x_n = spin_evolver.chi1x_p + 0.5*spin_evolver.delta_tau_hp*(chi1x_dot_p + chi1x_dot_n);
            spin_evolver.chi1y_n = spin_evolver.chi1y_p + 0.5*spin_evolver.delta_tau_hp*(chi1y_dot_p + chi1y_dot_n);
            spin_evolver.chi1z_n = spin_evolver.chi1z_p + 0.5*spin_evolver.delta_tau_hp*(chi1z_dot_p + chi1z_dot_n);
            spin_evolver.chi2x_n = spin_evolver.chi2x_p + 0.5*spin_evolver.delta_tau_hp*(chi2x_dot_p + chi2x_dot_n);
            spin_evolver.chi2y_n = spin_evolver.chi2y_p + 0.5*spin_evolver.delta_tau_hp*(chi2y_dot_p + chi2y_dot_n);
            spin_evolver.chi2z_n = spin_evolver.chi2z_p + 0.5*spin_evolver.delta_tau_hp*(chi2z_dot_p + chi2z_dot_n);

            // Evolve the orbital angular momentum using a more correct step
            let ell_x_dot_n: f64 = -spin_evolver.op_delta2_i4*chi1x_dot_n - spin_evolver.om_delta2_i4*chi2x_dot_n + ell_dot_p*spin_evolver.ell_x_n;
            let ell_y_dot_n: f64 = -spin_evolver.op_delta2_i4*chi1y_dot_n - spin_evolver.om_delta2_i4*chi2y_dot_n + ell_dot_p*spin_evolver.ell_y_n;
            let ell_z_dot_n: f64 = -spin_evolver.op_delta2_i4*chi1z_dot_n - spin_evolver.om_delta2_i4*chi2z_dot_n + ell_dot_p*spin_evolver.ell_z_n;
            spin_evolver.ell_x_n = spin_evolver.ell_x_p + 0.5*spin_evolver.delta_tau_hp*(ell_x_dot_n + ell_x_dot_p);
            spin_evolver.ell_y_n = spin_evolver.ell_y_p + 0.5*spin_evolver.delta_tau_hp*(ell_y_dot_n + ell_y_dot_p);
            spin_evolver.ell_z_n = spin_evolver.ell_z_p + 0.5*spin_evolver.delta_tau_hp*(ell_z_dot_n + ell_z_dot_p);
            spin_evolver.alpha_n = atan2(spin_evolver.ell_y_n,spin_evolver.ell_x_n);
            spin_evolver.iota_n = atan2(spin_evolver.ell_z_n,(spin_evolver.ell_x_n*spin_evolver.ell_x_n + spin_evolver.ell_y_n*spin_evolver.ell_y_n).sqrt());
            // Initialize the precession phase
            let alpha_dot_p: f64 = (spin_evolver.ell_y_p*ell_x_dot_p - spin_evolver.ell_x_p*ell_y_dot_p)/(ell_x_dot_p*ell_x_dot_p + ell_y_dot_p*ell_y_dot_p);
            let alpha_dot_n: f64 = (spin_evolver.ell_y_n*ell_x_dot_n - spin_evolver.ell_x_n*ell_y_dot_n)/(ell_x_dot_n*ell_x_dot_n + ell_y_dot_n*ell_y_dot_n);
            spin_evolver.psi_pr_p = 0.0;
            spin_evolver.psi_pr_n = -0.5*spin_evolver.delta_tau_hp*(alpha_dot_p*(spin_evolver.iota_p).cos() + alpha_dot_n*(spin_evolver.iota_n).cos());

            // Check to see whether we have crossed the 2nd/3rd quadrant line
            if spin_evolver.ell_y_n < 0.0 && spin_evolver.ell_y_p > 0.0 {
                if (spin_evolver.ell_x_n*spin_evolver.ell_y_p - spin_evolver.ell_x_p*spin_evolver.ell_y_n)/(spin_evolver.ell_y_p-spin_evolver.ell_y_n) < 0.0 {
                    spin_evolver.n_alpha_cycles = 1;
                }
            }
            else if spin_evolver.ell_y_n > 0.0 && spin_evolver.ell_y_p < 0.0 {
                if (spin_evolver.ell_x_n*spin_evolver.ell_y_p - spin_evolver.ell_x_p*spin_evolver.ell_y_n)/(spin_evolver.ell_y_p-spin_evolver.ell_y_n) < 0.0{
                    spin_evolver.n_alpha_cycles = -1;
                }
            }

            else{
                spin_evolver.n_alpha_cycles = 0;
            }
            spin_evolver.alpha_n = spin_evolver.alpha_n + (spin_evolver.n_alpha_cycles as f64)*2.0*PI;

            // Finally, update the times
            spin_evolver.tau_p = 0.0;
            spin_evolver.tau_n = spin_evolver.delta_tau_hp;
        }

        spin_evolver
    }
  


}
