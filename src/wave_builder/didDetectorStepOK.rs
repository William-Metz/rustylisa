use crate::test_case::test_case::TestCase;
pub fn didStepOK(step_num: u8, test_case: &TestCase) -> Boolean{  


    if (step_num + 2)*Δτ > test_case.tau_c {
        False
    }
    // Otherwise, check if we can get data from the spin evolver
    let spinData As SpinDataClass = SpinEvolver.GetSpinDataAtTime(StepNumber*Δτ)//SpinEvolver
    If spinData = Nil Then Return False  // If the method returns nothing, coalescence must have happened

    // We have data, so
    VDN = spinData.V // get the current speed
    // If our speed is half that of light, our approximations are breaking down, so bail out
    If VDN > 0.5 Then Return False

    // Otherwise, load the rest of the data coming from the spin evolver
    ιDN = spinData.ι
    αDN = spinData.α
    χaxDN = spinData.χax
    χayDN = spinData.χay
    χazDN = spinData.χaz
    χsxDN = spinData.χsx
    χsyDN = spinData.χsy
    χszDN = spinData.χsz

    // Calculate the wave phase
    Var τrm As Double = (StepNumber - 0.5)*Δτr
    ΨrDN = spinData.Ψ
    // do the following instead of the above if we want the data in the orbiting LISA frame
    // ΨrDN = ΨrDP + (1.0 + Parameters.Ve*Sin(Parameters.Θ)*Sin(Parameters.GMΩe*τrm - Parameters.Φ))*(spinData.Ψ - ΨP)
    ΨrDP = ΨrDN
    ΨP = spinData.Ψ
    τrDN = StepNumber*Δτr

    // Calculate the wave
    CalculateAmplitudes
    CalculateWaveFactors
    SumSourceH(W)

    // Write out useful information for plotting (if this is not a case from a file)
    ////SaveDataForPlotting(τrDN)

    // We have completed the detector step successfully

    True
}
