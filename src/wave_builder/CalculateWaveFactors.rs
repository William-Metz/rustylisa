//CalculateWaveFactors
use crate::wave_builder::wave_builder::Wave_Builder;
impl Wave_Builder{
    pub fn CalculateWaveFactors(&mut self) -> [f64; 248] {



        // Calculate the received wave phase

        // Calculate basic angle multiples for the phase _Psi
        self.Cos_Ap_Psi[0][1] = self.PsirDN.cos();
        self.Sin_Ap_Psi[0][1] = (self.PsirDN).sin();
        self.Cos_Ap_Psi[0][2] = self.Cos_Ap_Psi[0][1]*self.Cos_Ap_Psi[0][1] - self.Sin_Ap_Psi[0][1]*self.Sin_Ap_Psi[0][1];
        self.Sin_Ap_Psi[0][2]  = 2.0*self.Cos_Ap_Psi[0][1]*self.Sin_Ap_Psi[0][1];
        self.Cos_Ap_Psi[0][3] = self.Cos_Ap_Psi[0][2]*self.Cos_Ap_Psi[0][1] - self.Sin_Ap_Psi[0][2]*self.Sin_Ap_Psi[0][1];
        self.Sin_Ap_Psi[0][3]  = self.Sin_Ap_Psi[0][2]*self.Cos_Ap_Psi[0][1] + self.Cos_Ap_Psi[0][2]*self.Sin_Ap_Psi[0][1];
        self.Cos_Ap_Psi[0][4] = self.Cos_Ap_Psi[0][3]*self.Cos_Ap_Psi[0][1] - self.Sin_Ap_Psi[0][3]*self.Sin_Ap_Psi[0][1];
        self.Sin_Ap_Psi[0][4]  = self.Sin_Ap_Psi[0][3]*self.Cos_Ap_Psi[0][1] + self.Cos_Ap_Psi[0][3]*self.Sin_Ap_Psi[0][1];
        self.Cos_Ap_Psi[0][5] = self.Cos_Ap_Psi[0][4]*self.Cos_Ap_Psi[0][1] - self.Sin_Ap_Psi[0][4]*self.Sin_Ap_Psi[0][1];
        self.Sin_Ap_Psi[0][5]  = self.Sin_Ap_Psi[0][4]*self.Cos_Ap_Psi[0][1] + self.Cos_Ap_Psi[0][4]*self.Sin_Ap_Psi[0][1];

        // Calculate basic angle multiples for the phase α
        self.Cos_Ap_Psi[1][0] = (self.AlphaDN).cos();
        self.Sin_Ap_Psi[1][0] = (self.AlphaDN).sin();
        self.Cos_Ap_Psi[2][0] = self.Cos_Ap_Psi[1][0]*self.Cos_Ap_Psi[1][0] - self.Sin_Ap_Psi[1][0]*self.Sin_Ap_Psi[1][0];
        self.Sin_Ap_Psi[2][0]  = 2.0*self.Cos_Ap_Psi[1][0]*self.Sin_Ap_Psi[1][0];
        self.Cos_Ap_Psi[3][0] = self.Cos_Ap_Psi[2][0]*self.Cos_Ap_Psi[1][0] - self.Sin_Ap_Psi[2][0]*self.Sin_Ap_Psi[1][0];
        self.Sin_Ap_Psi[3][0]  = self.Sin_Ap_Psi[2][0]*self.Cos_Ap_Psi[1][0] + self.Cos_Ap_Psi[2][0]*self.Sin_Ap_Psi[1][0];
        self.Cos_Ap_Psi[4][0] = self.Cos_Ap_Psi[3][0]*self.Cos_Ap_Psi[1][0] - self.Sin_Ap_Psi[3][0]*self.Sin_Ap_Psi[1][0];
        self.Sin_Ap_Psi[4][0]  = self.Sin_Ap_Psi[3][0]*self.Cos_Ap_Psi[1][0] + self.Cos_Ap_Psi[3][0]*self.Sin_Ap_Psi[1][0];
        self.Cos_Ap_Psi[5][0] = self.Cos_Ap_Psi[4][0]*self.Cos_Ap_Psi[1][0] - self.Sin_Ap_Psi[4][0]*self.Sin_Ap_Psi[1][0];
        self.Sin_Ap_Psi[5][0]  = self.Sin_Ap_Psi[4][0]*self.Cos_Ap_Psi[1][0] + self.Cos_Ap_Psi[4][0]*self.Sin_Ap_Psi[1][0];

        // Now basically calculate all possible combinations and weight according to the
        // noise at a given frequency
        for k in 1..5{
            for j in 1 ..5{
                self.Cos_Ap_Psi[j][k] = (self.Cos_Ap_Psi[j][0]*self.Cos_Ap_Psi[0][k] - self.Sin_Ap_Psi[j][0]*self.Sin_Ap_Psi[0][k]);
                self.Cos_Am_Psi[j][k] = (self.Cos_Ap_Psi[j][0]*self.Cos_Ap_Psi[0][k] + self.Sin_Ap_Psi[j][0]*self.Sin_Ap_Psi[0][k]);
                self.Sin_Ap_Psi[j][k]  = (self.Sin_Ap_Psi[j][0]*self.Cos_Ap_Psi[0][k] + self.Cos_Ap_Psi[j][0]*self.Sin_Ap_Psi[0][k]);
                self.Sin_Am_Psi[j][k]  = (self.Sin_Ap_Psi[j][0]*self.Cos_Ap_Psi[0][k] - self.Cos_Ap_Psi[j][0]*self.Sin_Ap_Psi[0][k]);
            }
            self.Cos_Am_Psi[0][k] = self.Cos_Ap_Psi[0][k];
            self.Sin_Am_Psi[0][k] = -self.Sin_Ap_Psi[0][k];
        }
        // Now calculate all wavy parts
        // Factors for H0P
        self.W[0] = self.Cos_Ap_Psi[2][2] ; // cos(2α + 2_Psi];
        self.W[1] = self.Cos_Ap_Psi[1][2]  ; // cos(α + 2_Psi];
        self.W[2] = self.Cos_Am_Psi[1][2] ; // cos(α - 2_Psi];
        self.W[3] =  self.Cos_Am_Psi[2][2] ;// cos(2α - 2_Psi];
        self.W[4] = self.Cos_Ap_Psi[0][2] ; // cos(2_Psi];

        if self.PNOrder > 0 {

            // Factors for H1P
            self.W[5] = self.Cos_Ap_Psi[3][3];  // cos(3α + 3_Psi]
            self.W[6] = self.Cos_Ap_Psi[1][1];  // cos(α + _Psi]
            self.W[7] = self.Cos_Am_Psi[1][1];   // cos(α - _Psi]
            self.W[8] = self.Cos_Ap_Psi[3][1];   // cos(3α + _Psi]
            self.W[9] = self.Cos_Ap_Psi[1][3];   // cos(α + 3_Psi]
            self.W[10] = self.Cos_Am_Psi[1][3];   // cos(α - 3_Psi]
            self.W[11] = self.Cos_Am_Psi[3][1];   // cos(3α - _Psi]
            self.W[12] = self.Cos_Am_Psi[3][3];  // cos(3α - 3_Psi]
            self.W[13] = self.Cos_Ap_Psi[0][3];   // cos(3_Psi]
            self.W[14] = self.Cos_Ap_Psi[2][1];   // cos(2α + _Psi]
            self.W[15] = self.Cos_Ap_Psi[2][3];   // cos(2α + 3_Psi]
            self.W[16] = self.Cos_Am_Psi[2][1];   // cos(2α  - _Psi]
            self.W[17] = self.Cos_Am_Psi[2][3];   // cos(2α - 3_Psi]
            self.W[18] = self.Cos_Ap_Psi[0][1];  // cos(_Psi]

        }
        if self.PNOrder > 1 {

            // Factors for H2P
            self.W[19] =  self.Cos_Ap_Psi[2][2];
            self.W[20] =  self.Cos_Ap_Psi[4][4];
            self.W[21] =  self.Cos_Ap_Psi[3][4];
            self.W[22] =  self.Cos_Ap_Psi[3][2];
            self.W[23] =  self.Cos_Ap_Psi[2][4];
            self.W[24] =  self.Cos_Ap_Psi[4][2];
            self.W[25] =  self.Cos_Ap_Psi[1][4];
            self.W[26] =  self.Cos_Am_Psi[1][2];
            self.W[27] =  self.Cos_Am_Psi[2][2];
            self.W[28] =  self.Cos_Am_Psi[1][4];
            self.W[29] =  self.Cos_Am_Psi[3][2];
            self.W[30] =  self.Cos_Am_Psi[2][4];
            self.W[31] =  self.Cos_Am_Psi[4][2];
            self.W[32] =   self.Cos_Am_Psi[3][4];
            self.W[33] =   self.Cos_Am_Psi[4][4];
            self.W[34] =   self.Cos_Ap_Psi[0][2];
            self.W[35] =   self.Cos_Ap_Psi[0][4];
            self.W[36] =   self.Cos_Ap_Psi[1][2];

            self.W[37] =   self.Cos_Ap_Psi[1][1];
            self.W[38] =   self.Cos_Am_Psi[1][1];
            self.W[39] =   self.Sin_Am_Psi[1][1];
            self.W[40] =   self.Sin_Ap_Psi[0][1];
            self.W[41] =   self.Sin_Ap_Psi[1][1];
            self.W[42] =   self.Cos_Ap_Psi[1][1];
            self.W[43] =   self.Cos_Am_Psi[1][1];
            self.W[44] =   self.Sin_Am_Psi[1][1];
            self.W[45] =   self.Sin_Ap_Psi[0][1];
            self.W[46] =   self.Sin_Ap_Psi[1][1];

        }

        if self.PNOrder > 2 {

            // Factors for H3P

            self.W[47] =   self.Cos_Ap_Psi[2][2];
            self.W[48] =   self.Cos_Ap_Psi[1][2];
            self.W[49] =   self.Cos_Am_Psi[1][2];
            self.W[50] =   self.Cos_Am_Psi[2][2];
            self.W[51] =   self.Cos_Ap_Psi[0][2];
            self.W[52] =   self.Cos_Ap_Psi[5][5];
            self.W[53] =   self.Cos_Ap_Psi[1][1];
            self.W[54] =   self.Cos_Ap_Psi[3][3];
            self.W[55] =   self.Cos_Ap_Psi[4][5];
            self.W[56] =   self.Cos_Ap_Psi[4][3];
            self.W[57] =   self.Cos_Ap_Psi[5][3];
            self.W[58] =   self.Cos_Am_Psi[1][1];
            self.W[59] =   self.Cos_Ap_Psi[3][1];
            self.W[60] =   self.Cos_Ap_Psi[3][5];
            self.W[61] =   self.Cos_Ap_Psi[1][3];
            self.W[62] =   self.Cos_Ap_Psi[2][5];
            self.W[63] =   self.Cos_Ap_Psi[4][1];
            self.W[64] =   self.Cos_Ap_Psi[5][1];
            self.W[65] =   self.Cos_Am_Psi[3][1];
            self.W[66] =   self.Cos_Ap_Psi[1][5];
            self.W[67] =   self.Cos_Am_Psi[1][3];
            self.W[68] =   self.Cos_Am_Psi[4][1];
            self.W[69] =   self.Cos_Am_Psi[5][1];
            self.W[70] =   self.Cos_Am_Psi[3][3];
            self.W[71] =   self.Cos_Am_Psi[1][5];
            self.W[72] =   self.Cos_Am_Psi[2][5];
            self.W[73] =   self.Cos_Am_Psi[4][3];
            self.W[74] =   self.Cos_Am_Psi[5][3];
            self.W[75] =   self.Cos_Am_Psi[3][5];
            self.W[76] =   self.Cos_Am_Psi[4][5];
            self.W[77] =   self.Cos_Am_Psi[5][5];
            self.W[78] =   self.Cos_Ap_Psi[0][3];
            self.W[79] =   self.Cos_Ap_Psi[0][5];
            self.W[80] =   self.Cos_Ap_Psi[2][3];
            self.W[81] =   self.Cos_Am_Psi[2][3];
            self.W[82] =   self.Cos_Ap_Psi[2][1];
            self.W[83] =   self.Cos_Am_Psi[2][1];
            self.W[84] =   self.Cos_Ap_Psi[0][1];

            self.W[85] =   self.Cos_Ap_Psi[0][0];
            self.W[86] =   self.Cos_Ap_Psi[2][2];
            self.W[87] =   self.Cos_Ap_Psi[3][2];
            self.W[88] =   self.Cos_Am_Psi[3][2];
            self.W[89] =   self.Cos_Ap_Psi[1][2];
            self.W[90] =   self.Cos_Am_Psi[1][2];
            self.W[91] =   self.Cos_Am_Psi[2][2];
            self.W[92] =   self.Cos_Ap_Psi[0][0];
            self.W[93] =   self.Cos_Ap_Psi[3][0];
            self.W[94] =   self.Cos_Ap_Psi[0][2];
            self.W[95] =   self.Cos_Ap_Psi[2][0];
            self.W[96] =   self.Cos_Ap_Psi[1][0];
            self.W[97] =   self.Sin_Ap_Psi[1][0];
            self.W[98] =   self.Sin_Ap_Psi[2][0];
            self.W[99] =   self.Sin_Ap_Psi[3][0];
            self.W[100] =   self.Sin_Am_Psi[1][2];
            self.W[101] =   self.Sin_Am_Psi[2][2];
            self.W[102] =   self.Sin_Am_Psi[3][2];
            self.W[103] =   self.Sin_Ap_Psi[0][2];
            self.W[104] =   self.Sin_Ap_Psi[1][2];
            self.W[105] =   self.Sin_Ap_Psi[2][2];
            self.W[106] =   self.Sin_Ap_Psi[3][2];
            self.W[107] =   self.Cos_Ap_Psi[0][0];
            self.W[108] =   self.Cos_Ap_Psi[2][2];
            self.W[109] =   self.Cos_Ap_Psi[3][2];
            self.W[110] =   self.Cos_Am_Psi[3][2];
            self.W[111] =   self.Cos_Ap_Psi[1][2];
            self.W[112] =   self.Cos_Am_Psi[1][2];
            self.W[113] =   self.Cos_Am_Psi[2][2];
            self.W[114] =   self.Cos_Ap_Psi[0][0];
            self.W[115] =   self.Cos_Ap_Psi[3][0];
            self.W[116] =   self.Cos_Ap_Psi[0][2];
            self.W[117] =   self.Cos_Ap_Psi[2][0];
            self.W[118] =   self.Cos_Ap_Psi[1][0];
            self.W[119] =   self.Sin_Ap_Psi[1][0];
            self.W[120] =   self.Sin_Ap_Psi[2][0];
            self.W[121] =   self.Sin_Ap_Psi[3][0];
            self.W[122] =   self.Sin_Am_Psi[1][2];
            self.W[123] =   self.Sin_Am_Psi[2][2];
            self.W[124] =   self.Sin_Am_Psi[3][2];
            self.W[125] =   self.Sin_Ap_Psi[0][2];
            self.W[126] =   self.Sin_Ap_Psi[1][2];
            self.W[127] =   self.Sin_Ap_Psi[2][2];
            self.W[128] =   self.Sin_Ap_Psi[3][2];

        }
        // Factors for H0X
        self.W[129] = self.Sin_Am_Psi[1][2];
        self.W[130] = self.Sin_Am_Psi[2][2];
        self.W[131] = self.Sin_Ap_Psi[1][2];
        self.W[132] = self.Sin_Ap_Psi[2][2];

        if self.PNOrder > 0 {

            // Factors for H1X
            self.W[133] = self.Sin_Am_Psi[1][3];  // sin(α  - 3_Psi]
            self.W[134] = self.Sin_Am_Psi[2][3];  // sin(2α - 3_Psi]
            self.W[135] = self.Sin_Am_Psi[3][3];  // sin(3α - 3_Psi]
            self.W[136]= self.Sin_Am_Psi[1][1];    // sin(α - _Psi]
            self.W[137] = self.Sin_Am_Psi[2][1];   // sin(2α - _Psi]
            self.W[138] = self.Sin_Am_Psi[3][1];   // sin(3α - _Psi]
            self.W[139] = self.Sin_Ap_Psi[0][1];   // sin(_Psi]
            self.W[140] = self.Sin_Ap_Psi[1][1];   // sin(α + _Psi]
            self.W[141] = self.Sin_Ap_Psi[2][1];   // sin(2α + _Psi]
            self.W[142] = self.Sin_Ap_Psi[3][1];  // sin(3α + _Psi]
            self.W[143] = self.Sin_Ap_Psi[1][3];  // sin(α + 3_Psi]
            self.W[144] = self.Sin_Ap_Psi[2][3]; // sin(2α + 3_Psi]
            self.W[145] = self.Sin_Ap_Psi[3][3]; // sin(3α + 3_Psi]

        }
        if self.PNOrder > 1 {

            // Factors for H2X
            self.W[146] = self.Sin_Am_Psi[1][4];
            self.W[147] = self.Sin_Am_Psi[2][4];
            self.W[148] = self.Sin_Am_Psi[3][4];
            self.W[149] = self.Sin_Am_Psi[4][4];
            self.W[150] = self.Sin_Am_Psi[1][2];
            self.W[151] = self.Sin_Am_Psi[2][2];
            self.W[152] = self.Sin_Am_Psi[3][2];
            self.W[153] = self.Sin_Am_Psi[4][2];
            self.W[154] = self.Sin_Ap_Psi[0][2];
            self.W[155] = self.Sin_Ap_Psi[1][2];
            self.W[156] = self.Sin_Ap_Psi[2][2];
            self.W[157] = self.Sin_Ap_Psi[3][2];
            self.W[158] = self.Sin_Ap_Psi[4][2];
            self.W[159] = self.Sin_Ap_Psi[1][4];
            self.W[160] = self.Sin_Ap_Psi[2][4];
            self.W[161] = self.Sin_Ap_Psi[3][4];
            self.W[162] = self.Sin_Ap_Psi[4][4];

            self.W[163] = self.Cos_Ap_Psi[1][1];
            self.W[164] = self.Cos_Am_Psi[1][1];
            self.W[165] = self.Sin_Am_Psi[1][1];
            self.W[166] = self.Sin_Ap_Psi[0][1];
            self.W[167] = self.Sin_Ap_Psi[1][1];
            self.W[168] = self.Cos_Ap_Psi[1][1];
            self.W[169] = self.Cos_Am_Psi[1][1];
            self.W[170] = self.Sin_Am_Psi[1][1];
            self.W[171] = self.Sin_Ap_Psi[0][1];
            self.W[172] = self.Sin_Ap_Psi[1][1];

        }
        if self.PNOrder > 2{

            // Factors for H3X

            self.W[173] = self.Sin_Am_Psi[1][2];
            self.W[174] = self.Sin_Am_Psi[2][2];
            self.W[175] = self.Sin_Ap_Psi[1][2];
            self.W[176] = self.Sin_Ap_Psi[2][2];
            self.W[177] = self.Sin_Am_Psi[1][5];
            self.W[178] = self.Sin_Am_Psi[2][5];
            self.W[179] = self.Sin_Am_Psi[3][5];
            self.W[180] = self.Sin_Am_Psi[4][5];
            self.W[181] = self.Sin_Am_Psi[5][5];
            self.W[182] = self.Sin_Am_Psi[1][3];
            self.W[183] = self.Sin_Am_Psi[2][3];
            self.W[184] = self.Sin_Am_Psi[3][3];
            self.W[185] = self.Sin_Am_Psi[4][3];
            self.W[186] = self.Sin_Am_Psi[5][3];
            self.W[187] = self.Sin_Am_Psi[1][1];
            self.W[188] = self.Sin_Am_Psi[2][1];
            self.W[189] = self.Sin_Am_Psi[3][1];
            self.W[190] = self.Sin_Am_Psi[4][1];
            self.W[191] = self.Sin_Am_Psi[5][1];
            self.W[192] = self.Sin_Ap_Psi[0][1];
            self.W[193] = self.Sin_Ap_Psi[0][3];
            self.W[194] = self.Sin_Ap_Psi[1][1];
            self.W[195] = self.Sin_Ap_Psi[2][1];
            self.W[196] = self.Sin_Ap_Psi[1][3];
            self.W[197] = self.Sin_Ap_Psi[2][3];
            self.W[198] = self.Sin_Ap_Psi[3][3];
            self.W[199] = self.Sin_Ap_Psi[4][3];
            self.W[200] = self.Sin_Ap_Psi[5][3];
            self.W[201] = self.Sin_Ap_Psi[1][5];
            self.W[202] = self.Sin_Ap_Psi[2][5];
            self.W[203] = self.Sin_Ap_Psi[3][5];
            self.W[204] = self.Sin_Ap_Psi[4][5];
            self.W[205] = self.Sin_Ap_Psi[5][5];

            self.W[206] = self.Cos_Ap_Psi[0][0];
            self.W[207] = self.Cos_Ap_Psi[2][2];
            self.W[208] = self.Cos_Ap_Psi[3][2];
            self.W[209] = self.Cos_Ap_Psi[1][2];
            self.W[210] = self.Cos_Am_Psi[1][2];
            self.W[211] = self.Cos_Am_Psi[2][2];
            self.W[212] = self.Cos_Am_Psi[3][2];
            self.W[213] = self.Cos_Ap_Psi[2][0];
            self.W[214] = self.Cos_Ap_Psi[0][2];
            self.W[215] = self.Cos_Ap_Psi[3][0];
            self.W[216] = self.Cos_Ap_Psi[1][0];
            self.W[217] = self.Sin_Ap_Psi[1][0];
            self.W[218] = self.Sin_Ap_Psi[2][0];
            self.W[219] = self.Sin_Ap_Psi[3][0];
            self.W[220] = self.Sin_Am_Psi[1][2];
            self.W[221] = self.Sin_Am_Psi[2][2];
            self.W[222] = self.Sin_Am_Psi[3][2];
            self.W[223] = self.Sin_Ap_Psi[0][2];
            self.W[224] = self.Sin_Ap_Psi[1][2];
            self.W[225] = self.Sin_Ap_Psi[2][2];
            self.W[226] = self.Sin_Ap_Psi[3][2];
            self.W[227] = self.Cos_Ap_Psi[0][0];
            self.W[228] = self.Cos_Ap_Psi[2][2];
            self.W[229] = self.Cos_Ap_Psi[1][2];
            self.W[230] = self.Cos_Ap_Psi[3][2];
            self.W[231] = self.Cos_Am_Psi[1][2];
            self.W[232] = self.Cos_Am_Psi[2][2];
            self.W[233] = self.Cos_Am_Psi[3][2];
            self.W[234] = self.Cos_Ap_Psi[2][0];
            self.W[235] = self.Cos_Ap_Psi[0][2];
            self.W[236] = self.Cos_Ap_Psi[3][0];
            self.W[237] = self.Cos_Ap_Psi[1][0];
            self.W[239] = self.Sin_Ap_Psi[2][0];
            self.W[240] = self.Sin_Ap_Psi[3][0];
            self.W[241] = self.Sin_Am_Psi[1][2];
            self.W[242] = self.Sin_Am_Psi[2][2];
            self.W[243] = self.Sin_Am_Psi[3][2];
            self.W[244] = self.Sin_Ap_Psi[0][2];
            self.W[245] = self.Sin_Ap_Psi[1][2];
            self.W[246] = self.Sin_Ap_Psi[2][2];
            self.W[247] = self.Sin_Ap_Psi[3][2];

        }
        self.W










    } 
}
