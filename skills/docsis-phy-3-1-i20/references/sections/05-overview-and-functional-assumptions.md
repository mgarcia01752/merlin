# 5 - Overview and Functional Assumptions

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 34-38
Classification: Main body (mixed overview and requirements)

<!-- PDF_PAGE: 34 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


5 OVERVIEW AND FUNCTIONAL ASSUMPTIONS
This section describes the characteristics of a cable television plant, assumed to be for the purpose of operating a
data-over-cable system.
The cable plants have very diverse physical topologies. These topologies range from fiber to the home node
architectures as well as fiber nodes with many actives in cascade. The plant characteristics described in this section
covers the great majority of plant scenarios.
This section is not a description of CMTS or CM parameters. The data-over-cable system MUST be interoperable
within the environment described in this section.
Whenever a reference in this section to frequency plans, or compatibility with other services, conflicts with any legal
requirement for the area of operation, the latter takes precedence. Any reference to National Television System
Committee (NTSC) analog signals in 6 MHz channels does not imply that such signals are physically present.

5.1    Overview
This specification defines the PHY layer protocol of DOCSIS 3.1. It also describes the channel assumptions over
which DOCSIS 3.1 systems are expected to operate.
DOCSIS 3.1 ultimate service goal of multi-gigabit per second in the downstream direction and gigabit per second in
the upstream direction resulted in significant changes in the PHY layer approach compared to earlier DOCSIS
versions in addition to changes on the cable network assumptions. DOCSIS 3.1 focuses on the eventual use of the
entire spectrum resources available in cable environment by the CMTS and CM and on scalable cost effective
techniques to achieve full spectrum use.
DOCSIS 3.1 assumes Orthogonal Frequency Division Multiplexing (OFDM) downstream signals and Orthogonal
Frequency Division Multiple Access (OFDMA) upstream signals to achieve robust operation and provide more
efficient use of the spectrum than previous DOCSIS versions. In order to reach the target service goal in the upstream
direction, plant changes on the upstream/downstream spectrum split are expected. The DOCSIS 3.1 system will have
options of several split configurations that can be exercised based on traffic demand, services offered and the capability
of the cable plant. In the downstream direction, HFC plant spectrum extended beyond the current 1002 MHz is expected.
The DOCSIS 3.0 systems and earlier versions are sometimes referred to in this document as single carrier QAM
(SC-QAM) systems in contrast to the multicarrier DOCSIS 3.1 OFDM/OFDMA system.
The OFDM downstream multicarrier system is composed of a large number of subcarriers that have either 25 kHz or
50 kHz spacing. These subcarriers are grouped into independently configurable OFDM channels each occupying a
spectrum of up to 192 MHz in the downstream, totaling 7680 25 kHz subcarriers or 3840 50 kHz subcarriers; of
which up to 7600 (25 kHz) or 3800 (50 kHz) active subcarriers span 190 MHz. The OFDMA upstream multicarrier
system is also composed of either 25 kHz or 50 kHz subcarriers. In the upstream, the subcarriers are grouped into
independently configurable OFDMA channels each of up to 95 MHz encompassed spectrum, totaling 3800 25 kHz
spaced subcarriers or 1900 50 kHz spaced subcarriers or 1920 50 kHz spaced subcarriers. Many parameters of these
channels can be independently configured thereby optimizing configuration based on channel conditions.
The cable topologies have been evolving to smaller node architectures with fewer amplifiers in cascade. This natural
cable network evolution and gradual reduction in node sizes bring a corresponding improvement in channel
conditions. A DOCSIS 3.1 goal is to leverage the expected network improvements and achieve higher efficiency
levels corresponding to improvement in channel conditions. In the downstream and in the upstream directions,
profiles can be defined to match the transmission configuration to the channel conditions with greater granularity.
DOCSIS 3.1 technology is able to operate in classic cable network topologies, but those networks may not be able to
achieve full capabilities of DOCSIS 3.1 bandwidth efficiencies.
An assumption in the topology configuration of DOCSIS 3.1 is that the CM is predominantly placed in a gateway
architecture configuration. Specifically the CM is located either at the drop-home boundary or after the first two-
way splitter within the customer premises. This configuration limits potential attenuation within the home
environment. This reduced attenuation is leveraged to enable higher efficiencies intended in DOCSIS 3.1.




34                                                   CableLabs                                                 04/19/23

<!-- PDF_PAGE: 35 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419


Another intent of DOCSIS 3.1 is to leverage the granularity from multiple narrowband subcarriers to exclude
unwanted spectrum regions from transmission so that interferers can be avoided. Leveraging the same mechanism,
coexistence with existing systems can be implemented by carving out a portion of the spectrum to allow for another
transmission to co-exist.
It is expected that CMs and CMTSs are able to operate in multiple modes. A mode could be pure Single Carrier
Quadrature Amplitude Modulation (SC-QAM), pure OFDM transmission or a combination of the two. This
flexibility enables a smooth transition between SC-QAM and OFDM systems.
DOCSIS 3.1 uses Low-Density-parity-Check (LDPC) Forward Error Correction (FEC) coding instead of the Reed-
Solomon used in DOCSIS 3.0 and earlier versions. Long FEC codeword types are defined in the upstream and
downstream to optimize efficiency. LDPC FEC along with frequency and time interleaving is used to provide
robustness against narrowband interferers and burst events.
In several instances, equivalent characterization and metrics to those used in DOCSIS 3.0 and earlier versions have
been devised to facilitate comparison of requirements among different versions of the specification.
The DOCSIS 3.1 suite of specifications includes considerations to improve and optimize operation under special
modes. One mode is a light sleep mode that is introduced to minimize CM power consumption. Also, some features
are defined primarily operation in fiber to the home (FTTH) network architectures configured for RFoG.

5.2       Functional Assumptions
5.2.1      Equipment Assumptions

5.2.1.1     Frequency Plan
In the downstream direction, the cable system is assumed to have a pass band with a lower edge of either 54 MHz,
87.5 MHz, 108 MHz or 258 MHz, and an upper edge that is implementation-dependent but is typically in the range of
550 to 1002 MHz. Upper frequency edges extending to 1218 MHz, 1794 MHz and others are expected in future
migrations of the plants. Within that pass band, NTSC analog television signals in 6 MHz channels are assumed present
on the standard, HRC or IRC frequency plans of [CTA 542], as well as other narrowband and wideband digital signals.
In the upstream direction, the cable system may have a 5-42 MHz, 5-65 MHz, 5-85 MHz, 5-204 MHz or pass bands
with an upper band edge beyond 204 MHz. NTSC analog television signals in 6 MHz channels may be present, as
well as other signals.

5.2.1.2     Compatibility with Other Services
The CM MUST coexist with any services on the cable network.
The CMTS MUST coexist with any services on the cable network.
In particular:
      •    The CMTS MUST be interoperable in the cable spectrum assigned for CMTS and CM interoperation while
           the balance of the cable spectrum is occupied by any combination of television and other signals.
      •    The CM MUST be interoperable in the cable spectrum assigned for CMTS and CM interoperation while
           the balance of the cable spectrum is occupied by any combination of television and other signals.
      •    The CMTS MUST NOT cause harmful interference to any other services that are assigned to the cable
           network in spectrum outside of that allocated to the CMTS.
      •    The CM MUST NOT cause harmful interference to any other services that are assigned to the cable
           network in spectrum outside of that allocated to the CM.
Harmful interference is understood as:
      •    No measurable degradation (highest level of compatibility),
      •    No degradation below the perceptible level of impairments for all services (standard or medium level of
           compatibility), or


04/19/23                                            CableLabs                                                       35

<!-- PDF_PAGE: 36 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


     •    No degradation below the minimal standards accepted by the industry (for example, FCC for analog video
          services) or other service provider (minimal level of compatibility).

5.2.1.3     Fault Isolation Impact on Other Users
As CMTS transmissions are on a shared-media, point-to-multipoint system, fault-isolation procedures should take
into account the potential harmful impact of faults and fault-isolation procedures on numerous users of the data-
over-cable, video and other services.
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
The RF channel transmission characteristics of the cable network in the downstream direction are described in Table 3.
These numbers assume total average power of a digital signal in a 192 MHz channel bandwidth for subcarrier levels
unless indicated otherwise. For impairment levels, the numbers in Table 3 assume average power in a bandwidth in
which the impairment levels are measured in a standard manner for cable TV systems. For analog signal levels, the
numbers in Table 3 assume peak envelope power in a 6 MHz channel bandwidth. All conditions are present
concurrently. It is expected that the HFC plant will have better conditions for DOCSIS 3.1 to provide the higher
throughput and capacities anticipated.
                       Table 3 - Typical Downstream RF Channel Transmission Characteristics

                           Parameter                                                      Value
  Frequency range                                               Cable system normal downstream operating range is from
                                                                54 MHz to 1002 MHz. Extended operating ranges include
                                                                lower downstream edges of 108 MHz and 258 MHz and
                                                                upper downstream edges of 1218 MHz and 1794 MHz.
  RF channel spacing (design bandwidth)                         24 to 192 MHz
  One way transit delay from headend to most distant customer   ≤ 0.400 ms (typically much less)
  Signal to Composite Noise Ratio                               ≥ 35 dB
  Carrier-to-Composite triple beat distortion ratio             Not less than 41 dB
  Carrier-to-Composite second order distortion ratio            Not less than 41 dB
  Carrier-to-Cross-modulation ratio                             Not less than 41 dB
  Carrier-to-any other discrete interference (ingress)          Not less than 41 dB
  Maximum amplitude variation across the 6 MHz channel          ≤ 1.74 dB pk-pk/6 MHz
  (digital channels)
  Group Delay Variation                                         ≤113 ns over 24 MHz




36                                                       CableLabs                                             04/19/23

<!-- PDF_PAGE: 37 -->
Physical Layer Specification                                                                  CM-SP-PHYv3.1-I20-230419


                           Parameter                                                          Value
  Micro-reflections bound for dominant single echo               -20 dBc for echos ≤ 0.5 µs
                                                                 -25 dBc for echos ≤ 1.0 µs
                                                                 -30 dBc for echos ≤ 1.5 µs
                                                                 -35 dBc for echos > 2.0 µs
                                                                 -40 dBc for echos > 3.0 µs
                                                                 -45 dBc for echos > 4.5 µs
                                                                 -50 dBc for echos > 5.0 µs
  Carrier hum modulation                                         Not greater than -30 dBc (3%)
  Maximum analog video carrier level at the CM input             17 dBmV
  Maximum number of analog carriers                              121
  NOTE: Cascaded group delay could possibly exceed the ≤113 ns value within approximately 30 MHz above the downstream
  spectrum's lower band edge, depending on cascade depth, diplex filter design, and actual band split.


5.2.2.2     Transmission Upstream
The RF channel transmission characteristics of the cable network in the upstream direction are described in Table 4.
No combination of the following parameters will exceed any stated interface limit defined elsewhere in this
specification. Transmission is from the CM output at the customer location to the headend.
                        Table 4 - Typical Upstream RF Channel Transmission Characteristics

                           Parameter                                                          Value
  Frequency range                                                Cable standard upstream frequency range is from a lower
                                                                 band-edge of 5 MHz to upper band-edges to 42 MHz and
                                                                 65 MHz. Extended upstream frequency ranges include upper
                                                                 upstream band-edges of 85 MHz, 117 MHz, and 204 MHz.
  One way transit delay from most distant customer to headend.   ≤ 0.400 ms (typically much less)
  Carrier-to-interference plus ingress (the sum of noise,        Not less than 25 dB
  distortion, common-path distortion and cross modulation and
  the sum of discrete and broadband ingress signals, impulse
  noise excluded) ratio
  Carrier hum modulation                                         Not greater than -26 dBc (5.0%)
  Maximum amplitude variation across the 6 MHz channel           ≤ 2.78 dB pk-pk/6 MHz
  (digital channels)
  Group Delay Variation                                          ≤163 ns over 24 MHz
  Micro-reflections bound for dominant single echo               -16 dBc for echos ≤ 0.5 µs
                                                                 -22 dBc for echos ≤ 1.0 µs
                                                                 -29 dBc for echos ≤ 1.5 µs
                                                                 -35 dBc for echos > 2.0 µs
                                                                 -42 dBc for echos > 3.0 µs
                                                                 -51 dBc for echos > 4.5 µs
  Seasonal and diurnal reverse gain (loss) variation             Not greater than 14 dB min to max
  NOTE: Cascaded group delay could possibly exceed the ≤163 ns value within approximately 10 MHz of the upstream
  spectrum's lower and upper band edges, depending on cascade depth, diplex filter design, and actual band split.


5.2.2.2.1      Availability
Cable network availability is typically greater than 99.9%.

5.2.3     Transmission Levels
The nominal power level of the upstream CM signal(s) will be as low as possible to achieve the required margin
above noise and interference. Uniform power loading per unit bandwidth is commonly followed in setting upstream
signal levels, with specific levels established by the cable network operator to achieve the required carrier-to-noise
and carrier-to-interference ratios.




04/19/23                                                 CableLabs                                                     37

<!-- PDF_PAGE: 38 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


5.2.4    Frequency Inversion
There will be no frequency inversion in the transmission path in either the downstream or the upstream directions,
i.e., a positive change in frequency at the input to the cable network will result in a positive change in frequency at
the output.




38                                                   CableLabs                                                04/19/23
