use core::f64;
use crate::test_case::test_case::TestCase;
use std::f64::consts::PI;

struct SpinEvolverClass {
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
    //    VCalc: VCalculatorClass,
    iota_N: f64,
    iota_P: f64,
    alphaN: f64,
    alphaP: f64,
    ΔtuahF: f64,
    ΔtuahP: f64,
    η: f64,
    simga_ell_: f64,
    tuaN: f64,
    tuaP: f64,
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
    fn new(c_info: CaseInfoClass) -> Self {
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
            P: c_info,
            S_ell_: 0.0,
            V0: 0.0,
            //           VCalc: VCalculatorClass::new(0.0, 0.0, 0.0, 0.0),
            iota_N: 0.0,
            iota_P: 0.0,
            alphaN: 0.0,
            alphaP: 0.0,
            ΔtuahF: 0.0,
            ΔtuahP: 0.0,
            η: 0.0,
            simga_ell_: 0.0,
            tuaN: 0.0,
            tuaP: 0.0,
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
            spin_evolver.VCalc = VCalculatorClass::new(c_info.tuac, c_info.delta, 0.0, 0.0);
            spin_evolver.V0 = spin_evolver.VCalc.VAtTime(0.0);
        } else {
            // Initialize with spins, set up the spin coefficients and other calculations
            // Start with basic setup
            let delta: f64 = P.delta;
            η = 0.25*(1 - delta*delta);
            let η2: f64 = η*η;
            let η3: f64 = η2*η;

            // Calculate spin evolution coefficients
            C10 = 0.75*(1.0 - delta) + 0.5*η;
            C20 = 0.75*(1.0 + delta) + 0.5*η;
            C12 = 9/16 + 5/4*η - η2/24 + delta*(-9/16 + 5/8*η);
            C22 = 9/16 + 5/4*η - η2/24 - delta*(-9/16 + 5/8*η);
            C14 = 27/32 + 3/16*η - 105/32*η2 - η3/48 + delta*(-27/32 + 39/8*η -5/32*η2);
            C24 = 27/32 + 3/16*η - 105/32*η2 - η3/48 - delta*(-27/32 + 39/8*η -5/32*η2);

            // Calculate orbital AM magnitude coefficients
            Opdelta2I4 = 0.25*(1.0 + delta)*(1.0 + delta);
            Omdelta2I4 = 0.25*(1.0 - delta)*(1.0 - delta);
            chi1ell_ = P.chi1*Cos(P.θ1);
            chi2ell_ = P.chi2*Cos(P.θ2);
            S_ell_ = Opdelta2I4*chi1ell_ + Omdelta2I4*chi2ell_;
            simga_ell_ = 0.5*(1.0 - delta)*chi2ell_ - 0.5*(1.0 + delta)*chi1ell_;
            L2 = 3/2 + η/6;
            L3 = -35/6*S_ell_-5/2*delta*simga_ell_;
            L4 = 27/8 - 19/8*η + η2/24;

            // Initialize the velocity calculator
            //VCalc = New VCalculatorClass(P.tuac, delta, chi1ell_, chi2ell_)

            // Calculate the initial orbital angular momentum magnitude
            V0 = VCalc.VAtTime(0.0);
            let v2: f64 = V0*V0;
            let v3: f64 = v2*V0;
            let v4: f64 = v3*V0;
            let v5: f64 = v4*V0;
            let L0: f64 = η/V0*(1.0 + L2*v2 + L3*v3 + L4*v4);

            // Calculate spin components in the precessing frame
            let chi1xL: f64 = P.chi1*Sin(P.θ1)*Cos(P.φ1);
            let chi1yL: f64 = P.chi1*Sin(P.θ1)*Sin(P.φ1);
            let chi1zL: f64 = chi1ell_;
            let chi2xL: f64 = P.chi2*Sin(P.θ2)*Cos(P.φ2);
            let chi2yL: f64 = P.chi2*Sin(P.θ2)*Sin(P.φ2);
            let chi2zL: f64 = chi2ell_;

            // Calculate total angular momentum components in the precessing frame
            let j0x: f64 = Opdelta2I4*chi1xL + Omdelta2I4*chi2xL;
            let j0y: f64 = Opdelta2I4*chi1yL + Omdelta2I4*chi2yL;
            let j0z: f64 = Opdelta2I4*chi1zL + Omdelta2I4*chi2zL + L0;
            ;
            // Calculate rotation matrix
            let θ0: f64 = ATan2(j0z, Sqrt(j0x*j0x+j0y*j0y));
            let φ0: f64 = ATan2(j0y, j0x);
            let rxx: f64 = Cos(θ0)*Cos(φ0);
            let rxy: f64 = Cos(θ0)*Sin(φ0);
            let rxz: f64 = -Sin(θ0);
            let ryx: f64 = -Sin(φ0);
            let ryy: f64 = Cos(φ0);
            let ryz: f64 = 0.0;
            let rzx: f64 = Sin(θ0)*Cos(φ0);
            let rzy: f64 = Sin(θ0)*Sin(φ0);
            let rzz: f64 = Cos(θ0);

            // Calculate initial values for the spin vector components
            chi1xP = rxx*chi1xL + rxy*chi1yL  + rxz*chi1zL;
            chi1yP = ryx*chi1xL + ryy*chi1yL  + ryz*chi1zL;
            chi1zP = rzx*chi1xL + rzy*chi1yL  + rzz*chi1zL;
            chi2xP = rxx*chi2xL + rxy*chi2yL  + rxz*chi2zL;
            chi2yP = ryx*chi2xL + ryy*chi2yL  + ryz*chi2zL;
            chi2zP = rzx*chi2xL + rzy*chi2yL  + rzz*chi2zL;

            // Calculate intial values for the orbital angular momentum components and angles
            ell_xP = rxz*L0;
            ell_yP = ryz*L0;
            ell_zP = rzz*L0;
            alphaP = Atan2(ell_yP,ell_xP);
            iota_P = Atan2(ell_zP,Sqrt(ell_xP*ell_xP + ell_yP*ell_yP));

            // Calculate components of the initial spin rate of change
            let omega_1: f64 = v5*(C10 + C12*v2 + C14*v4)/L0;
            let chi1xDotP: f64 = omega_1*(ell_yP*chi1zP - ell_zP*chi1yP);
            let chi1yDotP: f64 = omega_1*(ell_zP*chi1xP - ell_xP*chi1zP);
            let chi1zDotP: f64 = omega_1*(ell_xP*chi1yP - ell_yP*chi1xP);
            let omega_2: f64 = v5*(C20 + C22*v2 + C24*v4)/L0;
            let chi2xDotP: f64 = omega_2*(ell_yP*chi2zP - ell_zP*chi2yP);
            let chi2yDotP: f64 = omega_2*(ell_zP*chi2xP - ell_xP*chi2zP);
            let chi2zDotP: f64 = omega_2*(ell_xP*chi2yP - ell_yP*chi2xP);

            // Calculate the first time step to be half the step that would
            // take 628 steps for the fastest spin to precess once
            let s1dot: f64 = Sqrt(chi1xDotP*chi1xDotP + chi1yDotP*chi1yDotP + chi1zDotP*chi1zDotP);
            let s2dot: f64 = Sqrt(chi2xDotP*chi2xDotP + chi2yDotP*chi2yDotP + chi2zDotP*chi2zDotP);
            if s1dot = 0.0 {
                ΔtuahP = P.chi2/s2dot}
            else if s2dot = 0.0 {
                ΔtuahP = P.chi1/s1dot;
            }
            else{
                ΔtuahP = 0.5*Min(P.chi1/s1dot, P.chi2/s2dot);
            }
            ΔtuahF = ΔtuahP;

            // Evolve the spins using an Euler step
            chi1xN = chi1xP + ΔtuahP*chi1xDotP;
            chi1yN = chi1yP + ΔtuahP*chi1yDotP;
            chi1zN = chi1zP + ΔtuahP*chi1zDotP;
            chi2xN = chi2xP + ΔtuahP*chi2xDotP;
            chi2yN = chi2yP + ΔtuahP*chi2yDotP;
            chi2zN = chi2zP + ΔtuahP*chi2zDotP;

            // Evolve the orbital angular momentum using an Euler step
            let ell_DotP: f64 = η*(-1.0/v2 + L2 + 2*L2*v0 + 3*L4*v2)*VCalc.VDotForLastV/L0;
            let ell_xDotP: f64 = -Opdelta2I4*chi1xDotP - Omdelta2I4*chi2xDotP + ell_DotP*ell_xP;
            let ell_yDotP: f64 = -Opdelta2I4*chi1yDotP - Omdelta2I4*chi2yDotP + ell_DotP*ell_yP;
            let ell_zDotP: f64 = -Opdelta2I4*chi1zDotP - Omdelta2I4*chi2zDotP + ell_DotP*ell_zP;
            ell_xN = ell_xP + ΔtuahP*ell_xDotP;
            ell_yN = ell_yP + ΔtuahP*ell_yDotP;
            ell_zN = ell_zP + ΔtuahP*ell_zDotP;

            // To get a more precise estimate of the future values, iterate the calculation
            // Calculate the orbital angular momentum magnitude at the first time step
            let v1: f64 = VCalc.VAtTime(ΔtuahP);
            v2 = v1*v1;
            v3 = v2*v1;
            v4 = v3*v1;
            v5 = v4*v1;
            L0  = η/v1*(1.0 + L2*v2 + L3*v3 + L4*v4);

            // Calculate components of the future spin rate of change
            omega_1 = v5*(C10 + C12*v2 + C14*v4)/L0;
            let chi1xDotN: f64 = omega_1*(ell_yN*chi1zN - ell_zN*chi1yN);
            let chi1yDotN: f64 = omega_1*(ell_zN*chi1xN - ell_xN*chi1zN);
            let chi1zDotN: f64 = omega_1*(ell_xN*chi1yN - ell_yN*chi1xN);
            omega_2 = v5*(C20 + C22*v2 + C24*v4)/L0;
            let chi2xDotN: f64 = omega_2*(ell_yN*chi2zN - ell_zN*chi2yN);
            let chi2yDotN: f64 = omega_2*(ell_zN*chi2xN - ell_xN*chi2zN);
            let chi2zDotN: f64 = omega_2*(ell_xN*chi2yN - ell_yN*chi2xN);

            // Evolve the spins using a more correct step
            chi1xN = chi1xP + 0.5*ΔtuahP*(chi1xDotP + chi1xDotN);
            chi1yN = chi1yP + 0.5*ΔtuahP*(chi1yDotP + chi1yDotN);
            chi1zN = chi1zP + 0.5*ΔtuahP*(chi1zDotP + chi1zDotN);
            chi2xN = chi2xP + 0.5*ΔtuahP*(chi2xDotP + chi2xDotN);
            chi2yN = chi2yP + 0.5*ΔtuahP*(chi2yDotP + chi2yDotN);
            chi2zN = chi2zP + 0.5*ΔtuahP*(chi2zDotP + chi2zDotN);

            // Evolve the orbital angular momentum using a more correct step
            let ell_DotN: f64 = η*(-1.0/v2 + L2 + 2*L2*v0 + 3*L4*v2)*VCalc.VDotForLastV/L0;
            let ell_xDotN: f64 = -Opdelta2I4*chi1xDotN - Omdelta2I4*chi2xDotN + ell_DotP*ell_xN;
            let ell_yDotN: f64 = -Opdelta2I4*chi1yDotN - Omdelta2I4*chi2yDotN + ell_DotP*ell_yN;
            let ell_zDotN: f64 = -Opdelta2I4*chi1zDotN - Omdelta2I4*chi2zDotN + ell_DotP*ell_zN;
            ell_xN = ell_xP + 0.5*ΔtuahP*(ell_xDotN + ell_xDotP);
            ell_yN = ell_yP + 0.5*ΔtuahP*(ell_yDotN + ell_yDotP);
            ell_zN = ell_zP + 0.5*ΔtuahP*(ell_zDotN + ell_zDotP);
            alphaN = Atan2(ell_yN,ell_xN);
            iota_N = Atan2(ell_zN,Sqrt(ell_xN*ell_xN + ell_yN*ell_yN));
            // Initialize the precession phase
            let alphaDotP: f64 = (ell_yP*ell_xDotP - ell_xP*ell_yDotP)/(ell_xDotP*ell_xDotP + ell_yDotP*ell_yDotP);
            let alphaDotN: f64 = (ell_yN*ell_xDotN - ell_xN*ell_yDotN)/(ell_xDotN*ell_xDotN + ell_yDotN*ell_yDotN);
            psi_prP = 0.0;
            psi_prN = -0.5*ΔtuahP*(alphaDotP*Cos(iota_P) + alphaDotN*Cos(iota_N));

            // Check to see whether we have crossed the 2nd/3rd quadrant line
            if ell_yN < 0.0 & ell_yP > 0.0 {
                if (ell_xN*ell_yP - ell_xP*ell_yN)/(ell_yP-ell_yN) < 0.0 {
                    NalphaCycles = 1;
                }
            }
            else if ell_yN > 0.0 & ell_yP < 0.0 {
                if (ell_xN*ell_yP - ell_xP*ell_yN)/(ell_yP-ell_yN) < 0.0{
                    NalphaCycles = -1;
                }
            }

            else{
                NalphaCycles = 0;
            }
            alphaN = alphaN + NalphaCycles*2*PI;

                // Finally, update the times
            tuaP = 0;
            tuaN = ΔtuahP;
        }

    spin_evolver
    }

}


