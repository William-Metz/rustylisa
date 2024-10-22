//CalculateWaveFactors
use crate::wave_builder::wave_builder::Wave_Builder;

pub fn CalculateWaveFactors(wave_builder: &mut Wave_Builder) -> [f64; 248] {



    // Calculate the received wave phase

    // Calculate basic angle multiples for the phase _Psi
    wave_builder.Cos_Ap_Psi[0][1] = wave_builder.PsirDN.cos();
    wave_builder.Sin_Ap_Psi[0][1] = (wave_builder.PsirDN).sin();
    wave_builder.Cos_Ap_Psi[0][2] = wave_builder.Cos_Ap_Psi[0][1]*wave_builder.Cos_Ap_Psi[0][1] - wave_builder.Sin_Ap_Psi[0][1]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Sin_Ap_Psi[0][2]  = 2.0*wave_builder.Cos_Ap_Psi[0][1]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Cos_Ap_Psi[0][3] = wave_builder.Cos_Ap_Psi[0][2]*wave_builder.Cos_Ap_Psi[0][1] - wave_builder.Sin_Ap_Psi[0][2]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Sin_Ap_Psi[0][3]  = wave_builder.Sin_Ap_Psi[0][2]*wave_builder.Cos_Ap_Psi[0][1] + wave_builder.Cos_Ap_Psi[0][2]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Cos_Ap_Psi[0][4] = wave_builder.Cos_Ap_Psi[0][3]*wave_builder.Cos_Ap_Psi[0][1] - wave_builder.Sin_Ap_Psi[0][3]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Sin_Ap_Psi[0][4]  = wave_builder.Sin_Ap_Psi[0][3]*wave_builder.Cos_Ap_Psi[0][1] + wave_builder.Cos_Ap_Psi[0][3]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Cos_Ap_Psi[0][5] = wave_builder.Cos_Ap_Psi[0][4]*wave_builder.Cos_Ap_Psi[0][1] - wave_builder.Sin_Ap_Psi[0][4]*wave_builder.Sin_Ap_Psi[0][1];
    wave_builder.Sin_Ap_Psi[0][5]  = wave_builder.Sin_Ap_Psi[0][4]*wave_builder.Cos_Ap_Psi[0][1] + wave_builder.Cos_Ap_Psi[0][4]*wave_builder.Sin_Ap_Psi[0][1];

    // Calculate basic angle multiples for the phase α
    wave_builder.Cos_Ap_Psi[1][0] = (wave_builder.AlphaDN).cos();
    wave_builder.Sin_Ap_Psi[1][0] = (wave_builder.AlphaDN).sin();
    wave_builder.Cos_Ap_Psi[2][0] = wave_builder.Cos_Ap_Psi[1][0]*wave_builder.Cos_Ap_Psi[1][0] - wave_builder.Sin_Ap_Psi[1][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Sin_Ap_Psi[2][0]  = 2.0*wave_builder.Cos_Ap_Psi[1][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Cos_Ap_Psi[3][0] = wave_builder.Cos_Ap_Psi[2][0]*wave_builder.Cos_Ap_Psi[1][0] - wave_builder.Sin_Ap_Psi[2][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Sin_Ap_Psi[3][0]  = wave_builder.Sin_Ap_Psi[2][0]*wave_builder.Cos_Ap_Psi[1][0] + wave_builder.Cos_Ap_Psi[2][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Cos_Ap_Psi[4][0] = wave_builder.Cos_Ap_Psi[3][0]*wave_builder.Cos_Ap_Psi[1][0] - wave_builder.Sin_Ap_Psi[3][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Sin_Ap_Psi[4][0]  = wave_builder.Sin_Ap_Psi[3][0]*wave_builder.Cos_Ap_Psi[1][0] + wave_builder.Cos_Ap_Psi[3][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Cos_Ap_Psi[5][0] = wave_builder.Cos_Ap_Psi[4][0]*wave_builder.Cos_Ap_Psi[1][0] - wave_builder.Sin_Ap_Psi[4][0]*wave_builder.Sin_Ap_Psi[1][0];
    wave_builder.Sin_Ap_Psi[5][0]  = wave_builder.Sin_Ap_Psi[4][0]*wave_builder.Cos_Ap_Psi[1][0] + wave_builder.Cos_Ap_Psi[4][0]*wave_builder.Sin_Ap_Psi[1][0];

    // Now basically calculate all possible combinations and weight according to the
    // noise at a given frequency
    for k in 1..5{
        for j in 1 ..5{
            wave_builder.Cos_Ap_Psi[j][k] = (wave_builder.Cos_Ap_Psi[j][0]*wave_builder.Cos_Ap_Psi[0][k] - wave_builder.Sin_Ap_Psi[j][0]*wave_builder.Sin_Ap_Psi[0][k]);
            wave_builder.Cos_Am_Psi[j][k] = (wave_builder.Cos_Ap_Psi[j][0]*wave_builder.Cos_Ap_Psi[0][k] + wave_builder.Sin_Ap_Psi[j][0]*wave_builder.Sin_Ap_Psi[0][k]);
            wave_builder.Sin_Ap_Psi[j][k]  = (wave_builder.Sin_Ap_Psi[j][0]*wave_builder.Cos_Ap_Psi[0][k] + wave_builder.Cos_Ap_Psi[j][0]*wave_builder.Sin_Ap_Psi[0][k]);
            wave_builder.Sin_Am_Psi[j][k]  = (wave_builder.Sin_Ap_Psi[j][0]*wave_builder.Cos_Ap_Psi[0][k] - wave_builder.Cos_Ap_Psi[j][0]*wave_builder.Sin_Ap_Psi[0][k]);
        }
        wave_builder.Cos_Am_Psi[0][k] = wave_builder.Cos_Ap_Psi[0][k];
        wave_builder.Sin_Am_Psi[0][k] = -wave_builder.Sin_Ap_Psi[0][k];
    }
    // Now calculate all wavy parts
    // Factors for H0P
    wave_builder.W[0] = wave_builder.Cos_Ap_Psi[2][2] ; // cos(2α + 2_Psi];
    wave_builder.W[1] = wave_builder.Cos_Ap_Psi[1][2]  ; // cos(α + 2_Psi];
    wave_builder.W[2] = wave_builder.Cos_Am_Psi[1][2] ; // cos(α - 2_Psi];
    wave_builder.W[3] =  wave_builder.Cos_Am_Psi[2][2] ;// cos(2α - 2_Psi];
    wave_builder.W[4] = wave_builder.Cos_Ap_Psi[0][2] ; // cos(2_Psi];

    if wave_builder.PNOrder > 0 {

        // Factors for H1P
        wave_builder.W[5] = wave_builder.Cos_Ap_Psi[3][3];  // cos(3α + 3_Psi]
        wave_builder.W[6] = wave_builder.Cos_Ap_Psi[1][1];  // cos(α + _Psi]
        wave_builder.W[7] = wave_builder.Cos_Am_Psi[1][1];   // cos(α - _Psi]
        wave_builder.W[8] = wave_builder.Cos_Ap_Psi[3][1];   // cos(3α + _Psi]
        wave_builder.W[9] = wave_builder.Cos_Ap_Psi[1][3];   // cos(α + 3_Psi]
        wave_builder.W[10] = wave_builder.Cos_Am_Psi[1][3];   // cos(α - 3_Psi]
        wave_builder.W[11] = wave_builder.Cos_Am_Psi[3][1];   // cos(3α - _Psi]
        wave_builder.W[12] = wave_builder.Cos_Am_Psi[3][3];  // cos(3α - 3_Psi]
        wave_builder.W[13] = wave_builder.Cos_Ap_Psi[0][3];   // cos(3_Psi]
        wave_builder.W[14] = wave_builder.Cos_Ap_Psi[2][1];   // cos(2α + _Psi]
        wave_builder.W[15] = wave_builder.Cos_Ap_Psi[2][3];   // cos(2α + 3_Psi]
        wave_builder.W[16] = wave_builder.Cos_Am_Psi[2][1];   // cos(2α  - _Psi]
        wave_builder.W[17] = wave_builder.Cos_Am_Psi[2][3];   // cos(2α - 3_Psi]
        wave_builder.W[18] = wave_builder.Cos_Ap_Psi[0][1];  // cos(_Psi]

    }
    if wave_builder.PNOrder > 1 {

        // Factors for H2P
        wave_builder.W[19] =  wave_builder.Cos_Ap_Psi[2][2];
        wave_builder.W[20] =  wave_builder.Cos_Ap_Psi[4][4];
        wave_builder.W[21] =  wave_builder.Cos_Ap_Psi[3][4];
        wave_builder.W[22] =  wave_builder.Cos_Ap_Psi[3][2];
        wave_builder.W[23] =  wave_builder.Cos_Ap_Psi[2][4];
        wave_builder.W[24] =  wave_builder.Cos_Ap_Psi[4][2];
        wave_builder.W[25] =  wave_builder.Cos_Ap_Psi[1][4];
        wave_builder.W[26] =  wave_builder.Cos_Am_Psi[1][2];
        wave_builder.W[27] =  wave_builder.Cos_Am_Psi[2][2];
        wave_builder.W[28] =  wave_builder.Cos_Am_Psi[1][4];
        wave_builder.W[29] =  wave_builder.Cos_Am_Psi[3][2];
        wave_builder.W[30] =  wave_builder.Cos_Am_Psi[2][4];
        wave_builder.W[31] =  wave_builder.Cos_Am_Psi[4][2];
        wave_builder.W[32] =   wave_builder.Cos_Am_Psi[3][4];
        wave_builder.W[33] =   wave_builder.Cos_Am_Psi[4][4];
        wave_builder.W[34] =   wave_builder.Cos_Ap_Psi[0][2];
        wave_builder.W[35] =   wave_builder.Cos_Ap_Psi[0][4];
        wave_builder.W[36] =   wave_builder.Cos_Ap_Psi[1][2];

        wave_builder.W[37] =   wave_builder.Cos_Ap_Psi[1][1];
        wave_builder.W[38] =   wave_builder.Cos_Am_Psi[1][1];
        wave_builder.W[39] =   wave_builder.Sin_Am_Psi[1][1];
        wave_builder.W[40] =   wave_builder.Sin_Ap_Psi[0][1];
        wave_builder.W[41] =   wave_builder.Sin_Ap_Psi[1][1];
        wave_builder.W[42] =   wave_builder.Cos_Ap_Psi[1][1];
        wave_builder.W[43] =   wave_builder.Cos_Am_Psi[1][1];
        wave_builder.W[44] =   wave_builder.Sin_Am_Psi[1][1];
        wave_builder.W[45] =   wave_builder.Sin_Ap_Psi[0][1];
        wave_builder.W[46] =   wave_builder.Sin_Ap_Psi[1][1];

    }

    if wave_builder.PNOrder > 2 {

        // Factors for H3P

        wave_builder.W[47] =   wave_builder.Cos_Ap_Psi[2][2];
        wave_builder.W[48] =   wave_builder.Cos_Ap_Psi[1][2];
        wave_builder.W[49] =   wave_builder.Cos_Am_Psi[1][2];
        wave_builder.W[50] =   wave_builder.Cos_Am_Psi[2][2];
        wave_builder.W[51] =   wave_builder.Cos_Ap_Psi[0][2];
        wave_builder.W[52] =   wave_builder.Cos_Ap_Psi[5][5];
        wave_builder.W[53] =   wave_builder.Cos_Ap_Psi[1][1];
        wave_builder.W[54] =   wave_builder.Cos_Ap_Psi[3][3];
        wave_builder.W[55] =   wave_builder.Cos_Ap_Psi[4][5];
        wave_builder.W[56] =   wave_builder.Cos_Ap_Psi[4][3];
        wave_builder.W[57] =   wave_builder.Cos_Ap_Psi[5][3];
        wave_builder.W[58] =   wave_builder.Cos_Am_Psi[1][1];
        wave_builder.W[59] =   wave_builder.Cos_Ap_Psi[3][1];
        wave_builder.W[60] =   wave_builder.Cos_Ap_Psi[3][5];
        wave_builder.W[61] =   wave_builder.Cos_Ap_Psi[1][3];
        wave_builder.W[62] =   wave_builder.Cos_Ap_Psi[2][5];
        wave_builder.W[63] =   wave_builder.Cos_Ap_Psi[4][1];
        wave_builder.W[64] =   wave_builder.Cos_Ap_Psi[5][1];
        wave_builder.W[65] =   wave_builder.Cos_Am_Psi[3][1];
        wave_builder.W[66] =   wave_builder.Cos_Ap_Psi[1][5];
        wave_builder.W[67] =   wave_builder.Cos_Am_Psi[1][3];
        wave_builder.W[68] =   wave_builder.Cos_Am_Psi[4][1];
        wave_builder.W[69] =   wave_builder.Cos_Am_Psi[5][1];
        wave_builder.W[70] =   wave_builder.Cos_Am_Psi[3][3];
        wave_builder.W[71] =   wave_builder.Cos_Am_Psi[1][5];
        wave_builder.W[72] =   wave_builder.Cos_Am_Psi[2][5];
        wave_builder.W[73] =   wave_builder.Cos_Am_Psi[4][3];
        wave_builder.W[74] =   wave_builder.Cos_Am_Psi[5][3];
        wave_builder.W[75] =   wave_builder.Cos_Am_Psi[3][5];
        wave_builder.W[76] =   wave_builder.Cos_Am_Psi[4][5];
        wave_builder.W[77] =   wave_builder.Cos_Am_Psi[5][5];
        wave_builder.W[78] =   wave_builder.Cos_Ap_Psi[0][3];
        wave_builder.W[79] =   wave_builder.Cos_Ap_Psi[0][5];
        wave_builder.W[80] =   wave_builder.Cos_Ap_Psi[2][3];
        wave_builder.W[81] =   wave_builder.Cos_Am_Psi[2][3];
        wave_builder.W[82] =   wave_builder.Cos_Ap_Psi[2][1];
        wave_builder.W[83] =   wave_builder.Cos_Am_Psi[2][1];
        wave_builder.W[84] =   wave_builder.Cos_Ap_Psi[0][1];

        wave_builder.W[85] =   wave_builder.Cos_Ap_Psi[0][0];
        wave_builder.W[86] =   wave_builder.Cos_Ap_Psi[2][2];
        wave_builder.W[87] =   wave_builder.Cos_Ap_Psi[3][2];
        wave_builder.W[88] =   wave_builder.Cos_Am_Psi[3][2];
        wave_builder.W[89] =   wave_builder.Cos_Ap_Psi[1][2];
        wave_builder.W[90] =   wave_builder.Cos_Am_Psi[1][2];
        wave_builder.W[91] =   wave_builder.Cos_Am_Psi[2][2];
        wave_builder.W[92] =   wave_builder.Cos_Ap_Psi[0][0];
        wave_builder.W[93] =   wave_builder.Cos_Ap_Psi[3][0];
        wave_builder.W[94] =   wave_builder.Cos_Ap_Psi[0][2];
        wave_builder.W[95] =   wave_builder.Cos_Ap_Psi[2][0];
        wave_builder.W[96] =   wave_builder.Cos_Ap_Psi[1][0];
        wave_builder.W[97] =   wave_builder.Sin_Ap_Psi[1][0];
        wave_builder.W[98] =   wave_builder.Sin_Ap_Psi[2][0];
        wave_builder.W[99] =   wave_builder.Sin_Ap_Psi[3][0];
        wave_builder.W[100] =   wave_builder.Sin_Am_Psi[1][2];
        wave_builder.W[101] =   wave_builder.Sin_Am_Psi[2][2];
        wave_builder.W[102] =   wave_builder.Sin_Am_Psi[3][2];
        wave_builder.W[103] =   wave_builder.Sin_Ap_Psi[0][2];
        wave_builder.W[104] =   wave_builder.Sin_Ap_Psi[1][2];
        wave_builder.W[105] =   wave_builder.Sin_Ap_Psi[2][2];
        wave_builder.W[106] =   wave_builder.Sin_Ap_Psi[3][2];
        wave_builder.W[107] =   wave_builder.Cos_Ap_Psi[0][0];
        wave_builder.W[108] =   wave_builder.Cos_Ap_Psi[2][2];
        wave_builder.W[109] =   wave_builder.Cos_Ap_Psi[3][2];
        wave_builder.W[110] =   wave_builder.Cos_Am_Psi[3][2];
        wave_builder.W[111] =   wave_builder.Cos_Ap_Psi[1][2];
        wave_builder.W[112] =   wave_builder.Cos_Am_Psi[1][2];
        wave_builder.W[113] =   wave_builder.Cos_Am_Psi[2][2];
        wave_builder.W[114] =   wave_builder.Cos_Ap_Psi[0][0];
        wave_builder.W[115] =   wave_builder.Cos_Ap_Psi[3][0];
        wave_builder.W[116] =   wave_builder.Cos_Ap_Psi[0][2];
        wave_builder.W[117] =   wave_builder.Cos_Ap_Psi[2][0];
        wave_builder.W[118] =   wave_builder.Cos_Ap_Psi[1][0];
        wave_builder.W[119] =   wave_builder.Sin_Ap_Psi[1][0];
        wave_builder.W[120] =   wave_builder.Sin_Ap_Psi[2][0];
        wave_builder.W[121] =   wave_builder.Sin_Ap_Psi[3][0];
        wave_builder.W[122] =   wave_builder.Sin_Am_Psi[1][2];
        wave_builder.W[123] =   wave_builder.Sin_Am_Psi[2][2];
        wave_builder.W[124] =   wave_builder.Sin_Am_Psi[3][2];
        wave_builder.W[125] =   wave_builder.Sin_Ap_Psi[0][2];
        wave_builder.W[126] =   wave_builder.Sin_Ap_Psi[1][2];
        wave_builder.W[127] =   wave_builder.Sin_Ap_Psi[2][2];
        wave_builder.W[128] =   wave_builder.Sin_Ap_Psi[3][2];

    }
    // Factors for H0X
    wave_builder.W[129] = wave_builder.Sin_Am_Psi[1][2];
    wave_builder.W[130] = wave_builder.Sin_Am_Psi[2][2];
    wave_builder.W[131] = wave_builder.Sin_Ap_Psi[1][2];
    wave_builder.W[132] = wave_builder.Sin_Ap_Psi[2][2];

    if wave_builder.PNOrder > 0 {

        // Factors for H1X
        wave_builder.W[133] = wave_builder.Sin_Am_Psi[1][3];  // sin(α  - 3_Psi]
        wave_builder.W[134] = wave_builder.Sin_Am_Psi[2][3];  // sin(2α - 3_Psi]
        wave_builder.W[135] = wave_builder.Sin_Am_Psi[3][3];  // sin(3α - 3_Psi]
        wave_builder.W[136]= wave_builder.Sin_Am_Psi[1][1];    // sin(α - _Psi]
        wave_builder.W[137] = wave_builder.Sin_Am_Psi[2][1];   // sin(2α - _Psi]
        wave_builder.W[138] = wave_builder.Sin_Am_Psi[3][1];   // sin(3α - _Psi]
        wave_builder.W[139] = wave_builder.Sin_Ap_Psi[0][1];   // sin(_Psi]
        wave_builder.W[140] = wave_builder.Sin_Ap_Psi[1][1];   // sin(α + _Psi]
        wave_builder.W[141] = wave_builder.Sin_Ap_Psi[2][1];   // sin(2α + _Psi]
        wave_builder.W[142] = wave_builder.Sin_Ap_Psi[3][1];  // sin(3α + _Psi]
        wave_builder.W[143] = wave_builder.Sin_Ap_Psi[1][3];  // sin(α + 3_Psi]
        wave_builder.W[144] = wave_builder.Sin_Ap_Psi[2][3]; // sin(2α + 3_Psi]
        wave_builder.W[145] = wave_builder.Sin_Ap_Psi[3][3]; // sin(3α + 3_Psi]

    }
    if wave_builder.PNOrder > 1 {

        // Factors for H2X
        wave_builder.W[146] = wave_builder.Sin_Am_Psi[1][4];
        wave_builder.W[147] = wave_builder.Sin_Am_Psi[2][4];
        wave_builder.W[148] = wave_builder.Sin_Am_Psi[3][4];
        wave_builder.W[149] = wave_builder.Sin_Am_Psi[4][4];
        wave_builder.W[150] = wave_builder.Sin_Am_Psi[1][2];
        wave_builder.W[151] = wave_builder.Sin_Am_Psi[2][2];
        wave_builder.W[152] = wave_builder.Sin_Am_Psi[3][2];
        wave_builder.W[153] = wave_builder.Sin_Am_Psi[4][2];
        wave_builder.W[154] = wave_builder.Sin_Ap_Psi[0][2];
        wave_builder.W[155] = wave_builder.Sin_Ap_Psi[1][2];
        wave_builder.W[156] = wave_builder.Sin_Ap_Psi[2][2];
        wave_builder.W[157] = wave_builder.Sin_Ap_Psi[3][2];
        wave_builder.W[158] = wave_builder.Sin_Ap_Psi[4][2];
        wave_builder.W[159] = wave_builder.Sin_Ap_Psi[1][4];
        wave_builder.W[160] = wave_builder.Sin_Ap_Psi[2][4];
        wave_builder.W[161] = wave_builder.Sin_Ap_Psi[3][4];
        wave_builder.W[162] = wave_builder.Sin_Ap_Psi[4][4];

        wave_builder.W[163] = wave_builder.Cos_Ap_Psi[1][1];
        wave_builder.W[164] = wave_builder.Cos_Am_Psi[1][1];
        wave_builder.W[165] = wave_builder.Sin_Am_Psi[1][1];
        wave_builder.W[166] = wave_builder.Sin_Ap_Psi[0][1];
        wave_builder.W[167] = wave_builder.Sin_Ap_Psi[1][1];
        wave_builder.W[168] = wave_builder.Cos_Ap_Psi[1][1];
        wave_builder.W[169] = wave_builder.Cos_Am_Psi[1][1];
        wave_builder.W[170] = wave_builder.Sin_Am_Psi[1][1];
        wave_builder.W[171] = wave_builder.Sin_Ap_Psi[0][1];
        wave_builder.W[172] = wave_builder.Sin_Ap_Psi[1][1];

    }
    if wave_builder.PNOrder > 2{

        // Factors for H3X

        wave_builder.W[173] = wave_builder.Sin_Am_Psi[1][2];
        wave_builder.W[174] = wave_builder.Sin_Am_Psi[2][2];
        wave_builder.W[175] = wave_builder.Sin_Ap_Psi[1][2];
        wave_builder.W[176] = wave_builder.Sin_Ap_Psi[2][2];
        wave_builder.W[177] = wave_builder.Sin_Am_Psi[1][5];
        wave_builder.W[178] = wave_builder.Sin_Am_Psi[2][5];
        wave_builder.W[179] = wave_builder.Sin_Am_Psi[3][5];
        wave_builder.W[180] = wave_builder.Sin_Am_Psi[4][5];
        wave_builder.W[181] = wave_builder.Sin_Am_Psi[5][5];
        wave_builder.W[182] = wave_builder.Sin_Am_Psi[1][3];
        wave_builder.W[183] = wave_builder.Sin_Am_Psi[2][3];
        wave_builder.W[184] = wave_builder.Sin_Am_Psi[3][3];
        wave_builder.W[185] = wave_builder.Sin_Am_Psi[4][3];
        wave_builder.W[186] = wave_builder.Sin_Am_Psi[5][3];
        wave_builder.W[187] = wave_builder.Sin_Am_Psi[1][1];
        wave_builder.W[188] = wave_builder.Sin_Am_Psi[2][1];
        wave_builder.W[189] = wave_builder.Sin_Am_Psi[3][1];
        wave_builder.W[190] = wave_builder.Sin_Am_Psi[4][1];
        wave_builder.W[191] = wave_builder.Sin_Am_Psi[5][1];
        wave_builder.W[192] = wave_builder.Sin_Ap_Psi[0][1];
        wave_builder.W[193] = wave_builder.Sin_Ap_Psi[0][3];
        wave_builder.W[194] = wave_builder.Sin_Ap_Psi[1][1];
        wave_builder.W[195] = wave_builder.Sin_Ap_Psi[2][1];
        wave_builder.W[196] = wave_builder.Sin_Ap_Psi[1][3];
        wave_builder.W[197] = wave_builder.Sin_Ap_Psi[2][3];
        wave_builder.W[198] = wave_builder.Sin_Ap_Psi[3][3];
        wave_builder.W[199] = wave_builder.Sin_Ap_Psi[4][3];
        wave_builder.W[200] = wave_builder.Sin_Ap_Psi[5][3];
        wave_builder.W[201] = wave_builder.Sin_Ap_Psi[1][5];
        wave_builder.W[202] = wave_builder.Sin_Ap_Psi[2][5];
        wave_builder.W[203] = wave_builder.Sin_Ap_Psi[3][5];
        wave_builder.W[204] = wave_builder.Sin_Ap_Psi[4][5];
        wave_builder.W[205] = wave_builder.Sin_Ap_Psi[5][5];

        wave_builder.W[206] = wave_builder.Cos_Ap_Psi[0][0];
        wave_builder.W[207] = wave_builder.Cos_Ap_Psi[2][2];
        wave_builder.W[208] = wave_builder.Cos_Ap_Psi[3][2];
        wave_builder.W[209] = wave_builder.Cos_Ap_Psi[1][2];
        wave_builder.W[210] = wave_builder.Cos_Am_Psi[1][2];
        wave_builder.W[211] = wave_builder.Cos_Am_Psi[2][2];
        wave_builder.W[212] = wave_builder.Cos_Am_Psi[3][2];
        wave_builder.W[213] = wave_builder.Cos_Ap_Psi[2][0];
        wave_builder.W[214] = wave_builder.Cos_Ap_Psi[0][2];
        wave_builder.W[215] = wave_builder.Cos_Ap_Psi[3][0];
        wave_builder.W[216] = wave_builder.Cos_Ap_Psi[1][0];
        wave_builder.W[217] = wave_builder.Sin_Ap_Psi[1][0];
        wave_builder.W[218] = wave_builder.Sin_Ap_Psi[2][0];
        wave_builder.W[219] = wave_builder.Sin_Ap_Psi[3][0];
        wave_builder.W[220] = wave_builder.Sin_Am_Psi[1][2];
        wave_builder.W[221] = wave_builder.Sin_Am_Psi[2][2];
        wave_builder.W[222] = wave_builder.Sin_Am_Psi[3][2];
        wave_builder.W[223] = wave_builder.Sin_Ap_Psi[0][2];
        wave_builder.W[224] = wave_builder.Sin_Ap_Psi[1][2];
        wave_builder.W[225] = wave_builder.Sin_Ap_Psi[2][2];
        wave_builder.W[226] = wave_builder.Sin_Ap_Psi[3][2];
        wave_builder.W[227] = wave_builder.Cos_Ap_Psi[0][0];
        wave_builder.W[228] = wave_builder.Cos_Ap_Psi[2][2];
        wave_builder.W[229] = wave_builder.Cos_Ap_Psi[1][2];
        wave_builder.W[230] = wave_builder.Cos_Ap_Psi[3][2];
        wave_builder.W[231] = wave_builder.Cos_Am_Psi[1][2];
        wave_builder.W[232] = wave_builder.Cos_Am_Psi[2][2];
        wave_builder.W[233] = wave_builder.Cos_Am_Psi[3][2];
        wave_builder.W[234] = wave_builder.Cos_Ap_Psi[2][0];
        wave_builder.W[235] = wave_builder.Cos_Ap_Psi[0][2];
        wave_builder.W[236] = wave_builder.Cos_Ap_Psi[3][0];
        wave_builder.W[237] = wave_builder.Cos_Ap_Psi[1][0];
        wave_builder.W[239] = wave_builder.Sin_Ap_Psi[2][0];
        wave_builder.W[240] = wave_builder.Sin_Ap_Psi[3][0];
        wave_builder.W[241] = wave_builder.Sin_Am_Psi[1][2];
        wave_builder.W[242] = wave_builder.Sin_Am_Psi[2][2];
        wave_builder.W[243] = wave_builder.Sin_Am_Psi[3][2];
        wave_builder.W[244] = wave_builder.Sin_Ap_Psi[0][2];
        wave_builder.W[245] = wave_builder.Sin_Ap_Psi[1][2];
        wave_builder.W[246] = wave_builder.Sin_Ap_Psi[2][2];
        wave_builder.W[247] = wave_builder.Sin_Ap_Psi[3][2];

    }
    wave_builder.W










}
