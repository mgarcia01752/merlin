# Appendix VI - Suggested Algorithm to Compute SNR Margin for Candidate Profile

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 219-219
Classification: Informative (explicit)

<!-- PDF_PAGE: 219 -->
Physical Layer Specification                                                     CM-SP-PHYv3.1-I20-230419


Appendix VI               Suggested Algorithm to Compute Signal-to-Noise Ratio
                          (SNR) Margin for Candidate Profile (Informative)
The CM measures the RxMER value for each data subcarrier as specified in Section 9.3.6. From these
measurements it calculates the average RxMER over all data subcarriers, MER1. The CM accepts as an input the
required RxMER delivering a defined threshold of CER = 1e-5 under ideal AWGN conditions for each bit loading.
The CM computes the difference of the measured RxMER values from the required RxMER values. The CM
computes the required average RxMER, denoted MER2, over all data subcarriers for the candidate profile. The
averaging computations for MER1 and MER2 use values in the log (dB) domain. The SNR margin is defined as
MER1 - MER2, where all quantities are in dB. As an example, if the CM measures MER1 = 33 dB, and the
candidate profile requires MER2 = 30 dB, the CM reports an SNR margin of 3 dB. In addition, the CM reports the
number of subcarriers whose RxMER is at least x dB below the defined threshold for the bit loading of the given
subcarrier, where x is a configurable parameter with default value = 1.




04/19/23                                         CableLabs                                                 219
