# Appendix I - ICI from CWT Sounding Signal

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 149-150
Classification: Informative (explicit)

<!-- PDF_PAGE: 149 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


Appendix I                 ICI from CWT Sounding Signal (Informative)
I.1          ICI Caused by Sounding Signal on Adjacent US Subcarriers
Given that the sounding CW does not coincide exactly on OFDM subcarrier center frequency (FFT grid points), the
sounding signal CW will cause inter-carrier interferences (ICI) to the adjacent subcarriers on US.
To minimize the impact of ICI caused by the sounding signal, one may need to adjust the profiles on the adjacent
subcarriers, if they are active subcarriers, to ensure that their normal operations will not be affected. The actual ICI
depends on the sounding power and the frequency offset value selected during the sounding procedure. As all noise
is additive, the effective noise floor N_eff will be 10*log10(10^(-N_ICI/10)+10^(-N0/10)); where N_ICI is the noise
floor contributed by ICI and N0 is the original noise floor without the sounding signal.
For example, in the case of 25 kHz subcarrier spacing, if the sounding CWs are transmitted with 10 dB power boost
and a 150 Hz frequency (the worst case), then the ICI to the closest subcarrier and adjacent subcarriers will be 35 dB
and 40 dB below their nominal US received signal (tone power), limiting their SNR to 35 dB and 40 dB,
respectively. Once N_eff is calculated, the corresponding SNR and QAM orders can be derived.
As the power boosting and frequency offset values are field configurable, the QAM orders on the closest and
adjacent subcarriers vary. To simplify the operation and be conservative, one could set their QAM orders always
based on the worst case (ICI is 35 dB and 40 dB, respectively, for 25 kHz carrier spacing, and 40 dB and 45 dB,
respectively, for 50 kHz carrier spacing).
The ICI on 3rd adjacent subcarrier is 55 dB and 60 dB for 25 kHz and 50 kHz carrier spacing, respectively. Even
with the max 10 dB sounding power boosting, the noise contribution by the ICI will be 45 dB and 55 dB below the
nominal US power. In addition, the marginal adverse ICI effects on 3rd subcarrier can be mitigated by the frequency
interleaving. Thus, the effects of ICI on the 3rd adjacent subcarrier and beyond can be ignored.
The maximum number of sounding signals is 255 per US OFDMA channel. The ICI contributions from adjacent
sounding signals, other than the closest one, can be ignored.

I.2          ICI Caused by Sounding Signal on Adjacent DS Subcarriers
The impact of ICI caused by the sounding signal on adjacent subcarriers on the DS can be evaluated in a similar way
as US. As the CM is not able to send a sounding signal and receive the DS signal at the same time, the impacts of
ICI caused by sounding signal are on the DS of neighboring CMs.
The sounding signal received by the neighboring CM could be as high as 7 dB above the nominal DS received
signal level (tone power). With 10 dB sounding power boosting, the max sounding signal received by the
neighboring CM will be 17 dB above the nominal DS received signal level.
To mitigate the impact of the ICI on DS caused by the sounding signal, one may need to adjust the profiles on the
adjacent subcarriers to ensure that their normal operations will not be affected. The actual ICI depends on the
sounding power and the frequency offset value selected during the sounding procedure.
Refer to Table 47, where the ICI is listed for 10 direct adjacent subcarriers with 25 kHz carrier spacing and 150 Hz
sounding signal frequency offset (worst case). When the sounding signal level is 17 dB above the nominal DS signal
level, the noise contribution from ICI on the first adjacent subcarrier will be 27 dB below the DS nominal signal
level. Similar to US, one needs to add up all the noises to compute the effective noise floor from which the effect
SNR and QAM order will be derived. The ICI is about 47 dB below the nominal DS signal level on 10th adjacent
subcarrier for the worst case (10 dB sounding signal boosting, 25 kHz carrier spacing and 150 Hz sounding signal
frequency offset), which is 6 dB below the noise floor for supporting the max 41 dB SNR required for DS. The
impact of the ICI on 11th subcarrier and beyond can be ignored.




06/27/25                                             CableLabs                                                    149
<!-- PDF_PAGE: 150 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


                                        Table 47 - Inter-Carrier Interference

                                          Inter-Carrier Interference (dB)
                                    (25 kHz carrier spacing, 150 Hz freq offset)
            Adjacent            Sounding Signal Equals DS            Sounding Signal 17 dB Above DS
           Carrier Index         Signal of Neighboring CM               Signal of Neighboring CM
                  0                           0.0                                    17.0
                  1                          -44.4                                   -27.4
                  2                          -50.4                                   -33.4
                  3                          -54.4                                   -37.0
                  4                          -56.4                                   -39.5
                  5                          -58.4                                   -41.4
                  6                          -60.0                                   -43.0
                  7                          -61.3                                   -44.3
                  8                          -62.5                                   -45.5
                  9                          -63.5                                   -46.5
                 10                          -64.4                                   -47.4


The maximum number of sounding signals is 254 per US OFDMA channel. So, the minimum interval between
sounding signal is 8 subcarriers. If one needs to consider the ICI up to 10 subcarriers, when 254 sounding signals are
activated on each DS subcarrier, the ICI will be contributed by up to three adjacent sounding signals. Most likely,
the adjacent sounding signals come from different CMs, and their ICI will be additive. In this case, the effective
noise floor N_eff on the victim DS subcarrier will be 10*log10(10(-N_ICI_a/10) +10(-N_ICI_b/10) +10(-
N_ICI_c/10)+10(-N0/10)); where N_ICI_a, N_ICI_b and N_ICI_c are the noise floors contributed by the ICI of
three sounding signals (signals a, b, and c) that are separate from the victim DS subcarrier by less than 10
subcarriers; and N0 is the original noise floor without any sounding signals. Once N_eff is calculated, the
corresponding SNR and QAM orders will be derived.
The ICI contributions from fourth sounding signal and beyond can be ignored.
For channel sounding purposes, multiple subcarriers are assigned to be sounding subcarriers where modems can
transmit continuous wave (CW) signals and other modems receive and measure those CW signals. By knowing the
transmit and receive levels of the sounding subcarriers, the RF isolation between CMs as a function of frequency can
be calculated.
The sounding subcarriers cannot be PLC, NCP, continuous pilot, or exclusion subcarriers. The CMTS sets the
modulation of DS data subcarriers with the same frequency as sounding CWs to "zero bit-loading" for the duration
of the sounding test. In the sounding test, one or more CMs transmit the sounding CWs with a small frequency
offset from the subcarrier center frequency of the DS channel. For any CM that is to receive and measure the
sounding CWs, the direction of the channel that is being sounded is to be switched to DS direction from the view
point of that CM.
CMs will only measure the SNR of a sounding CW when the sounding subcarrier coincides with the location of a
scattered pilot. Note that FDX-L CMs can use FDX channels as DS channels only. Therefore, they participate in the
sounding process in listen-only mode. FDX CMs can use FDX channels for either direction.




150                                                  CableLabs                                            06/27/25
