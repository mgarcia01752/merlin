# 5 - Overview and Functional Assumptions

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 34-38
Classification: Main body (mixed overview and requirements)

<!-- PDF_PAGE: 34 -->
CM-SP-PHYv4.0-I07-250627                                        Data-Over-Cable Service Interface Specifications


5 OVERVIEW AND FUNCTIONAL ASSUMPTIONS
This section describes the characteristics of a cable television plant, assumed to be for the purpose of operating a
data-over-cable system.
The cable plants have very diverse physical topologies. These topologies range from fiber to the home node
architectures as well as fiber nodes with many actives in cascade. The plant characteristics described in this section
cover the great majority of plant scenarios.
This section is not a description of CMTS or CM parameters. The data-over-cable system needs to be interoperable
within the environment described in this section.
Whenever a reference in this section to frequency plans, or compatibility with other services, conflicts with any legal
requirement for the area of operation, the latter takes precedence. Any reference to National Television System
Committee (NTSC) analog signals in 6 MHz channels does not imply that such signals are physically present.

5.1       Overview
This specification defines the PHY layer protocol of DOCSIS 4.0. It also describes the channel assumptions over
which DOCSIS 4.0 systems are expected to operate.
DOCSIS 4.0's ultimate service goal of multi-gigabit per second in the downstream and upstream directions resulted
in significant changes in the PHY layer approach compared to earlier DOCSIS versions, in addition to changes on
the cable network assumptions. As with DOCSIS 3.1, DOCSIS 4.0 focuses on the eventual use of the entire spectrum
resources available in the cable environment by the CMTS and CM and on scalable cost-effective techniques to
achieve full spectrum use. However, DOCSIS 4.0 adds Full Duplex (FDX) DOCSIS functionality to enable flexible
usage of a portion of this spectrum for simultaneous upstream and downstream transmissions. DOCSIS 4.0 also
provides an extended frequency division duplex (FDD) alternative to enable FDD upstream spectrum up to 684 MHz
and downstream spectrum to 1794 MHz, further enhancing the efficiency and scalability of the usable spectrum
according to system needs. Unified CMs and Unified CMTSs blend elements of both FDX and FDD by using FDX
channels in the FDX band and also utilizing the downstream spectrum to 1794 MHz. Unified devices operate in
either FDX mode or FDD mode. Unified CMs/CMTSs support both FDX mode and FDD mode, but not
concurrently.
DOCSIS 4.0 assumes the orthogonal frequency division multiplexing (OFDM) downstream signals and orthogonal
frequency division multiple access (OFDMA) upstream signals introduced by [DOCSIS PHYv3.1] to achieve robust
operation and provide more efficient use of the spectrum than previous DOCSIS versions.
The DOCSIS 3.0 systems and earlier versions are sometimes referred to in this document as single carrier QAM
(SC-QAM) systems in contrast to the multicarrier OFDM/OFDMA system introduced by DOCSIS 3.1.
See also [DOCSIS PHYv3.1], section 5.1.

5.2       Functional Assumptions
5.2.1      Equipment Assumptions

5.2.1.1     Frequency Plan
In the downstream direction, the cable system is assumed to have a pass band with a lower edge of 108 MHz and
upper frequency edges extending to 1218 MHz or 1794 MHz for FDX operation. Unified devices have the ability to
utilize cable systems extending up to 1794 MHz while operating in FDX mode. For FDD operation, including
Unified devices operating in FDD mode, the following frequency plan applies to the downstream.
      •    The pass band is 1,536 MHz in width when set to a lower edge of 258 MHz and extends to 1,794 MHz.
      •    Cable systems with downstream lower band edges above 258 MHz extend to 1,794 MHz. Higher upper
           frequency edges may be considered in the specification according to future migrations of the plants.




34                                                   CableLabs                                               06/27/25
<!-- PDF_PAGE: 35 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


    •     Cable systems with downstream lower band edges below 258 MHz are supported by the nodes to assist the
          operators in transitioning to DOCSIS 4.0, but the occupied downstream spectrum is limited to 1,536 MHz.
          FDD nodes that transmit below 258 MHz are considered to be operating in Legacy Transition Mode. Since
          DOCSIS 4.0 cable modems are not required to receive signals below 258 MHz, it is expected that any
          frequencies below 258 MHz will be restricted to video SC-QAM and other non-DOCSIS signals.
Within the downstream pass band, NTSC analog television signals in 6 MHz channels could be present on the
standard HRC or IRC frequency plans of [CTA 542], as well as other narrowband and wideband digital signals.
FDX DOCSIS 4.0 encompasses two frequency plans: CFDX ("classic FDX") and SFDX ("shifted FDX"). The
CFDX frequency plan specifies the continued full use of legacy upstream transmission capabilities in 5–85 MHz.
FDX introduces the possibility of simultaneous upstream and downstream transmissions within the 108–684 MHz
band. Above 684 MHz, the CFDX frequency plan supports legacy downstream transmission, albeit with a transition
band of reduced performance that is unspecified between 684 and 804 MHz. The SFDX frequency plan functions
similarly but offers legacy upstream transmission in 5–204 MHz while providing for simultaneous upstream and
downstream transmissions within the 258–834 MHz band and a transition band of reduced performance between
834 and 984 MHz.
FDD DOCSIS 4.0 supports backwards compatibility to DOCSIS 3.1 high-split operation with full use of legacy
upstream capabilities in 5 MHz to 204 MHz. As with DOCSIS 3.1 high-split, DOCSIS 4.0 supports legacy
downstream transmission beginning at 258 MHz. Unlike DOCSIS 3.1, the DOCSIS 4.0 downstream upper
frequency band edge can extend to 1794 MHz. DOCSIS 4.0 FDD nodes are required to support legacy plants via the
downstream transmission of video SC-QAM and other non-DOCSIS signals below 258 MHz to allow operators a
more gradual transition to a full FDD implementation of DOCSIS 4.0.
FDD DOCSIS 4.0 introduces ultra-high split (UHS) operation with the potential to add OFDMA capacity across the
added upstream frequencies. UHS adds the possibility of extended splits at 300 MHz (UHS-300), 396 MHz (UHS-
396), 492 MHz (UHS-492), and 684 MHz (UHS-684). Each of the new splits has a range in which the lower
downstream transmission band edge can lie as defined later in this specification. As with DOCSIS 4.0 high-split, the
UHS downstream upper frequency band edge can extend to 1794 MHz.
UHS operation includes continued support for DOCSIS 3.1 CMs, which can fully utilize legacy upstream transmission
capabilities in 5 MHz to 204 MHz. DOCSIS 4.0 CMs can utilize legacy upstream transmission capabilities in 5 MHz to
85 MHz while also supporting OFDMA transmissions from 108 MHz to the chosen UHS extended split. For more
detailed descriptions and graphical representations, refer to Section 7.3.2.5 and its subsections.

5.2.1.2    Compatibility with Other Services
The CM MUST coexist with any services on the cable network.
The CMTS MUST coexist with any services on the cable network.
In particular:
    •     The CMTS MUST be interoperable in the cable spectrum assigned for CMTS and CM interoperation while
          the balance of the cable spectrum is occupied by any combination of television and other signals.
    •     The CM MUST be interoperable in the cable spectrum assigned for CMTS and CM interoperation while
          the balance of the cable spectrum is occupied by any combination of television and other signals.
    •     The CMTS MUST NOT cause harmful interference to any other services that are assigned to the cable
          network in spectrum outside of that allocated to the CMTS.
    •     The CM MUST NOT cause harmful interference to any other services that are assigned to the cable
          network in spectrum outside of that allocated to the CM.
Harmful interference is understood as:
    •     No measurable degradation (highest level of compatibility),
    •     No degradation below the perceptible level of impairments for all services (standard or medium level of
          compatibility), or



06/27/25                                            CableLabs                                                      35
<!-- PDF_PAGE: 36 -->
CM-SP-PHYv4.0-I07-250627                                           Data-Over-Cable Service Interface Specifications


     •    No degradation below the minimal standards accepted by the industry (for example, FCC for analog video
          services) or other service provider (minimal level of compatibility).

5.2.1.3     Fault Isolation Impact on Other Users
As CMTS transmissions are on a shared-media, point-to-multipoint system, fault-isolation procedures should take
into account the potential harmful impact of faults and fault-isolation procedures on numerous users of the data-
over-cable, video, and other services.
For the interpretation of harmful impact, see Section 5.2.1.2 above.

5.2.1.4     Cable System Terminal Devices
The CM is expected to meet and preferably exceed all applicable regulations for Cable System Termination Devices
and Cable Ready Consumer Equipment as defined in FCC Part 15 [FCC15] and Part 76 [FCC76]. None of these
specific requirements may be used to relax any of the specifications contained elsewhere within this document.

5.2.2     RF Channel Assumptions
The data-over-cable system, configured with at least one set of defined physical-layer parameters (e.g., modulation,
interleaver depth, etc.) from the range of configurable settings described in this specification, is expected to be
interoperable on cable networks having characteristics defined in this section. This is accomplished in such a manner
that the forward error correction provides for equivalent operation in a cable system both with and without the
impaired channel characteristics described below.

5.2.2.1     Transmission Downstream
The RF channel transmission characteristics of the cable network in the downstream direction are described in
Table 3. These numbers assume the total average power of a digital signal in a 192 MHz channel bandwidth for
subcarrier levels unless indicated otherwise. For impairment levels, the numbers in Table 3 assume average power in
a bandwidth in which the impairment levels are measured in a standard manner for cable TV systems. For analog
signal levels, the numbers in Table 3 assume peak envelope power in a 6 MHz channel bandwidth. All conditions
are present concurrently. It is expected that the HFC plant will have better conditions for DOCSIS 4.0 to provide the
higher throughput and capacities anticipated.
                        Table 3 - Typical Downstream RF Channel Transmission Characteristics

                                Parameter                                                         Value
Frequency range                                                           CFDX: 108 to 1218 MHz or 1794 MHz
                                                                          SFDX: 258 to 1218 MHz or 1794 MHz
                                                                          FDD: 1,536 MHz of occupied downstream spectrum in
                                                                          the range from 54 MHz to 1,794 MHz1
RF channel spacing (design bandwidth)                                     24 to 192 MHz
One way transit delay from headend to most distant customer               ≤ 0.400 ms (typically much less)
Signal to composite noise ratio                                           ≥ 35 dB
Carrier-to-composite triple beat distortion ratio                         Not less than 41 dB
Carrier-to-composite second order distortion ratio                        Not less than 41 dB
Carrier-to-cross-modulation ratio                                         Not less than 41 dB
Carrier-to-any other discrete interference (ingress)                      Not less than 41 dB
Maximum amplitude variation across the 6 MHz channel (digital channels)   ≤ 1.74 dB pk-pk/6 MHz
Group delay variation                                                     ≤113 ns over 24 MHz2




36                                                      CableLabs                                               06/27/25
<!-- PDF_PAGE: 37 -->
Physical Layer Specification                                                                CM-SP-PHYv4.0-I07-250627


                               Parameter                                                            Value
Micro-reflections bound for dominant single echo                         -20 dBc for echoes ≤ 0.5 µs
                                                                         -25 dBc for echoes ≤ 1.0 µs
                                                                         -30 dBc for echoes ≤ 1.5 µs
                                                                         -35 dBc for echoes > 2.0 µs
                                                                         -40 dBc for echoes > 3.0 µs
                                                                         -45 dBc for echoes > 4.5 µs
                                                                         -50 dBc for echoes > 5.0 µs
Carrier hum modulation                                                   Not greater than -30 dBc (3%)
Maximum analog video carrier level at the CM input                       17 dBmV
Maximum number of analog carriers                                        121
TABLE NOTES
NOTE 1 Frequencies below 258 MHz are for legacy video SC-QAM and other non-DOCSIS signals to support operators in the
       transition to DOCSIS 4.0. DOCSIS 4.0 FDD nodes transmitting at frequencies below 258 MHz are said to be operating in
       legacy transition mode.
NOTE 2 Cascaded group delay could possibly exceed the ≤113 ns value within approximately 30 MHz above the downstream
       spectrum's lower band edge, depending on cascade depth, diplex filter design, and actual band split.



5.2.2.2     Transmission Upstream
The RF channel transmission characteristics of the cable network in the upstream direction are described in Table 4.
No combination of the following parameters will exceed any stated interface limit defined elsewhere in this
specification. Transmission is from the CM output at the customer location to the headend.
                         Table 4 - Typical Upstream RF Channel Transmission Characteristics

                             Parameter                                                          Value
Frequency range                                                       CFDX: 5 to 85 MHz and 108 to 684 MHz
                                                                      SFDX: 5 to 204 MHz and 258 to 834 MHz
                                                                      FDD high-split backward compatibility: 5 MHz to 204 MHz
                                                                      FDD UHS (DOCSIS 3.1 CMs): 5 MHz to 204 MHz
                                                                      FDD UHS (DOCSIS 4.0 CMs): 5 to 85 and 108 to xxx MHz
                                                                       (where xxx = 300, 396, 492, or 684 MHz)
One way transit delay from most distant customer to headend           ≤ 0.400 ms (typically much less)
Carrier-to-interference plus ingress (the sum of noise, distortion,   Not less than 25 dB
common-path distortion and cross modulation and the sum of discrete
and broadband ingress signals, impulse noise excluded) ratio
Carrier hum modulation                                                Not greater than -26 dBc (5.0%)
Maximum amplitude variation across the 6 MHz channel (digital         ≤ 2.78 dB pk-pk/6 MHz
channels)
Group delay variation                                                 ≤163 ns over 24 MHz
Micro-reflections bound for dominant single echo                      -16 dBc for echoes ≤ 0.5 µs
                                                                      -22 dBc for echoes ≤ 1.0 µs
                                                                      -29 dBc for echoes ≤ 1.5 µs
                                                                      -35 dBc for echoes > 2.0 µs
                                                                      -42 dBc for echoes > 3.0 µs
                                                                      -51 dBc for echoes > 4.5 µs
Seasonal and diurnal reverse gain (loss) variation                    Not greater than 14 dB min to max
TABLE NOTE
NOTE 1 Cascaded group delay could possibly exceed the ≤163 ns value within approximately 10 MHz of the upstream spectrum's
       lower and upper band edges, depending on cascade depth, diplex filter design, and actual band split.




06/27/25                                               CableLabs                                                         37
<!-- PDF_PAGE: 38 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


5.2.2.2.1    Availability
Cable network availability is typically greater than 99.9%.

5.2.3    Transmission Levels
The nominal power level of the upstream CM signal(s) will be as low as possible to achieve the required margin
above noise and interference. Uniform power loading per unit bandwidth is commonly followed in setting upstream
signal levels, with specific levels established by the cable network operator to achieve the required carrier-to-noise
and carrier-to-interference ratios.

5.2.4    Frequency Inversion
There will be no frequency inversion in the transmission path in either the downstream or the upstream directions,
i.e., a positive change in frequency at the input to the cable network will result in a positive change in frequency at
the output.




38                                                   CableLabs                                               06/27/25
