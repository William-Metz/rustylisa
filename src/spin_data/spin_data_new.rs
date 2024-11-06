use crate::spin_data::spin_data::SpinData;
impl SpinData {
    pub fn new() -> Self {
        SpinData {
            v: 0.0,
            iota: 0.0,
            alpha: 0.0,
            chi_ax: 0.0,
            chi_ay: 0.0,
            chi_az: 0.0,
            chi_sx: 0.0,
            chi_sy: 0.0,
            chi_sz: 0.0,
            psi: 0.0,
        }
    }
}
