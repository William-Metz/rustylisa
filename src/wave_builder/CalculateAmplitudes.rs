// CalculateAmplitudes.rs
use crate::wave_builder::wave_builder::Wave_Builder;
use std::f64::consts::PI;

// Now calculate all wave amplitudes
impl Wave_Builder{
    pub fn CalculateAmplitudes(&mut self){
        // Calculate some useful trig functions of angle lota

        // Input stuff
        let lotaDN: f64 = self.lotaDN; 
        println!("lotaDN: {}",lotaDN);
        let beta_: f64 = self.beta_;
        let delta: f64 = self.delta;
        let eta: f64 = self.eta;
        let pi: f64 = PI;
        let chiaxDN: f64 = self.chiaxDN;
        let chiayDN: f64 = self.chiayDN;
        let chiazDN: f64 = self.chiazDN;
        let chisxDN: f64 = self.chisxDN;
        let chisyDN: f64 = self.chisyDN;
        let chiszDN: f64 = self.chiszDN;


        //
        let c2: f64 = (lotaDN).cos();
        let s2: f64 = (lotaDN).sin();
        let c1: f64 = (0.5*lotaDN).cos();
        let s1: f64 = (0.5*lotaDN).sin();
        let c3: f64 = c2*c1 - s2*s1;
        let s3: f64 = s2*c1 + c2*s1;
        let c4: f64 = c2*c2-s2*s2;
        let s4: f64 = 2.0*c2*s2;
        let c5: f64 = c4*c1 - s4*s1;
        let s5: f64 = s4*c1 + c4*s1;
        let c6: f64 = c5*c1 - s5*s1;
        let s6: f64 = s5*c1 + c5*s1;
        let c7: f64 = c6*c1 - s6*s1;
        let s7: f64 = s6*c1 + c6*s1;
        let c8: f64 = c7*c1 - s7*s1;
        let s8: f64 = s7*c1 + c7*s1;
        let c9: f64 = c8*c1 - s8*s1;
        let s9: f64 = s8*c1 + c8*s1;
        let c10: f64 = c9*c1 - s9*s1;
        let s10: f64 = s9*c1 + c9*s1;
        let c1p2: f64 = c1*c1;
        let c1p3: f64 = c1p2*c1;
        let c1p4: f64 = c1p3*c1;
        let c1p5: f64 = c1p4*c1;
        let c1p6: f64 = c1p5*c1;
        let c1p7: f64 = c1p6*c1;
        let c1p8: f64 = c1p7*c1;
        let c1p9: f64 = c1p8*c1;
        let c1p10: f64 = c1p9*c1;
        let s1p2: f64 = s1*s1;
        let s1p3: f64 = s2*s1;
        let s1p4: f64 = s3*s1;
        let s1p5: f64 = s4*s1;
        let s1p6: f64 = s5*s1;
        let s1p7: f64 = s6*s1;
        let s1p8: f64 = s7*s1;
        let s1p9: f64 = s8*s1;
        let s1p10: f64 = s9*s1;
        let c2p2: f64 = c2*c2;
        let c2p3: f64 = c2p2*c2;
        let c2p4: f64 = c2p3*c2;
        let s2p2: f64 = s2*s2;
        let s2p3: f64 = s2p2*s2;
        let s2p4: f64 = s2p3*s2;
        let s2p5: f64 = s2p4*s2;

        // Define local beta_ trig functions
        let cbeta_: f64 = beta_.cos();
        let sbeta_: f64 = beta_.sin();
        let c2beta_: f64 = cbeta_*cbeta_ - sbeta_*sbeta_;
        let s2beta_: f64 = 2.0*sbeta_*cbeta_;
        let c3beta_: f64 = c2beta_*cbeta_ - s2beta_*sbeta_;
        let s3beta_: f64 = s2beta_*cbeta_ + c2beta_*sbeta_;
        let c4beta_: f64 = c3beta_*cbeta_ - s3beta_*sbeta_;
        let s4beta_: f64 = s3beta_*cbeta_ + c3beta_*sbeta_;
        let c5beta_: f64 = c4beta_*cbeta_ - s4beta_*sbeta_;
        let s5beta_: f64 = s4beta_*cbeta_ + c4beta_*sbeta_;
        let cbeta_2: f64 = cbeta_*cbeta_;
        let cbeta_3: f64 = c2beta_*cbeta_;
        let sbeta_2: f64 = sbeta_*sbeta_;
        let sbeta_3: f64 = s2beta_*sbeta_;
        let mut A: [f64; 248] = [0.0;248];
        // Amplitudes for H0P
        A[0] = (-1.5 - 0.5*c2beta_)*c1p4;
        A[1] = -2.0*c1p3*s2beta_*s1;
        A[2] =  2.0*s1p3*s2beta_*c1;
        A[3] = (-1.5 - 0.5*c2beta_)*s1p4;
        A[4] = -1.5*sbeta_2*s2p2;
        // Amplitudes for H0X
        A[129] = 4.0*sbeta_*c1*s1p3;
        A[130] = -2.0*cbeta_*s1p4;
        A[131] =  -4.0*sbeta_*c1p3*s1;
        A[132] =  -2.0*cbeta_*c1p4;

        //if Parameters.PNOrder > 0 {

        // Amplitude factors for H1P

        A[5] = delta*c1p6*(-45.0/32.0 *sbeta_ - 9.0/32.0 * s3beta_);
        A[6] = delta*c1p2*(-175.0/256.0 *sbeta_ + c2*(87.0/64.0 *sbeta_ - 5.0/64.0 * s3beta_) + c4*(-5.0/256.0 * sbeta_ + 15.0/256.0 * s3beta_) + 13.0/256.0 *s3beta_);
        A[7] = delta*s1p2*(175.0/256.0 *sbeta_ + c2*(87.0/64.0 *sbeta_ - 5.0/64.0 * s3beta_) + c4*(5.0/256.0 * sbeta_ - 15.0/256.0 * s3beta_) - 13.0/256.0 *s3beta_);
        A[8] = delta*c1p4*s1p2*(-5.0/32.0 *sbeta_ - 1.0/32.0 *s3beta_);
        A[9] = delta*c1p4*s1p2*(-45.0/32.0 *sbeta_ + 135.0/32.0 *s3beta_);
        A[10] = delta*c1p2*s1p4*(45.0/32.0 *sbeta_ - 135.0/32.0 *s3beta_);
        A[11] = delta*c1p2*s1p4*(5.0/32.0 *sbeta_ + 1.0/32.0 *s3beta_);
        A[12] = delta*s1p6*sbeta_*(27.0/16.0 + 9.0/16.0 *c2beta_);
        A[13] = delta*45.0/16.0 * s2p3*cbeta_*sbeta_2;
        A[14] = delta*((-85.0/256.0 *cbeta_ - 1.0/128.0 *cbeta_*c2beta_ - 1.0/32.0 *cbeta_*c2beta_*c2 - 3.0/128.0 *cbeta_*c2beta_*c4)*s2 - 11.0/64.0*cbeta_*s4 - 1.0/256.0*cbeta_*s6);
        A[15] = delta*((45.0/256.0 *cbeta_ + 81.0/128.0*cbeta_*c2beta_ + 27.0/32.0 *cbeta_*c2beta_*c2 + 27.0/128.0 *cbeta_*c2beta_*c4)*s2 + 9.0/64.0*cbeta_*s4 + 9.0/256.0 *cbeta_*s6);
        A[16] = delta*((1.0/256.0 * cbeta_*c2beta_ - 85.0/256.0 *cbeta_)*s2 + (11.0/64.0 * cbeta_ + 1.0/64.0 *cbeta_*c2beta_)*s4 - (1.0/256.0 *cbeta_ + 3.0/256.0 *cbeta_*c2beta_)*s6);
        A[17] = delta*((45.0/256.0 *cbeta_ + 135.0/256.0 *cbeta_*c2beta_)*s2 - (9.0/64.0 *cbeta_ + 27.0/64.0 *cbeta_*c2beta_)*s4 + (9.0/256.0 *cbeta_ + 27.0/256.0 *cbeta_*c2beta_)*s6);
        A[18] = delta*(1.0/64.0 *cbeta_*sbeta_2*s2 + 5.0/64.0 *cbeta_*sbeta_2*s6);

        // Ampitude factors for H1X

        A[136] = delta*(-1.0/64.0 *cbeta_*sbeta_ + 43.0/128.0 *cbeta_*c2*sbeta_ - 23.0/128.0 *c4*s2beta_ + 5.0/256.0 *c6*s2beta_);
        A[137] = delta*((-1.0 -1.0/4.0 *c2beta_)*c1 + 1.0/4.0 *c2beta_*c1*c2)*s1p3;
        A[138] = delta*(1.0/8.0 *c1p2*s2beta_*s1p4);
        A[139] = delta*(1.0/2.0 *sbeta_2*s4);
        A[140] = delta*(1.0/64.0 *cbeta_*sbeta_ + 43.0/128.0 *cbeta_*c2*sbeta_ + 23.0/128.0 *c4*s2beta_ + 5.0/256.0 *c6*s2beta_);
        A[141] = delta*((-1.0 - 1.0/4.0 *c2beta_)*c1p3 - 1.0/4.0 *c2beta_*c1p3*c2)*s1;
        A[142] = -delta*(1.0/8.0 *c1p4*s2beta_*s1p2);
        A[143] = delta*(45.0/8.0 *c1p4*s2beta_*s1p2);
        A[144] = delta*(9.0/2.0 *c2beta_*c1p5*s1);
        A[145] = -delta*(9.0/8.0 *c1p6*s2beta_);
        A[146] = (4.0*sbeta_ + 28.0/3.0 *s3beta_ - eta*(12.0*sbeta_ + 28.0*s3beta_))*c1p3*s1p5;
        A[147] = (eta*(4.0*cbeta_ + 28.0*c3beta_ - (4.0/3.0 *cbeta_ + 28.0/3.0 *c3beta_)))*c1p2*s1p6;
        A[148] = ((4.0/3.0 *sbeta_ - 4.0*s3beta_) + eta*(-4.0*sbeta_ + 12.0*s3beta_))*c1*s1p7;

        //}
        //if Parameters.PNOrder > 1 {

        // Amplitude factors for H2P

        A[19] = (59.0/16.0 + 5.0/2.0 *c2beta_ - 3.0/16.0 * c4beta_ + (5.0/24.0 - 11.0/6.0 * c2beta_ + 7.0/24.0 *c4beta_)*c2 - (5.0/48.0 + 1.0/12.0 *c2beta_ + 7.0/48.0 *c4beta_)*c4)*c1p4 
            + (-25.0/16.0 - 13.0/3.0 *c2beta_ + 9.0/16.0 *c4beta_ + (-5.0/8.0 + 11.0/2.0 *c2beta_ - 7.0/8.0 *c4beta_)*c2 + (5.0/16.0 + 1.0/4.0 *c2beta_ + 7.0/16.0 *c4beta_)*c4)*eta*c1p4;
        A[20] = (6.0 + 2.0*c2beta_)*eta*c1p8*sbeta_2 - (2.0 + 2.0/3.0 *c2beta_)*c1p8*sbeta_2;
        A[21] = 32.0*(1.0/3.0 - eta)*cbeta_3*c1p7*sbeta_*s1;
        A[22] = ((1.0/6.0 *c2beta_ - 5.0/6.0)*s2beta_ - 2.0/3.0 *cbeta_2*c2*s2beta_ + eta*((5.0/2.0 - 1.0/2.0 *c2beta_)*s2beta_ + 2.0 *cbeta_2*c2*s2beta_))*c1p5*s1;
        A[23] = (-(10.0/3.0 + 8.0/3.0 *c2beta_ + 14.0/3.0 *c4beta_) + eta*(10.0 + 8.0 *c2beta_ + 14.0*c4beta_))*c1p6*s1p2;
        A[24] = 1.0/2.0*(-(1.0 + 1.0/3.0 *c2beta_) + eta*(3.0 + c2beta_))*c1p6*sbeta_2*s1p2;
        A[25] = (8.0/3.0 - 56.0/3.0 *c2beta_ + eta*(56.0*c2beta_ - 8.0))*c1p5*s2beta_*s1p3;
        A[26] = eta*(c1*(16.0/3.0 *s2beta_ + 31.0/4.0 *c2*s2beta_ + 1.0/4.0 *c4*s2beta_ - 19.0/16.0 *s4beta_) - 7.0/8.0*c3*s4beta_ - 7.0/16.0*c5*s4beta_)*s1p3 
            +(c1*(-6.0*s2beta_ - 31.0/12.0*c2*s2beta_ - 1.0/12.0*c3*s2beta_ + 19.0/48.0*s4beta_) + 7.0/24.0*c3*s4beta_ + 7.0/48.0*c5*s4beta_)*s1p3            ;
        // double check
        A[27] = (59.0/16.0 + 5.0/2.0 *c2beta_ - 3.0/16.0 *c4beta_ - (5.0/24.0 - 11.0/6.0 *c2beta_ + 7.0/24.0 *c4beta_)*c2 - (5.0/48.0 + 1.0/12.0 *c2beta_ + 7.0/48.0 *c4beta_)*c4)*s1p4 
            + eta*(-25.0/16.0 - 13.0/3.0 *c2beta_ + 9.0/16.0 *c4beta_ + (5.0/8.0 - 11.0/2.0*c2beta_ + 7.0/8.0*c4beta_)*c2 + (5.0/16.0 + 1.0/4.0*c2beta_ + 7.0/16.0 *c4beta_)*c4)*s1p4;
        A[28] = (56.0/3.0 *c2beta_ - 8.0/3.0 + eta*(8.0 - 56.0*c2beta_))*c1p3*s2beta_*s1p5;
        // Interesting "space between number and ["
        A[29] = ((5.0/6.0 - 1.0/6.0 *c2beta_)*s2beta_ - 2.0/3.0*cbeta_2*c2*s2beta_ + eta*((-5.0/2.0 + 1.0/2.0*c2beta_)*s2beta_ + 2.0*cbeta_2*c2*s2beta_))*c1*s1p5;
        A[30] = (-(10.0/3.0 + 8.0/3.0 *c2beta_ + 14.0/3.0 *c4beta_) + eta*(10.0 + 8.0*c2beta_ + 14.0*c4beta_))*c1p2*s1p6;
        A[31] = (-(1.0/2.0 + 1.0/6.0 *c2beta_) + eta*(3.0/2.0 + 1.0/2.0 *c2beta_))*c1p2*sbeta_2*s1p6;
        A[32] = 32.0*(eta - 1.0/3.0)*cbeta_3*c1*sbeta_*s1p7;
        A[33] = (eta*(6.0 + 2.0*c2beta_) - (2.0 + 2.0/3.0 *c2beta_))*sbeta_2*s1p8;
        A[34] = 1.0/32.0*(1.0/3.0*(349.0 - 25.0 *c2beta_)*sbeta_2 - (25.0 + 35.0*c2beta_)*c4*sbeta_2) + eta*((25.0*c2beta_ - 45.0)*sbeta_2 + (25.0 + 35.0*c2beta_)*c4*sbeta_2)*s2p2;
        A[35] = 1.0/4.0*(eta*(25.0 + 35.0*c2beta_) - 1.0/3.0 *(25.0 - 35.0 *c2beta_))*sbeta_2*s2p4;
        A[36] = c1p3*(6.0 *s2beta_ - 31.0/12.0 *c2*s2beta_ + 1.0/12.0 *c4*s2beta_ - 19.0/48.0 *s4beta_)*s1 + 7.0/24.0 *c1p3*s4beta_*s3 - 7.0/48.0 *c1p3*s4beta_*s5 
            + eta*(c1p3*(-16.0/3.0 *s2beta_ + 31.0/4.0 *c2*s2beta_ - 1.0/4.0 *c4*s2beta_ + 19.0/16.0 *s4beta_)*s1 - 7.0/8.0 *c1p3*s4beta_*s3 + 7.0/16.0 *c1p3*s4beta_*s5);

        // Subscripts
        A[37] = chiaxDN*cbeta_*c1p2 - chiazDN*c1p2*sbeta_;
        A[38] = chiaxDN*(cbeta_/2.0 - cbeta_/2.0 *c2) - chiazDN*sbeta_*s1p2;
        A[39] = -chiayDN*cbeta_*s1p2;
        A[40] = -chiayDN*sbeta_*s2;
        A[41] = -chiayDN*cbeta_*c1p2;
        A[42] = delta*(chisxDN*cbeta_*c1p2 - chiszDN*c1p2*sbeta_);
        A[43] = delta*(chisxDN*(cbeta_/2.0 - cbeta_/2.0 *c2) - chiszDN*sbeta_*s1p2);
        A[44] = -delta*(chisyDN*cbeta_*s1p2);
        A[45] = -delta*(chisyDN*sbeta_*s2);
        A[46] = -delta*(chisyDN*cbeta_*c1p2);

        //Amplitude factors for H2X

        A[146] = (4.0*sbeta_ + 28.0/3.0 *s3beta_ - eta*(12.0*sbeta_ + 28.0*s3beta_))*c1p3*s1p5;
        A[147] = (eta*(4.0*cbeta_ + 28.0*c3beta_ - (4.0/3.0 *cbeta_ + 28.0/3.0 *c3beta_)))*c1p2*s1p6;
        A[148] = ((4.0/3.0 *sbeta_ - 4.0*s3beta_) + eta*(-4.0*sbeta_ + 12.0*s3beta_))*c1*s1p7;
        A[149] = (8.0*eta - 8.0/3.0)*cbeta_*sbeta_*s1p8;
        A[150] = c1*(-79.0/8.0 *sbeta_ + c2*(3.0/4.0 *sbeta_ - 19.0/12.0 *s3beta_) + c4*(1.0/8.0 *sbeta_ + 7.0/24.0 *s3beta_) - 3.0/8.0 *s3beta_)*s1p3 
            + eta*c1*(103.0/24.0 *sbeta_ - c4*(3.0/8.0 *sbeta_ + 7.0/8.0 *s3beta_) + 9.0/8.0 *s3beta_ + c2*(-9.0/4.0 *sbeta_ + 19.0/4.0 *s3beta_))*s1p3;
        A[151] = (47.0/8.0 *cbeta_ + 1.0/8.0 *c3beta_ + (7.0/6.0 *cbeta_ + 1.0/6.0 *c3beta_)*c2 - (1.0/24.0 *cbeta_ + 7.0/24.0 *c3beta_)*c4 
                  + eta*(-119.0/24.0 *cbeta_ - 3.0/8.0 *c3beta_ - (7.0/2.0 *cbeta_ + 1.0/2.0 *c3beta_)*c2 + (1.0/8.0 *cbeta_ + 7.0/8.0 *c3beta_)*c4))*s1p4;
        A[152] = (4.0/3.0 *sbeta_ - (1.0/3.0 + c2beta_)*c2*sbeta_ + eta*(-4.0*sbeta_ + (1.0 + 3.0*c2beta_)*c2*sbeta_))*c1*s1p5;
        A[153] = (2.0*eta - 2.0/3.0)*cbeta_*c1p2*sbeta_2*s1p6;
        A[154] = (15.0/2.0 *eta - 5.0/2.0)*cbeta_*c2*sbeta_2*s2p2;
        A[155] = c1p3*(79.0/8.0 *sbeta_ + c2*(3.0/4.0 *sbeta_ - 19.0/12.0 *s3beta_) - c4*(1.0/8.0 *sbeta_ + 7.0/24.0 *s3beta_) + 3.0/8.0 *s3beta_)*s1 
            + eta*c1p3*(-103.0/24.0 *sbeta_ + c4*(3.0/8.0 *sbeta_ + 7.0/8.0 *s3beta_) - 9.0/8.0 *s3beta_ + c2*(-9.0/4.0 *sbeta_ + 19.0/4.0 *c3beta_))*s1;
        A[156] = c1p4*(47.0/8.0 *cbeta_ + 1.0/8.0 *c3beta_ - (7.0/6.0 *cbeta_ + 1.0/6.0 *c3beta_)*c2 - (1.0/24.0 *cbeta_ + 7.0/24.0 *c3beta_)*c4) 
            + eta*c1p4*(-119.0/24.0 *cbeta_ - 3.0/8.0 *c3beta_ + (7.0/2.0 *cbeta_ + 1.0/2.0 *c3beta_)*c2 + (1.0/8.0 *cbeta_ + 7.0/8.0 *c3beta_)*c4);
        A[157] = (-4.0/3.0 *sbeta_ - (1.0/3.0 + c2beta_)*c2*sbeta_ + eta*(4.0*sbeta_ + (1.0 + 3.0*c2beta_)*c2*sbeta_))*c1p5*s1;
        A[158] = (2.0*eta - 2.0/3.0)*cbeta_*c1p6*sbeta_2*s1p2;
        A[159] = (eta*(12.0*sbeta_ + 28.0*s3beta_) - (4.0*sbeta_ + 28.0/3.0 *s3beta_))*c1p5*s1p3;
        A[160] = (eta*(4.0*cbeta_ + 28.0*c3beta_) - (4.0/3.0 *cbeta_ + 28.0/3.0 *c3beta_))*c1p6*s1p2;
        A[161] = (8.0/3.0 + 8.0*c2beta_ - eta*(8.0 + 24.0*c2beta_))*c1p7*sbeta_*s1;
        A[162] = (8.0*eta - 8.0/3.0)*cbeta_*c1p8*sbeta_2;

        //Subscripts

        A[163] = chiayDN*(1.0/2.0 + 1.0/2.0 *c2);
        A[164] = chiayDN*s1p2;
        A[165] = chiaxDN*(1.0/2.0 *cbeta_2 - 1.0/2.0 *cbeta_2*c2) + chiazDN*(-1.0/2.0 *cbeta_*sbeta_ + 1.0/2.0 *cbeta_*c2*sbeta_);
        A[166] = chiaxDN*cbeta_*sbeta_*s2 - chiazDN*sbeta_2*s2;
        A[167] = chiaxDN*(1.0/2.0 *cbeta_2 + 1.0/2.0 *cbeta_2*c2) + chiazDN*(-1.0/2.0 *cbeta_*sbeta_ - 1.0/2.0 *cbeta_*c2*sbeta_);
        A[168] = delta*(chisyDN*(1.0/2.0 + 1.0/2.0 *c2));
        A[169] = delta*(chisyDN*s1p2);
        A[170] = delta*(chisxDN*(1.0/2.0 *cbeta_2 - 1.0/2.0 *cbeta_2*c2) + chiszDN*(-1.0/2.0 *cbeta_*sbeta_ + 1.0/2.0 *cbeta_*c2*sbeta_));
        A[171] = delta*(chisxDN*cbeta_*sbeta_*s2 - chiszDN*sbeta_2*s2);
        A[172] = delta*(chisxDN*(1.0/2.0 *cbeta_2 + 1.0/2.0 *cbeta_2*c2) + chiszDN*(-1.0/2.0 *cbeta_*sbeta_ - 1.0/2.0 *cbeta_*c2*sbeta_));

        //}
        //if Parameters.PNOrder > 2 {
        // Ampitude factors for H3P

        A[47] = -(3.0*pi + pi*c2beta_)*c1p4;
        A[48] = -4.0*pi*c1p3*s2beta_*s1;
        A[49] = 4.0*pi*c1*s2beta_*s1p3;
        A[50] = -(3.0*pi + pi*c2beta_)*s1p4;
        A[51] = -3.0*pi*sbeta_2*s2p2;
        A[52] = delta*(eta*(625.0/128.0 + 625.0/384.0 *c2beta_) - (625.0/256.0 + 625.0/768.0 *c2beta_))*c1p10*sbeta_3;
        // I simplified the powers, is that okay?
        A[53] = delta*(eta*c1p2*( -7449.0/16384.0 *sbeta_ - 331.0/32768.0 *s3beta_ + c4*(337.0/12288.0 *sbeta_ - 47.0/8192.0 *s3beta_ - 21.0/8192.0 *s5beta_) 
                                  + c8*(7.0/49152.0 *sbeta_ + 7.0/32768.0 *s3beta_ - 35.0/32768.0 *s5beta_) + c6*(-59.0/6144.0 *sbeta_ - 91.0/4096.0 *s3beta_ + 7.0/4096.0 *s5beta_) 
                                  + c2*(1873.0/2048.0 *sbeta_ + 19.0/4096.0 *s3beta_ + 35.0/12288.0 *s5beta_) - 155.0/98304.0 *s5beta_) + c1p2*(43723.0/98304.0 *sbeta_ 
                                                                                                                                                - 9653.0/65536.0 *s3beta_ + c2*(-10675.0/12288.0 *sbeta_ + 1901.0/8192.0 *s3beta_ - 35.0/24576.0 *s5beta_)  + c6*(59.0/12288.0 *sbeta_ 
                                                                                                                                                                                                                                                                  + 91.0/8192.0 *s3beta_ - 7.0/8192.0 *s5beta_) + c8*(-7.0/98304.0 *sbeta_ - 7.0/65536.0 *s3beta_ + 35.0/65536.0 *s5beta_) + c4*(1103.0/24576.0 *sbeta_ 
                                                                                                                                                                                                                                                                                                                                                                                                 - 2833.0/16384.0 *s3beta_ + 21.0/16384.0 *s5beta_) + 155.0/196608.0 *s5beta_));
        A[54] = delta*(c1p6*(39249.0/8192.0 *sbeta_ + 38331.0/16384.0 *s3beta_ - c4*(1701.0/8192.0 *sbeta_ + 3159.0/16384.0 *s3beta_ + 3645.0/16384.0 *s5beta_) 
                             + c2*(2403.0/2048.0 *sbeta_ - 6399.0/4096.0 *s3beta_ + 2187.0/4096.0 *s5beta_) - 5751.0/16384.0 *s5beta_) + eta*c1p6*(-4689.0/4096.0 *sbeta_ 
                                                                                                                                                   - 24507.0/8192.0 *s3beta_ + c2*(-2403.0/1024.0 *sbeta_ + 6399.0/2048.0 *s3beta_ - 2187.0/2048.0 *s5beta_) + c4*(1701.0/4096.0 *sbeta_ 
                                                                                                                                                                                                                                                                   + 3159.0/8192.0 *s3beta_ + 3645.0/8192.0 *s5beta_) + 5751.0/8192.0 *s5beta_));
        A[55] = delta*((11875.0/768.0 *cbeta_ + 3125.0/768.0 *c3beta_ - eta*(11875.0/384.0 *cbeta_ + 3125.0/384.0 *c3beta_))*c1p9*sbeta_2*s1);
        A[56] = delta*(((-351.0/256.0 *cbeta_ + 243.0/256.0 *cbeta_*c2beta_)*sbeta_2 - (567.0/256.0 *cbeta_ + 405.0/256.0 *cbeta_*c2beta_)*c2*sbeta_2 
                        + eta*((351.0/128.0 *cbeta_ - 243.0/128.0 *cbeta_*c2beta_)*sbeta_2 + (567.0/128.0 *cbeta_ + 405.0/128.0 *cbeta_*c2beta_)*c2*sbeta_2))*c1p7*s1);
        A[57] = delta*((eta*(243.0/128.0 + 81.0/128.0 *c2beta_) - (243.0/256.0 + 81.0/256.0 *c2beta_))*c1p8*sbeta_3*s1p2);
        A[58] = delta*((-43723.0/98304.0 *sbeta_ + 9653.0/65536.0 *s3beta_ + c2*(-10675.0/12288.0 *sbeta_ + 1901.0/8192.0 *s3beta_ 
                                                                                 - 35.0/24576.0 *s5beta_) + c4*(-1103.0/24576.0 *sbeta_ + 2833.0/16384.0 *s3beta_ - 21.0/16384.0 *s5beta_) + c6*(59.0/12288.0 *sbeta_ 
                                                                                                                                                                                                 + 91.0/8192.0 *s3beta_ - 7.0/8192.0 *s5beta_) + c8*(7.0/98304.0 *sbeta_ + 7.0/65536.0 *s3beta_ - 35.0/65536.0 *s5beta_) 
                        -155.0/196608.0 *s5beta_)*s1p2 + eta*(7449.0/16384.0 *sbeta_ + 331.0/32768.0 *s3beta_ + c8*(-7.0/49152.0 *sbeta_ 
                                                                                                                    - 7.0/32768.0 *s3beta_ + 35.0/32768.0 *s5beta_) + c6*(-59.0/6144.0 *sbeta_ - 91.0/4096.0 *s3beta_ + 7.0/4096.0 *s5beta_) 
                                                              + c4*(-337.0/12288.0 *sbeta_ + 47.0/8192.0 *s3beta_ + 21.0/8192.0 *s5beta_) + c2*(1873.0/2048.0 *sbeta_ 
                                                                                                                                                + 19.0/4096.0 *s3beta_ + 35.0/12288.0 *s5beta_) + 155.0/98304.0 *s5beta_)*s1p2);
        A[59] = delta*(c1p4*(1675.0/4096.0 *sbeta_ + 825.0/8192.0 *s3beta_ - c4*(7.0/4096.0 *sbeta_ + 13.0/8192.0 *s3beta_ + 15.0/8192.0 *s5beta_) 
                             + c2*(27.0/1024.0 *sbeta_ - 151.0/2048.0 *s3beta_ + 3.0/2048.0 *s5beta_) - 13.0/8192.0 *s5beta_)*s1p2 + eta*c1p4*(245.0/2048.0 *sbeta_ 
                                                                                                                                               - 57.0/4096.0 *s3beta_ + c2*(-27.0/512.0 *sbeta_ + 151.0/1024.0 *s3beta_ - 3.0/1024.0 *s5beta_) + c4*(7.0/2048.0 *sbeta_ + 13.0/4096.0 *s3beta_ 
                                                                                                                                                                                                                                                     + 15.0/4096.0 *s5beta_) + 13.0/4096.0 *s5beta_)*s1p2);
        A[60] = delta*((eta*(4375.0/512.0 *sbeta_ + 8125.0/1024.0 *s3beta_ + 9375.0/1024.0 *s5beta_) - (4375.0/1024.0 *sbeta_ 
                                                                                                        + 8125.0/2048.0 *s3beta_ + 9375.0/2048.0 *s5beta_))*c1p8*s1p2);
        A[61] = delta*(c1p4*(20475.0/4096.0 *sbeta_ - 149391.0/8192.0 *s3beta_ + c2*(2187.0/1024.0 *sbeta_ + 10017.0/2048.0 *s3beta_ 
                                                                                     - 1701.0/2048.0 *s5beta_) + 7371.0/8192.0 *s5beta_ + c4*(-567.0/4096.0 *sbeta_ - 1701.0/8192.0 *s3beta_ + 8505.0/8192.0 *s5beta_))*s1p2 
                       + eta*c1p4*(-3195.0/2048.0 *sbeta_ + 45711.0/4096.0 *s3beta_ + c4*(567.0/2048.0 *sbeta_ + 1701.0/4096.0 *s3beta_ - 8505.0/4096.0 *s5beta_) 
                                   - 7371.0/4096.0 *s5beta_ + c2*(-2187.0/512.0 *sbeta_ - 10017.0/1024.0 *s3beta_ + 1701.0/1024.0 *s5beta_))*s1p2);
        A[62] = delta*((4375.0/384.0 *cbeta_ + 625.0/256.0 *c3beta_ + 3125.0/256.0 *c5beta_ - eta*(4375.0/192.0 *cbeta_ + 625.0/128.0 *c3beta_ + 3125.0/128.0 *c5beta_))*c1p7*s1p3);
        A[63] = delta*(c1p5*((-37.0/384.0 *cbeta_ + 1.0/384.0 *cbeta_*c2beta_)*sbeta_2 - (7.0/384.0 *cbeta_ + 5.0/384.0 *cbeta_*c2beta_)*c2*sbeta_2)*s1p3 
                       + eta*c1p5*((37.0/192.0 *cbeta_ - 1.0/192.0 *cbeta_*c2beta_)*sbeta_2 + (7.0/192.0 *cbeta_ + 5.0/192.0 *cbeta_*c2beta_)*c2*sbeta_2)*s1p3);
        A[64] = delta*((eta*(1.0/64.0 + 1.0/192.0 *c2beta_) - (1.0/128.0 + 1.0/384.0 *c2beta_))*c1p6*sbeta_3*s1p4);
        A[65] = delta*(eta*c1p2*(-245.0/2048.0 *sbeta_ + 57.0/4096.0 *s3beta_ - c4*(7.0/2048.0 *sbeta_ + 13.0/4096.0 *s3beta_ + 15.0/4096.0 *s5beta_) 
                                 + c2*(-27.0/512.0 *sbeta_ + 151.0/1024.0 *s3beta_ - 3.0/1024.0 *s5beta_) - 13.0/4096.0 *s5beta_)*s1p4 + c1p2*(-1675.0/4096.0 *sbeta_ 
                                                                                                                                               - 825.0/8192.0 *s3beta_ + c2*(27.0/1024.0 *sbeta_ - 151.0/2048.0 *s3beta_ + 3.0/2048.0 *s5beta_) + c4/4096.0*(7.0*sbeta_ + 13.0*s3beta_ + 15.0*s5beta_)*s1p4));
        A[66] = delta*(4375.0*eta*c1p6*(1.0/768.0 *sbeta_ + 1.0/512.0 *s3beta_ - 5.0/512.0 *s5beta_)*s1p4 + 4375.0*c1p6*(-1.0/1536.0 *sbeta_ - 1.0/1024.0 *s3beta_ + 5.0/1024.0 *s5beta_)*s1p4);
        A[67] = delta*(c1p2*(-20475.0/4096.0 *sbeta_ + 149391.0/8192.0 *s3beta_ + c4/4096.0 *(567.0 *sbeta_ + 1701.0/2.0 *s3beta_ - 8505.0/2.0 *s5beta_) 
                             + c2/2048.0 *(4374.0 *sbeta_ + 10017.0 *s3beta_ - 1701.0 *s5beta_) - 7371.0/8192.0 *s5beta_)*s1p4 + eta*c1p2*(3195.0/2048.0 *sbeta_ 
                                                                                                                                           - 45711.0/4096.0 *s3beta_ + 7371.0/4096.0 *s5beta_ + c2*(-2187.0/512.0 *sbeta_ - 10017.0/1024.0 *s3beta_ + 1701.0/1024.0 *s5beta_) 
                                                                                                                                           + c4*(-567.0/2048.0 *sbeta_ - 1701.0/4096.0 *s3beta_ + 8505.0/4096.0 *s5beta_))*s1p4);
        A[68] = delta*(eta*c1p3*((37.0/192.0 *cbeta_ - 1.0/192.0 *cbeta_*c2beta_)*sbeta_2 - (7.0/192.0 *cbeta_ + 5.0/192.0 *cbeta_*c2beta_)*c2*sbeta_2)*s1p5 
                       + c1p3*((-37.0/384.0 *cbeta_ + 1.0/384.0 *cbeta_*c2beta_)*sbeta_2 + (7.0/384.0 *cbeta_ + 5.0/384.0 *cbeta_*c2beta_)*c2*sbeta_2)*s1p5);
        A[69] = delta*(1.0/128.0 + 1.0/384.0 *c2beta_ - eta*(1.0/64.0 + 1.0/192.0 *c2beta_))*c1p4*sbeta_3*s1p6;
        A[70] = delta*(eta*((14067.0/4096.0 + 4689.0/1024.0 *c2beta_ - 5751.0/4096.0 *c4beta_)*sbeta_ + (-297.0/1024.0 + 1053.0/256.0 *c2beta_ 
                                                                                                         - 2187.0/1024.0 *c4beta_)*c2*sbeta_ - (5103.0/4096.0 + 1701.0/1024.0 *c2beta_ + 3645.0/4096.0 *c4beta_)*c4*sbeta_)*s1p6 
                       + ((-55539.0/8192.0 - 8145.0/2048.0 *c2beta_ + 5751.0/8192.0 *c4beta_)*sbeta_ + (297.0/2048.0 - 1053.0/512.0 *c2beta_ 
                                                                                                        + 2187.0/2048.0 *c4beta_)*c2*sbeta_ + (5103.0/8192.0 + 1701.0/2048.0 *c2beta_ + 3645.0/8192.0 *c4beta_)*c4*sbeta_)*s1p6);
        // The multiplication location
        A[71] = delta*(c1p4*(4375.0/1536.0 *sbeta_ + 4375.0/1024.0 *s3beta_ - 21875.0/1024.0 *s5beta_)*s1p6 
                       + eta*c1p4*(-4375.0/768.0 *sbeta_ - 4375.0/512.0 *s3beta_ + 21875.0/512.0 *s5beta_)*s1p6);
        A[72] = delta*((4375.0/384.0 *cbeta_ + 625.0/256.0 *c3beta_ + 3125.0/256.0 *c5beta_ - eta*(4375.0/192.0 *cbeta_ 
                                                                                                   + 625.0/128.0 *c3beta_ + 3125.0/128.0 *c5beta_))*c1p3*s1p7);
        A[73] = delta*(eta*c1*((351.0/128.0 *cbeta_ - 243.0/128.0 *cbeta_*c2beta_)*sbeta_2 - (567.0/128.0 *cbeta_ + 405.0/128.0 *cbeta_*c2beta_)*c2*sbeta_2)*s1p7 
                       + c1*((-351.0/256.0 *cbeta_ + 243.0/256.0 *cbeta_*c2beta_)*sbeta_2 + (567.0/256.0 *cbeta_ + 405.0/256.0 *cbeta_*c2beta_)*c2*sbeta_2)*s1p7);
        A[74] = delta*((243.0/256.0 - 81.0/256.0 *c2beta_ - eta*(243.0/128.0 + 81.0/128.0 *c2beta_))*c1p2*sbeta_3*s1p8);
        A[75] = delta*((4375.0/1024.0 *sbeta_ + 8125.0/2048.0 *s3beta_ + 9375.0/2048.0 *s5beta_ - eta*(4375.0/512.0 *sbeta_ + 8125.0/1024.0 *s3beta_ + 9375.0/1024.0 *s5beta_))*c1p2*s1p8);
        // Here is when Dr. Moore and i realized that using the '[ ]' is totally wrong, instead i should just replace them with normal parenthesis.
        A[76] = delta*((11875.0/768.0 *cbeta_ + 3125.0/768.0 *c3beta_ - eta*(11875.0/384.0 *cbeta_ + 3125.0/384.0 *c3beta_))*c1*sbeta_2*s1p9);
        A[77] = delta*((625.0/256.0 + 625.0/768.0 *c2beta_ - eta*(625.0/128.0 + 625.0/384.0 *c2beta_))*sbeta_3*s1p10);
        A[78] = delta*(eta*((10197.0/20488.0 *cbeta_ - 3969.0/2048.0 *cbeta_*c2beta_)*sbeta_2 - (1701.0/2048.0 *cbeta_ + 5103.0/2048.0 *cbeta_*c2beta_)*c4*sbeta_2)*s2p3 
                       + ((-44757.0/4096.0 *cbeta_ + 3969.0/4096.0 *cbeta_*c2beta_)*sbeta_2 + (1701.0/4096.0 *cbeta_ + 5103.0/4096.0 *cbeta_*c2beta_)*c4*sbeta_2)*s2p3);
        A[79] = delta*((21875.0/4096.0 *cbeta_ + 13125.0/4096.0 *c3beta_ - eta*(21875.0/2048.0 *cbeta_ + 13125.0/2048.0 *c3beta_))*sbeta_2*s2p5);
        A[80] = delta*((-37071.0/16384.0 *cbeta_*c2beta_ + cbeta_*(-7641.0/8192.0 + 567.0/32768.0 *c4beta_) - (10917.0/8192.0 *cbeta_ 
                                                                                                               + 2835.0/1024.0 *cbeta_*c2beta_)*c2 + (-10089.0/16284.0 *cbeta_ + 135.0/8192.0 *cbeta_*c2beta_)*c4 + 513.0/8192.0 *cbeta_*c6 
                        + 567.0/32768.0 *cbeta_*c8)*s2 - 81.0/8192.0 *cbeta_*c4beta_*s4 + 1053.0/65536.0 *cbeta_*c4beta_*s6   
                       + (2565.0/32768.0 *c3beta_ + 729.0/32768.0 *c5beta_)*s8 + (243.0/131072.0 *c3beta_ + 1215.0/131072.0 *c5beta_)*s10 
                       + eta*((5967.0/8192.0 *cbeta_*c2beta_ + cbeta_*(2457.0/4096.0 - 567.0/16384.0 *c4beta_) + (4005.0/4096.0 *cbeta_ + 243.0/512.0 *cbeta_*c2beta_)*c2 
                               + (6633.0/8192.0 *cbeta_ - 5319.0/4096.0 *cbeta_*c2beta_)*c4 - 513.0/4096.0 *cbeta_*c6 - 567.0/16384.0 *cbeta_*c8)*s2 
                              + 81.0/4096.0 *cbeta_*c4beta_*s4 - 1053.0/32768.0 *cbeta_*c4beta_*s6 - (2565.0/16384.0 *c3beta_ + 729.0/16384.0 *c5beta_)*s8 
                              - (243.0/65536.0 *c3beta_ + 1215.0/65536.0 *c5beta_)*s10));
        A[81] = delta*((-18603.0/8192.0 *cbeta_*c2beta_ + cbeta_*(-20475.0/32768.0 + 567.0/32768.0 *c4beta_))*s2 + (2835.0/2048.0 *cbeta_*c2beta_ 
                                                                                                                    + cbeta_*(5715.0/8192.0 + 81.0/8192.0 *c4beta_))*s4 + (135.0/16384.0 *cbeta_*c2beta_ + cbeta_*(-20745.0/65536.0 + 1053.0/65536.0 *c4beta_))*s6 
                       - (513.0/16384.0 *cbeta_ + 2565.0/32768.0 *c3beta_ + 729.0/32768.0 *c5beta_)*s8  
                       + (567.0/65536.0 *cbeta_ + 243.0/131072.0 *c3beta_ + 1215.0/131072.0*c5beta_)*s10 + eta*((5643.0/4096.0 *cbeta_*c2beta_ + cbeta_*(3195.0/16384.0 - 567.0/16384.0 *c4beta_))*s6 
                                                                                                                + (513.0/8192.0 *cbeta_ + 2565.0/16384.0 *c3beta_ + 729.0/16384.0 *c5beta_)*s8 - (567.0/32768.0 *cbeta_ + 243.0/65536.0 *c3beta_ + 1215.0/65536.0 *c5beta_)*s10));
        A[82] = delta*((319.0/24576.0 *cbeta_*c2beta_ + cbeta_*(871.0/4096.0 + 1.0/49152.0 *c4beta_) + (933.0/4096.0 *cbeta_ + 133.0/1536.0 *cbeta_*c2beta_)*c2 
                        + (625.0/24576.0 *cbeta_ + 211.0/4096.0 *cbeta_*c2beta_)*c4 - 11.0/12288.0 *cbeta_*c6 - 7.0/49152.0 *cbeta_*c8)*s2 - 1.0/12288.0 *cbeta_*c4beta_*s4 
                       + 1.0/32768.0 *cbeta_*c4beta_*s6 - (45.0/16384.0 *c3beta_ + 1.0/16384.0 *c5beta_)*s8   
                       - (1.0/65536.0 *c3beta_ + 5.0/65536.0 *c5beta_)*s10 + eta*((257.0/12288.0 *cbeta_*c2beta_ - cbeta_*(1493.0/6144.0 + 1.0/24576.0 *c4beta_) 
                                                                                   + (-1391.0/6144.0 + 11.0/768.0 *cbeta_*c2beta_)*c2 + (-49.0/12288.0 *cbeta_ + 77.0/2048.0 *cbeta_*c2beta_)*c4 + 11.0/6144.0 *cbeta_*c6 
                                                                                   + 7.0/24576.0 *cbeta_*c8)*s2 + 1.0/6144.0 *cbeta_*c4beta_*s4 - 1.0/16384.0 *cbeta_*c4beta_*s6 + (45.0/8192.0 *c3beta_ + 1.0/8192.0 *c5beta_)*s8 + (1.0/32768.0 *c3beta_ + 5.0/32768.0 *c5beta_)*s10));
        A[83] = delta*((-157.0/12288.0 *cbeta_*c2beta_ + cbeta_*(9827.0/49152.0 + 1.0/49152.0 *c4beta_))*s2 + (-133.0/3072.0 *cbeta_*c2beta_ 
                                                                                                               + cbeta_*(-1405.0/12288.0 + 1.0/12288.0 *c4beta_))*s4 + (211.0/8192.0 *cbeta_*c2beta_ + cbeta_*(419.0/32768.0 + 1.0/32768.0 *c4beta_))*s6 
                       + (11.0/24576.0 *cbeta_ + 45.0/16384.0 *c3beta_ + 1.0/16384.0 *c5beta_)*s8 + (-7.0/98304.0 *cbeta_ - 1.0/65536.0 *c3beta_ - 5.0/65536.0 *c5beta_)*s10   
                       + eta*((13.0/6144.0 *cbeta_*c2beta_ + cbeta_*(-5923.0/24576.0 - 1.0/24576.0 *c4beta_))*s2 + (-11.0/1536.0 *cbeta_*c2beta_ + cbeta_*(701.0/6144.0 - 1.0/6144.0 *c4beta_))*s4 
                              + (77.0/4096.0 *cbeta_*c2beta_ + cbeta_*(-35.0/16384.0 - 1.0/16384.0 *c4beta_))*s6 + (-11.0/12288.0 *cbeta_ - 45.0/8192.0 *c3beta_ - 1.0/8192.0 *c5beta_)*s8 + (7.0/49152.0 *cbeta_ 
                                                                                                                                                                                              + 1.0/32768.0 *c3beta_ + 5.0/32768.0 *c5beta_)*s10));
        A[84] = delta*((-341.0/8192.0 *cbeta_ + 1.0/8192.0 *cbeta_*c2beta_)*sbeta_2*s2 + (-3411.0/16384.0 *cbeta_ + 7.0/16384.0 *cbeta_*c2beta_)*sbeta_2*s6 + (35.0/32768.0 *cbeta_ 
                                                                                                                                                               + 21.0/32768.0 *c3beta_)*sbeta_2*s10 + eta*((-43.0/4096.0 *cbeta_ - 1.0/4096.0 *cbeta_*c2beta_)*sbeta_2*s2 + (-429.0/8192.0 *cbeta_ + 7.0/8192.0 *cbeta_*c2beta_)*sbeta_2*s6 
                                                                                                                                                                                                           + (-35.0/16384.0 *cbeta_ - 21.0/16384.0 *c3beta_)*sbeta_2*s10));

        //HP 3.0/2.0,SO

        A[85] = chisxDN*(2.0*cbeta_*c2p3*sbeta_ - eta*cbeta_*c2p3*sbeta_);
        A[86] = chiszDN*(eta*c1p4*(-5.0/2.0 - 7.0/2.0 *c2beta_ + (1.0/2.0 + 1.0/6.0 *c2beta_)*c2) + c1p4*(-3.0 - c2beta_ + (5.0 + 5.0/3.0 *c2beta_)*c4)) 
            + chisxDN*(c1p4*(7.0/3.0 *s2beta_ - 10.0/3.0 *c2*s2beta_) - eta*c1p4*(19.0/6.0 *s2beta_ + 1.0/3.0 *c2*s2beta_));
        A[87] = chisxDN*(eta*(1.0/2.0 + 1.0/6.0 *c2beta_)*c1p5*s1 + (5.0 + 5.0/3.0 *c2beta_)*c1p5*s1);
        A[88] = chisxDN*(eta*(1.0/2.0 + 1.0/6.0 *c2beta_)*c1*s1p5 + (5.0 + 5.0/3.0 *c2beta_)*c1*s1p5);
        A[89] = chisxDN*(eta*c1p3*(-17.0/4.0 + 79.0/12.0 *c2beta_ + (-1.0/4.0 + 7.0/12.0 *c2beta_)*c2)*s1 + c1p3*(3.0/2.0 - 13.0/6.0 *c2beta_ 
                                                                                                                  + (-5.0/2.0 + 35.0/6.0 *c2beta_)*c2)*s1) + chiszDN*(eta*c1p3*(-7.0 *s2beta_ + 2.0/3.0 *c2*s2beta_)*s1 + c1p3*(-2.0 *s2beta_ + 20.0/3.0 *c2*s2beta_)*s1);
        A[90] = chisxDN*(c1*(3.0/2.0 - 13.0/6.0 *c2beta_ + (5.0/2.0 - 35.0/6.0 *c2beta_)*c2)*s1p3 + eta*c1*(-17.0/4.0 + 79.0/12.0*c2beta_ 
                                                                                                            + (1.0/4.0 - 7.0/12.0 *c2beta_)*c2)*s1p3) + chiszDN*(-c1 *(2.0*s2beta_ + 20.0/3.0 *c2*s2beta_)*s1p3 - eta*c1*(7.0*s2beta_ + 2.0/3.0 *c2*s2beta_)*s1p3);
        A[91] = chiszDN*(eta*(5.0/2.0 + 7.0/2.0 *c2beta_ + (1.0/2.0 + 1.0/6.0 *c2beta_)*c2)*s1p4 + (3.0 + c2beta_ + (5.0 + 5.0/3.0 *c2beta_)*c2)*s1p4) 
            + chisxDN*(-(7.0/3.0 *s2beta_ + 10.0/3.0 *c2*s2beta_)*s1p4 + eta*(19.0/6.0 *s2beta_ - 1.0/3.0 *c2*s2beta_)*s1p4);
        A[92] = chiszDN*(-3.0 + 3.0/2.0 *eta)*c2*sbeta_2*s2p2;
        A[93] = chisxDN*(3.0/4.0 + 1.0/4.0 *c2beta_ - eta*(3.0/8.0 + 1.0/8.0 *c2beta_))*s2p3;
        A[94] = chisxDN*(10.0/3.0 + 1.0/3.0 *eta)*cbeta_*c2*sbeta_*s2p2 + chiszDN*(5.0 + 1.0/2.0 *eta)*c2*sbeta_2*s2p2;
        A[95] = chiszDN*(3.0/2.0 + 1.0/2.0 *c2beta_ - eta*(3.0/4.0 + 1.0/4.0 *c2beta_))*c2*s2p2 + chisxDN*(1.0/2.0 *eta - 1.0)*c2*s2beta_*s2p2;
        A[96] = chisxDN*(-11.0/16.0 *c2beta_*s2 - 3.0/4.0 *s2p3 - 7.0/16.0 *c2beta_*s6 + eta*(11.0/32.0 *c2beta_*s2 + 3.0/8.0 *s2p3 + 7.0/32.0 *c2beta_*s6)) 
            + chiszDN*(1.0/2.0 *s2beta_*s2 - 1.0/2.0 *s2beta_*s6 + eta*(-1.0/4.0 *s2beta_*s2 + 1.0/4.0 *s2beta_*s6));
        A[97] = chisyDN*((15.0/8.0 - 3.0/8.0 *c2beta_ + (9.0/8.0 - 5.0/8.0 *c2beta_)*c4)*s2 + eta*(-15.0/16.0 + 3.0/16.0 *c2beta_ + (-9.0/16.0 + 5.0/16.0 *c2beta_)*c4)*s2);
        A[98] = chisyDN*(eta - 2.0)*cbeta_*c2*sbeta_*s2p2;
        A[99] = chisyDN*(3.0/4.0 + 1.0/4.0 *c2beta_ - eta*(3.0/8.0 + 1.0/8.0 *c2beta_))*s2p3;
        A[100] = chisyDN*(c1*(5.0/2.0 - 11.0/6.0 *c2beta_ + (15.0/2.0 - 25.0/6.0 *c2beta_)*c2)*s1p3 + eta*c1*(1.0/4.0 - 31.0/12.0 *c2beta_ + (3.0/4.0 - 5.0/12.0 *c2beta_)*c2)*s1p3);
        A[101] = chisyDN*(-(7.0/3.0 *s2beta_ + 10.0/3.0 *c2*s2beta_)*s1p4 - eta*(5.0/6.0 *s2beta_ + 1.0/3.0 *c2*s2beta_)*s1p4);
        A[102] = chisyDN*(5.0 + 5.0/3.0 *c2beta_ + eta*(1.0/2.0 + 1.0/6.0 *c2beta_))*c1*s1p5;
        // First time seeing a negative sign at the beginning.
        A[103] = -chisyDN*(1.0/3.0 + 11.0/6.0 *eta)*cbeta_*sbeta_*s2p2;
        A[104] = chisyDN*(eta*c1p3*(1.0/4.0 - 31.0/12.0 *c2beta_ + (-3.0/4.0 + 5.0/12.0 *c2beta_)*c2)*s1 + c1p3*(5.0/2.0 - 11.0/6.0 *c2beta_ 
                                                                                                                 + (-15.0/2.0 + 25.0/6.0 *c2beta_)*c2)*s1);
        A[105] = chisyDN*(c1p4*(7.0/3.0 *s2beta_ - 10.0/3.0 *c2*s2beta_) + eta*c1p4*(5.0/6.0 *s2beta_ - 1.0/3.0 *c2*s2beta_));
        A[106] = chisyDN*(eta*(1.0/2.0 + 1.0/6.0 *c2beta_) + 5.0 + 5.0/3.0 *c2beta_)*c1p5*s1;
        A[107] = 2.0*delta*chiaxDN*cbeta_*c2p3*sbeta_;
        A[108] = delta*(chiazDN*c1p4*(-3.0 - c2beta_ + (5.0 + 5.0/3.0 *c2beta_)*c2) + chiaxDN*c1p4*(7.0/3.0 *s2beta_ - 10.0/3.0 *c2*s2beta_));
        A[109] = delta*chiaxDN*(5.0 + 5.0/3.0 *c2beta_)*c1p5*s1;
        A[110] = delta*chiaxDN*(5.0 + 5.0/3.0 *c2beta_)*c1*s1p5;
        A[111] = delta*(chiaxDN*(3.0/2.0 - 13.0/6.0 *c2beta_ + (-5.0/2.0 + 35.0/6.0 *c2beta_)*c2) + chiazDN*(-2.0*s2beta_ + 20.0/3.0 *c2*s2beta_))*c1p3*s1;
        A[112] = delta*(chiaxDN*(3.0/2.0 - 13.0/6.0 *c2beta_ + (5.0/2.0 - 35.0/6.0 *c2beta_)*c2) + chiazDN*(-2.0*s2beta_ - 20.0/3.0 *c2*s2beta_))*c1*s1p3;
        A[113] = delta*(chiazDN*(3.0 + c2beta_ + (5.0 + 5.0/3.0 *c2beta_)*c2)*s1p4 - chiaxDN*(7.0/3.0 *s2beta_ + 10.0/3.0 *c2*s2beta_)*s1p4);
        A[114] = -3.0*delta*chiazDN*c2*sbeta_2*s2p2;
        A[115] = delta*chiaxDN*(3.0/4.0 + 1.0/4.0 *c2beta_)*s2p3;
        A[116] = delta*(10.0/3.0 *chiaxDN*cbeta_*c2*sbeta_*s2p2 + 5.0*chiazDN*c2*sbeta_2*s2p2);
        A[117] = delta*chiazDN*(3.0/2.0 + 1.0/2.0 *c2beta_)*c2*s2p2 - chiaxDN*c2*s2beta_*s2p2;
        A[118] = delta*(chiaxDN*(-11.0/16.0 *c2beta_*s2 - 3.0/4.0 *s2p3 - 7.0/16.0 *c2beta_*s6) + chiazDN*(1.0/2.0 *s2beta_*s2 - 1.0/2.0 *s2beta_*s6));
        A[119] = delta*(chiayDN*(15.0/8.0 - 3.0/8.0 *c2beta_ + (9.0/8.0 - 5.0/8.0 *c2beta_)*c4)*s2);
        A[120] = -2.0*delta*chiayDN*cbeta_*c2*sbeta_*s2p2;
        A[121] = delta*chiayDN*(3.0/4.0 + 1.0/4.0 *c2beta_)*s2p3;
        A[122] = delta*chiayDN*c1*(5.0/2.0 - 11.0/6.0 *c2beta_ + (15.0/2.0 - 25.0/6.0 *c2beta_)*c2)*s1p3;
        A[123] = delta*chiayDN*(-7.0/3.0 *s2beta_ - 10.0/3.0 *c2*s2beta_)*s1p4;
        A[124] = delta*chiayDN*(5.0 + 5.0/3.0 *c2beta_)*c1*s1p5;
        A[125] = -1.0/3.0 *delta*chiayDN*cbeta_*sbeta_*s2p2;
        A[126] = delta*chiayDN*c1p3*(5.0/2.0 - 11.0/6.0 *c2beta_ + (-15.0/2.0 + 25.0/6.0 *c2beta_)*c2)*s1;
        A[127] = delta*chiayDN*c1p4*(7.0/3.0 *s2beta_ - 10.0/3.0 *c2*s2beta_);
        A[128] = delta*chiayDN*(5.0 + 5.0/3.0 *c2beta_)*c1p5*s1;

        //Amplitude factors for H3X

        A[173] = 8.0*pi*c1*sbeta_*s1p3;
        A[174] = -4.0*pi*cbeta_*s1p4;
        A[175] = -8.0*pi*c1p3*sbeta_*s1;
        A[176] = -4.0*pi*cbeta_*c1p4;
        A[177] = delta*(c1p4*(-4375.0/384.0 *s2beta_ - 4375.0/256.0 *s4beta_)*s1p6 + eta*c1p4*(4375.0/192.0 *s2beta_ + 4375.0/128.0 *s4beta_)*s1p6);
        A[178] = delta*(625.0/96.0 *c2beta_ + 625.0/32.0 *c4beta_ - eta*(625.0/48.0 *c2beta_ + 625.0/16.0 *c4beta_))*c1p3*s1p7;
        A[179] = delta*(-625.0/256.0 *s2beta_ + 5625.0/512.0 *s4beta_ + eta*(625.0/128.0 *s2beta_ - 5625.0/256.0 *s4beta_))*c1p2*s1p8;
        A[180] = delta*(625.0/96.0 + 625.0/48.0 *c2beta_ - eta*(625.0/48.0 + 625.0/24.0 *c2beta_))*c1*sbeta_2*s1p9;
        A[181] = delta*(625.0/192.0 - 625.0/96.0 *eta)*cbeta_*sbeta_3*s1p10;
        A[182] = delta*(eta*c1p2*(-4923.0/512.0 *s2beta_ + c2*(459.0/128.0 *s2beta_ - 2079.0/256.0 *s4beta_) 
                                  - 945.0/1024.0 *s4beta_ + c4*(567.0/512.0 *s2beta_ + 1701.0/1024.0 *s4beta_))*s1p4 + c1p2*(22203.0/1024.0 *s2beta_ - c4*(567.0/1024.0 *s2beta_ + 1701.0/2048.0 *s4beta_) 
                                                                                                                             + 945.0/2048.0 *s4beta_ + c2*(-459.0/256.0 *s2beta_ + 2079.0/512.0 *s4beta_))*s1p4);
        A[183] = delta*(eta*c1*(27.0/16.0 + 1233.0/128.0 *c2beta_ + 27.0/128.0 *c4beta_ + (27.0/8.0 + 27.0/16.0 *c2beta_ + 27.0/16.0 *c4beta_)*c2 
                                - (81.0/128.0 *c2beta_ + 243.0/128.0 *c4beta_)*c4)*s1p5 + c1*(-27.0/32.0 - 4689.0/256.0 *c2beta_ - 27.0/256.0 *c4beta_ - (27.0/16.0 + 27.0/32.0 *c2beta_ + 27.0/32.0 *c4beta_)*c2   
                                                                                              + (81.0/256.0 *c2beta_ + 243.0/256.0 *c4beta_)*c4)*s1p5);
        A[184] = delta*(eta*((4761.0/1024.0 - 1377.0/1024.0 *c2beta_)*s2beta_ + (837.0/256.0 - 621.0/256.0 *c2beta_)*c2*s2beta_ + (243.0/1024.0 - 2187.0/1024.0 *c2beta_)*c4*s2beta_)*s1p6 
                        + ((-11673.0/2048.0 + 1377.0/2048.0 *c2beta_)*s2beta_ + (-837.0/512.0 + 621.0/512.0 *c2beta_)*c2*s2beta_ + (-243.0/2048.0 + 2187.0/2048.0 *c2beta_)*c4*s2beta_)*s1p6);
        A[185] = delta*(eta*c1*((81.0/32.0 - 27.0/16.0 *c2beta_)*sbeta_2 - (81.0/32.0 + 81.0/16.0 *c2beta_)*c2*sbeta_2)*s1p7 
                        + c1*((-81.0/64.0 + 27.0/32.0 *c2beta_)*sbeta_2 + (81.0/64.0 + 81.0/32.0 *c2beta_)*c2*sbeta_2)*s1p7);
        A[186] = delta*(81.0/64.0 - 81.0/32.0 *eta)*cbeta_*c1p2*sbeta_3*s1p8;
        A[187] = delta*(683.0/16384.0 *cbeta_*sbeta_ + (557.0/4096.0 - 11.0/12288.0 *c2beta_)*c4*s2beta_ + (-1719.0/32768.0 + 91.0/32768.0 *c2beta_)*c6*s2beta_ 
                        - 1.0/16384.0 *cbeta_*s3beta_ + c2*(-10511.0/49152.0 *cbeta_*sbeta_ + 173.0/49152.0 *cbeta_*s3beta_) + eta*(85.0/8192.0 *cbeta_*sbeta_ + (-679.0/6144.0 + 11.0/6144.0 *c2beta_)*c4*s2beta_   
                                                                                                                                    - (201.0/16384.0 + 91.0/16384.0 *c2beta_)*c6*s2beta_ + 1.0/8192.0 *cbeta_*s3beta_ + c2*(6031.0/24576.0 *cbeta_*sbeta_ - 173.0/24576.0 *cbeta_*s3beta_) 
                                                                                                                                    - c10*(7.0/49152.0 *s2beta_ + 7.0/32768.0 *s4beta_) + c8*(-37.0/24576.0 *s2beta_ + 91.0/16384.0 *s4beta_)) 
                        + c8*(37.0/49152.0 *s2beta_ - 91.0/32768.0 *s4beta_) + c10*(7.0/98304.0 *s2beta_ + 7.0/65536.0 *s4beta_));
        A[188] = delta*(eta*(19.0/512.0 *c4beta_*c3 + 9.0/512.0 *c4beta_*c5 + c1*(-11.0/16.0 - 35.0/128.0 *c2beta_ + 79.0/1536.0 *c4beta_ + (1.0/32.0 - 37.0/256.0 *c2beta_)*c2 
                                                                                  + (1.0/32.0 + 3.0/128.0 *c2beta_)*c4 - 1.0/768.0 *c2beta_*c6) - 1.0/512.0 *c4beta_*c7)*s1p3 + (-19.0/1024.0 *c4beta_*c3 - 9.0/1024.0 *c4beta_*c5 
                                                                                                                                                                                 + c1*(19.0/32.0 - 23.0/768.0 *c2beta_ - 79.0/3072.0 *c4beta_ - (1.0/64.0 + 347.0/512.0 *c2beta_)*c2 - (1.0/64.0 + 3.0/256.0 *c2beta_)*c4 + 1.0/1536.0 *c2beta_*c6) 
                                                                                                                                                                                 + 1.0/1024.0 *c4beta_*c7)*s1p3);
        A[189] = delta*(c1p2*(-355.0/1024.0 *s2beta_ - c2*(13.0/256.0 *s2beta_ + 11.0/512.0 *s4beta_) + c4*(-1.0/1024.0 *s2beta_ + 9.0/2048.0 *s4beta_) - 5.0/2048.0 *s4beta_)*s1p4 
                        + eta*c1p2*(-29.0/512.0 *s2beta_ + c4*(1.0/512.0 *s2beta_ - 9.0/1024.0 *s4beta_) + c2*(13.0/128.0 *s2beta_ + 11.0/256.0 *s4beta_) + 5.0/1024.0 *s4beta_)*s1p4);
        A[190] = delta*(eta*c1p3*((7.0/48.0 + 1.0/24.0 *c2beta_)*sbeta_2 - (1.0/48.0 + 1.0/24.0 *c2beta_)*c2*sbeta_2)*s1p5 
                        + c1p3*(-(7.0/96.0 + 1.0/48.0 *c2beta_)*sbeta_2 + (1.0/96.0 + 1.0/48.0 *c2beta_)*c2*sbeta_2)*s1p5);
        A[191] = delta*(1.0/96.0 - 1.0/48.0 *eta)*cbeta_*c1p4*sbeta_3*s1p6;
        A[192] = delta*((-77.0/256.0 + 1.0/256.0 *cbeta_)*sbeta_2*s4 + (5.0/512.0 + 7.0/512.0 *c2beta_)*sbeta_2*s8 
                        + eta*((45.0/128.0 - 1.0/128.0 *c2beta_)*sbeta_2*s4 - (5.0/256.0 - 7.0/256.0 *c2beta_)*sbeta_2*s8));
        A[193] = delta*(135.0/64.0 + 189.0/64.0 *c2beta_ - eta*(135.0/32.0 + 189.0/32.0 *c2beta_))*c2*sbeta_2*s2p3;
        A[194] = delta*(-683.0/16384.0 *cbeta_*sbeta_ + (-557.0/4096.0 + 11.0/12288.0 *c2beta_)*c4*s2beta_ + (-1719.0/32768.0 + 91.0/32768.0 *c2beta_)*c6*s2beta_ 
                        + 1.0/16384.0 *cbeta_*sbeta_ + c2*(-10511.0/49152.0 *cbeta_*sbeta_ + 173.0/49152.0 *cbeta_*s3beta_) + eta*(-85.0/8192.0 *cbeta_*sbeta_ + (679.0/6144.0 - 11.0/6144.0 *c2beta_)*c4*s2beta_   
                                                                                                                                   - (201.0/16384.0 + 91.0/16384.0 *c2beta_)*c6*s2beta_ - 1.0/8192.0 *cbeta_*s3beta_ + c2*(6031.0/24576.0 *cbeta_*sbeta_ - 173.0/24576.0 *cbeta_*s3beta_) + c8*(37.0/24576.0 *s2beta_ 
                                                                                                                                                                                                                                                                                                - 91.0/16384.0 *s4beta_) - c10*(7.0/49152.0 *s2beta_ + 7.0/32768.0 *s4beta_)) + c10*(7.0/98304.0 *s2beta_ + 7.0/65536.0 *s4beta_) + c8*(-37.0/49152.0 *s2beta_ + 91.0/32768.0 *s4beta_));
        A[195] = delta*(c1p3*(19.0/32.0 - 23.0/768.0 *c2beta_ - 79.0/3072.0 *c4beta_ + (1.0/64.0 + 347.0/512.0 *c2beta_)*c2 - (1.0/64.0 + 3.0/256.0 *c2beta_)*c4 - 1.0/1536.0 *c2beta_*c6)*s1 
                        + 19.0/1024.0 *c4beta_*c1p3*s3 - 9.0/1024.0 *c4beta_*c1p3*s5 - 1.0/1024.0 *c4beta_*c1p3*s7 + eta*(c1p3*(-11.0/16.0 - 35.0/128.0 *c2beta_ + 79.0/1536.0 *c4beta_ + (-1.0/32.0 + 37.0/256.0 *c2beta_)*c2  
                                                                                                                                +(1.0/32.0 + 3.0/128.0 *c2beta_)*c4 + 1.0/768.0 *c2beta_*c6)*s1 - 19.0/512.0 *c4beta_*c1p3*s3 + 9.0/512.0 *c4beta_*c1p3*s5 + 1.0/512.0 *c4beta_*c1p3*s7));
        A[196] = delta*(eta*c1p4*(4923.0/512.0 *s2beta_ + c4*(567.0/1024.0 *s2beta_ + 1701.0/2048.0 *s4beta_) - 945.0/2048.0 *s4beta_ + c2*(-459.0/256.0 *s2beta_ + 2079.0/512.0 *s4beta_))*s1p2);
        A[197] = delta*(eta*c1p5*(27.0/16.0 + 1233.0/128.0 *c2beta_ + 27.0/128.0 *c4beta_ - (27.0/8.0 + 27.0/16.0 *c2beta_ + 27.0/16.0 *c4beta_)*c2 - (81.0/128.0 *c2beta_ + 243.0/128.0 *c4beta_)*c4)*s1 
                        + c1p5*(-27.0/32.0 - 4689.0/256.0 *c2beta_ - 27.0/256.0 *c4beta_ + (27.0/16.0 + 27.0/32.0 *c2beta_ + 27.0/32.0 *c4beta_)*c2 + (81.0/256.0 *c2beta_ + 243.0/256.0 *c4beta_)*c4)*s1);
        // The multiplication denominator, first time seeing that.;
        A[198] = delta*(c1p6*(11673.0/2048.0 *s2beta_ + c4*(243.0/2048.0 *s2beta_ - 2187.0/4096.0 *s4beta_) + c2*(-837.0/512.0 *s2beta_ + 621.0/1024.0 *s4beta_) - 1377.0/4096.0 *s4beta_) 
                        + eta*c1p6*(-4761.0/1024.0 *s2beta_ + c2*(837.0/256.0 *s2beta_ - 621.0/512.0 *s4beta_) + 1377.0/2048.0 *s4beta_ + c4*(-243.0/1024.0 *s2beta_ + 2187.0/2048.0*s4beta_)));
        A[199] = delta*(c1p7*((-81.0/64.0 + 27.0/32.0 *c2beta_)*sbeta_2 - (81.0/64.0 + 81.0/32.0 *c2beta_)*c2*sbeta_2)*s1  
                        + eta*c1p7*((81.0/32.0 - 27.0/16.0 *c2beta_)*sbeta_2 + (81.0/32.0 + 81.0/16.0 *c2beta_)*c2*sbeta_2)*s1);
        A[200] = delta*(81.0/32.0 *eta - 81.0/64.0)*cbeta_*c1p8*sbeta_3*s1p2;
        A[201] = delta*(4375.0/384.0 *s2beta_ + 4375.0/256.0 *s4beta_ - eta*(4375.0/192.0 *s2beta_ + 4375.0/128.0 *s4beta_))*c1p6*s1p4;
        A[202] = delta*(625.0/96.0 *c2beta_ + 625.0/32.0 *c4beta_ - eta*(625.0/48.0 *c2beta_ + 625.0/16.0 *c4beta_))*c1p7*s1p3;
        A[203] = delta*(625.0/256.0 *s2beta_ - 5625.0/512.0 *s4beta_ + eta*(-625.0/128.0 *s2beta_ + 5625.0/256.0 *s4beta_))*c1p8*s1p2;
        A[204] = delta*(625.0/96.0 + 625.0/48.0 *c2beta_ - eta*(625.0/48.0 + 625.0/24.0 *c2beta_))*c1p9*sbeta_2*s1;
        A[205] = delta*(625.0/96.0 *eta - 625.0/192.0)*cbeta_*c1p10*sbeta_3;
        //HX 3.0/2.0, SO
        A[206] = chisyDN*(2.0*c2p3*sbeta_ - eta*c2p3*sbeta_);
        A[207] = chisyDN*(eta*c1p4*(-5.0/3.0 *sbeta_ + 2.0/3.0 *c2*sbeta_) + c1p4*(-14.0/3.0 *sbeta_ + 20.0/3.0 *c2*sbeta_));
        A[208] = chisyDN*(-20.0/3.0 *cbeta_*c1p5*s1 - 2.0/3.0 *eta*cbeta_*c1p5*s1);
        A[209] = chisyDN*(eta*c1p3*(7.0/3.0 *cbeta_ + 1.0/3.0 *cbeta_*c2)*s1 + c1p3*(-2.0/3.0 *cbeta_ + 10.0/3.0 *cbeta_*c2)*s1);
        A[210] = chisyDN*(eta*c1*(7.0/3.0 *cbeta_ - 1.0/3.0 *cbeta_*c2)*s1p3 + c1*(-2.0/3.0 *cbeta_ - 10.0/3.0 *cbeta_*c2)*s1p3);
        // The bigg?;
        //A[211] = chisyDN*(eta*(5.0/3.0 *sbeta_ + 2.0/3.0 *c2*sbeta_)*s1p4 + bigg*(14.0/3.0 *sbeta_ + 20.0/3.0 *c2*sbeta_)*s1p4)
        A[211] = chisyDN*s1p4*sbeta_*(7.0+5.0/2.0*eta + c2*(10.0 + eta));
        A[212] = chisyDN*(-20.0/3.0 *cbeta_*c1*s1p5 - 2.0/3.0 *eta*cbeta_*c1*s1p5);
        A[213] = chisyDN*(2.0*c2*sbeta_*s2p2 - eta*c2*sbeta_*s2p2);
        A[214] = chisyDN*(10.0/3.0 *c2*sbeta_*s2p2 + 1.0/3.0 *eta*c2*sbeta_*s2p2);
        A[215] = chisyDN*(-cbeta_*s2p3 + 1.0/2.0 *eta*cbeta_*s2p3);
        A[216] = chisyDN*(-5.0/4.0 *cbeta_*s2 - 1.0/4.0 *cbeta_*s6 + eta*(5.0/8.0 *cbeta_*s2 + 1.0/8.0 *cbeta_*s6));
        A[217] = chisxDN*((-3.0/2.0 *cbeta_ - 1.0/2.0 *cbeta_*c4)*s2 + eta*(3.0/4.0 *cbeta_ + 1.0/4.0 *cbeta_*c4)*s2) + chiszDN*(-2.0*c4*sbeta_*s2 + eta*c4*sbeta_*s2);
        A[218] = chiszDN*(2.0*cbeta_*c2*s2p2 - eta*cbeta_*c2*s2p2) + chisxDN*(-2.0*c2*sbeta_*s2p2 + eta*c2*sbeta_*s2p2);
        A[219] = chisxDN*(cbeta_*s2p3 - 1.0/2.0 *eta*cbeta_*s2p3);
        A[220] = chisxDN*(c1*(-2.0/3.0 *cbeta_ - 10.0/3.0 *cbeta_*c2)*s1p3 + eta*c1*(-5.0/3.0 *cbeta_ + 4.0*c3beta_ - 1.0/3.0 *cbeta_*c2)*s1p3)
            + chiszDN*(c1*(-4.0*sbeta_ - 40.0/3.0 *c2*sbeta_)*s1p3 + eta*c1*(-2.0*sbeta_ - 4.0/3.0 *c2*sbeta_ - 4.0*s3beta_)*s1p3);
        A[221] = chiszDN*(eta*(5.0*cbeta_ + c3beta_ + 2.0/3.0 *cbeta_*c2)*s1p4 + (4.0*cbeta_ + 20.0/3.0 *cbeta_*c2)*s1p4) 
            + chisxDN*((-14.0/3.0 *sbeta_ - 20.0/3.0 *c2*sbeta_)*s1p4 + eta*(10.0/3.0 *sbeta_ - 2.0/3.0 *c2*sbeta_ + s3beta_)*s1p4);
        A[222] = chisxDN*(20.0/3.0 *cbeta_*c1*s1p5 + 2.0/3.0 *eta*cbeta_*c1*s1p5);
        A[223] = -6.0*eta*chiszDN*cbeta_*sbeta_2*s2p2 + chisxDN*(1.0/3.0 *sbeta_*s2p2 + eta*(-7.0/6.0 + 3.0*c2beta_)*sbeta_*s2p2);
        A[224] = chisxDN*(eta*c1p3*(-5.0/3.0 *cbeta_ + 4.0*c3beta_ + 1.0/3.0 *cbeta_*c2)*s1 + c1p3*(-2.0/3.0 *cbeta_ + 10.0/3.0 *cbeta_*c2)*s1) 
            + chiszDN*(c1p3*(-4.0*sbeta_ + 40.0/3.0 *c2*sbeta_)*s1 + eta*c1p3*(-2.0*sbeta_ + 4.0/3.0 *c2*sbeta_ - 4.0*s3beta_)*s1);
        A[225] = chiszDN*(eta*c1p4*(-5.0*cbeta_ - c3beta_ + 2.0/3.0 *cbeta_*c2) + c1p4*(-4.0*cbeta_ + 20.0/3.0 *cbeta_*c2)) 
            + chisxDN*(c1p4*(14.0/3.0 *sbeta_ - 20.0/3.0 *c2*sbeta_) + eta*c1p4*(-10.0/3.0 *sbeta_ - 2.0/3.0 *c2*sbeta_ - s3beta_));
        A[226] = chisxDN*(20.0/3.0 *cbeta_*c1p5*s1 + 2.0/3.0 *eta*cbeta_*c1p5*s1);
        A[227] = delta*(2.0*chiayDN*c2p3*sbeta_);
        A[228] = delta*(chiayDN*c1p4*(-14.0/3.0 *sbeta_ + 20.0/3.0 *c2*sbeta_));
        A[229] = delta*(chiayDN*c1p3*(-2.0/3.0 *cbeta_ + 10.0/3.0 *cbeta_*c2)*s1);
        A[230] = delta*(-20.0/3.0 *chiayDN*cbeta_*c1p5*s1);
        A[231] = delta*(chiayDN*c1*(-2.0/3.0 *cbeta_ - 10.0/3.0 *cbeta_*c2)*s1p3);
        A[232] = delta*(chiayDN*(14.0/3.0 *sbeta_ + 20.0/3.0 *c2*sbeta_)*s1p4);
        A[233] = delta*(-20.0/3.0 *chiayDN*cbeta_*c1*s1p5);
        A[234] = delta*(2.0*chiayDN*c2*sbeta_*s2p2);
        // Interesting + sign in sheet
        A[235] = delta*(10.0/3.0 *chiayDN*c2*sbeta_*s2p2);
        A[236] = delta*(-chiayDN*cbeta_*s1p3);
        A[237] = delta*(chiayDN*(-5.0/4.0 *cbeta_*s2 - 1.0/4.0 *cbeta_*s6));
        A[238] = delta*(chiaxDN*(-3.0/2.0 *cbeta_ - 1.0/2.0 *cbeta_*c4)*s2 - 2.0*chiazDN*c4*sbeta_*s2);
        A[239] = delta*(2.0*chiazDN*cbeta_*c2*s2p2 - 2.0*chiaxDN*c2*sbeta_*s2p2);
        A[240] = delta*(chiaxDN*cbeta_*s2p3);
        A[241] = delta*(chiaxDN*c1*(-2.0/3.0 *cbeta_ - 10.0/3.0 *cbeta_*c2)*s1p3 + chiazDN*c1*(-4.0*sbeta_ - 40.0/3.0 *c2*sbeta_)*s1p3);
        A[242] = delta*(chiazDN*(4.0*cbeta_ + 20.0/3.0 *cbeta_*c2)*s1p4 + chiaxDN*(-14.0/3.0 *sbeta_ - 20.0/3.0 *c2*sbeta_)*s1p4);
        A[243] = delta*(20.0/3.0 *chiaxDN*cbeta_*c1*s1p5);
        A[244] = delta*(1.0/3.0 *chiaxDN*sbeta_*s2p2);
        A[245] = delta*(chiaxDN*c1p3*(-2.0/3.0 *cbeta_ + 10.0/3.0 *cbeta_*c2)*s1 + chiazDN*c1p3*(-4.0*sbeta_ + 40.0/3.0 *c2*sbeta_)*s1);
        A[246] = delta*(chiazDN*c1p4*(-4.0*cbeta_ + 20.0/3.0 *cbeta_*c2) + chiaxDN*c1p4*(14.0/3.0 *sbeta_ - 20.0/3.0 *c2*sbeta_));
        A[247] = delta*(20.0/3.0 *chiaxDN*cbeta_*c1p5*s1);


            //}
    }
}
