# 7.2.4 - FDX Upstream Transmit and Receive

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 53-76
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 53 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


7.2.4     Upstream Transmit and Receive

7.2.4.1     Signal Processing Requirements
See [DOCSIS PHYv3.1], section 7.4.1.

7.2.4.2     Time and Frequency Synchronization
See [DOCSIS PHYv3.1], section 7.4.2.

7.2.4.2.1    Channel Frequency Accuracy
This section augments section 7.4.2.1 of [DOCSIS PHYv3.1].
The frequency of the upstream subcarrier clock (or upstream subcarrier spacing) is required to be accurate within
0.4 ppm and each subcarrier frequency accurate within 30 Hz, both relative to the master clock reference, and both
for five sigma of the upstream OFDMA transmissions, for subcarrier frequencies up to 684 MHz for CFDX
operation or 834 MHz for SFDX operation. The measurements of the frequency of the upstream subcarrier clock,
and the subcarrier frequencies, are averaged over the duration of an upstream single-frame grant. A constant
temperature is maintained during the measurements within a range of 20 ºC ± 2 ºC. A minimum warm-up time of
30 minutes occurs before the CM frequency measurements are made.

7.2.4.2.2    Channel Timing Accuracy
See [DOCSIS PHYv3.1], section 7.4.2.2.

7.2.4.2.3    Modulation Timing Jitter
See [DOCSIS PHYv3.1], section 7.4.2.3.

7.2.4.3     Forward Error Correction
See [DOCSIS PHYv3.1], section 7.4.3.

7.2.4.4     Data Randomization
See [DOCSIS PHYv3.1], section 7.4.4.

7.2.4.5     Time and Frequency Interleaving and De-interleaving
See [DOCSIS PHYv3.1], section 7.4.5.

7.2.4.6     Mapping of Bits to Cell Words
See [DOCSIS PHYv3.1], section 7.4.6.

7.2.4.7     Mapping and Demapping Bits to/from QAM Subcarriers
See [DOCSIS PHYv3.1], section 7.4.7.

7.2.4.8     REQ Messages
See [DOCSIS PHYv3.1], section 7.4.8.

7.2.4.9     IDFT
See [DOCSIS PHYv3.1], section 7.4.9.

7.2.4.10 Cyclic Prefix and Windowing
See [DOCSIS PHYv3.1], section 7.4.10.




06/27/25                                           CableLabs                                                   53
<!-- PDF_PAGE: 54 -->
CM-SP-PHYv4.0-I07-250627                                                                Data-Over-Cable Service Interface Specifications


7.2.4.11 Burst Timing Convention
See [DOCSIS PHYv3.1], section 7.4.11.

7.2.4.12 FDX Fidelity Requirements
For FDX devices operating in FDX mode, this section augments the requirements of section 7.4.12.5 of [DOCSIS
PHYv3.1] and corresponding subsections unless otherwise noted.
A DOCSIS FDX CM is required to generate 7 OFDMA channels if it supports the CFDX frequency plan or
8 OFDMA channels if it supports the SFDX frequency plan, as defined in Section 7.2.2.
A CM's transmit channel set in the FDX spectrum (TCS_FDX), i.e., 108 MHz to 684 MHz for CFDX or 258 to 834
MHz for SFDX, is the FDX OFDMA channels that can be transmitted by the CM in that band, independent of the
current RBA in use. The FDX allocated spectrum has five possible values: 96 MHz, 192 MHz, 288 MHz, 384 MHz,
and 576 MHz. The upstream occupied bandwidth in the FDX allocated spectrum is the sum of the occupied
bandwidth of all OFDMA channels in the cable modem's FDX transmit channel set. The bandwidth occupied by an
OFDMA probe with a skip value of zero is equal to the upstream occupied bandwidth of an OFDMA channel. The
FDX CM MUST comply with the fidelity requirements in this section.

7.2.4.12.1 Upstream Fidelity Measurement Framework
The upstream fidelity measurement framework for the FDX band illustrated in Figure 8 is referenced for upstream
channel power requirements that follow.

                           Cable Modem Upstream Power & Fidelity Measurements at Interface F

                                                            Flat Measurement PSD                                  Upstream Reference PSD z dB uptilt
                                                      dBmV                                                         dBmV
                                                                                                                                              y+z


                                                                  X dBmV (flat)                                           y

                                                                                           MHz                                                          MHz
                                      CFDX frequency plan   108                  684                                    108                   684
                                      SFDX frequency plan   258                  834                                    258                   834




              dBmV
                                                                                                Convert
                                                                  INTERFACE F’         Upstream Reference PSD                     INTERFACE F           CABLE
     x dBmV                                                                            To Flat Measurement PSD                                         MODEM


                                                    MHz
                                                                     F’                (Mathematical Adjustment)                      F
                                                                                                                                                        CABLE
                                                                                                                                                       MODEM
           Measured deviations from flat line                                                                                                          CONTROL
           Indicate absolute power deviations from                                Mathematical adjustment converts Upstream
           the Upstream Reference PSD at                                          Reference PSD to a flat measurement PSD.
           Interface F.                                                           May be included in analyzer function.

          Fidelity measurements to be taken on                                                                              Alternative:             BROADBAND
          flattened spectrum for consistency with                                                                     Total Composite Power         POWER METER
          DOCSIS 3.1 techniques                                                                                        (TCP) Measurement



              Figure 8 - FDX Cable Modem Upstream Power and Fidelity Measurements at Interface F




54                                                                      CableLabs                                                                       06/27/25
<!-- PDF_PAGE: 55 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627


With BWOFDMA-FDX being the combined occupied bandwidth of the OFDMA channel(s) in its TCS_FDX, the CM is
said to have Neq-FDX = ceil (BWOFDMA-FDX (MHz)/1.6 MHz) "equivalent DOCSIS channels" in its TCS_FDX.
An FDX CM MUST be capable of transmitting a TCP of 65 dBmV.
When operating with the CFDX frequency plan, an FDX CM MUST be capable of transmitting a total average
output power of 64.5 dBmV in the FDX band.
When operating with the CFDX frequency plan, the FDX CM MUST be capable of transmitting a total average
output power of 55 dBmV in the legacy US band when operating in FDX mode.
When operating with the SFDX frequency plan, an FDX CM MUST be capable of transmitting a total average
output power of 63.75 dBmV in the FDX band.
When operating with the SFDX frequency plan, the FDX CM MUST be capable of transmitting a total average
output power of 59 dBmV in the legacy US band when operating in FDX mode.
An FDX CM that supports CFDX operation MAY be capable of transmitting a total average output power greater
than 64.5 dBmV in the FDX band.
An FDX CM that supports SFDX operation MAY be capable of transmitting a total average output power greater
than 63.75 dBmV in the FDX band.
Interface F has a requirement on the maximum total average power in the FDX band that is 64.5 dBmV for CFDX
operation or 63.75 dBmV for SFDX operation. This power value is realized when the FDX modulated spectrum is at
the maximum, which is 570 MHz, and reduces from its maximum as modulated spectrum reduces.
At Interface F, the upstream reference PSD is defined, which is a line in dB for the y-axis and linear frequency in the
x-axis. For the CFDX frequency plan, this line passes through the points 33.0 dBmV in 1.6 MHz centered at
108.8 MHz and 43.0 dBmV centered at 683.2 MHz. For the SFDX frequency plan, this line passes through the
points 32.25 dBmV in 1.6 MHz centered at 258.8 MHz and at 42.25 dBmV centered at 833.2 MHz.
The upstream reference PSD is illustrated in Figure 9 below.




06/27/25                                            CableLabs                                                     55
<!-- PDF_PAGE: 56 -->
CM-SP-PHYv4.0-I07-250627                                           Data-Over-Cable Service Interface Specifications


                                                    CFDX Frequency Plan
                                                Upstream Reference PSD vs Frequency
                    PSD
                                                                          43 dBmV/1.6MHz




 38.0 dBmV/1.6MHz
                                                      33 dBmV/1.6MHz




                                                                                                                 MHz
                      5         85        108                                                    684


                      Ptotal_leg =                             Ptotal_FDX = 64.5 dBmV
                       55.0 dBmV




                                                    SFDX Frequency Plan
                                                Upstream Reference PSD vs Frequency
                    PSD

                                                                                      42.25 dBmV/1.6MHz



 38.0 dBmV/1.6MHz

                                                                    32.25 dBmV/1.6MHz



                                                                                                                 MHz
                      5                     204          258                                              834


                           Ptotal_leg =                                     Ptotal_FDX = 63.75 dBmV
                            59.0 dBmV

                    Figure 9 - Upstream Reference PSD for CFDX and SFDX Frequency Plans
At Interface F, the dynamic range window (DRW_FDX) is defined, which is the result of adjustment relative to the
upstream reference PSD, lowered by the amount commanded by the CMTS. The DRW_FDX is managed in the
FDX spectrum in a similar manner as the DRW is managed in the legacy DOCSIS 3.1 upstream spectrum, but
DRW_FDX and DRW are different. DOCSIS 3.1 CMs are still managed with the DOCSIS 3.1 DRW. DRW_FDX
does not apply to DOCSIS 3.1 CMs.
Channel power adjustments within the FDX spectrum are managed similarly to the channel power adjustments in
the legacy DOCSIS 3.1 upstream spectrum, which adjusts each individual channel PSD at or below the top of the
DRW_FDX, and can also adjust an individual channel PSD above the top of the DRW_FDX a small amount.
Figure 8 illustrates the measurement for channel reported power accuracy and MER and spurious emissions for
signals in the FDX spectrum. The test setup defining the measurements provides that a signal with the upstream
reference PSD at Interface F has a flat PSD at Interface Fˊ.




56                                                      CableLabs                                         06/27/25
<!-- PDF_PAGE: 57 -->
Physical Layer Specification                                                        CM-SP-PHYv4.0-I07-250627


7.2.4.12.2 FDX Upstream Reference PSD
The channel reported power for an FDX CM for channels in the FDX spectrum are reported relative to the upstream
reference PSD. The channel commanded power (per 1.6 MHz) for a channel at Interface F is, by definition, the
channel power adjustments up or down from the upstream reference PSD.
The "equivalent channel power" of an FDX OFDMA channel is the average power at Interface Fˊ of the OFDMA
subcarriers of the channel normalized to 1.6 MHz bandwidth and referenced to the upstream reference PSD at
Interface Fˊ. This equivalent channel power of an OFDMA channel is denoted as P1.6r_n-FDX. The TCS_FDX has zero
to six OFDMA channels, but also is described as having Neq-FDX number of equivalent DOCSIS channels.
Each channel in the TCS_FDX is described by its reported power P1.6r_n-FDX, which is the channel power when it is
fully granted and normalized to 1.6 MHz. The relation of the reported power to the expected true power of a fully
granted channel is a function of the number of active subcarriers in the channel and their frequency. The reported
power for each channel is referenced to Interface Fˊ to simplify the upstream power management at the CMTS,
which is generally expected to operate with a flat received PSD.
For an FDX CM, Pref-FDX is a parameter which is a function of frequency and is the power in dBmV in 1.6 MHz of
subcarriers with no pre-equalization. Pref-FDX has a slope of (10 dB/360 slots of 1.6 MHz in the FDX band)
dBmV/1.6 MHz, and a total variation of 10.0 dB across the FDX band. For the CFDX frequency plan (108 MHz to
684 MHz), Pref-FDX (108.8 MHz) = 33.0 dBmV/1.6 MHz and Pref-FDX (683.2 MHz) = 43.0 dBmV/1.6 MHz,
corresponding to 64.5 dBmV total average power in the FDX band with 570 MHz modulated spectrum in the
TCS_FDX (Neq-FDX = 357). For the SFDX frequency plan, Pref-FDX (258.8 MHz) = 32.25 dBmV/1.6 MHz and Pref-FDX
(833.2 MHz) = 42.25 dBmV/1.6 MHz, corresponding to 63.75 dBmV total average power in the FDX band with
570 MHz modulated spectrum in the TCS_FDX (Neq-FDX = 357). Note that there are no upstream SC-QAM channels
in the FDX band.
Fidelity requirements apply with Pref-FDX for the transmit power spectral density, with 10 dB uptilt and no other
channel adjustments or pre-equalization. Fidelity requirements also apply with 8 dB uptilt (and no other channel
adjustments or pre-equalization beyond accomplishing the slope). Fidelity requirements with 1 dB additional
allowance for spurious emissions, MER and inband (compared to the 10 dB uptilt case) apply with 12 dB uptilt (and
no other channel adjustments or pre-equalization beyond accomplishing the slope).
Plimit-FDX is the maximum power per channel, increased from Pref-FDX, for which only gradual degradation of fidelity
requirements may be expected. Plimit-FDX is 1.5 dB for upstream channels with no modulated spectrum above
300 MHz for CFDX or 450 MHz for SFDX. Plimit-FDX is 1 dB for channels with any modulated spectrum above
300 MHz for CFDX or 450 MHz for SFDX. Note that although some channels may be commanded above Pref-FDX,
and fidelity requirements may still apply, if the total average power in the FDX band exceeds PmaxFDX, then fidelity
requirements do not apply.
The maximum power per any individual subcarrier, increased from Pref-FDX, for which gradual degradation of fidelity
requirements may be expected are given as follows: with any subcarrier power commanded more than 3 dB higher
than Pref-FDX, in any channel which has no modulated spectrum above 300 MHz for CFDX or 450 MHz for SFDX;
with any subcarrier power commanded more than 2.5 dB higher than Pref-FDX, in any channel, which, for CFDX, has
no modulated spectrum above 492 MHz but some modulated spectrum above 300 MHz, or for SFDX, has no
modulated spectrum above 642 MHz but some modulated spectrum above 450 MHz; or with subcarrier power
commanded more than 2.0 dB higher than Pref-FDX, in any channel which has some modulated spectrum above
492 MHz for CFDX or 642 MHz for SFDX. Note that although some subcarriers may be commanded above Pref-FDX,
and fidelity requirements may still apply, if the total average power in the FDX band exceeds PmaxFDX then fidelity
requirements do not apply.

7.2.4.12.3 Maximum Scheduled Minislots
Maximum scheduled minislots are not supported for FDX CMs operating in FDX mode.

7.2.4.12.4 Transmit Power Requirements
The transmit power is a function of the number and occupied bandwidth of the OFDMA channels in the TCS_FDX,
or equivalently the amount of TCS_FDX modulated spectrum and the frequency of the modulated spectrum. The
highest value of the total power output in the FDX spectrum of the CM, PmaxFDX, is 64.5 dBmV for CFDX operation



06/27/25                                           CableLabs                                                     57
<!-- PDF_PAGE: 58 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


or 63.75 dBmV for SFDX operation, and occurs when all the CM's potential FDX spectrum is occupied with
OFDMA channels in its TCS_FDX and these are fully granted to the CM, and all channels are commanded to 0 dB
channel power. The upstream reference PSD is denoted as Pref-FDX and is a function of frequency as defined in
section 7.2.4.12.2. When the TCS_FDX occupies a subset of the potential FDX spectrum, the total average power in
the FDX band is reduced from PmaxFDX. The top of the dynamic range window of the OFDMA FDX spectrum can be
reduced from Pref-FDX by adjusting Pload_min_set-FDX to a value greater than 0 dB. The OFDMA FDX spectrum's nth
channel power can be raised by a small amount (up to 1.5 dB in some portion of the FDX band) when the top of the
DRW_FDX is at Pref-FDX, or reduced for any setting of the DRW_FDX. For example, the nth channel power can be
reduced by setting its P1.6r_n-FDX below 0 dB (negative), thereby reducing all the channel subcarriers from their power
that was based on Pref-FDX and P1.6r_n-FDX = 0 dB. There are limits on the amount of reduction as described below.
This adjustability in channel power ensures that each channel can be set to a power range (within the DRW_FDX)
between its maximum power, Pref-FDX – Pload_min_set-FDX, or up to 1.5 dB higher in some portion of the FDX band when
Pload_min_set-FDX = 0, and minimum power, P1.6low-FDX, and that any possible transmit grant combination can be
accommodated without exceeding the transmit power capability of the CM.
For full duplex, P1.6low-FDX = P1.6min-FDX = -15 dB. Boosted pilots are not supported in the full duplex channels. Before
completion of fine ranging, the FDX CM has no need to transmit with power per subcarrier which is lower than
indicated by P1.6low-FDX. These transmissions are prior to any data grant transmissions in the FDX band from the CM
and as such the CM analog and digital gain balancing may be optimized for these transmissions.
When operating with the CFDX frequency plan, when Pload_min_set-FDX is 0 dB, the FDX CMTS SHOULD NOT
command the FDX CM to set P1.6r_n-FDX on any channel in the TCS_FDX between 108 MHz and 300 MHz to a value
more than 1.5 dB above the top of the DRW_FDX, or any channel in the TCS_FDX above 300 MHz, to a value
more than 1 dB above the top of the DRW_FDX, or lower than the bottom of the DRW_FDX.
When operating with the SFDX frequency plan, when Pload_min_set-FDX is 0 dB, the FDX CMTS SHOULD NOT
command the FDX CM to set P1.6r_n-FDX on any channel in the TCS_FDX between 258 MHz and 450 MHz to a value
more than 1.5 dB above the top of the DRW_FDX, or any channel in the TCS_FDX above 450 MHz, to a value
more than 1 dB above the top of the DRW_FDX, or lower than the bottom of the DRW_FDX.
When Pload_min_set-FDX is greater than 0 dB, the FDX CMTS SHOULD NOT command the FDX CM to set P1.6r_n-FDX
on any channel in the TCS_FDX to a value more than 0 dB above the top of the DRW_FDX, or lower than the
bottom of the DRW_FDX.
If the FDX CMTS commands the FDX CM to exceed the top of the DRW_FDX, fidelity and performance
requirements on the FDX CM do not apply, except for the following two narrow cases; with the 8 dB uptilt case
spurious emissions and MER requirements are the same as with the 10 dB uptilt specified case; and with the 12 dB
uptilt receiving 1 dB relaxation for spurious emissions and MER:
     •   8 dB uptilt, PmaxFDX total average power in the FDX band: 1.3 dB higher at the lowest FDX band frequency
         (for CFDX, 34.3 dBmV/1.6 MHz) at 108.8 MHz, or for SFDX, 33.55 dBmV/1.6 MHz at 258.8 MHz) and
         0.7 dB lower at the highest FDX band frequency (for CFDX, 42.3 dBmV/1.6 MHz at 683.2 MHz, or for
         SFDX, 41.55 dBmV/1.6 MHz at 833.2 MHz).
     •   12 dB uptilt, PmaxFDX total average power in the FDX band: 1.4 dB lower at the lowest FDX band frequency
         (for CFDX, 31.6 dBmV/1.6 MHz at 108.8 MHz, or for SFDX, 30.85 dBmV/1.6 MHz at 258.8 MHz) and
         0.6 dB higher at the highest FDX band frequency (for CFDX, 43.6 dBmV/1.6 MHz at 683.2 MHz, or for
         SFDX, 42.85 dBmV/1.6 MHz at 833.2 MHz).
Table 8 summarizes the FDX upstream reference PSD for both CFDX and SFDX for different amounts of spectral
tilt, as shown in Figure 8.
                                       Table 8 - Reference PSD and Tilt (FDX)

     Upstream Center        Spectral Tilt          y                  y+z                 y                  y+z
       Frequency               (dB)           (108.8 MHz)         (683.2 MHz)        (258.8 MHz)         (833.8 MHz)
     Frequency Plan                               CFDX                CFDX               SFDX                SFDX
 Upstream reference PSD            8               34.3                42.3               33.55              41.55
 (dBmV/1.6 MHz)




58                                                   CableLabs                                               06/27/25
<!-- PDF_PAGE: 59 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


    Upstream Center         Spectral Tilt          y                  y+z                 y                  y+z
      Frequency                (dB)           (108.8 MHz)         (683.2 MHz)        (258.8 MHz)         (833.8 MHz)
     Frequency Plan                               CFDX               CFDX                SFDX                SFDX
 Upstream reference PSD           10                33                 43                32.25               42.25
 (dBmV/1.6 MHz)
 Upstream reference PSD           12               31.6                43.6              30.85               42.85
 (dBmV/1.6 MHz)


If Pload_min_set-FDX is more than 0 dB, and the FDX CM is commanded to transmit on any channel in the TCS_FDX at
a value higher than the top of the DRW_FDX or lower than the bottom of the DRW_FDX, the cable modem
indicates an error condition by setting the appropriate bit in the SID field of RNG-REQ messages for that channel
until the error condition is cleared [DOCSIS MULPIv4.0].
If Pload_min_set-FDX is 0 dB, and the FDX CM is commanded to transmit on any channel in the TCS_FDX at a value
more than 1.5 dB higher than the top of the DRW_FDX for channels in the lowest 192 MHz of the FDX band
(108 MHz to 300 MHz for CFDX or 258 to 450 MHz for SFDX), or more than 1 dB higher than the top of the
DRW_FDX for channels more than 192 MHz above the lowest FDX band frequency (higher than 300 MHz for
CFDX or higher than 450 MHz for SFDX), or lower than the bottom of the DRW_FDX, the cable modem indicates
an error condition by setting the appropriate bit in the SID field of RNG-REQ messages for that channel until the
error condition is cleared [DOCSIS MULPIv4.0].
The FDX CMTS sends transmit power level commands and pre-equalizer coefficients to the FDX CM [DOCSIS
MULPIv4.0] to compensate for upstream plant conditions. The top edge of the DRW_FDX is set to a level,
P­1.6load_min_set-FDX, close to the highest P1.6r_n-FDX transmit channel to optimally load the DAC. In conditions of tilt
significantly different than the nominal 10 dB tilt, some of the channels will be sent commands to transmit at lower
P1.6r_n-FDX values that use up a significant portion of the DRW_FDX, and perhaps exceed the top of the DRW_FDX.
Additionally, the pre-equalizer coefficients of the OFDMA channels will also compensate for plant tilts away from
the nominal 10 dB. The FDX CMTS normally administers a DRW_FDX of 10 dB [DOCSIS MULPIv4.0] which is
sufficient to accommodate plant tilts of up to +2 dB to – 2 dB different from the specified tilt cases, 8 dB and 12 dB,
and to accommodate plant flatness variations and loss variations as a function of frequency. Since the fidelity
requirements are specified in flat frequency conditions at Interface Fˊ relative to the top of the DRW_FDX, it is
desirable to maintain FDX CM transmission power levels as close to the top of the DRW_FDX as possible. When
conditions change sufficiently to warrant it, a global reconfiguration time should be granted and the top of the
DRW_FDX adjusted to maintain the best transmission fidelity and optimize system performance.

7.2.4.12.4.1 FDX Transmit Power Detailed Requirements
For FDX CMs operating in FDX mode, multiple transmit channel mode is always enabled.
The FDX CM MUST support varying the amount of transmit power.
Requirements are presented for 1) range of reported transmit power per channel; 2) step size of power commands;
3) step size accuracy (actual change in output power per channel compared to commanded change); and 4) absolute
accuracy of CM output power per channel. The protocol by which power adjustments are performed is defined in
[DOCSIS MULPIv4.0]. The FDX CM MUST support such adjustments, commanded by the FDX CMTS, within the
ranges of tolerances described below.
An FDX CM MUST confirm that the transmit power per channel limits are met after a RNG-RSP is received for
each of the CM's active channels that is referenced and indicate that an error has occurred in the next RNG-REQ
messages for the channel until the error condition is cleared [DOCSIS MULPIv4.0].
Some time after registration the FDX CMTS can initialize FDX operations for an FDX CM. During this process, the
CM is assigned to a TG and receives FDX channel assignment via the dynamic bonding request (DBC) mechanism.
The group of active channels in the FDX band assigned to a CM is known as the CM's full duplex transmit channel
set (TCS_FDX). If the FDX CMTS needs to add, remove, or replace channels in the CM's TCS_FDX, it uses the
DBC-REQ message with transmit channel configuration encodings to define the new desired TCS_FDX. The set of
channels actually bursting upstream from an FDX CM at any time could be all or a subset of the active channels on




06/27/25                                             CableLabs                                                      59
<!-- PDF_PAGE: 60 -->
CM-SP-PHYv4.0-I07-250627                                             Data-Over-Cable Service Interface Specifications


that FDX CM. Often one or all active channels on an FDX CM will not be bursting, but such quiet channels are still
active channels for that FDX CM.
Transmit power per channel is defined as the average RF power in the occupied bandwidth (channel width),
assuming equally likely QAM symbols, relative to the upstream reference PSD, measured at Interface Fˊ of Figure 8
as detailed below. Reported transmit power for an OFDMA channel is expressed as P1.6r_n-FDX and is defined as the
average RF power of the FDX CM transmission in the OFDMA channel, relative to the upstream reference PSD at
Interface Fˊ when transmitting in a grant composed of 64 25 kHz subcarriers or 32 50 kHz subcarriers. Total
transmit power is defined as the sum of the transmit power per channel of each channel transmitting a burst at a
given time.
The FDX CM MUST maintain its actual transmitted power per equivalent channel to within ± 2 dB of the reported
power, P1.6r_n-FDX, with pre-equalization off, taking into account symbol constellation values.
The FDX CM MUST allow its target transmit power per channel to vary over the range specified in section
7.2.4.12.4, Transmit Power Requirements. The fidelity requirements do not apply when the FDX CM is commanded
to transmit at power levels which exceed the top of the DRW_FDX, except for the two narrow cases 8 dB uptilt and
12 dB uptilt.
The transmit channel loading P1.6load-FDX describes how close the transmit power level for a particular channel is to
the top of the DRW_FDX. Let P1.6load-FDX = Pref-FDX - P1.6r_n-FDX, for each channel, using the definitions for Pref-FDX
and P1.6r_n-FDX in the following subsections of Section 7.2.4.12. The channel corresponding to the minimum value of
P1.6load-FDX is called the highest loaded channel, and its value is denoted as P1.6load_1-FDX, in this specification even if
there is only one channel in the full duplex transmit channel set (TCS_FDX). A channel with high loading has a low
P1.6load_i-FDX value; the value of P1.6load_n-FDX is analogous to an amount of back-off for an amplifier from its max
power output, except that it is normalized to 1.6 MHz of bandwidth. A channel has lower power output when that
channel has a lower loading (more back-off) and thus a higher value of P1.6load_i-FDX. Note that the highest loaded
channel is not necessarily the channel with the highest transmit power at Interface Fˊ in Figure 8 since a channel's
max power at Interface Fˊ depends on the bandwidth of the channel. The channel with the second lowest value of
P1.6load-FDX is denoted as the second highest loaded channel, and its loading value is denoted as P1.6load_2-FDX; the
channel with the ith lowest value of P1.6load-FDX is the ith highest loaded channel, and its loading value is denoted as
P1.6load_i-FDX.
P1.6load_min_set-FDX defines the upper end of the DRW_FDX for the FDX CM with respect to Pref-FDX. P1.6load_min_set-FDX
will normally limit the maximum power possible for each active channel to a value less than Pref-FDX, but a
commanded power adjustment can result in a violation of the DRW_FDX, in which case the FDX CM compliance
with the fidelity requirements is not enforced, with two narrow exceptions for 8 dB uptilt and 12 dB uptilt described
in the previous section. P1.6load_min_set_FDX is a value commanded to the FDX CM from the FDX CMTS when the FDX
CM is given a TCC in registration and RNG-RSP messages after Registration [DOCSIS MULPIv4.0]. P1.6load_min_set-
FDX, P1.6load_ n-FDX, Pref-FDX, Plimit-FDX, P1.6 r_n-FDX, etc., are defined for DOCSIS FDX modems operating on a DOCSIS
FDX CMTS. See Section 7.2.4.12.4 for a summary of these and other terms related to transmit power.
The FDX CMTS SHOULD command the FDX CM to use a value for P1.6load_min_set-FDX such that Pref-FDX –
P1.6load_min_set-FDX ≥ P1.6low _n-FDX for each active channel, with allowance for higher channel power up to Plimit-FDX in
some channels, as long as PmaxFDX is maintained (to support different uptilt than 10 dB), or equivalently:
                                       0 ≤ P1.6load_min_set-FDX ≤ Pref-FDX – P1.6low_n-FDX
A value is computed, P1.6 low_multi-FDX, which sets the lower end of the transmit power DRW_FDX for that channel,
given the upper end of the range which is determined by P1.6load_min_set-FDX.
                       P1.6low_multi-FDX = max{Pref-FDX – P1.6load_min_set-FDX - 10 dB, Pref-FDX – 15 dB}
The effect of P1.6low_multi-FDX is to restrict the dynamic range required (or even allowed) by a FDX CM across its
multiple channels, when operating with multiple active channels.
The FDX CMTS SHOULD command a P1.6r _n-FDX consistent with the P1.6load_min_set-FDX assigned to the FDX CM and
with the following limits (with allowance up to Plimit-FDX rather than Pref-FDX to accommodate different uptilt than 10
dB):
                            P1.6load_min_set-FDX ≤ Pref-FDX – P1.6r_n-FDX ≤ P1.6load_min_set-FDX + 10 dB



60                                                       CableLabs                                               06/27/25
<!-- PDF_PAGE: 61 -->
Physical Layer Specification                                                                    CM-SP-PHYv4.0-I07-250627


and the equivalent:
                      Pref-FDX – (P1.6load_min_set-FDX + 10 dB) ≤ P1.6r _n-FDX ≤ Pref-FDX – P1.6load_min_set-FDX
When the FDX CMTS sends a new value of P1.6load_min_set-FDX to the FDX CM, there is a possibility that the FDX CM
will not be able to implement the change to the new value immediately, because the FDX CM may be in the middle
of bursting on one or more of its upstream channels at the instant the command to change P1.6load_min_set-FDX is
received at the FDX CM. Some amount of time may elapse before the FDX CMTS grants global reconfiguration
time to the FDX CM. Similarly, commanded changes to P1.6r_n-FDX may not be implemented immediately upon
reception at the FDX CM if the nth channel is bursting. Commanded changes to P1.6r_n-FDX may occur simultaneously
with the command to change P1.6load_min_set-FDX.
The FDX CMTS SHOULD NOT issue a change in P1.6load_min_set-FDX after commanding a change in P1.6r_n-FDX until
after also providing a sufficient reconfiguration time on the nth channel. The FDX CMTS SHOULD NOT issue a
change in P1.6load_min_set-FDX after commanding a prior change in P1.6load_min_set-FDX until after also providing a global
reconfiguration time for the first command.
Also, the FDX CMTS SHOULD NOT issue a change in P1.6r_n-FDX until after providing a global reconfiguration time
following a command for a new value of P1.6load_min_set-FDX and until after providing a sufficient reconfiguration time
on the nth channel after issuing a previous change in P1.6r_n-FDX. In other words, the FDX CMTS is to avoid sending
consecutive changes in P1.6r_n-FDX and/or P1.6load_min_set-FDX to the FDX CM without a sufficient reconfiguration time
for instituting the first command.
When a concurrent new value of P1.6load_min_set-FDX and change in P1.6r_n-FDX are commanded, the FDX CM MAY wait
to apply the change in P1.6r_n-FDX at the next global reconfiguration time (i.e., concurrent with the institution of the
new value of P1.6load_min_set-FDX) rather than applying the change at the first sufficient reconfiguration time of the nth
channel. The value of P1.6load_min_set-FDX which applies to the new P1.6r_n-FDX is the concurrently commanded
P1.6load_min_set-FDX value.
If the change to P1.6r_n-FDX falls outside the DRW_FDX of the old P1.6load_min_set-FDX, then the FDX CM MUST wait
for the global reconfiguration time to apply the change in P1.6r_n-FDX.
The FDX CMTS SHOULD NOT command the FDX CM to decrease the per-channel transmit power if such a
command would cause P1.6load_ n-FDX for that channel to drop below P1.6load_min_set-FDX.
Note that the FDX CMTS can allow small changes of power in the FDX CM's highest loaded channel without these
fluctuations impacting the transmit power dynamic range with each such small change. This is accomplished by
setting P1.6load_min_set-FDX to a smaller value than normal, and fluctuation of the power per channel in the highest
loaded channel is expected to wander.
The FDX CMTS MUST NOT command a change of per channel transmit power or pre-equalization which could
result in exceeding the CM's PmaxFDX in the FDX band. If the FDX CMTS improperly commands the FDX CM to
exceed PmaxFDX, the FDX CM informs the FDX CMTS of the error as described in [DOCSIS MULPIv4.0].
The FDX CMTS SHOULD NOT command a change of per channel transmit power which would result in P1.6r_n-FDX
falling below the DRW_FDX, P1.6r_n-FDX < P1.6 low_multi-FDX.
The FDX CMTS SHOULD NOT command a change in P1.6load_min_set-FDX such that existing values of P1.6r_n-FDX
would fall outside the new DRW_FDX.
The following paragraphs define the FDX CM and FDX CMTS behavior in cases where there are DRW_FDX
violations due to addition of a new channel with incompatible parameters without direct change of P1.6r_n-FDX or
P1.6load_min_set-FDX.
When adding a new active channel to the transmit channel set, the new channel's power is calculated according to
the offset value defined in TLV 46.8.4 [DOCSIS MULPIv4.0], if it is provided.
The FDX CMTS SHOULD NOT set an offset value that will result in a P1.6r_n-FDX for the new channel outside the
DRW_FDX. In the absence of the TLV, the new channel's power is initially set by the FDX CM at the minimum
allowable power, i.e., the bottom of the DRW_FDX.
The FDX CM MUST maintain its actual transmitted power per every minislot within a burst constant to within 0.1
dB peak to valley even in the presence of power changes on other active channels. The 0.1 dB peak to valley does



06/27/25                                                  CableLabs                                                     61
<!-- PDF_PAGE: 62 -->
CM-SP-PHYv4.0-I07-250627                                          Data-Over-Cable Service Interface Specifications


not include amplitude variation theoretically present in the signal (e.g., varying QAM constellations, transmit
window). Specifically, within a continuous burst of duration up to n frames (1 millisecond), for each minislot
participating in the burst and while the minislot is actively used for transmission, a constant power has to be
maintained in that minislot within 0.1 dB peak to valley, even in the presence of a transmission starting or stopping
on other minislots and other active channels.
The FDX CM MUST support the transmit power calculations defined in section 7.2.4.12.5.

7.2.4.12.5 Transmit Power Calculations
The FDX CM determines its target transmit power per channel P1.6t_n-FDX, as follows, for each channel which is
active. Define for each active channel, for example, upstream channel n:P1.6c_n-FDX = commanded power for channel
n. (TLV-17 in RNG-RSP).
     •    P1.6r_n-FDX = reported power level (dBmV) of the FDX CM for channel n
     •    Plimit_FDX = 1.5 dB for channels with no modulated spectrum above 300 MHz for CFDX, or 450 MHz for
          SFDX, and 1.0 dB for channels with any modulated spectrum above 300 MHz for CFDX, or 450 MHz for
          SFDX
The FDX CM updates its reported power per channel in each channel by the following steps:
     1.   ΔP = P1.6c_n-FDX - P1.6r_n-FDX
     2.   P1.6r_n-FDX = P1.6r_n-FDX + ΔP //Add power level adjustment (for each channel) to reported power level for
          each channel.
The FDX CMTS SHOULD ensure the following:
     1.   P1.6r_n-FDX ≤ ;Plimit-FDX, when P1.6load_min_set-FDX = 0 //Clip at max power limit per channel unless the FDX
          CMTS accommodates a need to increase the PSD for the channel in which case the fidelity performance of
          the FDX CM is potentially degraded.
     2.   P1.6r_n-FDX ≥ P1.6low_n-FDX //Clip at min power limit per channel.
     3.   P1.6r_n-FDX ≥ P1.6low_multi-FDX //Power per channel from this command would violate the set DRW-FDX.
     4.   P1.6r _n-FDX ≤ - P1.6load_min_set-FDX, when P1.6load_min_set-FDX > 0 //Power per channel from this command violates
          the set DRW_FDX, but the FDX CMTS could accommodate a need to increase the PSD for the channel in
          which case the fidelity performance of the FDX CM is potentially degraded.
For OFDMA, the CM then transmits each data subcarrier with target power:
                   Pt_sc_i = P1.6r_n-FDX + Pre-Eqi - 10log(number_of subcarriers in 1.6 MHz {32 or 64})
where Pre-Eqi is the magnitude of the ith subcarrier pre-equalizer coefficient (dB).
That is, the reported power for channel n, normalized to 1.6 MHz, plus the pre-equalization for the subcarrier, less a
factor taking into account the number of subcarriers in 1.6 MHz.
Probedelta_n-FDX for the nth FDX OFDMA channel is the change in subcarrier power for probes compared to subcarrier
power for data depending on the mode as defined in [DOCSIS MULPIv4.0] in addition to pre-equalization on or off.
The FDX CM transmits probes with the same target power as given above plus Probedelta_n-FDX when pre-equalization
is enabled for probes in the P-MAP which provides the probe opportunity:
          Pt_sc_i = P1.6r_n-FDX + Probedelta_n-FDX + Pre-Eqi - 10log(number_of subcarriers in 1.6 MHz {32 or 64})
When the pre-equalization is disabled for the probe opportunity in the P-MAP, the FDX CM then transmits probe
subcarrier with target power:
              Pt_sc_i = P1.6r_n-FDX + Probedelta_n-FDX - 10log10(number_of subcarriers in 1.6 MHz {32 or 64})
That is, the reported power for channel n, normalized to 1.6 MHz, less a factor taking into account the number of
subcarriers in 1.6 MHz.




62                                                     CableLabs                                                 06/27/25
<!-- PDF_PAGE: 63 -->
Physical Layer Specification                                                                       CM-SP-PHYv4.0-I07-250627


The total transmit power in channel n, Pt_n, in a frame is the sum of the individual transmit powers Pt_sc_i of each
subcarrier in channel n, where the sum is performed using absolute power quantities (non-dB domain).
The transmitted power level in channel n varies dynamically as the number and type of allocated subcarriers varies.

7.2.4.12.5.1 Terminology Used in Sections Covering FDX Upstream Transmit Power Requirements
This section provides a brief description of the terms used in elaboration of the transmit power requirements.
BWOFDMA-FDX            The combined occupied bandwidth of the full duplex OFDMA channel(s) in the FDX band transmit channel set
                       (TCS_FDX).
DRW_FDX                The dynamic range window of a full duplex channel.
Neq-FDX                Number of equivalent DOCSIS 1.6 MHz upstream channels in the cable modem's FDX band transmit channel
                       set (TCS_FDX). Neq= ceil(BWOFDMA (MHz)/1.6 MHz)
P1.6c_n-FDX            Commanded power for full duplex channel n. (TLV-17 in RNG-RSP)
P1.6load_i-FDX         The transmit channel loading P1.6load_i-FDX. This describes how close the transmit power level for a particular
                       channel is to the top of the DRW_FDX. The highest loaded full duplex channel P1.6load_1-FDX is the channel for
                       which the reported power P1.6r_n-FDX is closest to the top of the DRW_FDX. In the case where there are j
                       channels in the TCS_FDX, the lowest loaded channel P1.6load_j-FDX is the full duplex channel whose reported
                       power P1.6r_n-FDX is furthest from the top of the DRW_FDX.
P1.6load_min_set-FDX   The number of dB below Pref-FDX which defines the top of the DRW_FDX. The top of the dynamic range window
                       of the OFDMA FDX spectrum can be reduced from Pref-FDX by adjusting Pload_min_set-FDX to a value greater than
                       0 dB.
P1.6low-FDX            Minimum transmit power to which a CM can be configured to transmit in the FDX band. OFDMA channels in
                       the FDX band do not have boosted pilots so P1.6low-FDX = P1.6Min-FDX.
P1.6low_multi-FDX      Bottom of DRW_FDX.
P1.6low_n-FDX          The minimum equivalent channel power for a particular FDX channel that the CM is permitted to support.
P1.6min-FDX            Minimum transmit power to which a CM can be configured to transmit in the FDX band. OFDMA channels in
                       the FDX band do not have boosted pilots so P1.6low-FDX = P1.6Min-FDX. P1.6min-FDX = -15 dB.
P1.6r_n-FDX            The equivalent channel power of an FDX OFDMA channel "n". P1.6r_n-FDX is the average power at Interface Fˊ of
                       the OFDMA subcarriers of the channel normalized to 1.6 MHz bandwidth and referenced to the upstream
                       reference power spectral density at Interface Fˊ when the channel is fully granted. P1.6r_n-FDX is the channel
                       power reported in the RNG-REQ messages. This is also referred to in the specification as reported transmit
                       power.
Plimit-FDX             The maximum power per channel, increased from Pref-FDX for which fidelity requirements only degrade
                       gradually, when P1.6load_min_set-FDX = 0. Fidelity requirements do not apply for channel power commanded higher
                       than Plimit-FDX. Fidelity requirements do not apply whenever total average power in the FDX band exceeds
                       PmaxFDX.
PmaxFDX                The maximum total transmit power that the CM can support in the FDX band. The default value and the lowest
                       allowable value for PmaxFDX is 64.5 dBmV when operating with the CFDX frequency plan, or 63.75 dBmV when
                       operating with the SFDX frequency plan. Fidelity requirements do not apply if total average power in the FDX
                       band exceeds PmaxFDX.
Pref-FDX               The upstream reference power spectral density in the full duplex band. Pref-FDX is the power in dBmV in 1.6 MHz
                       of subcarriers with no pre-equalization (see section 7.2.4.12.2 for details of Pref-FDX).
Pt_n-FDX               The total transmit power in a channel n in the FDX band.
Pt_sc_i                The average target power transmitted by the ith subcarrier for either probes or other transmissions, possibly
                       with different power for the probe transmissions (see Probedelta_n-FDX below).
Pre-Eqi                The magnitude of the ith subcarrier pre-equalizer coefficient (dB).
Probedelta_n-FDX       This term is used to account for reduction in probe power resulting from the power bit and Start Subc bits in the
                       probe information element in the P-MAP for the nth FDX OFDMA channel.
TCS_FDX                A cable modem's transmit channel set in the FDX band (108 MHz to 684 MHz for CFDX or 258 to 834 MHz for
                       SFDX).


7.2.4.12.6 Reconfiguration Time for FDX CMs
In an FDX DOCSIS system, there are two independent transmission channel sets: one for the legacy upstream
channels (TCS), and one for the FDX upstream channels (TCS_FDX).
Section 7.2.4.12.4 of [DOCSIS PHYv3.1] applies to TCS only, while this section applies to TCS_FDX.




06/27/25                                                      CableLabs                                                                 63
<!-- PDF_PAGE: 64 -->
CM-SP-PHYv4.0-I07-250627                                     Data-Over-Cable Service Interface Specifications


Reconfiguration time for FDX upstream channels is the inactive time interval provided between active upstream
transmissions on a given FDX upstream channel when a change is commanded for a transmission parameter on that
channel. For changes in the ranging offset and/or pre-equalization of an FDX upstream channel, the FDX CM
MUST be able to transmit consecutive bursts as long as the CMTS allocates the time duration (reconfiguration time)
of at least one inactive frame in between the bursts on the FDX upstream channel with the changed parameter.
"Global reconfiguration time" in the FDX upstream channels is defined as the inactive time interval provided
between active FDX upstream transmissions, which simultaneously satisfies the requirement in this section for all
OFDMA channels in the TCS_FDX.
Global "quiet" across all active FDX upstream channels requires the intersection of ungranted burst intervals across
all active OFDMA FDX channels to be at least 20 microseconds. Even with a change or re-command of
P1.6load_min_set-FDX, the FDX CM MUST be able to transmit consecutive bursts as long as the FDX CMTS allocates at
least one frame in between bursts, across all OFDMA channels in the TCS_FDX, where the quiet lapses in each
channel contain an intersection of at least 20 microseconds. (From the end of a burst on one FDX upstream channel
to the beginning of the next burst on any other FDX upstream channel, there is to be at least 20 microseconds
duration to provide a "global reconfiguration time" for all channels in the FDX CM's TCS_FDX.)
The FDX CMTS SHOULD provide global reconfiguration time to the TCS_FDX for the FDX CM before (or
concurrently as) the FDX CM has been commanded to change any upstream channel transmit power in the
TCS_FDX by ±3 dB cumulative since its last global reconfiguration time.
Global reconfiguration time for the legacy upstream channels (TCS) is completely disassociated with TCS_FDX
grants or commands to the FDX CM, and global reconfiguration time for the FDX upstream channels (TCS_FDX) is
completely disassociated with the TCS grants or commands to the FDX CM.
A resource block allocation change does not require a reconfiguration time. Imposed "quiet" time (no grants) on
FDX upstream channels with a status change indicated by a resource block allocation is described in [DOCSIS
MULPIv4.0]. No "quiet" time is required on an FDX upstream channel with a resource block allocation change
which maintains the upstream status of the FDX channel.

7.2.4.12.7 Fidelity Requirements for FDX
The following requirements assume that any pre-equalization is disabled, unless otherwise noted. Signal power and
measurements are all referenced to Interface Fˊ of Figure 8.
When channels in the TCS_FDX are commanded to the same equivalent channel powers, the reference signal power
in the "dBc" definition is to be interpreted as the measured average total transmitted power at Interface Fˊ. When
channels in the TCS_FDX are commanded to different equivalent channel powers, the commanded total power of
the transmission is computed, and a difference is derived compared to the commanded total power which would
occur if all channels had the same P1.6r_n-FDX as the highest equivalent channel power in the TCS_FDX, whether or
not the channel with the largest equivalent channel power is included in the grant. Then this difference is added to
the measured total transmit power to form the reference signal power for the "dBc" spurious emissions requirements.

7.2.4.12.7.1 Spurious Emissions for FDX
A Unified CM operating in FDX mode in a cable system capable of downstream operation above 1218 MHz MUST
meet the requirements of 7.3.4.12.7.1for CM spurious emissions from 1218–1794 MHz, with "FDX" replacing
"FDD" in the terminology of that section.
The noise and spurious power generated by the FDX CM MUST NOT exceed the levels given in Table 9 - Spurious
Emissions, Table 10 - Spurious Emissions Requirements in the Upstream Frequency Range for Grants of Under-
Grant Hold Bandwidth and Larger, and section 7.2.4.12.7.1.2, Adjacent Channel Spurious Emissions.
Up to five discrete spurs can be excluded from the emissions requirements listed in Table 9 and Table 10 for each of
five spectral regions, while the CM is transmitting a burst upstream in the FDX band. For the CFDX frequency plan,
the five spectral regions are 85 MHz to 300 MHz, 300 MHz to 492 MHz, 492 MHz to 684 MHz, 684 MHz to
804 MHz, and 804 MHz to 1218 MHz. For the SFDX frequency plan, the five spectral regions are 204 MHz to
450 MHz, 450 MHz to 642 MHz, 642 MHz to 834 MHz, 834 MHz to 984 MHz, and 984 MHz to 1218 MHz
(spurious emissions from 1218–1794 MHz are addressed in section 7.3.4.12.7.1, as referenced above). The five



64                                                 CableLabs                                             06/27/25
<!-- PDF_PAGE: 65 -->
Physical Layer Specification                                                             CM-SP-PHYv4.0-I07-250627


excluded discrete spurs have to be no more than 2 dB in excess of the MER value of Table 12, with 100% grant,
relative to a single subcarrier power level at the top of the DRW_FDX.
For example, when operating with the CFDX frequency plan, with 12 dB uptilt at 108 MHz, the MER requirement
from Table 12 is 36 dB, and so a discrete spur at 108 MHz, if one of the five to be excluded in the range of 85 MHz
to 300 MHz, could reach as high as -34 dBc and still qualify for exclusion, where 0 dBc corresponds to the power in
a subcarrier at the top of the DRW_FDX at 108 MHz. For the exclusions in the CFDX spectral regions 684 MHz to
804 MHz and 804 MHz to 1218 MHz, the exclusion limit corresponds to the value 2 dB relaxed from the MER
requirement at 684 MHz, and the 0 dBc reference is the top of the DRW_FDX at 684 MHz. For the exclusions in the
spectral region 85 MHz to 108 MHz, the exclusion limit corresponds to the value 2 dB relaxed from the MER
requirement at 108 MHz, and the 0 dBc reference is the top of the DRW_FDX at 108 MHz. In each of the five
spectral regions listed above, up to 3 discrete spurs up to -40 dBmV may be excluded from the Between Burst
requirements, and also 3 such discrete spur exclusions up to -40 dBmV for the 5 MHz to 85 MHz Transmitting Burst
requirement. Only a total of ten different discrete spur exclusion frequencies are allowed in 5 MHz to 1218 MHz.
The ten different exclusion frequencies are allowed, with the limitation of five or three per band as described above,
but these ten exclusion frequencies are applied to all tests; a different set of ten exclusion frequencies is NOT
allowed for different tests and different modes.
SpurFloor is defined as:
         SpurFloor = -51.8 dBc
Under-grant hold number of users is defined as:
         Under-Grant Hold Number of Users = Floor{ 0.2 + 10^( (-44 - SpurFloor)/10) } = 6
Under-grant hold bandwidth (UGHB) is defined as:
         Under-Grant Hold Bandwidth = (Full Duplex Allocated Spectrum)/(Under-Grant Hold Number of Users)
The spurious performance requirements defined above only apply when the FDX CM is operating within certain
ranges of values for P 1.6load_i-FDX, for i = 1 to the number of upstream channels in the TCS_FDX, and for granted
bandwidth of under-grant hold bandwidth or larger; where P1.6load_1-FDX is the highest loaded channel in this
specification (i.e., its power is the one closest to P ref-FDX).
When a modem is transmitting over a bandwidth of less than under-grant hold bandwidth the spurious emissions
requirement limit is the power value (in dBmV per 1.6 MHz), corresponding to the specifications for the power level
associated with a grant of bandwidth equal to under-grant hold bandwidth.
The FDX CM MUST meet the spurious emissions performance requirements when the equivalent DOCSIS channel
powers (P1.6r_n-FDX) are within 0–6 dB below the top of the DRW_FDX (P1.6load_min_set-FDX +6 ≥ P1.6load_i-FDX ≥
P1.6load_min_set-FDX) but is not required to meet spurious emissions performance requirements when P1.6r_n-FDX are not
within this range.
Further, the FDX CM MUST meet the spurious emissions performance requirements when P1.6load_1-FDX =
P1.6load_min_set-FDX. When P1.6load_1-FDX > P1.6load_min_set-FDX, the spurious emissions requirements in absolute terms are
relaxed by P1.6load_1-FDX – P1.6load_min_set-FDX but is not required to meet spurious emissions performance requirements
when this condition is not met.
The spurious performance requirements do not apply to any upstream channel from the time the output power on
any active upstream channel has varied by more than ±3 dB since the last global reconfiguration time through the
end of the next global reconfiguration time changes.
In Table 9, in-band spurious emissions include noise, carrier leakage, clock lines, synthesizer spurious products, and
other undesired transmitter products. It does not include ISI. The measurement bandwidth for in-band spurious
emissions for OFDM is equal to the subcarrier clock frequency (25 kHz or 50 kHz) and is not a synchronous
measurement. The signal reference power for OFDMA in-band spurious emissions is the total transmit power
measured and adjusted (if applicable) as described in section 7.2.4.12.7, and then apportioned to a single data
subcarrier.
The measurement bandwidth is 4 MHz for the between bursts (none of the channels in the TCS_FDX is bursting)
specs of Table 9. The signal reference power for between bursts transmissions, "0 dBr", is the PSD for the top of the
DRW_FDX, as measured at Interface Fˊ.


06/27/25                                               CableLabs                                                        65
<!-- PDF_PAGE: 66 -->
CM-SP-PHYv4.0-I07-250627                                               Data-Over-Cable Service Interface Specifications


The transmitting burst specs apply during the minislots granted to the FDX CM in the FDX band (when the FDX
CM uses all or a portion of the grant), and for 20 µs before and after the granted minislot for OFDMA. The between
bursts specs apply except during a used grant of minislots on any active channel in the FDX band for the FDX CM,
and 20 µs before and after the used grant for OFDMA. In Table 9 entries, the signal reference power, "0 dBr", is the
PSD for the top of the DRW_FDX, as measured at Interface Fˊ.
For the purpose of spurious emissions definitions, a granted burst refers to a burst of minislots to be transmitted at
the same time from the same FDX CM; these minislots are not necessarily contiguous in frequency.
For initial ranging and before completion of fine ranging, spurious emissions requirements use Table 9 and Table
10, and if transmissions use subcarrier power which is X dB lower than indicated by P1.6low-FDX, then the spurious
emissions requirements in absolute terms are relaxed by X dB.
Spurious emissions requirements for grants of 10% or less of the FDX allocated spectrum may be relaxed by 2 dB in
an amount of spectrum equal to:
         measurement BW * ceil(10% of the FDX allocated spectrum/measurement BW)
anywhere in the whole upstream spectrum for emission requirements specified in Table 10.
A 2 dB relief applies in the measurement bandwidth. This relief does not apply to between bursts emission
requirements.
The FDX CMTS MUST NOT command a grant to the FDX CM in the FDX band which is smaller than the
minimum grant bandwidth shown in Table 10 - Spurious Emissions Requirements in the Upstream Frequency Range
for Grants of Under-Grant Hold Bandwidth and Larger, which is 5.2 MHz with 96 MHz FDX allocated spectrum,
10.4 MHz with 192 MHz FDX allocated spectrum, 16.0 MHz with 288 MHz FDX allocated spectrum, 21.2 MHz
with 384 MHz FDX allocated spectrum, and 32.0 MHz with 576 MHz FDX allocated spectrum.
                                                Table 9 - Spurious Emissions

                 Parameter                       Transmitting Burst 1,5,10,11,12,14,16        Between Bursts 5,10,11,12,15,16
 Inband (modulated spectrum of the grant)       -42 dBr OFDMA 100% grant     4,5,6,8,9
                                                                                           Max{-72 dBr, -43 dBmV}
                                                -47 dBr OFDMA UGHB% grant 4,5,6,8,9        See Note 7
 Adjacent minislot (adjacent to the modulated   + 0.2 dB relaxation.                       Same as for inband
 spectrum of the grant)                         See Table 10                               See Note 7
 400 kHz next to modulated spectrum
 FDX band                                  See 7.2.4.12.7.1.2                              Same as for inband
 Within the FDX band, excluding assigned                                                   See Note 7
 channel, adjacent channels. FDX band is
 defined for CFDX as 108 MHz to 684 MHz or
 for SFDX as 258 to 834 MHz.
 Requirements for the emissions from 5 MHz–
 108 MHz and 684 MHz and above
 CM integrated spurious emissions limits (all
 in 4 MHz, includes discretes)1
        CFDX                   SFDX
 5 MHz to 85 MHz        5 MHz to 204 MHz        -45 dBmV                                   -50 dBmV
 85 MHz to 108 MHz      204 MHz to 258 MHz      -35 dBr                                    Same as inband
 684 MHz to 804 MHz     834 MHz to 984 MHz      -42 dBr                                    Same as inband - Note 7
 804 MHz to 1218        984 MHz to 1218         -45 dBmV                                   -45 dBmV
 MHz                    MHz
 For the case where the upstream operating      For all four bands:                        For 5–85 MHz in CFDX, or 5–204 MHz
 range is 108 MHz to 684 MHz:                                                              in SFDX:

 CM discrete spurious emissions limits1         Largest discrete spurious emissions at     Largest discrete spurious emissions at
                                                least 5 dB lower power than the limit on   least 3 dB lower power than the limit on




66                                                       CableLabs                                                     06/27/25
<!-- PDF_PAGE: 67 -->
Physical Layer Specification                                                                     CM-SP-PHYv4.0-I07-250627


                  Parameter                        Transmitting Burst 1,5,10,11,12,14,16         Between Bursts 5,10,11,12,15,16
        CFDX                    SFDX             integrated spurious emissions in       integrated spurious emissions in
                                                 4 MHz, in the same band (see table row 4 MHz, in the same band (see table
 5 MHz to 85 MHz        5 MHz to 204 MHz         above)                                 row above)
 85 MHz to 108 MHz      204 MHz to 258 MHz
 684 MHz to 804 MHz     834 MHz to 984 MHz                                                   For the remaining three bands: Largest
 804 MHz to 1218        984 MHz to 1218                                                      discrete spurious emissions at least
 MHz                    MHz                                                                  5 dB lower power than the limit on
                                                                                             integrated spurious emissions in
                                                                                             4 MHz, in the same band (see table
                                                                                             row above)
 TABLE NOTES
 NOTE 1    Up to 5 discrete spurs in each of the following bands may be excluded: For CFDX, 85–300 MHz; 300–492 MHz; 492–
           684 MHz; 684–804 MHz; and 804–1218 MHz or for SFDX, 204–450 MHz; 450–642 MHz; 642–834 MHz; 834–984
           MHz; and 984–1218 MHz, while the CM is transmitting burst in the FDX band. These 5 spurs are the same spurs that
           may be excluded for spurious emissions and MER and not an additional or different set.
 NOTE 2    N/A.
 NOTE 3    N/A.
 NOTE 4    N/A.
 NOTE 5    This value is to be met when P1.6load = P1.6load_min_set. "0 dBr" is referenced to the top of the DRW_FDX.
 NOTE 6    Receive equalization is allowed if an MER test approach is used, to take ISI out of the measurement; measurements
           other than MER-based to find spurs or other unwanted power may be applied to this requirement.
 NOTE 7    Between burst spurious emissions in the FDX band (108–684 MHz for CFDX or 258–834 MHz for SFDX) is limited to -
           66 dBr when the CM is transmitting a background echo cancellation training signal and limited to -60 dBr in the FDX
           transition band (684–804 MHz for CFDX or 834–984 MHz for SFDX).
 NOTE 8    Frequency-dependent relaxation as a function of the measurement center frequency, linearly scales in frequency from
           5 dB at 108 MHz to 0 dB at 684 MHz in the CFDX frequency plan or from 5 dB at 258 MHz to 0 dB at 834 MHz for the
           SFDX frequency plan.
 NOTE 9    1 dB relaxation with 12 dB uptilt.
 NOTE 10 "dBr" values measured at Interface Fˊ, "dBmV" values measured at Interface F, and all measurements and averages
         computed over 4 MHz bandwidth unless noted otherwise.
 NOTE 11 Transmitting burst is with FDX band upstream bursting, and between burst is with FDX band upstream between bursts
         entirely in the FDX band (108–684 MHz for CFDX or 258 to 834 MHz for SFDX).
 NOTE 12 For all the requirements except for the 5–85 MHz and 85–108 MHz in CFDX, or 5–204 MHz and 204–258 MHz in
         SFDX, both transmitting burst and between burst, the requirements need to be met with the legacy upstream (5–
         85 MHz in CFDX or 5–204 MHz in SFDX) bursting. With the requirements in 5–85 MHz and 85–108 MHz, the
         requirements need to be met with the legacy upstream (5–85 MHz in CFDX or 5–204 MHz in SFDX) between bursts.
 NOTE 13 No fidelity requirements when transmitting CW tones for sounding.
 NOTE 14 Allowance for 3 excluded discrete spurs up to -40 dBmV in the legacy upstream (5–85 MHz for CFDX or 5–204 MHz
         for SFDX) while transmitting burst.
 NOTE 15 Allowance for 3 excluded discrete spurs in each of the bands (for CFDX, 5–85; 85–300; 300–492; 492–684; 684–804;
         and 804–1218 MHz or for SFDX, 5–204; 204–450; 450–642; 642–834; 834–984; and 984–1218 MHz) to -40 dBmV
         while between bursts.
 NOTE 16 For the discrete spur exclusions of Note 1, Note 14, and Note 15, a total of ten different such exclusion frequencies
         are allowed to be applied for all the requirements, across the full 5–1218 MHz range, while also accommodating the
         restrictions on the number of exclusions in any one band. The ten exclusion frequencies are not allowed to change for
         application to different requirements, nor due to changes in mode in the CM under test.

7.2.4.12.7.1.1 Spurious Emissions in the Upstream FDX Frequency Range
Table 10 lists the required spurious level in a measurement interval. The initial measurement interval at which to
start measuring the spurious emissions (from the transmitted burst's modulation edge) is 400 kHz from the edge of
the transmission's modulation spectrum. Measurements should start at the initial distance and be repeated at
increasing distance from the carrier, until the upstream band edge or spectrum adjacent to another modulated
spectrum is reached.




06/27/25                                                   CableLabs                                                              67
<!-- PDF_PAGE: 68 -->
CM-SP-PHYv4.0-I07-250627                                            Data-Over-Cable Service Interface Specifications


In addition to the spurious emissions level generated in Table 11, there is a frequency-dependent relaxation provided
as a function of the center frequency of the measurement, which is given in Table Note 3 of Table 11 as:
          Frequency-Dependent_Spurious_Emissions_Relaxation =

          5 * (684 MHz – measurement_center_frequency) / (684 MHz–108 MHz) dB for CFDX; or
          5 * (834 MHz – measurement_center_frequency) / (834 MHz–258 MHz) dB for SFDX
For example, using the CFDX frequency plan with 576 MHz FDX allocated spectrum and 96 MHz grant, the
requirement is -61.8 dBc at 679.2 MHz and -56.8 dBc at 112.8 MHz.
For OFDMA transmissions with non-zero transmit windowing, the FDX CM MUST meet the required performance
measured within the 2.0 MHz adjacent to the modulated spectrum using slicer values from an FDX CMTS burst
receiver or equivalent, synchronized to the downstream transmission provided to the FDX CM.
In the rest of the spectrum, the FDX CM MUST meet the required performance measured with a bandpass filter
(e.g., an unsynchronized measurement).
For OFDMA transmissions with zero transmit windowing, the FDX CM MUST meet the required performance
using synchronized measurements across the complete upstream spectrum.
Spurious emissions allocation for far out spurious emissions =
          Round {SpurFloor + 10*log10(Measurement Bandwidth/Under-Grant Hold Bandwidth),0.1}.
For transmission bursts with modulation spectrum less than the under-grant hold bandwidth, the spurious power
requirement is calculated as above, but increased by 10*log10 (under-grant hold bandwidth/grant bandwidth).
                   Table 10 - Spurious Emissions Requirements in the Upstream Frequency Range
                                for Grants of Under-Grant Hold Bandwidth and Larger

 FDX Allocated Minimum Grant SpurFloor Under-Grant  Under-Grant   Measurement Specification
   Spectrum      Bandwidth     (dBc)   Hold #Users Hold Bandwidth  Bandwidth in the Interval
    (MHz)          (MHz)                                (MHz)        (MHz)1      (dBc)2, 3
       96                     5.2                - 51.8       6                  16                 9.6              -54.0
       192                   10.4                - 51.8       6                  32                 9.6              -57.0
       288                   16.0                - 51.8       6                  48                 9.6              -58.8
       384                   21.2                - 51.8       6                  64                 9.6              -60.0
       576                   32.0                - 51.8       6                  96                 9.6              -61.8
 TABLE NOTES
 NOTE 1      The measurement bandwidth is a contiguous sliding measurement window.
 NOTE 2     Frequency-dependent relaxation as a function of the measurement center frequency, linearly scales in frequency in
            CFDX from 5 dB at 108 MHz to 0 dB at 684 MHz or in SFDX, from 5 dB at 258 MHz to 0 dB at 834 MHz
 NOTE 3      1 dB relaxation with 12 dB uptilt


The FDX CM MUST control transmissions such that within the measurement bandwidth of Table 10 - Spurious
Emissions Requirements in the Upstream Frequency Range for Grants of Under-Grant Hold Bandwidth and Larger,
spurious emissions measured for individual subcarriers contain no more than +3 dB power larger than the required
average power of the spurious emissions in the full measurement bandwidth divided by the number of subcarriers in
the measurement bandwidth. When non-synchronous measurements are made, only 25 kHz measurement bandwidth
is used.
For OFDMA transmissions, bandpass measurements rather than synchronous measurements may be applied.

7.2.4.12.7.1.2 Adjacent Channel Spurious Emissions
Spurious emissions from a transmitted burst may occur in adjacent spectrum, which could be occupied by OFDMA
subcarriers transmitted by another CM.




68                                                        CableLabs                                                  06/27/25
<!-- PDF_PAGE: 69 -->
Physical Layer Specification                                                          CM-SP-PHYv4.0-I07-250627


The spurious emissions requirements for adjacent spectrum to a transmitted burst are given in Table 10 but with an
additional 0.2 dB allowance, where the measurement is over the 9.6 MHz spectrum adjacent to the modulated
spectrum.
The measurement is performed starting on an adjacent subcarrier of the transmitted spectrum (both above and
below), using the slicer values from a CMTS burst receiver or equivalent synchronized to the downstream
transmission provided to the CM.
The FDX CM MUST control transmissions such that within the adjacent 400 kHz of modulated spectrum, spurious
emissions measured for individual subcarriers contain no more than +5 dB power larger than the required average
power of the spurious emissions in the full measurement bandwidth divided by the number of subcarriers in the
measurement bandwidth.
For the 9.2 MHz measurement bandwidth which is outside the 400 kHz adjacent to the modulated spectrum, the
FDX CM MUST control transmissions such that spurious emissions measured for individual subcarriers contain no
more than +3 dB power larger than the required average power of the spurious emissions in the full measurement
bandwidth divided by the number of subcarriers in the measurement bandwidth. For any portion of the 9.6 MHz
measurement bandwidth where non-synchronous measurements are made, only 25 kHz measurement bandwidth is
used.
Bandpass measurements rather than synchronous measurements may be applied where possible.

7.2.4.12.7.1.3 Spurious Emissions During FDX Burst On/Off Transients
The FDX CM MUST control spurious emissions prior to and during ramp-up, during and following ramp-down, and
before and after a burst.
The FDX CM's on/off spurious emissions, such as the change in voltage at the upstream transmitter output, due to
enabling or disabling transmission, MUST be no more than 50 mV.
The FDX CM's voltage step MUST be dissipated no faster than 4 μs of constant slewing. This requirement applies
when the FDX CM is transmitting at +55 dBmV or more per channel on any channel.
At backed-off transmit levels, the FDX CM's maximum change in voltage MUST decrease by a factor of 2 for each
6 dB decrease of power level in the highest power active channel, from +55 dBmV per channel, down to a
maximum change of 3.5 mV at 31 dBmV per channel and below. This requirement does not apply to FDX CM
power-on and power-off transients.

7.2.4.12.7.2 FDX OFDMA MER Requirements
Transmit modulation error ratio (TxMER or just MER) measures the cluster variance caused by the FDX CM during
upstream transmission due to transmitter imperfections. The terms "equalized MER" and "unequalized MER" refer
to a measurement with linear distortions equalized or not equalized, respectively, by the test equipment receive
equalizer. The requirements in this section refer only to unequalized MER, as described for each requirement. MER
is measured on each modulated data subcarrier and non-boosted pilot (MER is computed based on the unboosted
pilot power) in a minislot of a granted burst and averaged for all the subcarriers in each minislot. MER includes the
effects of inter-carrier interference (ICI), spurious emissions, phase noise, noise, distortion, and all other undesired
transmitter degradations with an exception for a select number of discrete spurs impacting a select number of
subcarriers. MER requirements are measured with a calibrated test instrument that synchronizes to the OFDMA
signal, applies a receive equalizer in the test instrument that removes MER contributions from nominal channel
imperfections related to the measurement equipment, and calculates the value. The equalizer in the test instrument is
calculated, applied and frozen for the CM testing. Receiver equalization of FDX CM linear distortion is not
provided; hence, this is considered to be a measurement of unequalized MER, even though the test equipment
contains a fixed equalizer setting.

7.2.4.12.7.2.1 Definitions
MER is defined as follows for OFDMA. The transmitted RF waveform at the F connector of the CM (after
appropriate down conversion) is filtered, converted to baseband, sampled, and processed using standard OFDMA
receiver methods, with the exception that receiver equalization is not provided. The processed values are used in the
following formula. No external noise (AWGN) is added to the signal.


06/27/25                                            CableLabs                                                      69
<!-- PDF_PAGE: 70 -->
CM-SP-PHYv4.0-I07-250627                                               Data-Over-Cable Service Interface Specifications


The carrier frequency offset, carrier amplitude, carrier phase offset, and timing will be adjusted during each burst to
maximize MER as follows:
     •   One carrier amplitude adjustment common for all subcarriers and OFDM symbols in burst.
     •   One carrier frequency offset common for all subcarriers resulting in phase offset ramping across OFDM
         symbols in bursts.
     •   One timing adjustment resulting in phase ramp across subcarriers.
     •   One carrier phase offset common to all subcarriers per OFDM symbol in addition to the phase ramp.
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
channel set are granted to the FDX CM. For purposes of testing MER, a grant of all OFDMA minislots in the
transmit channel set may be used; there may be non-excluded subcarriers that are not within minislots.
MER with a UGHB is defined as the condition when less than or equal to UGHB of the FDX OFDMA minislots
have been granted to the FDX CM.

7.2.4.12.7.2.2 FDX Requirements
Unless otherwise stated, the FDX CM MUST meet or exceed the following MER limits over the full transmit power
range, all modulation orders, all grant configurations and over the full upstream frequency range.
The following flat channel measurements (ideally flat channel except for downtilt specified between Interface F and
F') with the transmitted specified uptilt (Table 11) are made after the pre-equalizer coefficients have been set to their
optimum values. The receiver uses best effort synchronization to optimize the MER measurement.
                             Table 11 - Upstream MER Requirements (with Pre-Equalization)

         Parameter                                                              Value
 MER (100% grant)                    Each minislot MER ≥ 42 dB (Notes 1, 2, 3, 4, 5) at 684 MHz for CFDX or 834 MHz for SFDX
                                     with 8 dB and 10 dB uptilt
 MER (UGHB % grant)                  Each minislot MER ≥ 47 dB (Notes 1, 2, 3, 4) at 684 MHz for CFDX or 834 MHz for SFDX with
                                     8 dB and 10 dB uptilt
 Pre-equalizer constraints           Coefficients set to their optimum values
TABLE NOTES
NOTE 1    Up to 5 subcarriers within the entire upstream bandwidth with discrete spurs may be excluded from the MER calculation
          if they fall within transmitted minislots. These 5 spurs are the same spurs that may be excluded for spurious emissions
          and not an additional or different set.
NOTE 2    This value is to be met when P1.6load = P1.6load_min_set.
NOTE 3    Frequency-Dependent Relaxation as a function of the measurement center frequency, linearly scales in frequency from
          5 dB at 108 MHz to 0 dB at 684 MHz.




70                                                            CableLabs                                              06/27/25
<!-- PDF_PAGE: 71 -->
Physical Layer Specification                                                                    CM-SP-PHYv4.0-I07-250627


NOTE 4      1 dB relaxation with 12 dB uptilt
NOTE 5      For testing 100% grant, a grant of all OFDMA minislots may be used.


The following flat channel measurements (Table 12) are made with the pre-equalizer coefficients set to unity and
with the transmitted specified uptilt and the receiver implementing best effort synchronization. For this
measurement, the receiver may also apply partial equalization. The partial equalizer is not to correct for the portion
of the CM's time-domain impulse response greater than 200 ns or frequency-domain amplitude response greater than
+1 dB or less than -3 dB from the average amplitude. An additional 1 dB attenuation in the amplitude response is
allowed in the upper 10% of the specified passband frequency. It is not expected that the partial equalizer is
implemented on CMTS receiver. A partial equalizer could be implemented offline via commercial receivers or
simulation tools.
                              Table 12 - Upstream MER Requirements (no Pre-Equalization)

            Parameter                                                             Value
 MER (100% grant)                    Each minislot MER ≥ 39 dB (Notes 1, 2, 3, 4) at 684 MHz for CFDX or 834 MHz for SFDX with
                                     8 dB and 10 dB uptilt
 MER (UGHB% grant)                   Each minislot MER ≥ 39 dB (Notes 1, 2, 3, 4) at 684 MHz for CFDX or 834 MHz for SFDX with
                                     8 dB and 10 dB uptilt
 Pre-equalizer constraints           Pre-equalization not used
 TABLE NOTES
 NOTE 1 Up to 5 subcarriers within the entire upstream bandwidth with discrete spurs may be excluded from the MER
        calculation if they fall within transmitted minislots. These 5 spurs are the same spurs that may be excluded for spurious
        emissions and not an additional or different set.
 NOTE 2 This value is to be met when P1.6load = P1.6load_min_set.
 NOTE 3 Frequency-Dependent Relaxation as a function of the measurement center frequency, linearly scales in frequency for
        CFDX, from 5 dB at 108 MHz to 0 dB at 684 MHz, or in SFDX, from 5 dB at 258 MHz to 0 dB at 834 MHz
 NOTE 4 1 dB relaxation with 12 dB uptilt

7.2.4.13 Cable Modem Transmitter Output Requirements in the FDX Band
For FDX devices operating in FDX mode, this section augments section 7.4.13 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.
The FDX CM MUST output an RF modulated signal with characteristics delineated in Table 13 - CM Transmitter
Output Signal Characteristics for the FDX Band.
                      Table 13 - CM Transmitter Output Signal Characteristics for the FDX Band

                  Parameter                                                             Value
 Frequency                                         108 MHz to 684 MHz for CFDX or 258 MHz to 834 MHz for SFDX
 Signal type                                       OFDMA
 Maximum OFDMA channel bandwidth                   95 MHz
 Minimum OFDMA encompassed spectrum                86 MHz per sub-band (Notes 1, 2 below)
 Number of independently configurable              The downstream channel and upstream channels sharing the same sub-band are
 OFDMA channels                                    configured to the same subcarrier spacing and cyclic prefix length. The subcarrier
                                                   spacing and cyclic prefix on different sub-bands are allowed to be different.
 Subcarrier channel spacing                        25 kHz, 50 kHz
 FFT size                                          50 kHz: 2048 (2K FFT); 1900 maximum active subcarriers
                                                   25 kHz: 4096 (4K FFT); 3800 maximum active subcarriers
 Sampling rate                                     102.4 MHz
 FFT time duration                                 40 µs (25 kHz subcarriers)
                                                   20 µs (50 kHz subcarriers)
 Modulation type                                   BPSK, QPSK, 8-QAM, 16-QAM, 32-QAM, 64-QAM, 128-QAM, 256-QAM, 512-
                                                   QAM, 1024-QAM, 2048-QAM, 4096-QAM




06/27/25                                                     CableLabs                                                           71
<!-- PDF_PAGE: 72 -->
CM-SP-PHYv4.0-I07-250627                                           Data-Over-Cable Service Interface Specifications


                 Parameter                                                        Value
 Bit loading                                 Variable from minislot to minislot
                                             Constant for subcarriers of the same type in the minislot
                                             Support zero valued subcarriers per profile and minislot
 Pilot tones                                 Boosted pilots are not supported in the FDX band
 Cyclic prefix options                       Samples      µs
                                             96           0.9375
                                             128          1.25
                                             256          2.5
                                             384          3.75
                                             512          5.0
 Windowing size options                      Samples     µs
                                             0           0
                                             32          0.3125
                                             64          0.625
                                             96          0.9375
                                             128         1.25
                                             160         1.5625
                                             192         1.875
                                             224         2.1875
                                             Raised cosine absorbed by CP
 Level                                       Total average output power of 64.5 dBmV
 Output impedance                            75 ohms
 Output return loss while in FDX mode        > 6 dB: 5 MHz to 85 MHz for CFDX or 5–204 MHz for SFDX
                                             > 6 dB: 108 MHz to the downstream upper band edge for CFDX or 258 MHz to
                                             the downstream upper band edge for SFDX
 Connector                                   F connector per [ISO/IEC-61169-24] or [SCTE 02]
TABLE NOTES
NOTE 1 Use case: Ranging probe transmission. Value set as wide as possible for per tone pre-EQ coverage, when the tapper
       region is the largest.
NOTE 2 This is the same value as the min contiguous modulated spectrum.



7.2.4.14 FDX CMTS Receiver Capabilities
For FDX devices operating in FDX mode, this section augments section 7.4.14 of [DOCSIS PHYv3.1] and
corresponding subsections unless otherwise noted.

7.2.4.14.1 FDX CMTS Receiver Input Power Requirements
Demodulator input power characteristics are not applicable to an FDX node as interface C is not a defined
measurement interface in an FDX node. Measurements are performed on interface D and thus demodulator input
power is replaced by minimum performance requirements defined in Table 16.
For a Unified Node operating in FDX mode, measurements are also made at Interface D as described in this section;
i.e., the measurement point is the same as for an FDX device.

7.2.4.14.2 FDX Node Receiver Error Ratio Performance in AWGN Channel
The post-FEC Packet Error Ratio (PER) performance of the OFDMA receiver at the node is defined with reference
to a PER of 10-6 with 1500 byte Ethernet packets. This section describes the conditions under which this packet error
ratio measurement has to be made.
The FDX node MUST NOT exceed packet error ratio of 10-6 in any of the six FDX upstream channels when
operating under downstream transmission and channel conditions described below and when carrier-to-noise ratio of
the channel is at or below the values shown in Table 16 - Node Minimum CNR Performance in FDX Channel.
     •   A single transmitter CM, pre-equalized and ranged to provide a flat power spectral density at interface D.
     •   Measurement on single FDX OFDMA channels with 95 MHz modulated spectrum.


72                                                     CableLabs                                               06/27/25
<!-- PDF_PAGE: 73 -->
Physical Layer Specification                                                           CM-SP-PHYv4.0-I07-250627


    •   Ranging with same CNR and input level to FDX node as with data bursts, and with 8-symbol probes.
    •   Any valid transmit combination (frequency, subcarrier clock frequency, transmit window, cyclic prefix,
        OFDMA frame length, interleaving depth, pilot patterns, etc.) as defined for the FDX band.
    •   Input power level per constellation is per the set points as defined in Table 16.
    •   OFDMA phase noise and frequency offset are at the upper limits as defined for the CM transmission
        specification.
    •   Large grants consisting of several 1500 Byte packets.
    •   The CMTS is allowed to construct MAPs according to its own scheduler implementation.
    •   The FDX node is transmitting as follows: for CFDX operation over the frequency band from 108 MHz to
        1218 MHz with an up-tilted spectrum as defined in the section covering node downstream fidelity
        requirements; or for SFDX operation, using the SFDX downstream PSD as defined in section 7.2.4.14.2.1,
        "SFDX Node Output PSD".
    •   Using a cable network model to provide micro-reflections of the downstream transmission back to the FDX
        node, as shown in Table 14. The cable network model is specified by the s11 parameter of the cable
        network at FDX node Interface D. This parameter is tabulated in [NodePortEchoResponse], as a function of
        frequency across the FDX band.
    •   The peak envelope echo return loss at interface D is not less than the return loss given in Table 14 below.
        Linear interpolation is applied to the table to obtain the return loss at intermediate frequencies.
                                           Table 14 - Peak Return Loss

                         Frequency Plan         Frequency (MHz)          Return Loss (dB)
                               CFDX                     108                     10.4
                                                        204                     12.8
                                                        300                     14.4
                                                        396                     15.8
                                                        492                     16.9
                                                        588                     17.9
                                                        684                     18.7
                                                        780                     19.4
                                                        876                     19.8
                               SFDX                     258                     13.7
                                                        254                     15,2
                                                        450                     16.4
                                                        546                     17.5
                                                        642                     18.4
                                                        738                     19.1
                                                        834                     19.6
                                                        930                     19.9
                                                       1026                     20.0


The average echo return loss in each of the six 96 MHz upstream FDX channels as well as the two 96 MHz channels
immediately above the FDX transition band upper band edge at interface D is not less than the corresponding
average echo return loss obtained from the cable model, given in Table 15.




06/27/25                                           CableLabs                                                    73
<!-- PDF_PAGE: 74 -->
CM-SP-PHYv4.0-I07-250627                                        Data-Over-Cable Service Interface Specifications


                                          Table 15 - Average Return Loss

                          Frequency Plan         Frequency (MHz)          Average Return
                                                                            Loss (dB)
                                CFDX                  108–204                    17.6
                                                      204–300                    19.0
                                                      300–396                    19.9
                                                      396–492                    20.8
                                                      492–588                    21.5
                                                      588–684                    22.1
                                                      684–780                    22.6
                                                      780–876                    22.9
                                SFDX                  258–354                    19.5
                                                      354–450                    20.4
                                                      450–546                    21.2
                                                      546–642                    21.8
                                                      642–738                    22.4
                                                      738–834                    22.8
                                                      834–930                    23.0
                                                      930–1026                   23.0


In the case of multiple cable legs from the node, only one leg is active, with other legs terminated appropriately.




                   Figure 10 - Set-up for the FDX Node Packet-Error-Ratio Performance Test
The drop cable in Figure 10 connecting the CM is intended to compensate for the up-tilt in the CM output in order to
provide a flat upstream power spectral density at interface D.
Figure 11 shows the node operating with the CFDX frequency plan receiving an upstream channel in the frequency
range 396 MHz to 492 MHz. This is for illustration purposes only. FDX node PER requirements apply with all six
96 MHz FDX upstream channels active and with worst case CNR conditions listed in Table 16.




74                                                  CableLabs                                              06/27/25
<!-- PDF_PAGE: 75 -->
Physical Layer Specification                                                             CM-SP-PHYv4.0-I07-250627




     Figure 11 – Example Spectrum at Interface D for the Node Receiver PER Test, CFDX Frequency Plan
CNR is defined as the ratio of the received signal power to the additive white Gaussian noise power over the 95
MHz received bandwidth.
                           Table 16 - Node Minimum CNR Performance in FDX Channel

                                             Set Point Measured at Interface D
         Constellation       CNR (dB)                (dBmV/6.4 MHz)                              Offset
         QPSK                   12.5                           0                                  0 dB
         8-QAM                  15.5                           0                                  0 dB
         16-QAM                 18.5                           0                                  0 dB
         32-QAM                 22.0                           0                                  0 dB
         64-QAM                 25.5                           0                                  0 dB
         128-QAM                29.0                           1                                  0 dB
         256-QAM                32.0                           3                                  0 dB
         512-QAM                36.0                           5                                  0 dB
         1024-QAM               44.0                           7                                  0 dB
         2048-QAM               N/A                           N/A                                 0 dB
         4096-QAM               N/A                           N/A                                 0 dB
         TABLE NOTES
         NOTE 1 CNR is defined here as the ratio of average signal power in occupied bandwidth to the average noise
                power in the occupied bandwidth given by the noise power spectral density integrated over the same
                occupied bandwidth.
         NOTE 2 Channel CNR is adjusted to the required level by measuring the source in-band noise including
                phase noise component and adding the required delta noise from an external AWGN generator.
         NOTE 3 The channel CNR requirements are for OFDMA channels with non-boosted pilots



7.2.4.14.2.1 FDX Node Output PSD in FDX Band for SFDX Frequency Plan
The SFDX frequency plan is expected to receive the highest levels of adoption among operators employing a high
split spectrum arrangement for DOCSIS 3.1 deployments. Such an arrangement provides upstream-only spectrum
from 5 MHz to 204 MHz, with downstream transmission beginning at 258 MHz and continuing to the downstream
upper band edge of the plant.
The majority of these operators have expressed an intention to progress toward a downstream upper band edge of
1794 MHz. Downstream transmitter fidelity for this case is measured at Interface C, as described in section 7.3.5.9,
which describes downstream fidelity requirements for FDD devices.
For purposes of establishing a performance baseline for FDX upstream receiver operation using the SFDX
frequency plan, it is necessary to define an "SFDX downstream PSD" at Interface D of an FDX node. This PSD is
used to define the node transmitter conditions under which the upstream receiver performance specification applies
when operating with the SFDX frequency plan. The details of this usage are described in section 7.2.4.14.2.
Various approaches may be favored by different operators for defining a suitable downstream FDX node transmitter
output profile for SFDX operation depending on plant characteristics, spectrum and power levels assigned for legacy
services, available TCP from nodes and plant amplifiers, and other considerations. This specification does not
attempt to capture the full range of variability in operator product specifications.


06/27/25                                              CableLabs                                                      75
<!-- PDF_PAGE: 76 -->
CM-SP-PHYv4.0-I07-250627                                                Data-Over-Cable Service Interface Specifications


The SFDX downstream PSD serves as a reference PSD for use with the SFDX frequency plan, defining the energy
transmitted by an FDX node, which results in self-interference (echo) reaching the FDX upstream receiver. As such,
the SFDX downstream PSD covers the frequency range of 258 MHz to 984 MHz; this includes the FDX band itself,
as well as the diplexer transition band immediately above the FDX band. Inclusion of this transition band ensures
that any echo energy reaching the receiver within this band is accounted for in the receiver performance.
The SFDX downstream PSD is illustrated in Figure 12 below. It is defined as:
     •   a nominal RF output power of 35.6 dBmV/6 MHz within the 6 MHz band centered at 261 MHz;
     •   a nominal RF output power of 45.7 dBmV/6 MHz within the 6 MHz band centered at 981 MHz; and
     •   a PSD between these two bands providing a smooth, linear tilt connecting the two points defined above.
A real-world deployment will include additional energy above 980 MHz up to the downstream upper band edge of
the plant. The amount and distribution of this additional Tx power is not defined as part of the SFDX downstream
PSD but is left for configuration by the operator as desired.
The total composite power of the SFDX downstream PSD is 60.0 dBmV in the range of 258–834 MHz and
62.4 dBmV in the range of 258–984 MHz.


                                            SFDX Band Downstream PSD
                      dBmV/
                      6 MHz




                                                          45.7 dBmV/6 MHz
                                                                                                 examples of possible
                                                                                                 Tx profiles at higher
                                           43.6 dBmV/6 MHz                                           frequencies

                              35.6 dBmV/6 MHz




                                                                                frequency, MHz
                                   258                            834   984

                                            TCP = 60.0 dBmV


                                                TCP = 62.4 dBmV




                          Figure 12 - The SFDX Band Downstream PSD vs. Frequency

7.2.4.15 Ranging
See [DOCSIS PHYv3.1], section 7.4.15.

7.2.4.16 Upstream Pilot Structure
See [DOCSIS PHYv3.1], section 7.4.16.

7.2.4.17 Upstream Pre-Equalization
See [DOCSIS PHYv3.1], section 7.4.17.




76                                                        CableLabs                                                     06/27/25
