# 7.2.2 - FDX Upstream and Downstream Frequency Plan

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 46-52
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 46 -->
CM-SP-PHYv4.0-I07-250627                                               Data-Over-Cable Service Interface Specifications


To avoid the risk of co-channel interference (CCI) and adjacent channel interference (ACI) between CMs, the
CMTS schedules transmissions and grants such that a CM does not transmit at the same time as other CMs that are
susceptible to interference from the transmitting CM are receiving. CM to CM interference susceptibility is
measured through a sounding process that is defined in the specification. After measuring CM to CM interference
susceptibility, the CMTS creates groups of CMs that are susceptible to interfering with one another, called
interference groups (IG), and schedules transmissions and grants to CMs to avoid having a CM transmit when other
CMs in its IG are receiving.
A DOCSIS 4.0-compliant CM supports two modes of operation related to FDX:
1.   DOCSIS 3.1 mode: The FDX CM behaves identically to a DOCSIS 3.1-compliant cable modem and adheres to
     the requirements in section 7 of [DOCSIS PHYv3.1]. An FDX CM is in this mode if it is on a non-FDX plant or
     it is on an FDX plant but configured to operate in DOCSIS 3.1 mode.
2.   FDX mode: An FDX CM is in this mode when it is on an FDX plant and is configured for FDX operation. In
     this mode the CM adheres to the requirements in this specification. This includes when in both non-FDX and
     FDX operational states.
Transitioning between modes 1 and 2 requires a CM reboot.
A Unified CM operating in FDX mode MUST adhere to all the requirements for an FDX CM as described in this
document, in addition to requirements for support of downstream operation at frequencies up to 1794 MHz.
A Unified CMTS, Unified Node, or Unified RMD/RPD operating in FDX mode MUST adhere to all the
requirements for an FDX CMTS, FDX node, or FDX RMD/RPD, respectively, as described in this document, in
addition to requirements for support of downstream operation at frequencies up to 1794 MHz.

7.2.1    Full Duplex Node Reference Interfaces
In comparison to reference interfaces defined in annex D of [DOCSIS DRFI], FDX node defines two additional
interfaces, E and F, that compensate for upstream and downstream power tilt. These interfaces are indicated in
Figure 4 below.
                               Interface B      Interface C              Interface D                          Interface E      Interface F

                                        FDX Node                                       DS
                                                                                               Hard Line               Drop
                                                                                              Coaxial Cable
     CMTS
                                                          DS                       Tilted                              Cable
                            Digital                   Flat Spectrum,             Spectrum,
                                                                        Node
         Digital            Fiber            RPD/        Low Level       RF      High Level                                           Cable
                                                                                                              Tap
         Output                              RMD                       Launch      RF/DS                                             Modem
                                                                        Amp        Output




                                  Figure 4 - Full Duplex Node Reference Interfaces

7.2.2    Upstream and Downstream Frequency Plan
For DOCSIS 4.0 devices operating in FDX mode, this section augments section 7.2 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
For DOCSIS CMTSs that implement FDX DOCSIS capability and FDX CMs, the legacy DOCSIS requirement for
downstream transmission frequencies to always reside above the upstream transmission frequencies in the cable
plant no longer applies.
If a CMTS implements FDX DOCSIS capability, the CMTS MUST support US reception and DS transmission on
channels occupying the same FDX spectrum yielding concurrent US and DS transmissions at the MAC level.
The FDX CM MAY support US transmission and DS reception on channels occupying the same FDX spectrum
yielding concurrent US and DS transmissions at the MAC and PHY level.
The channel resources for FDX devices are summarized in Table 7; the requirements are described in the following
subsections.




46                                                    CableLabs                                                                  06/27/25
<!-- PDF_PAGE: 47 -->
Physical Layer Specification                                                            CM-SP-PHYv4.0-I07-250627


FDX operation can take place using one of two frequency plans: CFDX (classic FDX) or SFDX (shifted FDX). The
frequency range defined for full duplex operation using CFDX is 108 MHz to 684 MHz. The upper limit of 684
MHz is derived from starting with the lower band edge of mid-split (108 MHz) and allowing for three OFDM
channels at 192 MHz each. For SFDX, the frequency range for full duplex operation is defined in a similar manner,
but the starting frequency is the downstream lower band edge of high split (258 MHz), giving a frequency range of
258–834 MHz.
An FDX node MUST support either the CFDX frequency plan or the SFDX frequency plan. An FDX node is
allowed to support both CFDX and SFDX frequency plans.
An FDX CM MUST support either the CFDX frequency plan or the SFDX frequency plan. An FDX CM is allowed
to support both CFDX and SFDX frequency plans.
The FDX OFDM channels operate on a predefined grid as described in section 7.2.2.5.5.
                                 Table 7 - Channel Resources for FDX Devices

    Item         Device                            OFDM/OFDMA                                           SC-QAM
Downstream     CM          5 total OFDM channels;                                             32
channel                    3 channels capable of FDX operation;
support                    All channels capable of non-FDX operation to the supported
                           downstream upper band edge (1218 MHz to 1794 MHz)
               CMTS        6 total OFDM channels;                                             32
                           3 channels capable of FDX operation;
                           All channels capable of non-FDX operation to the supported
                           downstream upper band edge (1218 or 1794 MHz).
Upstream       CM          At least 7 total OFDMA channels for CFDX support, or at least      4 (or 8) SC-QAM channels,
channel                    8 total OFDMA channels for SFDX support;                           operating within the legacy
support                    6 channels capable of FDX operation;                               diplexer configuration
                           2 channels capable of non-FDX operation within the legacy diplexer
                           configuration.
                           (Some channels can be configurable to support either FDX or
                           non-FDX operation. When supporting 6 FDX OFDMA channels in
                           the CFDX frequency plan, only 1 non-FDX OFDMA channel is
                           required. When supporting 6 FDX OFDMA channels in the SFDX
                           frequency plan, 2 non-FDX OFDMA channels are required.)
               CMTS        8 total OFDMA channels;                                            4 (or 8) SC-QAM channels,
                           6 channels with FDX operation;                                     operation dependent on
                           2 channels capable of non-FDX operation based on operator          operator deployment
                                                                                              requirements
                           deployment requirements.


7.2.2.1    Downstream FDX CM Spectrum
The CM complies with the following requirements, which are additional to the requirements of [DOCSIS PHYv3.1],
section 7.2.1:
    •     The demodulator of an FDX CM operating in the CFDX frequency plan MUST support receiving
          downstream full duplex channels from 108 MHz to 684 MHz.
    •     The demodulator of an FDX CM operating in the SFDX frequency plan MUST support receiving
          downstream full duplex channels from 258 to 834 MHz.
    •     The FDX CM MUST be capable of receiving 5 total OFDM channels, both FDX and non-FDX combined.
          The FDX CM MUST support FDX operation at either 96 MHz width or 192 MHz width on three of the
          OFDM channels in the FDX spectrum (refer to Figure 5). When not operating on an FDX system, 2 of
          these OFDM channels MUST be capable of legacy (non-FDX) operation at a width up to 192 MHz.

7.2.2.2    Downstream FDX CMTS Spectrum
The node complies with the following requirements, in addition to the requirements of [DOCSIS PHYv3.1], section
7.2.2:



06/27/25                                            CableLabs                                                          47
<!-- PDF_PAGE: 48 -->
CM-SP-PHYv4.0-I07-250627                                   Data-Over-Cable Service Interface Specifications


     •    The FDX node modulator operating with the CFDX frequency plan MUST support downstream full duplex
          channel transmissions from 108 MHz to 684 MHz.
     •    The FDX node modulator operating with the SFDX frequency plan MUST support downstream full duplex
          channel transmissions from 258 MHz to 834 MHz.
     •    The FDX node MUST support a minimum of 6 independently configurable OFDM channels, each
          occupying a spectrum of up to 192 MHz in the downstream, bounded by the lower band edge and the upper
          band edge of the DS spectrum. The FDX node MUST support 3 independently configurable OFDM
          channels, each occupying a spectrum of either 96 MHz or 192 MHz in the FDX spectrum.

7.2.2.3    Upstream FDX CM Spectrum
The CM complies with the following requirements:
     •    The FDX CM modulator operating with the CFDX frequency plan MUST support upstream transmissions
          from 5 MHz to 85 MHz and from 108 MHz to 684 MHz. Upstream transmission between 85 MHz and 108
          MHz is not required for CFDX operation.
     •    The FDX CM modulator operating with the SFDX frequency plan MUST support upstream transmissions
          from 5 MHz to 204 MHz and from 258 MHz to 834 MHz. Upstream transmission between 204 and 258
          MHz is not required for SFDX operation.
     •    An FDX CM that supports the CFDX frequency plan MUST support a minimum of 7 independently
          configurable OFDMA upstream channels, each occupying a spectrum of up to 96 MHz. This applies to all
          OFDMA channels supported by the FDX CM, not just FDX OFDMA channels.
     •    An FDX CM that supports the SFDX frequency plan MUST support a minimum of 8 independently
          configurable OFDMA upstream channels, each occupying a spectrum of up to 96 MHz. This applies to all
          OFDMA channels supported by the FDX CM, not just FDX OFDMA channels.
     •    The FDX CM MUST be capable of supporting FDX upstream operation on 6 of the upstream OFDMA
          channels.
     •    The FDX CM MUST be capable of transmitting on all upstream channels simultaneously.
     •    The FDX CM MUST support a minimum of 4 upstream SC-QAM channels in 5 MHz to 85 MHz. The
          FDX CM SHOULD support a minimum of 8 upstream SC-QAM channels. This is the total number of SC-
          QAM channels, and not additional channels with respect to a DOCSIS 3.1 CM.
     •    An FDX CM that supports the CFDX frequency plan MUST be capable of supporting non-FDX upstream
          operation on 1 of the upstream OFDMA channels in the range 5 MHz to 85 MHz when operating on an
          FDX plant.
     •    An FDX CM that supports the SFDX frequency plan MUST be capable of supporting non-FDX operation
          on two (2) of the upstream OFDMA channels in the range 5 MHz to 204 MHz when operating on an FDX
          plant.
     •    The FDX CM MUST be capable of supporting upstream operation on 2 of the upstream OFDMA channels
          when operating on a legacy (non-FDX) plant. The FDX CM MUST support operation on these non-FDX
          channels from 5 MHz to either 85 MHz or 204 MHz, depending on the legacy diplexer configuration.

7.2.2.4    Upstream FDX Node Spectrum
If a CMTS implements Full Duplex DOCSIS capability, the node complies with the following requirements, in
addition to the requirements of [DOCSIS PHYv3.1], section 7.2.4:
     •    If an FDX CMTS implements Full Duplex DOCSIS capability and supports the CFDX frequency plan, the
          FDX node demodulator MUST be capable of receiving upstream transmissions from 5 MHz to 85 MHz
          and from 108 MHz to 684 MHz. Upstream reception between 85 MHz and 108 MHz is not required.




48                                               CableLabs                                          06/27/25
<!-- PDF_PAGE: 49 -->
Physical Layer Specification                                                      CM-SP-PHYv4.0-I07-250627


    •     If an FDX CMTS implements Full Duplex DOCSIS capability and supports the SFDX frequency plan, the
          FDX node demodulator MUST be capable of receiving upstream transmissions from 5 MHz to 204 MHz
          and from 258 MHz to 834 MHz. Upstream reception between 204 MHz and 258 MHz is not required.
    •     If an FDX CMTS implements Full Duplex DOCSIS capability, the FDX node MUST support 8
          configurable OFDMA upstream channels, each occupying a spectrum of up to 95 MHz. This applies to all
          OFDMA channels supported by the CMTS, not just FDX OFDMA channels.

7.2.2.5     FDX Channel Band Rules

7.2.2.5.1    Downstream Channel Bandwidth Rules
See [DOCSIS PHYv3.1], section 7.2.5.1.

7.2.2.5.2    Downstream Exclusion Band Rules
See [DOCSIS PHYv3.1], section 7.2.5.2.

7.2.2.5.3    Upstream Channel Bandwidth Rules
See [DOCSIS PHYv3.1], section 7.2.5.3.

7.2.2.5.4    Upstream Exclusions and Unused Subcarriers Rules
See [DOCSIS PHYv3.1], section 7.2.5.4.

7.2.2.5.5    Channel Band Rules for the FDX Band
In the CFDX frequency plan, the full duplex band is defined as extending from 108 MHz to 684 MHz regardless of
whether FDX channels occupy the whole band.
In the SFDX frequency plan, the full duplex band is defined as extending from 258 MHz to 834 MHz regardless of
whether FDX channels occupy the whole band.
The FDX allocated spectrum is defined as the same as the occupied spectrum, which is all the spectrum in an access
network allocated to full duplex operation, including guard bands, whether it is used for full duplex or not.
The FDX CMTS MUST ensure that first and last active subcarriers of the OFDMA channels in a sub-band do not
extend beyond the first and last active subcarriers of a single DS OFDM channel in the same sub-band.
The FDX CMTS MUST ensure that any excluded subcarrier in the downstream channel is excluded in the upstream
channel.
For CFDX operation, the FDX node MUST configure the FDX allocated spectrum to start at 108 MHz.
For CFDX operation, the FDX node MUST support the configuration of the following bandwidths for the FDX
allocated spectrum and the sub-band options:
    •     96 MHz: Occupying 108 MHz to 204 MHz
          Supporting 1 sub-band composed of a 96 MHz FDX US channel and a 96 MHz FDX DS channel.

    •     192 MHz: Occupying 108 MHz to 300 MHz
          Supporting 2 sub-bands. Each sub-band is configured to be 96 MHz wide, composed of 1 FDX US channel
          and 1 FDX DS channel.

    •     288 MHz: Occupying 108 MHz to 396 MHz
          Supporting 3 sub-bands. Each sub-band is configured to be 96 MHz wide, composed of 1 FDX US channel
          and 1 FDX DS channel.

    •     384 MHz: Occupying 108 MHz to 492 MHz



06/27/25                                          CableLabs                                                  49
<!-- PDF_PAGE: 50 -->
CM-SP-PHYv4.0-I07-250627                                                                          Data-Over-Cable Service Interface Specifications


              Supporting 2 sub-bands. Each sub-band is configured to be 192 MHz wide, composed of 2 FDX US
              channels and 1 FDX DS channel.

     •        576 MHz: Occupying 108 MHz to 684 MHz
              Supporting 3 sub-bands. Each sub-band is configured to be 192 MHz wide, composed of 2 FDX US
              channels (96 MHz wide each) and 1 FDX DS channel.

For SFDX operation, the FDX node MUST configure the FDX allocated spectrum to start at 258 MHz.
For SFDX operation, the FDX node MUST support the configuration of the following bandwidths for the FDX
allocated spectrum and the sub-band options:
     •        96 MHz: Occupying 258 MHz to 354 MHz
              Supporting 1 sub-band composed of a 96 MHz FDX US channel and a 96 MHz FDX DS channel.

     •        192 MHz: Occupying 258 MHz to 450 MHz
              Supporting 2 sub-bands. Each sub-band is configured to be 96 MHz wide, composed of 1 FDX US channel
              and 1 FDX DS channel.

     •        288 MHz: Occupying 258 MHz to 546 MHz
              Supporting 3 sub-bands. Each sub-band is configured to be 96 MHz wide, composed of 1 FDX US channel
              and 1 FDX DS channel.

     •        384 MHz: Occupying 258 MHz to 642 MHz
              Supporting 2 sub-bands. Each sub-band is configured to be 192 MHz wide, composed of 2 FDX US
              channels and 1 FDX DS channel.

     •        576 MHz: Occupying 258 MHz to 834 MHz
              Supporting 3 sub-bands. Each sub-band is configured to be 192 MHz wide, composed of 2 FDX US
              channels (96 MHz wide each) and 1 FDX DS channel.

The configurable allocated spectrum bandwidths described above are illustrated in Figure 5 below for CFDX with
downstream operation to 1218 MHz.
                                  FDX Allocated
                                    Spectrum


                 Non-FDX              DS                                                                                         DS QAM or OFDM             OFDM Only
     96 MHz                                                     DS QAM or OFDM (non-FDX CMs)
                   US                 US                                                                                             (All CMs)           (DOCSIS 3./4.0 CMs)




                 Non-FDX              DS          DS                                                                             DS QAM or OFDM             OFDM Only
     192 MHz                                                            DS QAM or OFDM (non-FDX CMs)
                   US                 US          US                                                                                 (All CMs)           (DOCSIS 3./4.0 CMs)




                 Non-FDX              DS          DS           DS
     288 MHz                                                                  DS QAM or OFDM (non-FDX CMs)                       DS QAM or OFDM             OFDM Only
                   US                 US          US           US                                                                    (All CMs)           (DOCSIS 3./4.0 CMs)




                 Non-FDX                    DS                         DS                 DS QAM or OFDM (non-FDX                DS QAM or OFDM             OFDM Only
     384 MHz
                   US                 US          US           US            US                     CMs)                             (All CMs)           (DOCSIS 3./4.0 CMs)




                 Non-FDX                    DS                         DS                           DS                           DS QAM or OFDM             OFDM Only
     576 MHz
                   US                 US          US           US            US              US            US                        (All CMs)           (DOCSIS 3./4.0 CMs)


         Legacy TCS
                            108             204        300             396          492             588             684   804                     1002                         1218
                  Legacy Transition                          TCS_FDX           Fre que ncy [MHz]
                    Band (85-108)                                                                               FDX Transition
                                                                                                                    Band


                                       Figure 5 - Configurable FDX Allocated Spectrum Bandwidths for
                                                CFDX Frequency Plan - 1218 MHz Downstream




50                                                                             CableLabs                                                                                       06/27/25
<!-- PDF_PAGE: 51 -->
Physical Layer Specification                                                                                                                                                     CM-SP-PHYv4.0-I07-250627


Unified devices support downstream operation at frequencies up to 1794 MHz. Figure 6 below illustrates the
configurable allocated spectrum bandwidths described above for CFDX with downstream operation up to
1794 MHz.

                               FDX Allocated
                                 Spectrum


             Non-FDX              DS                                                                                                                                DS QAM or OFDM               OFDM Only
   96 MHz                                                                    DS QAM or OFDM (non-FDX CMs)
               US                 US                                                                                                                                    (All CMs)             (DOCSIS 3./4.0 CMs)



             Non-FDX              DS                   DS                                                                                                           DS QAM or OFDM               OFDM Only
  192 MHz                                                                                DS QAM or OFDM (non-FDX CMs)
               US                 US                   US                                                                                                               (All CMs)             (DOCSIS 3./4.0 CMs)



             Non-FDX              DS                   DS                    DS                                                                                     DS QAM or OFDM               OFDM Only
  288 MHz                                                                                          DS QAM or OFDM (non-FDX CMs)
               US                 US                   US                    US                                                                                         (All CMs)             (DOCSIS 3./4.0 CMs)



             Non-FDX                         DS                                         DS                          DS QAM or OFDM (non-FDX                         DS QAM or OFDM               OFDM Only
  384 MHz
               US                 US                   US                    US                    US                         CMs)                                      (All CMs)             (DOCSIS 3./4.0 CMs)



             Non-FDX                         DS                                         DS                                          DS                              DS QAM or OFDM               OFDM Only
  576 MHz
               US                 US                   US                    US                    US                    US                    US                       (All CMs)             (DOCSIS 3./4.0 CMs)

     Legacy TCS
                        108              204                      300                   396                   492                   588                  684    804              1002                               1794
              Legacy Transition                                         TCS_FDX                     Frequency [MHz]
                Band (85-108)                                                                                                                        FDX Transition
                                                                                                                                                         Band


                                       Figure 6 - Configurable FDX Allocated Spectrum Bandwidths for
                                                CFDX Frequency Plan - 1794 MHz Downstream
The configurable allocated spectrum bandwidths for the SFDX frequency plan described above are illustrated in
Figure 7 below.

                                             FDX Allocated                                                                                                                     DS QAM or
                                               Spectrum                                                                                                                      OFDM (All CMs)



                                                  DS                                                                                                                                             OFDM Only
   96 MHz         Non-FDX US                                                                  DS QAM or OFDM (non-FDX CMs)
                                                  US                                                                                                                                          (DOCSIS 3./4.0 CMs)


                                                  DS                    DS                                                                                                                       OFDM Only
  192 MHz         Non-FDX US                                                                             DS QAM or OFDM (non-FDX CMs)
                                                  US                    US                                                                                                                    (DOCSIS 3./4.0 CMs)


                                                  DS                    DS                    DS                                                                                                 OFDM Only
  288 MHz         Non-FDX US                                                                                         DS QAM or OFDM (non-FDX CMs)
                                                  US                    US                    US                                                                                              (DOCSIS 3./4.0 CMs)


                                                            DS                                          DS                          DS QAM or OFDM (non-FDX                                      OFDM Only
  384 MHz         Non-FDX US
                                                  US                    US                    US                    US                        CMs)                                            (DOCSIS 3./4.0 CMs)


                                                            DS                                          DS                                          DS                                           OFDM Only
  576 MHz         Non-FDX US
                                                  US                    US                    US                    US                    US                   US                             (DOCSIS 3./4.0 CMs)

     Legacy TCS
                                       258                  354                   450                   546                   642                   738               834      1002                                 1794
                   Legacy Transition                                                    TCS_FDX                                                                             984
                                                                                                                     Frequency [MHz]
                    Band (204-258)                                                                                                                              FDX Transition
                                                                                                                                                                    Band


                  Figure 7 - Configurable FDX Allocated Spectrum Bandwidths for SFDX Frequency Plan
An FDX CMTS MUST configure the downstream channel and upstream channels sharing the same sub-band to the
same subcarrier spacing and cyclic prefix length. The subcarrier spacing and cyclic prefix on different sub-bands are
allowed to be different.
An FDX CMTS MUST configure the FDX allocated spectrum to contain only DS and US FDX channels.




06/27/25                                                                                                CableLabs                                                                                                   51
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
