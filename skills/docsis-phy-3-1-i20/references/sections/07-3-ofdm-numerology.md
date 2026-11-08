# 7.3 - OFDM Numerology

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 47-48
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 47 -->
Physical Layer Specification                                                              CM-SP-PHYv3.1-I20-230419


      •   The 6 MHz of contiguous spectrum reserved for the PLC cannot have any exclusion bands or excluded
          subcarrier.

7.2.5.3    Upstream Channel Bandwidth Rules
The CMTS MUST ensure that the encompassed spectrum of an upstream OFDMA channel does not exceed 95
MHz. Therefore, the number of contiguous active subcarriers in an upstream OFDMA channel MUST NOT exceed
1900 for 2K FFT and 3800 for 4K FFT. When configured for 2K FFT, the CMTS MUST only use subcarriers in the
range 74 ≤ k ≤ 1973, where k is the spectral index of the subcarrier in the IDFT equation defining the OFDMA
signal. When configured for 4K FFT, the CMTS MUST only use subcarriers in the range 148 ≤ k ≤ 3947, where k is
the spectral index of the subcarrier in the IDFT equation defining the OFDMA signal.

7.2.5.4    Upstream Exclusions and Unused Subcarriers Rules
      •   Subcarriers which lie outside the Encompassed Spectrum are excluded.
      •   Excluded and unused subcarriers within the Encompassed Spectrum are not allowed within minislots.
          There is no restriction on the number or placement of excluded and unused subcarriers between minislots.

7.3       OFDM Numerology
7.3.1     Downstream OFDM Numerology
DOCSIS 3.1 uses OFDM for downstream modulation.
Two modes of operation are defined for the downstream: 4K FFT and 8K FFT modes for a sampling rate of
204.8 MHz. These are described in Section 7.5.7.
Table 6 summarizes the numerical values for the downstream OFDM parameters; a more detailed description of the
parameters is given in the sections which follow.
                                        Table 6 - Downstream OFDM Parameters

                             Parameter                                     4K Mode                       8K Mode
  Downstream master clock frequency                                                        10.24 MHz
  Downstream Sampling Rate (fs)                                                            204.8 MHz
  Downstream Elementary Period (Tsd)                                                     1/(204.8 MHz)
  Channel bandwidths                                                                  24 MHz … 192 MHz
  IDFT size                                                                   4096                          8192
  Subcarrier spacing                                                         50 kHz                        25 kHz
  FFT duration (Useful symbol duration) (Tu)                                  20 µs                         40 µs
  Maximum number of active subcarriers in signal (192 MHz channel)            3800                          7600
  Values refer to 190 MHz of used subcarriers.
  Maximum spacing between first and last active subcarrier                                  190 MHz
  Cyclic Prefix                                                      0.9375 µs       (192 * Tsd)
                                                                     1.25 µs         (256 * Tsd)
                                                                     2.5 µs          (512 * Tsd)
                                                                     3.75 µs         (768 * Tsd)
                                                                     5 µs            (1024 * Tsd)
  Windowing                                                          Tukey raised cosine window, embedded into cyclic prefix
                                                                     0 µs           (0 * Tsd)
                                                                     0.3125 µs      (64 * Tsd)
                                                                     0.625 µs       (128 * Tsd)
                                                                     0.9375 µs      (192 * Tsd)
                                                                     1.25 µs        (256 * Tsd)




04/19/23                                               CableLabs                                                          47

<!-- PDF_PAGE: 48 -->
CM-SP-PHYv3.1-I20-230419                                            Data-Over-Cable Service Interface Specifications


The downstream OFDM channel bandwidth can vary from 24 MHz to 192 MHz. Smaller bandwidths than 192 MHz
are achieved by zero-valuing the subcarriers prior to the IDFT, i.e., by adjusting the equivalent number of active
subcarriers while maintaining the same subcarrier spacing of 25 kHz or 50 kHz.

7.3.2    Upstream OFDMA Numerology
DOCSIS 3.1 uses OFDMA (orthogonal frequency-division multiple access) for upstream modulation. OFDMA is a
multi-user version of OFDM, and assigns subsets of subcarriers to individual CMs. The upstream OFDMA
parameters are derived from the downstream parameters, and are summarized in Table 7. A more detailed
description of the parameters is given in the sections which follow.
                                            Table 7 - Upstream OFDMA Parameters

                              Parameter                              2k Mode                      4k Mode
             Upstream Sampling Rate (fsu)                                          102.4 MHz
             Upstream Elementary Period Rate (Tsu)                                1/102.4 MHz
             Channel bandwidths                                 10 MHz, …, 95 MHz            6.4 MHz, …, 95 MHz
             IDFT size                                                 2048                          4096
             (depending on channel bandwidth)
             Subcarrier spacing                                       50 kHz                        25 kHz
             FFT duration (Useful symbol duration) (Tu)                20 µs                        40 µs
             Maximum number of active subcarriers in signal            1900                          3800
             Values refer to 95 MHz of active subcarriers
             Upstream Cyclic Prefix                           0.9375 µs     (96 * Tsu)
                                                              1.25 µs       (128 * Tsu)
                                                              1.5625 µs     (160 * Tsu)
                                                              1.875 µs      (192 * Tsu)
                                                              2.1875 µs     (224 * Tsu)
                                                              2.5 µs        (256 * Tsu)
                                                              2.8125 µs     (288 * Tsu)
                                                              3.125 µs      (320 * Tsu)
                                                              3.75 µs       (384 * Tsu)
                                                              5.0 µs        (512 *Tsu)
                                                              6.25 µs       (640 *Tsu)
             Upstream window size                             Tukey raised cosine window, embedded into cyclic prefix
                                                              0 µs          (0 * Tsu)
                                                              0.3125 µs     (32 * Tsu)
                                                              0.625 µs      (64 * Tsu)
                                                              0.9375 µs     (96 * Tsu)
                                                              1.25 µs       (128 * Tsu)
                                                              1.5625 µs     (160 * Tsu)
                                                              1.875 µs      (192 * Tsu)
                                                              2.1875 µs     (224 * Tsu)


7.3.3    Subcarrier Clocking
The "locking" of subcarrier "clock and carrier" in the downstream transmission are defined and characterized by the
following rules:
     •   Each OFDM symbol is defined with an FFT duration (equal to subcarrier clock period) of nominally 20 µs
         or 40 µs. For each OFDM symbol, the subcarrier clock period (µs) may vary from nominal with limits
         defined in Section 7.5.3.
         •     The number of cycles of each subcarrier generated by the CMTS during one period of the subcarrier
               clock (for each OFDM symbol) MUST be an integer number.
         •     The CMTS subcarrier clock MUST be synchronous with the 10.24 MHz Master Clock defined by:
               subcarrier clock frequency = (M/N)*Master Clock frequency where M = 20 or 40, and N = 8192.


48                                                        CableLabs                                                    04/19/23
