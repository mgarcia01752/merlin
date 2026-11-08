# Appendix II - Example Use of CWT Sounding to Form IGs

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 151-151
Classification: Informative (explicit)

<!-- PDF_PAGE: 151 -->
Physical Layer Specification                                                  CM-SP-PHYv4.0-I07-250627


Appendix II                Example Use of CWT Sounding to Form IGs (Informative)
The FDX CMTS can use the MER measurements collected from the modems operating in the FDX band to form
IGs. An example of that procedure is provided below:
    •   The FDX CMTS asks CM to report MER twice during each sounding operation:
        First, prior to introducing CW tones, MER_pre_CW; then, after introducing CW tones and having allowed
        CM enough time to work out new MER values, MER_CW.
    •   The FDX CMTS then derives MER at CM due to CW power alone, MERCMTSderived, combining the
        two MER measurements, MER_pre_CW and MER_CW, reported by CM.
        MERCMTSderived = -10*log10(10-MER_CW/10 - 10-MER_pre_CW/10)
    •   The FDX CMTS compensates for the Kis_boost value by adjusting the CMTS calculated MER value, derived
        from the CM reported MERs, during sounding operation as follows:
        MERadjusted = MERCMTSderived - Kis_boost.
    •   The FDX CMTS compensates for the Kps_boost value by adjusting the modem reported MER values
        collected during the periodic sounding operation as follows:
        MERadjusted = MERCMTSderived - Kps_boost.




06/27/25                                            CableLabs                                           151
