
pub struct SpinData {
    pub v: f64,
    pub iota: f64,
    pub alpha: f64,
    pub chi_ax: f64,
    pub chi_ay: f64,
    pub chi_az: f64,
    pub chi_sx: f64,
    pub chi_sy: f64,
    pub chi_sz: f64,
    pub psi: f64,
}

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
