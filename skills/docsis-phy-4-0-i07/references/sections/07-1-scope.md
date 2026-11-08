# 7.1 - OFDM PHY Scope

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 45-45
Classification: Main body

<!-- PDF_PAGE: 45 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


7 PHY SUBLAYER FOR OFDM
7.1    Scope
This specification defines the electrical characteristics and signal processing operations for a cable modem (CM) and
cable modem termination system (CMTS). It is the intent of this specification to define an interoperable CM and
CMTS such that any implementation of a CM can work with any CMTS. It is not the intent of this specification to
imply any specific implementation.
This section describes CM and CMTS physical layer requirements for DOCSIS 4.0-compliant devices in either Full
Duplex DOCSIS (FDX) or frequency division duplexed (FDD) environments. This section also describes a Unified
device capability for Unified operation.
DOCSIS 4.0 FDX is an expansion of the DOCSIS 3.1 specification that is targeted at significantly increasing
upstream capacity by using the spectrum currently used for downstream transmission for simultaneous upstream and
downstream communications via full duplex communications. Therefore, many sections refer to basic OFDM
sublayer definitions described in the [DOCSIS PHYv3.1] specification.
DOCSIS 4.0 FDD is an expansion of the DOCSIS 3.1 specification that is targeted at significantly increasing
upstream and downstream capacity by extending the available upstream dedicated spectrum beyond previously
defined upper bounds to as much as 684 MHz and also extending the available downstream spectrum capabilities to
1794 MHz, while maintaining spectrum separation between upstream and downstream transmissions at a predefined
frequency.
Unified operation is a means of leveraging capabilities of both FDX and FDD. In Unified operation, FDX channels
are used in the FDX band, and the downstream spectrum is utilized to 1794 MHz. A Unified CM can operate in any
of the following modes: DOCSIS 3.1 mode, FDX mode, or FDD mode. Unified CMs, Unified Nodes, and Unified
RMDs/RPDs support downstream operation at frequencies up to 1794 MHz. Subsequent subsections of this
document provide specifications for such operation in both FDX mode and FDD mode.
The following sections are intended to treat separately the PHY OFDM sublayer requirements for FDX or FDD
capability requirements. Section 7.2 and its subsections address the PHY sublayer requirements specific to DOCSIS
4.0 FDX functionality. Section 7.3 and its subsections address the PHY sublayer requirements specific to DOCSIS
4.0 FDD functionality.

7.2    PHY Sublayer for FDX OFDM
Full duplex capability requires additional functions to be added to the DOCSIS 3.1 requirements. These new
functions are specified in this specification. As with previous generations of DOCSIS technologies, DOCSIS 4.0-
compliant devices will be backward-compatible with previous generations of DOCSIS technology. The FDX
allocated spectrum is subdivided into FDX channels that can be assigned to modems according to system
requirements. FDX channels carry both upstream and downstream traffic. The CMTS assignment of FDX channels
within the FDX band for Full Duplex DOCSIS operation can be done incrementally over time as a transition
strategy, from existing DOCSIS networks to Full Duplex DOCSIS networks, as FDX-capable CMTSs and modems
become available.
For an FDX DOCSIS system, a distributed architecture is assumed due to the echo cancellation functionality that is
required. Thus, this portion of the specification refers to the physical layer functionality of an FDX-capable CMTS
as the FDX node. The CMTS is occasionally used to refer to the total functionality across the MAC layer and the
Physical layer.
An FDX-compliant FDX node supports simultaneous upstream and downstream communications over each FDX
channel; this is enabled by cancellation techniques for self-interference and echo cancellation. FDX-compliant cable
modems will operate in frequency division duplexing mode, where on any FDX channel, the CM is either
transmitting in the upstream or receiving in the downstream. An FDX-compliant CMTS allocates FDX channels to
cable modems by providing modems access to upstream and downstream channels through FDD; a CM's operation
on an FDX channel in either US or DS can be changed by the CMTS. FDX channels can be bonded with non-FDX
channels and with other FDX channels.



06/27/25                                           CableLabs                                                    45
