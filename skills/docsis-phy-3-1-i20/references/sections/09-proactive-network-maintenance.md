# 9 - Proactive Network Maintenance

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 185-196
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 185 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


9 PROACTIVE NETWORK MAINTENANCE
9.1     Scope
This section defines the requirements supporting Proactive Network Maintenance (PNM). CMTS and cable modem
features and capabilities can be leveraged to enable measurement and reporting of network conditions such that
undesired impacts such as plant equipment and cable faults, interference from other systems and ingress can be
detected and measured. With this information cable network operations personnel can make modifications necessary
to improve conditions and monitor network trends to detect when network improvements are needed.

9.2     System Description
As shown in Figure 97, the CMTS and CM contain test points which include essential functions of a spectrum
analyzer, vector signal analyzer (VSA), and network analyzer, while the cable plant is considered the Device Under
Test (DUT). The goal is to rapidly and accurately characterize, maintain and troubleshoot the upstream and
downstream cable plant, in order to guarantee the highest throughput and reliability of service. The CMTS and CM
make the specified measurements and report the results to the PNM management entity as defined in [DOCSIS
CCAP-OSSIv3.1] and [DOCSIS CM-OSSIv3.1]. Unless otherwise specified, the CM MUST make all its PNM
measurements while in service, without suspending normal operational modes or data transmission and reception.
Unless otherwise specified, the CMTS MUST make all its PNM measurements while in service, without suspending
normal operational modes or data transmission and reception. Any specified timestamping of PNM measurements is
done with nominal accuracy of 100 ms or better.




             Figure 97 - Test Points in CM and CMTS Supporting Proactive Network Maintenance

9.3     Downstream PNM Requirements
9.3.1    Downstream Symbol Capture
The purpose of downstream symbol capture is to provide partial functionality of a network analyzer to analyze the
response of the cable plant.
At the CMTS, the transmitted frequency-domain modulation values of one full OFDM symbol before the IFFT are
captured and made available for analysis. This includes the I and Q modulation values of all subcarriers in the active
bandwidth of the OFDM channel, including data subcarriers, pilots, PLC preamble symbols and excluded subcarriers.
This capture will result in a number of samples that depends on the OFDM channel width, per Section 7.5.7.1. As



04/19/23                                            CableLabs                                                    185

<!-- PDF_PAGE: 186 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications


examples, for 50 kHz subcarrier spacing in a 192 MHz channel with an active bandwidth of 190 MHz, 3800 samples
will be captured; for 25 kHz subcarrier spacing in a 192 MHz channel with an active bandwidth of 190 MHz, 7600
samples will be captured; for 25 kHz subcarrier spacing in a 24 MHz channel with an active bandwidth of 22 MHz,
880 samples will be captured.
At the CM, the received I and Q time-domain samples of one full OFDM symbol before the FFT, not including the
guard interval, are captured and made available for analysis. This capture will result in a number of data points equal
to the FFT length in use, time aligned for receiver FFT processing. The number of captured samples can be reduced
for narrower channels if the sampling rate, which is implementation dependent, is reduced. The capture includes a
bit indicating if receiver windowing effects are present in the data.
As examples, for 50 kHz subcarrier spacing in a 192 MHz channel with 204.8 MHz sampling rate, 4096 samples
will be captured; for 25 kHz subcarrier spacing in a 192 MHz channel with 204.8 MHz sampling rate, 8192 samples
will be captured; for 50 kHz subcarrier spacing in a 24 MHz channel with a reduced sampling rate of 25.6 MHz, 512
samples will be captured.
Capturing the input and output of the cable plant is equivalent to a wideband sweep of the channel, which permits
full characterization of the linear and nonlinear response of the downstream plant. The MAC provides signaling via
the PLC Trigger Message to ensure that the same symbol is captured at the CMTS and CM.
The CMTS MUST be capable of capturing the modulation values of the full downstream symbol marked by the
trigger for analysis.
The CM MUST be capable of locating and capturing the time-domain samples of the full downstream symbol
marked by the trigger for analysis.

9.3.2    Downstream Wideband Spectrum Analysis
The purpose of downstream wideband spectrum capture is to provide a downstream wideband spectrum analyzer
function in the DOCSIS 3.1 CM similar to the capability provided in DOCSIS 3.0.
The CM MUST provide a downstream wideband spectrum capture and analysis capability.
The CM SHOULD provide the capability to capture and analyze the full downstream band of the cable plant.
The CM MUST provide a calibration constant permitting the downstream wideband spectrum capture measurement
to be related to the downstream received power measurement of Section 9.3.9.

9.3.3    Downstream Noise Power Ratio (NPR) Measurement
The purpose of downstream NPR measurement is to view the noise, interference and intermodulation products
underlying a portion of the OFDM signal. As part of its normal operation or in an out-of-service test, the CMTS can
define an exclusion band of zero-valued subcarriers which forms a spectral notch in the downstream OFDM signal
for all profiles of a given downstream channel. The CM provides its normal spectral capture measurements per
Section 9.3.2, or symbol capture per Section 9.3.1, which permit analysis of the notch depth. A possible use case is
to observe LTE interference occurring within an OFDM band; another is to observe intermodulation products
resulting from signal-level alignment issues. Since the introduction and removal of a notch affects all profiles,
causing possible link downtime, this measurement is intended for infrequent maintenance.

9.3.4    Downstream Channel Estimate Coefficients
The purpose of the Downstream Channel Estimate Coefficients item is for the CM to report its estimate of the
downstream channel response. The reciprocals of the channel response coefficients are typically used by the CM as
its frequency-domain downstream equalizer coefficients. The channel estimate consists of a single complex value
per subcarrier. [DOCSIS CCAP-OSSIv3.1] defines summary metrics to avoid having to send all coefficients on
every query.
The CM MUST report its downstream channel estimate (full set or summary) for any single OFDM downstream
channel upon request.




186                                                 CableLabs                                               04/19/23

<!-- PDF_PAGE: 187 -->
Physical Layer Specification                                                        CM-SP-PHYv3.1-I20-230419


9.3.5    Downstream Constellation Display
The downstream constellation display provides received QAM constellation points for display. Equalized soft
decisions (I and Q) at the slicer input are collected over time, possibly subsampling to reduce complexity, and made
available for analysis. Only data-bearing subcarriers with the specified QAM constellation are sampled. Pilots and
excluded subcarriers within the range are ignored. Up to 8192 samples are provided for each query; additional
queries may be made to further fill in the plot.
The CM MUST be capable of capturing and reporting received soft-decision samples, for a single selected
constellation from the set of profiles it is receiving within a single OFDM downstream channel.

9.3.6    Downstream Receive Modulation Error Ratio (RxMER) per Subcarrier
This item provides measurements of the receive modulation error ratio (RxMER) for each subcarrier. The CM
measures the RxMER using pilots and PLC preamble symbols, which are not subject to symbol errors as data
subcarriers would be. Since scattered pilots visit all data subcarriers and the PLC preamble symbols are known, the
RxMER of all active subcarriers in the OFDM band can be measured over time. For the purposes of this measurement,
RxMER is defined as the ratio of the average power of the ideal QAM constellation to the average error-vector power.
The error vector is the difference between the equalized received pilot or preamble value and the known correct pilot
value or preamble value. As a defining test case, for an ideal AWGN channel, an OFDM block containing a mix of
QAM constellations, with data-subcarrier CNR = 35 dB CNR on the QAM subcarriers, will yield an RxMER
measurement of nominally 35 dB averaged over all subcarrier locations. If some subcarriers (such as exclusion bands)
cannot be measured by the CM, the CM indicates that condition in the measurement data for those subcarriers.
RxMER may be more clearly defined in mathematical notation in accordance with Figure 98, which shows an ideal
transmit and receive model, with no intent to imply an implementation. Let p = scattered pilot (or PLC preamble)
symbol before transmit IFFT, H = channel coefficient for a given subcarrier frequency, n = noise, y = Hp + n =
unequalized received symbol after receive FFT. The receiver computes G = estimate of H, and computes the
equalized received symbol as r = y/G. Using the known modulation value of the pilot or preamble symbol p, the
receiver computes the equalized error vector as e = r - p. All the above quantities are complex scalars for a given
subcarrier. To compute RxMER, the receiver computes E = time average of |e|^2 over many visits of the scattered
pilot to the given subcarrier (or PLC preamble symbol as applicable), and E_dB = 10*log10(E). Let S_dB = average
power of ideal QAM data subcarrier constellation (not including pilots) expressed in dB. According to Annex 0,
QAM Constellation Scaling, all QAM constellations have the same average power. The CM reports RxMER_dB =
S_dB - E_dB. The CM MUST be capable of providing measurements of RxMER for all active subcarrier locations
for a single OFDM downstream channel, using pilots and PLC preamble symbols.




04/19/23                                           CableLabs                                                   187

<!-- PDF_PAGE: 188 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications




          Figure 98 - Computation of Received Modulation Error Ratio (RxMER) for a Given Subcarrier
Performance requirements for downstream RxMER measurements are defined under the following specified
conditions:
      •   Channel center frequency is fixed.
      •   Channel loading consists of a single OFDM channel with no other signals.
      •   OFDM channel being measured has a fixed configuration with a 192 MHz channel bandwidth with 190
          MHz modulated spectrum and no excluded subcarriers other than at band edges.
      •   Channel is flat without impairments other than AWGN.
      •   AWGN level is set to two values giving data-subcarrier CNR = 30 dB and 35 dB at the cable access
          network F connector of the CM across all data subcarriers in the OFDM channel.
      •   Signal level is fixed at a nominal receive level of 6 dBmV per 6 MHz.
      •   A minimum warm-up time of 30 minutes occurs before measurements are made.
      •   Each measurement consists of the frequency average across all subcarriers of the reported time-averaged
          individual subcarrier RxMER values as defined above. Frequency averaging is performed by external
          computation.
      •   An ensemble of M frequency-averaged RxMER measurements (M large enough for reliable statistics, i.e.,
          such that the result lies within a given confidence interval) are taken in succession (e.g., over a period of up
          to 10 minutes) at both CNR values. The mean, RxMER_mean in dB, and standard deviation, RxMER_std
          in dB, are computed over the M measurements at both CNR values. The statistical computations are
          performed directly on the dB values.
The CM MUST provide RxMER measurements with RxMER_std ≤ 0.5 dB under the above specified conditions.
Define delta_RxMER = (RxMER_mean at CNR_data_subcarrier = 35 dB) − (RxMER_mean at
CNR_data_subcarrier = 30 dB). The CM MUST provide RxMER measurements such that 4 dB ≤ delta_RxMER ≤
6 dB under the above specified conditions.



188                                                   CableLabs                                                04/19/23

<!-- PDF_PAGE: 189 -->
Physical Layer Specification                                                            CM-SP-PHYv3.1-I20-230419


9.3.6.1    Signal-to-Noise Ratio (SNR) Margin for Candidate Profile
The purpose of this item is to provide an estimate of the SNR margin available on the downstream data channel with
respect to a candidate modulation profile. The CMTS has the capability described in [DOCSIS MULPIv3.1] section
7.8.1, CM and CMTS Profile Support, in which it sends test data to the CM to measure the performance of a
transition profile. In addition, the CM MUST implement an algorithm to estimate the SNR margin available on the
downstream data channel for a candidate profile. Suggested Algorithm to Compute Signal-to-Noise Ratio (SNR)
Margin for Candidate Profile suggests an algorithm that the CM can use to compute this estimate. The CM only
performs this computation upon request from the CMTS via management message.

9.3.7     Downstream FEC Statistics
The purpose of this item is to monitor downstream link quality via FEC and related statistics. Statistics are taken on
FEC codeword error events, taking into account both the inner LDPC code and outer BCH code. Statistics are
provided on each OFDM channel and for each profile being received by the CM. That is, if the CM is receiving 4
downstream profiles, there will be 4 sets of FEC counters plus a set of counters for the transition profile used for
OFDM Downstream Profile Test (OPT) (see [DOCSIS MULPIv3.1]). For profiles 1-4, statistics for data codewords
include all codewords. For profile 5 (transition profile), statistics for data codewords include either all codewords, if
Codeword Tagging [DOCSIS MULPIv3.1] is disabled; or only codewords marked with T bit = 1 in the NCP, if
Codeword Tagging is enabled. Similar statistics are taken on the NCP and PLC, and on MAC frames.
The CM MUST be capable of providing the following downstream performance metrics on data codewords for each
profile:
    •     Uncorrectables: Number of codewords that failed BCH decoding.
    •     Correctables: Number of codewords that failed pre-decoding LDPC syndrome check and passed BCH
          decoding.
    •     Total number of FEC codewords.
The CM MUST be capable of providing the following downstream performance metrics on Next Codeword Pointer
(NCP) codewords and data fields:
    •     NCP CRC failures: Number of NCP fields that failed CRC check.
    •     Total number of NCP fields.
The CM MUST be capable of providing the following downstream performance metrics on PHY Link Channel
(PLC) codewords:
    •     Unreliable PLC Codewords: Number of PLC codewords that failed LDPC post-decoding syndrome check.
    •     Total number of PLC codewords.
The CM MUST be capable of providing the following downstream performance metrics on Media Access Control
(MAC) frames addressed to the CM for each profile excluding the transition profile:
    •     MAC frame failures: Number of frames that failed MAC CRC check.
    •     Total number of MAC frames.
The CM MUST be capable of providing the following downstream FEC summaries for data codewords on each
OFDM channel for each profile being received by the CM:
    •     Codeword errors vs time (seconds): Number of uncorrectable codewords and total number of codewords in
          each one-second interval for a rolling 10-minute period (600 values).
    •     Codeword errors vs time (minutes): Number of uncorrectable codewords and total number of codewords in
          each one-minute interval for a rolling 24-hour period (1440 values).
There is a UTC (Coordinated Universal Time) timestamp associated with each individual set of Codeword error
statistics. This timestamp allows the user to relate changes in Profile parameters with Codeword error performance,
during the capture period, The 32-bit UTC encodes the date and time with a resolution of 1 second.



04/19/23                                             CableLabs                                                      189

<!-- PDF_PAGE: 190 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


The CM MUST provide two collection and reporting methods for each error-count metric on data codewords:
      •   Long-term statistics. The CM always collects metrics in the background for each profile being received.
          The codeword (or frame) and error counters are long (e.g., 64-bit) integers, so that overflow is not an issue.
          To perform a measurement over a particular time interval, the user reads the counters, waits a period of
          time, reads the counters again, and computes the difference in the counter values.
      •   Short-term statistics. The CM performs a one-shot measurement with two configured parameters, Ne and
          Nc. The CM reports the results when at least Ne errors have occurred or at least Nc codewords have been
          processed, whichever comes first. This measurement is particularly useful for OPT testing of the transition
          profile. To perform this measurement, the CM reads the long-term counters, waits a short time, reads the
          counters again, and computes the difference in the counter values.

9.3.8     Downstream Histogram
The purpose of the downstream histogram is to provide a measurement of nonlinear effects in the channel such as
amplifier compression and laser clipping. For example, laser clipping causes one tail of the histogram to be
truncated and replaced with a spike. The CM MUST be capable of capturing the histogram of time domain samples
at the wideband front end of the receiver (full downstream band). When a CM creates a downstream histogram, the
CM MUST create it such that it is two-sided; that is, it encompasses values from far-negative to far-positive values
of the samples. When the CM creates a downstream histogram, the CM MUST create it such that it has either 256
equally spaced bins with even symmetry about the origin, or 255 equally spaced bins with odd symmetry about the
origin. These bins typically correspond to the 8 MSBs of the wideband analog-to-digital converter (ADC). The
histogram dwell count, a 32-bit unsigned integer, is the number of samples observed while counting hits for a given
bin, and may have the same value for all bins. The histogram hit count, a 32-bit unsigned integer, is the number of
samples falling in a given bin. The CM MUST be capable of reporting the dwell count per bin and the hit count per
bin. When enabled, the CM MUST compute a histogram with a dwell of at least 10 million samples in 30 seconds or
less. With this many samples, the histogram can reliably measure a probability density per bin as low as 10-6 with at
least 10 hits in each bin. The CM MUST continue accumulating histogram samples until it is restarted, disabled,
approaches its 32-bit overflow value, or times out. The CM MUST provide a UTC timestamp, in the header of the
PNM file, to indicate the time of the start of the capture.

9.3.9     Downstream Received Power
The purpose of the downstream received power metric is to measure the average received downstream power in a
set of non-overlapping 6 MHz bands for any DOCSIS 3.0 and 3.1 signals in the receive channel set (RCS) of the
CM including the DOCSIS 3.1 PLC. While digital power measurements are inherently accurate, the measurement
referred to the analog signal at the input F connector depends on the measurement conditions and available
calibration accuracy. The measurements are made along a contiguous set of defined 6 MHz bands. The measurement
bands are designed to align with DOCSIS 3.0 channel locations, so that the total power of a DOCSIS 3.0 single
carrier QAM signal in a 6 MHz bandwidth is measured. For DOCSIS 3.1 OFDM signals, the measurement bands
will also align with the edges of the occupied spectrum of the OFDM channel, although the measurement bands may
not align with the edges of the modulated spectrum of the OFDM channel. In general, a DOCSIS 3.1 OFDM signal
may contain excluded subcarriers within a given 6 MHz measurement band. However, the 6 MHz band containing
the PLC at its center is a special case which contains no excluded subcarriers and contains extra pilots, properties
which make it useful for reliable power measurements. The PLC measurement band may be offset from the 6 MHz
measurement bands due to the location of subcarriers in the PLC placement where the center frequency of the lowest
frequency subcarrier of the 6 MHz encompassed spectrum containing the PLC is an integer when the frequency is
measured in units of MHz.
The CM MUST provide an estimate of the average power for any 6 MHz band in the RCS referenced to the F
connector input of the CM under the following specified received signal conditions:
The measurement band is defined as either of the following:
      •   Any 6 MHz bandwidth with a center frequency of 111 + 6(n-1) MHz for n = 1, ..., 185 (i.e., 111, 117, ...,
          1215 MHz) contained in the RCS of the CM.




190                                                  CableLabs                                               04/19/23

<!-- PDF_PAGE: 191 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419


      •    The 6 MHz bandwidth containing the PLC with a lower channel edge (center frequency of lowest
           subcarrier) frequency of 108 + m MHz for m = 0, 1, ..., 1104 (i.e., 108, 109, ..., 1212 MHz).
All measurements are made under the following conditions:
      •    The measurement bands contain only DOCSIS signals.
      •    The measured bands do not contain any gaps (regions with no signal present) greater than 24 MHz wide
           each from a 108 MHz or 258 MHz lower band edge to a 1002 MHz or 1218 MHz upper band edge.
      •    A constant temperature is maintained during measurements within a range of 20 °C ± 2 °C.
      •    A minimum warm up time of 30 minutes occurs before CM power measurements are made.
      •    The measured 6 MHz band does not contain gaps totaling more than 20 percent of that band.
      •    A maximum 8 dB range of signal input power variation can be measured without allowing recalibration via
           CM re-initialization.
      •    The signal power variation (i.e., the deviation from nominal relative carrier power level) between the
           defined 6 MHz bands containing DOCSIS signals varies up to a maximum of ±3 dB over the full
           downstream band.
      •    The signal tilt and signal power variation are determined using 6 MHz channel power levels as specified in
           section 6.5 of [SCTE RMP] - Nominal Relative Carrier Power Levels and Carrier Level Variations.
      •    The total spectrum of all the gaps is to be less than 20% of the total encompassed spectrum.
The CM MUST provide an average power estimate in any defined 6 MHz measurement bandwidth within ±3 dB of
the actual power at the F connector input under the following conditions:
      •    The power in the defined 6 MHz bands has a maximum tilt of ±1 dB over the entire downstream spectrum.
      •    The power in the defined 6 MHz bands (other than gaps) is in the range of -12 dBmV to +12 dBmV.
The CM MUST provide an average power estimate in any defined 6 MHz measurement band within ±5 dB of the
actual power at the F connector input under the following relaxed signal conditions:
      •    The power in the defined 6 MHz measurement bands has a maximum upward tilt of +4 dB and a maximum
           downward tilt of -9 dB over the entire downstream band.
      •    The power in the defined 6 MHz bands (other than gaps) is in the range of -15 dBmV to +15 dBmV.

9.4       Upstream PNM Requirements
9.4.1      Upstream Capture for Active and Quiet Probe
The purpose of upstream capture is to measure plant response and view the underlying noise floor, by capturing at
least one OFDM symbol during a scheduled active or quiet probe. An active probe provides the partial functionality
of a network analyzer, since the input is known and the output is captured. This permits full characterization of the
linear and nonlinear response of the upstream cable plant. A quiet probe provides an opportunity to view the
underlying noise and ingress while no traffic is being transmitted in the OFDMA band being measured.
The PNM server selects an active CM to analyze by specifying its MAC address, or requests a quiet probe
measurement. The CMTS MUST be capable of selecting a specified transmitting CM, or quiet period when no CMs
are transmitting, for the capture. The CMTS sets up the capture as described in [DOCSIS MULPIv3.1], selecting
either an active SID corresponding to the specified MAC address or the idle SID, and defining an active or quiet
probe. The active probe symbol for this capture normally includes all non-excluded subcarriers across the upstream
OFDMA channel, and normally has pre-equalization off. The quiet probe symbol normally includes all subcarriers,
that is, during the quiet probe time there are no transmissions in the given upstream OFDMA channel. The CMTS
MUST capture samples of a full OFDMA symbol including the guard interval. The CMTS MUST begin the capture
with the first symbol of the specified probe. The sample rate is the FFT sample rate (102.4 Msym/s).




04/19/23                                             CableLabs                                                     191

<!-- PDF_PAGE: 192 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


The CMTS MUST report the list of excluded subcarriers, cyclic prefix length, and transmit window rolloff period in
order to fully define the transmitted waveform. The CMTS MUST report the index of the starting sample used by
the receiver for its FFT. The CMTS MUST report the timestamp corresponding to the beginning of the probe. In the
case where the P-MAPs for the OFDMA upstream being analyzed are being sent in an OFDM downstream, the
timestamp reported is the extended timestamp, while in a case with OFDMA upstream channels but no OFDM
downstream channels, the reported timestamp is the DOCSIS 3.0 timestamp. For an active probe, the CMTS MUST
report the contents of the Probe Information Element (P-IE) message describing that probe.

9.4.2     Upstream Triggered Spectrum Analysis
The upstream triggered spectrum analysis measurement provides a wideband spectrum analyzer function in the
CMTS which can be triggered to examine desired upstream transmissions as well as underlying noise/interference
during a quiet period.
The CMTS MUST provide wideband upstream spectrum analysis capability covering the full upstream spectrum of the
cable plant. The CMTS MUST provide 100 kHz or better resolution (bin spacing) in the wideband upstream spectrum
measurement. Free-running capture is done at a rate supported by the CMTS, and does not imply real-time operation.
The CMTS SHOULD provide the capability to average the FFT bin power of the spectrum over multiple captures.
The CMTS SHOULD provide a variable upstream spectrum analysis span.
The CMTS SHOULD be capable of providing the time-domain input samples as an alternative to the frequency-
domain upstream spectrum results.
In pre-DOCSIS-3.1 mode, the CMTS MUST provide the ability to trigger the spectrum sample capture and perform
spectrum analysis using the following modes:
      •   Free running
      •   Trigger on minislot count
      •   Trigger on SID (service identifier)
      •   Trigger during quiet period (idle SID)
In DOCSIS 3.1 mode, the CMTS MUST provide the ability to trigger spectrum sample capture and perform
spectrum analysis using the following modes:
      •   Free running
      •   A specified timestamp value
      •   A specified MAC address, triggering at the beginning of the first minislot granted to any SID
          corresponding to the specified MAC address
      •   The idle SID, triggering at the beginning of the first minislot granted to that SID
      •   A specified active or quiet probe symbol, triggering at the beginning of the probe symbol

9.4.3     Upstream Impulse Noise Statistics
Upstream impulse noise statistics gather statistics of burst/impulse noise occurring in a selected narrow band as
defined in [DOCSIS CCAP-OSSIv3.1]. A bandpass filter is positioned in an unoccupied upstream band. A threshold
is set, energy exceeding the threshold triggers the measurement of an event, and energy falling below the threshold
ends the event. The CMTS MAY allow the threshold to be set to zero, in which case the average power in the band
will be measured. The measurement is timestamped using the DOCSIS 3.0 field of the 64-bit extended timestamp
(bits 9-40, where bit 0 is the LSB), which provides a resolution of 98 ns and a range of 7 minutes.
The CMTS MUST provide the capability to capture the following statistics in a selected band up to 5.12 MHz wide:
      •   Timestamp of event
      •   Duration of event
      •   Average power of event


192                                                  CableLabs                                           04/19/23

<!-- PDF_PAGE: 193 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


The CMTS MUST provide a time history buffer of up to 1024 events.

9.4.4    Upstream Equalizer Coefficients
This item provides access to CM upstream pre-equalizer coefficients, and CMTS upstream adaptive equalizer
coefficients, which taken together describe the linear response of the upstream cable plant for a given CM. [DOCSIS
CM-OSSIv3.1] specification defines summary metrics to avoid having to send all equalizer coefficients on every
query. During the ranging process, the CMTS computes adaptive equalizer coefficients based on upstream probes;
these coefficients describe the residual channel remaining after any pre-equalization. The CMTS sends these
equalizer coefficients to the CM as a set of Transmit Equalization Adjust coefficients as part of the ranging process.
The CM MUST provide the capability to report its upstream pre-equalizer coefficients (full set or summary) upon
request. The CM MUST provide the capability to also report the most recent set of Transmit Equalization Adjust
coefficients which were applied to produce the reported set of upstream pre-equalizer coefficients. The CM MUST
report a condition in which it modified or did not apply the Transmit Equalization Adjust coefficients sent to it by
the CMTS.
The CMTS MUST provide a capability for reporting its upstream adaptive equalizer coefficients associated with
probes from a CM upon request.

9.4.5    Upstream FEC Statistics
Upstream FEC statistics provide for monitoring upstream link quality via FEC and related statistics. Statistics are
taken on codeword error events. An LDPC codeword that fails post-decoding syndrome check will be labeled
"unreliable", but the data portion of the codeword may not contain bit errors; hence the "unreliable codeword" count
will tend to be pessimistic. All codewords, whether full-length or shortened, are included in the measurements. The
codeword (or frame) and error counters are long (e.g., 64-bit) integers, so that overflow is not an issue.
The CMTS MUST be capable of reporting the upstream FEC statistics and other information listed below for all
upstream data traffic for a selected data IUC on a selected upstream channel for any cable modem served by the
CMTS. Counts reported are current values of counters maintained by the CMTS:
    •    Pre-FEC Error-Free Codewords: Number of codewords that passed pre-decoding syndrome check.
    •    Unreliable Codewords: Number of codewords that failed post-decoding syndrome check.
    •    Corrected Codewords: Number of codewords that failed pre-decoding syndrome check, but passed post-
         decoding syndrome check.
    •    Total number of FEC codewords. The total number of FEC codewords is expected to be the sum of the
         previous three counter values.
    •    Date and time when the counters were read.
Identifier for the cable modem being measured. The CMTS MUST be capable of reporting the upstream MAC CRC
statistics and other information listed below for all upstream data traffic for any cable modem served by the CMTS.
Counts reported are the current values of counters maintained by the CMTS:
    •    MAC CRC failures: Number of frames that failed MAC CRC check.
    •    Total number of MAC frames.
    •    Date and time when the counters were read.
    •    Identifier for the cable modem being measured.
When upstream FEC statistics collection for a 10-minute summary has been enabled for a selected data IUC on a
selected upstreachannel for a selected cable modem, the CMTS MUST report the upstream FEC statistics summaries
and other information listed below for all upstream data traffic for the selected data IUC on the selected upstream
channel for the selected cable modem for the 10 minute period ending when a request is received, i.e., for the most
recent 10-minute time interval, or for the amount of time upstream FEC statistics collection has been enabled for the
cable modem if it has been enabled for less than 10 minutes:




04/19/23                                           CableLabs                                                    193

<!-- PDF_PAGE: 194 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


      •   Total number of seconds of the statistics collection time interval.
      •   Number of errored seconds (seconds during which at least one unreliable codeword occurred).
      •   Count of codeword errors (unreliable codewords) in each 1-second interval (600 values over 10 minutes).
      •   Start time and stop time of the collection time interval.
      •   Identifier for the cable modem being measured.
The CMTS MUST be capable of enabling and disabling upstream FEC statistics collection for the 10-minute
summary.
When upstream FEC statistics collection for a 24-hour summary has been enabled for a selected data IUC on a
selected upstream channel for a selected cable modem, the CMTS MUST report the upstream FEC statistics
summaries and other information listed below for all upstream data traffic for the selected data IUC on the selected
upstream channel for the selected cable modem for the 24-hour time interval ending when the request is received,
i.e., for the most recent 24-hour time interval, or for the amount of time upstream FEC statistics collection has been
enabled for the cable modem if it has been enabled for less than 24 hours:
      •   Total number of minutes of the statistics collection time interval.
      •   Number of errored minutes (minutes during which at least one unreliable codeword occurred).
      •   Count of codeword errors (unreliable codewords) in each 1-minute interval (1440 values over 24 hours).
      •   Start time and stop time of the collection time interval.
      •   Identifier for the cable modem being measured.
The CMTS MUST be capable of enabling and disabling upstream FEC statistics collection for the 24-hour
summary.
When upstream MAC CRC statistics collection for a 10-minute summary has been enabled for a selected cable
modem, the CMTS MUST report the upstream MAC CRC statistics summaries and other information listed below
for all upstream data traffic for the selected cable modem for the 10-minute period ending when a request is
received, i.e., for the most recent 10-minute time interval, or for the amount of time upstream MAC CRC statistics
collection has been enabled for the cable modem if it has been enabled for less than 10 minutes:
      •   Total number of seconds of the statistics collection time interval.
      •   Number of errored seconds (seconds during which at least one errored CRC frame occurred).
      •   Count of MAC CRC errored frames in each 1-second interval (600 values over 10 minutes).
      •   Count of MAC CRC frames in each 1-second interval (600 values over 10 minutes).
      •   Start time and stop time of the collection time interval.
      •   Identifier for the cable modem being measured.
The CMTS MUST be capable of enabling and disabling upstream MAC CRC statistics collection for the 10-minute
summary.
When upstream MAC CRC statistics collection for a 24-hour summary has been enabled for a selected cable
modem, the CMTS MUST report the upstream MAC CRC statistics summaries and other information listed below
for all upstream data traffic for the cable modem for the 24-hour period ending when a request is received, i.e., for
the most recent 24-hour time interval, or for the amount of time upstream MAC CRC statistics collection has been
enabled for the cable modem if it has been enabled for less than 24 hours:
      •   Total number of minutes of the statistics collection time interval.
      •   Number of errored minutes (minutes during which at least one errored CRC frame occurred).
      •   Count of MAC CRC errored frames in each 1-minute interval (1440 values over 24 hours).
      •   Count of MAC CRC frames in each 1-minute interval (1440 values over 24 hours),


194                                                   CableLabs                                             04/19/23

<!-- PDF_PAGE: 195 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


    •    Start time and stop time of the collection time interval.
    •    Identifier for the cable modem being measured.
The CMTS MUST be capable of enabling and disabling upstream MAC CRC statistics collection for the 24-hour
summary.

9.4.6    Upstream Histogram
The purpose of the upstream histogram is to provide a measurement of nonlinear effects in the channel such as
amplifier compression and laser clipping. For example, laser clipping causes one tail of the histogram to be
truncated and replaced with a spike. The CMTS MUST be capable of capturing the histogram of time domain
samples at the wideband front end of the receiver (full upstream band). When the CMTS creates an upstream
histogram, the CMTS MUST create it such that it is a two-sided histogram; that is, it encompasses values from far-
negative to far-positive values of the samples. When a CMTS creates an upstream histogram, the CMTS MUST
create it such that is has either 256 equally spaced bins with even symmetry about the origin, or 255 equally spaced
bins with odd symmetry about the origin. These bins typically correspond to the 8 MSBs of the wideband analog-to-
digital converter (ADC). The histogram dwell count, a 32-bit unsigned integer, is the number of samples observed
while counting hits for a given bin, and may have the same value for all bins. The histogram hit count, a 32-bit
unsigned integer, is the number of samples falling in a given bin. The CMTS MUST be capable of reporting the
dwell count per bin and the hit count per bin. When enabled, the CMTS MUST compute a histogram with a dwell of
at least 10 million samples in 30 seconds or less. With this many samples, the histogram can reliably measure a
probability density per bin as low as 10-6 with at least 10 hits in each bin. The CMTS MUST continue accumulating
histogram samples until it is restarted, disabled, approaches its 32-bit overflow value, or times out. The CMTS
MUST provide a UTC timestamp, in the header of the PNM file, to indicate the time of the start of the histogram
capture.

9.4.7    Upstream Channel Power
The purpose of the upstream channel power metric is to provide an estimate of the total received power in a specified
OFDMA channel at the F connector input of the CMTS line card, or other agreed measurement point, for a given
user. The measurement is based on upstream probes, which are typically the same probes used for pre-equalization
adjustment. While digital power measurements are inherently accurate, the measurement referred to the analog input
depends on available calibration accuracy.
The CMTS MUST provide an estimate of total received power in a specified OFDMA channel at a reference input
point, for a single specified upstream user. The CMTS MUST provide configurable averaging over a range of at
least 1 to 32 probes.
The CMTS MUST provide upstream power measurements with a standard deviation of 0.33 dB or better under the
following test conditions:
    •    Center frequency is fixed.
    •    Probe being measured has a fixed configuration containing at least 256 active subcarriers for 4K FFT, and
         at least 200 active subcarriers for 2K FFT.
    •    Channel is without impairments other than AWGN at 25 dB CNR.
    •    Signal level is fixed at a value within ± 6 dB relative to a nominal receive level of 0 dBmV.
    •    A minimum warm up time of 5 minutes occurs before power measurements are made.
    •    Averaging is set to N = 8 probes per measurement.
    •    M measurements (M large enough for reliable statistics) are taken in succession (e.g., over a period of up to
         10 minutes). The standard deviation is computed over the M measurements, where each measurement is the
         average of N probes.




04/19/23                                             CableLabs                                                  195

<!-- PDF_PAGE: 196 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


9.4.8     Upstream Receive Modulation Error Ratio (RxMER) per Subcarrier
This item provides measurements of the upstream receive modulation error ratio (RxMER) for each subcarrier. The
CMTS measures the RxMER using an upstream probe, which is not subject to symbol errors as data subcarriers
would be. The probes used for RxMER measurement are typically distinct from the probes used for pre-equalization
adjustment. For the purposes of this measurement, RxMER is defined as the ratio of the average power of the ideal
BPSK constellation to the average error-vector power. The error vector is the difference between the equalized
received probe value and the known correct probe value.
The CMTS MUST be capable of providing measurements of RxMER for all active subcarriers for any single
specified user in a specified OFDMA upstream channel, using probe symbols. A sufficient number of upstream
probe symbols should be used for a reliable estimate of RxMER.
Performance requirements for upstream RxMER measurements are defined under the following specified
conditions:
      •   Channel loading consists of a single upstream OFDMA channel with no other signals.
      •   OFDMA channel being measured has a fixed configuration with a 95 MHz channel bandwidth with
          95 MHz modulated spectrum and no excluded subcarriers other than at band edges.
      •   Channel is flat without impairments other than AWGN.
      •   AWGN level is set to two values giving data-subcarrier CNR = 30 dB and 35 dB at the cable access
          network F connector of the CMTS receiver across all data subcarriers in the OFDMA channel.
      •   Signal level is fixed at a nominal receive level of 10 dBmV per 6.4 MHz.
      •   A minimum warm-up time of 30 minutes occurs before measurements are made.
      •   Measurement is done using 8-symbol RxMER probes with a skip value of 0 (non-staggered probes).
      •   Each measurement consists of the frequency average across all subcarriers of the reported time-averaged
          individual subcarrier RxMER values in dB, where time averaging is over the 8 symbols in a single probe.
          Frequency averaging can be provided by the OFDMA receiver or performed by external computation.
To gather statistics for a test, an ensemble of M of the above frequency- and time-averaged RxMER measurements
(M large enough for reliable statistics, i.e., such that the result lies within a given confidence interval) are taken in
succession (e.g., over a period of up to 10 minutes) at both CNR values. The mean, RxMER_mean in dB, and
standard deviation, RxMER_std in dB, are computed over the M measurements at both CNR values. The statistical
computations are performed directly on the dB values.
The CMTS MUST provide RxMER measurements with RxMER_std ≤ 0.5 dB under the above specified conditions.
Define delta_RxMER = (RxMER_mean at CNR_data_subcarrier = 35 dB) − (RxMER_mean at
CNR_data_subcarrier = 30 dB). The CMTS MUST provide RxMER measurements such that 4 dB ≤ delta_RxMER
≤ 6 dB under the above specified conditions.




196                                                   CableLabs                                                04/19/23
