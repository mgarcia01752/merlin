# 7.2 - Upstream and Downstream Frequency Plan

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 44-46
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 44 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications


7 PHY SUBLAYER FOR OFDM
7.1     Scope
This specification defines the electrical characteristics and signal processing operations for a cable modem (CM) and
Cable Modem Termination System (CMTS). It is the intent of this specification to define an interoperable CM and
CMTS such that any implementation of a CM can work with any CMTS. It is not the intent of this specification to
imply any specific implementation.
This section describes CM and CMTS physical layer requirements for DOCSIS 3.1 compliant devices.

7.2     Upstream and Downstream Frequency Plan
The following spectrum definitions are based on the system requirement that the downstream transmission
frequencies always reside above the upstream transmission frequencies in the cable plant.

7.2.1    Downstream CM Spectrum
The CM MUST support a minimum of two independently configurable OFDM channels each occupying a spectrum
of up to 192 MHz in the downstream.
The demodulator in the CM MUST support receiving downstream transmissions up to at least 1218 MHz.
The demodulator in the CM MAY support receiving downstream transmissions up to at least 1794 MHz.
The demodulator in the CM MUST support agile placement of the OFDM channels within the entire supported
downstream range.
Operators may need individual CM implementations to limit the spectrum over which the CM is able to receive
downstream signals in order to coexist with other network signals above 1 GHz.
As a result of this operational constraint, the CM MUST support one or more of the following downstream upper
band edges: 1002 MHz, 1218 MHz, 1794 MHz. Support in this context includes the capability to demodulate up to
the supported band edge. This operational consideration does not modify the requirement for the demodulator
capabilities; it only acknowledges that certain configurations may need to operate at a slightly lower upper band
edge due to MSO-specific operational constraints.
The CM MAY be configurable to operate with any supported downstream upper band edge. The nature and
operation of this configurability is vendor-specific.
The CM MUST support a downstream lower band edge of 258 MHz. Support in this context includes the capability
to demodulate down to this supported band edge.
The CM SHOULD support a downstream lower band edge of 108 MHz when the CM is configured to use an
upstream upper band edge of 85 MHz or less.

7.2.2    Downstream CMTS Spectrum
The CMTS MUST support a minimum of two independently configurable OFDM channels each occupying a
spectrum of up to 192 MHz in the downstream.
The CMTS MUST support a downstream upper band edge of 1218 MHz.
The CMTS MAY support a downstream upper band edge of 1794 MHz.
The CMTS MUST support a downstream lower band edge of 258 MHz.
The CMTS SHOULD support a downstream lower band edge of 108 MHz.




44                                                 CableLabs                                              04/19/23

<!-- PDF_PAGE: 45 -->
Physical Layer Specification                                                        CM-SP-PHYv3.1-I20-230419


7.2.3     Upstream CM Spectrum
The CM MUST support a minimum of two independently configurable OFDMA channels each occupying a
spectrum of up to 95 MHz in the upstream.
The CM MAY support more than two independently configurable OFDMA channels each occupying a spectrum of
up to 95 MHz in the upstream.
The CM modulator MUST support upstream transmissions from 5 to at least 204 MHz and agile placement of the
OFDMA channels within that range.
Individual CM implementations can limit the spectrum over which the CM is able to transmit upstream signals. As a
result, in order to be compliant with this specification a CM MUST support one or more of the following upstream
upper band edges, as long as one of the upstream upper band edges supported is 85 MHz or greater: 42 MHz; 65
MHz, 85 MHz, 117 MHz, and/or 204 MHz.
The CM MUST be configurable to operate with any supported upstream upper band edge. The nature and operation
of this configurability is vendor-specific. Possible forms of configurability include a hardware switch on the modem
housing, a software-controlled diplex filter responsive to OSSI commands, or other forms.
The CM MAY support additional spectrum beyond 204 MHz for the upstream.
The CM MUST NOT cause harmful interference to any downstream signals that might exist above its configured
upstream upper band edge.
The CM MUST be capable of transmitting 192 MHz of active channels when operating with the 204 MHz upstream
upper band edge.
In DOCSIS 3.1 upstream mode the CM MUST be capable of transmitting OFDMA channels and legacy SC-QAM
channels at the same time (as controlled by the CMTS). In all cases the CM is not required to transmit legacy SC-
QAM channels above a frequency of 85 MHz.

7.2.4     Upstream CMTS Spectrum
The CMTS MUST support a minimum of two independently configurable OFDMA channels each occupying a
spectrum of up to 95 MHz in the upstream.
The CMTS MAY support more than two independently configurable OFDMA channels each occupying a spectrum
of up to 95 MHz in the upstream.
The CMTS MUST support upstream transmissions from 5 to at least 204 MHz and agile placement of the OFDMA
blocks within that range.
The CMTS MAY support additional spectrum beyond 204 MHz for the upstream.
The CMTS MUST capable of receiving 192 MHz of active channels when operating with the 204 MHz upstream
upper band edge. In DOCSIS 3.1 upstream mode the CM is capable of transmitting OFDMA channels and legacy
SC-QAM channels at the same time (as controlled by the CMTS). In all cases, the CMTS MUST NOT configure the
CM to transmit legacy SC-QAM channels above a frequency of 85 MHz.

7.2.5     Channel Band Rules
During OFDM/OFDMA channel planning, the following rules are to be observed to ensure proper operation of
DOCSIS 3.1 CMTS and CM.
The CMTS MUST ensure that the configured OFDM/OFDMA channels are aligned with the rules specified in
Section 7.2.5.2, Section 7.2.5.3, and Section 7.2.5.4.

7.2.5.1    Downstream Channel Bandwidth Rules
The CMTS MUST ensure that the encompassed spectrum of a 192 MHz downstream OFDM channel does not
exceed 190 MHz. Therefore, the CMTS MUST ensure that the number of contiguous active subcarriers in a
downstream OFDM channel does not exceed 3800 for 4K FFT and 7600 for 8K FFT. When configured for 4K FFT,
the CMTS MUST only use subcarriers in the range 148 ≤ k ≤ 3947, where k is the spectral index of the subcarrier in



04/19/23                                           CableLabs                                                    45

<!-- PDF_PAGE: 46 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications


the IDFT equation defining the OFDM signal. When configured for 8K FFT, the CMTS MUST only use subcarriers
in the range 296 ≤ k ≤ 7895, where k is the spectral index of the subcarrier in the IDFT equation defining the OFDM
signal.
The CMTS MUST ensure that there is at least 1 MHz of exclusion band between the spectral edge of a legacy SC-
QAM channel and the center frequency of the nearest OFDM subcarrier. This SC-QAM channel may be external to
the OFDM channel or may be embedded within the OFDM channel.
The CMTS MUST also ensure that there is at least 2 MHz exclusion band between any two adjacent asynchronous
OFDM channels. In other words the CMTS MUST ensure that the frequency separation between the highest
frequency active subcarrier of one OFDM channel and the lowest frequency active subcarrier of the adjacent
asynchronous OFDM channel is not less than 2 MHz.
Such an exclusion band is not needed if the two OFDM channels are fully synchronous. The term synchronous here
implies that both OFDM channels have the same FFT length, the same cyclic prefix, and are synchronized in time,
frequency and phase. For example, the CMTS may use a single 16K FFT with a sample rate of 409.6 MHz to
construct two 8K FFT OFDM channels each with sample rate 204.8 MHz. The use of a single FFT guarantees that
all synchronization criteria are met. The CMTS may place 15200 contiguous active subcarriers, with an
encompassed spectrum of 380 MHz, anywhere within this 16K FFT. These 15200 subcarriers may be partitioned
equally between two adjacent downstream 8K FFT OFDM channels.

7.2.5.2       Downstream Exclusion Band Rules
The CM and CMTS are not expected to meet performance and fidelity requirements when the system configuration
does not comply with the downstream exclusion band rules listed below. These rules apply to each OFDM channel
and also to the composite downstream inclusive of OFDM and non-OFDM channels.
     •    There has to be at least one contiguous modulated OFDM bandwidth of 22 MHz or greater.
     •    Exclusion bands separate contiguous modulation bands.
     •    The minimum contiguous modulation band has to be 2 MHz.
     •    Exclusion bands and individually excluded subcarriers are common to all downstream profiles.
     •    Exclusion bands are a minimum of 1 MHz but increment above 1 MHz by granularity of individual
          subcarrier (25 kHz for 8k FFT and 50 kHz for 4K FFT).
     •    The ONLY exception to the above is for exclusion bands that are allowed to occupy the following
          frequency ranges in alignment with FCC regulations.
          •    121.400 MHz to 121.600 MHz
          •    156.750 MHz to 156.850 MHz
          •    242.950 MHz to 243.050 MHz
          •    405.925 MHz to 406.176 MHz
Unique spurious emissions requirements exist for these bands separate from the general exclusion bands requirements.
     •    Exclusion bands plus individually excluded subcarriers are limited to 20% or less of spanned modulation
          spectrum, where the spanned modulation spectrum is defined as: frequency of maximum active subcarrier −
          frequency of minimum active subcarrier.
     •    The number of individually excluded subcarriers is limited by the following:
          •    The total spectrum of individually excluded subcarriers cannot exceed 5% of any contiguous
               modulation spectrum.
          •    The total spectrum of individually excluded subcarriers cannot exceed 5% of a 6 MHz moving window
               across the contiguous modulation spectrum.
          •    The total spectrum of individually excluded subcarriers cannot exceed 20% of a 1 MHz moving
               window across the contiguous modulation spectrum.



46                                                  CableLabs                                              04/19/23
