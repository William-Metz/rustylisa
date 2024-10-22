use core::f64;

// src/wave_builder/wave_builder.rs 
pub struct Wave_Builder{
    pub lotaDN: f64, 
    pub beta_: f64, 
    pub delta: f64, 
    pub eta: f64, 
    pub pi: f64, 
    pub chiaxDN: f64, 
    pub chiayDN: f64, 
    pub chiazDN: f64, 
    pub chisxDN: f64, 
    pub chisyDN: f64, 
    pub chiszDN: f64, 
    pub PsirDN: f64,
    pub AlphaDN: f64,
    pub PNOrder: u8, //Remove later
    pub W: [f64;248],
    pub Cos_Am_Psi: [[f64; 6]; 6],
    pub Cos_Ap_Psi: [[f64; 6]; 6],
    pub Sin_Am_Psi: [[f64; 6]; 6],
    pub Sin_Ap_Psi: [[f64; 6]; 6],

} 


impl Default for Wave_Builder {
    fn default() -> Self {
        Wave_Builder {
            lotaDN: 0.0,
            beta_: 0.0,
            delta: 0.0,
            eta: 0.0,
            pi: 0.0,
            chiaxDN: 0.0,
            chiayDN: 0.0,
            chiazDN: 0.0,
            chisxDN: 0.0,
            chisyDN: 0.0,
            chiszDN: 0.0,
            PsirDN: 0.0,
            PNOrder: 10,
            AlphaDN:0.0,
            W: [0.0; 248], 
            Cos_Am_Psi: [[0.0; 6]; 6],
            Cos_Ap_Psi: [[0.0; 6]; 6],
            Sin_Am_Psi: [[0.0; 6]; 6],
            Sin_Ap_Psi: [[0.0; 6]; 6],

        }
    }
}

