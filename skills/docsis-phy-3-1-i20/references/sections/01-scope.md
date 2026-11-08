# 1 - Scope

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 12-16
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 12 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


1 SCOPE
1.1       Introduction and Purpose
This specification is part of the DOCSIS family of specifications developed by Cable Television Laboratories
(CableLabs). In particular, this specification is part of a series of specifications that defines the fifth generation of
high-speed data-over-cable systems, commonly referred to as the DOCSIS 3.1 specifications. This specification was
developed for the benefit of the cable industry, and includes contributions by operators and vendors from North and
South America, Europe and Asia.
This generation of the DOCSIS specifications builds upon the previous generations of DOCSIS specifications
(commonly referred to as the DOCSIS 3.0 and earlier specifications), leveraging the existing Media Access Control
(MAC) and Physical (PHY) layers, but with the addition of a new PHY layer designed to improve spectral
efficiency and provide better scaling for larger bandwidths (and appropriate updates to the MAC and management
layers to support the new PHY layer). It includes backward compatibility for the existing PHY layers in order to
enable a seamless migration to the new technology.
There are differences in the cable spectrum planning practices adopted for different networks in the world. For the
new PHY layer defined in this specification, there is flexibility to deploy the technology in any spectrum plan;
therefore, no special accommodation for different regions of the world is required for this new PHY layer.
However, due to the inclusion of the DOCSIS 3.0 PHY layers for backward compatibility purposes, there is still a
need for different region-specific physical layer technologies. Therefore, three options for physical layer
technologies are included in this specification, which have equal priority and are not required to be interoperable.
One technology option is based on the downstream channel identification plan that is deployed in North America
using 6 MHz spacing. The second technology option is based on the corresponding European multi-program
television distribution. The third technology option is based on the corresponding Chinese multi-program television
distribution. All three options have the same status, notwithstanding that the document structure does not reflect this
equal priority. The first of these options is defined in Sections 5 and 6, whereas the second is defined by replacing
the content of those sections with the content of Annex C. The third is defined by replacing the content of those
sections with the content of Annex D. Correspondingly, [ITU-T J.83-B] and [CEA-542] apply only to the first
option, and [EN 300 429] applies to the second and third. Compliance with this document requires compliance with
one of these implementations, but not with all three. It is not required that equipment built to one option interoperate
with equipment built to the other.
Compliance with frequency planning and EMC requirements is not covered by this specification and remains the
operators' responsibility. In this respect, [FCC15] and [FCC76] are relevant to the USA; [CAN/CSA CISPR 22-10]
and [ICES 003 Class A] to Canada; [EG 201 212], [EN 50083-1], [EN 50083-2], [EN 50083-7], [EN 61000-6-1],
and [EN 61000-6-3] are relevant to the European Union; [GB 8898-2011] and [GB/T 11318.1-1996] are relevant to
China.

1.2       Background
1.2.1      Broadband Access Network
A coaxial-based broadband access network is assumed. This may take the form of either an all-coax or hybrid-
fiber/coax (HFC) network. The generic term "cable network" is used here to cover all cases.
A cable network uses a tree-and-branch architecture with analog transmission. The key functional characteristics
assumed in this document are the following:
      •    Two-way transmission.
      •    A maximum optical/electrical spacing between the CMTS and the most distant CM of 100 miles (160 km)
           in each direction, although typical maximum separation may be 10-15 miles (16-24 km).




12                                                   CableLabs                                                04/19/23

<!-- PDF_PAGE: 13 -->
Physical Layer Specification                                                            CM-SP-PHYv3.1-I20-230419


1.2.2     Network and System Architecture

1.2.2.1    The DOCSIS Network
The elements that participate in the provisioning of DOCSIS services are shown in the following figure:




                                           Figure 1 - The DOCSIS Network
The CM connects to the operator's HFC network and to a home network, bridging packets between them. Many
CPEs can connect to the CMs' LAN interfaces. CPE can be embedded with the CM in a single device, or they can be
separated into standalone devices, as shown in Figure 1. CPE may use IPv4, IPv6 or both forms of IP addressing.
Examples of typical CPE are gateways, home routers, set-top devices, personal computers, etc.
The CMTS connects the operator's back office and core network to the HFC network. The CMTS's main function is
to forward packets between these two domains, and between upstream and downstream channels on the HFC
network.
Various applications are used to provide back office configuration and other support to the devices on the DOCSIS
network. These applications use IPv4 and/or IPv6 as appropriate to the particular operator's deployment. The
following applications include:
Provisioning Systems:
    •     The DHCP servers provide the CM with initial configuration information, including the device IP
          address(es), when the CM boots.
    •     The Config File server is used to download configuration files to CMs when they boot. Configuration files
          are in binary format and permit the configuration of the CM's parameters.
    •     The Software Download server is used to download software upgrades to the CM.
    •     The Time Protocol server provides Time Protocol clients, typically CMs, with the current time of day.
    •     Certificate Revocation server provides certificate status.
Network Management System (NMS):
    •     The SNMP Manager allows the operator to configure and monitor SNMP Agents, typically the CM and the
          CMTS.
    •     The Syslog server collects messages pertaining to the operation of devices.
    •     The IPDR Collector server allows the operator to collect bulk statistics in an efficient manner.




04/19/23                                             CableLabs                                                   13

<!-- PDF_PAGE: 14 -->
CM-SP-PHYv3.1-I20-230419                                                                       Data-Over-Cable Service Interface Specifications


1.2.3      Service Goals
As cable operators have widely deployed high-speed data services on cable television systems, the demand for
bandwidth has increased. To this end, CableLabs' member companies have decided to add new features to the
DOCSIS specification for the purpose of increasing capacity, increasing peak speeds, improving scalability,
enhancing network maintenance practices and deploying new service offerings.
The DOCSIS system allows transparent bi-directional transfer of Internet Protocol (IP) traffic, between the cable
system headend and customer locations, over an all-coaxial or HFC cable network. This is shown in simplified form
in Figure 2.




                              Figure 2 - Transparent IP Traffic through the Data-Over-Cable System

1.2.4      Statement of Compatibility
This specification defines the DOCSIS 3.1 interface. Prior generations of DOCSIS were commonly referred to as the
DOCSIS 1.0, 1.1, 2.0, and 3.0 interfaces. DOCSIS 3.1 is backward-compatible with some equipment built to the
previous specifications. DOCSIS 3.1-compliant CMs interoperate seamlessly with DOCSIS 3.1 and DOCSIS 3.0
CMTSs. DOCSIS 3.1-compliant CMTSs seamlessly support DOCSIS 3.1, DOCSIS 3.0, DOCSIS 2.0, and
DOCSIS 1.1 CMs.

1.2.5      Reference Architecture
                                       Distribution Hub or Headend
                                         DOCSIS Timing
                                         Interface (DTI)       DOCSIS                               Operations               Operations Support System
                                                             Timing Server                        Support System                  Interface (OSSI)




                                                                        EQAM
                                      M-CMTS           Downstream
                                       Core            External Phy                    Downstream
                                                     Interface (DEPI)                  RF Interface
                 Network Side                                                           ( DRFI)
                Interface (NSI)                Upstream                                                                                      Cable Modem to
                                               Receiver
                                                               M-CMTS                                                                         CPE Interface
                                                                                 Downstream
                                                                                                                                                 (CMCI)
                                                                                 RF Network
                                                                                                     Tx
                                                                                                  Opt.
                                                                                                   Tx                    Fiber
                                                                                                  Tx                    Fiber
                                                                                                                         Node                               Customer
                                                                                                          Fiber       Fiber
                                                                                                                       Node    Coax       Cable             Premises
                                                                                                       Distribution   Node Distribution   Modem             Equipment
        Wide Area                                                                                    Rx                                    (CM)
                                                                                                  Opt.
                                                                                                   Rx
         Network                                                                                  Rx
                                                                                   Upstream
                                                   I-CMTS                         RF Network
                                                   / CCAP



                                                                                                                                                  MAC & Upper Layer
                                                                                                                                               Protocols Interface (MULPI)
                                                                                                                                               & Security Interface (SEC)
                      NOTE: Gray-shaded areas represent related                          Physical Layer Interface
                      functionality, but are out of scope of this document.                       (PHY)




                                              Figure 3 - Data-Over-Cable Reference Architecture
The reference architecture for data-over-cable services and interfaces is shown in Figure 3.



14                                                                             CableLabs                                                                          04/19/23

<!-- PDF_PAGE: 15 -->
Physical Layer Specification                                                                    CM-SP-PHYv3.1-I20-230419


1.2.6     DOCSIS 3.1 Documents
A list of the specifications in the DOCSIS 3.1 series is provided in Table 1. For further information, please refer to
http://www.cablemodem.com.
                                     Table 1 - DOCSIS 3.1 Series of Specifications

           Designation                                                         Title
      CM-SP-PHYv3.1                 Physical Layer Specification
      CM-SP-MULPIv3.1               MAC and Upper Layer Protocols Interface Specification
      CM-SP-CM-OSSIv3.1             Cable Modem Operations Support System Interface Specification
      CM-SP-CCAP-OSSIv3.1           CCAP Operations Support System Interface Specification
      CM-SP-SECv3.1                 Security Specification
      CM-SP-CMCIv3.0                Cable Modem CPE Interface Specification

This specification defines the interface for the physical layer.
Related DOCSIS specifications are listed in Table 2.
                                      Table 2 - DOCSIS 3.1 Related Specifications

           Designation                                                         Title
      CM-SP-eDOCSIS                 eDOCSIS™ Specification
      CM-SP-DRFI                    Downstream Radio Frequency Interface Specification
      CM-SP-DTI                     DOCSIS Timing Interface Specification
      CM-SP-DEPI                    Downstream External PHY Interface Specification
      CM-SP-DSG                     DOCSIS Set-Top Gateway Interface Specification
      CM-SP-ERMI                    Edge Resource Manager Interface Specification
      CM-SP-L2VPN                   Layer 2 Virtual Private Networks Specification
      CM-SP-TEI                     TDM Emulation Interfaces Specification


1.3     Requirements
Throughout this document, the words that are used to define the significance of particular requirements are
capitalized. These words are:
  "MUST"                 This word means that the item is an absolute requirement of this specification.
  "MUST NOT"             This phrase means that the item is an absolute prohibition of this specification.
  "SHOULD"               This word means that there may exist valid reasons in particular circumstances to ignore this item,
                         but the full implications should be understood and the case carefully weighed before choosing a
                         different course.
  "SHOULD NOT"           This phrase means that there may exist valid reasons in particular circumstances when the listed
                         behavior is acceptable or even useful, but the full implications should be understood and the case
                         carefully weighed before implementing any behavior described with this label.
  "MAY"                  This word means that this item is truly optional. One vendor may choose to include the item
                         because a particular marketplace requires it or because it enhances the product, for example;
                         another vendor may omit the same item.

This document defines many features and parameters, and a valid range for each parameter is usually specified.
Equipment (CM and CMTS) requirements are always explicitly stated. Equipment must comply with all mandatory
(MUST and MUST NOT) requirements to be considered compliant with this specification. Support of non-
mandatory features and parameter values is optional.




04/19/23                                                 CableLabs                                                            15

<!-- PDF_PAGE: 16 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


1.4    Conventions
In this specification, the following convention applies any time a bit field is displayed in a figure. The bit field
should be interpreted by reading the figure from left to right, then top to bottom, with the most-significant bit (MSB)
being the first bit read, and the least-significant bit (LSB) being the last bit read.

1.5    Organization of Document
Section 1 provides an overview of the DOCSIS 3.1 series of specifications including the DOCSIS reference
architecture and statement of compatibility.
Section 2 includes a list of normative and informative references used within this specification.
Section 3 defines the terms used throughout this specification.
Section 4 defines the abbreviations and acronyms used throughout this specification.
Section 5 provides a technical overview and lists the key features of DOCSIS 3.1 technology for the functional area
of this specification; it also describes the key functional assumptions for the DOCSIS 3.1 system.
Section 6 defines the interface and related requirements for a DOCSIS 3.1 CM or CMTS is operating with SC-QAM
operation only, with no OFDM/OFDMA operation and for the SC-QAM (Single Carrier QAM) channels with
simultaneous operation of SC-QAM and OFDM/OFDMA channels unless otherwise noted for each of: the CM
downstream and upstream physical layer; and for the CMTS downstream upstream physical layer.
Section 7 defines the interface and related requirements for operation with the new DOCSIS 3.1 channels, as well as
for combined operation of DOCSIS 3.0 and DOCSIS 3.1 channels. This is addressed for each of: the CM
downstream and upstream physical layer; and for the CMTS downstream upstream physical layer.
Section 8 defines PHY-MAC convergence - how information is transferred between the MAC layer and the PHY
layer - in both the upstream and downstream.
Section 9 defines the requirements supporting Proactive Network Maintenance (PNM).
Annex A presents requirements for QAM Constellation Mappings and Annex B contains the requirement for RFoG
Operating Mode.
Annex C and Annex D define the requirements for additions and modifications for European Specification with SC-
QAM operation and additions and modifications for Chinese Specification with SC-QAM operation respectively.
Annex E contains the normative 24-bit Cyclic Redundancy Check (CRC) Code.
The informative appendices cover various sample codes, use cases, proposed configuration parameters, and
suggested algorithms.




16                                                  CableLabs                                               04/19/23
