# 7.3.5 - FDD Downstream Transmit and Receive

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 134-146
Classification: Main body (DOCSIS 4.0 FDD requirements)

<!-- PDF_PAGE: 134 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


7.3.4.15 Ranging
See [DOCSIS PHYv3.1], section 7.4.15.

7.3.4.16 Upstream Pilot Structure
See [DOCSIS PHYv3.1], section 7.4.16.

7.3.4.17 Upstream Pre-Equalization
See [DOCSIS PHYv3.1], section 7.4.17.

7.3.5     FDD Downstream Transmit and Receive

7.3.5.1    Overview
This section specifies the downstream electrical and signal processing requirements for the transmission of OFDM
modulated RF signals from the FDD node to the FDD CM.

7.3.5.2    Signal Processing
See [DOCSIS PHYv3.1], section 7.5.2.

7.3.5.3    Time and Frequency Synchronization
See [DOCSIS PHYv3.1], section 7.5.3.

7.3.5.4    Downstream Forward Error Correction
See [DOCSIS PHYv3.1], section 7.5.4.

7.3.5.5    Mapping Bits to QAM Constellations
See [DOCSIS PHYv3.1], section 7.5.5.

7.3.5.6    Interleaving and De-interleaving
See [DOCSIS PHYv3.1], section 7.5.6.

7.3.5.7    IDFT
See [DOCSIS PHYv3.1], section 7.5.7.

7.3.5.8    Cyclic Prefix and Windowing
See [DOCSIS PHYv3.1], section 7.5.8.

7.3.5.9    Downstream Fidelity Requirements
For the purposes of this specification, the number of occupied CTA channels of an OFDM channel is the occupied
bandwidth of the OFDM channel divided by 6 MHz.
FDD RMD/RPDs capable of generating N-channels of legacy DOCSIS plus NOFDM-channels of OFDM per RF port,
for purposes of the DRFI output electrical requirements, the device is said to be capable of generating Neq-channels
per RF port, where Neq = N + 32*NOFDM "equivalent legacy DOCSIS channels."
An Neq-channel per RF port FDD RMD/RPD MUST comply with all requirements operating with all Neq channels
on the RF port.
An Neq'-channel per RF port device operating with Neq' active channels on the RF port for all values of Neq' less than
Neq.




134                                                 CableLabs                                             06/27/25
<!-- PDF_PAGE: 135 -->
Physical Layer Specification                                                        CM-SP-PHYv4.0-I07-250627


For an OFDM channel there is a) the occupied bandwidth, b) the encompassed spectrum, c) the modulated spectrum,
and d) the number of equivalent legacy DOCSIS channels.
The encompassed spectrum in MHz is 204.8 MHz minus the number of subcarriers in the band edge exclusion sub-
band for the upper and lower band edges (combined) times the subcarrier spacing in MHz. For example, with
subcarrier spacing of 50 kHz and 150 lower band edge subcarriers and 152 upper band edge subcarriers for a total of
302 subcarriers in the two band edge exclusion sub-bands, the encompassed spectrum = 204.8 - 302*(0.05) = 189.7
MHz. The encompassed spectrum is also equal to the center frequency of the highest frequency modulated
subcarrier minus the center frequency of the lowest frequency modulated subcarrier in an OFDM channel, plus the
subcarrier spacing.
The modulated spectrum of an OFDM channel is the encompassed spectrum minus the total spectrum in the Internal
Excluded Sub-bands of the channel, where the total spectrum in the Internal Excluded Sub-bands is equal to the
number of subcarriers in all of the Internal Excluded Sub-bands of the OFDM channel multiplied by the subcarrier
spacing of the OFDM channel. In the previous example, if there are 188 subcarriers total in three Internal Exclusion
Sub-bands, then the total spectrum in the Internal Excluded Sub-bands (in MHz) is 188*0.05 = 9.4 MHz, and the
modulated spectrum is 189.7 MHz - 9.4 MHz = 180.3 MHz.
The occupied bandwidth is a multiple of 6 MHz, with a minimum of 24 MHz, and consists of all CTA channels
which include the modulated spectrum plus taper region shaped by the OFDM channels' transmit windowing; the
out-of-band spurious emissions requirements apply outside the occupied bandwidth. With a 1 MHz taper region on
each band edge of the OFDM channel, shaped by the transmit windowing function, encompassed spectrum of 189.7
MHz may provide 192 MHz of occupied bandwidth.
The number of equivalent active legacy DOCSIS channels in the OFDM channel Neq' is the ceiling function applied
to the modulated spectrum divided by 6 MHz. For the example, the number of equivalent legacy DOCSIS channels
in the OFDM channel is ceiling(180.3 MHz/6 MHz) = 31.
For an Neq-channel per RF port device, the applicable maximum power per channel and spurious emissions
requirements are defined using a value of N* = minimum( 4Neq', ceiling[Neq/4]) for Neq' < Neq/4, and N* = Neq'
otherwise.
These specifications assume that the FDD RMD/RPD entity will be terminated with a 75 Ohm load.
For the purpose of defining the downstream fidelity requirements in the next several sections, please refer to Figure
28; normative statements in this section apply to Interface C.
Figure 31 shows a reference waveform for the output of interface C.
           dBmV




                                                                                     Downstream Reference PSD
                                                             D
                                                                                     Power Down Step




                         f0                          fD                  f1
                                Frequency, MHz

                     Figure 31 - Downstream Power RPD Step Down with D dB at F(D) MHz




06/27/25                                            CableLabs                                                   135
<!-- PDF_PAGE: 136 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


7.3.5.9.1    RMD/RPD Output Electrical Requirements
For OFDM, all modulated subcarriers in an OFDM channel are set to the same average power (except pilots which
are boosted by 6 dB). For purposes of spurious emissions requirements, the "commanded transmit power per
channel" for an equivalent legacy DOCSIS channel is computed as follows:
      •   FDD RMP/RPD power is configured by power per CEA channel and number of occupied CEA channels
          for each OFDM channel.
      •   For each OFDM channel, the total power is Power per CEA channel + 10log10(Number of occupied CEA
          channels) for that OFDM channel.
      •   The FDD RMD/RPD calculates power for data subcarrier and pilots (using total number of non-zero valued
          (non-excluded) subcarriers).
      •   The FDD RMD/RPD calculates power in 6 MHz containing PLC.
      •   For the spurious emissions requirements, power calculated for the 6 MHz containing the PLC is the
          commanded average power of an equivalent DOCSIS legacy channel for that OFDM channel.
An FDD RMD/RPD MUST output an OFDM RF modulated signal with the characteristics defined in Table 41 -
FDD RMD/RPD RF Output Electrical Requirements, Table 42 - FDD RMD/RPD Output Power, and Table 43 -
FDD RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements. Legacy DOCSIS RF modulated
signal characteristics are provided in Section 6.2.22.
The condition for these requirements is all Neq' combined channels, legacy DOCSIS channels and equivalent legacy
DOCSIS channels, commanded to the same average power, except for the single channel active phase noise,
diagnostic carrier suppression, OFDM phase noise, OFDM diagnostic suppression, and power difference
requirements, and except as described for out-of-band noise and spurious requirements.
                              Table 41 - FDD RMD/RPD RF Output Electrical Requirements

                         Parameter                                                      Value
  Downstream lower band edge of an FDD RMD/RPD       DOCSIS 4.0 FDD Mode: 258 MHz
                                                     Legacy Transition Mode: 54 MHz
                                                      (See Notes 3, 12, and 13 following this table.)
  Downstream upper band edge of an FDD RMD/RPD       1794 MHz.
                                                      (See Notes 3 and 12 following this table.)
  Level                                              Adjustable. See Table 42.
  Modulation type                                    See Section 7.3.2.4.
  OFDM channels' subcarrier spacing                  25 kHz and 50 kHz
  Inband Spurious, Distortion, and Noise
  576 MHz total occupied bandwidth,
  6 MHz gap (Internal Excluded subcarriers)
  88 equivalent legacy DOCSIS channels.
    See Notes 4, 6

  For measurements below 600 MHz                     ≤ -50 dBr Average over center 400 kHz subcarriers within gap

  For measurements from 600 MHz to 1002 MHz          ≤ -47 dBr Average over center 400 kHz subcarriers within gap

  For measurements 1002 MHz to 1794 MHz              ≤ -45 dBr Average over center 400 kHz subcarriers within gap
  MER in any 192 MHz OFDM channel total occupied     ≥ 48 dB Average over the complete OFDM channel.
  bandwidth                                          See Note 1

  See Notes 2, 4, 5, 6

  For measurements from 258 MHz to 1794 MHz          1.0 dB MER relaxation per 1.0 dB step-down value




136                                                CableLabs                                                06/27/25
<!-- PDF_PAGE: 137 -->
Physical Layer Specification                                                                   CM-SP-PHYv4.0-I07-250627


                     Parameter                                                                 Value
  MER in 24 MHz OFDM channel occupied bandwidth,           ≥ 48 dB     Average over the complete OFDM channel See Note 1.
  single OFDM channel only, 24 MHz total occupied
  bandwidth:
   See Notes 1, 2, 4, 8
  For measurements from 258 MHz to 1794 MHz
  Phase noise, double sided maximum, Full power CW         1 kHz to 10 kHz:       -48 dBc
  signal 1002 MHz or lower                                 10 kHz to 100 kHz:     -56 dBc
                                                           100 kHz to 1 MHz:      -60 dBc
                                                           1 MHz to 10 MHz:       -54 dBc
                                                           10 MHz to 100 MHz:     -58 dBc (Note 9)

  Full power 192 MHz OFDM channel block with 6 MHz         1 kHz to 10 kHz:       -48 dBc
  in center as Internal Exclusion sub-band + 0 dBc CW      10 kHz to 100 kHz:     -56 dBc
  in center, with block not extending beyond 1002 MHz
    (CW not processed via FFT)

  Full power 192 MHz OFDM channel block with 24 MHz        100 kHz to 1 MHz:      -59 dBc
  in center as Internal Exclusion sub-band + 6 dBc CW
  in center, with block not extending beyond 1002 MHz
    (CW not processed via FFT)

  Full power 192 MHz OFDM channel block with 30 MHz        1 MHz to 10 MHz:       -53 dBc
  in center as Internal Exclusion sub-band + 7 dBc CW
  in center, with block not extending beyond 1002 MHz
    (CW not processed via FFT)
  Output Impedance                                         75 ohms
  Output Return Loss (Note 3)                              > 14 dB within an active output channel from 108 MHz to 750 MHz
                                                           > 13 dB within an active output channel from 750 MHz to 870 MHz
                                                           > 12 dB within an active output channel from 870 MHz to 1794 MHz
                                                           > 12 dB in every inactive channel from 54 MHz to 870 MHz
                                                           > 10 dB in every inactive channel from 870 MHz to 1794 MHz
 TABLE NOTES
 NOTE 1    Receiver channel estimation is applied in the test receiver; test receiver does best estimation possible. Transmit windowing
           is applied to potentially interfering channel and selected to be sufficient to suppress cross channel interference.
 NOTE 2    MER (modulation error ratio) is determined by the cluster variance caused by the transmit waveform at the output of the
           ideal receive matched filter. MER includes all discrete spurious, noise, subcarrier leakage, clock lines, synthesizer products,
           distortion, and other undesired transmitter products. Phase noise up to ±50 kHz of the subcarrier is excluded from inband
           specification, to separate the phase noise and inband spurious requirements as much as possible. In measuring MER,
           record length or carrier tracking loop bandwidth may be adjusted to exclude low frequency phase noise from the
           measurement. MER requirements assume measuring with a calibrated test instrument with its residual MER contribution
           removed.
 NOTE 3    Frequency ranges are edge-to-edge.
 NOTE 4    Phase noise up to 10 MHz offset is mitigated in test receiver processing or by test equipment (latter using hardline carrier
           from modulator, which requires special modulator test port and functionality).
 NOTE 5    Up to 5 subcarriers in one OFDM channel can be excluded from this requirement.
 NOTE 6    The measured OFDM channel is allocated 204.8 MHz of spectrum which is free from the other OFDM channel and 32 SC-
           QAM channels which together comprise 576 MHz of occupied bandwidth.
 NOTE 7    The estimated channel impulse response used by the test receiver is limited to half of length of smallest transmit cyclic
           prefix.
 NOTE 8    A single subcarrier in the OFDM channel can be excluded from this requirement, no windowing is applied, and minimum CP
           is selected.
 NOTE 9    Test limit includes 2 dB added to -60 dBc due to the contribution from the modulator noise floor which is allowed by spurious
           emissions.
 NOTE 10 CCAP spec approved and commonly used in CMTS/EQAM.
 NOTE 11 Commonly used in nodes.
 NOTE 12 When the downstream lower band edge is below 258 MHz, the occupied downstream spectrum is limited to 1,536 MHz.
         Therefore, the downstream upper band edge is not necessarily 1,794 MHz under these conditions.
 NOTE 13 Any frequencies transmitted by an FDD node below 258 MHz are intended for video SC-QAM and other non-DOCSIS
         signals.




06/27/25                                                 CableLabs                                                            137
<!-- PDF_PAGE: 138 -->
CM-SP-PHYv4.0-I07-250627                                           Data-Over-Cable Service Interface Specifications


7.3.5.9.1.1    Power per Channel for FDD RMD/RPD
Control over an FDD RMD/RPD's electrical output is required for many of the characteristics, such as RF channel
power, number of RF channels, modulation characteristics of the channels, center frequency of channels, and so
forth. Different mechanisms of control can exist for an FDD RMD/RPD and these mechanisms of control can be
found in the DCA specifications. One possible mechanism of control is via commands carried in the downstream
link into the FDD RMD/RPD, remote node control. A second mechanism of control is local node control, also
referenced as "local-only", separate from the downstream link into the FDD RMD/RPD, such as an electrical
interface operable at installation or even pluggable components set at installation. In an FDD RMD/RPD some
adjustable characteristics can be controlled by one mechanism, and not the other, or by both; therefore, some
"adjustable" characteristics can perhaps not be remotely changed. Local-only adjustments made at installation can be
subsequently amended, but not remotely, and could incur service interruption.
An FDD RMD/RPD MUST be capable of generating 256 equivalent legacy DOCSIS channels, Neq, onto the RF
port at Interface C.
An FDD RMD/RPD has to be capable of generating a power per channel of at least 20 dBmV/6 MHz. The Channel
Power Reference Setting (dBmV/6 MHz) of the FDD RMD/RPD could possibly be adjustable remotely, but is also
permitted to be adjustable only locally, or even fixed (not adjustable), and serves as the reference power (0 dBc) for
independently controlled individual channel power adjustment, and for spurious emissions.
The power per channel of the FDD RMD/RPD has to be adjustable for each channel independently, via remote
adjustment, over a range of up to10 dB below the Channel Power Reference Setting. An FDD RMD/RPD has to be
adjustable to operate with fewer than Neq = 281 channels on its RF port, for all Neq' down to Neq' = 96 channels. The
FDD RMD/RPD has to comply with all requirements operating with all Neq = 281 channels on the RF port, and has
to comply with all requirements operating with Neq' channels on the RF port for all values of Neq' less than Neq that it
supports.
These specifications assume that the FDD RMD/RPD device will be terminated with a 75 Ohm load.
An FDD RMD/RPD MUST generate an RF output with power capabilities as defined in Table 42 - FDD RMD/RPD
Output Power.
The FDD RMD/RPD MUST be capable of adjusting channel RF power on a per channel basis as stated in Table 42 -
FDD RMD/RPD Output Power.
The FDD RMD/RPD MUST be capable of adjusting power on a per channel basis for the legacy DOCSIS channels,
with each channel independently meeting the power capabilities defined in Table 42 - FDD RMD/RPD Output
Power.
                                        Table 42 - FDD RMD/RPD Output Power

              for Neq' (Number of Active Channels Combined per RF Port) in the Range of 96 to 281
                  Parameter                                                             Value
 Channel Power Reference Setting                    Required power in dBmV per 6 MHz channel
 (Maximum required power, i.e., 0 dBc, per          ≥ 20 dBmV
 channel for Neq' channels combined onto a single   (See item #1 in the requirements list immediately following this table)
 RF port for an Neq channel RPD):
                                                    NOTES:
                                                    No upper limit to the Channel Power Reference Setting which an RPD can
                                                    provide

                                                    No requirement for remote adjustability for Channel Power Reference Setting

                                                    An RMD/RPD which has fixed Channel Power Reference Setting to meet full
                                                    fidelity (Table 41, and Table 43 at that setting. (See item #2 in the
                                                    requirements list immediately following this table)

                                                    For FDD RMD/RPDs which have adjustable Channel Power Reference
                                                    Setting, see the corresponding row in this table.




138                                                     CableLabs                                                      06/27/25
<!-- PDF_PAGE: 139 -->
Physical Layer Specification                                                                   CM-SP-PHYv4.0-I07-250627


              for Neq' (Number of Active Channels Combined per RF Port) in the Range of 96 to 281
                   Parameter                                                              Value
 Range of Channel Power Reference Setting             may be adjustable locally

                                                      An FDD RMD/RPD with adjustable Channel Power Reference Setting is to
                                                      meet full fidelity (Table 41 and Table 43) whenever Channel Power
                                                      Reference Setting is at or below 20 dBmV
                                                      (See item #2 in the requirements list immediately following this table)

                                                      For FDD RMD/RPD with Channel Power Reference Setting range which
                                                      does not reach as low as 20 dBmV, the FDD RMD/RPD is to meet full fidelity
                                                      with Channel Power Reference Setting at the lowest setting for the device.
                                                      (See item #4 in the requirements list immediately following this table)
 Range of commanded power per channel;                CMTS 0 dBc to -2 dBc relative to Channel Power Reference Setting, via
 adjusted on a per channel basis                      remote adjustment.

                                                      larger variations than 2 dB below Channel Power Reference Setting, via
                                                      remote adjustment are allowed.
                                                      (See item #3 in the requirements list immediately following this table)
 Commanded power per channel step size                ≤ 0.2 dB Strictly monotonic
 Power difference between any two adjacent            ≤ 0.5 dB
 channels in the 154 MHz to 1794 MHz
 downstream spectrum (with commanded power
 difference removed if channel power is
 independently adjustable and/or accounting for
 pilot density variation and subcarrier exclusions)
 Power difference between any two non-adjacent        ≤ 1 dB
 channels in a 48 MHz contiguous bandwidth
 block (with commanded power difference
 removed if channel power is independently
 adjustable)
 Power difference (normalized for bandwidth)          ≤ 10 dB
 between any two channels OFDM channel blocks
 or legacy DOCSIS channels in the 258 MHz to
 1794 MHz downstream spectrum (with
 commanded power difference removed if channel
 power is independently adjustable)
 Power per channel absolute accuracy                  ±3 dB
                                                      Table footnote: This specification contains a stability requirement which is
                                                      much tighter than +-3 dB.
 Diagnostic carrier suppression (3 modes)
 Mode 1: One channel suppressed must be               1) ≥ 50 dB carrier suppression within the occupied bandwidth in any one
 controlled remotely                                  active channel. When suppressing the carrier ≥ 50 dB within the occupied
                                                      bandwidth in any one active channel the CMTS is to control transmissions
                                                      such that no service impacting discontinuity or detriment to the unsuppressed
                                                      channels occurs.
                                                      (See item #5 in the requirements list immediately following this table)

 Mode 2: All channels suppressed except one           2) 50 dB carrier suppression within the occupied bandwidth in every active
 must be controlled remotely                          channel except one. The suppression is not required to be glitchless, and the
                                                      remaining unsuppressed active channel is allowed to operate with increased
                                                      power such as the total power of the Neq' active channels combined.




06/27/25                                                  CableLabs                                                            139
<!-- PDF_PAGE: 140 -->
CM-SP-PHYv4.0-I07-250627                                         Data-Over-Cable Service Interface Specifications


              for Neq' (Number of Active Channels Combined per RF Port) in the Range of 96 to 281
                  Parameter                                                          Value
 Mode 3: All channels suppressed are to be        3) 50 dB carrier suppression within the occupied bandwidth in every active
 controlled remotely                              channel.

                                                  The CMTS is to control transmissions such that in all three diagnostic carrier
                                                  suppression modes the output return loss of the suppressed channel(s)
                                                  complies with the Output Return Loss requirements for active channels given
                                                  in Table 41 (See item #5 in the requirements list immediately following this
                                                  table).

                                                  The total noise and spur requirement is the combination of noise power from
                                                  the 50 dBc suppressed channel and the normal noise and spur requirement
                                                  for the CMTS output when operating with all channels unsuppressed.
TABLE NOTE
NOTE 1 "Channel" in mode 1 or mode 2 carrier suppression refers to an OFDM channel with at least 22 MHz of contiguous
       modulated spectrum or an SC-QAM channel.


The following is a list of FDD RMD/RPD RF Output Electrical Requirements based on Table 42 above.
1.    An FDD RMD/RPD MUST support a Channel Power Reference Setting ≥ 20 dBmV.
      •   An FDD RMD/RPD which has fixed Channel Power Reference Setting MUST meet full fidelity (Table 41
          - FDD RMD/RPD RF Output Electrical Requirements and Table 43 - FDD RMD/RPD Output Out-of-Band
          Noise and Spurious Emissions Requirements) at that setting.
2.    An FDD RMD/RPD MAY support a locally adjustable Range of Channel Power Reference Setting.
      •   An FDD RMD/RPD with adjustable Channel Power Reference Setting MUST meet full fidelity (Table 41 -
          FDD RMD/RPD RF Output Electrical Requirements and Table 43 - FDD RMD/RPD Output Out-of-Band
          Noise and Spurious Emissions Requirements) whenever Channel Power Reference Setting is at or below:
          20 dBmV.
      •   For FDD RMD/RPD with Channel Power Reference Setting range which does not reach as low as 20
          dBmV, the FDD RMD/RPD MUST meet full fidelity with Channel Power Reference Setting at the lowest
          setting for the device.
3.    An FDD RMD/RPD MAY support a range of commanded power per channel, adjusted on a per channel basis,
      larger variations than 2 dB below Channel Power Reference Setting, via remote adjustment.
4.    An FDD RMD/RPD MUST support a range of commanded power per channel, adjusted on a per channel basis,
      from 0 dBc to -2 dBc relative to Channel Power Reference Setting, via remote adjustment.
5.    In an FDD RMD/RPD, Diagnostic carrier suppression (3 modes)
      •   Mode 1: One channel suppressed to be controlled remotely - for this mode, the FDD CMTS MUST support
          ≥ 50 dB carrier suppression within the occupied bandwidth in any one active channel. When suppressing
          the carrier ≥ 50 dB within the occupied bandwidth in any one active channel, the FDD CMTS MUST
          control transmissions such that no service impacting discontinuity or detriment to the unsuppressed
          channels occurs.
      •   Mode 2: All channels suppressed except one to be controlled remotely - for this mode, the FDD CMTS
          MUST support 50 dB carrier suppression within the occupied bandwidth in any one active channel except
          one. The suppression is not required to be glitchless, and the remaining unsuppressed active channel is
          allowed to operate with increased power such as the total power of the Neq' active channels combined.
      •   Mode 3: All channels suppressed to be controlled remotely - for this mode, the FDD CMTS MUST support
          50 dB carrier suppression within the occupied bandwidth in every active channel.




140                                                   CableLabs                                                    06/27/25
<!-- PDF_PAGE: 141 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


     •   The FDD CMTS MUST control transmissions such that in all three diagnostic carrier suppression modes,
         the output return loss of the suppressed channel(s) complies with the Output Return Loss requirements for
         active channels given in Table 41 - FDD RMD/RPD RF Output Electrical Requirements.
6.   When configured to mute an RF output port, the FDD RMD/RPD MUST control transmissions such that the
     output return loss of the output port of the muted device complies with the Output Return Loss requirements for
     inactive channels given in Table 41 - FDD RMD/RPD RF Output Electrical Requirements.

7.3.5.9.1.2   Out-of-Band Noise and Spurious Requirements for RMD/RPD
One of the goals of the [DOCSIS DRFI] specification is to provide the minimum intended analog channel CNR
protection of 60 dB for systems deploying up to 119 DRFI-compliant QAM channels. A DOCSIS PHYv4.0 goal is
to provide protection for legacy DOCSIS channels and for high density constellations of OFDM channel blocks if
they are generated from another DRFI-compliant device.
The specification assumes that the transmitted power level of the digital channels will be 6 dB below the peak
envelope power of the visual signal of analog channels, which is the typical condition for 256-QAM transmission. It
is further assumed that the channel lineup will place analog channels at lower frequencies than digital channels, and
in systems deploying DOCSIS 3.1 modulators, analog channels will be placed at center frequencies below 600 MHz.
An adjustment of 10*log10 (6 MHz / 4 MHz) is used to account for the difference in a 6 MHz equivalent digital
channel, versus an analog channel's noise power bandwidth. With the assumptions above, for a 119-6 MHz
equivalent channel system, the specification in item 5 of Table 43 equates to an analog CNR protection of 60 dB.
With all digital channels at the same equivalent power per 6 MHz channel, the specification provides for 58 dB SNR
protection for analog channels below 600 MHz (even with transmissions above 600 MHz) with 192 MHz occupied
bandwidth (one full OFDM channel) and 51 dB SNR protection for digital channels below 600 MHz with
transmission of 960 MHz modulated spectrum (160 equivalent legacy DOCSIS channels). The SNR protection
between 600 MHz and 1002 MHz is 55 dB for analog channels operating above a 192 MHz occupied bandwidth
generated by a DOCSIS 4.0 FDD compliant device, and is 48 dB between 600 MHz and 1002 MHz for digital
channels operating above 960 MHz occupied bandwidth generated by a DOCSIS 4.0 FDD compliant device.
Table 43 lists the out-of-band spurious requirements. In cases where the N' combined channels are not commanded
to the same power level, "dBc" denotes decibels relative to the strongest channel among the active channels. When
commanded to the same power level, "dBc" should be interpreted as the average channel power, averaged over the
active channels, to mitigate the variation in channel power across the active channels (see Table 43), which is
allowed with all channels commanded to the same power.
The FDD RMD/RPD modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - FDD
RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements, in measurements below 600 MHz
and outside the encompassed spectrum when the active channels are contiguous or when the ratio of modulated
spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater.
The FDD RMD/RPD modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - FDD
RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements, with 1 dB relaxation, in
measurements within gaps in modulated spectrum below 600 MHz and within the encompassed spectrum when the
ratio of modulated spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater.
The FDD RMD/RPD modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - FDD
RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements, with 3 dB relaxation, when the ratio
of modulated spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater, in measurements with
603 MHz ≤ center frequency ≤ 999 MHz, outside the encompassed spectrum or in gap channels within the
encompassed spectrum.
The FDD RMD/RPD modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - FDD
RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements, with 5 dB relaxation, when the ratio
of modulated spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater, in measurements with
999 MHz < center frequency ≤ 1215 MHz, outside the encompassed spectrum or in gap channels within the
encompassed spectrum.
The FDD RMD/RPD modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - FDD
RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements, with 5 dB relaxation, when the ratio


06/27/25                                           CableLabs                                                  141
<!-- PDF_PAGE: 142 -->
CM-SP-PHYv4.0-I07-250627                                     Data-Over-Cable Service Interface Specifications


of modulated spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater, in measurements with
1215 MHz < center frequency ≤ 1791 MHz, outside the encompassed spectrum or in gap channels within the
encompassed spectrum.
The FDD RMD/RPD modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - FDD
RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements, in addition to contributions from
theoretical transmit windowing, with permissible configurations of lower edge and upper edge sub-band exclusions
of at least 1 MHz each, FFT Size, cyclic prefix length (Ncp) and windowing roll-off period (Nrp) values.
Recommendations for configuration parameters are provided in Section 5.2.1.2. The test limit for determining
compliance to the spurious emissions requirements is the power sum of the spurious emissions requirements taken in
accordance with Table 43; and the contributions from the theoretical transmit windowing for the configured
transmissions.
When the Neq' combined active channels are not contiguous, and the ratio of modulated spectrum to gap spectrum
within the encompassed spectrum is 4:1 or greater, the spurious emissions requirements are determined by summing
the spurious emissions power allowed in a given measurement bandwidth by each of the contiguous subblocks
among the occupied bandwidth. In the gap channels within the encompassed spectrum and below 600 MHz there is
a 1 dB relaxation in the spurious emissions requirements, so that within the encompassed spectrum the spurious
emissions requirements (in absolute power) are 26% higher power in the measurement band determined by the
summing of the contiguous subblocks' spurious emissions requirements. In all channels above 600 MHz there is a 3
dB relaxation in the spurious emissions requirements, so that the spurious emissions requirements (in absolute
power) are double the power in the measurement band determined by the summing of the contiguous subblocks'
spurious emissions requirements. The following three paragraphs provide the details of the spurious emissions
requirements for non-contiguous channel operation outside the encompassed spectrum; within the encompassed
spectrum the same details apply except there is an additional 1 dB allowance below 600 MHz; and 3 dB allowance
is applied above 600 MHz for all channels.
The full set of Neq' channels is referred to throughout this specification as the modulated channels or the active
channels. However, for purposes of determining the spurious emissions requirements for non-contiguous transmitted
channels, each separate contiguous subblock of channels within the active channels is identified, and the number of
channels in each contiguous subblock is denoted as Neqi, for i = 1 to K, where K is the number of contiguous
subblocks. Therefore,



Note that K = 1 when and only when the entire set of active channels is contiguous. Also note that an isolated
transmit channel, i.e., a transmit channel with empty adjacent channels, is described by Ni = 1 and constitutes a
subblock of one contiguous channel. Any number of the "contiguous subblocks" may have such an isolated transmit
channel; if each active channel was an isolated channel, then K = N'.
When Neq' ≥ Neq/4, Table 43 is used for determining the noise and spurious power requirements for each contiguous
subblock, even if the subblock contains fewer than Neq/4 active channels. When Neq' < Neq/4, Table 43 is used for
determining the noise and spurious power requirements for each contiguous subblock. Thus, the noise and spurious
power requirements for all contiguous subblocks of transmitted channels are determined from Table 43, where the
applicable table is determined by Neq' being greater than or equal to Neq/4, or not. The noise and spurious power
requirements for the ith contiguous subblock of transmitted channels is determined from Table 43 using the value Ni
for the "number of active channels combined per RF port", and using "dBc" relative to the highest commanded
power level of a 6 MHz equivalent channel among all the active channels, and not just the highest commanded
power level in the ith contiguous subblock, in cases where the Neq' combined channels are not commanded to the
same power. The noise and spurious emissions power in each measurement band, including harmonics, from all K
contiguous subblocks, is summed (absolute power, NOT in dB) to determine the composite noise floor for the non-
contiguous channel transmission condition.
For the measurement channels adjacent to a contiguous subblock of channels, the spurious emissions requirements
from the non-adjacent subblocks are divided on an equal "per Hz" basis for the narrow and wide adjacent
measurement bands. For a measurement channel wedged between two contiguous subblocks, adjacent to each, the
measurement channel is divided into three measurement bands, one wider in the middle and two narrower bands
each abutting one of the adjacent transmit channels. The wideband spurious and noise requirement is split into two



142                                                CableLabs                                            06/27/25
<!-- PDF_PAGE: 143 -->
Physical Layer Specification                                                                  CM-SP-PHYv4.0-I07-250627


parts, on an equal "per Hz" basis, to generate the allowed contribution of power to the middle band and to the
farthest narrowband. The ceiling function is applied to the resulting sum of noise and spurious emissions, per the
first Note in Table 43 to produce a requirement of ½ dB resolution.
Items 1 through 4 list the requirements in channels adjacent to the commanded channels.
Item 5 lists the requirements in all other channels further from the commanded channels. Some of these "other"
channels are allowed to be excluded from meeting the Item 5 specification. All the exclusions, such as 2nd and 3rd
harmonics of the commanded channel, are fully identified in Table 43.
Item 6 lists the requirements on the 2N' 2nd harmonic channels and the 3N' 3rd harmonic channels.
          Table 43 - FDD RMD/RPD Output Out-of-Band Noise and Spurious Emissions Requirements

               For Neq' (number of active channels combined per RF port) in the range of 96 to 281
                                            Band                                                  Requirement (in dBc)
 1    Adjacent channel up to 750 kHz from channel block edge                                 For N* = 1, 2, 3, 4: ≤ -58;

                                                                                             For N* ≥ 5: ≤ 10*log10 [10-58/10
                                                                                             +(0.75/6)*(10-65/10 + (N*-2)*10-73/10)]
 2    Adjacent channel (750 kHz from channel block edge to 6 MHz from channel block          For N* = 1: ≤ -62;
      edge)
                                                                                             For N* ≥ 2: ≤ 10*log10 [10-62/10
                                                                                             +(5.25/6)*(10-65/10 +(N*-2)*10-73/10)]
 3    Next-adjacent channel (6 MHz from channel block edge to 12 MHz from channel            ≤ 10*log10 [10-65/10 +( N*-1)*10-73/10]
      block edge)
 4    Third-adjacent channel (12 MHz from channel block edge to 18 MHz from channel          For N* = 1: ≤ -73;
      block edge)                                                                            For N* = 2: ≤ -70;
                                                                                             For N* = 3: ≤ -67;
                                                                                             For N* = 4: ≤ -65;
                                                                                             For N* = 5: ≤ -64.5;
                                                                                             For N* = 6, 7: ≤ -64;

                                                                                             For N'* ≥ 8: ≤-73 + 10*log10 (N*)
 5    Noise in other channels (47 MHz to 1794 MHz) Measured in each 6 MHz channel            For N* = 1: ≤ -73;
      excluding the following:                                                               For N* = 2: ≤ -70;
      a) Desired channel(s)                                                                  For N* = 3: ≤ -68;
      b) 1st, 2nd, and 3rd adjacent channels (see Items 1, 2, 3, 4 in this table)            For N* = 4: ≤ -67;
      c) Channels coinciding with 2nd and 3rd harmonics (see Item 6 in this table)
                                                                                             For N'* ≥ 5: ≤ -73 + 10*log10 (N*)
 6    In each of 2Neq' contiguous 6 MHz channels or in each of 3N' contiguous 6 MHz          ≤ -73 + 10*log10(N*) dBc, or -63,
      channels coinciding with 2nd harmonic and with 3rd harmonic components                 whichever is greater
      respectively (up to 1218 MHz)
 7    Lower out of band noise in the band of 5 MHz to 47 MHz                                 ≤ -50 + 10*log10(N*)
      Measured in 6 MHz channel bandwidth
 8    Higher out of band noise in the band of 1794 MHz to 3000 MHz                           For N* ≤ 8: ≤ -55 + 10*log10(N*)
      Measured in 6 MHz channel bandwidth
                                                                                             For N* > 8: ≤ -60 + 10*log10(N*)
 TABLE NOTES
 NOTE 1 All equations are Ceiling(Power, 0.5) dBc. Use "Ceiling(2*Power) / 2" to get 0.5 steps from ceiling functions that return
        only integer values. For example, Ceiling(-63.9, 0.5) = -63.5 dBc.
 NOTE 2 Add 3 dB relaxation to the values specified above for noise and spurious emissions requirements in all channels with
        603 MHz ≤ center frequency of the noise measurement ≤ 999 MHz. For example, -73 dBc becomes -70 dBc.
        Add 5 dB relaxation to the values specified above for noise and spurious emissions requirements in all channels with
        999 MHz < center frequency of the noise measurement ≤ 1791 MHz. For example, -73 dBc becomes -68 dBc.
 NOTE 3 Add 1 dB relaxation to the values specified above for noise and spurious emissions requirements in gap channels with
        center frequency below 600 MHz. For example, -73 dBc becomes -72 dBc.

7.3.5.10 Independence of Individual Channels Within Multiple Channels on a Single RF Port
See [DOCSIS PHYv3.1], section 7.5.10.




06/27/25                                                 CableLabs                                                               143
<!-- PDF_PAGE: 144 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


7.3.5.11 Cable Modem Receiver Input Requirements
For FDD devices operating in FDD mode, this section augments section 7.5.11 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
The FDD CM MUST be able to accept any range of OFDM subcarriers defined between Lower Frequency
Boundary and Upper Frequency Boundary simultaneously.
Active subcarrier frequencies, loading, and other OFDM characteristics are described by OFDM configuration
settings and CM exclusion bands and profile definition. The OFDM signals and CM interfaces will have the
characteristics and limitations defined in Table 44.
The FDD CM MUST support the requirements in Table 44 - Electrical Input to FDD CM, which supersedes the
corresponding requirements in [DOCSIS PHYv3.1], table 45 - Electrical Input to CM, unless otherwise noted.
                                     Table 44 - Electrical Input to FDD CM

                Parameter                                                     Value
  Lower band edge                    As selected from Table 31
  Upper band edge                    1794 MHz
  Frequency boundary assignment      25 kHz 8K FFT
  granularity                        50 kHz 4K FFT
  Signal type                        OFDM, SC-QAM
  Maximum single OFDM channel        192 MHz
  bandwidth
  Minimum contiguous-modulated       22 MHz
  OFDM bandwidth
  Number of channels                 Support minimum of 5 OFDM Channels AND
                                     32 SC-QAM Channels
  Subcarrier spacing/FFT duration    25 kHz / 40 µs
                                     50 kHz / 20 µs
  Modulation type                    QPSK, 16-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM, 1024-QAM, 2048-QAM,
                                     4096-QAM
  Variable bit loading               Support with subcarrier granularity
                                     Support zero-bit-loaded subcarriers
  Total input power                  < 40 dBmV, 54 MHz to 1794 MHz
                                     * Assuming negligible power outside this range
  Level range (24 MHz min occupied   -24 dBmV/24 MHz to 21 dBmV/24 MHz
  BW)
  Equivalent PSD to SC-QAM of
  -30 dBmV to + 15 dBmV per 6 MHz
  Maximum average power per MHz      Let X = Average power of lowest power 24 MHz of modulated spectrum for
  input to the CM from 258 MHz to    demodulation
  1794 MHz
  (dBmV/MHz)                         Additional Demodulated Bandwidth, Bdemod:
                                     ≤ Min [X - 10*log10(24) + 12; 21 - 10*log10(24)]

                                     Additional Non-Demodulated Bandwidth, Bno-demod :
                                     ≤ Min [X - 10*log10(24) + 12; 26 - 10*log10(24)]
                                     For up to 12 MHz of occupied bandwidth (analog, OOB, QAM, OFDM)

                                     ≤ Min [X - 10*log10(24) + 12; 21 - 10*log10(24)]
                                     for all remaining bandwidth

                                     NOTE: Level range does not imply anything about BER performance or capability vs.
                                     QAM. CM BER performance is separately described.
  Input impedance                    75 ohms
  Input return loss                  > 6 dB within the selected downstream frequency range
  Connector                          F connector per [ISO/IEC-61169-24] or [SCTE 02]




144                                                CableLabs                                                 06/27/25
<!-- PDF_PAGE: 145 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


7.3.5.12 FDD Cable Modem Receiver Capabilities
The required level for FDD CM downstream post-FEC error ratio is defined as less than or equal to 10-6 PER
(packet error ratio) with 1500-byte Ethernet packets. This section describes the conditions at which the FDD CM is
required to meet this error ratio.

7.3.5.12.1 CM Error Ratio Performance in AWGN Channel
For FDD operation the FDD CM is required to support a downstream upper band edge of 1794 MHz (see Section
7.3.2.1). The FDD CM CNR requirements are defined for two different cases of the input downstream signal upper
limit: 1218 MHz and 1794 MHz. The downstream signal upper limit in this context means that there will be
negligible energy above this frequency at the CM input. For example, in the case of 1218 MHz, there will be no
channel or part of the channel above 1218 MHz either desired or undesired. This could occur if the FDD CM is
operating in a cable plant that only supports downstream signals up to 1218 MHz or if the operator chooses not to
provide downstream signals above 1218 MHz. The downstream lower band edge depends on the supported diplex
filter option and can be one of the following: 258, 372, 492, 606, or 834 MHz.
The FDD CM MUST achieve the required error ratio when operating at a CNR as shown in Table 45 - CM
Minimum CNR Performance1 in AWGN Channel for 1218 MHz Input Signal Upper Frequency Limit and Table 46
- CM Minimum CNR Performance1 in AWGN Channel for 1794 MHz Input Signal Upper Frequency Limit, under
input load and channel conditions as follows:
    •   Any valid transmit combination (frequency, subcarrier clock frequency, transmit window, cyclic prefix,
        pilot, PLC, subcarrier exclusions, interleaving depth, multiple modulation profile configuration, etc.) as
        defined in this spec.
    •   P6AVG (the measured channel power divided by number of occupied CTA channels) ≤ 15 dBmV.
    •   Up to fully loaded spectrum of 54 MHz to 1794 MHz, including up to 48 analog channels placed lower in
        the spectrum than the digital channels.
    •   Power in (both above and below) 4 adjacent 6 MHz channels ≤ P6AVG+3 dB.
    •   Power in any 6 MHz channel over the spectrum ≤ P6AVG+6 dB.
    •   Peak envelope power in any analog channel over the spectrum P6AVG+6 dB.
    •   Average power per channel across spectrum P6AVG+3 dB.
    •   OFDM channel phase noise for the FDD node (Table 41 - FDD RMD/RPD RF Output Electrical
        Requirements).
    •   No other artifacts (reflections, burst noise, tilt, etc.).
    •   The upstream transmit power is within the allowable range and no upstream channel in the TCS FDD
        exceeds the Pref-FDD.
                         Table 45 - CM Minimum CNR Performance1 in AWGN Channel for
                                   1218 MHz Input Signal Upper Frequency Limit

                                          CNR2,3,4 (dB)                  CNR2,3,4 (dB)
           Constellation                 Up to 1002 MHz              1002 MHz to 1218 MHz   Min P6AVG dBmV
                4096                             41.0                        41.5                   -6
                2048                             37.0                        37.5                   -9
                1024                             34.0                        34.0                   -12
                 512                             30.5                        30.5                   -12
                 256                             27.0                        27.0                   -15
                 128                             24.0                        24.0                   -15
                 64                              21.0                        21.0                   -15
                 16                              15.0                        15.0                   -15




06/27/25                                                CableLabs                                              145
<!-- PDF_PAGE: 146 -->
CM-SP-PHYv4.0-I07-250627                                            Data-Over-Cable Service Interface Specifications


      TABLE NOTES
      NOTE 1 Applicable to an OFDM channel with 192 MHz of occupied bandwidth.
      NOTE 2 CNR is defined here as total signal power in occupied bandwidth divided by total noise in occupied bandwidth.
      NOTE 3 Channel CNR is adjusted to the required level by measuring the source inband noise including phase noise
             component along with transmitter noise and distortion and adding the required delta noise from an external
             AWGN generator to achieve the desired CNR at the CM F-connector.
      NOTE 4 The CNR defined in this table is applicable when the cable plant or operator configuration limits downstream
             signals to below 1218 MHz and is, therefore, applicable from its lower band edge to 1218 MHz.


           Table 46 - CM Minimum CNR Performance1 in AWGN Channel for 1794 MHz Input Signal Upper
                                             Frequency Limit

                                                           CNR2,3,4 (dB)
                            Constellation                 Up to 1794 MHz               Min P6AVG dBmV
                                 4096                             41                            0
                                 4096                            44.0                          -6
                                 2048                             37                           -6
                                 2048                            40.0                          -9
                                 1024                            36.0                          -12
                                  512                             33                           -15
                                  256                             30                           -18
                                  128                             28                           -21
                                   64                             26                           -24
                                   16                             20                           -27
                    TABLE NOTES
                    NOTE 1 Applicable to an OFDM channel with 192 MHz of occupied bandwidth.
                    NOTE 2 CNR is defined here as total signal power in occupied bandwidth divided by total
                           noise in occupied bandwidth.
                    NOTE 3 Channel CNR is adjusted to the required level by measuring the source inband
                           noise including phase noise component along with transmitter noise and
                           distortion and adding the required delta noise from an external AWGN generator
                           to achieve the desired CNR at the CM F-connector.
                    NOTE 4 The CNR defined in this table is applicable in the entire downstream band from
                           its lower band edge to 1794 MHz.



7.3.5.13 Physical Layer Link Channel (PLC)
See [DOCSIS PHYv3.1], section 7.5.13.

7.3.5.14 Next Codeword Pointer
See [DOCSIS PHYv3.1], section 7.5.14.

7.3.5.15 Downstream Pilot Patterns
See [DOCSIS PHYv3.1], section 7.5.15.




146                                                      CableLabs                                                  06/27/25
