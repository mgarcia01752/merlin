# 6 - PHY Sublayer for SC-QAM

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 39-43
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 39 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419


6 PHY SUBLAYER FOR SC-QAM
6.1     Scope
This section applies to cases where a DOCSIS 3.1 CM or CMTS is operating with SC-QAM operation only, with no
OFDM/OFDMA operation and for the SC-QAM channels with simultaneous operation of SC-QAM and
OFDM/OFDMA channels unless otherwise noted. As such, it represents backward compatibility requirements when
operating with DOCSIS 3.0 systems or with the new DOCSIS 3.1 PHY disabled. It also applies only to the first
technology option referred to in Section 1.1; for the second option refer to Annex C; and for the third option refer to
Annex D. Throughout this entire document, "OFDM" pertains to downstream, "OFDMA" pertains to upstream, and
"SC-QAM" pertains to either a) DOCSIS 3.0 or earlier downstream channels, or b) TDMA, ATDMA, and S-
CDMA, collectively, from DOCSIS 3.0 or earlier, for the upstream channels.
This specification defines the electrical characteristics and signal processing operations for a CM and CMTS. It is
the intent of this specification to define an interoperable CM and CMTS such that any implementation of a CM can
work with any CMTS. It is not the intent of this specification to imply any specific implementation.
As the requirements for a DOCSIS 3.1 CM and CMTS are largely unchanged relative to DOCSIS 3.0 devices for
SC-QAM operation, this section is comprised primarily of references to the appropriate DOCSIS 3.0 specification
sections for the specific requirements for a DOCSIS 3.1 CM and CMTS, as well as any deltas relative to those
requirements (the primary difference being that a DOCSIS 3.1 CM and CMTS are required to support a minimum of
32 downstream and 8 upstream channels instead of 4 downstream and 4 upstream as in DOCSIS 3.0 devices).
A DOCSIS 3.1 CM MUST comply with the referenced requirements in the PHYv3.0 and DRFI specifications noted
in this section, with the exception of any deltas called out in this section (which will be identified with separate
requirement statements). A DOCSIS 3.1 CMTS MUST comply with the referenced requirements in the PHYv3.0
and DRFI specifications noted in this section, with the exception of any deltas called out in this section (which will
be identified with separate requirement statements).

6.2     Upstream Transmit and Receive
This section is based on section 6.2 of [DOCSIS PHYv3.0].

6.2.1    Overview
See section 6.2.1 of [DOCSIS PHYv3.0], with the exceptions noted below.
A CM MUST support at least eight (8) active upstream channels (which are referred to as the Transmit Channel Set
for that CM).
A CMTS MUST support at least eight (8) active upstream channels.
A CMTS MAY support S-CDMA mode. If a CMTS implements S-CDMA mode, the CMTS MUST comply with S-
CDMA requirements defined in [DOCSIS PHYv3.0].
A CM MAY support S-CDMA mode. If a CM implements S-CDMA mode, the CM MUST comply with S-CDMA
requirements defined in [DOCSIS PHYv3.0].

6.2.2    Signal Processing Requirements
See section 6.2.2 of [DOCSIS PHYv3.0].

6.2.3    Modulation Formats
See section 6.2.3 of [DOCSIS PHYv3.0].

6.2.4    R-S Encode
See section 6.2.4 of [DOCSIS PHYv3.0].




04/19/23                                            CableLabs                                                      39

<!-- PDF_PAGE: 40 -->
CM-SP-PHYv3.1-I20-230419                                Data-Over-Cable Service Interface Specifications


6.2.5   Upstream R-S Frame Structure (Multiple Transmit Channel Mode Enabled)
See section 6.2.5 of [DOCSIS PHYv3.0].

6.2.6   Upstream R-S Frame Structure (Multiple Transmit Channel Mode Disabled)
See section 6.2.6 of [DOCSIS PHYv3.0].

6.2.7   TDMA Byte Interleaver
See section 6.2.7 of [DOCSIS PHYv3.0].

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
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.0 CMTS or where a DOCSIS 3.1 CMTS is
operating with a DOCSIS 3.0 CM, see section 6.2.19 of [DOCSIS PHYv3.0].




40                                            CableLabs                                        04/19/23

<!-- PDF_PAGE: 41 -->
Physical Layer Specification                                                             CM-SP-PHYv3.1-I20-230419


For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.1 CMTS regardless of channel type, the
transmit power requirements are described in Section 7.4.13. The requirements in Section 7.4.13 apply even when
all upstream channels on the DOCSIS 3.1 CM are SC-QAM.

6.2.20 Burst Profiles
See section 6.2.20 of [DOCSIS PHYv3.0].

6.2.21 Burst Timing Convention
See section 6.2.21 of [DOCSIS PHYv3.0].

6.2.22 Fidelity Requirements
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.0 CMTS or where a DOCSIS 3.1 CMTS is
operating with a DOCSIS 3.0 CM; see section 6.2.22 of [DOCSIS PHYv3.0].
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.1 CMTS regardless of channel type, the
fidelity requirements are described in Section 7.4.13. The requirements in Section 7.4.13, except Section 7.4.12.5.3,
MER, apply even when all upstream channels on the DOCSIS 3.1 CM are SC-QAM. For this case there are no MER
requirements for SC-QAM channels.

6.2.23 Upstream Demodulator Input Power Characteristics
See section 6.2.23 of [DOCSIS PHYv3.0].

6.2.24 Upstream Electrical Output from the CM
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.0 CMTS or where a DOCSIS 3.1 CMTS is
operating with a DOCSIS 3.0 CM; see section 6.2.24 of [DOCSIS PHYv3.0].
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.1 CMTS regardless of channel type, the
upstream electrical output requirements are described in Section 7.4.13. The requirements in Section 7.4.13 apply
even when all upstream channels on the DOCSIS 3.1 CM are SC-QAM.
                 Table 5 - CM Transmitter Output Signal Characteristics for SC-QAM Channels

           Parameter                                                      Value
  Frequency                      Support and be configurable to a permitted subset (see Section 7.2.3 for allowed
                                 combinations) of the following list of frequency ranges:
                                 5-42 MHz
                                 5-65 MHz
                                 5-85 MHz
                                 NOT to cause harmful interference above these frequencies for any configured option.
  Signal Type                    TDMA
                                 S-CDMA (optional)
  Modulation Type                QPSK, 8-QAM, 16-QAM, 32-QAM, 64-QAM, and 128-QAM
  Modulation Rate (nominal)      TDMA: 1280, 2560, and 5120 kHz
                                 S-CDMA: 1280, 2560, and 5120 kHz
                                 Optional pre-3.0-DOCSIS operation,
                                 TDMA: 160, 320, and 640 kHz
  Bandwidth                      TDMA: 1600, 3200, and 6400 kHz
                                 S-CDMA: 1600, 3200, and 6400 kHz
                                 Optional pre-3.0-DOCSIS operation,
                                 TDMA: 200, 400, and 800 kHz
  Level                          Total average output power of 65 dBmV.
                                 (See item # 1 immediately following this table)
                                 Total average output power greater than 65 dBmV.
                                 (See item # 2 following this table)
  Output Impedance               75 ohms
  Output Return Loss             > 6 dB 5 MHz - fmax (fmax = 42/65/85/204 MHz)
  Connector                      F connector per [ISO/IEC-61169-24] or [SCTE 02]



04/19/23                                            CableLabs                                                          41

<!-- PDF_PAGE: 42 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


The following is an itemized list of CM Transmitter Output Signal Characteristics based on Table 5 above.
1.    The CM MUST be capable of transmitting a total average output power of 65 dBmV.
2.    The CM MAY be capable of transmitting a total average output power greater than 65 dBmV.

6.2.25 Upstream CM Transmitter Capabilities
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.0 CMTS or where a DOCSIS 3.1 CMTS is
operating with a DOCSIS 3.0 CM, see section 6.2.25 of [DOCSIS PHYv3.0].
For the case in which a DOCSIS 3.1 CM is operating with a DOCSIS 3.1 CMTS regardless of channel type, the
transmitter capabilities are described in this specification.

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
Applies only the case where a DOCSIS 3.1 device is operating in DOCSIS 3.0 mode only.
For legacy SC-QAMs, the EQAM and CMTS MUST support the electrical output requirements specified in the
following sections and tables of [DOCSIS DRFI]:
      •    Section 6.3.5, DRFI Output Electrical
      •    Section 6.3.5.1, CMTS or EQAM Output Electrical
      •    Table titled: RF Output Electrical Requirements, in Section 6
      •    Section 6.3.5.1.1, Power per Channel CMTS or EQAM
      •    Table titled: DRFI Device Output Power, in Section 6
      •    Section 6.3.5.1.2, Independence of Individual Channels within the Multiple Channels on a Single RF Port
      •    Section 6.3.5.1.3, Out-of-Band Noise and Spurious Requirements for CMTS or EQAM
      •    Table titled: EQAM or CMTS Output Out-of-Band Noise and Spurious Emissions Requirements for N ≤ 8,
           in Section 6
      •    Table titled: EQAM or CMTS Output Out-of-Band Noise and Spurious Emissions Requirements N ≥ 9 and
           N' ≥ N/4, in Section 6
      •    Table titled: EQAM or CMTS Output Out-of-Band Noise and Spurious Emissions Requirements N ≥ 9 and
           N' < N/4, in Section 6




42                                                   CableLabs                                             04/19/23

<!-- PDF_PAGE: 43 -->
Physical Layer Specification                                                 CM-SP-PHYv3.1-I20-230419


6.3.6    CMTS or EQAM Clock Generation
Applies only the case where a DOCSIS 3.1 CMTS is operating in a DOCSIS 3.0 mode only.
See section 6.3.6 of [DOCSIS DRFI].

6.3.7    Downstream Symbol Clock Jitter for Synchronous Operation
Applies only the case where a DOCSIS 3.1 CMTS is operating in a DOCSIS 3.0 mode only.
See section 6.3.7 of [DOCSIS DRFI].

6.3.8    Downstream Symbol Clock Drift for Synchronous Operation
Applies only the case where a DOCSIS 3.1 CMTS is operating in a DOCSIS 3.0 mode only.
See section 6.3.8 of [DOCSIS DRFI].

6.3.9    Timestamp Jitter
See section 6.3.9 of [DOCSIS DRFI].

6.4     Downstream Receive
This section is based on section 6.3 of [DOCSIS PHYv3.0].

6.4.1    Downstream Protocol and Interleaving Support
See section 6.3.1 of [DOCSIS PHYv3.0].

6.4.2    Downstream Electrical Input to the CM
See section 6.3.2 of [DOCSIS PHYv3.0], with the exception noted below.
A CM MUST support at least 32 active downstream channels.
A CMTS MUST support at least 32 active downstream channels.

6.4.3    CM BER Performance
See section 6.3.3 of [DOCSIS PHYv3.0].

6.4.4    Downstream Multiple Receiver Capabilities
See section 6.3.4 of [DOCSIS PHYv3.0].

6.4.5    Non-Synchronous DS Channel Support
Applies only to the case where a DOCSIS 3.1 CM operating with a DOCSIS 3.0 CMTS.
See section 6.3.5 of [DOCSIS PHYv3.0].




04/19/23                                        CableLabs                                        43
