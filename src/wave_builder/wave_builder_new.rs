use crate::{
    constants::HUBBLECONSTANT, spin_evolver::spin_evolver::SpinEvolverClass,
    test_case::test_case::TestCase,
    wave_builder::wave_builder::WaveBuilder,

};


impl WaveBuilder {
    pub fn new(c_info: &TestCase) -> Self {
        WaveBuilder {
            lota_dn: 0.0,
            beta_: c_info.beta_,
            delta: c_info.delta,
            delta_tau_r: c_info.delta_t / c_info.GM,
            delta_tau: (c_info.delta_t / c_info.GM) / (1.0 + (c_info.R * HUBBLECONSTANT)),
            spin_evolver: SpinEvolverClass::new(c_info),
            tau_r_dn: 0.0,
            eta: 0.25 * (1.0 - (c_info.delta * c_info.delta)),
            chiax_dn: 0.0,
            chiay_dn: 0.0,
            chiaz_dn: 0.0,
            chisx_dn: 0.0,
            chisy_dn: 0.0,
            chisz_dn: 0.0,
            psi_r_dn: 0.0,
            psi_r_dp: 0.0,
            psi_p: 0.0,
            pn_order: c_info.pn_order as u64,
            alpha_dn: 0.0,
            hp: 0.0,
            hx: 0.0,
            vdn: 0.0,
            w: [0.0; 248],
            a: [0.0; 248],
            cos_am_psi: [[0.0; 6]; 6],
            cos_ap_psi: [[0.0; 6]; 6],
            sin_am_psi: [[0.0; 6]; 6],
            sin_ap_psi: [[0.0; 6]; 6],
        }
    }
}
