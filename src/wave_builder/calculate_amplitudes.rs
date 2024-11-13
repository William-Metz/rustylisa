// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
// CalculateAmplitudes.rs
use crate::wave_builder::wave_builder::WaveBuilder;
use std::f64::consts::PI;

// Now calculate all wave amplitudes
impl WaveBuilder {
    pub fn calculate_amplitudes(&mut self) {
        // Calculate some useful trig functions of angle lota
        // Input stuff
        //
        let c2: f64 = (self.lota_dn).cos();
        let s2: f64 = (self.lota_dn).sin();
        let c1: f64 = (0.5 * self.lota_dn).cos();
        let s1: f64 = (0.5 * self.lota_dn).sin();
        let c3: f64 = c2 * c1 - s2 * s1;
        let s3: f64 = s2 * c1 + c2 * s1;
        let c4: f64 = c2 * c2 - s2 * s2;
        let s4: f64 = 2.0 * c2 * s2;
        let c5: f64 = c4 * c1 - s4 * s1;
        let s5: f64 = s4 * c1 + c4 * s1;
        let c6: f64 = c5 * c1 - s5 * s1;
        let s6: f64 = s5 * c1 + c5 * s1;
        let c7: f64 = c6 * c1 - s6 * s1;
        let s7: f64 = s6 * c1 + c6 * s1;
        let c8: f64 = c7 * c1 - s7 * s1;
        let s8: f64 = s7 * c1 + c7 * s1;
        let c9: f64 = c8 * c1 - s8 * s1;
        let s9: f64 = s8 * c1 + c8 * s1;
        let c10: f64 = c9 * c1 - s9 * s1;
        let s10: f64 = s9 * c1 + c9 * s1;
        let c1p2: f64 = c1 * c1;
        let c1p3: f64 = c1p2 * c1;
        let c1p4: f64 = c1p3 * c1;
        let c1p5: f64 = c1p4 * c1;
        let c1p6: f64 = c1p5 * c1;
        let c1p7: f64 = c1p6 * c1;
        let c1p8: f64 = c1p7 * c1;
        let c1p9: f64 = c1p8 * c1;
        let c1p10: f64 = c1p9 * c1;
        let s1p2: f64 = s1 * s1;
        let s1p3: f64 = s2 * s1;
        let s1p4: f64 = s3 * s1;
        let s1p5: f64 = s4 * s1;
        let s1p6: f64 = s5 * s1;
        let s1p7: f64 = s6 * s1;
        let s1p8: f64 = s7 * s1;
        let s1p9: f64 = s8 * s1;
        let s1p10: f64 = s9 * s1;
        let c2p2: f64 = c2 * c2;
        let c2p3: f64 = c2p2 * c2;
        let s2p2: f64 = s2 * s2;
        let s2p3: f64 = s2p2 * s2;
        let s2p4: f64 = s2p3 * s2;
        let s2p5: f64 = s2p4 * s2;

        // Define local beta_ trig functions
        let cbeta_: f64 = self.beta_.cos();
        let sbeta_: f64 = self.beta_.sin();
        let c2beta_: f64 = cbeta_ * cbeta_ - sbeta_ * sbeta_;
        let s2beta_: f64 = 2.0 * sbeta_ * cbeta_;
        let c3beta_: f64 = c2beta_ * cbeta_ - s2beta_ * sbeta_;
        let s3beta_: f64 = s2beta_ * cbeta_ + c2beta_ * sbeta_;
        let c4beta_: f64 = c3beta_ * cbeta_ - s3beta_ * sbeta_;
        let s4beta_: f64 = s3beta_ * cbeta_ + c3beta_ * sbeta_;
        let c5beta_: f64 = c4beta_ * cbeta_ - s4beta_ * sbeta_;
        let s5beta_: f64 = s4beta_ * cbeta_ + c4beta_ * sbeta_;
        let cbeta_2: f64 = cbeta_ * cbeta_;
        let cbeta_3: f64 = c2beta_ * cbeta_;
        let sbeta_2: f64 = sbeta_ * sbeta_;
        let sbeta_3: f64 = s2beta_ * sbeta_;
        self.a = [0.0; 248];
        // Amplitudes for H0P
        self.a[0] = (-1.5 - 0.5 * c2beta_) * c1p4;
        self.a[1] = -2.0 * c1p3 * s2beta_ * s1;
        self.a[2] = 2.0 * s1p3 * s2beta_ * c1;
        self.a[3] = (-1.5 - 0.5 * c2beta_) * s1p4;
        self.a[4] = -1.5 * sbeta_2 * s2p2;
        // Amplitudes for H0X
        self.a[129] = 4.0 * sbeta_ * c1 * s1p3;
        self.a[130] = -2.0 * cbeta_ * s1p4;
        self.a[131] = -4.0 * sbeta_ * c1p3 * s1;
        self.a[132] = -2.0 * cbeta_ * c1p4;

        if self.pn_order > 0 {
            // Amplitude factors for H1P

            self.a[5] = self.delta * c1p6 * (-45.0 / 32.0 * sbeta_ - 9.0 / 32.0 * s3beta_);
            self.a[6] = self.delta
                * c1p2
                * (-175.0 / 256.0 * sbeta_
                    + c2 * (87.0 / 64.0 * sbeta_ - 5.0 / 64.0 * s3beta_)
                    + c4 * (-5.0 / 256.0 * sbeta_ + 15.0 / 256.0 * s3beta_)
                    + 13.0 / 256.0 * s3beta_);
            self.a[7] = self.delta
                * s1p2
                * (175.0 / 256.0 * sbeta_
                    + c2 * (87.0 / 64.0 * sbeta_ - 5.0 / 64.0 * s3beta_)
                    + c4 * (5.0 / 256.0 * sbeta_ - 15.0 / 256.0 * s3beta_)
                    - 13.0 / 256.0 * s3beta_);
            self.a[8] = self.delta * c1p4 * s1p2 * (-5.0 / 32.0 * sbeta_ - 1.0 / 32.0 * s3beta_);
            self.a[9] = self.delta * c1p4 * s1p2 * (-45.0 / 32.0 * sbeta_ + 135.0 / 32.0 * s3beta_);
            self.a[10] = self.delta * c1p2 * s1p4 * (45.0 / 32.0 * sbeta_ - 135.0 / 32.0 * s3beta_);
            self.a[11] = self.delta * c1p2 * s1p4 * (5.0 / 32.0 * sbeta_ + 1.0 / 32.0 * s3beta_);
            self.a[12] = self.delta * s1p6 * sbeta_ * (27.0 / 16.0 + 9.0 / 16.0 * c2beta_);
            self.a[13] = self.delta * 45.0 / 16.0 * s2p3 * cbeta_ * sbeta_2;
            self.a[14] = self.delta
                * ((-85.0 / 256.0 * cbeta_
                    - 1.0 / 128.0 * cbeta_ * c2beta_
                    - 1.0 / 32.0 * cbeta_ * c2beta_ * c2
                    - 3.0 / 128.0 * cbeta_ * c2beta_ * c4)
                    * s2
                    - 11.0 / 64.0 * cbeta_ * s4
                    - 1.0 / 256.0 * cbeta_ * s6);
            self.a[15] = self.delta
                * ((45.0 / 256.0 * cbeta_
                    + 81.0 / 128.0 * cbeta_ * c2beta_
                    + 27.0 / 32.0 * cbeta_ * c2beta_ * c2
                    + 27.0 / 128.0 * cbeta_ * c2beta_ * c4)
                    * s2
                    + 9.0 / 64.0 * cbeta_ * s4
                    + 9.0 / 256.0 * cbeta_ * s6);
            self.a[16] = self.delta
                * ((1.0 / 256.0 * cbeta_ * c2beta_ - 85.0 / 256.0 * cbeta_) * s2
                    + (11.0 / 64.0 * cbeta_ + 1.0 / 64.0 * cbeta_ * c2beta_) * s4
                    - (1.0 / 256.0 * cbeta_ + 3.0 / 256.0 * cbeta_ * c2beta_) * s6);
            self.a[17] = self.delta
                * ((45.0 / 256.0 * cbeta_ + 135.0 / 256.0 * cbeta_ * c2beta_) * s2
                    - (9.0 / 64.0 * cbeta_ + 27.0 / 64.0 * cbeta_ * c2beta_) * s4
                    + (9.0 / 256.0 * cbeta_ + 27.0 / 256.0 * cbeta_ * c2beta_) * s6);
            self.a[18] = self.delta
                * (1.0 / 64.0 * cbeta_ * sbeta_2 * s2 + 5.0 / 64.0 * cbeta_ * sbeta_2 * s6);

            // AmPItude factors for H1X

            self.a[136] = self.delta
                * (-1.0 / 64.0 * cbeta_ * sbeta_ + 43.0 / 128.0 * cbeta_ * c2 * sbeta_
                    - 23.0 / 128.0 * c4 * s2beta_
                    + 5.0 / 256.0 * c6 * s2beta_);
            self.a[137] = self.delta
                * ((-1.0 - 1.0 / 4.0 * c2beta_) * c1 + 1.0 / 4.0 * c2beta_ * c1 * c2)
                * s1p3;
            self.a[138] = self.delta * (1.0 / 8.0 * c1p2 * s2beta_ * s1p4);
            self.a[139] = self.delta * (1.0 / 2.0 * sbeta_2 * s4);
            self.a[140] = self.delta
                * (1.0 / 64.0 * cbeta_ * sbeta_
                    + 43.0 / 128.0 * cbeta_ * c2 * sbeta_
                    + 23.0 / 128.0 * c4 * s2beta_
                    + 5.0 / 256.0 * c6 * s2beta_);
            self.a[141] = self.delta
                * ((-1.0 - 1.0 / 4.0 * c2beta_) * c1p3 - 1.0 / 4.0 * c2beta_ * c1p3 * c2)
                * s1;
            self.a[142] = -self.delta * (1.0 / 8.0 * c1p4 * s2beta_ * s1p2);
            self.a[143] = self.delta * (45.0 / 8.0 * c1p4 * s2beta_ * s1p2);
            self.a[144] = self.delta * (9.0 / 2.0 * c2beta_ * c1p5 * s1);
            self.a[145] = -self.delta * (9.0 / 8.0 * c1p6 * s2beta_);
            self.a[146] = (4.0 * sbeta_ + 28.0 / 3.0 * s3beta_
                - self.eta * (12.0 * sbeta_ + 28.0 * s3beta_))
                * c1p3
                * s1p5;
            self.a[147] = (self.eta
                * (4.0 * cbeta_ + 28.0 * c3beta_ - (4.0 / 3.0 * cbeta_ + 28.0 / 3.0 * c3beta_)))
                * c1p2
                * s1p6;
            self.a[148] = ((4.0 / 3.0 * sbeta_ - 4.0 * s3beta_)
                + self.eta * (-4.0 * sbeta_ + 12.0 * s3beta_))
                * c1
                * s1p7;
        }
        if self.pn_order > 1 {
            // Amplitude factors for H2P

            self.a[19] = (59.0 / 16.0 + 5.0 / 2.0 * c2beta_ - 3.0 / 16.0 * c4beta_
                + (5.0 / 24.0 - 11.0 / 6.0 * c2beta_ + 7.0 / 24.0 * c4beta_) * c2
                - (5.0 / 48.0 + 1.0 / 12.0 * c2beta_ + 7.0 / 48.0 * c4beta_) * c4)
                * c1p4
                + (-25.0 / 16.0 - 13.0 / 3.0 * c2beta_
                    + 9.0 / 16.0 * c4beta_
                    + (-5.0 / 8.0 + 11.0 / 2.0 * c2beta_ - 7.0 / 8.0 * c4beta_) * c2
                    + (5.0 / 16.0 + 1.0 / 4.0 * c2beta_ + 7.0 / 16.0 * c4beta_) * c4)
                    * self.eta
                    * c1p4;
            self.a[20] = (6.0 + 2.0 * c2beta_) * self.eta * c1p8 * sbeta_2
                - (2.0 + 2.0 / 3.0 * c2beta_) * c1p8 * sbeta_2;
            self.a[21] = 32.0 * (1.0 / 3.0 - self.eta) * cbeta_3 * c1p7 * sbeta_ * s1;
            self.a[22] = ((1.0 / 6.0 * c2beta_ - 5.0 / 6.0) * s2beta_
                - 2.0 / 3.0 * cbeta_2 * c2 * s2beta_
                + self.eta
                    * ((5.0 / 2.0 - 1.0 / 2.0 * c2beta_) * s2beta_ + 2.0 * cbeta_2 * c2 * s2beta_))
                * c1p5
                * s1;
            self.a[23] = (-(10.0 / 3.0 + 8.0 / 3.0 * c2beta_ + 14.0 / 3.0 * c4beta_)
                + self.eta * (10.0 + 8.0 * c2beta_ + 14.0 * c4beta_))
                * c1p6
                * s1p2;
            self.a[24] = 1.0 / 2.0
                * (-(1.0 + 1.0 / 3.0 * c2beta_) + self.eta * (3.0 + c2beta_))
                * c1p6
                * sbeta_2
                * s1p2;
            self.a[25] = (8.0 / 3.0 - 56.0 / 3.0 * c2beta_ + self.eta * (56.0 * c2beta_ - 8.0))
                * c1p5
                * s2beta_
                * s1p3;
            self.a[26] = self.eta
                * (c1
                    * (16.0 / 3.0 * s2beta_
                        + 31.0 / 4.0 * c2 * s2beta_
                        + 1.0 / 4.0 * c4 * s2beta_
                        - 19.0 / 16.0 * s4beta_)
                    - 7.0 / 8.0 * c3 * s4beta_
                    - 7.0 / 16.0 * c5 * s4beta_)
                * s1p3
                + (c1
                    * (-6.0 * s2beta_ - 31.0 / 12.0 * c2 * s2beta_ - 1.0 / 12.0 * c3 * s2beta_
                        + 19.0 / 48.0 * s4beta_)
                    + 7.0 / 24.0 * c3 * s4beta_
                    + 7.0 / 48.0 * c5 * s4beta_)
                    * s1p3;
            // double check
            self.a[27] = (59.0 / 16.0 + 5.0 / 2.0 * c2beta_
                - 3.0 / 16.0 * c4beta_
                - (5.0 / 24.0 - 11.0 / 6.0 * c2beta_ + 7.0 / 24.0 * c4beta_) * c2
                - (5.0 / 48.0 + 1.0 / 12.0 * c2beta_ + 7.0 / 48.0 * c4beta_) * c4)
                * s1p4
                + self.eta
                    * (-25.0 / 16.0 - 13.0 / 3.0 * c2beta_
                        + 9.0 / 16.0 * c4beta_
                        + (5.0 / 8.0 - 11.0 / 2.0 * c2beta_ + 7.0 / 8.0 * c4beta_) * c2
                        + (5.0 / 16.0 + 1.0 / 4.0 * c2beta_ + 7.0 / 16.0 * c4beta_) * c4)
                    * s1p4;
            self.a[28] = (56.0 / 3.0 * c2beta_ - 8.0 / 3.0 + self.eta * (8.0 - 56.0 * c2beta_))
                * c1p3
                * s2beta_
                * s1p5;
            // Interesting "space between number and ["
            self.a[29] = ((5.0 / 6.0 - 1.0 / 6.0 * c2beta_) * s2beta_
                - 2.0 / 3.0 * cbeta_2 * c2 * s2beta_
                + self.eta
                    * ((-5.0 / 2.0 + 1.0 / 2.0 * c2beta_) * s2beta_
                        + 2.0 * cbeta_2 * c2 * s2beta_))
                * c1
                * s1p5;
            self.a[30] = (-(10.0 / 3.0 + 8.0 / 3.0 * c2beta_ + 14.0 / 3.0 * c4beta_)
                + self.eta * (10.0 + 8.0 * c2beta_ + 14.0 * c4beta_))
                * c1p2
                * s1p6;
            self.a[31] = (-(1.0 / 2.0 + 1.0 / 6.0 * c2beta_)
                + self.eta * (3.0 / 2.0 + 1.0 / 2.0 * c2beta_))
                * c1p2
                * sbeta_2
                * s1p6;
            self.a[32] = 32.0 * (self.eta - 1.0 / 3.0) * cbeta_3 * c1 * sbeta_ * s1p7;
            self.a[33] =
                (self.eta * (6.0 + 2.0 * c2beta_) - (2.0 + 2.0 / 3.0 * c2beta_)) * sbeta_2 * s1p8;
            self.a[34] = 1.0 / 32.0
                * (1.0 / 3.0 * (349.0 - 25.0 * c2beta_) * sbeta_2
                    - (25.0 + 35.0 * c2beta_) * c4 * sbeta_2)
                + self.eta
                    * ((25.0 * c2beta_ - 45.0) * sbeta_2 + (25.0 + 35.0 * c2beta_) * c4 * sbeta_2)
                    * s2p2;
            self.a[35] = 1.0 / 4.0
                * (self.eta * (25.0 + 35.0 * c2beta_) - 1.0 / 3.0 * (25.0 - 35.0 * c2beta_))
                * sbeta_2
                * s2p4;
            self.a[36] = c1p3
                * (6.0 * s2beta_ - 31.0 / 12.0 * c2 * s2beta_ + 1.0 / 12.0 * c4 * s2beta_
                    - 19.0 / 48.0 * s4beta_)
                * s1
                + 7.0 / 24.0 * c1p3 * s4beta_ * s3
                - 7.0 / 48.0 * c1p3 * s4beta_ * s5
                + self.eta
                    * (c1p3
                        * (-16.0 / 3.0 * s2beta_ + 31.0 / 4.0 * c2 * s2beta_
                            - 1.0 / 4.0 * c4 * s2beta_
                            + 19.0 / 16.0 * s4beta_)
                        * s1
                        - 7.0 / 8.0 * c1p3 * s4beta_ * s3
                        + 7.0 / 16.0 * c1p3 * s4beta_ * s5);

            // Subscripts
            self.a[37] = self.chiax_dn * cbeta_ * c1p2 - self.chiaz_dn * c1p2 * sbeta_;
            self.a[38] =
                self.chiax_dn * (cbeta_ / 2.0 - cbeta_ / 2.0 * c2) - self.chiaz_dn * sbeta_ * s1p2;
            self.a[39] = -self.chiay_dn * cbeta_ * s1p2;
            self.a[40] = -self.chiay_dn * sbeta_ * s2;
            self.a[41] = -self.chiay_dn * cbeta_ * c1p2;
            self.a[42] =
                self.delta * (self.chisx_dn * cbeta_ * c1p2 - self.chisz_dn * c1p2 * sbeta_);
            self.a[43] = self.delta
                * (self.chisx_dn * (cbeta_ / 2.0 - cbeta_ / 2.0 * c2)
                    - self.chisz_dn * sbeta_ * s1p2);
            self.a[44] = -self.delta * (self.chisy_dn * cbeta_ * s1p2);
            self.a[45] = -self.delta * (self.chisy_dn * sbeta_ * s2);
            self.a[46] = -self.delta * (self.chisy_dn * cbeta_ * c1p2);

            //Amplitude factors for H2X

            self.a[146] = (4.0 * sbeta_ + 28.0 / 3.0 * s3beta_
                - self.eta * (12.0 * sbeta_ + 28.0 * s3beta_))
                * c1p3
                * s1p5;
            self.a[147] = (self.eta
                * (4.0 * cbeta_ + 28.0 * c3beta_ - (4.0 / 3.0 * cbeta_ + 28.0 / 3.0 * c3beta_)))
                * c1p2
                * s1p6;
            self.a[148] = ((4.0 / 3.0 * sbeta_ - 4.0 * s3beta_)
                + self.eta * (-4.0 * sbeta_ + 12.0 * s3beta_))
                * c1
                * s1p7;
            self.a[149] = (8.0 * self.eta - 8.0 / 3.0) * cbeta_ * sbeta_ * s1p8;
            self.a[150] = c1
                * (-79.0 / 8.0 * sbeta_
                    + c2 * (3.0 / 4.0 * sbeta_ - 19.0 / 12.0 * s3beta_)
                    + c4 * (1.0 / 8.0 * sbeta_ + 7.0 / 24.0 * s3beta_)
                    - 3.0 / 8.0 * s3beta_)
                * s1p3
                + self.eta
                    * c1
                    * (103.0 / 24.0 * sbeta_ - c4 * (3.0 / 8.0 * sbeta_ + 7.0 / 8.0 * s3beta_)
                        + 9.0 / 8.0 * s3beta_
                        + c2 * (-9.0 / 4.0 * sbeta_ + 19.0 / 4.0 * s3beta_))
                    * s1p3;
            self.a[151] = (47.0 / 8.0 * cbeta_
                + 1.0 / 8.0 * c3beta_
                + (7.0 / 6.0 * cbeta_ + 1.0 / 6.0 * c3beta_) * c2
                - (1.0 / 24.0 * cbeta_ + 7.0 / 24.0 * c3beta_) * c4
                + self.eta
                    * (-119.0 / 24.0 * cbeta_
                        - 3.0 / 8.0 * c3beta_
                        - (7.0 / 2.0 * cbeta_ + 1.0 / 2.0 * c3beta_) * c2
                        + (1.0 / 8.0 * cbeta_ + 7.0 / 8.0 * c3beta_) * c4))
                * s1p4;
            self.a[152] = (4.0 / 3.0 * sbeta_ - (1.0 / 3.0 + c2beta_) * c2 * sbeta_
                + self.eta * (-4.0 * sbeta_ + (1.0 + 3.0 * c2beta_) * c2 * sbeta_))
                * c1
                * s1p5;
            self.a[153] = (2.0 * self.eta - 2.0 / 3.0) * cbeta_ * c1p2 * sbeta_2 * s1p6;
            self.a[154] = (15.0 / 2.0 * self.eta - 5.0 / 2.0) * cbeta_ * c2 * sbeta_2 * s2p2;
            self.a[155] = c1p3
                * (79.0 / 8.0 * sbeta_ + c2 * (3.0 / 4.0 * sbeta_ - 19.0 / 12.0 * s3beta_)
                    - c4 * (1.0 / 8.0 * sbeta_ + 7.0 / 24.0 * s3beta_)
                    + 3.0 / 8.0 * s3beta_)
                * s1
                + self.eta
                    * c1p3
                    * (-103.0 / 24.0 * sbeta_ + c4 * (3.0 / 8.0 * sbeta_ + 7.0 / 8.0 * s3beta_)
                        - 9.0 / 8.0 * s3beta_
                        + c2 * (-9.0 / 4.0 * sbeta_ + 19.0 / 4.0 * c3beta_))
                    * s1;
            self.a[156] = c1p4
                * (47.0 / 8.0 * cbeta_ + 1.0 / 8.0 * c3beta_
                    - (7.0 / 6.0 * cbeta_ + 1.0 / 6.0 * c3beta_) * c2
                    - (1.0 / 24.0 * cbeta_ + 7.0 / 24.0 * c3beta_) * c4)
                + self.eta
                    * c1p4
                    * (-119.0 / 24.0 * cbeta_ - 3.0 / 8.0 * c3beta_
                        + (7.0 / 2.0 * cbeta_ + 1.0 / 2.0 * c3beta_) * c2
                        + (1.0 / 8.0 * cbeta_ + 7.0 / 8.0 * c3beta_) * c4);
            self.a[157] = (-4.0 / 3.0 * sbeta_ - (1.0 / 3.0 + c2beta_) * c2 * sbeta_
                + self.eta * (4.0 * sbeta_ + (1.0 + 3.0 * c2beta_) * c2 * sbeta_))
                * c1p5
                * s1;
            self.a[158] = (2.0 * self.eta - 2.0 / 3.0) * cbeta_ * c1p6 * sbeta_2 * s1p2;
            self.a[159] = (self.eta * (12.0 * sbeta_ + 28.0 * s3beta_)
                - (4.0 * sbeta_ + 28.0 / 3.0 * s3beta_))
                * c1p5
                * s1p3;
            self.a[160] = (self.eta * (4.0 * cbeta_ + 28.0 * c3beta_)
                - (4.0 / 3.0 * cbeta_ + 28.0 / 3.0 * c3beta_))
                * c1p6
                * s1p2;
            self.a[161] = (8.0 / 3.0 + 8.0 * c2beta_ - self.eta * (8.0 + 24.0 * c2beta_))
                * c1p7
                * sbeta_
                * s1;
            self.a[162] = (8.0 * self.eta - 8.0 / 3.0) * cbeta_ * c1p8 * sbeta_2;

            //Subscripts

            self.a[163] = self.chiay_dn * (1.0 / 2.0 + 1.0 / 2.0 * c2);
            self.a[164] = self.chiay_dn * s1p2;
            self.a[165] = self.chiax_dn * (1.0 / 2.0 * cbeta_2 - 1.0 / 2.0 * cbeta_2 * c2)
                + self.chiaz_dn * (-1.0 / 2.0 * cbeta_ * sbeta_ + 1.0 / 2.0 * cbeta_ * c2 * sbeta_);
            self.a[166] = self.chiax_dn * cbeta_ * sbeta_ * s2 - self.chiaz_dn * sbeta_2 * s2;
            self.a[167] = self.chiax_dn * (1.0 / 2.0 * cbeta_2 + 1.0 / 2.0 * cbeta_2 * c2)
                + self.chiaz_dn * (-1.0 / 2.0 * cbeta_ * sbeta_ - 1.0 / 2.0 * cbeta_ * c2 * sbeta_);
            self.a[168] = self.delta * (self.chisy_dn * (1.0 / 2.0 + 1.0 / 2.0 * c2));
            self.a[169] = self.delta * (self.chisy_dn * s1p2);
            self.a[170] = self.delta
                * (self.chisx_dn * (1.0 / 2.0 * cbeta_2 - 1.0 / 2.0 * cbeta_2 * c2)
                    + self.chisz_dn
                        * (-1.0 / 2.0 * cbeta_ * sbeta_ + 1.0 / 2.0 * cbeta_ * c2 * sbeta_));
            self.a[171] =
                self.delta * (self.chisx_dn * cbeta_ * sbeta_ * s2 - self.chisz_dn * sbeta_2 * s2);
            self.a[172] = self.delta
                * (self.chisx_dn * (1.0 / 2.0 * cbeta_2 + 1.0 / 2.0 * cbeta_2 * c2)
                    + self.chisz_dn
                        * (-1.0 / 2.0 * cbeta_ * sbeta_ - 1.0 / 2.0 * cbeta_ * c2 * sbeta_));
        }
        if self.pn_order > 2 {
            // AmPItude factors for H3P

            self.a[47] = -(3.0 * PI + PI * c2beta_) * c1p4;
            self.a[48] = -4.0 * PI * c1p3 * s2beta_ * s1;
            self.a[49] = 4.0 * PI * c1 * s2beta_ * s1p3;
            self.a[50] = -(3.0 * PI + PI * c2beta_) * s1p4;
            self.a[51] = -3.0 * PI * sbeta_2 * s2p2;
            self.a[52] = self.delta
                * (self.eta * (625.0 / 128.0 + 625.0 / 384.0 * c2beta_)
                    - (625.0 / 256.0 + 625.0 / 768.0 * c2beta_))
                * c1p10
                * sbeta_3;
            // I simplified the powers, is that okay?
            self.a[53] = self.delta
                * (self.eta
                    * c1p2
                    * (-7449.0 / 16384.0 * sbeta_ - 331.0 / 32768.0 * s3beta_
                        + c4 * (337.0 / 12288.0 * sbeta_
                            - 47.0 / 8192.0 * s3beta_
                            - 21.0 / 8192.0 * s5beta_)
                        + c8 * (7.0 / 49152.0 * sbeta_ + 7.0 / 32768.0 * s3beta_
                            - 35.0 / 32768.0 * s5beta_)
                        + c6 * (-59.0 / 6144.0 * sbeta_ - 91.0 / 4096.0 * s3beta_
                            + 7.0 / 4096.0 * s5beta_)
                        + c2 * (1873.0 / 2048.0 * sbeta_
                            + 19.0 / 4096.0 * s3beta_
                            + 35.0 / 12288.0 * s5beta_)
                        - 155.0 / 98304.0 * s5beta_)
                    + c1p2
                        * (43723.0 / 98304.0 * sbeta_ - 9653.0 / 65536.0 * s3beta_
                            + c2 * (-10675.0 / 12288.0 * sbeta_ + 1901.0 / 8192.0 * s3beta_
                                - 35.0 / 24576.0 * s5beta_)
                            + c6 * (59.0 / 12288.0 * sbeta_ + 91.0 / 8192.0 * s3beta_
                                - 7.0 / 8192.0 * s5beta_)
                            + c8 * (-7.0 / 98304.0 * sbeta_ - 7.0 / 65536.0 * s3beta_
                                + 35.0 / 65536.0 * s5beta_)
                            + c4 * (1103.0 / 24576.0 * sbeta_ - 2833.0 / 16384.0 * s3beta_
                                + 21.0 / 16384.0 * s5beta_)
                            + 155.0 / 196608.0 * s5beta_));
            self.a[54] = self.delta
                * (c1p6
                    * (39249.0 / 8192.0 * sbeta_ + 38331.0 / 16384.0 * s3beta_
                        - c4 * (1701.0 / 8192.0 * sbeta_
                            + 3159.0 / 16384.0 * s3beta_
                            + 3645.0 / 16384.0 * s5beta_)
                        + c2 * (2403.0 / 2048.0 * sbeta_ - 6399.0 / 4096.0 * s3beta_
                            + 2187.0 / 4096.0 * s5beta_)
                        - 5751.0 / 16384.0 * s5beta_)
                    + self.eta
                        * c1p6
                        * (-4689.0 / 4096.0 * sbeta_ - 24507.0 / 8192.0 * s3beta_
                            + c2 * (-2403.0 / 1024.0 * sbeta_ + 6399.0 / 2048.0 * s3beta_
                                - 2187.0 / 2048.0 * s5beta_)
                            + c4 * (1701.0 / 4096.0 * sbeta_
                                + 3159.0 / 8192.0 * s3beta_
                                + 3645.0 / 8192.0 * s5beta_)
                            + 5751.0 / 8192.0 * s5beta_));
            self.a[55] = self.delta
                * ((11875.0 / 768.0 * cbeta_ + 3125.0 / 768.0 * c3beta_
                    - self.eta * (11875.0 / 384.0 * cbeta_ + 3125.0 / 384.0 * c3beta_))
                    * c1p9
                    * sbeta_2
                    * s1);
            self.a[56] = self.delta
                * (((-351.0 / 256.0 * cbeta_ + 243.0 / 256.0 * cbeta_ * c2beta_) * sbeta_2
                    - (567.0 / 256.0 * cbeta_ + 405.0 / 256.0 * cbeta_ * c2beta_) * c2 * sbeta_2
                    + self.eta
                        * ((351.0 / 128.0 * cbeta_ - 243.0 / 128.0 * cbeta_ * c2beta_) * sbeta_2
                            + (567.0 / 128.0 * cbeta_ + 405.0 / 128.0 * cbeta_ * c2beta_)
                                * c2
                                * sbeta_2))
                    * c1p7
                    * s1);
            self.a[57] = self.delta
                * ((self.eta * (243.0 / 128.0 + 81.0 / 128.0 * c2beta_)
                    - (243.0 / 256.0 + 81.0 / 256.0 * c2beta_))
                    * c1p8
                    * sbeta_3
                    * s1p2);
            self.a[58] = self.delta
                * ((-43723.0 / 98304.0 * sbeta_
                    + 9653.0 / 65536.0 * s3beta_
                    + c2 * (-10675.0 / 12288.0 * sbeta_ + 1901.0 / 8192.0 * s3beta_
                        - 35.0 / 24576.0 * s5beta_)
                    + c4 * (-1103.0 / 24576.0 * sbeta_ + 2833.0 / 16384.0 * s3beta_
                        - 21.0 / 16384.0 * s5beta_)
                    + c6 * (59.0 / 12288.0 * sbeta_ + 91.0 / 8192.0 * s3beta_
                        - 7.0 / 8192.0 * s5beta_)
                    + c8 * (7.0 / 98304.0 * sbeta_ + 7.0 / 65536.0 * s3beta_
                        - 35.0 / 65536.0 * s5beta_)
                    - 155.0 / 196608.0 * s5beta_)
                    * s1p2
                    + self.eta
                        * (7449.0 / 16384.0 * sbeta_
                            + 331.0 / 32768.0 * s3beta_
                            + c8 * (-7.0 / 49152.0 * sbeta_ - 7.0 / 32768.0 * s3beta_
                                + 35.0 / 32768.0 * s5beta_)
                            + c6 * (-59.0 / 6144.0 * sbeta_ - 91.0 / 4096.0 * s3beta_
                                + 7.0 / 4096.0 * s5beta_)
                            + c4 * (-337.0 / 12288.0 * sbeta_
                                + 47.0 / 8192.0 * s3beta_
                                + 21.0 / 8192.0 * s5beta_)
                            + c2 * (1873.0 / 2048.0 * sbeta_
                                + 19.0 / 4096.0 * s3beta_
                                + 35.0 / 12288.0 * s5beta_)
                            + 155.0 / 98304.0 * s5beta_)
                        * s1p2);
            self.a[59] = self.delta
                * (c1p4
                    * (1675.0 / 4096.0 * sbeta_ + 825.0 / 8192.0 * s3beta_
                        - c4 * (7.0 / 4096.0 * sbeta_
                            + 13.0 / 8192.0 * s3beta_
                            + 15.0 / 8192.0 * s5beta_)
                        + c2 * (27.0 / 1024.0 * sbeta_ - 151.0 / 2048.0 * s3beta_
                            + 3.0 / 2048.0 * s5beta_)
                        - 13.0 / 8192.0 * s5beta_)
                    * s1p2
                    + self.eta
                        * c1p4
                        * (245.0 / 2048.0 * sbeta_ - 57.0 / 4096.0 * s3beta_
                            + c2 * (-27.0 / 512.0 * sbeta_ + 151.0 / 1024.0 * s3beta_
                                - 3.0 / 1024.0 * s5beta_)
                            + c4 * (7.0 / 2048.0 * sbeta_
                                + 13.0 / 4096.0 * s3beta_
                                + 15.0 / 4096.0 * s5beta_)
                            + 13.0 / 4096.0 * s5beta_)
                        * s1p2);
            self.a[60] = self.delta
                * ((self.eta
                    * (4375.0 / 512.0 * sbeta_
                        + 8125.0 / 1024.0 * s3beta_
                        + 9375.0 / 1024.0 * s5beta_)
                    - (4375.0 / 1024.0 * sbeta_
                        + 8125.0 / 2048.0 * s3beta_
                        + 9375.0 / 2048.0 * s5beta_))
                    * c1p8
                    * s1p2);
            self.a[61] = self.delta
                * (c1p4
                    * (20475.0 / 4096.0 * sbeta_ - 149391.0 / 8192.0 * s3beta_
                        + c2 * (2187.0 / 1024.0 * sbeta_ + 10017.0 / 2048.0 * s3beta_
                            - 1701.0 / 2048.0 * s5beta_)
                        + 7371.0 / 8192.0 * s5beta_
                        + c4 * (-567.0 / 4096.0 * sbeta_ - 1701.0 / 8192.0 * s3beta_
                            + 8505.0 / 8192.0 * s5beta_))
                    * s1p2
                    + self.eta
                        * c1p4
                        * (-3195.0 / 2048.0 * sbeta_
                            + 45711.0 / 4096.0 * s3beta_
                            + c4 * (567.0 / 2048.0 * sbeta_ + 1701.0 / 4096.0 * s3beta_
                                - 8505.0 / 4096.0 * s5beta_)
                            - 7371.0 / 4096.0 * s5beta_
                            + c2 * (-2187.0 / 512.0 * sbeta_ - 10017.0 / 1024.0 * s3beta_
                                + 1701.0 / 1024.0 * s5beta_))
                        * s1p2);
            self.a[62] = self.delta
                * ((4375.0 / 384.0 * cbeta_ + 625.0 / 256.0 * c3beta_ + 3125.0 / 256.0 * c5beta_
                    - self.eta
                        * (4375.0 / 192.0 * cbeta_
                            + 625.0 / 128.0 * c3beta_
                            + 3125.0 / 128.0 * c5beta_))
                    * c1p7
                    * s1p3);
            self.a[63] = self.delta
                * (c1p5
                    * ((-37.0 / 384.0 * cbeta_ + 1.0 / 384.0 * cbeta_ * c2beta_) * sbeta_2
                        - (7.0 / 384.0 * cbeta_ + 5.0 / 384.0 * cbeta_ * c2beta_) * c2 * sbeta_2)
                    * s1p3
                    + self.eta
                        * c1p5
                        * ((37.0 / 192.0 * cbeta_ - 1.0 / 192.0 * cbeta_ * c2beta_) * sbeta_2
                            + (7.0 / 192.0 * cbeta_ + 5.0 / 192.0 * cbeta_ * c2beta_)
                                * c2
                                * sbeta_2)
                        * s1p3);
            self.a[64] = self.delta
                * ((self.eta * (1.0 / 64.0 + 1.0 / 192.0 * c2beta_)
                    - (1.0 / 128.0 + 1.0 / 384.0 * c2beta_))
                    * c1p6
                    * sbeta_3
                    * s1p4);
            self.a[65] = self.delta
                * (self.eta
                    * c1p2
                    * (-245.0 / 2048.0 * sbeta_ + 57.0 / 4096.0 * s3beta_
                        - c4 * (7.0 / 2048.0 * sbeta_
                            + 13.0 / 4096.0 * s3beta_
                            + 15.0 / 4096.0 * s5beta_)
                        + c2 * (-27.0 / 512.0 * sbeta_ + 151.0 / 1024.0 * s3beta_
                            - 3.0 / 1024.0 * s5beta_)
                        - 13.0 / 4096.0 * s5beta_)
                    * s1p4
                    + c1p2
                        * (-1675.0 / 4096.0 * sbeta_ - 825.0 / 8192.0 * s3beta_
                            + c2 * (27.0 / 1024.0 * sbeta_ - 151.0 / 2048.0 * s3beta_
                                + 3.0 / 2048.0 * s5beta_)
                            + c4 / 4096.0
                                * (7.0 * sbeta_ + 13.0 * s3beta_ + 15.0 * s5beta_)
                                * s1p4));
            self.a[66] = self.delta
                * (4375.0
                    * self.eta
                    * c1p6
                    * (1.0 / 768.0 * sbeta_ + 1.0 / 512.0 * s3beta_ - 5.0 / 512.0 * s5beta_)
                    * s1p4
                    + 4375.0
                        * c1p6
                        * (-1.0 / 1536.0 * sbeta_ - 1.0 / 1024.0 * s3beta_
                            + 5.0 / 1024.0 * s5beta_)
                        * s1p4);
            self.a[67] = self.delta
                * (c1p2
                    * (-20475.0 / 4096.0 * sbeta_
                        + 149391.0 / 8192.0 * s3beta_
                        + c4 / 4096.0
                            * (567.0 * sbeta_ + 1701.0 / 2.0 * s3beta_ - 8505.0 / 2.0 * s5beta_)
                        + c2 / 2048.0 * (4374.0 * sbeta_ + 10017.0 * s3beta_ - 1701.0 * s5beta_)
                        - 7371.0 / 8192.0 * s5beta_)
                    * s1p4
                    + self.eta
                        * c1p2
                        * (3195.0 / 2048.0 * sbeta_ - 45711.0 / 4096.0 * s3beta_
                            + 7371.0 / 4096.0 * s5beta_
                            + c2 * (-2187.0 / 512.0 * sbeta_ - 10017.0 / 1024.0 * s3beta_
                                + 1701.0 / 1024.0 * s5beta_)
                            + c4 * (-567.0 / 2048.0 * sbeta_ - 1701.0 / 4096.0 * s3beta_
                                + 8505.0 / 4096.0 * s5beta_))
                        * s1p4);
            self.a[68] = self.delta
                * (self.eta
                    * c1p3
                    * ((37.0 / 192.0 * cbeta_ - 1.0 / 192.0 * cbeta_ * c2beta_) * sbeta_2
                        - (7.0 / 192.0 * cbeta_ + 5.0 / 192.0 * cbeta_ * c2beta_) * c2 * sbeta_2)
                    * s1p5
                    + c1p3
                        * ((-37.0 / 384.0 * cbeta_ + 1.0 / 384.0 * cbeta_ * c2beta_) * sbeta_2
                            + (7.0 / 384.0 * cbeta_ + 5.0 / 384.0 * cbeta_ * c2beta_)
                                * c2
                                * sbeta_2)
                        * s1p5);
            self.a[69] = self.delta
                * (1.0 / 128.0 + 1.0 / 384.0 * c2beta_
                    - self.eta * (1.0 / 64.0 + 1.0 / 192.0 * c2beta_))
                * c1p4
                * sbeta_3
                * s1p6;
            self.a[70] = self.delta
                * (self.eta
                    * ((14067.0 / 4096.0 + 4689.0 / 1024.0 * c2beta_ - 5751.0 / 4096.0 * c4beta_)
                        * sbeta_
                        + (-297.0 / 1024.0 + 1053.0 / 256.0 * c2beta_
                            - 2187.0 / 1024.0 * c4beta_)
                            * c2
                            * sbeta_
                        - (5103.0 / 4096.0
                            + 1701.0 / 1024.0 * c2beta_
                            + 3645.0 / 4096.0 * c4beta_)
                            * c4
                            * sbeta_)
                    * s1p6
                    + ((-55539.0 / 8192.0 - 8145.0 / 2048.0 * c2beta_
                        + 5751.0 / 8192.0 * c4beta_)
                        * sbeta_
                        + (297.0 / 2048.0 - 1053.0 / 512.0 * c2beta_ + 2187.0 / 2048.0 * c4beta_)
                            * c2
                            * sbeta_
                        + (5103.0 / 8192.0
                            + 1701.0 / 2048.0 * c2beta_
                            + 3645.0 / 8192.0 * c4beta_)
                            * c4
                            * sbeta_)
                        * s1p6);
            // The multiplication location
            self.a[71] = self.delta
                * (c1p4
                    * (4375.0 / 1536.0 * sbeta_ + 4375.0 / 1024.0 * s3beta_
                        - 21875.0 / 1024.0 * s5beta_)
                    * s1p6
                    + self.eta
                        * c1p4
                        * (-4375.0 / 768.0 * sbeta_ - 4375.0 / 512.0 * s3beta_
                            + 21875.0 / 512.0 * s5beta_)
                        * s1p6);
            self.a[72] = self.delta
                * ((4375.0 / 384.0 * cbeta_ + 625.0 / 256.0 * c3beta_ + 3125.0 / 256.0 * c5beta_
                    - self.eta
                        * (4375.0 / 192.0 * cbeta_
                            + 625.0 / 128.0 * c3beta_
                            + 3125.0 / 128.0 * c5beta_))
                    * c1p3
                    * s1p7);
            self.a[73] = self.delta
                * (self.eta
                    * c1
                    * ((351.0 / 128.0 * cbeta_ - 243.0 / 128.0 * cbeta_ * c2beta_) * sbeta_2
                        - (567.0 / 128.0 * cbeta_ + 405.0 / 128.0 * cbeta_ * c2beta_)
                            * c2
                            * sbeta_2)
                    * s1p7
                    + c1 * ((-351.0 / 256.0 * cbeta_ + 243.0 / 256.0 * cbeta_ * c2beta_)
                        * sbeta_2
                        + (567.0 / 256.0 * cbeta_ + 405.0 / 256.0 * cbeta_ * c2beta_)
                            * c2
                            * sbeta_2)
                        * s1p7);
            self.a[74] = self.delta
                * ((243.0 / 256.0
                    - 81.0 / 256.0 * c2beta_
                    - self.eta * (243.0 / 128.0 + 81.0 / 128.0 * c2beta_))
                    * c1p2
                    * sbeta_3
                    * s1p8);
            self.a[75] = self.delta
                * ((4375.0 / 1024.0 * sbeta_
                    + 8125.0 / 2048.0 * s3beta_
                    + 9375.0 / 2048.0 * s5beta_
                    - self.eta
                        * (4375.0 / 512.0 * sbeta_
                            + 8125.0 / 1024.0 * s3beta_
                            + 9375.0 / 1024.0 * s5beta_))
                    * c1p2
                    * s1p8);
            // Here is when Dr. Moore and i realized that using the '[ ]' is totally wrong, instead i should just self.etalace them with normal parenthesis.
            self.a[76] = self.delta
                * ((11875.0 / 768.0 * cbeta_ + 3125.0 / 768.0 * c3beta_
                    - self.eta * (11875.0 / 384.0 * cbeta_ + 3125.0 / 384.0 * c3beta_))
                    * c1
                    * sbeta_2
                    * s1p9);
            self.a[77] = self.delta
                * ((625.0 / 256.0 + 625.0 / 768.0 * c2beta_
                    - self.eta * (625.0 / 128.0 + 625.0 / 384.0 * c2beta_))
                    * sbeta_3
                    * s1p10);
            self.a[78] = self.delta
                * (self.eta
                    * ((10197.0 / 20488.0 * cbeta_ - 3969.0 / 2048.0 * cbeta_ * c2beta_)
                        * sbeta_2
                        - (1701.0 / 2048.0 * cbeta_ + 5103.0 / 2048.0 * cbeta_ * c2beta_)
                            * c4
                            * sbeta_2)
                    * s2p3
                    + ((-44757.0 / 4096.0 * cbeta_ + 3969.0 / 4096.0 * cbeta_ * c2beta_)
                        * sbeta_2
                        + (1701.0 / 4096.0 * cbeta_ + 5103.0 / 4096.0 * cbeta_ * c2beta_)
                            * c4
                            * sbeta_2)
                        * s2p3);
            self.a[79] = self.delta
                * ((21875.0 / 4096.0 * cbeta_ + 13125.0 / 4096.0 * c3beta_
                    - self.eta * (21875.0 / 2048.0 * cbeta_ + 13125.0 / 2048.0 * c3beta_))
                    * sbeta_2
                    * s2p5);
            self.a[80] = self.delta
                * ((-37071.0 / 16384.0 * cbeta_ * c2beta_
                    + cbeta_ * (-7641.0 / 8192.0 + 567.0 / 32768.0 * c4beta_)
                    - (10917.0 / 8192.0 * cbeta_ + 2835.0 / 1024.0 * cbeta_ * c2beta_) * c2
                    + (-10089.0 / 16284.0 * cbeta_ + 135.0 / 8192.0 * cbeta_ * c2beta_) * c4
                    + 513.0 / 8192.0 * cbeta_ * c6
                    + 567.0 / 32768.0 * cbeta_ * c8)
                    * s2
                    - 81.0 / 8192.0 * cbeta_ * c4beta_ * s4
                    + 1053.0 / 65536.0 * cbeta_ * c4beta_ * s6
                    + (2565.0 / 32768.0 * c3beta_ + 729.0 / 32768.0 * c5beta_) * s8
                    + (243.0 / 131072.0 * c3beta_ + 1215.0 / 131072.0 * c5beta_) * s10
                    + self.eta
                        * ((5967.0 / 8192.0 * cbeta_ * c2beta_
                            + cbeta_ * (2457.0 / 4096.0 - 567.0 / 16384.0 * c4beta_)
                            + (4005.0 / 4096.0 * cbeta_ + 243.0 / 512.0 * cbeta_ * c2beta_) * c2
                            + (6633.0 / 8192.0 * cbeta_ - 5319.0 / 4096.0 * cbeta_ * c2beta_)
                                * c4
                            - 513.0 / 4096.0 * cbeta_ * c6
                            - 567.0 / 16384.0 * cbeta_ * c8)
                            * s2
                            + 81.0 / 4096.0 * cbeta_ * c4beta_ * s4
                            - 1053.0 / 32768.0 * cbeta_ * c4beta_ * s6
                            - (2565.0 / 16384.0 * c3beta_ + 729.0 / 16384.0 * c5beta_) * s8
                            - (243.0 / 65536.0 * c3beta_ + 1215.0 / 65536.0 * c5beta_) * s10));
            self.a[81] = self.delta
                * ((-18603.0 / 8192.0 * cbeta_ * c2beta_
                    + cbeta_ * (-20475.0 / 32768.0 + 567.0 / 32768.0 * c4beta_))
                    * s2
                    + (2835.0 / 2048.0 * cbeta_ * c2beta_
                        + cbeta_ * (5715.0 / 8192.0 + 81.0 / 8192.0 * c4beta_))
                        * s4
                    + (135.0 / 16384.0 * cbeta_ * c2beta_
                        + cbeta_ * (-20745.0 / 65536.0 + 1053.0 / 65536.0 * c4beta_))
                        * s6
                    - (513.0 / 16384.0 * cbeta_
                        + 2565.0 / 32768.0 * c3beta_
                        + 729.0 / 32768.0 * c5beta_)
                        * s8
                    + (567.0 / 65536.0 * cbeta_
                        + 243.0 / 131072.0 * c3beta_
                        + 1215.0 / 131072.0 * c5beta_)
                        * s10
                    + self.eta
                        * ((5643.0 / 4096.0 * cbeta_ * c2beta_
                            + cbeta_ * (3195.0 / 16384.0 - 567.0 / 16384.0 * c4beta_))
                            * s6
                            + (513.0 / 8192.0 * cbeta_
                                + 2565.0 / 16384.0 * c3beta_
                                + 729.0 / 16384.0 * c5beta_)
                                * s8
                            - (567.0 / 32768.0 * cbeta_
                                + 243.0 / 65536.0 * c3beta_
                                + 1215.0 / 65536.0 * c5beta_)
                                * s10));
            self.a[82] = self.delta
                * ((319.0 / 24576.0 * cbeta_ * c2beta_
                    + cbeta_ * (871.0 / 4096.0 + 1.0 / 49152.0 * c4beta_)
                    + (933.0 / 4096.0 * cbeta_ + 133.0 / 1536.0 * cbeta_ * c2beta_) * c2
                    + (625.0 / 24576.0 * cbeta_ + 211.0 / 4096.0 * cbeta_ * c2beta_) * c4
                    - 11.0 / 12288.0 * cbeta_ * c6
                    - 7.0 / 49152.0 * cbeta_ * c8)
                    * s2
                    - 1.0 / 12288.0 * cbeta_ * c4beta_ * s4
                    + 1.0 / 32768.0 * cbeta_ * c4beta_ * s6
                    - (45.0 / 16384.0 * c3beta_ + 1.0 / 16384.0 * c5beta_) * s8
                    - (1.0 / 65536.0 * c3beta_ + 5.0 / 65536.0 * c5beta_) * s10
                    + self.eta
                        * ((257.0 / 12288.0 * cbeta_ * c2beta_
                            - cbeta_ * (1493.0 / 6144.0 + 1.0 / 24576.0 * c4beta_)
                            + (-1391.0 / 6144.0 + 11.0 / 768.0 * cbeta_ * c2beta_) * c2
                            + (-49.0 / 12288.0 * cbeta_ + 77.0 / 2048.0 * cbeta_ * c2beta_) * c4
                            + 11.0 / 6144.0 * cbeta_ * c6
                            + 7.0 / 24576.0 * cbeta_ * c8)
                            * s2
                            + 1.0 / 6144.0 * cbeta_ * c4beta_ * s4
                            - 1.0 / 16384.0 * cbeta_ * c4beta_ * s6
                            + (45.0 / 8192.0 * c3beta_ + 1.0 / 8192.0 * c5beta_) * s8
                            + (1.0 / 32768.0 * c3beta_ + 5.0 / 32768.0 * c5beta_) * s10));
            self.a[83] = self.delta
                * ((-157.0 / 12288.0 * cbeta_ * c2beta_
                    + cbeta_ * (9827.0 / 49152.0 + 1.0 / 49152.0 * c4beta_))
                    * s2
                    + (-133.0 / 3072.0 * cbeta_ * c2beta_
                        + cbeta_ * (-1405.0 / 12288.0 + 1.0 / 12288.0 * c4beta_))
                        * s4
                    + (211.0 / 8192.0 * cbeta_ * c2beta_
                        + cbeta_ * (419.0 / 32768.0 + 1.0 / 32768.0 * c4beta_))
                        * s6
                    + (11.0 / 24576.0 * cbeta_
                        + 45.0 / 16384.0 * c3beta_
                        + 1.0 / 16384.0 * c5beta_)
                        * s8
                    + (-7.0 / 98304.0 * cbeta_
                        - 1.0 / 65536.0 * c3beta_
                        - 5.0 / 65536.0 * c5beta_)
                        * s10
                    + self.eta
                        * ((13.0 / 6144.0 * cbeta_ * c2beta_
                            + cbeta_ * (-5923.0 / 24576.0 - 1.0 / 24576.0 * c4beta_))
                            * s2
                            + (-11.0 / 1536.0 * cbeta_ * c2beta_
                                + cbeta_ * (701.0 / 6144.0 - 1.0 / 6144.0 * c4beta_))
                                * s4
                            + (77.0 / 4096.0 * cbeta_ * c2beta_
                                + cbeta_ * (-35.0 / 16384.0 - 1.0 / 16384.0 * c4beta_))
                                * s6
                            + (-11.0 / 12288.0 * cbeta_
                                - 45.0 / 8192.0 * c3beta_
                                - 1.0 / 8192.0 * c5beta_)
                                * s8
                            + (7.0 / 49152.0 * cbeta_
                                + 1.0 / 32768.0 * c3beta_
                                + 5.0 / 32768.0 * c5beta_)
                                * s10));
            self.a[84] = self.delta
                * ((-341.0 / 8192.0 * cbeta_ + 1.0 / 8192.0 * cbeta_ * c2beta_) * sbeta_2 * s2
                    + (-3411.0 / 16384.0 * cbeta_ + 7.0 / 16384.0 * cbeta_ * c2beta_)
                        * sbeta_2
                        * s6
                    + (35.0 / 32768.0 * cbeta_ + 21.0 / 32768.0 * c3beta_) * sbeta_2 * s10
                    + self.eta
                        * ((-43.0 / 4096.0 * cbeta_ - 1.0 / 4096.0 * cbeta_ * c2beta_)
                            * sbeta_2
                            * s2
                            + (-429.0 / 8192.0 * cbeta_ + 7.0 / 8192.0 * cbeta_ * c2beta_)
                                * sbeta_2
                                * s6
                            + (-35.0 / 16384.0 * cbeta_ - 21.0 / 16384.0 * c3beta_)
                                * sbeta_2
                                * s10));

            //HP 3.0/2.0,SO

            self.a[85] =
                self.chisx_dn * (2.0 * cbeta_ * c2p3 * sbeta_ - self.eta * cbeta_ * c2p3 * sbeta_);
            self.a[86] = self.chisz_dn
                * (self.eta
                    * c1p4
                    * (-5.0 / 2.0 - 7.0 / 2.0 * c2beta_ + (1.0 / 2.0 + 1.0 / 6.0 * c2beta_) * c2)
                    + c1p4 * (-3.0 - c2beta_ + (5.0 + 5.0 / 3.0 * c2beta_) * c4))
                + self.chisx_dn
                    * (c1p4 * (7.0 / 3.0 * s2beta_ - 10.0 / 3.0 * c2 * s2beta_)
                        - self.eta * c1p4 * (19.0 / 6.0 * s2beta_ + 1.0 / 3.0 * c2 * s2beta_));
            self.a[87] = self.chisx_dn
                * (self.eta * (1.0 / 2.0 + 1.0 / 6.0 * c2beta_) * c1p5 * s1
                    + (5.0 + 5.0 / 3.0 * c2beta_) * c1p5 * s1);
            self.a[88] = self.chisx_dn
                * (self.eta * (1.0 / 2.0 + 1.0 / 6.0 * c2beta_) * c1 * s1p5
                    + (5.0 + 5.0 / 3.0 * c2beta_) * c1 * s1p5);
            self.a[89] = self.chisx_dn
                * (self.eta
                    * c1p3
                    * (-17.0 / 4.0
                        + 79.0 / 12.0 * c2beta_
                        + (-1.0 / 4.0 + 7.0 / 12.0 * c2beta_) * c2)
                    * s1
                    + c1p3
                        * (3.0 / 2.0 - 13.0 / 6.0 * c2beta_
                            + (-5.0 / 2.0 + 35.0 / 6.0 * c2beta_) * c2)
                        * s1)
                + self.chisz_dn
                    * (self.eta * c1p3 * (-7.0 * s2beta_ + 2.0 / 3.0 * c2 * s2beta_) * s1
                        + c1p3 * (-2.0 * s2beta_ + 20.0 / 3.0 * c2 * s2beta_) * s1);
            self.a[90] = self.chisx_dn
                * (c1
                    * (3.0 / 2.0 - 13.0 / 6.0 * c2beta_ + (5.0 / 2.0 - 35.0 / 6.0 * c2beta_) * c2)
                    * s1p3
                    + self.eta
                        * c1
                        * (-17.0 / 4.0
                            + 79.0 / 12.0 * c2beta_
                            + (1.0 / 4.0 - 7.0 / 12.0 * c2beta_) * c2)
                        * s1p3)
                + self.chisz_dn
                    * (-c1 * (2.0 * s2beta_ + 20.0 / 3.0 * c2 * s2beta_) * s1p3
                        - self.eta * c1 * (7.0 * s2beta_ + 2.0 / 3.0 * c2 * s2beta_) * s1p3);
            self.a[91] = self.chisz_dn
                * (self.eta
                    * (5.0 / 2.0 + 7.0 / 2.0 * c2beta_ + (1.0 / 2.0 + 1.0 / 6.0 * c2beta_) * c2)
                    * s1p4
                    + (3.0 + c2beta_ + (5.0 + 5.0 / 3.0 * c2beta_) * c2) * s1p4)
                + self.chisx_dn
                    * (-(7.0 / 3.0 * s2beta_ + 10.0 / 3.0 * c2 * s2beta_) * s1p4
                        + self.eta * (19.0 / 6.0 * s2beta_ - 1.0 / 3.0 * c2 * s2beta_) * s1p4);
            self.a[92] = self.chisz_dn * (-3.0 + 3.0 / 2.0 * self.eta) * c2 * sbeta_2 * s2p2;
            self.a[93] = self.chisx_dn
                * (3.0 / 4.0 + 1.0 / 4.0 * c2beta_ - self.eta * (3.0 / 8.0 + 1.0 / 8.0 * c2beta_))
                * s2p3;
            self.a[94] =
                self.chisx_dn * (10.0 / 3.0 + 1.0 / 3.0 * self.eta) * cbeta_ * c2 * sbeta_ * s2p2
                    + self.chisz_dn * (5.0 + 1.0 / 2.0 * self.lota_dn) * c2 * sbeta_2 * s2p2;
            self.a[95] = self.chisz_dn
                * (3.0 / 2.0 + 1.0 / 2.0 * c2beta_ - self.eta * (3.0 / 4.0 + 1.0 / 4.0 * c2beta_))
                * c2
                * s2p2
                + self.chisx_dn * (1.0 / 2.0 * self.lota_dn - 1.0) * c2 * s2beta_ * s2p2;
            self.a[96] = self.chisx_dn
                * (-11.0 / 16.0 * c2beta_ * s2 - 3.0 / 4.0 * s2p3 - 7.0 / 16.0 * c2beta_ * s6
                    + self.eta
                        * (11.0 / 32.0 * c2beta_ * s2
                            + 3.0 / 8.0 * s2p3
                            + 7.0 / 32.0 * c2beta_ * s6))
                + self.chisz_dn
                    * (1.0 / 2.0 * s2beta_ * s2 - 1.0 / 2.0 * s2beta_ * s6
                        + self.eta * (-1.0 / 4.0 * s2beta_ * s2 + 1.0 / 4.0 * s2beta_ * s6));
            self.a[97] = self.chisy_dn
                * ((15.0 / 8.0 - 3.0 / 8.0 * c2beta_ + (9.0 / 8.0 - 5.0 / 8.0 * c2beta_) * c4)
                    * s2
                    + self.eta
                        * (-15.0 / 16.0
                            + 3.0 / 16.0 * c2beta_
                            + (-9.0 / 16.0 + 5.0 / 16.0 * c2beta_) * c4)
                        * s2);
            self.a[98] = self.chisy_dn * (self.eta - 2.0) * cbeta_ * c2 * sbeta_ * s2p2;
            self.a[99] = self.chisy_dn
                * (3.0 / 4.0 + 1.0 / 4.0 * c2beta_ - self.eta * (3.0 / 8.0 + 1.0 / 8.0 * c2beta_))
                * s2p3;
            self.a[100] = self.chisy_dn
                * (c1
                    * (5.0 / 2.0 - 11.0 / 6.0 * c2beta_
                        + (15.0 / 2.0 - 25.0 / 6.0 * c2beta_) * c2)
                    * s1p3
                    + self.eta
                        * c1
                        * (1.0 / 4.0 - 31.0 / 12.0 * c2beta_
                            + (3.0 / 4.0 - 5.0 / 12.0 * c2beta_) * c2)
                        * s1p3);
            self.a[101] = self.chisy_dn
                * (-(7.0 / 3.0 * s2beta_ + 10.0 / 3.0 * c2 * s2beta_) * s1p4
                    - self.eta * (5.0 / 6.0 * s2beta_ + 1.0 / 3.0 * c2 * s2beta_) * s1p4);
            self.a[102] = self.chisy_dn
                * (5.0 + 5.0 / 3.0 * c2beta_ + self.eta * (1.0 / 2.0 + 1.0 / 6.0 * c2beta_))
                * c1
                * s1p5;
            // First time seeing a negative sign at the beginning.
            self.a[103] =
                -self.chisy_dn * (1.0 / 3.0 + 11.0 / 6.0 * self.eta) * cbeta_ * sbeta_ * s2p2;
            self.a[104] = self.chisy_dn
                * (self.eta
                    * c1p3
                    * (1.0 / 4.0 - 31.0 / 12.0 * c2beta_
                        + (-3.0 / 4.0 + 5.0 / 12.0 * c2beta_) * c2)
                    * s1
                    + c1p3
                        * (5.0 / 2.0 - 11.0 / 6.0 * c2beta_
                            + (-15.0 / 2.0 + 25.0 / 6.0 * c2beta_) * c2)
                        * s1);
            self.a[105] = self.chisy_dn
                * (c1p4 * (7.0 / 3.0 * s2beta_ - 10.0 / 3.0 * c2 * s2beta_)
                    + self.eta * c1p4 * (5.0 / 6.0 * s2beta_ - 1.0 / 3.0 * c2 * s2beta_));
            self.a[106] = self.chisy_dn
                * (self.eta * (1.0 / 2.0 + 1.0 / 6.0 * c2beta_) + 5.0 + 5.0 / 3.0 * c2beta_)
                * c1p5
                * s1;
            self.a[107] = 2.0 * self.delta * self.chiax_dn * cbeta_ * c2p3 * sbeta_;
            self.a[108] = self.delta
                * (self.chiaz_dn * c1p4 * (-3.0 - c2beta_ + (5.0 + 5.0 / 3.0 * c2beta_) * c2)
                    + self.chiax_dn * c1p4 * (7.0 / 3.0 * s2beta_ - 10.0 / 3.0 * c2 * s2beta_));
            self.a[109] = self.delta * self.chiax_dn * (5.0 + 5.0 / 3.0 * c2beta_) * c1p5 * s1;
            self.a[110] = self.delta * self.chiax_dn * (5.0 + 5.0 / 3.0 * c2beta_) * c1 * s1p5;
            self.a[111] = self.delta
                * (self.chiax_dn
                    * (3.0 / 2.0 - 13.0 / 6.0 * c2beta_
                        + (-5.0 / 2.0 + 35.0 / 6.0 * c2beta_) * c2)
                    + self.chiaz_dn * (-2.0 * s2beta_ + 20.0 / 3.0 * c2 * s2beta_))
                * c1p3
                * s1;
            self.a[112] = self.delta
                * (self.chiax_dn
                    * (3.0 / 2.0 - 13.0 / 6.0 * c2beta_ + (5.0 / 2.0 - 35.0 / 6.0 * c2beta_) * c2)
                    + self.chiaz_dn * (-2.0 * s2beta_ - 20.0 / 3.0 * c2 * s2beta_))
                * c1
                * s1p3;
            self.a[113] = self.delta
                * (self.chiaz_dn * (3.0 + c2beta_ + (5.0 + 5.0 / 3.0 * c2beta_) * c2) * s1p4
                    - self.chiax_dn * (7.0 / 3.0 * s2beta_ + 10.0 / 3.0 * c2 * s2beta_) * s1p4);
            self.a[114] = -3.0 * self.delta * self.chiaz_dn * c2 * sbeta_2 * s2p2;
            self.a[115] = self.delta * self.chiax_dn * (3.0 / 4.0 + 1.0 / 4.0 * c2beta_) * s2p3;
            self.a[116] = self.delta
                * (10.0 / 3.0 * self.chiax_dn * cbeta_ * c2 * sbeta_ * s2p2
                    + 5.0 * self.chiaz_dn * c2 * sbeta_2 * s2p2);
            self.a[117] =
                self.delta * self.chiaz_dn * (3.0 / 2.0 + 1.0 / 2.0 * c2beta_) * c2 * s2p2
                    - self.chiax_dn * c2 * s2beta_ * s2p2;
            self.a[118] = self.delta
                * (self.chiax_dn
                    * (-11.0 / 16.0 * c2beta_ * s2 - 3.0 / 4.0 * s2p3 - 7.0 / 16.0 * c2beta_ * s6)
                    + self.chiaz_dn * (1.0 / 2.0 * s2beta_ * s2 - 1.0 / 2.0 * s2beta_ * s6));
            self.a[119] = self.delta
                * (self.chiay_dn
                    * (15.0 / 8.0 - 3.0 / 8.0 * c2beta_ + (9.0 / 8.0 - 5.0 / 8.0 * c2beta_) * c4)
                    * s2);
            self.a[120] = -2.0 * self.delta * self.chiay_dn * cbeta_ * c2 * sbeta_ * s2p2;
            self.a[121] = self.delta * self.chiay_dn * (3.0 / 4.0 + 1.0 / 4.0 * c2beta_) * s2p3;
            self.a[122] = self.delta
                * self.chiay_dn
                * c1
                * (5.0 / 2.0 - 11.0 / 6.0 * c2beta_ + (15.0 / 2.0 - 25.0 / 6.0 * c2beta_) * c2)
                * s1p3;
            self.a[123] = self.delta
                * self.chiay_dn
                * (-7.0 / 3.0 * s2beta_ - 10.0 / 3.0 * c2 * s2beta_)
                * s1p4;
            self.a[124] = self.delta * self.chiay_dn * (5.0 + 5.0 / 3.0 * c2beta_) * c1 * s1p5;
            self.a[125] = -1.0 / 3.0 * self.delta * self.chiay_dn * cbeta_ * sbeta_ * s2p2;
            self.a[126] = self.delta
                * self.chiay_dn
                * c1p3
                * (5.0 / 2.0 - 11.0 / 6.0 * c2beta_ + (-15.0 / 2.0 + 25.0 / 6.0 * c2beta_) * c2)
                * s1;
            self.a[127] = self.delta
                * self.chiay_dn
                * c1p4
                * (7.0 / 3.0 * s2beta_ - 10.0 / 3.0 * c2 * s2beta_);
            self.a[128] = self.delta * self.chiay_dn * (5.0 + 5.0 / 3.0 * c2beta_) * c1p5 * s1;

            //Amplitude factors for H3X

            self.a[173] = 8.0 * PI * c1 * sbeta_ * s1p3;
            self.a[174] = -4.0 * PI * cbeta_ * s1p4;
            self.a[175] = -8.0 * PI * c1p3 * sbeta_ * s1;
            self.a[176] = -4.0 * PI * cbeta_ * c1p4;
            self.a[177] = self.delta
                * (c1p4 * (-4375.0 / 384.0 * s2beta_ - 4375.0 / 256.0 * s4beta_) * s1p6
                    + self.eta
                        * c1p4
                        * (4375.0 / 192.0 * s2beta_ + 4375.0 / 128.0 * s4beta_)
                        * s1p6);
            self.a[178] = self.delta
                * (625.0 / 96.0 * c2beta_ + 625.0 / 32.0 * c4beta_
                    - self.eta * (625.0 / 48.0 * c2beta_ + 625.0 / 16.0 * c4beta_))
                * c1p3
                * s1p7;
            self.a[179] = self.delta
                * (-625.0 / 256.0 * s2beta_
                    + 5625.0 / 512.0 * s4beta_
                    + self.eta * (625.0 / 128.0 * s2beta_ - 5625.0 / 256.0 * s4beta_))
                * c1p2
                * s1p8;
            self.a[180] = self.delta
                * (625.0 / 96.0 + 625.0 / 48.0 * c2beta_
                    - self.eta * (625.0 / 48.0 + 625.0 / 24.0 * c2beta_))
                * c1
                * sbeta_2
                * s1p9;
            self.a[181] =
                self.delta * (625.0 / 192.0 - 625.0 / 96.0 * self.eta) * cbeta_ * sbeta_3 * s1p10;
            self.a[182] = self.delta
                * (self.eta
                    * c1p2
                    * (-4923.0 / 512.0 * s2beta_
                        + c2 * (459.0 / 128.0 * s2beta_ - 2079.0 / 256.0 * s4beta_)
                        - 945.0 / 1024.0 * s4beta_
                        + c4 * (567.0 / 512.0 * s2beta_ + 1701.0 / 1024.0 * s4beta_))
                    * s1p4
                    + c1p2
                        * (22203.0 / 1024.0 * s2beta_
                            - c4 * (567.0 / 1024.0 * s2beta_ + 1701.0 / 2048.0 * s4beta_)
                            + 945.0 / 2048.0 * s4beta_
                            + c2 * (-459.0 / 256.0 * s2beta_ + 2079.0 / 512.0 * s4beta_))
                        * s1p4);
            self.a[183] = self.delta
                * (self.eta
                    * c1
                    * (27.0 / 16.0
                        + 1233.0 / 128.0 * c2beta_
                        + 27.0 / 128.0 * c4beta_
                        + (27.0 / 8.0 + 27.0 / 16.0 * c2beta_ + 27.0 / 16.0 * c4beta_) * c2
                        - (81.0 / 128.0 * c2beta_ + 243.0 / 128.0 * c4beta_) * c4)
                    * s1p5
                    + c1 * (-27.0 / 32.0
                        - 4689.0 / 256.0 * c2beta_
                        - 27.0 / 256.0 * c4beta_
                        - (27.0 / 16.0 + 27.0 / 32.0 * c2beta_ + 27.0 / 32.0 * c4beta_) * c2
                        + (81.0 / 256.0 * c2beta_ + 243.0 / 256.0 * c4beta_) * c4)
                        * s1p5);
            self.a[184] = self.delta
                * (self.eta
                    * ((4761.0 / 1024.0 - 1377.0 / 1024.0 * c2beta_) * s2beta_
                        + (837.0 / 256.0 - 621.0 / 256.0 * c2beta_) * c2 * s2beta_
                        + (243.0 / 1024.0 - 2187.0 / 1024.0 * c2beta_) * c4 * s2beta_)
                    * s1p6
                    + ((-11673.0 / 2048.0 + 1377.0 / 2048.0 * c2beta_) * s2beta_
                        + (-837.0 / 512.0 + 621.0 / 512.0 * c2beta_) * c2 * s2beta_
                        + (-243.0 / 2048.0 + 2187.0 / 2048.0 * c2beta_) * c4 * s2beta_)
                        * s1p6);
            self.a[185] = self.delta
                * (self.eta
                    * c1
                    * ((81.0 / 32.0 - 27.0 / 16.0 * c2beta_) * sbeta_2
                        - (81.0 / 32.0 + 81.0 / 16.0 * c2beta_) * c2 * sbeta_2)
                    * s1p7
                    + c1 * ((-81.0 / 64.0 + 27.0 / 32.0 * c2beta_) * sbeta_2
                        + (81.0 / 64.0 + 81.0 / 32.0 * c2beta_) * c2 * sbeta_2)
                        * s1p7);
            self.a[186] = self.delta
                * (81.0 / 64.0 - 81.0 / 32.0 * self.eta)
                * cbeta_
                * c1p2
                * sbeta_3
                * s1p8;
            self.a[187] = self.delta
                * (683.0 / 16384.0 * cbeta_ * sbeta_
                    + (557.0 / 4096.0 - 11.0 / 12288.0 * c2beta_) * c4 * s2beta_
                    + (-1719.0 / 32768.0 + 91.0 / 32768.0 * c2beta_) * c6 * s2beta_
                    - 1.0 / 16384.0 * cbeta_ * s3beta_
                    + c2 * (-10511.0 / 49152.0 * cbeta_ * sbeta_
                        + 173.0 / 49152.0 * cbeta_ * s3beta_)
                    + self.eta
                        * (85.0 / 8192.0 * cbeta_ * sbeta_
                            + (-679.0 / 6144.0 + 11.0 / 6144.0 * c2beta_) * c4 * s2beta_
                            - (201.0 / 16384.0 + 91.0 / 16384.0 * c2beta_) * c6 * s2beta_
                            + 1.0 / 8192.0 * cbeta_ * s3beta_
                            + c2 * (6031.0 / 24576.0 * cbeta_ * sbeta_
                                - 173.0 / 24576.0 * cbeta_ * s3beta_)
                            - c10 * (7.0 / 49152.0 * s2beta_ + 7.0 / 32768.0 * s4beta_)
                            + c8 * (-37.0 / 24576.0 * s2beta_ + 91.0 / 16384.0 * s4beta_))
                    + c8 * (37.0 / 49152.0 * s2beta_ - 91.0 / 32768.0 * s4beta_)
                    + c10 * (7.0 / 98304.0 * s2beta_ + 7.0 / 65536.0 * s4beta_));
            self.a[188] = self.delta
                * (self.eta
                    * (19.0 / 512.0 * c4beta_ * c3
                        + 9.0 / 512.0 * c4beta_ * c5
                        + c1 * (-11.0 / 16.0 - 35.0 / 128.0 * c2beta_
                            + 79.0 / 1536.0 * c4beta_
                            + (1.0 / 32.0 - 37.0 / 256.0 * c2beta_) * c2
                            + (1.0 / 32.0 + 3.0 / 128.0 * c2beta_) * c4
                            - 1.0 / 768.0 * c2beta_ * c6)
                        - 1.0 / 512.0 * c4beta_ * c7)
                    * s1p3
                    + (-19.0 / 1024.0 * c4beta_ * c3 - 9.0 / 1024.0 * c4beta_ * c5
                        + c1 * (19.0 / 32.0
                            - 23.0 / 768.0 * c2beta_
                            - 79.0 / 3072.0 * c4beta_
                            - (1.0 / 64.0 + 347.0 / 512.0 * c2beta_) * c2
                            - (1.0 / 64.0 + 3.0 / 256.0 * c2beta_) * c4
                            + 1.0 / 1536.0 * c2beta_ * c6)
                        + 1.0 / 1024.0 * c4beta_ * c7)
                        * s1p3);
            self.a[189] = self.delta
                * (c1p2
                    * (-355.0 / 1024.0 * s2beta_
                        - c2 * (13.0 / 256.0 * s2beta_ + 11.0 / 512.0 * s4beta_)
                        + c4 * (-1.0 / 1024.0 * s2beta_ + 9.0 / 2048.0 * s4beta_)
                        - 5.0 / 2048.0 * s4beta_)
                    * s1p4
                    + self.eta
                        * c1p2
                        * (-29.0 / 512.0 * s2beta_
                            + c4 * (1.0 / 512.0 * s2beta_ - 9.0 / 1024.0 * s4beta_)
                            + c2 * (13.0 / 128.0 * s2beta_ + 11.0 / 256.0 * s4beta_)
                            + 5.0 / 1024.0 * s4beta_)
                        * s1p4);
            self.a[190] = self.delta
                * (self.eta
                    * c1p3
                    * ((7.0 / 48.0 + 1.0 / 24.0 * c2beta_) * sbeta_2
                        - (1.0 / 48.0 + 1.0 / 24.0 * c2beta_) * c2 * sbeta_2)
                    * s1p5
                    + c1p3
                        * (-(7.0 / 96.0 + 1.0 / 48.0 * c2beta_) * sbeta_2
                            + (1.0 / 96.0 + 1.0 / 48.0 * c2beta_) * c2 * sbeta_2)
                        * s1p5);
            self.a[191] =
                self.delta * (1.0 / 96.0 - 1.0 / 48.0 * self.eta) * cbeta_ * c1p4 * sbeta_3 * s1p6;
            self.a[192] = self.delta
                * ((-77.0 / 256.0 + 1.0 / 256.0 * cbeta_) * sbeta_2 * s4
                    + (5.0 / 512.0 + 7.0 / 512.0 * c2beta_) * sbeta_2 * s8
                    + self.eta
                        * ((45.0 / 128.0 - 1.0 / 128.0 * c2beta_) * sbeta_2 * s4
                            - (5.0 / 256.0 - 7.0 / 256.0 * c2beta_) * sbeta_2 * s8));
            self.a[193] = self.delta
                * (135.0 / 64.0 + 189.0 / 64.0 * c2beta_
                    - self.eta * (135.0 / 32.0 + 189.0 / 32.0 * c2beta_))
                * c2
                * sbeta_2
                * s2p3;
            self.a[194] = self.delta
                * (-683.0 / 16384.0 * cbeta_ * sbeta_
                    + (-557.0 / 4096.0 + 11.0 / 12288.0 * c2beta_) * c4 * s2beta_
                    + (-1719.0 / 32768.0 + 91.0 / 32768.0 * c2beta_) * c6 * s2beta_
                    + 1.0 / 16384.0 * cbeta_ * sbeta_
                    + c2 * (-10511.0 / 49152.0 * cbeta_ * sbeta_
                        + 173.0 / 49152.0 * cbeta_ * s3beta_)
                    + self.eta
                        * (-85.0 / 8192.0 * cbeta_ * sbeta_
                            + (679.0 / 6144.0 - 11.0 / 6144.0 * c2beta_) * c4 * s2beta_
                            - (201.0 / 16384.0 + 91.0 / 16384.0 * c2beta_) * c6 * s2beta_
                            - 1.0 / 8192.0 * cbeta_ * s3beta_
                            + c2 * (6031.0 / 24576.0 * cbeta_ * sbeta_
                                - 173.0 / 24576.0 * cbeta_ * s3beta_)
                            + c8 * (37.0 / 24576.0 * s2beta_ - 91.0 / 16384.0 * s4beta_)
                            - c10 * (7.0 / 49152.0 * s2beta_ + 7.0 / 32768.0 * s4beta_))
                    + c10 * (7.0 / 98304.0 * s2beta_ + 7.0 / 65536.0 * s4beta_)
                    + c8 * (-37.0 / 49152.0 * s2beta_ + 91.0 / 32768.0 * s4beta_));
            self.a[195] = self.delta
                * (c1p3
                    * (19.0 / 32.0 - 23.0 / 768.0 * c2beta_ - 79.0 / 3072.0 * c4beta_
                        + (1.0 / 64.0 + 347.0 / 512.0 * c2beta_) * c2
                        - (1.0 / 64.0 + 3.0 / 256.0 * c2beta_) * c4
                        - 1.0 / 1536.0 * c2beta_ * c6)
                    * s1
                    + 19.0 / 1024.0 * c4beta_ * c1p3 * s3
                    - 9.0 / 1024.0 * c4beta_ * c1p3 * s5
                    - 1.0 / 1024.0 * c4beta_ * c1p3 * s7
                    + self.eta
                        * (c1p3
                            * (-11.0 / 16.0 - 35.0 / 128.0 * c2beta_
                                + 79.0 / 1536.0 * c4beta_
                                + (-1.0 / 32.0 + 37.0 / 256.0 * c2beta_) * c2
                                + (1.0 / 32.0 + 3.0 / 128.0 * c2beta_) * c4
                                + 1.0 / 768.0 * c2beta_ * c6)
                            * s1
                            - 19.0 / 512.0 * c4beta_ * c1p3 * s3
                            + 9.0 / 512.0 * c4beta_ * c1p3 * s5
                            + 1.0 / 512.0 * c4beta_ * c1p3 * s7));
            self.a[196] = self.delta
                * (self.eta
                    * c1p4
                    * (4923.0 / 512.0 * s2beta_
                        + c4 * (567.0 / 1024.0 * s2beta_ + 1701.0 / 2048.0 * s4beta_)
                        - 945.0 / 2048.0 * s4beta_
                        + c2 * (-459.0 / 256.0 * s2beta_ + 2079.0 / 512.0 * s4beta_))
                    * s1p2);
            self.a[197] = self.delta
                * (self.eta
                    * c1p5
                    * (27.0 / 16.0 + 1233.0 / 128.0 * c2beta_ + 27.0 / 128.0 * c4beta_
                        - (27.0 / 8.0 + 27.0 / 16.0 * c2beta_ + 27.0 / 16.0 * c4beta_) * c2
                        - (81.0 / 128.0 * c2beta_ + 243.0 / 128.0 * c4beta_) * c4)
                    * s1
                    + c1p5
                        * (-27.0 / 32.0 - 4689.0 / 256.0 * c2beta_ - 27.0 / 256.0 * c4beta_
                            + (27.0 / 16.0 + 27.0 / 32.0 * c2beta_ + 27.0 / 32.0 * c4beta_) * c2
                            + (81.0 / 256.0 * c2beta_ + 243.0 / 256.0 * c4beta_) * c4)
                        * s1);
            // The multiplication denominator, first time seeing that.;
            self.a[198] = self.delta
                * (c1p6
                    * (11673.0 / 2048.0 * s2beta_
                        + c4 * (243.0 / 2048.0 * s2beta_ - 2187.0 / 4096.0 * s4beta_)
                        + c2 * (-837.0 / 512.0 * s2beta_ + 621.0 / 1024.0 * s4beta_)
                        - 1377.0 / 4096.0 * s4beta_)
                    + self.eta
                        * c1p6
                        * (-4761.0 / 1024.0 * s2beta_
                            + c2 * (837.0 / 256.0 * s2beta_ - 621.0 / 512.0 * s4beta_)
                            + 1377.0 / 2048.0 * s4beta_
                            + c4 * (-243.0 / 1024.0 * s2beta_ + 2187.0 / 2048.0 * s4beta_)));
            self.a[199] = self.delta
                * (c1p7
                    * ((-81.0 / 64.0 + 27.0 / 32.0 * c2beta_) * sbeta_2
                        - (81.0 / 64.0 + 81.0 / 32.0 * c2beta_) * c2 * sbeta_2)
                    * s1
                    + self.eta
                        * c1p7
                        * ((81.0 / 32.0 - 27.0 / 16.0 * c2beta_) * sbeta_2
                            + (81.0 / 32.0 + 81.0 / 16.0 * c2beta_) * c2 * sbeta_2)
                        * s1);
            self.a[200] = self.delta
                * (81.0 / 32.0 * self.eta - 81.0 / 64.0)
                * cbeta_
                * c1p8
                * sbeta_3
                * s1p2;
            self.a[201] = self.delta
                * (4375.0 / 384.0 * s2beta_ + 4375.0 / 256.0 * s4beta_
                    - self.eta * (4375.0 / 192.0 * s2beta_ + 4375.0 / 128.0 * s4beta_))
                * c1p6
                * s1p4;
            self.a[202] = self.delta
                * (625.0 / 96.0 * c2beta_ + 625.0 / 32.0 * c4beta_
                    - self.eta * (625.0 / 48.0 * c2beta_ + 625.0 / 16.0 * c4beta_))
                * c1p7
                * s1p3;
            self.a[203] = self.delta
                * (625.0 / 256.0 * s2beta_ - 5625.0 / 512.0 * s4beta_
                    + self.eta * (-625.0 / 128.0 * s2beta_ + 5625.0 / 256.0 * s4beta_))
                * c1p8
                * s1p2;
            self.a[204] = self.delta
                * (625.0 / 96.0 + 625.0 / 48.0 * c2beta_
                    - self.eta * (625.0 / 48.0 + 625.0 / 24.0 * c2beta_))
                * c1p9
                * sbeta_2
                * s1;
            self.a[205] =
                self.delta * (625.0 / 96.0 * self.eta - 625.0 / 192.0) * cbeta_ * c1p10 * sbeta_3;
            //HX 3.0/2.0, SO
            self.a[206] = self.chisy_dn * (2.0 * c2p3 * sbeta_ - self.eta * c2p3 * sbeta_);
            self.a[207] = self.chisy_dn
                * (self.eta * c1p4 * (-5.0 / 3.0 * sbeta_ + 2.0 / 3.0 * c2 * sbeta_)
                    + c1p4 * (-14.0 / 3.0 * sbeta_ + 20.0 / 3.0 * c2 * sbeta_));
            self.a[208] = self.chisy_dn
                * (-20.0 / 3.0 * cbeta_ * c1p5 * s1 - 2.0 / 3.0 * self.eta * cbeta_ * c1p5 * s1);
            self.a[209] = self.chisy_dn
                * (self.eta * c1p3 * (7.0 / 3.0 * cbeta_ + 1.0 / 3.0 * cbeta_ * c2) * s1
                    + c1p3 * (-2.0 / 3.0 * cbeta_ + 10.0 / 3.0 * cbeta_ * c2) * s1);
            self.a[210] = self.chisy_dn
                * (self.eta * c1 * (7.0 / 3.0 * cbeta_ - 1.0 / 3.0 * cbeta_ * c2) * s1p3
                    + c1 * (-2.0 / 3.0 * cbeta_ - 10.0 / 3.0 * cbeta_ * c2) * s1p3);
            // The bigg?;
            //A[211] = chisyDN*(eta*(5.0/3.0 *sbeta_ + 2.0/3.0 *c2*sbeta_)*s1p4 + bigg*(14.0/3.0 *sbeta_ + 20.0/3.0 *c2*sbeta_)*s1p4)
            self.a[211] = self.chisy_dn
                * s1p4
                * sbeta_
                * (7.0 + 5.0 / 2.0 * self.eta + c2 * (10.0 + self.lota_dn));
            self.a[212] = self.chisy_dn
                * (-20.0 / 3.0 * cbeta_ * c1 * s1p5 - 2.0 / 3.0 * self.eta * cbeta_ * c1 * s1p5);
            self.a[213] =
                self.chisy_dn * (2.0 * c2 * sbeta_ * s2p2 - self.eta * c2 * sbeta_ * s2p2);
            self.a[214] = self.chisy_dn
                * (10.0 / 3.0 * c2 * sbeta_ * s2p2 + 1.0 / 3.0 * self.eta * c2 * sbeta_ * s2p2);
            self.a[215] = self.chisy_dn * (-cbeta_ * s2p3 + 1.0 / 2.0 * self.eta * cbeta_ * s2p3);
            self.a[216] = self.chisy_dn
                * (-5.0 / 4.0 * cbeta_ * s2 - 1.0 / 4.0 * cbeta_ * s6
                    + self.eta * (5.0 / 8.0 * cbeta_ * s2 + 1.0 / 8.0 * cbeta_ * s6));
            self.a[217] = self.chisx_dn
                * ((-3.0 / 2.0 * cbeta_ - 1.0 / 2.0 * cbeta_ * c4) * s2
                    + self.eta * (3.0 / 4.0 * cbeta_ + 1.0 / 4.0 * cbeta_ * c4) * s2)
                + self.chisz_dn * (-2.0 * c4 * sbeta_ * s2 + self.lota_dn * c4 * sbeta_ * s2);
            self.a[218] = self.chisz_dn
                * (2.0 * cbeta_ * c2 * s2p2 - self.eta * cbeta_ * c2 * s2p2)
                + self.chisx_dn * (-2.0 * c2 * sbeta_ * s2p2 + self.lota_dn * c2 * sbeta_ * s2p2);
            self.a[219] = self.chisx_dn * (cbeta_ * s2p3 - 1.0 / 2.0 * self.eta * cbeta_ * s2p3);
            self.a[220] = self.chisx_dn
                * (c1 * (-2.0 / 3.0 * cbeta_ - 10.0 / 3.0 * cbeta_ * c2) * s1p3
                    + self.eta
                        * c1
                        * (-5.0 / 3.0 * cbeta_ + 4.0 * c3beta_ - 1.0 / 3.0 * cbeta_ * c2)
                        * s1p3)
                + self.chisz_dn
                    * (c1 * (-4.0 * sbeta_ - 40.0 / 3.0 * c2 * sbeta_) * s1p3
                        + self.eta
                            * c1
                            * (-2.0 * sbeta_ - 4.0 / 3.0 * c2 * sbeta_ - 4.0 * s3beta_)
                            * s1p3);
            self.a[221] = self.chisz_dn
                * (self.eta * (5.0 * cbeta_ + c3beta_ + 2.0 / 3.0 * cbeta_ * c2) * s1p4
                    + (4.0 * cbeta_ + 20.0 / 3.0 * cbeta_ * c2) * s1p4)
                + self.chisx_dn
                    * ((-14.0 / 3.0 * sbeta_ - 20.0 / 3.0 * c2 * sbeta_) * s1p4
                        + self.eta
                            * (10.0 / 3.0 * sbeta_ - 2.0 / 3.0 * c2 * sbeta_ + s3beta_)
                            * s1p4);
            self.a[222] = self.chisx_dn
                * (20.0 / 3.0 * cbeta_ * c1 * s1p5 + 2.0 / 3.0 * self.eta * cbeta_ * c1 * s1p5);
            self.a[223] = -6.0 * self.eta * self.chisz_dn * cbeta_ * sbeta_2 * s2p2
                + self.chisx_dn
                    * (1.0 / 3.0 * sbeta_ * s2p2
                        + self.lota_dn * (-7.0 / 6.0 + 3.0 * c2beta_) * sbeta_ * s2p2);
            self.a[224] = self.chisx_dn
                * (self.eta
                    * c1p3
                    * (-5.0 / 3.0 * cbeta_ + 4.0 * c3beta_ + 1.0 / 3.0 * cbeta_ * c2)
                    * s1
                    + c1p3 * (-2.0 / 3.0 * cbeta_ + 10.0 / 3.0 * cbeta_ * c2) * s1)
                + self.chisz_dn
                    * (c1p3 * (-4.0 * sbeta_ + 40.0 / 3.0 * c2 * sbeta_) * s1
                        + self.eta
                            * c1p3
                            * (-2.0 * sbeta_ + 4.0 / 3.0 * c2 * sbeta_ - 4.0 * s3beta_)
                            * s1);
            self.a[225] = self.chisz_dn
                * (self.eta * c1p4 * (-5.0 * cbeta_ - c3beta_ + 2.0 / 3.0 * cbeta_ * c2)
                    + c1p4 * (-4.0 * cbeta_ + 20.0 / 3.0 * cbeta_ * c2))
                + self.chisx_dn
                    * (c1p4 * (14.0 / 3.0 * sbeta_ - 20.0 / 3.0 * c2 * sbeta_)
                        + self.eta
                            * c1p4
                            * (-10.0 / 3.0 * sbeta_ - 2.0 / 3.0 * c2 * sbeta_ - s3beta_));
            self.a[226] = self.chisx_dn
                * (20.0 / 3.0 * cbeta_ * c1p5 * s1 + 2.0 / 3.0 * self.eta * cbeta_ * c1p5 * s1);
            self.a[227] = self.delta * (2.0 * self.chiay_dn * c2p3 * sbeta_);
            self.a[228] = self.delta
                * (self.chiay_dn * c1p4 * (-14.0 / 3.0 * sbeta_ + 20.0 / 3.0 * c2 * sbeta_));
            self.a[229] = self.delta
                * (self.chiay_dn * c1p3 * (-2.0 / 3.0 * cbeta_ + 10.0 / 3.0 * cbeta_ * c2) * s1);
            self.a[230] = self.delta * (-20.0 / 3.0 * self.chiay_dn * cbeta_ * c1p5 * s1);
            self.a[231] = self.delta
                * (self.chiay_dn * c1 * (-2.0 / 3.0 * cbeta_ - 10.0 / 3.0 * cbeta_ * c2) * s1p3);
            self.a[232] = self.delta
                * (self.chiay_dn * (14.0 / 3.0 * sbeta_ + 20.0 / 3.0 * c2 * sbeta_) * s1p4);
            self.a[233] = self.delta * (-20.0 / 3.0 * self.chiay_dn * cbeta_ * c1 * s1p5);
            self.a[234] = self.delta * (2.0 * self.chiay_dn * c2 * sbeta_ * s2p2);
            // Interesting + sign in sheet
            self.a[235] = self.delta * (10.0 / 3.0 * self.chiay_dn * c2 * sbeta_ * s2p2);
            self.a[236] = self.delta * (-self.chiay_dn * cbeta_ * s1p3);
            self.a[237] =
                self.delta * (self.chiay_dn * (-5.0 / 4.0 * cbeta_ * s2 - 1.0 / 4.0 * cbeta_ * s6));
            self.a[238] = self.delta
                * (self.chiax_dn * (-3.0 / 2.0 * cbeta_ - 1.0 / 2.0 * cbeta_ * c4) * s2
                    - 2.0 * self.chiaz_dn * c4 * sbeta_ * s2);
            self.a[239] = self.delta
                * (2.0 * self.chiaz_dn * cbeta_ * c2 * s2p2
                    - 2.0 * self.chiax_dn * c2 * sbeta_ * s2p2);
            self.a[240] = self.delta * (self.chiax_dn * cbeta_ * s2p3);
            self.a[241] = self.delta
                * (self.chiax_dn * c1 * (-2.0 / 3.0 * cbeta_ - 10.0 / 3.0 * cbeta_ * c2) * s1p3
                    + self.chiaz_dn * c1 * (-4.0 * sbeta_ - 40.0 / 3.0 * c2 * sbeta_) * s1p3);
            self.a[242] = self.delta
                * (self.chiaz_dn * (4.0 * cbeta_ + 20.0 / 3.0 * cbeta_ * c2) * s1p4
                    + self.chiax_dn * (-14.0 / 3.0 * sbeta_ - 20.0 / 3.0 * c2 * sbeta_) * s1p4);
            self.a[243] = self.delta * (20.0 / 3.0 * self.chiax_dn * cbeta_ * c1 * s1p5);
            self.a[244] = self.delta * (1.0 / 3.0 * self.chiax_dn * sbeta_ * s2p2);
            self.a[245] = self.delta
                * (self.chiax_dn * c1p3 * (-2.0 / 3.0 * cbeta_ + 10.0 / 3.0 * cbeta_ * c2) * s1
                    + self.chiaz_dn * c1p3 * (-4.0 * sbeta_ + 40.0 / 3.0 * c2 * sbeta_) * s1);
            self.a[246] = self.delta
                * (self.chiaz_dn * c1p4 * (-4.0 * cbeta_ + 20.0 / 3.0 * cbeta_ * c2)
                    + self.chiax_dn * c1p4 * (14.0 / 3.0 * sbeta_ - 20.0 / 3.0 * c2 * sbeta_));
            self.a[247] = self.delta * (20.0 / 3.0 * self.chiax_dn * cbeta_ * c1p5 * s1);
        }
    }
}
