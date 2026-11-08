# 6 - PHY Sublayer for SC-QAM

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 39-44
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 39 -->
Physical Layer Specification                                                        CM-SP-PHYv4.0-I07-250627


6 PHY SUBLAYER FOR SC-QAM
6.1     Scope
This section applies to cases where a DOCSIS 4.0 CM or CMTS is operating with simultaneous operation of SC-
QAM and OFDM/OFDMA channels unless otherwise noted. Throughout this entire document, "OFDM" pertains to
downstream, "OFDMA" pertains to upstream, and "SC-QAM" pertains to either a) DOCSIS 3.0 or earlier
downstream channels, or b) TDMA, ATDMA, collectively, from DOCSIS 3.0 or earlier, for the upstream channels.
The S-CDMA support is not required for the DOCSIS 4.0 CM and CMTS.
This specification defines the electrical characteristics and signal processing operations for a CM and CMTS. It is
the intent of this specification to define an interoperable DOCSIS 4.0 CM and CMTS such that any implementation
of a DOCSIS 4.0 CM can work with any DOCSIS 4.0 or DOCSIS 3.1 CMTS. It is not the intent of this specification
to imply any specific implementation.
As the requirements for a DOCSIS 4.0 CM and CMTS are largely unchanged relative to SC-QAM operation, this
section is composed primarily of references to the appropriate DOCSIS 3.0 specification sections for the specific
requirements for a DOCSIS 4.0 CM and CMTS, as well as any deltas relative to those requirements.
A DOCSIS 4.0 CM MUST comply with the referenced requirements in the [DOCSIS PHYv3.0] and [DOCSIS
DRFI] specifications noted in this section, with the exception of any deltas called out in this section (which will be
identified with separate requirement statements). A DOCSIS 4.0 CMTS MUST comply with the referenced
requirements in the [DOCSIS PHYv3.0] and [DOCSIS DRFI] specifications noted in this section, with the exception
of any deltas called out in this section (which will be identified with separate requirement statements).
A Unified CM operating in FDX mode MUST meet the requirements in this section that apply to an FDX CM.

6.2     Upstream Transmit and Receive
This section is based on section 6.2 of [DOCSIS PHYv3.0].

6.2.1    Overview
See section 6.2.1 of [DOCSIS PHYv3.0], with the exceptions noted in section 7.2 for the FDX CM and section 7.3
for the FDD CM.

6.2.2    Signal Processing Requirements
See section 6.2.2 of [DOCSIS PHYv3.0].

6.2.3    Modulation Formats
See section 6.2.3 of [DOCSIS PHYv3.0].

6.2.4    R-S Encode
See section 6.2.4 of [DOCSIS PHYv3.0].

6.2.5    Upstream R-S Frame Structure (Multiple Transmit Channel Mode Enabled)
See section 6.2.5 of [DOCSIS PHYv3.0].

6.2.6    Upstream R-S Frame Structure (Multiple Transmit Channel Mode Disabled)
See section 6.2.6 of [DOCSIS PHYv3.0].

6.2.7    TDMA Byte Interleaver
See section 6.2.7 of [DOCSIS PHYv3.0].




06/27/25                                           CableLabs                                                     39
<!-- PDF_PAGE: 40 -->
CM-SP-PHYv4.0-I07-250627                                    Data-Over-Cable Service Interface Specifications


6.2.8   Scrambler (randomizer)
See section 6.2.8 of [DOCSIS PHYv3.0].

6.2.9   TCM Encoder
See section 6.2.9 of [DOCSIS PHYv3.0].

6.2.10 Preamble Prepend
See section 6.2.10 of [DOCSIS PHYv3.0].

6.2.11 Modulation Rates
See section 6.2.11 of [DOCSIS PHYv3.0].

6.2.12 S-CDMA Framer and Interleaver
See section 6.2.12 of [DOCSIS PHYv3.0].

6.2.13 S-CDMA Framer
See section 6.2.13 of [DOCSIS PHYv3.0].

6.2.14 Symbol Mapping
See section 6.2.14 of [DOCSIS PHYv3.0].

6.2.15 S-CDMA Spreader
See section 6.2.15 of [DOCSIS PHYv3.0].

6.2.16 Transmit Pre-Equalizer
See section 6.2.16 of [DOCSIS PHYv3.0].

6.2.17 Spectral Shaping
See section 6.2.17 of [DOCSIS PHYv3.0].

6.2.18 Relative Processing Delays
See section 6.2.18 of [DOCSIS PHYv3.0].

6.2.19 Transmit Power Requirements
For the case in which a DOCSIS 4.0 CM is operating with a DOCSIS 3.0 CMTS regardless of channel type, the
transmit power requirements are described in section 7.2.4.12.4 for the FDX CM and in section 7.3.4.12.4 for the
FDD CM. The requirements in section 7.2.4.12.4 or 7.3.4.12.4 apply even when all upstream channels on the
DOCSIS 4.0 CM are SC-QAM.

6.2.20 Burst Profiles
See section 6.2.20 of [DOCSIS PHYv3.0].

6.2.21 Burst Timing Convention
See section 6.2.21 of [DOCSIS PHYv3.0].




40                                                CableLabs                                            06/27/25
<!-- PDF_PAGE: 41 -->
Physical Layer Specification                                                              CM-SP-PHYv4.0-I07-250627


6.2.22 Fidelity Requirements
For the case in which a DOCSIS 4.0 CM is operating with a DOCSIS 4.0 CMTS regardless of channel type, the
fidelity requirements are described in section 7.2.4.12.7 for FDX CMs and section 7.3.4.12.7 for FDD CM. The
requirements in sections 7.2.4.12.7 and 7.3.4.12.7 apply even when all upstream channels on the DOCSIS 3.1 CM
are SC-QAM. For this case there are no MER requirements for SC-QAM channels. So the sections 7.2.4.12.7.2 and
7.3.4.12.7.2 do not apply for this case.

6.2.23 Upstream Demodulator Input Power Characteristics
See section 6.2.23 of [DOCSIS PHYv3.0].

6.2.24 Upstream Electrical Output from the CM
For the case in which a DOCSIS 4.0 CM is operating with a DOCSIS 3.1 CMTS regardless of channel type, the
upstream electrical output requirements are described in section 7.2.4.13 (FDX CM) and 7.3.4.13 (FDD CM). The
requirements in these sections apply even when all upstream channels on the DOCSIS 4.0 CM are SC-QAM.
                     Table 5 - CM Transmitter Output Signal Characteristics for SC-QAM Channels

            Parameter                                                       Value
Frequency                         Support and be configurable to a permitted subset (see section7.2.2.3 for allowed
                                  combinations) of the following frequency range:
                                  5 MHz to 85 MHz
                                  NOT to cause harmful interference above these frequencies for any configured option.
Signal type                       TDMA
Modulation type                   QPSK, 8-QAM, 16-QAM, 32-QAM, and 64-QAM
Modulation rate (nominal)         TDMA: 1280, 2560, and 5120 kHz
                                  Optional pre-3.0-DOCSIS operation,
                                  TDMA: 160, 320, and 640 kHz
Bandwidth                         TDMA: 1600, 3200, and 6400 kHz
                                  Optional pre-3.0-DOCSIS operation,
                                  TDMA: 200, 400, and 800 kHz
Level                             Total average output power of 65 dBmV.
                                  (See item # 1 immediately following this table.)
                                  Total average output power greater than 65 dBmV.
                                  (See item # 2 following this table.)
Output impedance                  75 ohms
Output return loss                > 6 dB 5 MHz to 85 MHz
Connector                         F connector per [ISO/IEC-61169-24] or [SCTE 02]


The following is an itemized list of CM transmitter output signal characteristics based on Table 5 above.
    1.   The DOCSIS 4.0 CM MUST be capable of transmitting a total average output power of 65 dBmV when
         operating in DOCSIS 3.1 mode.
    2.   The DOCSIS 4.0 CM MAY be capable of transmitting a total average output power greater than 65 dBmV
         when operating in DOCSIS 3.1 mode.

6.2.25 Upstream CM Transmitter Capabilities
For the case in which a DOCSIS 4.0 CM is operating with a DOCSIS 3.0 CMTS or where a DOCSIS 4.0 CMTS is
operating with a DOCSIS 3.0 CM; see section 6.2.25 of [DOCSIS PHYv3.0].
For the case in which a DOCSIS 4.0 CM is operating with a DOCSIS 4.0 CMTS regardless of channel type, the
transmitter capabilities are described in this specification.




06/27/25                                               CableLabs                                                        41
<!-- PDF_PAGE: 42 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


6.3       Downstream Transmit
This section is based on section 6.3 of [DOCSIS DRFI].

6.3.1      Downstream Protocol
See section 6.3.1 of [DOCSIS DRFI].

6.3.2      Spectrum Format
See section 6.3.2 of [DOCSIS DRFI].

6.3.3      Scalable Interleaving to Support Video and High-Speed Data Services
See section 6.3.3 of [DOCSIS DRFI].

6.3.4      Downstream Frequency Plan
See section 6.3.4 of [DOCSIS DRFI].

6.3.5      DRFI Output Electrical
Applies only the case where a DOCSIS 4.0 device is operating in DOCSIS 3.0 mode only.
For legacy SC-QAMs, the EQAM and CMTS MUST support the electrical output requirements specified in the
following sections and tables of [DOCSIS DRFI]:
      •    Section 6.3.5, DRFI Output Electrical
      •    Section 6.3.5.1, CMTS or EQAM Output Electrical
      •    Table titled: RF Output Electrical Requirements, in section 6
      •    Section 6.3.5.1.1, Power per Channel CMTS or EQAM
      •    Table titled: DRFI Device Output Power, in section 6
      •    Section 6.3.5.1.2, Independence of Individual Channels within the Multiple Channels on a Single RF Port
      •    Section 6.3.5.1.3, Out-of-Band Noise and Spurious Requirements for CMTS or EQAM
      •    Table titled: EQAM or CMTS Output Out-of-Band Noise and Spurious Emissions Requirements for N ≤ 8,
           in section 6
      •    Table titled: EQAM or CMTS Output Out-of-Band Noise and Spurious Emissions Requirements N ≥ 9 and
           N' ≥ N/4, in section 6
      •    Table titled: EQAM or CMTS Output Out-of-Band Noise and Spurious Emissions Requirements N ≥ 9 and
           N' < N/4, in section 6

6.3.6      CMTS or EQAM Clock Generation
Applies only to the case where a DOCSIS 4.0 CMTS is operating in a DOCSIS 3.0 mode only.
See section 6.3.6 of [DOCSIS DRFI].

6.3.7      Downstream Symbol Clock Jitter for Synchronous Operation
Applies only to the case where a DOCSIS 4.0 CMTS is operating in a DOCSIS 3.0 mode only.
See section 6.3.7 of [DOCSIS DRFI].

6.3.8      Downstream Symbol Clock Drift for Synchronous Operation
Applies only to the case where a DOCSIS 4.0 CMTS is operating in a DOCSIS 3.0 mode only.


42                                                   CableLabs                                          06/27/25
<!-- PDF_PAGE: 43 -->
Physical Layer Specification                                                    CM-SP-PHYv4.0-I07-250627


See section 6.3.8 of [DOCSIS DRFI].

6.3.9     Timestamp Jitter
See section 6.3.9 of [DOCSIS DRFI].

6.4     Downstream Receive
This section is based on section 6.3 of [DOCSIS PHYv3.0].

6.4.1     Downstream Protocol and Interleaving Support
See section 6.3.1 of [DOCSIS PHYv3.0].

6.4.2     Downstream Electrical Input to the CM
See section 6.3.2 of [DOCSIS PHYv3.0], with the exception noted below.
A CM MUST support at least 32 active downstream SC-QAM channels.
A CMTS MUST support at least 32 active downstream SC-QAM channels.

6.4.3     CM BER Performance
See section 6.3.3 of [DOCSIS PHYv3.0].

6.4.3.1    FDX CM BER Performance Above the FDX Transition Band
The FDX transition band upper band edge is defined as 804 MHz for CFDX operation, or 984 MHz for SFDX
operation. The CM BER performance of an FDX CM operating in FDX mode receiving SC-QAM channels between
the FDX transition band upper band edge and 1002 MHz with a flat receive PSD in the presence of AWGN is given
in sections 6.3.3.1.1 and 6.3.3.2.1 of [DOCSIS PHYv3.0].
Implementation loss of the FDX CM when in FDX mode MUST be such that the CM achieves a post-FEC BER less
than or equal to 10-8 when operating at a carrier to noise ratio (ES/N0) as shown in Table 6 - Minimum CNR
Performance from FDX Transition Band Upper Band Edge to 1002 MHz for FDX CM Operating in FDX Mode. If
it is not possible to measure post-FEC BER directly, Codeword Error Ratio, RC (as defined in section 6.3.3.1.1 of
[DOCSIS PHYv3.0]) may be used. In this case, the FDX CM MUST achieve a Codeword Error Ratio of less than or
equal to 9 x 10-7 when operating at a carrier to noise ratio (ES/N0), as shown in Table 6 - Minimum CNR
Performance from FDX Transition Band Upper Band Edge to 1002 MHz for FDX CM Operating in FDX Mode
below.
                       Table 6 - Minimum CNR Performance from FDX Transition Band
                      Upper Band Edge to 1002 MHz for FDX CM Operating in FDX Mode

               SC-QAM Modulation              Receive Signal Level              Minimum CNR
                    Order                        (dBmV/6 MHz)                       (dB)
                       256                           -6 to +15                         30
                       256                          -15 to <-6                         33
                        64                          -15 to +15                        23.5


For an FDX CM not operating in FDX mode, the requirements in [DOCSIS PHYv3.0] apply for downstream
SC-QAM performance.

6.4.4     Downstream Multiple Receiver Capabilities
See section 6.3.4 of [DOCSIS PHYv3.0].




06/27/25                                         CableLabs                                                  43
<!-- PDF_PAGE: 44 -->
CM-SP-PHYv4.0-I07-250627                               Data-Over-Cable Service Interface Specifications


6.4.5   Non-Synchronous DS Channel Support
Applies only to the case where a DOCSIS 4.0 CM operating with a DOCSIS 3.0 CMTS.
See section 6.3.5 of [DOCSIS PHYv3.0].




44                                            CableLabs                                      06/27/25
