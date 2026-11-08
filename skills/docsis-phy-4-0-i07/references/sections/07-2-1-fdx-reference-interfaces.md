# 7.2.1 - Full Duplex Node Reference Interfaces

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 46-46
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
