//CalculateWaveFactors
use crate::wave_builder::wave_builder::WaveBuilder;
impl WaveBuilder{
    pub fn calculate_wave_factors(&mut self) -> [f64; 248] {

        // Calculate the received wave phase

        // Calculate basic angle multiples for the phase _Psi
        self.cos_ap_psi[0][1] = (self.psi_r_dn).cos();
        self.sin_ap_psi[0][1] = (self.psi_r_dn).sin();
        
        self.cos_ap_psi[0][2] = self.cos_ap_psi[0][1]*self.cos_ap_psi[0][1] - self.sin_ap_psi[0][1]*self.sin_ap_psi[0][1];
        self.sin_ap_psi[0][2]  = 2.0*self.cos_ap_psi[0][1]*self.sin_ap_psi[0][1];
        self.cos_ap_psi[0][3] = self.cos_ap_psi[0][2]*self.cos_ap_psi[0][1] - self.sin_ap_psi[0][2]*self.sin_ap_psi[0][1];
        self.sin_ap_psi[0][3]  = self.sin_ap_psi[0][2]*self.cos_ap_psi[0][1] + self.cos_ap_psi[0][2]*self.sin_ap_psi[0][1];
        self.cos_ap_psi[0][4] = self.cos_ap_psi[0][3]*self.cos_ap_psi[0][1] - self.sin_ap_psi[0][3]*self.sin_ap_psi[0][1];
        self.sin_ap_psi[0][4]  = self.sin_ap_psi[0][3]*self.cos_ap_psi[0][1] + self.cos_ap_psi[0][3]*self.sin_ap_psi[0][1];
        self.cos_ap_psi[0][5] = self.cos_ap_psi[0][4]*self.cos_ap_psi[0][1] - self.sin_ap_psi[0][4]*self.sin_ap_psi[0][1];
        self.sin_ap_psi[0][5]  = self.sin_ap_psi[0][4]*self.cos_ap_psi[0][1] + self.cos_ap_psi[0][4]*self.sin_ap_psi[0][1];

        // Calculate basic angle multiples for the phase α
        self.cos_ap_psi[1][0] = (self.alpha_dn).cos();
        self.sin_ap_psi[1][0] = (self.alpha_dn).sin();
        self.cos_ap_psi[2][0] = self.cos_ap_psi[1][0]*self.cos_ap_psi[1][0] - self.sin_ap_psi[1][0]*self.sin_ap_psi[1][0];
        self.sin_ap_psi[2][0]  = 2.0*self.cos_ap_psi[1][0]*self.sin_ap_psi[1][0];
        self.cos_ap_psi[3][0] = self.cos_ap_psi[2][0]*self.cos_ap_psi[1][0] - self.sin_ap_psi[2][0]*self.sin_ap_psi[1][0];
        self.sin_ap_psi[3][0]  = self.sin_ap_psi[2][0]*self.cos_ap_psi[1][0] + self.cos_ap_psi[2][0]*self.sin_ap_psi[1][0];
        self.cos_ap_psi[4][0] = self.cos_ap_psi[3][0]*self.cos_ap_psi[1][0] - self.sin_ap_psi[3][0]*self.sin_ap_psi[1][0];
        self.sin_ap_psi[4][0]  = self.sin_ap_psi[3][0]*self.cos_ap_psi[1][0] + self.cos_ap_psi[3][0]*self.sin_ap_psi[1][0];
        self.cos_ap_psi[5][0] = self.cos_ap_psi[4][0]*self.cos_ap_psi[1][0] - self.sin_ap_psi[4][0]*self.sin_ap_psi[1][0];
        self.sin_ap_psi[5][0]  = self.sin_ap_psi[4][0]*self.cos_ap_psi[1][0] + self.cos_ap_psi[4][0]*self.sin_ap_psi[1][0];

        // Now basically calculate all possible combinations and weight according to the
        // noise at a given frequency
        for k in 1..5{
            for j in 1 ..5{
                self.cos_ap_psi[j][k] = self.cos_ap_psi[j][0]*self.cos_ap_psi[0][k] - self.sin_ap_psi[j][0]*self.sin_ap_psi[0][k];
                self.cos_am_psi[j][k] = self.cos_ap_psi[j][0]*self.cos_ap_psi[0][k] + self.sin_ap_psi[j][0]*self.sin_ap_psi[0][k];
                self.sin_ap_psi[j][k]  = self.sin_ap_psi[j][0]*self.cos_ap_psi[0][k] + self.cos_ap_psi[j][0]*self.sin_ap_psi[0][k];
                self.sin_am_psi[j][k]  = self.sin_ap_psi[j][0]*self.cos_ap_psi[0][k] - self.cos_ap_psi[j][0]*self.sin_ap_psi[0][k];
            }
            self.cos_am_psi[0][k] = self.cos_ap_psi[0][k];
            self.sin_am_psi[0][k] = -self.sin_ap_psi[0][k];
        }
        // Now calculate all wavy parts
        // Factors for H0P
        self.w[0] = self.cos_ap_psi[2][2] ; // cos(2α + 2_Psi];
        self.w[1] = self.cos_ap_psi[1][2]  ; // cos(α + 2_Psi];
        self.w[2] = self.cos_am_psi[1][2] ; // cos(α - 2_Psi];
        self.w[3] =  self.cos_am_psi[2][2] ;// cos(2α - 2_Psi];
        self.w[4] = self.cos_ap_psi[0][2] ; // cos(2_Psi];

        if self.pn_order > 0 {

            // Factors for H1P
            self.w[5] = self.cos_ap_psi[3][3];  // cos(3α + 3_Psi]
            self.w[6] = self.cos_ap_psi[1][1];  // cos(α + _Psi]
            self.w[7] = self.cos_am_psi[1][1];   // cos(α - _Psi]
            self.w[8] = self.cos_ap_psi[3][1];   // cos(3α + _Psi]
            self.w[9] = self.cos_ap_psi[1][3];   // cos(α + 3_Psi]
            self.w[10] = self.cos_am_psi[1][3];   // cos(α - 3_Psi]
            self.w[11] = self.cos_am_psi[3][1];   // cos(3α - _Psi]
            self.w[12] = self.cos_am_psi[3][3];  // cos(3α - 3_Psi]
            self.w[13] = self.cos_ap_psi[0][3];   // cos(3_Psi]
            self.w[14] = self.cos_ap_psi[2][1];   // cos(2α + _Psi]
            self.w[15] = self.cos_ap_psi[2][3];   // cos(2α + 3_Psi]
            self.w[16] = self.cos_am_psi[2][1];   // cos(2α  - _Psi]
            self.w[17] = self.cos_am_psi[2][3];   // cos(2α - 3_Psi]
            self.w[18] = self.cos_ap_psi[0][1];  // cos(_Psi]

        }
        if self.pn_order > 1 {

            // Factors for H2P
            self.w[19] =  self.cos_ap_psi[2][2];
            self.w[20] =  self.cos_ap_psi[4][4];
            self.w[21] =  self.cos_ap_psi[3][4];
            self.w[22] =  self.cos_ap_psi[3][2];
            self.w[23] =  self.cos_ap_psi[2][4];
            self.w[24] =  self.cos_ap_psi[4][2];
            self.w[25] =  self.cos_ap_psi[1][4];
            self.w[26] =  self.cos_am_psi[1][2];
            self.w[27] =  self.cos_am_psi[2][2];
            self.w[28] =  self.cos_am_psi[1][4];
            self.w[29] =  self.cos_am_psi[3][2];
            self.w[30] =  self.cos_am_psi[2][4];
            self.w[31] =  self.cos_am_psi[4][2];
            self.w[32] =   self.cos_am_psi[3][4];
            self.w[33] =   self.cos_am_psi[4][4];
            self.w[34] =   self.cos_ap_psi[0][2];
            self.w[35] =   self.cos_ap_psi[0][4];
            self.w[36] =   self.cos_ap_psi[1][2];

            self.w[37] =   self.cos_ap_psi[1][1];
            self.w[38] =   self.cos_am_psi[1][1];
            self.w[39] =   self.sin_am_psi[1][1];
            self.w[40] =   self.sin_ap_psi[0][1];
            self.w[41] =   self.sin_ap_psi[1][1];
            self.w[42] =   self.cos_ap_psi[1][1];
            self.w[43] =   self.cos_am_psi[1][1];
            self.w[44] =   self.sin_am_psi[1][1];
            self.w[45] =   self.sin_ap_psi[0][1];
            self.w[46] =   self.sin_ap_psi[1][1];

        }

        if self.pn_order > 2 {

            // Factors for H3P

            self.w[47] =   self.cos_ap_psi[2][2];
            self.w[48] =   self.cos_ap_psi[1][2];
            self.w[49] =   self.cos_am_psi[1][2];
            self.w[50] =   self.cos_am_psi[2][2];
            self.w[51] =   self.cos_ap_psi[0][2];
            self.w[52] =   self.cos_ap_psi[5][5];
            self.w[53] =   self.cos_ap_psi[1][1];
            self.w[54] =   self.cos_ap_psi[3][3];
            self.w[55] =   self.cos_ap_psi[4][5];
            self.w[56] =   self.cos_ap_psi[4][3];
            self.w[57] =   self.cos_ap_psi[5][3];
            self.w[58] =   self.cos_am_psi[1][1];
            self.w[59] =   self.cos_ap_psi[3][1];
            self.w[60] =   self.cos_ap_psi[3][5];
            self.w[61] =   self.cos_ap_psi[1][3];
            self.w[62] =   self.cos_ap_psi[2][5];
            self.w[63] =   self.cos_ap_psi[4][1];
            self.w[64] =   self.cos_ap_psi[5][1];
            self.w[65] =   self.cos_am_psi[3][1];
            self.w[66] =   self.cos_ap_psi[1][5];
            self.w[67] =   self.cos_am_psi[1][3];
            self.w[68] =   self.cos_am_psi[4][1];
            self.w[69] =   self.cos_am_psi[5][1];
            self.w[70] =   self.cos_am_psi[3][3];
            self.w[71] =   self.cos_am_psi[1][5];
            self.w[72] =   self.cos_am_psi[2][5];
            self.w[73] =   self.cos_am_psi[4][3];
            self.w[74] =   self.cos_am_psi[5][3];
            self.w[75] =   self.cos_am_psi[3][5];
            self.w[76] =   self.cos_am_psi[4][5];
            self.w[77] =   self.cos_am_psi[5][5];
            self.w[78] =   self.cos_ap_psi[0][3];
            self.w[79] =   self.cos_ap_psi[0][5];
            self.w[80] =   self.cos_ap_psi[2][3];
            self.w[81] =   self.cos_am_psi[2][3];
            self.w[82] =   self.cos_ap_psi[2][1];
            self.w[83] =   self.cos_am_psi[2][1];
            self.w[84] =   self.cos_ap_psi[0][1];

            self.w[85] =   self.cos_ap_psi[0][0];
            self.w[86] =   self.cos_ap_psi[2][2];
            self.w[87] =   self.cos_ap_psi[3][2];
            self.w[88] =   self.cos_am_psi[3][2];
            self.w[89] =   self.cos_ap_psi[1][2];
            self.w[90] =   self.cos_am_psi[1][2];
            self.w[91] =   self.cos_am_psi[2][2];
            self.w[92] =   self.cos_ap_psi[0][0];
            self.w[93] =   self.cos_ap_psi[3][0];
            self.w[94] =   self.cos_ap_psi[0][2];
            self.w[95] =   self.cos_ap_psi[2][0];
            self.w[96] =   self.cos_ap_psi[1][0];
            self.w[97] =   self.sin_ap_psi[1][0];
            self.w[98] =   self.sin_ap_psi[2][0];
            self.w[99] =   self.sin_ap_psi[3][0];
            self.w[100] =   self.sin_am_psi[1][2];
            self.w[101] =   self.sin_am_psi[2][2];
            self.w[102] =   self.sin_am_psi[3][2];
            self.w[103] =   self.sin_ap_psi[0][2];
            self.w[104] =   self.sin_ap_psi[1][2];
            self.w[105] =   self.sin_ap_psi[2][2];
            self.w[106] =   self.sin_ap_psi[3][2];
            self.w[107] =   self.cos_ap_psi[0][0];
            self.w[108] =   self.cos_ap_psi[2][2];
            self.w[109] =   self.cos_ap_psi[3][2];
            self.w[110] =   self.cos_am_psi[3][2];
            self.w[111] =   self.cos_ap_psi[1][2];
            self.w[112] =   self.cos_am_psi[1][2];
            self.w[113] =   self.cos_am_psi[2][2];
            self.w[114] =   self.cos_ap_psi[0][0];
            self.w[115] =   self.cos_ap_psi[3][0];
            self.w[116] =   self.cos_ap_psi[0][2];
            self.w[117] =   self.cos_ap_psi[2][0];
            self.w[118] =   self.cos_ap_psi[1][0];
            self.w[119] =   self.sin_ap_psi[1][0];
            self.w[120] =   self.sin_ap_psi[2][0];
            self.w[121] =   self.sin_ap_psi[3][0];
            self.w[122] =   self.sin_am_psi[1][2];
            self.w[123] =   self.sin_am_psi[2][2];
            self.w[124] =   self.sin_am_psi[3][2];
            self.w[125] =   self.sin_ap_psi[0][2];
            self.w[126] =   self.sin_ap_psi[1][2];
            self.w[127] =   self.sin_ap_psi[2][2];
            self.w[128] =   self.sin_ap_psi[3][2];

        }
        // Factors for H0X
        self.w[129] = self.sin_am_psi[1][2];
        self.w[130] = self.sin_am_psi[2][2];
        self.w[131] = self.sin_ap_psi[1][2];
        self.w[132] = self.sin_ap_psi[2][2];

        if self.pn_order > 0 {

            // Factors for H1X
            self.w[133] = self.sin_am_psi[1][3];  // sin(α  - 3_Psi]
            self.w[134] = self.sin_am_psi[2][3];  // sin(2α - 3_Psi]
            self.w[135] = self.sin_am_psi[3][3];  // sin(3α - 3_Psi]
            self.w[136]= self.sin_am_psi[1][1];    // sin(α - _Psi]
            self.w[137] = self.sin_am_psi[2][1];   // sin(2α - _Psi]
            self.w[138] = self.sin_am_psi[3][1];   // sin(3α - _Psi]
            self.w[139] = self.sin_ap_psi[0][1];   // sin(_Psi]
            self.w[140] = self.sin_ap_psi[1][1];   // sin(α + _Psi]
            self.w[141] = self.sin_ap_psi[2][1];   // sin(2α + _Psi]
            self.w[142] = self.sin_ap_psi[3][1];  // sin(3α + _Psi]
            self.w[143] = self.sin_ap_psi[1][3];  // sin(α + 3_Psi]
            self.w[144] = self.sin_ap_psi[2][3]; // sin(2α + 3_Psi]
            self.w[145] = self.sin_ap_psi[3][3]; // sin(3α + 3_Psi]

        }
        if self.pn_order > 1 {

            // Factors for H2X
            self.w[146] = self.sin_am_psi[1][4];
            self.w[147] = self.sin_am_psi[2][4];
            self.w[148] = self.sin_am_psi[3][4];
            self.w[149] = self.sin_am_psi[4][4];
            self.w[150] = self.sin_am_psi[1][2];
            self.w[151] = self.sin_am_psi[2][2];
            self.w[152] = self.sin_am_psi[3][2];
            self.w[153] = self.sin_am_psi[4][2];
            self.w[154] = self.sin_ap_psi[0][2];
            self.w[155] = self.sin_ap_psi[1][2];
            self.w[156] = self.sin_ap_psi[2][2];
            self.w[157] = self.sin_ap_psi[3][2];
            self.w[158] = self.sin_ap_psi[4][2];
            self.w[159] = self.sin_ap_psi[1][4];
            self.w[160] = self.sin_ap_psi[2][4];
            self.w[161] = self.sin_ap_psi[3][4];
            self.w[162] = self.sin_ap_psi[4][4];

            self.w[163] = self.cos_ap_psi[1][1];
            self.w[164] = self.cos_am_psi[1][1];
            self.w[165] = self.sin_am_psi[1][1];
            self.w[166] = self.sin_ap_psi[0][1];
            self.w[167] = self.sin_ap_psi[1][1];
            self.w[168] = self.cos_ap_psi[1][1];
            self.w[169] = self.cos_am_psi[1][1];
            self.w[170] = self.sin_am_psi[1][1];
            self.w[171] = self.sin_ap_psi[0][1];
            self.w[172] = self.sin_ap_psi[1][1];

        }
        if self.pn_order > 2{

            // Factors for H3X

            self.w[173] = self.sin_am_psi[1][2];
            self.w[174] = self.sin_am_psi[2][2];
            self.w[175] = self.sin_ap_psi[1][2];
            self.w[176] = self.sin_ap_psi[2][2];
            self.w[177] = self.sin_am_psi[1][5];
            self.w[178] = self.sin_am_psi[2][5];
            self.w[179] = self.sin_am_psi[3][5];
            self.w[180] = self.sin_am_psi[4][5];
            self.w[181] = self.sin_am_psi[5][5];
            self.w[182] = self.sin_am_psi[1][3];
            self.w[183] = self.sin_am_psi[2][3];
            self.w[184] = self.sin_am_psi[3][3];
            self.w[185] = self.sin_am_psi[4][3];
            self.w[186] = self.sin_am_psi[5][3];
            self.w[187] = self.sin_am_psi[1][1];
            self.w[188] = self.sin_am_psi[2][1];
            self.w[189] = self.sin_am_psi[3][1];
            self.w[190] = self.sin_am_psi[4][1];
            self.w[191] = self.sin_am_psi[5][1];
            self.w[192] = self.sin_ap_psi[0][1];
            self.w[193] = self.sin_ap_psi[0][3];
            self.w[194] = self.sin_ap_psi[1][1];
            self.w[195] = self.sin_ap_psi[2][1];
            self.w[196] = self.sin_ap_psi[1][3];
            self.w[197] = self.sin_ap_psi[2][3];
            self.w[198] = self.sin_ap_psi[3][3];
            self.w[199] = self.sin_ap_psi[4][3];
            self.w[200] = self.sin_ap_psi[5][3];
            self.w[201] = self.sin_ap_psi[1][5];
            self.w[202] = self.sin_ap_psi[2][5];
            self.w[203] = self.sin_ap_psi[3][5];
            self.w[204] = self.sin_ap_psi[4][5];
            self.w[205] = self.sin_ap_psi[5][5];

            self.w[206] = self.cos_ap_psi[0][0];
            self.w[207] = self.cos_ap_psi[2][2];
            self.w[208] = self.cos_ap_psi[3][2];
            self.w[209] = self.cos_ap_psi[1][2];
            self.w[210] = self.cos_am_psi[1][2];
            self.w[211] = self.cos_am_psi[2][2];
            self.w[212] = self.cos_am_psi[3][2];
            self.w[213] = self.cos_ap_psi[2][0];
            self.w[214] = self.cos_ap_psi[0][2];
            self.w[215] = self.cos_ap_psi[3][0];
            self.w[216] = self.cos_ap_psi[1][0];
            self.w[217] = self.sin_ap_psi[1][0];
            self.w[218] = self.sin_ap_psi[2][0];
            self.w[219] = self.sin_ap_psi[3][0];
            self.w[220] = self.sin_am_psi[1][2];
            self.w[221] = self.sin_am_psi[2][2];
            self.w[222] = self.sin_am_psi[3][2];
            self.w[223] = self.sin_ap_psi[0][2];
            self.w[224] = self.sin_ap_psi[1][2];
            self.w[225] = self.sin_ap_psi[2][2];
            self.w[226] = self.sin_ap_psi[3][2];
            self.w[227] = self.cos_ap_psi[0][0];
            self.w[228] = self.cos_ap_psi[2][2];
            self.w[229] = self.cos_ap_psi[1][2];
            self.w[230] = self.cos_ap_psi[3][2];
            self.w[231] = self.cos_am_psi[1][2];
            self.w[232] = self.cos_am_psi[2][2];
            self.w[233] = self.cos_am_psi[3][2];
            self.w[234] = self.cos_ap_psi[2][0];
            self.w[235] = self.cos_ap_psi[0][2];
            self.w[236] = self.cos_ap_psi[3][0];
            self.w[237] = self.cos_ap_psi[1][0];
            self.w[239] = self.sin_ap_psi[2][0];
            self.w[240] = self.sin_ap_psi[3][0];
            self.w[241] = self.sin_am_psi[1][2];
            self.w[242] = self.sin_am_psi[2][2];
            self.w[243] = self.sin_am_psi[3][2];
            self.w[244] = self.sin_ap_psi[0][2];
            self.w[245] = self.sin_ap_psi[1][2];
            self.w[246] = self.sin_ap_psi[2][2];
            self.w[247] = self.sin_ap_psi[3][2];

        }
        self.w










    } 
}
