# 7.3.1 - FDD Node Reference Interfaces

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 107-107
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
