# 7.3.2 - FDD Upstream and Downstream Frequency Plan

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 107-113
Classification: Main body (DOCSIS 4.0 FDD requirements)

<!-- PDF_PAGE: 107 -->
Physical Layer Specification                                                            CM-SP-PHYv4.0-I07-250627


A Unified CM operating in FDD mode MUST adhere to all requirements for an FDD CM as described in this
document.
A Unified CMTS, Unified FDD Node, or Unified RMD/RPD operating in FDD mode MUST adhere to all
requirements for an FDD CMTS or FDD RMD/RPD, respectively, as described in this document.
The CM is allowed to reboot when transitioning between the two operational modes or between any of the splits in
DOCSIS 4.0 FDD Mode.
A DOCSIS 4.0-compliant FDD node supports two modes of operation related to FDD:
    1.   Legacy Transition Mode: When using downstream lower band edge below 258 MHz, the node is operating
         in a mode that transmits legacy video SC-QAM and other non-DOCSIS signals below 258 MHz and is
         restricted to 1,536 MHz of occupied downstream spectrum. The CMTS may not define any upstream
         channels that would overlap with the downstream transmissions.
    2.   DOCSIS 4.0 FDD Mode: A DOCSIS 4.0 FDD node is in this mode when it is on a plant capable of high-
         split or any of the UHS defined splits.

7.3.1    FDD Node Reference Interfaces
This specification assumes the downstream CMTS modulator is located in an FDD node in a cable television plant.
[DOCSIS PHYv3.1] defines specifications for a CMTS which is presumed to be located at a headend or hub site.
However, the MHAv2 and FMA family of specifications describe an alternate system architecture in which the
downstream modulator is located within an optical node as part of a DCA node. Most, but not all, of the
requirements for a PHY Device in a MHAv2 architecture and a MACPHY Device in FMA are the same as those
which apply to an Integrated CMTS.
The purpose of this section is to define the RF characteristics required in the OFDM downstream modulator of a
DCA function located within an optical node, with sufficient specificity to enable vendors to build devices meeting
the needs of cable operators around the world.
In comparison to reference interfaces defined in annex D of [DOCSIS DRFI], the FDD node defines two additional
interfaces, E and F, that compensate for upstream and downstream power tilt. These interfaces are indicated in
Figure 28 below.
                              Interface B      Interface C        Interface D                   Interface E      Interface F

                                       FDD Node                                  Hard Line               Drop
                                                                                Coaxial Cable
    CMTS                   Digital
                                                                                                         Cable

         Digital           Fiber                               Node
                                            RMD/                                                Tap
                                                                                                                        Cable
         Output                                              RF Launch                                                 Modem
                                             RPD                Amp



                                     Figure 28 - FDD Node Reference Interfaces

7.3.2    Upstream and Downstream Frequency Plan for FDD Operation
For FDD DOCSIS 4.0 devices operating in FDD mode, this section augments section 7.2 of [DOCSIS PHYv3.1]
and corresponding subsections unless otherwise noted. The following spectrum definitions are based on the system
requirement that the downstream transmission frequencies always reside above the upstream transmission
frequencies in the cable plant.
For DOCSIS CMTSs that implement FDD DOCSIS capability and FDD CMs, the following spectrum definitions
are based on the legacy DOCSIS requirement for downstream transmission frequencies to always reside above the
upstream transmission frequencies in the RF spectrum.
DOCSIS 4.0 FDD nodes are required to support two concepts shown in Table 29. The first is that the output at
Interface C has to be capable of transmitting 1,536 MHz of occupied downstream spectrum when the downstream
lower band edge is at 258 MHz and below. In order to assist operators in a more gradual transition to DOCSIS 4.0



06/27/25                                               CableLabs                                                              107
<!-- PDF_PAGE: 108 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


FDD, nodes are required to support legacy downstream lower band edges below 258 MHz but are not required to
support greater occupied downstream spectrum. If the downstream lower band edge is 258 MHz, then the occupied
downstream spectrum is 1,536 MHz and the downstream upper band edge will be 1,794 MHz. If the downstream
lower band edge is at a higher frequency, the occupied downstream spectrum will be less than 1,536 MHz. If the
downstream lower band edge is at a lower frequency, the node is only required to support an occupied downstream
spectrum of 1,536 MHz. The following items should be noted when nodes are transmitting with a downstream lower
band edge lower than 258 MHz.
      •   DOCSIS 4.0 FDD cable modems are not required to receive frequencies below 258 MHz; therefore, any
          node transmitting DOCSIS channels below 258 MHz will only be capable of being received by DOCSIS
          3.1 and earlier cable modems. A DOCSIS 4.0 FDD node is also not required to transmit DOCSIS channels
          below 258 MHz, so transmission of DOCSIS channels below 258 MHz is unsupported. Operators are
          strongly encouraged to limit downstream frequencies below 258 MHz to video SC-QAM and other non-
          DOCSIS signals.
      •   The ability of a DOCSIS 4.0 FDD node to support downstream lower band edges below 258 MHz should
          be viewed by operators as providing options to transition to a fully DOCSIS 4.0-compliant network. Future
          DOCSIS versions could remove support for downstream lower band edges below 258 MHz.
      •   The 1,536 MHz occupied downstream spectrum is not required to be contiguous. It is possible, for
          example, for the downstream lower band edge to be 108 MHz, the downstream upper band edge to remain
          at 1,794 MHz, and leave a hole in the occupied downstream spectrum of 150 MHz at LTE frequencies.
          Table 29 assumes that the occupied downstream spectrum is contiguous, but that is not a requirement of the
          specification.
The second concept introduced in DOCSIS 4.0 FDD is that of a variable-width transition band between the upstream
upper band edge and the downstream lower band edge. In DOCSIS 3.1 and previous DOCSIS specifications, both
the upstream upper band edge and downstream lower band edge had a defined frequency. With the introduction of
UHS-300, UHS-396, UHS-492, and UHS-684, the traditional fixed band edges on each side of this transition band
get increasingly large. Since each megahertz of downstream spectrum has a value to the operators, these
increasingly larger transition bands come at a cost to the operators. To encourage the vendor community to innovate
in order to reduce this width of the transition band without requiring lengthy updates to the specification text,
DOCSIS 4.0 FDD keeps the upstream upper band edge at a fixed frequency for all the new UHS splits. The
downstream lower band edge, however, has a range that DOCSIS 4.0 FDD devices can support. This range extends
from the upstream upper band edge to the static downstream lower band edge initially specified. To ensure
backwards compatibility, none of the splits defined in DOCSIS 3.1 and earlier support variable-width transition
bands. Table 29 shows the band edges for each of the splits defined in DOCSIS 4.0 FDD and the devices to which
these splits apply. Note that Table 29 assumes that the occupied downstream spectrum is contiguous; however, this
is not a requirement of these specifications.
                         Table 29 - DOCSIS 4.0 FDD Band Edges and Device Applicability
                               (Assuming a Contiguous Occupied Downstream Spectrum)
                       US Lower              US Upper                DS Lower         DS Upper         Device
      Split Name       Band Edge             Band Edge               Band Edge        Band Edge      Applicability
  Subsplit/                            Up to 42 MHz                                   1,584 MHz to
                       5 MHz                                       50 MHz to 54 MHz                   Node Only
  Extended Subsplit                    (support for SCTE 55-1,2)                      1,590 MHz
                                       65 MHz
  EuroDOCSIS-Split     5 MHz                                       87.5 MHz           1,623.5 MHz     Node Only
                                       (support for SCTE 55-1,2)
                                       85 MHz
  Mid-Split            5 MHz                                       108 MHz            1,644 MHz       Node Only
                                       (support for SCTE 55-1,2)
                                                                                                      Node and
  High-Split           5 MHz           204 MHz                     258 MHz            1,794 MHz
                                                                                                      Cable Modem
                                                                                                      Node and
  UHS-3001             5 MHz           300 MHz                     300 MHz–372 MHz    1,794 MHz
                                                                                                      Cable Modem
                                                                                                      Node and
  UHS-3961             5 MHz           396 MHz                     396 MHz–492 MHz    1,794 MHz
                                                                                                      Cable Modem




108                                                 CableLabs                                            06/27/25
<!-- PDF_PAGE: 109 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


                      US Lower              US Upper                 DS Lower            DS Upper           Device
    Split Name        Band Edge             Band Edge                Band Edge           Band Edge        Applicability
                                                                                                          Node and
  UHS-4921            5 MHz           492 MHz                     492 MHz–606 MHz       1,794 MHz
                                                                                                          Cable Modem
                                                                                                          Node and
  UHS-6841            5 MHz           684 MHz                     684 MHz–834 MHz       1,794 MHz
                                                                                                          Cable Modem
 TABLE NOTE
 NOTE 1 DOCSIS 4.0 cable modems are not required to support upstream transmissions between 85 MHz and 108 MHz for
        these splits.


The channel resources for FDD devices are summarized in Table 30; the requirements are described in the following
subsections. DOCSIS 4.0 products support ATDMA type upstream channels, and these are referred to as upstream
SC-QAM channels.
The FDD OFDMA channels operate on a predefined 96 MHz grid as described in Section 7.3.2.5.5.
                                 Table 30 - Channel Resources for FDD Devices

       Item          Device         OFDM/OFDMA                                       SC-QAM
 Downstream         CM         5 OFDM channels             32
 channel support
                    CMTS       6 OFDM channels             32
 Upstream channel   CM         7 OFDMA channels            4 upstream SC-QAM channels, operating within the diplex filter
 support                                                   configuration. Optional support for up to 8 SC-QAM channels
                    CMTS       8 OFDMA channels            4 upstream SC-QAM channels, operation dependent on operator
                                                           deployment requirements. Optional support for up to 8 upstream
                                                           SC-QAM channels.



7.3.2.1    Downstream CM Spectrum
The FDD CM MUST support a minimum of five independently configurable OFDM channels each occupying a
spectrum of up to 192 MHz in the downstream.
The demodulator in the FDD CM MUST support receiving downstream transmissions to at least 1794 MHz.
The demodulator in the FDD CM MUST support agile placement of the OFDM channels within the entire supported
downstream range.
The FDD CM MUST support a downstream lower band edge within the frequency range shown in Table 31 - FDD
CM Diplex Filter Band Edge Frequencies.
The FDD CM MUST support a 1794 MHz downstream upper band edge. Support in this context includes the
capability to demodulate up to the supported band edge.
For each of the upstream upper band edges supported, the FDD CM MUST support a downstream lower band edge
within the frequency range shown in Table 31 - FDD CM Diplex Filter Band Edge Frequencies. Support in this
context includes the capability to demodulate down to this supported band edge.
The FDD CM could be optimized to operate with any supported downstream upper band edge. The nature and
operation of this optimization is vendor-specific.

7.3.2.2    Downstream FDD Node Spectrum
The FDD node MUST support a minimum of six independently configurable OFDM channels each occupying a
spectrum of up to 192 MHz in the downstream.
The FDD node MUST support an occupied downstream spectrum width of 1,536 MHz.
The FDD node MUST support a downstream upper band edge of 1794 MHz.
The FDD node MUST support a downstream lower band edge of 258 MHz for DOCSIS 4.0 FDD operation.



06/27/25                                           CableLabs                                                         109
<!-- PDF_PAGE: 110 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


The FDD node MUST support a downstream lower band edge of 54 MHz for legacy transition mode.
For each of the upstream upper band edges supported, the FDD node MUST support a downstream lower band edge
within the frequency range shown in Table 29 - DOCSIS 4.0 FDD Band Edges and Device Applicability.

7.3.2.3      Upstream CM Spectrum
Individual CM implementations can limit the spectrum over which the CM is able to transmit upstream signals.
As a result, in order to be compliant with this specification an FDD CM MUST support at least two of the following
upstream upper band edges: 204 MHz; 300 MHz; 396 MHz; 492 MHz; and/or 684 MHz as shown in Table 31 -
FDD CM Diplex Filter Band Edge Frequencies.
The FDD CM MAY support more than two of the following upstream upper band edges: 204 MHz; 300 MHz; 396
MHz; 492 MHz; and/or 684 MHz as shown in Table 31 - FDD CM Diplex Filter Band Edge Frequencies.
The diplex filters listed in Table 31 are defined for FDD technology.
                                 Table 31 - FDD CM Diplex Filter Band Edge Frequencies

          Split Name            Diplex Filter Start Frequency            Diplex Filter Stop Frequency
                             (Upstream Upper Band Edge) (MHz)        (Downstream Lower Band Edge) (MHz)
           High-Split                       204*                                        258
           UHS-300                          300                                  300 MHz–372 MHz
           UHS-396                          396                                  396 MHz–492 MHz
           UHS-492                          492                                  492 MHz–606 MHz
           UHS-684                          684                                  684 MHz–834 MHz

*NOTE: For cable modems, the 204 MHz upstream upper band edge is a special case for compatibility with DOCSIS
       3.1 high-split networks. In this case, the FDD CM supports the full band 5 MHz to 204 MHz from a single
       (legacy) TCS, therefore, including the range 85 MHz to 108 MHz. For all other supported diplex filters, the
       FDD CM's upstream occurs in two TCSs isolated by an additional diplex filter over the range 85 MHz to 108
       MHz. For these cases, the range 85 MHz to 108 MHz is unusable for US transmission. This is further
       described and illustrated in Section 7.3.2.5 and Figure 29. Also, refer to Section 7.3.4 for additional details
       regarding the TCS and TCS_FDD.
The FDD CM MUST be configurable to operate with any supported upstream upper band edge. The nature and
operation of this configurability is vendor-specific. Possible forms of configurability include a hardware switch on
the modem housing, a software-controlled diplex filter responsive to OSSI commands, or other forms.
The FDD CM also complies with the following requirements:
      •    The FDD CM modulator MUST support upstream transmissions from 5 MHz to 204 MHz in a single
           (legacy) TCS, including the frequency range from 85 MHz to 108 MHz when using a 204 MHz diplex filter
           and the transmit power requirements in [DOCSIS PHYv3.1].
      •    The FDD CM modulator MUST support upstream transmissions from 5 MHz to 85 MHz (legacy TCS) and
           from 108 MHz to the upstream upper band edge (TCS_FDD) when using any of UHS-300, UHS-396,
           UHS-492, or UHS-684 diplex filter options; in these cases, upstream transmission between 85 MHz and
           108 MHz is not required.
                o       The FDD CM MUST support 96 MHz wide upstream FDD channels between 108 MHz and the
                        upstream upper band edge, conforming to the channel grid of Figure 29.
      •    The FDD CM MUST support a minimum number of independently configurable OFDMA upstream
           channels, each occupying a spectrum of up to 96 MHz, for any UHS diplexer setting, as specified in Table
           32 and as illustrated in Figure 29.




110                                                  CableLabs                                            06/27/25
<!-- PDF_PAGE: 111 -->
Physical Layer Specification                                                     CM-SP-PHYv4.0-I07-250627


                    Table 32: Minimum Total of OFDMA Channels for a UHS Diplexer Setting

                                   UHS Diplexer          Min. Total Number of
                                     Setting              OFDMA Channels
                                      UHS-300                      3
                                      UHS-396                      4
                                      UHS-492                      5
                                      UHS-684                      7

    •     The FDD CM MUST support at least one OFDMA channel below 85 MHz. The FDD CM MUST support
          additional 96 MHz wide channels above 108 MHz as indicated in Table 32.
    •     The FDD CM MUST be capable of transmitting on all upstream channels simultaneously.
    •     The FDD CM MUST support a minimum of 4 upstream SC-QAM channels in the range of 5 MHz to 85
          MHz. The FDD CM MAY support a minimum of 8 upstream SC-QAM channels in the range 5 MHz to 85
          MHz. This is the total number of upstream SC-QAM channels, and not additional channels with respect to
          a DOCSIS 3.1 CM.

7.3.2.4     Upstream FDD Node Spectrum
If a CMTS implements FDD capability, the node complies with the following requirements, in addition to the
requirements of section 7.2.4 of [DOCSIS PHYv3.1]:
    •     The FDD RPD MUST be capable of receiving upstream transmissions from 5 MHz to 684 MHz.
    •     The FDD RPD MUST support 8 configurable OFDMA upstream channels, each independently
          configurable for up to 96 MHz. This applies to all OFDMA channels supported by the CMTS, not just UHS
          FDD OFDMA channels.
Additional requirements:
    •     The FDD RPD MUST support upstream FDD channels between 108 MHz and the upstream upper band
          edge that are 96 MHz wide and conform to the channel grid illustrated in Figure 29.
    •     The FDD RPD MUST be capable of receiving upstream transmissions from 5 MHz to 204 MHz when
          operating in DOCSIS 3.1 high-split operational mode.
    •     The FDD RPD MUST support 2 configurable OFDMA upstream channels, each occupying spectrum of up
          to 96 MHz when operating in DOCSIS 3.1 high-split operational mode.

7.3.2.5     FDD Channel Band Rules

7.3.2.5.1    Downstream Channel Bandwidth Rules
See [DOCSIS PHYv3.1], section 7.2.5.1.

7.3.2.5.2    Downstream Exclusion Band Rules
See [DOCSIS PHYv3.1], section 7.2.5.2.

7.3.2.5.3    Upstream Channel Bandwidth Rules
See [DOCSIS PHYv3.1], section 7.2.5.3.

7.3.2.5.4    Upstream Exclusions and Unused Subcarriers Rules
See [DOCSIS PHYv3.1], section 7.2.5.4.




06/27/25                                          CableLabs                                                 111
<!-- PDF_PAGE: 112 -->
CM-SP-PHYv4.0-I07-250627                                                                            Data-Over-Cable Service Interface Specifications


7.3.2.5.5         Channel Band Rules for the FDD Band
For FDD operation, the extended upstream ranges beyond high-split (204 MHz) is provided for by the use of two
separate transmit channel sets (TCSs). As with the FDX US operation, the two TCS are separated/isolated within the
FDD CM by a diplex filter over the range 85 MHz to 108 MHz. In addition to the extended FDD US ranges (300
MHz, 396 MHz, 492 MHz and 684 MHz) and the associated diplex filter transition band, there is another diplex
filter region in the range of 85 MHz to 108 MHz which is unusable for US transmissions by the FDD CM operating
under one of these associated FDD upstream allocated spectrums. Therefore, the FDD CM can transmit legacy (non-
FDD OFDM or SC-QAM) US channels over the range 5 MHz to 85 MHz and FDD US channels in the range of 108
MHz up to the chosen/configured US upper band edge.
The 204 MHz (high-split) diplex filter option is included as a special case for compatibility with a legacy DOCSIS 3.1
high-split network and the upstream FDD transmission is achieved using a single TCS, as it is for a DOCSIS 3.1 CM
supporting the high-split case at 204 MHz and supporting 65 dBmV TCP capability over the range 5 MHz to 204 MHz.
The largest possible FDD upstream allocated spectrum for FDD upstream channels is 576 MHz (108 MHz to
684 MHz). The upper limit of 684 MHz is derived from starting with the lower band edge of mid-split (108 MHz) and
allowing for six OFDMA channels, 96 MHz each. Other FDD upstream allocated spectrum assignments allow for:
      •     Two 96 MHz OFDMA channels (192 MHz), or
      •     Three 96 MHz OFDMA channels (288 MHz), or
      •     Four 96 MHz OFDMA channels (384 MHz).
For each of these extended FDD US cases, the available US spectrum is combined with the range 5 MHz to 85 MHz
for non-FDD, legacy US transmissions. These FDD upstream allocated spectrum options are illustrated in Figure 29.


   High-Split:                                          HS                                                                                                       OFDM
                           Legacy US (Single TCS)   Transition                                    DS SC-QAM or OFDM
   Up to 199 MHz TCS                                   Band                                                                                                       only
                                                        258 MHz


   High-Split:                                          HS
                           Legacy US (Single TCS)   Transition                                    DS SC-QAM or OFDM                                                      OFDM only
   Up to 199 MHz TCS                                   Band
                                                        258 MHz

                                                                       372 MHz
   UHS-300:
                           Legacy
   Up to 80 MHz TCS,                      US             US                                             DS SC-QAM or OFDM                                                OFDM only
   Up to 192 MHz TCS_FDD     US

                                                                                  492 MHz
   UHS-396:
                           Legacy
   Up to 80 MHz TCS,                      US             US            US                                       DS SC-QAM or OFDM                                        OFDM only
   Up to 288 MHz TCS_FDD     US

                                                                                                    606 MHz
   UHS-492:
                           Legacy
   Up to 80 MHz TCS,                      US             US            US         US                                  DS SC-QAM or OFDM                                  OFDM only
   Up to 384 MHz TCS_FDD     US

                                                                                                                            834 MHz
   UHS-684:
                           Legacy
   Up to 80 MHz TCS,                      US             US            US         US         US         US                            DS SC-QAM or OFDM                  OFDM only
   Up to 576 MHz TCS_FDD     US


                                                                                                                              FDD Transition
       Legacy Transition
                                    108         204              300        396        492        588         684           Bands (variable per           1002           1218        1794
      Band (85-108 MHz)
                                                                                                                             implementation)                                         MHz
                            TCS                                        TCS_FDD                                                                        Frequency [MHz]
                                                                                                                                                        (not to scale)

                                     DOCSIS 3.1 backward compatibility Mode,                                         DOCSIS 4.0 FDD Mode,
                                       CM is locked with DOCSIS 3.1 CMTS                                       CM is locked with DOCSIS 4.0 CMTS


                             Figure 29 - Configurable FDD Upstream Allocated Spectrum Bandwidths
An FDD CMTS MUST configure the FDD upstream allocated spectrum to contain only US FDD OFDMA
channels.

7.3.2.5.5.1          FDD CM Operation in a High-Split Network
A high-split network is defined as an HFC network where legacy DOCSIS technologies exist, and the upstream
band extends from 5 MHz to 204 MHz.



112                                                                                    CableLabs                                                                               06/27/25
<!-- PDF_PAGE: 113 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


If the FDD CM is configured for a 204 MHz diplex filter option, the FDD CM MUST support upstream
transmissions from 5 MHz to 204 MHz in legacy DOCSIS 3.1 high-split operational mode. Additional details
regarding this operation for the FDD CM is provided in Section 7.3.2.5.5 above.

7.3.2.6     Operation in the FDD Band in Non-Upstream Allocated Spectrum
The FDD CM supports downstream operation starting at the upper frequency edge of the selected diplex filter option
as shown in Table 30 and Figure 29.

7.3.2.7     Operation in the FDD Transition Bands
The FDD CM does not operate within the selected diplex transition bands as specified in Table 30 and shown in
Figure 29.

7.3.2.8     Operation Above the FDD Transition Band
The FDD CM supports normal DS operation in the spectrum above the FDD transition bands

7.3.3     FDD OFDM Numerology
See [DOCSIS PHYv3.1], section 7.3.

7.3.3.1     Downstream OFDM Numerology
See [DOCSIS PHYv3.1], section 7.3.1.

7.3.3.2     Upstream OFDMA Numerology
See [DOCSIS PHYv3.1], section 7.3.2.

7.3.3.3     Subcarrier Clocking
See [DOCSIS PHYv3.1], section 7.3.3.

7.3.4     Upstream Transmit and Receive

7.3.4.1     Signal Processing Requirements
See [DOCSIS PHYv3.1], section 7.4.1.

7.3.4.2     Time and Frequency Synchronization
See [DOCSIS PHYv3.1], section 7.4.2.

7.3.4.2.1    Channel Frequency Accuracy
This section augments section 7.4.2.1 of [DOCSIS PHYv3.1].
The frequency of the upstream subcarrier clock (or upstream subcarrier spacing) is required to be accurate within
0.4 ppm and each subcarrier frequency accurate within 30 Hz, both relative to the Master Clock reference, and both
for five sigma of the upstream OFDMA transmissions, for subcarrier frequencies up to 684 MHz, depending on the
selected diplex filter option. The measurements of the frequency of the upstream subcarrier clock, and the subcarrier
frequencies, are averaged over the duration of an upstream single frame grant. A constant temperature is maintained
during the measurements within a range of 20 ºC ± 2 ºC. A minimum warm up time of 30 minutes occurs before the
CM frequency measurements are made.

7.3.4.2.2    Channel Timing Accuracy
See [DOCSIS PHYv3.1], section 7.4.2.2.




06/27/25                                           CableLabs                                                   113
