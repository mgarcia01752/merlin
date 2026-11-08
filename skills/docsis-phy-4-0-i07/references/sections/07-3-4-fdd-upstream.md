# 7.3.4 - FDD Upstream Transmit and Receive

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 113-133
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
<!-- PDF_PAGE: 114 -->
CM-SP-PHYv4.0-I07-250627                                  Data-Over-Cable Service Interface Specifications


7.3.4.2.3    Modulation Timing Jitter
See [DOCSIS PHYv3.1], section 7.4.2.3.

7.3.4.3     Forward Error Correction
See [DOCSIS PHYv3.1], section 7.4.3.

7.3.4.4     Data Randomization
See [DOCSIS PHYv3.1], section 7.4.4.

7.3.4.5     Time and Frequency Interleaving and De-interleaving
See [DOCSIS PHYv3.1], section 7.4.5.

7.3.4.6     Mapping of Bits to Cell Words
See [DOCSIS PHYv3.1], section 7.4.6.

7.3.4.7     Mapping and Demapping Bits to/from QAM Subcarriers
See [DOCSIS PHYv3.1], section 7.4.7.

7.3.4.8     REQ Messages
See [DOCSIS PHYv3.1], section 7.4.8.

7.3.4.9     IDFT
See [DOCSIS PHYv3.1], section 7.4.9.

7.3.4.10 Cyclic Prefix and Windowing
See [DOCSIS PHYv3.1], section 7.4.10.

7.3.4.11 Burst Timing Convention
See [DOCSIS PHYv3.1], section 7.4.11.

7.3.4.12 Fidelity Requirements
For devices operating in FDD mode, this section augments the requirements of section 7.4.12 of [DOCSIS
PHYv3.1] and corresponding subsections unless otherwise noted.
If the FDD CM supports and is configured to operate with an upstream upper band edge of 204 MHz, the FDD CM
MUST adhere to the Fidelity Requirements of DOCSIS3.1 in [DOCSIS PHYv3.1], section 7.4.12.
In all other cases, when the FDD CM is configured to an upper upstream band edge higher than 204 MHz, the FDD
CM operates with two transmission channel sets similar to what is defined in the FDX mode of operation: one
transmit channel set within 5 MHz to 85 MHz legacy band (TCS), and the other for OFDMA channels in the
frequency region above 108 MHz (TCS_FDD).
An FDD CM's transmit channel set in the FDD spectrum (TCS_FDD), 108 MHz to the configured upstream upper
band edge, is the set of OFDMA channels that can be transmitted by the CM in that band.
The FDD CM MUST comply with the Fidelity Requirements in this section.




114                                              CableLabs                                          06/27/25
<!-- PDF_PAGE: 115 -->
Physical Layer Specification                                                   CM-SP-PHYv4.0-I07-250627


7.3.4.12.1 Upstream Fidelity Measurement Framework
The upstream fidelity measurement framework for the FDD band illustrated in Figure 30 - FDD Cable Modem
Upstream Power and Fidelity Measurements at Interface F is referenced for upstream channel power requirements
that follow.




           Figure 30 - FDD Cable Modem Upstream Power and Fidelity Measurements at Interface Fˊ
With BWOFDMA-FDD being the combined occupied bandwidth of the OFDMA channel(s) in its TCS_FDD, the FDD
CM is said to have Neq-FDD = ceil (BWOFDMA-FDD (MHz)/1.6 MHz) "equivalent DOCSIS channels" in its TCS_FDD.
An FDD CM that is operating in UHS-684 mode MUST be capable of transmitting a maximum TCP of at least 65
dBmV at Interface Fˊ for the combined power of TCS_FDD and legacy TCS.
The FDD CM MAY be capable of transmitting a maximum TCP greater than 65 dBmV at Interface Fˊ.
An FDD CM that is operating in UHS-300 mode MUST be capable of transmitting a maximum total average output
power of at least 58.6 dBmV in the FDD band.
An FDD CM that is operating in UHS-396 mode MUST be capable of transmitting a maximum total average output
power of at least 61.3 dBmV in the FDD band.
An FDD CM that is operating in UHS-492 mode MUST be capable of transmitting a maximum total average output
power of at least 63.6 dBmV in the FDD band.
An FDD CM that is operating in UHS-684 mode MUST be capable of transmitting a maximum total average output
power of at least 64.5 dBmV in the FDD band.
An FDD CM that is operating in UHS-300 mode MAY be capable of transmitting a maximum total average output
power greater than 58.6 dBmV in the FDD band.
An FDD CM that is operating in UHS-396 mode MAY be capable of transmitting a maximum total average output
power greater than 61.3 dBmV in the FDD band.
An FDD CM that is operating in UHS-492 mode MAY be capable of transmitting a maximum total average output
power greater than 63.6 dBmV in the FDD band.


06/27/25                                         CableLabs                                               115
<!-- PDF_PAGE: 116 -->
CM-SP-PHYv4.0-I07-250627                                     Data-Over-Cable Service Interface Specifications


An FDD CM that is operating in UHS-684 mode MAY be capable of transmitting a maximum total average output
power greater than 64.5 dBmV in the FDD band.
The FDD CM MUST be capable of transmitting a total average output power of 55 dBmV in the legacy US (5 MHz
to 85 MHz) band when operating in FDD mode.
Interface F has a requirement on the maximum TCP (total composite power) which is 64.5 dBmV, realized when the
FDD modulated spectrum is at the maximum which is 570 MHz, and reduces from 64.5 dBmV as modulated
spectrum reduces.
At Interface F, the upstream reference PSD is defined, which is a line in dBmV for the y-axis and in linear frequency
for the x-axis that passes through the points 33.0 dBmV/1.6 MHz centered at 108.8 MHz and 43.0 dBmV/1.6 MHz
centered at 683.2 MHz for a CM that is operating in UHS-684 split. For a CM operating in UHS-300, UHS-396, or
UHS-492, the upstream reference PSD is 3 dB higher, and it is a line in dBmV for the y-axis and in linear frequency
for the x-axis that passes through the points 36.0 dBmV/1.6 MHz centered at 108.8 MHz and 46.0 dBmV/1.6 MHz
centered at 683.2 MHz.
At Interface F, the Dynamic Range Window for FDD (DRW_FDD) is defined, which is the result of adjustment
relative to the upstream reference PSD, lowered by the amount commanded by the FDD CMTS. The DRW_FDD is
managed in the FDD spectrum in a similar manner as the DRW is managed in the legacy DOCSIS 3.1 upstream
spectrum, but DRW_FDD and DRW are different. DOCSIS 3.1 CMs are still managed with the DOCSIS 3.1 DRW.
DRW_FDD does not apply to DOCSIS 3.1 CMs.
Channel power adjustments within the FDD spectrum are managed similarly to the channel power adjustments in
the legacy DOCSIS 3.1 upstream spectrum, which adjusts each individual channel PSD at or below the top of the
DRW_FDD. Additionally, the individual channel PSD for a CM operating in any of the FDD modes can be adjusted
above the top of the DRW_FDD by up to 1.5 dB within the FDD band (i.e., Plimit-FDD is specified in Section
7.3.4.12.2) when the top of the DRW-FDD is at Pref-FDD.
Figure 30 illustrates the measurement for channel reported power accuracy and MER and spurious emissions for
signals in the FDD spectrum. The test setup defining the measurements provides that a signal with the upstream
reference PSD at Interface F has a flat PSD at Interface Fˊ.

7.3.4.12.2 FDD Upstream Reference PSD
The channel reported power for an FDD CM for channels in the FDD spectrum are reported relative to the upstream
reference PSD. The channel commanded power (per 1.6 MHz) for a channel at Interface F is, by definition, the
channel power adjustments up or down from the upstream reference PSD.
The "equivalent channel power" of an FDD OFDMA channel is the average power at Interface Fˊ of the OFDMA
subcarriers of the channel normalized to 1.6 MHz bandwidth and referenced to the upstream reference PSD at
Interface Fˊ. This equivalent channel power of an FDD OFDMA channel is denoted as P1.6r_n-FDD. The TCS_FDD
has zero to six OFDMA channels, but also is described as having Neq-FDD number of equivalent DOCSIS channels.
Each channel in the TCS_FDD is described by its reported power P1.6r_n-FDD, which is the channel power when it is
fully granted and normalized to 1.6 MHz. The relation of the reported power to the expected true power of a fully
granted channel is a function of the number of active subcarriers in the channel and their frequency. The reported
power for each channel is referenced to Interface Fˊ to simplify the upstream power management at the FDD CMTS,
which is generally expected to operate with a flat received PSD.
For an FDD CM, Pref-FDD is a parameter which is a function of frequency and is the power in dBmV in 1.6 MHz of
subcarriers with no pre-equalization. Pref-FDD has a slope of (10 dB/360 slots of 1.6 MHz in the FDD band)
dBmV/1.6 MHz, and a total variation of 10.0 dB across 108 MHz to 684 MHz. For a CM that is operating in UHS-
684, Pref-FDD (108.8 MHz) = 33.0 dBmV/1.6 MHz and Pref-FDD (683.2 MHz) = 43.0 dBmV/1.6 MHz. This
corresponds to 64.5 dBmV total average power in the FDD band with 570 MHz modulated spectrum in the
TCS_FDD (Neq-FDD = 357). For a CM that is operating in UHS-300, UHS-396, or UHS-492, Pref-FDD (108.8 MHz) =
36.0 dBmV/1.6 MHz and Pref-FDD (683.2 MHz) = 46.0 dBmV/1.6 MHz. For UHS-300, this corresponds to 58.6
dBmV total average power in the FDD band with 190 MHz modulated spectrum in the TCS_FDD (Neq-FDD = 119).
For UHS-396, this corresponds to 61.3 dBmV total average power in the FDD band with 285 MHz modulated
spectrum in the TCS_FDD (Neq-FDD = 179). For UHS 492, this corresponds to 63.6 dBmV total average power in the



116                                                CableLabs                                             06/27/25
<!-- PDF_PAGE: 117 -->
Physical Layer Specification                                                             CM-SP-PHYv4.0-I07-250627


FDD band with 380 MHz modulated spectrum in the TCS_FDD (Neq-FDD = 238). Note that there are no upstream
SC-QAM channels in the FDD band.
Table 33 summarizes the DOCSIS 4.0 CM upstream reference PSD, maximum total average power in the FDD
band, spectral tilt, and modulated spectrum at each of the FDD modes as shown in Figure 30.
                  Table 33 - Upstream Reference PSD, Total Average Power in the FDD Band,
                               and Spectral Tilt Values for Different FDD Modes

                                                                                        Maximum
                                                                                      Total Average
                          y                      y+z             Spectral Tilt for     Power in the         Modulated
  FDD Mode           (108.8 MHz)             (683.2 MHz)          Fidelity Req.         FDD Band          FDD Spectrum
 UHS-300            36 dBmV/1.6 MHz        46 dBmV/1.6 MHz             10 dB             58.6 dBmV             190 MHz
 UHS-396            36 dBmV/1.6 MHz        46 dBmV/1.6 MHz             10 dB             61.3 dBmV             285 MHz
 UHS-492            36 dBmV/1.6 MHz        46 dBmV/1.6 MHz             10 dB             63.6 dBmV             380 MHz
 UHS-684            33 dBmV/1.6 MHz        43 dBmV/1.6 MHz             10 dB             64.5 dBmV             570 MHz
 UHS-684           34.3 dBmV/1.6 MHz      42.3 dBmV/1.6 MHz             8 dB             64.5 dBmV             570 MHz
 UHS-684           31.6 dBmV/1.6 MHz      43.6 dBmV/1.6 MHz            12 dB   1
                                                                                         64.5 dBmV             570 MHz
TABLE NOTE
NOTE 1 Fidelity requirements are met with 1 dB additional allowance for MER, inband and out-of-band spurious emissions
       compared to the 10 dB uptilt case.


For all four FDD split options (UHS-300, UHS-396, UHS-492, and UHS-684), fidelity requirements apply with Pref-FDD
for the transmit power spectral density, with 10 dB uptilt and no other channel adjustments or pre-equalization. For
the UHS-684 split option only, fidelity requirements also apply with 8 dB uptilt (and no other channel adjustments or
pre-equalization beyond accomplishing the slope). For the UHS-684 split option only, fidelity requirements with 1
dB additional allowance for spurious emissions, MER and Inband (compared to the 10 dB uptilt case) apply with 12
dB uptilt (and no other channel adjustments or pre-equalization beyond accomplishing the slope).
Plimit-FDD is the maximum power per channel, increased from Pref-FDD, for which only gradual degradation of fidelity
requirements may be expected. Plimit-FDD is 1.5 dB for upstream channels with no modulated spectrum above 300
MHz and is 1 dB for channels with any modulated spectrum above 300 MHz. Note that although some channels
may be commanded above Pref-FDD, and fidelity requirements may still apply, if the total average power in the FDD
band exceeds PmaxFDD, then fidelity requirements do not apply.
The maximum power per any individual subcarrier, increased from Pref-FDD, for which gradual degradation of fidelity
requirements may be expected are given as follows: with any subcarrier power commanded more than 3 dB higher
than Pref-FDD, in any channel which has no modulated spectrum above 300 MHz; with any subcarrier power
commanded more than 2.5 dB higher than Pref-FDD, in any channel which has no modulated spectrum above 492
MHz but some modulated spectrum above 300 MHz; or with subcarrier power commanded more than 2.0 dB higher
than Pref-FDD, in any channel which has some modulated spectrum above 492 MHz. Note that although some
subcarriers may be commanded above Pref-FDD, and fidelity requirements may still apply, if the total average power
in the FDD band exceeds PmaxFDD then fidelity requirements do not apply.

7.3.4.12.3 Maximum Scheduled Minislots
Maximum Scheduled Minislots is not supported for FDD CMs operating in FDD mode.

7.3.4.12.4 FDD Transmit Power Requirements
The transmit power is a function of the number and occupied bandwidth of the OFDMA channels in the TCS_FDD, or
equivalently the amount of TCS_FDD modulated spectrum and the frequency of the modulated spectrum. The
highest value of the total power output in the FDD spectrum of the FDD CM, PmaxFDD, is 64.5 dBmV and occurs
when all the FDD CM's potential FDD spectrum (108 MHz to 684 MHz) is occupied with OFDMA channels in its
TCS_FDD and these are fully granted to the FDD CM, and all channels are commanded to 0 dB channel power. The
FDD upstream reference PSD is denoted as Pref-FDD and is a function of frequency as defined in Section 7.3.4.12.2.
When the TCS_FDD occupies a subset of the potential FDD spectrum, the total average power in the FDD band is


06/27/25                                              CableLabs                                                         117
<!-- PDF_PAGE: 118 -->
CM-SP-PHYv4.0-I07-250627                                        Data-Over-Cable Service Interface Specifications


reduced from 64.5 dBmV. The top of the Dynamic Range Window of the OFDMA FDD spectrum can be reduced
from Pref-FDD by adjusting Pload_min_set-FDD to a value greater than 0 dB. The OFDMA FDD spectrum's nth channel
power can be raised by up to 1.5 dB within the FDD band (Plimit-FDD) when the top of the DRW_FDD is at Pref-FDD, or
reduced for any setting of the DRW_FDD. For example, the nth channel power can be reduced by setting its P1.6r_n-
FDD below 0 dB (negative), thereby reducing all the channel subcarriers from their power that was based on Pref-FDD
and P1.6r_n-FDD = 0 dB. There are limits on the amount of reduction as described below. This adjustability in channel
power ensures that each channel can be set to a power range (within the DRW_FDD) between its maximum power,
Pref-FDD – Pload_min_set-FDD, or up to 1.5 dB higher within the FDD band (Plimit-FDD) when Pload_min_set-FDD = 0, and
minimum power, P1.6low-FDD, and that any possible transmit grant combination can be accommodated without
exceeding the transmit power capability of the CM.
For FDD, P1.6low-FDD = P1.6min-FDD = -15 dB for UHS-684 or P1.6low-FDD = P1.6min-FDD = -18 dB for UHS-300, UHS-396,
and UHS-492. Boosted pilots are not supported in the FDD channels. Before completion of Fine Ranging, the FDD
CM has no need to transmit with power per subcarrier which is lower than indicated by P1.6low-FDD. These
transmissions are prior to any data grant transmissions in the FDD band from the CM and as such the CM analog
and digital gain balancing may be optimized for these transmissions.
When Pload_min_set-FDD is 0 dB, the FDD CMTS SHOULD NOT command the CM to set P1.6r_n-FDD on any channel in
the TCS_FDD between 108 MHz and 300 MHz to a value more than 1.5 dB above the top of the DRW_FDD, or
any channel in the TCS_FDD above 300 MHz, to a value more than 1 dB above the top of the DRW_FDD, or lower
than the bottom of the DRW_FDD. When Pload_min_set-FDD is greater than 0 dB, the FDD CMTS SHOULD NOT
command the CM to set P1.6r_n-FDD on any channel in the TCS_FDD to a value more than 0 dB above the top of the
DRW_FDD, or lower than the bottom of the DRW_FDD.
If the FDD CMTS commands the FDD CM to exceed the top of the DRW_FDD, fidelity and performance
requirements on the FDD CM do not apply, except for the following two narrow cases, which are applicable in
UHS-684 operation mode only; with the 8 dB uptilt case spurious emissions and MER requirements are the same as
with the 10 dB uptilt specified case; and with the 12 dB uptilt receiving 1 dB relaxation for spurious emissions and
MER:
      •   8 dB uptilt, 64.5 dBmV total average power in the FDD band: 1.3 dB higher (34.3 dBmV/1.6 MHz) at
          108.8 MHz and 0.7 dB lower (42.3 dBmV/1.6 MHz) at 683.2 MHz.
      •   12 dB uptilt, 64.5 dBmV total average power in the FDD band: 1.4 dB lower (31.6 dBmV/1.6 MHz) at
          108.8 MHz and 0.6 dB higher (43.6 dBmV/1.6 MHz) at 683.2 MHz.
Table 33 (above) summarizes the FDD upstream reference PSD for different amount of spectral tilt as shown in
Figure 30 (above).
If Pload_min_set-FDD is more than 0 dB, and the FDD CM is commanded to transmit on any channel in the TCS_FDD at
a value higher than the top of the DRW_FDD or lower than the bottom of the DRW_FDD, the cable modem
indicates an error condition by setting the appropriate bit in the SID field of RNG-REQ messages for that channel
until the error condition is cleared as described in [DOCSIS MULPIv4.0].
If Pload_min_set-FDD is 0 dB, and the FDD CM is commanded to transmit on any channel in the TCS_FDD at a value
more than 1.5 dB higher than the top of the DRW_FDD for channels in 108 MHz to 300 MHz or more than 1 dB
higher than the top of the DRW_FDD for channels higher than 300 MHz, or lower than the bottom of the
DRW_FDD, the cable modem indicates an error condition by setting the appropriate bit in the SID field of RNG-
REQ messages for that channel until the error condition is cleared as described in [DOCSIS MULPIv4.0].
The FDD CMTS sends transmit power level commands and pre-equalizer coefficients to the FDD CM as described
in [DOCSIS MULPIv4.0] to compensate for upstream plant conditions. The top edge of the DRW_FDD is set to a
level, P­1.6load_min_set-FDD, close to the highest P1.6r_n-FDD transmit channel to optimally load the DAC. In conditions of
tilt significantly different than the nominal 10 dB tilt, some of the channels will be sent commands to transmit at
lower P1.6r_n-FDD values that use up a significant portion of the DRW_FDD, and perhaps exceed the top of the
DRW_FDD. Additionally, the pre-equalizer coefficients of the OFDMA channels will also compensate for plant tilts
away from the nominal 10 dB. The FDD CMTS normally administers a DRW_FDD of 10 dB [DOCSIS
MULPIv4.0] which is sufficient to accommodate plant tilts of up to +2 dB to – 2 dB different from the specified tilt
cases, 8 dB and 12 dB, and to accommodate plant flatness variations and loss variations as a function of frequency.
Since the fidelity requirements are specified in flat frequency conditions at Interface Fˊ relative to the top of the



118                                                   CableLabs                                               06/27/25
<!-- PDF_PAGE: 119 -->
Physical Layer Specification                                                              CM-SP-PHYv4.0-I07-250627


DRW_FDD, it is desirable to maintain FDD CM transmission power levels as close to the top of the DRW_FDD as
possible. When conditions change sufficiently to warrant it, a global reconfiguration time should be granted and the
top of the DRW_FDD adjusted to maintain the best transmission fidelity and optimize system performance.

7.3.4.12.4.1 FDD Transmit Power Detailed Requirements
For FDD CMs operating in FDD mode, Multiple Transmit Channel Mode is always enabled.
The FDD CM MUST support varying the amount of transmit power.
Requirements are presented for 1) range of reported transmit power per channel; 2) step size of power commands; 3)
step size accuracy (actual change in output power per channel compared to commanded change); and 4) absolute
accuracy of FDD CM output power per channel. The protocol by which power adjustments are performed is defined
in [DOCSIS MULPIv4.0].
The FDD CM MUST support such adjustments, commanded by the CMTS, within the ranges of tolerances
described below.
An FDD CM MUST confirm that the transmit power per channel limits are met after a RNG-RSP is received for
each of the FDD CM's active channels that is referenced and indicate that an error has occurred in the next RNG-
REQ messages for the channel until the error condition is cleared [DOCSIS MULPIv4.0].
Sometime after registration the FDD CMTS can initialize FDD operations for an FDD CM. During this process, the
CM is assigned to a TG and receives FDD channel assignment via the Dynamic Bonding Request (DBC)
mechanism. The group of active channels in the FDD band assigned to an FDD CM is known as the CM's FDD
transmit channel set (TCS_FDD). If the FDD CMTS needs to add, remove, or replace channels in the FDD CM's
TCS_FDD, it uses the DBC-REQ Message with Transmit Channel Configuration encodings to define the new
desired TCS_FDD. The set of channels actually bursting upstream from an FDD CM at any time could be all or a
subset of the active channels on that FDD CM. Often one or all active channels on an FDD CM will not be bursting,
but such quiet channels are still active channels for that FDD CM.
Transmit power per channel is defined as the average RF power in the occupied bandwidth (channel width),
assuming equally likely QAM symbols, relative to the upstream reference PSD, measured at Interface Fˊ of Figure
30 as detailed below. Reported transmit power for an OFDMA channel is expressed as P1.6r_n-FDD and is defined as
the average RF power of the FDD CM transmission in the OFDMA channel, relative to the upstream reference PSD
at Interface Fˊ when transmitting in a grant composed of 64 25 kHz subcarriers or 32 50 kHz subcarriers. Total
transmit power is defined as the sum of the transmit power per channel of each channel transmitting a burst at a
given time.
The FDD CM MUST maintain its actual transmitted power per equivalent channel to within ± 2 dB of the reported
power, P1.6r_n-FDD, with pre-equalization off, taking into account symbol constellation values.
The FDD CM MUST allow its target transmit power per channel to vary over the range specified in Section
7.2.4.12.4.
The fidelity requirements do not apply when the FDD CM is commanded to transmit at power levels which exceed
the top of the DRW_FDD in all four FDD splits (UHS-300, UHS-396, UHS-492, and UHS-684), except for the two
narrow cases 8 dB uptilt and 12 dB uptilt in the UHS-684 split option.
The transmit channel loading P1.6load-FDD describes how close the transmit power level for a particular channel is to
the top of the DRW_FDD. Let P1.6load-FDD = Pref-FDD - P1.6r_n-FDD, for each channel, using the definitions for Pref-FDD
and P1.6r_n-FDD in the following subsections of Section 7.3.4.12.5. The channel corresponding to the minimum value
of P1.6load-FDD is called the highest loaded channel, and its value is denoted as P1.6load_1-FDD, in this specification even if
there is only one channel in the FDD transmit channel set (TCS_FDD). A channel with high loading has a low
P1.6load_i-FDD value; the value of P1.6load_n-FDD is analogous to an amount of back-off for an amplifier from its max
power output, except that it is normalized to 1.6 MHz of bandwidth. A channel has lower power output when that
channel has a lower loading (more back-off) and thus a higher value of P1.6load_i-FDD. Note that the highest loaded
channel is not necessarily the channel with the highest transmit power at Interface Fˊ in Figure 30 since a channel's
max power at Interface Fˊ depends on the bandwidth of the channel. The channel with the second lowest value of
P1.6load-FDD is denoted as the second highest loaded channel, and its loading value is denoted as P1.6load_2-FDD; the




06/27/25                                               CableLabs                                                        119
<!-- PDF_PAGE: 120 -->
CM-SP-PHYv4.0-I07-250627                                                Data-Over-Cable Service Interface Specifications


channel with the ith lowest value of P1.6load-FDD is the ith highest loaded channel, and its loading value is denoted as
P1.6load_i-FDD.
P1.6load_min_set-FDD defines the upper end of the DRW_FDD for the FDD CM with respect to Pref-FDD. P1.6load_min_set-FDD
will normally limit the maximum power possible for each active channel to a value less than Pref-FDD, but a
commanded power adjustment can result in a violation of the DRW_FDD, in which case the FDD CM compliance
with the fidelity requirements is not enforced, with two narrow exceptions for 8 dB uptilt and 12 dB uptilt described
in the previous section. P1.6load_min_set_FDD is a value commanded to the FDD CM from the FDD CMTS when the FDD
CM is given a TCC in Registration and RNG-RSP messages after Registration as described in [DOCSIS
MULPIv4.0]. P1.6load_min_set-FDD, P1.6load_ n-FDD, Pref-FDD, Plimit-FDD, P1.6 r_n-FDD, etc., are defined for DOCSIS FDD
modems operating on an FDD CMTS. See Section 7.2.4.12.4 for a summary of these and other terms related to
transmit power.
The FDD CMTS SHOULD command the FDD CM to use a value for P1.6load_min_set-FDD such that Pref-FDD –
P1.6load_min_set-FDD ≥ P1.6low _n-FDD for each active channel, with allowance for higher channel power up to Plimit-FDD in
some channels, as long as PmaxFDD is maintained (to support different uptilt than 10 dB), or equivalently:
                                          0 ≤ P1.6load_min_set-FDD ≤ Pref-FDD – P1.6low_n-FDD
A value is computed, P1.6 low_multi-FDD, which sets the lower end of the transmit power DRW_FDD for that channel,
given the upper end of the range which is determined by P1.6load_min_set-FDD.
          P1.6low_multi-FDD = max{Pref-FDD – P1.6load_min_set-FDD – 10 dB, Pref-FDD – 15 dB} for UHS-684
          P1.6low_multi-FDD = max{Pref-FDD – P1.6load_min_set-FDD – 10 dB, Pref-FDD – 18 dB} for UHS-300, UHS-396, or
          UHS-492
The effect of P1.6low_multi-FDD is to restrict the dynamic range required (or even allowed) by an FDD CM across its
multiple channels, when operating with multiple active channels.
The FDD CMTS SHOULD command a P1.6r _n-FDD consistent with the P1.6load_min_set-FDD assigned to the FDD CM and
with the following limits (with allowance up to Plimit-FDD rather than Pref-FDD to accommodate different uptilt than 10
dB):
P1.6load_min_set-FDD ≤ Pref-FDD – P1.6r_n-FDD ≤ P1.6load_min_set-FDD + 10 dB
and the equivalent:
                        Pref-FDD – (P1.6load_min_set-FDD + 10 dB) ≤ P1.6r _n-FDD ≤ Pref-FDD – P1.6load_min_set-FDD
When the FDD CMTS sends a new value of P1.6load_min_set-FDD to the FDD CM, there is a possibility that the FDD CM
will not be able to implement the change to the new value immediately, because the FDD CM may be in the middle
of bursting on one or more of its upstream channels at the instant the command to change P1.6load_min_set-FDD is
received at the FDD CM. Some amount of time may elapse before the FDD CMTS grants global reconfiguration
time to the FDD CM. Similarly, commanded changes to P1.6r_n-FDD may not be implemented immediately upon
reception at the FDD CM if the nth channel is bursting. Commanded changes to P1.6r_n-FDD may occur simultaneously
with the command to change P1.6load_min_set-FDD.
The FDD CMTS SHOULD NOT issue a change in P1.6load_min_set-FDD after commanding a change in P1.6r_n-FDD until
after also providing a sufficient reconfiguration time on the nth channel.
The FDD CMTS SHOULD NOT issue a change in P1.6load_min_set-FDD after commanding a prior change in
P1.6load_min_set-FDD until after also providing a global reconfiguration time for the first command.
The FDD CMTS SHOULD NOT issue a change in P1.6r_n-FDD until after providing a global reconfiguration time
following a command for a new value of P1.6load_min_set-FDD and until after providing a sufficient reconfiguration time
on the nth channel after issuing a previous change in P1.6r_n-FDD.
In other words, the FDD CMTS is to avoid sending consecutive changes in P1.6r_n-FDD and/or P1.6load_min_set-
FDD to the FDD CM without a sufficient reconfiguration time for instituting the first command.
When a concurrent new value of P1.6load_min_set-FDD and change in P1.6r_n-FDD are commanded, the FDD CM MAY wait
to apply the change in P1.6r_n-FDD at the next global reconfiguration time (i.e., concurrent with the institution of the
new value of P1.6load_min_set-FDD) rather than applying the change at the first sufficient reconfiguration time of the nth



120                                                         CableLabs                                               06/27/25
<!-- PDF_PAGE: 121 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


channel. The value of P1.6load_min_set-FDD which applies to the new P1.6r_n-FDD is the concurrently commanded
P1.6load_min_set-FDD value.
If the change to P1.6r_n-FDD falls outside the DRW_FDD of the old P1.6load_min_set-FDD, then the FDD CM MUST wait
for the global reconfiguration time to apply the change in P1.6r_n-FDD.
The FDD CMTS SHOULD NOT command the FDD CM to decrease the per-channel transmit power if such a
command would cause P1.6load_ n-FDD for that channel to drop below P1.6load_min_set-FDD.
Note that the FDD CMTS can allow small changes of power in the FDD CM's highest loaded channel without these
fluctuations impacting the transmit power dynamic range with each such small change. This is accomplished by
setting P1.6load_min_set-FDD to a smaller value than normal, and fluctuation of the power per channel in the highest
loaded channel is expected to wander.
The FDD CMTS MUST NOT command a change of per channel transmit power or pre-equalization which could
result in exceeding the CM's PmaxFDD in the FDD band.
If the CMTS improperly commands the FDD CM to exceed PmaxFDD, the FDD CM informs the FDD CMTS of the
error as described in [DOCSIS MULPIv4.0].
The FDD CMTS SHOULD NOT command a change of per channel transmit power which would result in P1.6r_n-FDD
falling below the DRW_FDD, P1.6r_n-FDD < P1.6 low_multi-FDD.
The FDD CMTS SHOULD NOT command a change in P1.6load_min_set-FDD such that existing values of P1.6r_n-FDD
would fall outside the new DRW_FDD.
The following paragraphs define the FDD CM and FDD CMTS behavior in cases where there are DRW_FDD
violations due to addition of a new channel with incompatible parameters without direct change of P1.6r_n-FDD or
P1.6load_min_set-FDD.
When adding a new active channel to the transmit channel set, the new channel's power is calculated according to
the offset value defined in TLV 46.8.4 [DOCSIS MULPIv4.0], if it is provided.
The FDD CMTS SHOULD NOT set an offset value that will result in a P1.6r_n-FDD for the new channel outside the
DRW_FDD.
In the absence of TLV 46.8.4, the new channel's power is initially set by the FDD CM at the minimum allowable
power, i.e., the bottom of the DRW_FDD.
The FDD CM MUST maintain its actual transmitted power per every minislot within a burst constant to within 0.1
dB peak to valley even in the presence of power changes on other active channels.
The 0.1 dB peak to valley does not include amplitude variation theoretically present in the signal (e.g., varying
QAM constellations, transmit window). Specifically, within a continuous burst of duration up to n frames (1
millisecond), for each minislot participating in the burst and while the minislot is actively used for transmission, a
constant power has to be maintained in that minislot within 0.1 dB peak to valley, even in the presence of a
transmission starting or stopping on other minislots and other active channels.
The FDD CM MUST support the transmit power calculations defined in Section 7.3.4.12.5, FDD Transmit Power
Calculations.

7.3.4.12.5 FDD Transmit Power Calculations
The FDD CM determines its target transmit power per channel P1.6t_n-FDD, as follows, for each channel which is
active. Define for each active channel, for example, upstream channel n:P1.6c_n-FDD = Commanded Power for channel
n. (TLV-17 in RNG-RSP).
P1.6r_n-FDD = reported power level (dBmV) of the FDD CM for channel n
Plimit_FDD = 1.5 dB for channels with no modulated spectrum above 300 MHz and 1.0 dB for channels with any
modulated spectrum above 300 MHz
The FDD CM updates its reported power per channel in each channel by the following steps:
    1.   ΔP = P1.6c_n-FDD - P1.6r_n-FDD



06/27/25                                             CableLabs                                                    121
<!-- PDF_PAGE: 122 -->
CM-SP-PHYv4.0-I07-250627                                                 Data-Over-Cable Service Interface Specifications


      2.    P1.6r_n-FDD = P1.6r_n-FDD + ΔP //Add power level adjustment (for each channel) to reported power level for
            each channel.
The FDD CMTS SHOULD ensure the following:
      1.    P1.6r_n-FDD ≤ ;Plimit-FDD, when P1.6load_min_set-FDD = 0 //Clip at max power limit per channel unless the FDD
            CMTS accommodates a need to increase the PSD for the channel in which case the fidelity performance of
            the FDD CM is potentially degraded.
      2.    P1.6r_n-FDD ≥ P1.6low_n-FDD //Clip at min power limit per channel.
      3.    P1.6r_n-FDD ≥ P1.6low_multi-FDD //Power per channel from this command would violate the set DRW-FDD.
      4.    P1.6r _n-FDD ≤ - P1.6load_min_set-FDD, when P1.6load_min_set-FDD > 0 //Power per channel from this command violates
            the set DRW_FDD, but the CMTS could accommodate a need to increase the PSD for the channel in which
            case the fidelity performance of the CM is potentially degraded.
For OFDMA, the CM then transmits each data subcarrier with target power:
                      Pt_sc_i = P1.6r_n-FDD + Pre-Eqi - 10log10(number_of subcarriers in 1.6 MHz {32 or 64})
where Pre-Eqi is the magnitude of the ith subcarrier pre-equalizer coefficient (dB).
That is, the reported power for channel n, normalized to 1.6 MHz, plus the pre-equalization for the subcarrier, less a
factor taking into account the number of subcarriers in 1.6 MHz.
Probedelta_n-FDD for the nth FDD OFDMA channel is the change in subcarrier power for probes compared to subcarrier
power for data depending on the mode as defined in [DOCSIS MULPIv4.0] in addition to pre-equalization on or off.
The CM transmits probes with the same target power as given above plus Probedelta_n-FDD when pre-equalization is
enabled for probes in the P-MAP which provides the probe opportunity:
           Pt_sc_i = P1.6r_n-FDD + Probedelta_n-FDD + Pre-Eqi - 10log10(number_of subcarriers in 1.6 MHz {32 or 64})
When the pre-equalization is disabled for the probe opportunity in the P-MAP, the FDD CM then transmits probe
subcarrier with target power:
                   Pt_sc_i = P1.6r_n-FDD + Probedelta_n-FDD - 10log10 (number_of subcarriers in 1.6 MHz {32 or 64})
That is, the reported power for channel n, normalized to 1.6 MHz, less a factor taking into account the number of
subcarriers in 1.6 MHz.
The total transmit power in channel n, Pt_n, in a frame is the sum of the individual transmit powers Pt_sc_i of each
subcarrier in channel n, where the sum is performed using absolute power quantities (non-dB domain).
The transmitted power level in channel n varies dynamically as the number and type of allocated subcarriers varies.

7.3.4.12.5.1 Terminology Used in Sections Covering FDD Upstream Transmit Power Requirements
This section provides a brief description of the terms used in elaboration of the transmit power requirements.
  BWOFDMA-FDD            The combined occupied bandwidth of the FDD OFDMA channel(s) in the FDD band transmit channel set
                         (TCS_FDD).
  DRW_FDD                The dynamic range window of a FDD channel.
  Neq-FDD                Number of Equivalent DOCSIS 1.6 MHz upstream channels in the cable modem's FDD band transmit
                         channel set (TCS_FDD). Neq= ceil(BWOFDMA (MHz)/1.6 MHz)
  P1.6c_n-FDD            Commanded Power for FDD channel n. (TLV-17 in RNG-RSP)
  P1.6load_i-FDD         The transmit channel loading P1.6load_i-FDD. This describes how close the transmit power level for a particular
                         channel is to the top of the DRW_FDD. The highest loaded FDD channel P1.6load_1-FDD is the channel for which
                         the reported power P1.6r_n-FDD is closest to the top of the DRW_FDD. In the case where there are j channels in
                         the TCS_FDD, the lowest loaded channel P1.6load_j-FDD is the FDD channel whose reported power P1.6r_n-FDD is
                         furthest from the top of the DRW_FDD.
  P1.6load_min_set-      The number of dB below Pref-FDD which defines the top of the DRW_FDD. The top of the Dynamic Range
  FDD                    Window of the OFDMA FDD spectrum can be reduced from Pref-FDD by adjusting Pload_min_set-FDD to a value
                         greater than 0 dB.




122                                                           CableLabs                                                     06/27/25
<!-- PDF_PAGE: 123 -->
Physical Layer Specification                                                                      CM-SP-PHYv4.0-I07-250627


  P1.6low-FDD         Minimum transmit power to which a CM can be configured to transmit in the FDD band. OFDMA channels in
                      the FDD band do not have boosted pilots so P1.6low-FDD = P1.6Min-FDD.
  P1.6low_multi-FDD   Bottom of DRW_FDD.
  P1.6low_n-FDD       The minimum equivalent channel power for a particular FDD channel that the CM is permitted to support.
  P1.6min-FDD         Minimum transmit power to which a CM can be configured to transmit in the FDD band. OFDMA channels in
                      the FDD band do not have boosted pilots, so P1.6low-FDD = P1.6Min-FDD. P1.6min-FDD = -15 dB for the UHS-684 split
                      and P1.6min-FDD = -18 dB for the UHS-300, UHS-396, or UHS-492 split.
  P1.6r_n-FDD         The equivalent channel power of an FDD OFDMA channel "n". P1.6r_n-FDD is the average power at Interface Fˊ
                      of the OFDMA subcarriers of the channel normalized to 1.6 MHz bandwidth and referenced to the upstream
                      reference power spectral density at Interface Fˊ when the channel is fully granted. P1.6r_n-FDD is the channel
                      power reported in the RNG-REQ messages. This is also referred to in the specification as reported transmit
                      power.
  Plimit-FDD          The maximum power per channel, increased from Pref-FDD for which fidelity requirements only degrade
                      gradually, when P1.6load_min_set-FDD = 0. Fidelity requirements do not apply for channel power commanded higher
                      than Plimit-FDD. Fidelity requirements do not apply whenever total average power in the FDD band exceeds
                      PmaxFDD.
  PmaxFDD             The maximum total transmit power that the CM can support in the FDD band depends on the supported FDD
                      split option. The default value and the lowest allowable value for PmaxFDD is 58.6 dBmV for UHS-300, 61.3
                      dBmV for UHS-396, 63.6 dBmV for UHS-492, or 64.5 dBmV for UHS-684. Fidelity requirements do not apply
                      if total average power in the FDD band exceeds PmaxFDD.
  Pref-FDD            The upstream reference power spectral density in the FDD band. Pref-FDD is the power in dBmV in 1.6 MHz of
                      subcarriers with no pre-equalization. For UHS-684, Pref-FDD at 108.8 MHz is 33.0 dBmV/1.6 MHz and Pref-FDD
                      at 683.2 MHz is 43.0 dBmV/1.6 MHz for a slope of (10 dB/360 slots of 1.6 MHz in the FDD band) dBmV/1.6
                      MHz, and a total variation of 10.0 dB across 108 MHz to 684 MHz. For UHS-300, UHS-396, and UHS-492,
                      Pref-FDD at 108.8 MHz is 36.0 dBmV/1.6 MHz and Pref-FDD at 683.2 MHz is 46.0 dBmV/1.6 MHz for a slope of
                      (10 dB/360 slots of 1.6 MHz in the FDD band) dBmV/1.6 MHz and a total variation of 10.0 dB across 108
                      MHz to 684 MHz.
  Pt_n-FDD            The total transmit power in a channel n in the FDD band.
  Pt_sc_i             The average target power transmitted by the ith subcarrier for either probes or other transmissions, possibly
                      with different power for the probe transmissions (see Probedelta_n-FDD below).
  Pre-Eqi             The magnitude of the ith subcarrier pre-equalizer coefficient (dB).
  Probedelta_n-FDD    This term is used to account for reduction in Probe power resulting from the Power bit and Start Subc bits in
                      the Probe Information Element in the P-MAP for the nth FDD OFDMA channel.
  TCS_FDD             An FDD CM's transmit channel set in the FDD band (108 MHz to 684 MHz).



7.3.4.12.6 Reconfiguration Time for FDD CMs
In an FDD DOCSIS system, there are two independent transmission channel sets: one for the legacy upstream
channels (TCS), and one for the FDD upstream channels (TCS_FDD).
Section 7.2.4.12.4 of [DOCSIS PHYv3.1] applies to TCS only, while this section applies to TCS_FDD.
Reconfiguration time for FDD upstream channels is the inactive time interval provided between active upstream
transmissions on a given FDD upstream channel when a change is commanded for a transmission parameter on that
channel.
For changes in the ranging offset and/or pre-equalization of an FDD upstream channel, the FDD CM MUST be able
to transmit consecutive bursts as long as the CMTS allocates the time duration (reconfiguration time) of at least one
inactive frame in between the bursts on the FDD upstream channel with the changed parameter.
"Global reconfiguration time" in the FDD upstream channels is defined as the inactive time interval provided
between active FDD upstream transmissions, which simultaneously satisfies the requirement in this section for all
OFDMA channels in the TCS_FDD.
Global "quiet" across all active FDD upstream channels requires the intersection of ungranted burst intervals across
all active OFDMA FDD channels to be at least 20 microseconds.
Even with a change or re-command of P1.6load_min_set-FDD, the FDD CM MUST be able to transmit consecutive bursts
as long as the FDD CMTS allocates at least one frame in between bursts, across all OFDMA channels in the
TCS_FDD, where the quiet lapses in each channel contain an intersection of at least 20 microseconds. (From the end



06/27/25                                                    CableLabs                                                             123
<!-- PDF_PAGE: 124 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


of a burst on one FDD upstream channel to the beginning of the next burst on any other FDD upstream channel,
there is to be at least 20 microseconds duration to provide a "global reconfiguration time" for all channels in the
CM's TCS_FDD.)
The FDD CMTS SHOULD provide global reconfiguration time to the TCS_FDD for the FDD CM before (or
concurrently as) the CM has been commanded to change any upstream channel transmit power in the TCS_FDD by
±3 dB cumulative since its last global reconfiguration time.
Global Reconfiguration Time for the legacy upstream channels (TCS) is completely disassociated with TCS_FDD
grants or commands to the FDD CM, and Global Reconfiguration Time for the FDD upstream channels (TCS_FDD)
is completely disassociated with the TCS grants or commands to the FDD CM.

7.3.4.12.7 Fidelity Requirements for FDD
The following requirements assume that any pre-equalization is disabled, unless otherwise noted. Signal power and
measurements are all referenced to Interface Fˊ of Figure 30.
When channels in the TCS_FDD are commanded to the same equivalent channel powers, the reference signal power
in the "dBc" definition is to be interpreted as the measured average total transmitted power at Interface Fˊ. When
channels in the TCS_FDD are commanded to different equivalent channel powers, the commanded total power of
the transmission is computed, and a difference is derived compared to the commanded total power which would
occur if all channels had the same P1.6r_n-FDD as the highest equivalent channel power in the TCS_FDD, whether or
not the channel with the largest equivalent channel power is included in the grant. Then this difference is added to
the measured total transmit power to form the reference signal power for the "dBc" spurious emissions requirements.

7.3.4.12.7.1 Spurious Emissions for FDD
The noise and spurious power generated by the FDD CM MUST NOT exceed the levels given in Table 34 -
Spurious Emissions for FDD, Table 35 - FDD Spurious Emissions Requirements in the Upstream Frequency Range
for Grants of Under-Grant Hold Bandwidth and Larger, and Section 7.3.4.12.7.1.2, Adjacent Channel Spurious
Emissions.
Up to five discrete spurs can be excluded from the emissions requirements listed in Table 34 and Table 35 for each
of the spectral regions 85 MHz to 300 MHz, 300 MHz to 492 MHz, 492 MHz to 684 MHz, 684 MHz to 804 MHz,
804 MHz to 1218 MHz and 1218 MHz to 1794 MHz, while the FDD CM is Transmitting Burst upstream in the
FDD band. The five excluded discrete spurs have to be no more than 2 dB in excess of the MER value of Table 37,
with 100% grant, relative to a single subcarrier power level at the top of the DRW_FDD.
For example, with 12 dB uptilt at 108 MHz, the MER requirement from Table 37 is 36 dB, and so a discrete spur at
108 MHz, if one of the five to be excluded in the range of 85 MHz to 300 MHz, could reach as high as -34 dBc and still
qualify for exclusion, where 0 dBc corresponds to the power in a subcarrier at the top of the DRW_FDD at 108 MHz.
For the exclusions in the spectral regions 684 MHz to 804 MHz, 804 MHz to 1218 MHz and 1218 MHz to 1794 MHz,
the exclusion limit corresponds to the value 2 dB relaxed from the MER requirement at 684 MHz, and the 0 dBc
reference is the top of the DRW_FDD at 684 MHz. For the exclusions in the spectral region 85 MHz to108 MHz,
the exclusion limit corresponds to the value 2 dB relaxed from the MER requirement at 108 MHz, and the 0 dBc
reference is the top of the DRW_FDD at 108 MHz. In each band (5 MHz to 85 MHz; 108 MHz to 300 MHz; 300 MHz
to 492 MHz; 492 MHz to 684 MHz; 684 MHz to 804 MHz; 804 MHz to 1218 MHz and 1218 MHz to 1794 MHz)
up to 3 discrete spurs up to -40 dBmV may be excluded from the Between Burst requirements, and also 3 such
discrete spur exclusions up to -40 dBmV for the 5 MHz to 85 MHz Transmitting Burst requirement. Only a total of
15 different discrete spur exclusion frequencies are allowed in 5 MHz to 1218 MHz. The 15 different exclusion
frequencies are allowed, with the limitation of five or three per band as described above, but these 15 exclusion
frequencies are applied to all tests; a different set of ten exclusion frequencies is NOT allowed for different tests and
different modes.
SpurFloor is defined as:
         SpurFloor = -51.8 dBc
Under-grant hold number of users is defined as:
         Under-Grant Hold Number of Users = Floor{ 0.2 + 10^( (-44 - SpurFloor)/10) } = 6



124                                                  CableLabs                                               06/27/25
<!-- PDF_PAGE: 125 -->
Physical Layer Specification                                                             CM-SP-PHYv4.0-I07-250627


Under-grant hold bandwidth (UGHB) is defined as:
         Under-Grant Hold Bandwidth = (FDD Upstream Allocated Spectrum)/(Under-Grant Hold Number of Users)
The spurious performance requirements defined above only apply when the CM is operating within certain ranges of
values for P1.6load_i-FDD, for i = 1 to the number of upstream channels in the TCS_FDD, and for granted bandwidth of
under-grant hold bandwidth or larger; where P1.6load_1-FDD is the highest loaded channel in this specification (i.e., its
power is the one closest to Pref-FDD).
When an FDD CM is transmitting over a bandwidth of less than under-grant hold bandwidth the spurious emissions
requirement limit is the power value (in dBmV per 1.6 MHz), corresponding to the specifications for the power level
associated with a grant of bandwidth equal to under-grant hold bandwidth.
The FDD CM MUST meet the spurious emissions performance requirements when the equivalent DOCSIS channel
powers (P1.6r_n-FDD) are within 0-6 dB below the top of the DRW_FDD (P1.6load_min_set-FDD +6 ≥ P1.6load_i-FDD ≥
P1.6load_min_set-FDD) but is not required to meet spurious emissions performance requirements when P1.6r_n-FDD are not
within this range.
Further, the FDD CM MUST meet the spurious emissions performance requirements when P1.6oad_1-FDD =
P1.6load_min_set-FDD. When P1.6load_1-FDD > P1.6load_min_set-FDD, the spurious emissions requirements in absolute terms are
relaxed by P1.6load_1-FDD – P1.6load_min_set-FDD but is not required to meet spurious emissions performance requirements
when this condition is not met.
The spurious performance requirements do not apply to any upstream channel from the time the output power on
any active upstream channel has varied by more than ±3 dB since the last global reconfiguration time through the
end of the next global reconfiguration time changes.
In Table 34, in-band spurious emissions include noise, carrier leakage, clock lines, synthesizer spurious products,
and other undesired transmitter products. It does not include ISI. The measurement bandwidth for in-band spurious
emissions for OFDM is equal to the Subcarrier Clock Frequency (25 kHz or 50 kHz) and is not a synchronous
measurement. The signal reference power for OFDMA in-band spurious emissions is the total transmit power
measured and adjusted (if applicable) as described in Section 7.3.4.12.7, and then apportioned to a single data
subcarrier.
The measurement bandwidth is 4 MHz for the Between Bursts (none of the channels in the TCS_FDD is bursting)
specs of Table 34. The signal reference power for Between Bursts transmissions, "0 dBr", is the PSD for the top of
the DRW_FDD, as measured at Interface Fˊ.
The Transmitting Burst specs apply during the minislots granted to the FDD CM in the FDD band (when the FDD
CM uses all or a portion of the grant), and for 20 µs before and after the granted minislot for OFDMA. The Between
Bursts specs apply except during a used grant of minislots on any active channel in the FDD band for the CM, and
20 µs before and after the used grant for OFDMA. In Table 34 entries, the signal reference power, "0 dBr", is the
PSD for the top of the DRW_FDD, as measured at Interface Fˊ.
For the purpose of spurious emissions definitions, a granted burst refers to a burst of minislots to be transmitted at
the same time from the same FDD CM; these minislots are not necessarily contiguous in frequency.
For Initial Ranging and before completion of Fine Ranging, spurious emissions requirements use Table 34 and Table
35, and if transmissions use subcarrier power which is X dB lower than indicated by P1.6low-FDD, then the spurious
emissions requirements in absolute terms are relaxed by X dB.
Spurious emissions requirements for grants of 10% or less of the FDD upstream allocated spectrum may be relaxed
by 2 dB in an amount of spectrum equal to:
measurement BW * ceil(10% of the FDD upstream allocated spectrum/measurement BW)
anywhere in the whole upstream spectrum for emission requirements specified in Table 35.
A 2 dB relief applies in the measurement bandwidth. This relief does not apply to between bursts emission
requirements.
The FDD CMTS MUST NOT command a grant to the FDD CM in the FDD band which is smaller than the
minimum grant bandwidth shown in Table 35 - FDD Spurious Emissions Requirements in the Upstream Frequency
Range for Grants of Under-Grant Hold Bandwidth and Larger, which is 10.4 MHz with 192 MHz FDD upstream


06/27/25                                               CableLabs                                                      125
<!-- PDF_PAGE: 126 -->
CM-SP-PHYv4.0-I07-250627                                               Data-Over-Cable Service Interface Specifications


allocated spectrum, 16.0 MHz with 288 MHz FDD upstream allocated spectrum, 21.2 MHz with 384 MHz FDD
upstream allocated spectrum, and 32.0 MHz with 576 MHz FDD upstream allocated spectrum.
                                         Table 34 - Spurious Emissions for FDD CM

                 Parameter                       Transmitting Burst 1,5,10,11,12,14,16          Between Bursts 5,10,11,12,15,16
Inband (modulated spectrum of the grant)        -42 dBr OFDMA 100% grant      4,5,6,8,9
                                                                                            Max{-72 dBr, -43 dBmV}
                                                -47 dBr OFDMA UGHB% grant 4,5,6,8,9
Adjacent minislot (adjacent to the modulated    + 0.2 dB relaxation.                        Same as for Inband
spectrum of the grant)                          See Table 35
400 kHz next to modulated spectrum
FDD band                                        See 7.3.4.12.7.1.2                          Same as for Inband
Within 108 MHz to [upstream upper edge]
(excluding assigned channel, adjacent
channels).
Requirements for the emissions from
5 MHz to 108 MHz and 684 MHz and above
CM integrated spurious emissions limits
(all in 4 MHz, includes discretes)1

5 MHz to 85 MHz                                 -45 dBmV                                    -50 dBmV
85 MHz to 108 MHz                               -35 dBr                                     Same as Inband
Upstream/Downstream transition                  -42 dBr                                     Same as Inband
[DS lower band edge] to 1794 MHz                -45 dBmV                                    -45 dBmV
CM discrete spurious emissions limits1          For all four bands:                         5 MHz to 85 MHz

5 MHz to 85 MHz                                 Largest Discrete Spurious Emissions at      Largest discrete spurious emissions at
85 MHz to 108 MHz                               least 5 dB lower power than the limit on    least 3 dB lower power than the limit on
                                                Integrated Spurious Emissions in 4 MHz,     integrated spurious emissions in 4 MHz,
Upstream/Downstream transition band
                                                in the same band (see table row above)      in the same band (see table row above)
[DS lower band edge] to 1794 MHz

                                                                                            85 MHz to 108 MHz
                                                                                            Upstream/Downstream transition band
                                                                                            [DS lower band edge to 1794 MHz]
                                                                                            Largest discrete spurious emissions at
                                                                                            least 5 dB lower power than the limit on
                                                                                            integrated spurious emissions in 4
                                                                                            MHz, in the same band (see table row
                                                                                            above)
TABLE NOTES
NOTE 1    Up to 5 discrete spurs in each of the following bands may be excluded: 85 MHz to 300 MHz; 300 MHz to 492 MHz;
          492 MHz to 684 MHz; 684 MHz to 804 MHz; and 804 MHz to 1218 MHz, 1218 MHz to 1794 MHz while the CM is
          Transmitting Burst in the FDD band. These 5 spurs are the same spurs that may be excluded for spurious emissions
          and MER and not an additional or different set.
NOTE 2    N/A.
NOTE 3    N/A.
NOTE 4    N/A.
NOTE 5    This value is to be met when P1.6load = P1.6load_min_set. "0 dBr" is referenced to the top of the DRW_FDD.
NOTE 6    Receive equalization is allowed if an MER test approach is used, to take ISI out of the measurement; measurements
          other than MER-based to find spurs or other unwanted power may be applied to this requirement.
NOTE 7    N/A
NOTE 8    Frequency-Dependent Relaxation as a function of the measurement center frequency, linearly scales in frequency
          from 5 dB at 108 MHz to 0 dB at 684 MHz.
NOTE 9    1 dB relaxation with 12 dB uptilt.
NOTE 10 "dBr" values measured at Interface Fˊ, "dBmV" values measured at Interface F, and all measurements and averages
        computed over 4 MHz bandwidth unless noted otherwise.
NOTE 11 Transmitting Burst is with FDD band upstream bursting, and Between Burst is with FDD band upstream between
        bursts entirely in 108 MHz to 684 MHz.




126                                                        CableLabs                                                     06/27/25
<!-- PDF_PAGE: 127 -->
Physical Layer Specification                                                                 CM-SP-PHYv4.0-I07-250627


NOTE 12 For all the requirements except for the 5 MHz to 85 MHz and 85 MHz to 108 MHz, both Transmitting Burst and
        Between Burst, the requirements need to be met with the legacy upstream (5 MHz to 85 MHz) bursting. With the
        requirements in 5 MHz to 85 MHz and 85 MHz to 108 MHz, the requirements need to be met with the legacy upstream
        (5 MHz to 85 MHz) between bursts.
NOTE 13 N/A.
NOTE 14 Allowance for 3 excluded discrete spurs up to -40 dBmV in the 5 MHz to 85 MHz while Transmitting Burst.
NOTE 15 Allowance for 3 excluded discrete spurs in each of the bands (5 MHz to 85 MHz; 85 MHz to 300 MHz; 300 MHz to 492
        MHz; 492 MHz to 684 MHz; 684 MHz to 804 MHz, 804 MHz to 1218 MHz and 1218 MHz to 1794 MHz) up to -40
        dBmV while Between Bursts.
NOTE 16 For the discrete spur exclusions of Note 1, Note 14, and Note 15, a total of 15 different such exclusion frequencies are
        allowed to be applied for all the requirements, across the full 5 MHz to 1794 MHz range, while also accommodating
        the restrictions on the number of exclusions in any one band. The 15 exclusion frequencies are not allowed to change
        for application to different requirements, nor due to changes in mode in the CM under test.



7.3.4.12.7.1.1 Spurious Emissions in the Upstream FDD Frequency Range
Table 35 lists the required spurious level in a measurement interval. The initial measurement interval at which to
start measuring the spurious emissions (from the transmitted burst's modulation edge) is 400 kHz from the edge of
the transmission's modulation spectrum. Measurements should start at the initial distance and be repeated at
increasing distance from the carrier, until the upstream band edge or spectrum adjacent to another modulated
spectrum is reached.
In addition to the spurious emissions level generated in Table 36, there is a frequency-dependent relaxation provided
as a function of the center frequency of the measurement, which is given in Table Note 3 of Table 36 as:
         Frequency-Dependent_Spurious_Emissions_Relaxation =
         5 * (684 MHz – measurement_center_frequency)/(684 MHz – 108 MHz) dB.
For example, with 576 MHz FDD upstream allocated spectrum and 96 MHz grant, the requirement is -61.8 dBc at
679.2 MHz and -56.8 dBc at 112.8 MHz.
For OFDMA transmissions with non-zero transmit windowing, the FDD CM MUST meet the required performance
measured within the 2.0 MHz adjacent to the modulated spectrum using slicer values from an FDD CMTS burst
receiver or equivalent, synchronized to the downstream transmission provided to the FDD CM.
In the rest of the spectrum, the FDD CM MUST meet the required performance measured with a bandpass filter
(e.g., an unsynchronized measurement).
For OFDMA transmissions with zero transmit windowing, the FDD CM MUST meet the required performance
using synchronized measurements across the complete upstream spectrum.
Spurious emissions allocation for far out spurious emissions =
         Round {SpurFloor + 10*log10(Measurement Bandwidth/Under-Grant Hold Bandwidth),0.1}.
For transmission bursts with modulation spectrum less than the under-grant hold bandwidth, the spurious power
requirement is calculated as above, but increased by 10*log10 (under-grant hold bandwidth/grant bandwidth).
                  Table 35 - FDD Spurious Emissions Requirements in the Upstream Frequency
                          Range for Grants of Under-Grant Hold Bandwidth and Larger

 FDD Upstream         Minimum                                                 Under-
   Allocated           Grant                                Under-          Grant Hold       Measurement         Specification in
   Spectrum          Bandwidth          SpurFloor         Grant Hold        Bandwidth         Bandwidth            the Interval
     (MHz)             (MHz)              (dBc)            #Users             (MHz)             (MHz)1               (dBc)2,3
       192                10.4             - 51.8               6                 32                9.6                 -57.0
       288                16.0             - 51.8               6                 48                9.6                 -58.8
       384                21.2             - 51.8               6                 64                9.6                 -60.0
       576                32.0             - 51.8               6                 96                9.6                 -61.8




06/27/25                                                CableLabs                                                          127
<!-- PDF_PAGE: 128 -->
CM-SP-PHYv4.0-I07-250627                                         Data-Over-Cable Service Interface Specifications


 TABLE NOTES
 NOTE 1 The measurement bandwidth is a contiguous sliding measurement window.
 NOTE 2 Frequency-Dependent Relaxation as a function of the measurement center frequency, linearly scales in frequency from
        5 dB at 108 MHz to 0 dB at 684 MHz.
 NOTE 3 1 dB relaxation with 12 dB uptilt


The FDD CM MUST control transmissions such that within the measurement bandwidth of Table 35 - FDD
Spurious Emissions Requirements in the Upstream Frequency Range for Grants of Under-Grant Hold Bandwidth
and Larger, spurious emissions measured for individual subcarriers contain no more than +3 dB power larger than
the required average power of the spurious emissions in the full measurement bandwidth divided by the number of
subcarriers in the measurement bandwidth. When non-synchronous measurements are made, only 25 kHz
measurement bandwidth is used.
For OFDMA transmissions, bandpass measurements rather than synchronous measurements may be applied.

7.3.4.12.7.1.2 Adjacent Channel Spurious Emissions
Spurious emissions from a transmitted burst may occur in adjacent spectrum, which could be occupied by OFDMA
subcarriers transmitted by another CM.
The spurious emissions requirements for adjacent spectrum to a transmitted burst are given in Table 35 but with an
additional 0.2 dB allowance, where the measurement is over the 9.6 MHz spectrum adjacent to the modulated
spectrum.
The measurement is performed starting on an adjacent subcarrier of the transmitted spectrum (both above and
below), using the slicer values from a FDD CMTS burst receiver or equivalent synchronized to the downstream
transmission provided to the FDD CM.
The FDD CM MUST control transmissions such that within the adjacent 400 kHz of modulated spectrum, spurious
emissions measured for individual subcarriers contain no more than +5 dB power larger than the required average
power of the spurious emissions in the full measurement bandwidth divided by the number of subcarriers in the
measurement bandwidth.
For the 9.2 MHz measurement bandwidth which is outside the 400 kHz adjacent to the modulated spectrum, the
FDD CM MUST control transmissions such that spurious emissions measured for individual subcarriers contain no
more than +3 dB power larger than the required average power of the spurious emissions in the full measurement
bandwidth divided by the number of subcarriers in the measurement bandwidth. For any portion of the 9.6 MHz
measurement bandwidth where non-synchronous measurements are made, only 25 kHz measurement bandwidth is
used.
Bandpass measurements rather than synchronous measurements may be applied where possible.

7.3.4.12.7.1.3 Spurious Emissions During FDD Burst On/Off Transients
The FDD CM MUST control spurious emissions prior to and during ramp-up, during and following ramp-down, and
before and after a burst.
The FDD CM's on/off spurious emissions, such as the change in voltage at the upstream transmitter output, due to
enabling or disabling transmission, MUST be no more than 50 mV.
The FDD CM's voltage step MUST be dissipated no faster than 4 μs of constant slewing. This requirement applies
when the FDD CM is transmitting at +55 dBmV or more per channel on any channel.
At backed-off transmit levels, the FDD CM's maximum change in voltage MUST decrease by a factor of 2 for each
6 dB decrease of power level in the highest power active channel, from +55 dBmV per channel, down to a
maximum change of 3.5 mV at 31 dBmV per channel and below; this requirement does not apply to FDD CM
power-on and power-off transients.




128                                                   CableLabs                                                 06/27/25
<!-- PDF_PAGE: 129 -->
Physical Layer Specification                                                            CM-SP-PHYv4.0-I07-250627


7.3.4.12.7.2 FDD OFDMA MER Requirements
Transmit modulation error ratio (TxMER or just MER) measures the cluster variance caused by the FDD CM during
upstream transmission due to transmitter imperfections. The terms "equalized MER" and "unequalized MER" refer
to a measurement with linear distortions equalized or not equalized, respectively, by the test equipment receive
equalizer. The requirements in this section refer only to unequalized MER, as described for each requirement. MER
is measured on each modulated data subcarrier and non-boosted pilot (MER is computed based on the unboosted
pilot power) in a minislot of a granted burst and averaged for all the subcarriers in each minislot. MER includes the
effects of Inter-Carrier Interference (ICI), spurious emissions, phase noise, noise, distortion, and all other undesired
transmitter degradations with an exception for a select number of discrete spurs impacting a select number of
subcarriers. MER requirements are measured with a calibrated test instrument that synchronizes to the OFDMA
signal, applies a receive equalizer in the test instrument that removes MER contributions from nominal channel
imperfections related to the measurement equipment, and calculates the value. The equalizer in the test instrument is
calculated, applied and frozen for the FDD CM testing. Receiver equalization of FDD CM linear distortion is not
provided; hence, this is considered to be a measurement of unequalized MER, even though the test equipment
contains a fixed equalizer setting.

7.3.4.12.7.2.1 Definitions
MER is defined as follows for OFDMA. The transmitted RF waveform at the F connector of the FDD CM (after
appropriate down conversion) is filtered, converted to baseband, sampled, and processed using standard OFDMA
receiver methods, with the exception that receiver equalization is not provided. The processed values are used in the
following formula. No external noise (AWGN) is added to the signal.
The carrier frequency offset, carrier amplitude, carrier phase offset, and timing will be adjusted during each burst to
maximize MER as follows:
    •    One carrier amplitude adjustment common for all subcarriers and OFDM symbols in burst.
    •    One carrier frequency offset common for all subcarriers resulting in phase offset ramping across OFDM
         symbols in bursts.
    •    One timing adjustment resulting in phase ramp across subcarriers.
    •    One carrier phase offset common to all subcarriers per OFDM symbol in addition to the phase ramp.
MERi is computed as an average of all the subcarriers in a minislot for the ith minislot in the OFDMA grant:




where:
         Eavg is the average constellation energy for equally likely symbols,
         M is the number of symbols averaged,
         N is the number of subcarriers in a minislot,
         ej,k is the error vector from the jth subcarrier in the minislot and kth received symbol to the ideal transmitted
         QAM symbol of the appropriate modulation order.
A sufficient number of OFDMA symbols are to be included in the time average so that the measurement uncertainty
from the number of symbols is less than other limitations of the test equipment.
MER with a 100% grant is defined as the condition when all OFDMA non-excluded subcarriers in the transmit
channel set are granted to the FDD CM. For purposes of testing MER, a grant of all OFDMA minislots in the
transmit channel set may be used; there may be non-excluded subcarriers that are not within minislots.
MER with a UGHB is defined as the condition when less than or equal to UGHB of the FDD OFDMA minislots
have been granted to the FDD CM.




06/27/25                                              CableLabs                                                      129
<!-- PDF_PAGE: 130 -->
CM-SP-PHYv4.0-I07-250627                                                Data-Over-Cable Service Interface Specifications


7.3.4.12.7.2.2 FDD Requirements
Unless otherwise stated, the FDD CM MUST meet or exceed the following MER limits over the full transmit power
range, all modulation orders, all grant configurations and over the full upstream frequency range.
The following flat channel measurements (ideally flat channel except for downtilt specified between Interface F and
F') with the transmitted specified uptilt (Table 36) are made after the pre-equalizer coefficients have been set to their
optimum values. The receiver uses best effort synchronization to optimize the MER measurement.
                              Table 36 - Upstream MER Requirements (with Pre-Equalization)

          Parameter                                                               Value
  MER (100% grant)                     Each minislot MER ≥ 42 dB (Notes 1, 2, 3, 4, 5) at 684 MHz with 8 dB and 10 dB uptilt
  MER (UGHB % grant)                   Each minislot MER ≥ 47 dB (Notes 1, 2, 3, 4) at 684 MHz with 8 dB and 10 dB uptilt
  Pre-equalizer constraints            Coefficients set to their optimum values
 TABLE NOTES
 NOTE 1 Up to 5 subcarriers within the entire upstream bandwidth with discrete spurs may be excluded from the MER
        calculation if they fall within transmitted minislots. These 5 spurs are the same spurs that may be excluded for spurious
        emissions and not an additional or different set.
 NOTE 2 This value is to be met when P1.6load = P1.6load_min_set.
 NOTE 3 Frequency-Dependent Relaxation as a function of the measurement center frequency, linearly scales in frequency from
        5 dB at 108 MHz to 0 dB at 684 MHz.
 NOTE 4 1 dB relaxation with 12 dB uptilt
 NOTE 5 For testing 100% grant, a grant of all OFDMA minislots may be used.


The following flat channel measurements (Table 37) are made with the pre-equalizer coefficients set to unity and
with the transmitted specified uptilt and the receiver implementing best effort synchronization. For this
measurement, the receiver may also apply partial equalization. The partial equalizer is not to correct for the portion
of the FDD CM's time-domain impulse response greater than 200 ns or frequency-domain amplitude response
greater than +1 dB or less than -3 dB from the average amplitude. An additional 1 dB attenuation in the amplitude
response is allowed in the upper 10% of the specified passband frequency. It is not expected that the partial
equalizer is implemented on the FDD CMTS receiver. A partial equalizer could be implemented offline via
commercial receivers or simulation tools.
                              Table 37 - Upstream MER Requirements (no Pre-Equalization)

           Parameter                                                              Value
  MER (100% grant)                      Each minislot MER ≥ 39 dB (Notes 1, 2, 3, 4) at 684 MHz with 8 dB and 10 dB uptilt
  MER (UGHB% grant)                     Each minislot MER ≥ 39 dB (Notes 1, 2, 3, 4) at 684 MHz with 8 dB and 10 dB uptilt
  Pre-equalizer constraints             Pre-equalization not used
 TABLE NOTES
 Note 1    Up to 5 subcarriers within the entire upstream bandwidth with discrete spurs may be excluded from the MER
           calculation if they fall within transmitted minislots. These 5 spurs are the same spurs that may be excluded for spurious
           emissions and not an additional or different set.
 Note 2    This value is to be met when P1.6load = P1.6load_min_set.
 Note 3    Frequency-dependent relaxation as a function of the measurement center frequency, linearly scales in frequency from
           5 dB at 108 MHz to 0 dB at 684 MHz.
 Note 4    1 dB relaxation with 12 dB uptilt

7.3.4.13 Cable Modem Transmitter Output Requirements
The FDD CM MUST output an RF Modulated signal with characteristics delineated in Table 38 - FDD CM
Transmitter Output Signal Characteristics.




130                                                            CableLabs                                                   06/27/25
<!-- PDF_PAGE: 131 -->
Physical Layer Specification                                                               CM-SP-PHYv4.0-I07-250627


                            Table 38 - FDD CM Transmitter Output Signal Characteristics

                Parameter                                                       Value
  Frequency                             Support and be configurable to a permitted subset (see section 7.3.2.3 for allowed
                                        combinations) of the following list of frequency ranges:
                                        5 MHz to 204 MHz
                                        5 MHz to 85 MHz and 108 MHz to 300 MHz
                                        5 MHz to 85 MHz and 108 MHz to 396 MHz
                                        5 MHz to 85 MHz and 108 MHz to 492 MHz
                                        5 MHz to 85 MHz and 108 MHz to 684 MHz
                                        NOT to cause harmful interference above these frequencies for any configured option.
  Signal type                           OFDMA
  Maximum OFDMA channel                 95 MHz
  bandwidth
  Minimum OFDMA occupied                6.4 MHz for 25 kHz subcarrier spacing
  bandwidth                             10 MHz for 50 kHz subcarrier spacing
  Number of independently               Minimum of 7
  configurable OFDMA channels
  Subcarrier channel spacing            25 kHz, 50 kHz
  FFT size                              50 kHz: 2048 (2K FFT); 1900 maximum active subcarriers
                                        25 kHz: 4096 (4K FFT); 3800 maximum active subcarriers
  Sampling rate                         102.4 MHz
  FFT time duration                     40 µs (25 kHz subcarriers)
                                        20 µs (50 kHz subcarriers)
  Modulation type                       BPSK, QPSK, 8-QAM, 16-QAM, 32-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM,
                                        1024-QAM, 2048-QAM, 4096-QAM
  Bit loading                           Variable from minislot to minislot
                                        Constant for subcarriers of the same type in the minislot
                                        Support zero valued subcarriers per profile and minislot
  Pilot tones                           14 data patterns and 2 subslot patterns, minislot subcarrier size and length dependent
                                        - see Section 7.3.4.16.
  Cyclic prefix options                 Samples        µs
                                        96             0.9375
                                        128            1.25
                                        160            1.5625
                                        192            1.875
                                        224            2.1875
                                        256            2.5
                                        288            2.8125
                                        320            3.125
                                        384            3.75
                                        512            5.0
                                        640            6.25
  Windowing size options                Samples       µs
                                        0             0
                                        32            0.3125
                                        64            0.625
                                        96            0.9375
                                        128           1.25
                                        160           1.5625
                                        192           1.875
                                        224           2.1875
                                        Raised cosine absorbed by CP
  Level                                 TCP of 65 dBmV or greater
  Output impedance                      75 ohms
  Output return loss                    > 6 dB in the selected frequency range in row 1
  Connector                             F connector per [ISO/IEC-61169-24] or [SCTE 02]




06/27/25                                               CableLabs                                                        131
<!-- PDF_PAGE: 132 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


7.3.4.14 FDD RMD/RPD Receiver Capabilities

7.3.4.14.1.1 FDD RMD/RPD Receiver Input Power Requirements at Interface C
The FDD RMD/RPD MUST be settable according to Table 39 - FDD RMD/RPD Upstream Channel Demodulator
Input Power Characteristics for intended received power normalized to 6.4 MHz of bandwidth. To clarify, an FDD
RMD/RPD that implements a range of settable (chosen) set points (per US RF port) across a portion of the range in
Table 39 complies with this requirement, and so is an FDD RMD/RPD that implements just a single fixed set point
in this range.
The FDD RMD/RPD upstream demodulator MUST operate within its defined performance specifications at any set
point it supports, with received bursts within the ranges defined in Table 39 - FDD RMD/RPD Upstream Channel
Demodulator Input Power Characteristics of the set power.
             Table 39 - FDD RMD/RPD Upstream Channel Demodulator Input Power Characteristics

                                       Minimum Set Point        Maximum Set Point
                     Modulation         (dBmV/6.4 MHz)           (dBmV/6.4 MHz)           Range
                    QPSK                        -4                       25               -9 / +3
                    8-QAM                       -4                       25               -9 / +3
                    16-QAM                      -4                       25               -9 / +3
                    32-QAM                      -4                       25               -9 / +3
                    64-QAM                      -4                       25               -9 / +3
                    128-QAM                     0                        25               -9 / +3
                    256-QAM                     0                        25               -9 / +3
                    512-QAM                     0                        25               -3 / +3
                    1024-QAM                    0                        25               -3 / +3
                    2048-QAM                    7                        25               -3 / +3
                    4096-QAM                    10                       25               -3 / +3


BaseTargetRxPower TLV (98.3) in annex B of [R-PHY] is the reference power spectral density for all the upstream
channels, in units of "dBmV/1.6 MHz"; note that this corresponds to the set point in this section, except that the
latter has units of "dBmV/6.4 MHz." Each channel has its own TargetRxPowerAdjust TLV in annex B of [R-PHY],
moving the target power spectral density for the indicated channel up or down from the set point (i.e., moved from
the reference). However, the performance requirement admits no such adjustments (all channels are at the reference
power spectral density, i.e., "the chosen set point"). This is the intention of the requirements and how they will be
tested.
The FDD RMD/RPD MUST operate in the following conditions:
      •   The average upstream input total composite power, including ingress and noise, is up to 3 dB higher than
          the calculated total composite power based upon the chosen set point, as measured at interface C.
          and
      •   The average upstream input power, including ingress and noise, in any 6.4 MHz contiguous spectral region
          within the upstream frequency band is up to 8 dB higher than the set point, as measured at interface C.

7.3.4.14.2 FDD RMD/RPD Receiver Error Ratio Performance in AWGN Channel at Interface C
The required level for FDD RMD/RPD upstream post-FEC error ratio is defined for AWGN as less than or equal to
10-6 PER (packet error ratio) with 1500-byte Ethernet packets. This section describes the conditions at which the
FDD RMD/RPD is required to meet this error ratio.




132                                                  CableLabs                                            06/27/25
<!-- PDF_PAGE: 133 -->
Physical Layer Specification                                                                CM-SP-PHYv4.0-I07-250627


Implementation loss of the FDD RMD/RPD receiver MUST be such that the FDD RMD/RPD achieves the required
error ratio when operating at a CNR as shown in Table 40 - FDD RMD/RPD Minimum CNR Performance in
AWGN Channel, under input load and channel conditions as follows:
    •   A single transmitter, pre-equalized and ranged.
    •   An OFDMA channel with 95 MHz modulated spectrum (test channel).
    •   Ranging with same CNR and input level to FDD RMD/RPD measured at Interface C as with data bursts,
        and with 8-symbol probes.
    •   Input power level per constellation is at 0 dB offset from the minimum set point supported by the
        RMD/RPD, or the minimum set point defined in Table 39 - FDD RMD/RPD Upstream Channel
        Demodulator Input Power Characteristics, whichever is greater.
    •   Any remaining upstream spectrum is filled with DOCSIS 3.1 channels and/or DOCSIS 3.0 channels and/or
        R-OOB channels with the same PSD (except for minimal guard bands as required by the specifications).
    •   OFDMA phase noise and frequency offset are at the max limits as defined for the CM transmission
        specification.
    •   Ideal AWGN channel with no other artifacts (reflections, burst noise, tilt, etc.).
    •   Large grants consisting of several 1500 Bytes.
    •   The FDD CMTS is allowed to construct MAPs according to its own scheduler implementation.

                   Table 40 - FDD RMD/RPD Minimum CNR Performance in AWGN Channel

                                                                     Set Point4
                  Constellation            CNR1,2,3 (dB)          (dBmV/6.4 MHz)                Offset
                QPSK                            11.0                      -4                      0 dB
                8-QAM                           14.0                      -4                      0 dB
                16-QAM                          17.0                      -4                      0 dB
                32-QAM                          20.0                      -4                      0 dB
                64-QAM                          23.0                      -4                      0 dB
                128-QAM                         26.0                       0                      0 dB
                256-QAM                         29.0                       0                      0 dB
                512-QAM                         32.5                       0                      0 dB
                1024-QAM                        35.5                       0                      0 dB
                2048-QAM                        39.0                       7                      0 dB
                4096-QAM                        43.0                      10                      0 dB
               TABLE NOTES
               NOTE 1 CNR is defined here as the ratio of average signal power in occupied bandwidth to the
                      average noise power in the occupied bandwidth given by the noise power spectral
                      density integrated over the same occupied bandwidth.
               NOTE 2 Channel CNR is adjusted to the required level by measuring the source inband noise
                      including phase noise component and adding the required delta noise from an
                      external AWGN generator.
               NOTE 3 The channel CNR requirements are for OFDMA channels with non-boosted pilots. For
                      operation with boosted pilots (legacy TCS), the CNR requirements are increased by
                      a) 1 dB for channels with 50 kHz subcarrier spacing, and b) 0.5 dB for channels with
                      25 kHz subcarrier spacing. For example, the CNR requirement for QPSK with boosted
                      pilots is 12.0 dB with 50 kHz subcarrier spacing and 11.5 dB with 25 kHz subcarrier
                      spacing.
               NOTE 4 The set point is as listed in this table, if supported by the RMD/RPD. Otherwise, it is
                      the minimum set point supported by the RMD/RPD.




06/27/25                                               CableLabs                                               133
