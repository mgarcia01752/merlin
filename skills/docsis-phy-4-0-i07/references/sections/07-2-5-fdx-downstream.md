# 7.2.5 - FDX Downstream Transmit and Receive

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 77-93
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 77 -->
Physical Layer Specification                                                     CM-SP-PHYv4.0-I07-250627


7.2.5     FDX Downstream Transmit and Receive

7.2.5.1     Overview
This section specifies the downstream electrical and signal processing requirements for the transmission of OFDM
modulated RF signals from the CMTS to the CM.

7.2.5.2     Signal Processing
See [DOCSIS PHYv3.1], section 7.5.2.

7.2.5.3     Time and Frequency Synchronization
See [DOCSIS PHYv3.1], section 7.5.3.

7.2.5.4     Downstream Forward Error Correction
See [DOCSIS PHYv3.1], section 7.5.4.

7.2.5.5     Mapping Bits to QAM Constellations
See [DOCSIS PHYv3.1], section 7.5.5.

7.2.5.6     Interleaving and De-interleaving
See [DOCSIS PHYv3.1], section 7.5.6.

7.2.5.7     IDFT
See [DOCSIS PHYv3.1], section 7.5.7.

7.2.5.8     Cyclic Prefix and Windowing
See [DOCSIS PHYv3.1], section 7.5.8.

7.2.5.9     FDX Downstream Fidelity Requirements for 1218 MHz Operation with CFDX Frequency
            Plan
The requirements of this section apply to FDX nodes operating with the CFDX frequency plan and downstream
signals up to 1218 MHz. The reference point for downstream measurements for such devices is Interface D (see
section 7.2.1). For FDX downstream transmitter operation with the CFDX frequency plan and downstream signals
present up to 1794 MHz, or with the SFDX frequency plan, see section 7.2.5.10.1.
For FDX devices operating in FDX mode, this section augments section 7.5.9 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
For the purposes of this specification, the number of occupied CTA channels of an OFDM channel is the occupied
bandwidth of the OFDM channel divided by 6 MHz.
FDX nodes capable of generating N-channels of legacy DOCSIS plus NOFDM-channels of OFDM per RF port, for
purposes of the [DOCSIS DRFI] output electrical requirements, the device is said to be capable of generating Neq-
channels per RF port, where Neq = N + 32*NOFDM "equivalent legacy 6 MHz DOCSIS channels."
An Neq-channel per RF port FDX node MUST comply with all requirements operating with all Neq channels on the
RF port and MUST comply with all requirements for an Neq'-channel per RF port device operating with Neq' active
channels on the RF port for all values of Neq' less than Neq (which is at least 185) down to Neq' = 96.
These specifications assume that the FDX node will be terminated with a 75 Ohm load.

7.2.5.9.1    FDX Node Output Electrical Requirements
Multiple operators use a mid-split (5 MHz to 85 MHz/108 MHz to 1218 MHz) spectrum arrangement in
DOCSIS 3.1 fiber deep deployments. These operators are expected to prefer to use the CFDX frequency plan for


06/27/25                                          CableLabs                                                   77
<!-- PDF_PAGE: 78 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


DOCSIS 4.0 FDX. The nominal node RF output power practiced in such deployments is 58 dBmV per 6 MHz
equivalent channel at the upper edge of the DS active spectrum (6 MHz centered at 1215 MHz), and 37 dBmV per
6 MHz equivalent channel at the lower edge of the DS active spectrum (6 MHz centered at 111 MHz), forming a
linear tilt of 21 dB across the active DS spectrum. This is defined as the downstream reference PSD.
This sums up to 73.8 dBmV total composite power, and a power slope of approximately 1.89 dB per 100 MHz. Due
to the power tilt, much of this power is concentrated at the upper edge of the spectrum. For example, if a full OFDM
channel is used between 1026 MHz and 1218 MHz, that channel power is 71.4 dBmV, where the power of all other
channels between 108 MHz and 1026 MHz sums up to only 70.1 dBmV.
Implementation of FDX in the node is associated with more insertion loss between the hybrid power amplifier and
the node port. This additional loss is mainly associated with replacement of a diplex filter with a US/DS combiner or
directional coupler, and with the addition of another directional coupler required for implementation of echo
cancellation. Due to the additional insertion loss, achievement of the above-mentioned node output power level is
beyond the available hybrid power amplifiers technology as of the time of drafting these specifications. The scheme
outlined below is a compromise intended to reduce the total composite power at the node output to 72 dBmV or
lower, while maintaining the power level and tilt seen by legacy devices (set-tops and pre-DOCSIS 3.1 modems)
capable of receiving channels up to ~1 GHz. It is envisioned that when hybrid power amplifiers technology is
sufficiently improved, this compromise can be annulled in a future release of these specifications.
In order to reduce the total composite output power of an FDX node to 72 dBmV, the power level of the upper edge
of the active DS spectrum is reduced. This is implemented as a single down step in the power per 6 MHz equivalent
channel, while maintaining the same power slope of approximately 1.89 dB per 100 MHz across the active DS
spectrum. The power level at the upper portion of the spectrum prior to applying the down step is termed virtual
power, and the power level after the step is applied is termed actual power. Since different deployments are likely to
use various arrangements of OFDM channels at the upper portion of the spectrum, and since a power level step
cannot be implemented inside the encompassed spectrum boundaries of an OFDM channel, some flexibility is
required in setting the frequency of the power down-step. Accordingly, the step size required is also variable, since it
has to assure that the total composite power is at or below 72 dBmV. For example, reducing the top 192 MHz of
spectrum by 4 dB achieves that goal. A more extreme example is muting the spectrum above 1122 MHz (the 16 top
6 MHz equivalent channels).
The FDX node requirements use the defined downstream reference PSD with a 21 dB linear tilt between the 6 MHz
equivalent channel centered at 111 MHz and the 6 MHz equivalent channel centered at 1215 MHz, where the power
of the 6 MHz equivalent channel centered at 111 MHz is 37 dBmV, and the power of the 6 MHz equivalent channel
centered at 1215 MHz is 58 dBmV.
The FDX node MUST support a minimum TCP of 72 dBmV between 108 MHz and 1218 MHz.
The FDX node SHOULD support a minimum TCP of 73.8 dBmV between 108 MHz and 1218 MHz.
The FDX node MUST support setting a single power down-step at a frequency on a channel boundary between 1002
MHz and 1122 MHz at Interface D, and with a depth assuring that the maximum TCP (a minimum of 72 dBmV) is
achieved. The downstream power profile is shown in Figure 13. If the FDX node is generating a TCP of 73.8
dBmV, then no step down is required because the needed step down is 0 dB.
If the FDX node is operating at a TCP of 71 dBmV, then the FDX node supports the more stringent fidelity
requirements detailed in Table 18 and Table 20.
The FDX node MAY support setting a single power down-step at a frequency on a channel boundary below 1002
MHz.




78                                                  CableLabs                                              06/27/25
<!-- PDF_PAGE: 79 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627




                     Figure 13 - Downstream Power Profile with D dB Step-down at fD MHz
The power per 6 MHz equivalent channel for each of the 185 channels between 108 MHz and 1218 MHz is:




Where i is the index of the channel in the DS spectrum (not STD channel number), and Step(i) is given by:



The TCP in a band extending from fstart (MHz) to fstop (MHz) with a D dB drop down at frequency fD (MHz) is a
virtual power setting as illustrated in Figure 14 and described above, and is given by the following formula:




The required drop down in power at to achieve a TCP of 71 dBmV and 72 dBmV is given in Table 17.
A generalization of the calculation of TCP for any given frequency range and tilt for any number of step-downs is
derived in Appendix IV. For example, the above equation is derived from the generalized calculation below Figure
37 (in Appendix IV).
Downstream channel power measurements are made on a tilt-corrected version of the FDX node output at Interface
D. A mathematical tilt correction (calculated as linear down-tilt vs. frequency) is applied at Interface D after which
downstream power measurements are made on the resultant "flat" spectrum. The output of the mathematical
adjustment where power measurements are conducted is referred to as Interface Dˊ, as shown in Figure 14.
This flattening is calculated by subtracting the reference PSD and the commanded per channel attenuation from the
measured PSD providing a nominal 0 dB flat spectrum.
Measuring channel power adjustment accuracy across the spectrum is calculated by subtracting the reference PSD
and the commanded per channel attenuation (including both step down and additional per channel adjustment if
supported by the FDX node) from the measured PSD providing a nominal 0 dB flat spectrum.




06/27/25                                            CableLabs                                                     79
<!-- PDF_PAGE: 80 -->
CM-SP-PHYv4.0-I07-250627                                     Data-Over-Cable Service Interface Specifications




                    Figure 14 - FDX Node Downstream Power Measurement at Interface D
As explained above, for an FDX node not to exceed its maximum downstream TCP limits, the node can support a
single power down-step at a frequency on a channel boundary between 1002 MHz and 1122 MHz. This down step
capability is not required if the FDX node is capable of generating a TCP of at least 73.83 dBmV. The FDX node
will report its maximum downstream TCP limits to the CMTS, in addition to the stepdown limits supported.
              Table 17 - Required Power Drop-down to Achieve Target TCP of 71 and 72 dBmV

                Drop Frequency                 Drop for 72 dBmV               Drop for 71 dBmV
                       1002                            3.5                            6.5
                       1008                            3.6                            6.7
                       1014                            3.7                            7.0
                       1020                            3.8                            7.4
                       1026                            3.9                            7.8
                       1032                            4.1                            8.2
                       1038                            4.2                            8.8
                       1044                            4.4                            9.5
                       1050                            4.6
                       1056                            4.8
                       1062                             5
                       1068                            5.3
                       1074                            5.6
                       1080                            5.9
                       1086                            6.4
                       1092                            6.9
                       1098                            7.6
                       1104                            8.5
                       1110                            9.8




80                                               CableLabs                                          06/27/25
<!-- PDF_PAGE: 81 -->
Physical Layer Specification                                                           CM-SP-PHYv4.0-I07-250627


An FDX node MAY support individual channel power adjustment, which changes each individual channel PSD to
vary from the downstream reference PSD. A compliant FDX node is not required to support Channel Power
Adjustment, with the exception of a power step-down described previously. In this case of individual channel power
adjustment capability, the channel commanded power for a channel at Interface D is, by definition, the Channel
Power Adjust, and is relative to the downstream reference PSD. The power per channel is either 0 dB if it is on the
downstream reference PSD or the Channel Power Adjustment value is positive if it is higher than the downstream
reference PSD and negative if it is lower than the downstream reference PSD.
An FDX node which supports individual Channel Power Adjustment will report its capability (including range) to
the CMTS.
For OFDM, all modulated subcarriers in an OFDM channel are set to the same average power (except pilots which
are boosted by 6 dB). For purposes of spurious emissions requirements, the "commanded transmit power per
channel" for an equivalent legacy DOCSIS channel is referenced to Interface Dˊ, and is computed as follows:
    •   FDX node power is configured by power per CTA channel [CTA 542] and number of occupied CTA
        channels for each OFDM channel.
    •   For each OFDM channel, the total power is Power per CTA channel + 10log10(Number of occupied CTA
        channels) for that OFDM channel.
    •   CMTS Core calculates power for data subcarrier and pilots (using total number of non-zero valued (non-
        excluded) subcarriers).
    •   FDX node calculates power in 6 MHz containing PLC.
    •   For the spurious emissions requirements, power calculated for the 6 MHz containing the PLC is the
        commanded average power of an equivalent DOCSIS legacy channel for that OFDM channel.
An FDX node MUST output an OFDM RF modulated signal with the characteristics defined in Table 18 - RF
Output Electrical Requirements, Table 19 - FDX Node Output Power, and Table 20 - FDX Node Output Out-of-
Band Noise and Spurious Emissions Requirements.
The condition for these requirements is all Neq' combined channels, legacy DOCSIS channels and equivalent legacy
DOCSIS channels, commanded to the same average power, except for the single channel active phase noise,
diagnostic carrier suppression, OFDM phase noise, OFDM diagnostic suppression, and power difference
requirements, and except as described for out-of-band noise and spurious requirements.
                                    Table 18 - RF Output Electrical Requirements

                        Parameter                                                      Value
Downstream lower edge band of an FDX node               108 MHz.
                                                        (See Item #1 immediately following this table.)
                                                        (See Item #2 following this table.)
Downstream upper edge band of an FDX node               1218 MHz (required)
                                                        (See Item #3 following this table.)
Level                                                   See requirements detailed above in this section.
Modulation type                                         See Section 7.2.4.5
OFDM channels' subcarrier spacing                       25 kHz and 50 kHz
Inband Spurious, Distortion, and Noise
1110 MHz total occupied bandwidth,
6 MHz gap (Internal Excluded subcarriers)
185 equivalent 6 MHz channels.
See Notes 4, 6, 10

For measurements below 684 MHz                          ≤ -39 dBr Average over center 400 kHz within gap

For measurements from 684 MHz to 1002 MHz               ≤ -38 dBr Average over center 400 kHz within gap

For measurements 1002 MHz to 1218 MHz                   ≤ -37 dBr Average over center 400 kHz within gap

For frequencies above the stepdown frequency            0.8 dB relaxation per dB step down value




06/27/25                                           CableLabs                                                  81
<!-- PDF_PAGE: 82 -->
CM-SP-PHYv4.0-I07-250627                                               Data-Over-Cable Service Interface Specifications


                          Parameter                                                              Value
MER in 1110 MHz total occupied bandwidth,
185 equivalent 6 MHz channels.
See Notes 2, 4, 5, 6

For measurements below 684 MHz                                    ≥ 37 dB Any single subcarrier. See Notes 1 and 11
                                                                  ≥ 39 dB Average over the complete OFDM channel.
                                                                  See Notes 1 and 11

For measurements from 684 MHz to 1002 MHz                         ≥ 36 dB Any single subcarrier. See Notes 1 and 11
                                                                  ≥ 38 dB Average over the complete OFDM channel.
                                                                  See Notes 1 and 11

For measurements 1002 MHz to 1218 MHz                             ≥ 35 dB Any single subcarrier. See Notes 1 and 11
                                                                  ≥ 37 dB Average over the complete OFDM channel.
                                                                  See Notes 1,9 and 11.

MER For frequencies above the stepdown frequency                  0.8 dB relaxation per dB step down value

                                                                  Minimal test receiver equalization: See Note 7
                                                                   2 dB relief for above requirements
                                                                   (e.g., MER > 39 dB becomes MER > 37 dB)
Phase Noise, double sided maximum:
Fully loaded spectrum 108 MHz to 1218 MHz with 24 MHz as
exclusion sub-band centered at 999 MHz, + 6 dBc CW in
center of the exclusion sub-band.
+6 dBc CW is relative to the downstream reference PSD over
6 MHz.
(CW not processed via FFT)                                        1 kHz - 10 kHz: -48 dBc
See Note 6.                                                       10 kHz - 100 kHz: -55 dBc
Output Impedance                                                  75 Ohms
Output Return Loss (Note 3)                                       > 10 dB from 108 MHz to 1218 MHz
Connector                                                         5/8-24 port, Female Adapter [SCTE 91]
TABLE NOTES
NOTE 1      Receiver channel estimation is applied in the test receiver; test receiver does best estimation possible. Transmit
            windowing is applied to potentially interfering channel and selected to be sufficient to suppress cross channel
            interference.
NOTE 2      MER (modulation error ratio) is determined by the cluster variance caused by the transmit waveform at the output of the
            ideal receive matched filter. MER includes all discrete spurious, noise, subcarrier leakage, clock lines, synthesizer
            products, distortion, and other undesired transmitter products. Phase noise up to ±50 kHz of the subcarrier is excluded
            from inband specification, to separate the phase noise and inband spurious requirements as much as possible. In
            measuring MER, record length or carrier tracking loop bandwidth may be adjusted to exclude low frequency phase
            noise from the measurement. MER requirements assume measuring with a calibrated test instrument with its residual
            MER contribution removed.
NOTE 3      Frequency ranges are edge-to-edge.
NOTE 4      Phase noise up to 10 MHz offset is mitigated in test receiver processing or by test equipment (latter using hardline
            carrier from modulator, which requires special modulator test port and functionality).
NOTE 5      Up to 5 subcarriers in one OFDM channel can be excluded from this requirement.
NOTE 6      For test purposes full loading is defined as 3 OFDM channels 108 MHz to 684 MHz, 25 SC-QAM channels 684 MHz to
            834 MHz, and 2 OFDM channels 834 MHz to 1218 MHz, and with the power drop necessary to achieve the total
            composite power requirement.
NOTE 7      The estimated channel impulse response used by the test receiver is limited to half of length of smallest transmit cyclic
            prefix.
NOTE 8      A single subcarrier in the OFDM channel can be excluded from this requirement, no windowing is applied, and
            minimum CP is selected.
NOTE 9      This is the performance with the downstream reference PSD. It is not required for the FDX node to transmit at this level.
            A compliant device will incorporate a power step down and the MER requirement is adjusted from downstream
            reference PSD requirement according to 0.8 dB per dB of power step down.
NOTE 10 This is the minimum target in-band distortion with a downstream TCP of 72 dBmV. If the FDX node is operating with a
        TCP of 71 dBmV, then 2 dB is added to each requirement value, for example -39 dBr becomes -41 dBr.




82                                                         CableLabs                                                      06/27/25
<!-- PDF_PAGE: 83 -->
Physical Layer Specification                                                           CM-SP-PHYv4.0-I07-250627


NOTE 11 This is the minimum target MER with a downstream TCP of 72 dBmV. If the FDX node is operating with a TCP of 71
        dBmV, then 2 dB is added to each requirement value, for example 39 dB becomes 41 dB.



7.2.5.9.1.1   Power per Channel for FDX Node
FDX nodes perform the modulation of channels which are ordinarily generated by EQAMs and CMTSs at the
headend.
Control over an FDX node's electrical output is required for many of the characteristics, such as RF channel power,
number of RF channels, modulation characteristics of the channels, center frequency of channels, and so forth. Two
distinct mechanisms of control can exist for an FDX node. One mechanism of control is via commands carried in the
downstream link into the FDX node. A second mechanism of control is "local-only", separate from the downstream
link into the FDX node, such as an electrical interface operable at installation or even pluggable components set at
installation. In an FDX node some adjustable characteristics can be controlled by one mechanism, and not the other,
or by both; therefore, some "adjustable" characteristics can perhaps not be remotely changed. Local-only
adjustments made at installation can be subsequently amended, but not remotely, and could incur service
interruption.
An FDX node is capable of generating 185 equivalent legacy DOCSIS channels onto the RF port at Interface D, so
Neq = 185 for a compliant FDX node.
An FDX node has to be adjustable to operate with fewer than Neq = 185 channels on its RF port, for all Neq' down to
Neq' 96 channels. The FDX node has to comply with all requirements operating with all Neq = 185 channels on the
RF port, and has to comply with all requirements operating with Neq' channels on the RF port for all values of Neq'
less than Neq that it supports.
These specifications assume that the FDX node will be terminated with a 75 ohm load.
The FDX node MUST support setting PTCP_actual(0), the measured TCP at the node output immediately after the last
configuration change, in the range of 70 dBmV to 72 dBmV, for 1110 MHz of modulated spectrum.
The FDX node MUST accept a sufficiently large range of PTCP_config, the configuration parameter for the FDX node
TCP, to ensure that PTCP_actual(0) can be set over the 70 dBmV to 72 dBmV required range, even at the worst case
actual absolute power inaccuracy of the FDX node. For example, with -2.5 dB absolute accuracy, PTCP_config will be
74.5 dBmV to achieve PTCP_actual(0) of 72 dBmV.
The FDX node MUST comply with the fidelity spec Table 19 - FDX Node Output Power, when PTCP_actual(0) is
in the range of 70 dBmV to 72 dBmV. The FDX node does not need to comply with the fidelity spec when
PTCP_actual(0) is outside the range of 70 dBmV to 72 dBmV. If PTCP_actual(0) is within the range of 70 dBmV to 71
dBmV, the FDX node is expected to meet the requirements associated with 71 dBmV TCP. If PTCP_actual(0) is within
the range of >71 dBmV to 72 dBmV, the FDX node is expected to meet the requirements associated with 72 dBmV
TCP.
The FDX node MUST maintain power stability (over time and its specified operating temperature range) such that
        |PTCP_actual(t) - PTCP_actual(0)| ≤ 1 dB
        where, PTCP_actual(t) is the measured TCP at the node output any time after PTCP_actual(0) was last set.
        Note that the fidelity spec has to be met according to PTCP_actual(0) and not PTCP_actual(t).
NOTE: When the modulated spectrum is less than 1110 MHz, the TCP will drop but the power spectral density of
      the active channels is expected to remain within the above specified limits.
NOTE: The above requirements apply to slow variations on the FDX node TCP.
                                         Table 19 - FDX Node Output Power

               For Neq' (number of active channels combined per RF port) in the range 96 to 185
                       Parameter                                                       Value
 Power difference between any two adjacent 6 MHz equivalent   ≤ 0.5 dB
 channels in the 108 MHz to1218 MHz downstream spectrum
 (with commanded tilt mathematically removed and accounting
 for pilot density variation and subcarrier exclusions)




06/27/25                                             CableLabs                                                          83
<!-- PDF_PAGE: 84 -->
CM-SP-PHYv4.0-I07-250627                                        Data-Over-Cable Service Interface Specifications


                For Neq' (number of active channels combined per RF port) in the range 96 to 185
                         Parameter                                                        Value
 Power difference between any two non-adjacent 6 MHz          ≤ 1 dB
 equivalent channels in a 48 MHz contiguous bandwidth block
 (with commanded tilt mathematically removed and accounting
 for pilot density variation and subcarrier exclusions)
 Power difference between any two 6 MHz equivalent            ≤ 2 dB
 channels in the 108 MHz to 1218 MHz downstream spectrum
 (with commanded tilt mathematically removed and accounting
 for pilot density variation and subcarrier exclusions)
 Power per channel absolute accuracy                          ±3 dB at 25oC
 RF output composite power stability                          ± 1 dB over time and temperature relative to time immediately
                                                              after last configuration change.
 Power per channel stability                                  ± 1 dB over time and temperature relative to RF output
                                                              composite power.
 Single Channel Suppression                                   Channel suppression within the occupied bandwidth
                                                              The FDX node is required to control transmissions such that
                                                              when it suppresses a channel the FDX node output complies
                                                              with the spurious emissions and noise requirements in the gap
                                                              formed by suppressing the channel, as specified in Table 18.
                                                              No service interruption or detriment



7.2.5.9.1.2    Out-of-Band Noise and Spurious Requirements for FDX Node
In [DOCSIS DRFI] and [DOCSIS PHYv3.1], the targeted system architectures allowed other downstream signals to
be combined or added to the compliant downstream modulator's output (now referenced as Interface C). As a result,
there were stringent requirements for out-of-band spurious emissions so that the compliant downstream modulator's
signal will combine with other signals and provide suitable signal-to-noise ratio in the spectrum containing the
combined signals. With the FDX node downstream modulator the requirements are referenced to Interface D (or
Interface Dˊ), and at this port all downstream signals are present and there is no subsequent combining, so there is
less need for the out-of-band spurious requirements that exist for the CMTS and the remote node Interface C.
However, there are still requirements for spurious emissions for spectrum besides the downstream spectrum, such as
transition bands and legacy upstream spectrum. Also, there are still requirements for spurious emissions in
downstream spectrum that is suppressed, and exclusion sub-bands and gaps in the encompassed spectrum cannot
have unconstrained spurious emissions. These requirements are much simpler in form than the [DOCSIS DRFI] and
[DOCSIS PHYv3.1] spurious emissions requirements because Neq is a fixed number for the FDX node downstream
modulator.
In [DOCSIS DRFI] and [DOCSIS PHYv3.1], as the amount of modulated spectrum is increased in the compliant
device, the spurious emissions requirements allow more power within a given measurement bandwidth (relative to
the signal power spectral density). This is not the case for the FDX node requirements. Also, unlike the FDX node
downstream requirements, the signal PSD for DRFI and DOCSISv3.1 is applied with a flat modulated PSD, while
the FDX node generates a 21 dB uptilt (when operating with the maximum modulated spectrum, 108 MHz to 1218
MHz). The Interface D output incorporates a significant power amplifier which is not incorporated in the Interface C
requirements of the earlier requirements ([DOCSIS DRFI] and [DOCSIS PHYv3.1]). For these reasons, the FDX
node Interface D fidelity requirements are not as high fidelity as for the earlier DOCSIS downstream modulators.
The out-of-band spurious emissions requirements at Interface C have served as a rough reference and bound (of
sorts) for MER performance; there are additional contributors to MER, but on the other hand, performance can be
better than the requirement, so the out-of-band spurious emissions requirements are not a rigorous bound for MER.
Since out-of-band spurious emissions requirements include a distortion contribution, spurious emissions relaxation
is provided in gaps within modulated spectrum for measurements in gaps below 684 MHz. For measurements above
684 MHz there is no additional relaxation for gaps; the relaxation provided for higher frequency (above 684 MHz) is
already generous.
With the aforementioned listed considerations, it is informative to review the "protection" for digital signals from
DRFI and DOCSIS 3.1 downstream modulators provided at Interface C with the modulated spectrum of 108 MHz to


84                                                   CableLabs                                                   06/27/25
<!-- PDF_PAGE: 85 -->
Physical Layer Specification                                                        CM-SP-PHYv4.0-I07-250627


1218 MHz (185 equivalent legacy SC-QAM DOCSIS channels), and with 960 MHz modulated spectrum (160
equivalent legacy SC-QAM DOCSIS channels). Also note that some of the DOCSIS 3.1 downstream modulator
fidelity requirements specify 576 MHz modulated spectrum (96 equivalent DOCSIS channels).
With all digital channels at the same equivalent power per 6 MHz channel at Interface C, the [DOCSIS DRFI] and
[DOCSIS PHYv3.1] spurious emissions specifications provide for 51 dB SNR protection for digital channels below
600 MHz with transmissions of 960 MHz modulated spectrum (160 equivalent legacy DOCSIS channels). With the
additional 1 dB relaxation within a gap, 50 dB SNR protection is provided below 600 MHz in a gap within the
downstream modulated spectrum. The SNR protection is 48 dB between 600 MHz and 1002 MHz for digital
channels operating with 960 MHz occupied bandwidth, or in a gap within the encompassed spectrum within that
frequency range, generated by a DOCSIS 3.1-compliant device. The SNR protection is 46 dB between 1002 MHz
and 1218 MHz for digital channels operating with 960 MHz occupied bandwidth, or in a gap within the
encompassed spectrum within that frequency range, generated by a DOCSIS 3.1-compliant device.
There is an additional 0.63 dB lower SNR when the modulated spectrum corresponds to 185 equivalent legacy SC-
QAM DOCSIS channels, instead of 160, in [DOCSIS DRFI] and [DOCSIS PHYv3.1].
With all digital channels at the same equivalent power per 6 MHz channel at Interface C, the [DOCSIS DRFI] and
[DOCSIS PHYv3.1] spurious emissions specifications provide for 50 dB SNR protection for digital channels below
600 MHz with transmissions of 1110 MHz modulated spectrum (185 equivalent legacy DOCSIS channels). With the
additional 1 dB relaxation within a gap, 49 dB SNR protection is provided below 600 MHz in a gap within the
downstream modulated spectrum. The SNR protection is 47 dB between 600 MHz and 1002 MHz for digital
channels operating with 1110 MHz occupied bandwidth, or in a gap within the encompassed spectrum within that
frequency range, generated by a DOCSIS 3.1-compliant device. The SNR protection is 45 dB between 1002 MHz
and 1218 MHz for digital channels operating with 1110 MHz occupied bandwidth, or in a gap within the
encompassed spectrum within that frequency range, generated by a DOCSIS 3.1-compliant device.
With all digital channels at the same equivalent power per 6 MHz channel at Interface C, the [DOCSIS DRFI] and
[DOCSIS PHYv3.1] spurious emissions specifications provide for 53 dB SNR protection for digital channels below
600 MHz with transmissions of 576 MHz modulated spectrum (96 equivalent legacy DOCSIS channels). With the
additional 1 dB relaxation within a gap, 52 dB SNR protection is provided below 600 MHz in a gap within the
downstream modulated spectrum. The SNR protection is 50 dB between 600 MHz and 1002 MHz for digital
channels operating with 576 MHz occupied bandwidth, or in a gap within the encompassed spectrum within that
frequency range, generated by a DOCSIS 3.1 compliant device. The SNR protection is 48 dB between 1002 MHz
and 1218 MHz for digital channels operating with 576 MHz occupied bandwidth, or in a gap within the
encompassed spectrum within that frequency range, generated by a DOCSIS 3.1 compliant device.
The In-band Spurious, Distortion, and Noise requirements specified in Section 7.2.5.9.1 (at Interface C or
equivalent), with 96 equivalent legacy SC-QAM DOCSIS channels for modulated spectrum (576 MHz) has
requirements for 50 dBr, 47 dBr, and 45 dBr, for the three frequency bands, which corresponds to 2 dB and 3 dB
and 3 dB reduction from the spurious emissions requirements in the same bands, respectively, within a gap. With the
same margins applied to the spurious emissions requirements with 185 equivalent legacy SC-QAM DOCSIS
channels, the In-band Spurious, Distortion, and Noise requirements at Interface C would become 47 dBr, 44 dBr,
and 43 dBr, respectively.
For the FDX node downstream modulator, the requirements allow reduced fidelity compared to Interface C of
previous specifications, for reasons cited above. For the 72 dBmV (71 dBmV) TCP the following remarks apply
regarding the SNR "protection" for digital channels. As observed at Interface Dˊ (which corresponds to 185 equivalent
DOCSIS channels of modulated spectrum, or less), the SNR "protection" for digital channels below 684 MHz,
within a gap, is 39 dB (41 dB) (it is 49 dB at Interface C in DOCSIS 3.1 below 600 MHz); for digital channels
between 684 MHz and 1002 MHz, within a gap, is 38 dB (40 dB) (it is 47 dB at Interface C in DOCSIS 3.1 from
600 MHz to 1002 MHz); for digital channels between 1002 MHz and 1218 MHz, within a gap, is 37 dB (39 dB) (it
is 45 dB at Interface C in DOCSIS 3.1 between 1002 MHz and 1218 MHz). The In-band Spurious, Distortion, and
Noise requirements at Interface Dˊ are 39 dB (41 dB), 38 dB (40 dB), and 37 dB (39 dB) in the respective frequency
bands (below 684 MHz; between 684 MHz and 1002 MHz; and between 1002 MHz and 1218 MHz).




06/27/25                                           CableLabs                                                    85
<!-- PDF_PAGE: 86 -->
CM-SP-PHYv4.0-I07-250627                                          Data-Over-Cable Service Interface Specifications


            Table 20 - FDX Node Output Out-of-Band Noise and Spurious Emissions Requirements

               For Neq' (Number of Active Channels Combined per RF Port) in the Range 96 to 185
                          Condition                                                Requirement (in dBr)
 1110 MHz total occupied bandwidth down to 576 MHz total
 occupied bandwidth,
 6 MHz measurement interval outside modulated spectrum
 See Note 1

 For measurements below 684 MHz                                 ≤ -39 dBr Average over 6 MHz, 72 dBmV TCP
                                                                ≤ -41 dBr Average over 6 MHz, 71 dBmV TCP

 For measurements from 684 MHz to 1002 MHz                      ≤ -38 dBr Average over 6 MHz, 72 dBmV TCP
                                                                ≤ -40 dBr Average over 6 MHz, 71 dBmV TCP

 For measurements from1002 MHz to 1218 MHz                      ≤ -37 dBr Average over 6 MHz, 72 dBmV TCP
                                                                ≤ -39 dBr Average over 6 MHz, 71 dBmV TCP

 For frequencies above the stepdown frequency                   0.8 dB relaxation per dB step down value

                                                                For frequencies between channels with different PSD the zero
                                                                dBr reference is the channel with the highest PSD.
 TABLE NOTE
 NOTE 1 Loading is defined as 3 OFDM channels 108 MHz to 684 MHz, 25 SC-QAM channels 684 MHz to 834 MHz, and
        2 OFDM channels 834 MHz to 1218 MHz, and with the power drop necessary to achieve the total composite power
        requirement; and any number and combination of reduced modulated spectrum down to 576 MHz modulated spectrum.
        Different types of channels and locations of channels can also be tested and meet the requirements, as long as the gap
        ratio requirements are satisfied.



7.2.5.10 Independence of Individual FDX Channels Within Multiple Channels on a Single RF Port
         for 1218 MHz Operation
The requirements of this section apply only to FDX nodes operating with the CFDX frequency plan and downstream
signals up to 1218 MHz. The reference point for downstream measurements for such devices is Interface D (see
section 7.2.1). For FDX downstream transmitter operation with the CFDX frequency plan and downstream signals
present up to 1794 MHz, or with the SFDX frequency plan, see section 7.2.5.10.2.
For FDX devices operating in FDX mode, this section augments section 7.5.10 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
The CMTS output OFDM channel characteristics are collected in section 7.5.10 of [DOCSIS PHYv3.1]. The CMTS
requirements in section 7.5.10 of [DOCSIS PHYv3.1] apply to the FDX node with the exception of the following:
     •   The FDX node MUST support 86 MHz minimum contiguous modulated OFDM bandwidth per channel in
         the full duplex band.
     •   An FDX node MUST provide for 1 mode of carrier suppression of RF power for diagnostic and test
         purposes. See Table 19 for mode descriptions and carrier RF power suppression level instead of table 42 as
         referenced in item 2 and item 7 in section 7.5.10 of [DOCSIS PHYv3.1].
     •   Instead of item 4 in [DOCSIS PHYv3.1], section 7.5.10, an FDX node MUST provide the ratio of amount
         of modulated spectrum to gap spectrum in the encompassed spectrum between 108 MHz and 1218 MHz
         being at least 2:1, and with each channel independently meeting the DOCSIS 3.1 requirements in Section
         7.2.5.9.1, and requirements for DRFI in Section 6.3.5, except for fidelity requirements.
     •   Instead of item 4 in [DOCSIS PHYv3.1], section 7.5.10, an FDX node MUST meet the requirements in
         Section 7.2.5.9.1, and requirements for DRFI in Section 6.3.5, when the ratio amount of modulated
         spectrum to gap spectrum in the encompassed spectrum is at least 4:1. (A ratio of amount of modulated
         spectrum to gap spectrum of at least 4:1 provides that at least 80% of the encompassed spectrum contains
         modulated spectrum, and the amount of gap spectrum is at most 20% of the encompassed spectrum.)



86                                                     CableLabs                                                  06/27/25
<!-- PDF_PAGE: 87 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627


Instead of item 6 in [DOCSIS PHYv3.1], section 7.5.10, an FDX node MUST provide a test mode of operation, for
out of service testing, configured for 1076 MHz of modulated spectrum (i.e., 1100 MHz with a 24 MHz gap in an
OFDM channel centered on approximately 900 MHz plus a CW tone as described here). Centered within the 24
MHz gap there is a CW tone which is 6 dB higher power than the power in 6 MHz of the downstream modulated
spectrum at the downstream reference PSD, as measured at Interface D.
An FDX node generation of the CW test tone SHOULD exercise the signal generation chain to the fullest extent
practicable, in such manner as to exhibit phase noise characteristics typical of actual operational performance. One
purpose of this test mode is to support one method for testing the phase noise requirements of Table 18.

7.2.5.10.1    FDX Downstream Fidelity Requirements for 1794 MHz Operation
The requirements of this section apply to FDX RPDs/RMDs operating with downstream signals up to 1794 MHz or
with the CFDX frequency plan. The reference point for downstream measurements for such devices is Interface C
(see section 7.2.1).
An FDX RPD/RMD operating with the CFDX frequency plan and downstream signals above 1218 MHz and up to
1794 MHz MUST adhere to the requirements of section 7.3.5.9 and underlying subsections, with "FDX RPD/RMD"
substituted for "FDD RPD/RMD."
An FDX RPD/RMD operating with the SFDX frequency plan MUST adhere to the requirements of section 7.3.5.9
and underlying subsections with "FDX RPD/RMD" substituted for "FDD RPD/RMD."

7.2.5.10.2    Independence of FDX Channels Within Multiple Channels on a Single RF Port for 1794 MHz
              Operation
The requirements of this section apply to FDX RPDs/RMDs operating with downstream signals up to 1794 MHz or
with the CFDX frequency plan. The reference point for downstream measurements for such devices is Interface C
(see section 7.2.1).
An FDX RPD/RMD operating with the CFDX frequency plan and downstream signals above 1218 MHz and up to
1794 MHz MUST adhere to the requirements of section 7.3.5.10, with "FDX RPD/RMD" substituted for "FDD
RPD/RMD."
An FDX RPD/RMD operating with the SFDX frequency plan MUST adhere to the requirements of section 7.3.5.10,
with "FDX RPD/RMD" substituted for "FDD RPD/RMD."

7.2.5.11 FDX Cable Modem Receiver Input Requirements
For FDX devices operating in FDX mode, this section augments section 7.5.11 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
The FDX CM MUST be able to accept any range of OFDM subcarriers defined between Lower Frequency
Boundary and Upper Frequency Boundary simultaneously.
Active subcarrier frequencies, loading, and other OFDM characteristics are described by OFDM configuration
settings and CM exclusion bands and profile definition. The OFDM signals and CM interfaces will have the
characteristics and limitations defined in Table 21.
The FDX CM MUST support the requirements in Table 21 - Electrical Input to FDX CM, which supersede the
corresponding requirements in [DOCSIS PHYv3.1], table 45 - Electrical Input to CM, unless otherwise noted.
                                       Table 21 - Electrical Input to FDX CM

                                          Parameter                                   Value
                   FDX downstream lower band edge CFDX frequency plan          108 MHz
                   FDX downstream lower band edge SFDX frequency plan          258 MHz
                   Number of FFT blocks                                        Refer to Table 7




06/27/25                                           CableLabs                                                     87
<!-- PDF_PAGE: 88 -->
CM-SP-PHYv4.0-I07-250627                                     Data-Over-Cable Service Interface Specifications


7.2.5.12 FDX Cable Modem Receiver Capabilities
For FDX devices operating in FDX mode, this section augments section 7.5.12 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
Acceptable downstream performance for a cable modem is defined with respect to a packet error ratio (PER) of 10-6.
A packet is taken as a 1500-byte Ethernet packet. To satisfy the downstream PER performance requirement, the
FDX CM MUST achieve a PER of 10-6 for CNR values not exceeding those given by Table 22 - CNR Performance
Requirement of an FDX CM for External-ACI Test and Table 23 - CNR Performance Requirement of FDX CM for
Self ACI Test for the tests described below.
External-ACI test: This corresponds to the case in which the CM under test is receiving an FDX channel but not
transmitting upstream in the FDX band. Other CMs in the IG are transmitting upstream in other FDX channels. CMs
in other IGs are also transmitting upstream in the FDX band under test. The purpose of this is to test the CM
performance under time varying ACI, CCI and ALI (Adjacent Leakage Interference).
Self-ACI test: This is designed to test the echo cancellation performance of the FDX CM. This corresponds to the
case in which CM under test is receiving in one FDX channel while transmitting upstream in the other two FDX
channels.
Above FDX transition band test: This corresponds to the case in which the CM under test is receiving an OFDM
channel that is not located in the FDX band or in the FDX transition band.

7.2.5.12.1 Conditions Common to Both ACI Tests
This subsection defines the conditions that are common to both previously mentioned tests.
     •   CMTS downstream transmission for the channel under test will be an FDX band with a modulated
         spectrum of 190 MHz.
     •   Although the central FDX channel is shown as the channel under test in Figure 16 and Figure 17, tests have
         to be performed for all three FDX channels and the worst case CNR that gives a PER of 10-6 has to be used
         to validate the performance requirement.
     •   This transmission will consist of any valid combination of the following downstream parameters: subcarrier
         spacing, cyclic prefix size, transmitter window, PLC location, number of profiles, codeword size, NCP
         QAM constellation (but the QAM constellation of the NCP will not be greater than the QAM constellation
         used by the data subcarriers).
     •   The objective of the test is to identify the lowest CNR value that gives a PER of 10-6 for every valid QAM
         constellation for every combination of the previous parameters, and to compare this with corresponding
         entry in Table 22 and Table 23.
     •   Depth of time interleaving will be set to 12 for 50 kHz subcarrier spacing and 6 for 25 kHz subcarrier
         spacing.
     •   Power spectral density of the downstream transmission, measured as the power at CM input per 6 MHz
         (P6AVG), will be set to 0 dBmV per 6 MHz over the modulated spectrum of the channel under test. The Self-
         ACI test includes a second case in which P6AVG is set to 3 dBmV per 6 MHz.
     •   Downstream spectrum within and outside the FDX band, i.e., from 108 MHz to 1218 MHz for the CFDX
         frequency plan or from 258 MHz to the downstream upper band edge for the SFDX frequency plan will be
         fully loaded with FDX and DOCSIS 3.1 channels with the same power spectral density as the FDX channel
         under test.
     •   Test modulator (FDX Signal Generator in Figure 15) phase noise per CMTS downstream modulator spec
         (not the FDX node modulator spec).
     •   The performance will be measured in the steady state mode, i.e., with a static RBA, after channel
         acquisition and after echo canceller training.




88                                                 CableLabs                                                06/27/25
<!-- PDF_PAGE: 89 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


7.2.5.12.2 External ACI Test
The test set-up for the external ACI test is illustrated in Figure 15.




                                    Figure 15 - Test Set-Up for External ACI Test


The channel model for this test consists of the following.
1.   Fixed-ACI: This corresponds to the downstream signal, FDX as well as non-FDX, and has the same power
     spectral density as the OFDM signal in the channel under test. This is made up of FDX and DOCSIS 3.1 OFDM
     downstream channels.
2.   On/Off ACI: This corresponds to upstream transmissions of other CMs in same IG as the CM under test. (Note
     that the CM under test is not transmitting upstream in this test.) These will be in the two FDX channels other
     than the FDX channel under test. This ACI may be introduced in the test set-up using either FDX or DOCSIS
     3.1 OFDM channels.
3.   The linearly up-tilted PSD of on/off ACI (during on-state) has a value as follows: for CFDX, 4 dBmV/6 MHz at
     108 MHz and 10 dBmV/6 MHz at 684 MHz and for SFDX, 4 dBmV/6 MHz at 258 MHz and 10 dBmV/6 MHz
     at 834 MHz.
4.   Fixed AWGN: This is the background noise level. The term "fixed" implies that it is not time-varying during
     the course of a test. However, the level of this is to be varied from test to test until a PER of 10-6 is obtained.
     Channel CNR is adjusted to the required level by measuring the source in-band noise including phase noise
     component along with transmitter noise and distortion and adding the required delta noise from an external
     AWGN generator to achieve the desired CNR at the CM F-connector. Let this AWGN level be defined as
     AWGN0 dBmV per 6 MHz. Since the signal power has been defined as 0 dBmV per 6 MHz, CNR is equal to -
     AWGN0. This CNR is the parameter under test (see Figure 15).
5.   On/off Interference: This corresponds to the combination of ALI from upstream transmissions in adjacent
     channels in the same IG as well as CCI from upstream transmissions in other IGs. The level of this is -44 dBmV
     per 6 MHz during the on-state.
         •    The channel will be periodically switched between state 0 and state 1, shown in Figure 16 and Figure
              17, and as shown in Figure 19. It may be noted that ACI and AWGN are switched on and off at the
              same time.
         •    State 0 corresponds to the situation in which on/off ACI and on/off Interference are in the off state.
              State 1 corresponds to the situation in which on/off ACI and on/off Interference are in the on state.
              AWGN1 of Figure 17 is the additive combination of the AWGN and the interference.




06/27/25                                              CableLabs                                                       89
<!-- PDF_PAGE: 90 -->
CM-SP-PHYv4.0-I07-250627                                        Data-Over-Cable Service Interface Specifications



        PSD                                      CFDX

                             Channel             FDX                 DOCSIS 3.1          Optional support
                 CNR          Under          Downstream             Downstream            for 1794 MHz
                               Test            channel                Channels             downstream

                                          AWGN0                                              AWGN0
                                                                                                             MHz
           108         300             492                684                     1218                1794




        PSD                                      SFDX

                                       Channel             FDX
                                                                             DOCSIS 3.1 Downstream
                       CNR              Under          Downstream
                                                                                    Channels
                                         Test            channel

                                                          AWGN0
                                                                                                             MHz
                 258         450                 642                834                               1794
                             Figure 16 - State 0 for the External ACI Test


        PSD                                      CFDX

                                              Upstream
                                               of other
                                              CM in IG

                             Channel
                                                 FDX                                     Optional support
                              Under                          Non-FDX Downstream
                               Test
                                             Downstream                                   for 1794 MHz
                                                                  Channels                 downstream
                                               channel
                             AWGN1
                                                                                                             MHz
           108         300             492                684                     1218                1794




        PSD                                      SFDX

                                                        Upstream
                                                         of other
                                                        CM in IG

                                       Channel
                                                           FDX
                                        Under
                                         Test
                                                       Downstream         Non-FDX Downstream Channels
                                                         channel
                                       AWGN1
                                                                                                             MHz
                 258         450                 642                834                               1794

                             Figure 17 - State 1 for the External ACI Test




90                                                 CableLabs                                                      06/27/25
<!-- PDF_PAGE: 91 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627




                           Figure 18 - Time-Varying ACI and AWGN for External ACI Test


The CNR requirements to achieve a PER of 10-6 for all valid QAM constellations are given in Table 22.
                  Table 22 - CNR Performance Requirement of an FDX CM for External-ACI Test

                                        Constellation                  CNR (dB)
                                             4096                          NA
                                             2048                          40
                                             1024                          35
                                              512                         31.5
                                              256                         28.5
                                              128                         25.5
                                              64                          22.5
                                              16                          16.5



7.2.5.12.3 Self ACI Test
The test set-up for the Self ACI test is illustrated in Figure 19.




                                       Figure 19 - Test Set-Up for Self ACI Test


    •    The only difference between the channel model of Figure 19 and that of the External-ACI test is the
         inclusion of a cable model. The purpose of this is to introduce micro-reflections to the signal transmitted
         upstream by the CM under test.




06/27/25                                              CableLabs                                                   91
<!-- PDF_PAGE: 92 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


     •   This cable model consists of a tap with 75 to 150 feet length of Series 6 cable from tap to a ground block
         compliant with SCTE 129, followed by up to 10 feet of Series 6 cable to the CM providing a combined
         return loss of 25 dB across the FDX band (108 MHz to 684 MHz for CFDX or 258 to 834 MHz for SFDX).
     •   The signal from the FDX signal generator will be up-tilted such that the PSD of the downstream signal at
         the CM input is flat and is equal in value to 0 dBmV per 6 MHz and 3 dBmV per 6 MHz for the two tests.
         The two tests are described later.
     •   The CM under test is:
             •    Receiving in the FDX sub-band under test
             •    Transmitting in the other two FDX sub-bands
     •   The remaining channel model for this test consists of the following.
             •    Fixed-ACI: This corresponds to the downstream signal, FDX as well as non-FDX, and has the
                  same power spectral density as the OFDM signal in channel under test. This is made up of FDX
                  and DOCSIS 3.1 OFDM downstream channels.
             •    On/Off ACI: This corresponds to upstream transmissions of CM in other IGs. Therefore, this is
                  significantly weaker than on/off ACI in the external-ACI test and hence can be ignored in this test.
             •    Fixed AWGN: Set AWGN0 for the background noise level for a desired CNR as in Figure 16.
                  This is the background noise level. The term "fixed" implies that it is not time-varying during the
                  course of a test. However, the level of this is to be varied from test to test until a PER of 10-6 is
                  obtained. Let this AWGN level be defined as AWGN0 dBmV per 6 MHz. If the signal power has
                  been set to 0 dBmV per 6 MHz, CNR is equal to -AWGN0. If the signal power is set to 3 dBmV
                  per 6 MHZ, CNR is equal to 3-AWGN0. This CNR is the parameter under test.
             •    On-off Interference: This corresponds to upstream transmissions of CMs in other IGs. The level of
                  this is defined as -44 dBmV per 6 MHz.
     •   In addition, there will be the self-ACI generated by the CM under test. This will also be turning on and off
         with the same period and with the same mark-to-space ratio as external on/off ACI and external on/off
         interference. However, self-ACI switching times need not be synchronized with the switching times of
         external on/off ACI and interference.




                            Figure 20 - Time-Varying ACI and AWGN for Self ACI Test



92                                                  CableLabs                                               06/27/25
<!-- PDF_PAGE: 93 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627


The CNR requirements to achieve a PER of 10-6 for all valid QAM constellations are given in Table 23.
                       Table 23 - CNR Performance Requirement of FDX CM for Self ACI Test

                                CNR (dB) for 0 dBmV/6 MHz Input        CNR (dB) for 3 dBmV/6 MHz Input
                                     PSD Tx Output Power:                   PSD Tx Output Power:
                                      CFDX - 64.5 dBmV or                        CFDX - 63 dBmV or
            Constellation              SFDX - 63.75 dBmV                         SFDX - 62.25 dBmV
                 4096                           N/A                                     N/A
                 2048                           N/A                                    44.5
                 1024                            42                                     36
                  512                            33                                     32
                  256                            29                                     29
                  128                            26                                    25.5
                  64                             23                                     23
                  16                             17                                     17


7.2.5.12.4 Period and Mark-to-Space Ratio
Each of the previous two tests has to be implemented for nine combinations of TPER and TON as given by Table
24. This table defines three periods - 10, 70, and 200 ms - and mark-to-space ratios - 10%, 50%, and 90% - for each
period.
                               Table 24 - TPER and TON for the Two Sets of Tests

                                      Test             TPER(ms)          TON(ms)
                                        1                 10                1
                                        2                 10                5
                                        3                 10                9
                                        4                 70                7
                                        5                 70                35
                                        6                 70                63
                                        7                200                20
                                        8                200               100
                                        9                200               180


Each of the two tests, therefore, is to be done nine times for each QAM constellation. The highest CNR value of
these nine tests are not to exceed that listed for the corresponding QAM constellation types in Figure 20 or Table 24.

7.2.5.12.5 Above FDX Transition Band Test

7.2.5.12.5.1 Above FDX Transition Band Test for Downstream up to 1218 MHz
The requirements of this section apply to FDX CMs when the cable plant or operator configuration limits
downstream signals to below 1218 MHz. For operation up to 1794 MHz, see section 7.2.5.12.5.2.
Testing for the band above the FDX transition band assumes the same test and conditions as section 7.5.12.1 of
[DOCSIS PHYv3.1] with the following exceptions:
    •    The FDX Transition band upper band edge is defined as 804 MHz for CFDX operation or 984 MHz for
         SFDX operation.
    •    The FDX downstream lower band edge is defined as 108 MHz for CFDX operation or 258 MHz for SFDX
         operation.
    •    Channel under test frequency is entirely located above the FDX Transition band upper band edge


06/27/25                                              CableLabs                                                  93
