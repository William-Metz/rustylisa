use std::usize;

use crate::wave_builder::wave_builder::WaveBuilder;
impl WaveBuilder{
    // These constants define static variables indicating the endpoints of certain polarizations
    pub fn sum_source_h(&mut self, do_v_deriv: bool){

        let h0_p_last_index: usize = 5;
        let h1_p_last_index: usize= 19;
        let h2_p_last_index: usize = 47;
        let h3_p_last_index: usize = 129;
        let h0_x_last_index: usize = 133;
        let h1_x_last_index: usize = 146;
        let h2_x_last_index: usize = 173;
        let h3_x_last_index: usize = 248;

        // First, do the plus polarization
        let mut j_start: usize; 
        let mut sum: f64 = 0.0;
        for j in 0.. h0_p_last_index {
            sum = sum + self.a[j]*self.w[j];

        }
        let mut vpower: f64;

        if do_v_deriv{
            vpower = 2.0*self.vdn
        } 
        else{
            vpower = self.vdn*self.vdn
        }
        self.hp = sum*vpower;
        if self.pn_order > 0{ 
            sum = 0.0;
            j_start = h0_p_last_index + 1;
            for j in j_start.. h1_p_last_index{
                sum = sum + self.a[j]*self.w[j]
            }
            vpower = vpower*self.vdn;
            if do_v_deriv {
                vpower = 1.5*vpower;
            }
            self.hp = self.hp + sum*vpower
        }
        if self.pn_order> 1 {
            sum = 0.0;
            j_start = h1_p_last_index + 1;
            for j in j_start.. h2_p_last_index{
                sum = sum + self.a[j]*self.w[j];
            }
            vpower = vpower*self.vdn;
            if do_v_deriv {
                vpower = 1.33333333333333333*vpower;
            }
            self.hp = self.hp + sum*vpower;
        }
        if self.pn_order > 2{ 
            j_start = h2_p_last_index + 1;
            sum = 0.0;
            for j in j_start.. h3_p_last_index{
                sum = sum + self.a[j]*self.w[j];
            }
            vpower = vpower*self.vdn;
            if do_v_deriv {
                vpower = 1.25*vpower;
            }
            self.hp = self.hp + sum*vpower;
        }

        // now assemble cross polarization
        sum = 0.0;
        j_start = h3_p_last_index + 1;
        for j in j_start.. h0_x_last_index{
            sum = sum + self.a[j]*self.w[j];
        }
        if do_v_deriv{ 
            vpower = 2.0*self.vdn;
        }
        else{
            vpower = self.vdn*self.vdn;
        }
        self.hx = sum*vpower;

        if self.pn_order > 0{ 
            sum = 0.0;
            j_start = h0_x_last_index + 1;
            for j in j_start.. h1_x_last_index{
                sum = sum + self.a[j]*self.w[j];
            }
            vpower = vpower*self.vdn;
            if do_v_deriv {
                vpower = 1.5*vpower;
            }
            self.hx = self.hx + sum*vpower;
        }

        if self.pn_order > 1 {
            sum = 0.0;
            j_start = h1_x_last_index + 1;
            for j in j_start.. h2_x_last_index{
                sum = sum + self.a[j]*self.w[j];
            }
            vpower = vpower*self.vdn;
            if do_v_deriv {
                vpower = 1.3333333333333333*vpower;
            }
            self.hx = self.hx + sum*vpower
        }

        if self.pn_order > 2 {
            j_start = h2_x_last_index + 1;
            sum = 0.0;
            for j in j_start.. h3_x_last_index{
            sum = sum + self.a[j]*self.w[j];
            }
            vpower = vpower*self.vdn;
            if do_v_deriv {
                vpower = 1.25*vpower;
            }
            self.hx = self.hx + sum*vpower;
        }

            // Calculate overall wave amplitude constant
            let mut h0: f64 = 0.5*(1.0 - self.spin_evolver.test_case.delta*self.spin_evolver.test_case.delta)*self.spin_evolver.test_case.GM/(self.spin_evolver.test_case.R);
            self.hp = h0*self.hp;
            self.hx = h0*self.hx;
    }
}
