# 7.3.3 - FDD OFDM Numerology

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 113-113
Classification: Main body (DOCSIS 4.0 FDD requirements)

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
