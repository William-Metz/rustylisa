use std::usize;

use crate::wave_builder::wave_builder::WaveBuilder;
impl WaveBuilder{
    // These constants define static variables indicating the endpoints of certain polarizations
    pub fn sum_source_h(&mut self, DoVDeriv: bool){

        let H0PLastIndex: usize = 4;
        let H1PLastIndex: usize= 18;
        let H2PLastIndex: usize = 46;
        let H3PLastIndex: usize = 128;
        let H0XLastIndex: usize = 132;
        let H1XLastIndex: usize = 145;
        let H2XLastIndex: usize = 172;
        let H3XLastIndex: usize = 247;

        // First, do the plus polarization
        let mut jStart: usize; 
        let mut sum: f64 = 0.0;
        for j in 0.. H0PLastIndex {
            sum = sum + self.A[j]*self.w[j];
        }
        let mut vpower: f64;

        if DoVDeriv{
            vpower = 2.0*self.VDN
        } 
        else{
            vpower = self.VDN*self.VDN
        }
        self.HP = sum*vpower;
        println!("pn{}",self.pn_order);
 

        if self.pn_order > 0{ 
            sum = 0.0;
            jStart = H0PLastIndex + 1;
            for j in jStart.. H1PLastIndex{
                sum = sum + self.A[j]*self.w[j]
            }
            vpower = vpower*self.VDN;
            if DoVDeriv {
                vpower = 1.5*vpower;
            }
            self.HP = self.HP + sum*vpower
        }
        if self.pn_order> 1 {
            sum = 0.0;
            jStart = H1PLastIndex + 1;
            for j in jStart.. H2PLastIndex{
                sum = sum + self.A[j]*self.w[j];
            }
            vpower = vpower*self.VDN;
            if DoVDeriv {
                vpower = 1.33333333333333333*vpower;
            }
            self.HP = self.HP + sum*vpower;
        }
        if self.pn_order > 2{ 
            jStart = H2PLastIndex + 1;
            sum = 0.0;
            for j in jStart.. H3PLastIndex{
                sum = sum + self.A[j]*self.w[j];
            }
            vpower = vpower*self.VDN;
            if DoVDeriv {
                vpower = 1.25*vpower;
            }
            self.HP = self.HP + sum*vpower;
        }

        // now assemble cross polarization
        sum = 0.0;
        vpower = self.VDN*self.VDN;
        jStart = H3PLastIndex + 1;
        for j in jStart.. H0XLastIndex{
            sum = sum + self.A[j]*self.w[j];
        }
        if DoVDeriv{ 
            vpower = 2.0*self.VDN;
        }
        else{
            vpower = self.VDN*self.VDN;
        }
        self.HX = sum*vpower;

        if self.pn_order > 0{ 
            sum = 0.0;
            jStart = H0XLastIndex + 1;
            for j in jStart.. H1XLastIndex{
                sum = sum + self.A[j]*self.w[j];
            }
            vpower = vpower*self.VDN;
            if DoVDeriv {
                vpower = 1.5*vpower;
            }
            self.HX = self.HX + sum*vpower;
        }

        if self.pn_order > 1 {
            sum = 0.0;
            jStart = H1XLastIndex + 1;
            for j in jStart.. H2XLastIndex{
                sum = sum + self.A[j]*self.w[j];
            }
            vpower = vpower*self.VDN;
            if DoVDeriv {
                vpower = 1.3333333333333333*vpower;
            }
            self.HX = self.HX + sum*vpower
        }

        if self.pn_order > 2 {
            jStart = H2XLastIndex + 1;
            sum = 0.0;
            for j in jStart.. H3XLastIndex{
            sum = sum + self.A[j]*self.w[j];
            }
            vpower = vpower*self.VDN;
            if DoVDeriv {
                vpower = 1.25*vpower;
            }
            self.HX = self.HX + sum*vpower;
        }

            // Calculate overall wave amplitude constant
            let mut h0: f64 = 0.5*(1.0 - self.spin_evolver.test_case.delta*self.spin_evolver.test_case.delta)*self.spin_evolver.test_case.GM/(self.spin_evolver.test_case.R);
            self.HP = h0*self.HP;
            self.HX = h0*self.HX;
    }
}
