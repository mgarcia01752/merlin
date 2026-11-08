# 7.2.3 - FDX OFDM Numerology

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 52-53
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 52 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


7.2.2.5.5.1   Operation of FDX CMs and FDX-L CMs Supporting CFDX Operation in a High-Split Network
A high-split network is defined as an HFC network where legacy DOCSIS technologies exist, and the upstream
band extends from 5 MHz to 204 MHz. Given that the FDX allocated spectrum always starts at 108 MHz, then the
operation of an FDX DOCSIS system with CFDX-capable devices in a high-split network will entail some overlap
between the legacy upstream band and the FDX allocated spectrum.
In such a situation, certain precautions need to be taken into consideration to make sure that the upstream bursts in
the 108 MHz to 204 MHz band from DOCSIS devices (FDX-L cable modems supporting CFDX) do not interfere
with the downstream FDX channel operating in the same band. This is accomplished by having the FDX-L cable
modems participate in the channel sounding procedure as described in Section 7.2.6.
In the above described scenario, the upstream band for FDX-L cable modems extends from 5 MHz to 204 MHz.
For an FDX CM that supports CFDX and that is operating in a 5 MHz to 204 MHz network, the legacy upstream
channels for the FDX CM operate in the 5 MHz to 85 MHz band. The 85 MHz to 108 MHz band is a transition
region that is not used by the FDX CM in the CFDX plan to transmit or receive DOCSIS signals. Upstream FDX
channels in the CFDX frequency plan operate in the FDX allocated spectrum starting at 108 MHz.

7.2.2.6    Operation in the FDX Band in Non-Allocated Spectrum
Non-allocated spectrum is the spectrum within the FDX band between the highest frequency of the FDX allocated
spectrum and the uppermost frequency of the FDX band (684 MHz for CFDX or 834 MHz for SFDX).
An FDX node MUST be able to place either SC-QAM or non-FDX OFDM channels in the non-allocated spectrum
of the FDX band with RF performance requirements specified in this section.
An FDX CM in FDX mode is not required to receive either SC-QAM or non-FDX OFDM channels in the non-
allocated spectrum of the FDX band.
There are no US channels in the non-allocated spectrum of the FDX band.

7.2.2.7    Operation in the FDX Transition Band
The FDX transition band is the spectrum between 684 MHz and 804 MHz in the CFDX frequency plan or between
834 MHz and 984 MHz in the SFDX frequency plan.
An FDX node MUST be able to place either SC-QAM or non-FDX OFDM channels in the FDX transition band
with RF performance requirements specified in this section.
An FDX CM in FDX mode SHOULD be able to receive either SC-QAM or non-FDX OFDM channels in this band.
It is possible that performance in this band could be impaired compared to a compliant DOCSIS 3.1 CM. As such,
performance in the FDX transition band for an FDX CM operating in FDX mode is not specified.

7.2.2.8    Operation Above the FDX Transition Band
The FDX transition band upper band edge is defined as 804 MHz for CFDX operation or 984 MHz for SFDX
operation.
An FDX node MUST be able to place either SC-QAM or non-FDX OFDM channels in the spectrum above the FDX
transition band upper band edge with RF performance requirements specified in this section.
An FDX CM operating in FDX mode MUST be able to receive either SC-QAM or non-FDX OFDM channels above
the FDX transition band upper band edge. FDX CM CNR and minimum PSD performance above the FDX transition
band testing are defined in Section 7.2.5.12.5.

7.2.3     FDX OFDM Numerology
See [DOCSIS PHYv3.1], section 7.3.




52                                                  CableLabs                                              06/27/25
<!-- PDF_PAGE: 53 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


7.2.4     Upstream Transmit and Receive

7.2.4.1     Signal Processing Requirements
See [DOCSIS PHYv3.1], section 7.4.1.

7.2.4.2     Time and Frequency Synchronization
See [DOCSIS PHYv3.1], section 7.4.2.

7.2.4.2.1    Channel Frequency Accuracy
This section augments section 7.4.2.1 of [DOCSIS PHYv3.1].
The frequency of the upstream subcarrier clock (or upstream subcarrier spacing) is required to be accurate within
0.4 ppm and each subcarrier frequency accurate within 30 Hz, both relative to the master clock reference, and both
for five sigma of the upstream OFDMA transmissions, for subcarrier frequencies up to 684 MHz for CFDX
operation or 834 MHz for SFDX operation. The measurements of the frequency of the upstream subcarrier clock,
and the subcarrier frequencies, are averaged over the duration of an upstream single-frame grant. A constant
temperature is maintained during the measurements within a range of 20 ºC ± 2 ºC. A minimum warm-up time of
30 minutes occurs before the CM frequency measurements are made.

7.2.4.2.2    Channel Timing Accuracy
See [DOCSIS PHYv3.1], section 7.4.2.2.

7.2.4.2.3    Modulation Timing Jitter
See [DOCSIS PHYv3.1], section 7.4.2.3.

7.2.4.3     Forward Error Correction
See [DOCSIS PHYv3.1], section 7.4.3.

7.2.4.4     Data Randomization
See [DOCSIS PHYv3.1], section 7.4.4.

7.2.4.5     Time and Frequency Interleaving and De-interleaving
See [DOCSIS PHYv3.1], section 7.4.5.

7.2.4.6     Mapping of Bits to Cell Words
See [DOCSIS PHYv3.1], section 7.4.6.

7.2.4.7     Mapping and Demapping Bits to/from QAM Subcarriers
See [DOCSIS PHYv3.1], section 7.4.7.

7.2.4.8     REQ Messages
See [DOCSIS PHYv3.1], section 7.4.8.

7.2.4.9     IDFT
See [DOCSIS PHYv3.1], section 7.4.9.

7.2.4.10 Cyclic Prefix and Windowing
See [DOCSIS PHYv3.1], section 7.4.10.




06/27/25                                           CableLabs                                                   53
