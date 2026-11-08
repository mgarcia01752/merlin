# Appendix V - CMTS Proposed Configuration Parameters

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 159-159
Classification: Informative (explicit)

<!-- PDF_PAGE: 159 -->
Physical Layer Specification                                                             CM-SP-PHYv4.0-I07-250627


Appendix V                 CMTS Proposed Configuration Parameters (Informative)
                               Table 48 - CMTS Proposed Configuration Parameters

                     FFT             Roll-Off Period Samples                      Taper Region
                                               (Nrp)                                 (MHz)
                      4K                          64                                   3.575
                                                  128                                  1.875
                                                  192                                  1.325
                                                  256                                  0.975
                      8K                          64                                   3.3375
                                                  128                                  1.7125
                                                  192                                  1.1625
                                                  256                                 0.98751
              TABLE NOTE
              NOTE 1 The taper region of 0.9875 MHz is in accordance with the requirement for a minimum
                     taper region of 1 MHz minus half subcarrier spacing. Achieving up to approximately
                     0.5 dB impact to the noise power in the adjacent spurious emissions integration region
                     would allow a taper region of 0.8625 MHz, if the specification did not mandate the
                     minimum taper region to be larger than this.


The taper region consists of the spectrum extending from a) the spectral edge of the encompassed spectrum, to b) the
frequency above (or below) the edge of the encompassed spectrum by the amount (MHz) as given in the third
column of Table 48.
The spectral edge of the taper region which is opposite the spectral edge adjacent to the encompassed spectrum, falls
on the center frequency of an inactive subcarrier in the band edge exclusion sub-band. When the spectral edge of a
taper region of an OFDM signal falls on a CEA channel boundary, the CEA channel containing the taper region is
an occupied CEA channel (of that OFDM signal) and the other CEA channel, which abuts the taper region, is not an
occupied channel of that OFDM signal.




06/27/25                                                CableLabs                                             159
