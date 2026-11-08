# 7.2.6 - FDX Sounding

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 94-98
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 94 -->
CM-SP-PHYv4.0-I07-250627                                          Data-Over-Cable Service Interface Specifications


     •    Fully loaded spectrum is defined as downstream lower band edge to 1218 MHz with a mix of FDX OFDM
          channels, SC-QAM channels and non-FDX OFDM channels
     •    No analog channels
     •    DS PSD at the CM is flat across the spectrum
     •    Table 25 updates the column headings and frequency ranges of table 46 of [DOCSIS PHYv3.1].
                 Table 25 - FDX CM in FDX Mode Minimum CNR Performance in AWGN Channel

                                         CNR1,2 (dB)
                                       FDX Downstream
                                      Lower Band Edge to                CNR1,2 (dB)
          Constellation                   1002 MHz                 1002 MHz to 1218 MHz                 Min P6AVG dBmV
               4096                             41.0                          41.5                               -6
               2048                             37.0                          37.5                               -9
               1024                             34.0                          34.0                              -12
               512                              30.5                          30.5                              -12
               256                              27.0                          27.0                              -15
               128                              24.0                          24.0                              -15
                64                              21.0                          21.0                              -15
                16                              15.0                          15.0                              -15
TABLE NOTES
NOTE 1 CNR is defined here as total signal power in occupied bandwidth divided by total noise in occupied bandwidth.
NOTE 2 Channel CNR is adjusted to the required level by measuring the source inband noise including phase noise component along
       with transmitter noise and distortion and adding the required delta noise from an external AWGN generator to achieve the
       desired CNR at the CM F-connector.
NOTE 3 Applicable to an OFDM channel with 192 MHz of occupied bandwidth.



7.2.5.12.5.2 Above FDX Transition Band Test for Downstream up to 1794 MHz
The requirements of this section apply to FDX CMs that support downstream operation up to 1794 MHz when the
cable plant or operator configuration includes downstream signals above 1218 MHz and up to 1794 MHz.
An FDX CM that supports 1794 MHz downstream operation MUST adhere to the requirements of Table 46 - CM
Minimum CNR Performance1 in AWGN Channel for 1794 MHz Input Signal Upper Frequency Limit for reception
at frequencies above the FDX transition band upper band edge, under the test conditions described in the
accompanying text of section 7.3.5.12.1.

7.2.5.13 Physical Layer Link Channel (PLC)
See [DOCSIS PHYv3.1], section 7.5.13.

7.2.5.14 Next Codeword Pointer
See [DOCSIS PHYv3.1], section 7.5.14.

7.2.6     Sounding
Channel sounding is required to assign cable modems (CMs) to Interference Groups. This is achieved by making a
set of CMs (referred to in this section as the Test CMs) transmit upstream signals and making other CMs in the
network (referred to as Measurer CMs) measure MER of subcarriers.
There are two sounding methods:
     1.   OUDP Method
     2.   CW Tone (CWT) Method


94                                                     CableLabs                                                     06/27/25
<!-- PDF_PAGE: 95 -->
Physical Layer Specification                                                     CM-SP-PHYv4.0-I07-250627


In the OUDP method the Test CMs may be FDX-L or FDX, but the Measurer CMs have to be FDX CM. The
transmitted signals are made up of sequences of OFDMA frames occupying the modulated bandwidth of the
upstream FDX channel. The Measurer CMs measure the MER of all the subcarriers covered by the OFDMA frames.
In the CWT method, the Test CMs have to be FDX, but the Measurer CMs can be either FDX-L or FDX CMs. The
transmitted signals consist of a sequences CW tones. The receiving CMs measure the MER at subcarrier frequencies
corresponding to the CWTs.
An FDX CM can transmit or receive on any FDX DS/US channel, while an FDX-L CM can only transmit on the
FDX US channels that are below the upstream upper band edge of the CM's diplexer and receive on the DS FDX
channels that are above the downstream lower band edge CM's downstream diplexer. For example, a high-split
FDX-L CM can be a Test CM in the FDX spectrum between 108 MHz and 204 MHz, and a Measurer CM in the
FDX spectrum above 258 MHz; a mid-split FDX-L CM can only be a Measurer CM in the FDX band, as its
upstream frequency range is below 108 MHz.
The choice of the channel sounding methods for different combinations of Test and Measurer CMs are given by
Table 26.
                  Table 26 - Channel Sounding Methods for Test-Measurer CM Combinations

                                Test CM         Measurer CM           Sounding Method
                          FDX                FDX                    OUDP or CWT
                          FDX                FDX-L                  CWT
                          FDX-L              FDX                    OUDP


FDX CMs MUST be designed to enable implementing both OUDP and CWT methods. Therefore, any of the two
methods may be used when both Test CMs and Measurer CMs are FDX CMs.
Since FDX-L CMs cannot transmit a multiplicity of CWTs, OUDP method is to be used when the FDX-L CMs are
required to operate as Test CMs. Since FDX-L CMs are not designed to measure MER from OUDP frames, CWT
method has to be used for an FDX-L CM operating as a Measurer CM. The sequence of operations in a typical
sounding operation to identify the IG of a new CM is described in [DOCSIS MULPIv4.0].

7.2.6.1     OUDP Method
The OUDP sounding uses OUDP Test Bursts as the test signal for sounding when the measurer CMs involve only
FDX CMs. However, both FDX CMs and FDX-L CMs may operate as test CMs. In this method, the Test CM
MUST transmit a set of successive OFDMA frames carrying pseudo-random data over the modulated bandwidth of
an upstream channel. The Measurer CMs MUST report the MER of each subcarrier in this sub-band. It is preferable
to simultaneously sound all upstream channels covering a downstream channel, because sounding over one upstream
channel interferes with the whole of the downstream channel that overlaps with this upstream channel.
The specification states that the FFT and CP size of FDX upstream and downstream channels sharing the same
frequency band are to be the same. However, there is no requirement for the Test CM to synchronize these upstream
OFDMA symbols to the downstream OFDM symbols.
Profile and OCD (OFDM Channel Descriptor) changes are not required for OUDP sounding. Furthermore, sounding
can be carried out over all subcarriers, including PLC and all pilots. Note that PLC and continuous pilots are
excluded from sounding in the CWT method.
Sounding can result in many uncorrectable codewords. Therefore, the Measurer CM does not update any counter
that is related to PNM error ratio monitoring during the OUDP Measuring Period while DS is protected (see
[DOCSIS MULPIv4.0] specification for more details).

7.2.6.1.1    Sounding Period
Refer to Section 7.2.8.




06/27/25                                           CableLabs                                                 95
<!-- PDF_PAGE: 96 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


7.2.6.1.2    Upstream Transmit Power
The FDX CMTS MUST command the upstream power level based upon ranging power for the FDX CM to use
during OUDP Sounding.

7.2.6.1.3    MER Measurement Procedure
Refer to Section 7.2.8.

7.2.6.2     CW Tone Method
In this procedure, the Transmitting FDX CMs send a set of CW tones as the test signal. The Measurer CMs, namely
FDX-L and FDX CMs, measure the MER values at the subcarrier frequencies corresponding to these tones.

7.2.6.2.1    Frequency Offset
The CW tones cannot be placed precisely at subcarrier frequencies of the downstream transmission for reasons
given below. If a CW tone is placed at a subcarrier frequency, an FDX-L CM will detect the same amplitude and
phase for this tone, at subcarriers that are 128 symbols apart. Note that FDX-L CMs measure the MER of subcarriers
using the mean and variance of scattered pilots, which are 128 symbols apart; the mean is the channel frequency
response and variance is the noise power. Hence the CW tone at a precise subcarrier frequency will contribute only
to the mean, i.e., the channel frequency response, and not to the variance, i.e., interference power. As a result, the
MER measurement will be incorrect. In order to make the MER a measure of the interference caused by CW tones,
these tones have to contribute to the variance and not to the mean.
A simple way of achieving this is by offsetting the frequencies of the CW tones by a small amount from the OFDM
subcarrier frequencies. Let this frequency offset be αΔf, where Δf is the subcarrier spacing (25 or 50 kHz) and α is a
small unsigned fractional number. Then the CW tone corresponding to subcarrier index k may be written as:



It can then be shown that the phase difference of the CW tone seen by a CM at two successive scattered pilot
locations is given by the equation below, where TFFT is the FFT time (20 or 40 µs) and TCP is the cyclic prefix time.




If α is chosen such that ΔØ is equal to (2n+1)π then the contribution of the CW tone to the mean will cancel out
every two scattered pilots. Therefore, over an even number of scattered pilots the entire contribution from the CW
tone will be to the variance and not to the mean.
However, adjacent subcarriers are also impacted by the CW tone if its frequency is offset from the subcarrier
frequency grid. Therefore, the value of the offset is to be kept as small as possible. The smallest value of α to give
ΔØ equal to (2n+1)π at subcarriers 128 symbols apart is:




For a typical case of a 20 µs symbol (i.e., 4K FFT) with 2.5 µs cyclic prefix, this gives a value of α of 0.003472. The
corresponding frequency offset αΔf is 173.6 Hz.
This frequency offset can be halved by choosing α such that ΔØ is equal to π/2. Then the contribution of the CW
tone to the mean is cancelled out every four scattered pilots. This is acceptable because averaging for MER
calculation is done over many scattered pilots. Frequency offsets close to the precise values obtained from the above
equations may be used for sounding.

7.2.6.2.2    Ramping to Reduce ICI at Start and End
CW tones, if created with an abrupt start and end, can cause excessive ICI (Inter-Carrier Interference) in the symbols
overlapping with start and end of CW tones. This is because tone start/end cannot be synchronized to downstream
symbol boundaries of all Measurer CMs. Therefore, it is important that CW tones start and end with tapering to have


96                                                   CableLabs                                              06/27/25
<!-- PDF_PAGE: 97 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627


a smooth transition from amplitude 0 to full amplitude. Linear ramping, where amplitudes vary linearly from 0 to
full amplitude, achieves necessary ICI reduction with reasonable ramping length (Figure 21).




Figure 21 - CW Tone Power Ramp Up and Ramp Down in Start and End to Reduce ICI to Adjacent Subcarriers
Amplitude of CW tones at the Measurer CM can be up to 17 dB higher than the average received subcarrier power
level. This accounts for possible +10 dB boosting and +7 dB leakage, from sounding CM to Measurer CM. Table 27
shows the worst-case ICI level for symbols affected by the linear tapered part of CW for 8K OFDM mode with
receiver window of 64. Ramping length, L, is given in natural OFDM sample rate of 204.8 MHz. CW tone is
assumed to be 17 dB above average subcarrier level at the Measurer CM.
        Table 27 - Worst Case ICI for Ramped CW Tones Relative to Average DS Data Subcarrier Power

            Adjacent Carrier Index        L = 32*8192               L = 64*8192             L = 128*8192
                      1                       -29 dB                   -35 dB                   -41 dB
                      2                       -35 dB                   -41 dB                   -47 dB
                      3                       -39 dB                   -45 dB                   -51 dB


When initiating transmission of CW sounding tones, the FDX CM MUST increase tone amplitude linearly from zero
to its maximum over 128 * 8192 samples in OFDM sample rate of 204.8 MHz.
When terminating transmission of CW sounding tones, the FDX CM MUST decrease tone amplitude linearly from
its maximum to zero over 128 * 8192 samples in OFDM sample rate of 204.8 MHz.
The reasons for ramping length 128*8192 are as follows.
    •    This lowers ICI level beyond second neighboring subcarrier to below -51 dB.
It is clear from the ICI data given in Table 47 in Appendix I that ramping brings the ICI contribution in transition
regions to well below the ICI level due to frequency offset itself.

7.2.6.2.3     Phase Randomization
With a reasonable number of CW tones required to do sounding, any potential for CW tones to constructively
combine and form large time domain peaks should be avoided. If all the CW tones start off with the same phase, we
clearly have constructive addition at time zero. With the linear ramping of amplitude at the beginning of CW tones,
we can expect this peak at time 0 to not cause any issue. However, CW tones can be expected to periodically
produce similar peaks with time.
For example, assume CW tones are generated at a set of frequencies, which are a multiple of 50 kHz (4K OFDM
mode) plus a small frequency offset, as specified. If all CW tones start with the same starting phase, then all CW
tones will achieve the same phase after 20 us (1/50 kHz), but with a different value than the starting phase.
Regardless of the value of the starting phase, all CW will add constructively whenever all the starting phases are the
same. Therefore, a peak is achieved once every 20 us in this case.
To avoid the above issue, the starting phases of CW tones are made random. The pseudo random sequence, wm,
described in section 7.5.15.3 of [DOCSIS PHYv3.1] for pilot modulation, is also applicable for CW tone phase
randomization. The FDX sub-band may comprise 1 or 2 OFDMA upstream channels, depending on the FDX
allocated spectrum, as defined in Section 7.2.2.5.5. For sub-bands with 2 OFDMA upstream channels, denote
upstream channel occupying the lower and upper half of a sub-band as US0 and US1 respectively. For sub-bands



06/27/25                                               CableLabs                                                  97
<!-- PDF_PAGE: 98 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


with 1 OFDMA upstream channel, denote this upstream channel as US0. The starting phase for CW tone
corresponding to upstream subcarrier index k, Φ_k, is derived as follows:
Define m such that m = k for US0 channels, and m = k + 4096 for US1 channels.
         wm = 0: CW starting phase, Φk= 0
         wm = 1: CW starting phase, Φk=π
Please note: the starting phases of CW tones are defined as the relative phase offsets to each other, and there could
be a common phase offset from the specified starting phases for all the CW tones.

7.2.6.2.4   CWT Transmit Power and Boosting
Transmit power of the CW tones adheres to the same rules as regular data subcarriers including pre-equalization,
with the exception of boosting (raising the transmit power of a CW to a level above the commanded power per
subcarrier). Boosting level, Kboost, is communicated by the FDX CMTS to the FDX CM. The FDX CM MUST apply
the commanded boosting level when transmitting CW tones. Same Kboost is applied to all CWT within a sub-band, a
different Kboost can be applied to different sub-bands. The range of Kboost value is between 0 to +10 dB with ¼dB
resolution.The FDX CMTS MUST limit the value of boosting (Kboost), the number of CW tones in a minislot, the
number of CW tones per sub-band (n), and the total number of CW tones per FDX CM which are transmitted
concurrently (N), according to the following rules:
     •   The total transmit power of any minislot containing CW tones does not exceed an equivalent total transmit
         power of that minislot if it were to contain data subcarriers without exclusions (commanded power for that
         minislot).
     •   n ≤ 255
     •   N ≤ 765
0 ≤ Kboost [dB] ≤ Max(0, Min(10*log10(255/N),10))

7.2.6.2.5   CMTS Requirements
     •   The FDX CMTS MUST set the modulation of data subcarriers corresponding to sounding CWs to zero bit-
         loading for the duration of the sounding test.
     •   The FDX CMTS MUST NOT grant any CM US transmission opportunities at the frequency locations of
         sounding CWs. In addition, on the upstream, the CMTS can set the modulation order of the 3 data
         subcarriers on each side of every sounding CW to zero bit-loading or reduce the modulation order enough
         to avoid errors on corresponding data subcarriers due to ICI.
     •   The FDX CMTS MUST support configuring and advertising the center frequency of the sounding CW
         signals which are offset from the OFDM subcarrier center frequency and render one of the following phase
         rotations across the Scattered Pilot period: π/2, 2π/3 or π. Once it receives the commands from the FDX
         CMTS that configures and advertises the center frequency of the sounding CW signal, the FDX CM MUST
         use Table 28 - Fractional Frequency Offset as a Function of Phase Rotation, Ncp and Subcarrier Spacing to
         compute the fractional frequency offset from the OFDM subcarrier center frequency. CW frequency offset
         is then computed from the fractional frequency offset and the subcarrier spacing as follows:
                     CW Frequency Offset = Fractional Frequency Offset * Subcarrier Spacing


     Table 28 - Fractional Frequency Offset as a Function of Phase Rotation, Ncp and Subcarrier Spacing

                            Fractional Offset for π/2     Fractional Offset for 2π/3    Fractional Offset for π
     Subcarrier             Phase Rotation Across          Phase Rotation Across        Phase Rotation Across
      Spacing       Ncp      Scattered Pilot Period        Scattered Pilot Period       Scattered Pilot Period
         50 kHz      192               1/536                         1/402                         1/268
                     256               1/544                         1/408                         1/272




98                                                  CableLabs                                              06/27/25
