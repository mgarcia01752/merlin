# 3 - Terms and Definitions

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 19-29
Classification: Main body (definitions)

<!-- PDF_PAGE: 19 -->
Physical Layer Specification                                                                CM-SP-PHYv3.1-I20-230419


3 TERMS AND DEFINITIONS
This specification uses the following terms.
  100% Grant                       The amount of Occupied Bandwidth in the TCS (see Occupied Bandwidth).
  Active Channel                   Any channel which has been assigned to a cable modem's transmit channel set either in
                                   a registration response message or a dynamic bonding request message, and prior to
                                   registration. After registration, the set of active channels also is called the transmit
                                   channel set. If the CMTS needs to add, remove, or replace channels in the cable
                                   modem's transmit channel set, it uses the dynamic bonding request message with
                                   transmit channel configuration encodings to define the de sired new transmit channel set.
                                   Note that the set of channels actually bursting upstream from a cable modem is a subset
                                   of that cable modem's active channels. In many instances one or all of a cable modem's
                                   active channels will not be bursting, but such quiet channels are still considered active
                                   channels for that cable modem.
  Active Subcarrier                1) In a downstream OFDM channel, any subcarrier other than an excluded subcarrier. 2)
                                   In an upstream OFDMA channel, any subcarrier other than an excluded subcarrier
                                   (subcarriers in zero-valued minislots as defined in OFDMA profiles, and unused
                                   subcarriers are considered active subcarriers because they are used in probes).
  Adaptive Equalizer               A circuit in a digital receiver that compensates for channel response impairments. In
                                   effect, the circuit creates a digital filter that has approximately the opposite complex
                                   frequency response of the channel through which the desired signal was transmitted.
  Adaptive Equalizer Tap           See tap.
  Adaptive Pre-Equalizer           A circuit in a DOCSIS 1.1 or newer cable modem that pre-equalizes or pre-distorts the
                                   transmitted upstream signal to compensate for channel response impairments. In effect,
                                   the circuit creates a digital filter that has approximately the opposite complex frequency
                                   response of the channel through which the desired signal is to be transmitted.
  Additive White Gaussian          See thermal noise.
  Noise
  Availability                     The ratio of time that a service, device, or network is available for use to total time,
                                   usually expressed as a percentage of the total time. For example, four-nines availability,
                                   written as 99.99%, means that a service is available 8759.12 hours out of 8760 total
                                   hours in a year.
  BCH                              A class of error correction codes named after the inventors Raj Bose, D. K. Ray-
                                   Chaudhuri, and Alexis Hocquenghem.
  Binary Phase Shift Keying        A form of digital modulation in which two phases separated by 180 degrees support the
                                   transmission of one bit per symbol.
  Bit Error Rate                   See bit error ratio.
  Bit Error Ratio                  The ratio of errored bits to the total number of bits transmitted, received, or processed
                                   over a defined amount of time. Mathematically, BER = (number of errored bits)/(total
                                   number of bits) or BER = (error count in measurement period)/(bit rate * measurement
                                   period). Usually expressed in scientific notation format. Also called bit error rate.
  Bit Loading                      The technique of assigning the optimum number of bits (modulation order) for
                                   transmission per subcarrier in an OFDM or OFDMA system.
  Burst                            A single continuous RF signal from the cable modem upstream transmitter, from
                                   transmitter on to transmitter off.
  Burst Noise                      1) Another name for impulse noise. 2) A type of noise comprising random and sudden
                                   step-like changes between levels, often occurring in semiconductors. Sometimes called
                                   popcorn noise.
  Cable Modem                      A modulator-demodulator at the subscriber premises intended for use in conveying data
                                   communications on a cable television system.
  Cable Modem Termination          A device located at the cable television system headend or distribution hub, which
  System                           provides complementary functionality to the cable modems to enable data connectivity to
                                   a wide-area network.




04/19/23                                              CableLabs                                                                19

<!-- PDF_PAGE: 20 -->
CM-SP-PHYv3.1-I20-230419                                   Data-Over-Cable Service Interface Specifications


 Carrier-To-Noise Ratio    1) The ratio of signal (or carrier) power to noise power in a defined measurement
                           bandwidth.
                           2) For OFDM and OFDMA signals, the ratio of average signal power (PSIGNAL) in the
                           occupied bandwidth to the average noise power in the occupied bandwidth given by the
                           noise power spectral density integrated over the same occupied bandwidth, expressed
                           mathematically as CNR = 10log10 [PSIGNAL/∫ N(f)df] dB. Note: This is a lower bound on the
                           actual received signal-to-noise ratio.
                           3) For SC-QAM, the ratio of the average signal power (PSIGNAL) to the average noise
                           power in the QAM signal's symbol rate bandwidth (NSYM), and expressed mathematically
                           as CNR = 10log10(PSIGNAL/NSYM) dB or equivalently for an AWGN channel as CNR =
                           10log10(ES/N0) dB.
                           Note: For an AWGN channel, PSIGNAL/NSYM = (ES/TS)/(N0 BN) = (ES/TS)/(N0/TS) = ES/N0,,
                           where ES and TS are the symbol energy and duration respectively, N0 is the noise power
                           spectral density, and BN is the noise bandwidth equal to the symbol rate bandwidth 1/TS.
                           4) For analog television signals, the ratio of visual carrier peak envelope power during the
                           transmission of synchronizing pulses (PPEP) to noise power (N), where the visual carrier
                           power measurement bandwidth is nominally 300 kHz and the noise power measurement
                           bandwidth is 4 MHz for NTSC signals. For the latter, the noise measurement bandwidth
                           captures the total noise power present over a 4 MHz band centered within the television
                           channel, and is expressed mathematically as CNR = 10log10(PPEP/N) dB. Note: For
                           analog PAL and SECAM channels, the noise measurement bandwidth is a larger value
                           than the 4 MHz specified for NTSC (4.75 MHz, 5.00 MHz, 5.08 MHz, or 5.75 MHz,
                           depending on the specific system).
 CEA-542                   A Consumer Electronics Association standard that defines a channel identification plan
                           for 6 MHz-wide channel frequency allocations in cable systems.
 Ceiling                   A mathematical function that returns the lowest-valued integer that is greater than or
                           equal to a given value.
 Channel                   A portion of the electromagnetic spectrum used to convey one or more RF signals
                           between a transmitter and receiver. May be specified by parameters such as center
                           frequency, bandwidth, or CEA channel number.
 Codeword                  Forward error correction data block, comprising a combination of information bytes and
                           parity bytes.
 Codeword Error Ratio      The ratio of errored codewords to the total number of codewords transmitted, received, or
                           processed over a defined amount of time. Mathematically, CER = (number of errored
                           codewords)/(total number of codewords). Usually expressed in scientific notation format.
 Coefficient               Complex number that establishes the gain of each tap in an adaptive equalizer or
                           adaptive pre-equalizer.
 Common Path Distortion    Clusters of second and third order distortion beats generated in a diode-like nonlinearity
                           such as a corroded connector in the signal path common to downstream and upstream.
                           The beats tend to be prevalent in the upstream spectrum. When the primary RF signals
                           are digitally modulated signals instead of analog TV channels, the distortions are noise-
                           like rather than clusters of discrete beats.
 Complementary Pilots      Subcarriers that carry data, but with a lower modulation order than other data subcarriers
                           in a given minislot. Complementary pilots allow phase tracking along the time axis for
                           frequency offset and phase noise correction, and may be used by the CMTS upstream
                           receiver to enhance signal processing, such as improving the accuracy of center
                           frequency offset acquisition.
 Composite Second Order    Clusters of second order distortion beats generated in cable network active devices that
                           carry multiple RF signals. When the primary RF signals are digitally modulated signals
                           instead of analog TV channels, the distortions are noise-like rather than clusters of
                           discrete beats.
 Composite Triple Beat     Clusters of third order distortion beats generated in cable network active devices that
                           carry multiple RF signals. When the primary RF signals are digitally modulated signals
                           instead of analog TV channels, the distortions are noise-like rather than clusters of
                           discrete beats.
 Continuous Pilots         Pilots that occur at the same frequency location in every OFDM symbol, and which are
                           used for frequency and phase synchronization.
 Convolution               A process of combining two signals in which one of the signals is time-reversed and
                           correlated with the other signal. The output of a filter is the convolution of its impulse
                           response with the input signal.




20                                            CableLabs                                                         04/19/23

<!-- PDF_PAGE: 21 -->
Physical Layer Specification                                                              CM-SP-PHYv3.1-I20-230419


  Convolutional Interleaver    An interleaver in which symbols are sequentially shifted into a bank of "I" registers. Each
                               successive register has "J" symbols more storage than the preceding register. The first
                               interleaver path has zero delay, the second has a J symbol period of delay, the third 2 x J
                               symbol periods of delay, etc., up to the Ith path which has (I−1) x J symbol periods of
                               delay. This process is reversed in the receiver's deinterleaver so that the net delay of
                               each symbol is the same through the interleaver and deinterleaver. See also interleaver.
  Correlation                  1) A process of combining two signals in which the signals are multiplied sample-by-
                               sample and summed; the process is repeated at each sample as one signal is slid in time
                               past the other.
                               2) Cross-correlation is a measure of similarity between two signals.
  Cross Modulation             A form of television signal distortion in which modulation from one or more television
                               signals is imposed on another signal or signals.
  Customer Premises            Device such as a cable modem or set-top at the subscriber's or other end user's location.
  Equipment                    May be provided by the end user or the service provider.
  Cyclic Prefix                A copy of the end of a symbol that is added to the beginning of the same symbol, in order
                               to help mitigate the effects of micro-reflections and similar impairments.
  Data-Subcarrier CNR          The ratio of the time-average power of a single data subcarrier to the underlying noise
                               power, with the noise measured in a bandwidth equal to the nominal subcarrier spacing.
  Decibel                      Ratio of two power levels expressed mathematically as dB = 10log10(P1/P2).
  Decibel Carrier              Ratio of the power of a signal to the power of a reference carrier, expressed
                               mathematically as dBc = 10log10(Psignal/Pcarrier).
  Decibel Millivolt            Unit of RF power expressed in terms of voltage, defined as decibels relative to 1 millivolt,
                               where 1 millivolt equals 13.33 nanowatts in a 75 ohm impedance. Mathematically,
                               dBmV = 20log10(value in mV/1 mV).
  Decibel Reference            Ratio of a signal level to a reference signal level, When the signals are noise or noise-
                               like; the density units measurement bandwidth for the two signals is the same. When
                               both signal levels are in the same units of power, the ratio is expressed mathematically
                               as dBr = 10log10(Psignal/Preference). When both signal levels are in the same units of voltage,
                               assuming the same impedance, the ratio is expressed mathematically as
                               dBr = 20log10(Vsignal/Vreference).
  Discrete Fourier Transform   Part of the family of mathematical methods known as Fourier analysis, which defines the
                               "decomposition" of signals into sinusoids. Discrete Fourier transform defines the
                               transformation from the time to the frequency domain. See also inverse discrete Fourier
                               transform.
  Distortion                   See linear distortion and nonlinear distortion.
  Distribution Hub             A facility in a cable network which performs the functions of a headend for customers in
                               its immediate area, and which receives some or all of its content for transmission from a
                               master headend in the same metropolitan or regional area.
  DOCSIS                       Data-Over-Cable Service Interface Specifications. A group of specifications that defines
                               interoperability between cable modem termination systems and cable modems.
  DOCSIS 1.x                   Abbreviation for DOCSIS versions 1.0 or 1.1.
  Downstream                   1) The direction of RF signal transmission from headend or hub site to subscriber. In
                               North American cable networks, the downstream or forward spectrum may occupy
                               frequencies from just below 54 MHz to as high as 1002 MHz or more.
                               2) The DOCSIS 3.1 downstream is 258 MHz (optional 108 MHz) to 1218 MHz (optional
                               1794 MHz).
  Downstream Channel           A portion of the electromagnetic spectrum used to convey one or more RF signals from
                               the headend or hub site to the subscriber premises. For example, a single CEA channel's
                               bandwidth is 6 MHz, and a downstream DOCSIS 3.1 OFDM channel's bandwidth can be
                               up to 192 MHz.
  Drop                         Coaxial cable and related hardware that connects a residence or other service location to
                               a tap in the nearest coaxial feeder cable. Also called drop cable or subscriber drop.
  Dynamic Host Configuration   A protocol that defines the dynamic or temporary assignment of Internet protocol
  Protocol                     addresses, so that the addresses may be reused when they are no longer needed by the
                               devices to which they were originally assigned.
  Dynamic Range Window         1) DOCSIS 3.0 - The range, in decibels, of the maximum power difference between
                               multiple transmitters in a cable modem's Transmit Channel Set.
                               2) DOCSIS 3.1 - The range, in decibels, of the maximum difference in power per 1.6 MHz
                               between multiple transmitters in a cable modem's Transmit Channel Set.




04/19/23                                          CableLabs                                                                     21

<!-- PDF_PAGE: 22 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications


 Encompassed Spectrum          1) For an OFDM or OFDMA channel, the range of frequencies from the center frequency
                               of the channel's lowest active subcarrier minus half the subcarrier spacing, to the center
                               frequency of the channel's highest active subcarrier plus half the subcarrier spacing.
                               2) For an SC-QAM channel, the encompassed spectrum is the signal bandwidth (i.e., 6
                               MHz or 8 MHz in the downstream; 1.6 MHz, 3.2 MHz, and 6.4 MHz in the upstream).
                               3) For the RF output of a downstream or upstream port including multiple OFDM,
                               OFDMA, and/or SC-QAM channels, the range of frequencies from the lowest frequency
                               of the encompassed spectrum of the lowest frequency channel to the highest frequency
                               of the encompassed spectrum of the highest frequency channel.
 Equalizer Tap                 See tap.
 Equivalent Legacy DOCSIS      Within a downstream OFDM channel, an integer number equal to ceil(modulated
 Channels                      spectrum in MHz / 6).
 Excluded Subcarrier           Subcarrier that cannot be used because another type of service is using the subcarrier's
                               frequency or a permanent ingressor is present on the frequency. The CMTS or cable
                               modem is administratively configured to not transmit on excluded subcarriers.
 Exclusion Band                A set of contiguous subcarriers within the OFDM or OFDMA channel bandwidth that are
                               set to zero-value by the transmitter to reduce interference to other co-existing
                               transmissions such as legacy SC-QAM signals.
 F Connector                   A threaded, nominally 75-ohm impedance RF connector, whose electrical and physical
                               specifications are defined in various SCTE standards. Commonly used on smaller sizes of
                               coaxial cable such as 59-series and 6-series, and on mating interfaces of subscriber drop
                               components, customer premises equipment, and some headend and test equipment.
 Fast Fourier Transform        An algorithm to compute the discrete Fourier transform from the time domain to the
                               frequency domain, typically far more efficiently than methods such as correlation or
                               solving simultaneous linear equations. See also discrete Fourier transform, inverse
                               discrete Fourier transform, and inverse fast Fourier transform.
 FFT Duration                  Reciprocal of subcarrier spacing. For example, with 50 kHz subcarrier spacing, FFT
                               duration is 20 µs, and with 25 kHz subcarrier spacing, FFT duration is 40 µs. Sometimes
                               called "useful symbol duration." See also symbol duration.
 Fiber Node                    See node.
 Filler Subcarrier             A zero-bit-loaded subcarrier that is inserted in an OFDM symbol when no data is
                               transmitted, or when the number of codewords has exceeded the upper limit, or when it is
                               not possible to begin a new codeword because of insufficient space to include a next
                               codeword pointer.
 Floor                         A mathematical function that returns the highest-valued integer that is less than or equal
                               to a given value.
 Forward                       See downstream.
 Forward Error Correction      A method of error detection and correction in which redundant information is sent with a
                               data payload in order to allow the receiver to reconstruct the original data if an error
                               occurs during transmission.
 Frequency Division Multiple   A multiple access technology that accommodates multiple users by allocating each user's
 Access                        traffic to one or more discrete frequency bands, channels, or subcarriers.
 Frequency Division            The transmission of multiple signals through the same medium at the same time. Each
 Multiplexing                  signal is on a separate frequency or assigned to its own channel. For example, an analog
                               TV signal might be carried on CEA channel 7 (174 MHz-180 MHz), a 256-QAM digital
                               video signal on channel 8 (180-186 MHz), and so on.
 Frequency Response            A complex quantity describing the flatness of a channel or specified frequency range, and
                               which has two components: amplitude (magnitude)-versus-frequency, and phase-versus-
                               frequency.
 Gigahertz                     One billion (109) hertz. See also hertz.
 Group Delay                   The negative derivative of phase with respect to frequency, expressed mathematically as
                               GD = -(dφ/dω) and stated in units of time such as nanoseconds or microseconds.
 Group Delay Ripple            Group delay variation which has a sinusoidal or scalloped sinusoidal shape across a
                               specified frequency range.
 Group Delay Variation or      The difference in group delay between one frequency and another in a circuit, device, or
 Group Delay Distortion        system.
 Guard Interval                In the time domain, the period from the end of one symbol to the beginning of the next
                               symbol, which includes the cyclic prefix and applied transmit windowing. Also called
                               guard time.




22                                               CableLabs                                                      04/19/23

<!-- PDF_PAGE: 23 -->
Physical Layer Specification                                                                  CM-SP-PHYv3.1-I20-230419


  Guard Band                        A narrow range of frequencies in which user data is not transmitted, located at the lower
                                    and upper edges of a channel, at the lower and upper edges of a gap within a channel, or
                                    in between channels. A guard band minimizes interference from adjacent signals, but is
                                    not needed in the case of adjoining OFDM channels that are synchronous with identical
                                    FFT size and cyclic prefix that would not mutually interfere.
  Harmonic Related Carriers         A method of spacing channels on a cable television system defined in [CEA-542], in
                                    which visual carriers are multiples of 6.0003 MHz. A variation of HRC channelization
                                    used in some European cable networks is based upon multiples of 8 MHz.
  Headend                           A central facility that is used for receiving, processing, and combining broadcast,
                                    narrowcast and other signals to be carried on a cable network. Somewhat analogous to a
                                    telephone company's central office. Location from which the DOCSIS cable plant fans out
                                    to subscribers. See also distribution hub.
  Header                            Protocol control information located at the beginning of a protocol data unit.
  Hertz                             A unit of frequency equivalent to one cycle per second.
  Hum Modulation                    Amplitude distortion of a signal caused by the modulation of that signal by components of
                                    the power source (e.g., 60 Hz) and/or its harmonics.
  Hybrid Fiber/Coax                 A broadband bidirectional shared-media transmission system or network architecture
                                    using optical fibers between the headend and fiber nodes, and coaxial cable distribution
                                    from the fiber nodes to the subscriber locations.
  Impedance                         The combined opposition to current in a component, circuit, device, or transmission line
                                    that contains both resistance and reactance. Represented by the symbol Z and
                                    expressed in ohms.
  Impulse Noise                     Noise that is bursty in nature, characterized by non-overlapping transient disturbances.
                                    May be periodic (e.g., automobile ignition noise or high-voltage power line corona noise),
                                    or random (e.g., switching noise or atmospheric noise from thunderstorms). It is generally
                                    of short duration - from about 1 microsecond to a few tens of microseconds - with a fast
                                    risetime and moderately fast falltime.
  Incremental Related Carriers      A method of spacing channels on a cable television system defined in [CEA-542], in
                                    which all visual carriers except channels 5 and 6 are offset +12.5 kHz with respect to the
                                    standard channel plan. Channels 5 and 6 are offset +2.0125 MHz with respect to the
                                    standard channel plan. See also standard frequencies.
  In-Phase                          The real part of a vector that represents a signal, with 0 degrees phase angle relative to a
                                    reference carrier. See also quadrature (Q).
  Interleaver                       A subset or layer of the forward error correction process, in which the data to be
                                    transmitted is rearranged or mixed such that the original bits, bytes, or symbols are no
                                    longer adjacent. The latter provides improved resistance to various forms of interference,
                                    especially burst or impulse noise. Interleaving may be performed in the time domain,
                                    frequency domain, or both. A de-interleaver in the receiver rearranges the bits, bytes, or
                                    symbols into their original order prior to additional error correction.
  International Electrotechnical    An organization that prepares and publishes international standards for electrical,
  Commission                        electronic and related technologies.
  International Organization for    An organization that develops voluntary international standards for technology, business,
  Standardization                   manufacturing, and other industries.
  Internet Engineering Task Force   A body responsible, among other things, for developing standards used in the Internet.
  Internet Protocol                 A network layer protocol that supports connectionless internetwork service, and which
                                    contains addressing and control information that allows packets to be routed. Widely
                                    used in the public Internet as well as private networks. The vast majority of IP devices
                                    support IP version 4 (IPv4) defined in RFC-791, although support for IP version 6 (IPv6,
                                    RFC-2460) continues to increase.
  Internet Protocol Detail Record   The record formatter and exporter functions of the CMTS that provides information about
                                    Internet protocol-based service usage, and other activities that can be used by
                                    operational support systems and business support systems.
  Inverse Discrete Fourier          Part of the family of mathematical methods known as Fourier analysis, which defines the
  Transform                         "decomposition" of signals into sinusoids. Inverse discrete Fourier transform defines the
                                    transformation from the frequency to the time domain. See also discrete Fourier transform.
  Inverse Fast Fourier              An algorithm to compute the inverse discrete Fourier transform from the frequency
  Transform                         domain to the time domain, typically far more efficiently than methods such as correlation
                                    or solving simultaneous linear equations. See also discrete Fourier transform, fast Fourier
                                    transform, and inverse discrete Fourier transform.
  Jitter                            The fluctuation in the arrival time of a regularly scheduled event such as a clock edge or
                                    a packet in a stream of packets. Jitter is defined as fluctuations above 10 Hz.




04/19/23                                              CableLabs                                                                 23

<!-- PDF_PAGE: 24 -->
CM-SP-PHYv3.1-I20-230419                                     Data-Over-Cable Service Interface Specifications


 Kilohertz                   One thousand (103) hertz. See also hertz.
 Latency                     1) The time taken for a signal element to propagate through a transmission medium or
                             device.
                             2) The delay between a device's request for network access and when permission is
                             granted for transmission.
                             3) The delay from when a frame is received by a device to when the frame is forwarded
                             via the device's destination port.
 Layer                       One of seven subdivisions of the Open System Interconnection reference model.
 Linear Distortion           A class of distortions that occurs when the overall response of the system (including
                             transmitter, cable plant, and receiver) differs from the ideal or desired response. This
                             class of distortions maintains a linear, or 1:1, signal-to-distortion relationship (increasing
                             signal by 1 dB causes distortion to increase by 1 dB), and often occurs when amplitude-
                             versus-frequency and/or phase-versus-frequency depart from ideal. Linear distortions
                             include impairments such as micro-reflections, amplitude ripple, and group delay
                             variation, and can be corrected by an adaptive equalizer.
 Low Density Parity Check    An error correction code used in DOCSIS 3.1. LDPC is more robust than Reed-Solomon
                             error correction codes.
 MAC Address                 The "built-in" hardware address of a device connected to a shared medium.
 MAC Frame                   MAC header plus optional protocol data unit.
 MAC Management Message      Unclassified traffic between the CMTS and cable modem. Examples include MAC
                             domain descriptor, ranging-request, ranging-response, and upstream channel descriptor
                             messages.
 Media Access Control        A sublayer of the Open Systems Interconnection model's data link layer (Layer 2), which
                             manages access to shared media such as the Open Systems Interconnection model's
                             physical layer (Layer 1).
 Megahertz                   One million (106) hertz. See also hertz.
 Micro-reflection            A short time delay echo or reflection caused by an impedance mismatch. A micro-
                             reflection's time delay is typically in the range from less than a symbol period to several
                             symbol periods.
 Microsecond                 One millionth (10-6) of a second
 Millisecond                 One thousandth (10-3) of a second
 Millivolt                   One thousandth (10-3) of a volt
 Minislot                    In DOCSIS 3.0 and earlier TDMA applications, a unit of time for upstream transmission
                             that is an integer multiple of 6.25 μs units of time called "ticks." In DOCSIS 3.1 OFDMA
                             applications, a group of dedicated subcarriers, all with the same modulation order, for
                             upstream transmission by a given cable modem. For both TDMA and OFDMA, a cable
                             modem may be assigned one or more minislots in a transmission burst.
 Modulated Spectrum          1) Downstream modulated spectrum - Encompassed spectrum minus the excluded
                             subcarriers within the encompassed spectrum, where excluded subcarriers include all the
                             individually excluded subcarriers and all the subcarriers comprising excluded sub-bands.
                             This also is the spectrum comprising all active subcarriers. Note: For this definition, the
                             width of an active or excluded subcarrier is equal to the subcarrier spacing.
                             2) Upstream modulated spectrum - The spectrum comprising all non-zero-valued
                             subcarriers of a cable modem's OFDMA transmission, resulting from the exercised
                             transmit opportunities. Note: For this definition, the width of a transmitted subcarrier is
                             equal to the subcarrier spacing.
 Modulation Error Ratio      The ratio of average signal constellation power to average constellation error power - that
                             is, digital complex baseband signal-to-noise ratio - expressed in decibels. In effect, MER
                             is a measure of how spread out the symbol points in a constellation are. More
                             specifically, MER is a measure of the cluster variance that exists in a transmitted or
                             received waveform at the output of an ideal receive matched filter. MER includes the
                             effects of all discrete spurious, noise, carrier leakage, clock lines, synthesizer products,
                             linear and nonlinear distortions, other undesired transmitter and receiver products,
                             ingress, and similar in-channel impairments.
 Modulation Rate             The signaling rate of the upstream modulator (for example, 1280 to 5120 kHz). In S-
                             CDMA it is the chip rate. In TDMA, it is the channel symbol rate.
 Multiple Transmit Channel   Operational mode in a cable modem that enables the simultaneous transmission of more
 [Mode]                      than one upstream channel. With MTC Mode enabled, the CM and CMTS use Queue
                             Depth Based Requesting and Continuous Concatenation and Fragmentation. DOCSIS
                             3.1 Cable Modems require MTC Mode to be enabled in Registration.
 Nanosecond                  One billionth (10-9) of a second.



24                                              CableLabs                                                         04/19/23

<!-- PDF_PAGE: 25 -->
Physical Layer Specification                                                             CM-SP-PHYv3.1-I20-230419


  National Television System   The committee that defined the analog television broadcast standards (black and white in
  Committee                    1941, color in 1953) used in North America and some other parts of the world. The NTSC
                               standards are named after the committee.
  Next Codeword Pointer        A message block used to identify where a codeword begins.
  Node                         An optical-to-electrical (RF) interface between a fiber optic cable and the coaxial cable
                               distribution network. Also called fiber node.
  Noise                        Typically any undesired signal or signals-other than discrete carriers or discrete distortion
                               products-in a device, communications circuit, channel or other specified frequency range.
                               See also impulse noise, phase noise, and thermal noise.
  Nonlinear Distortion         A class of distortions caused by a combination of small signal nonlinearities in active
                               devices and by signal compression that occurs as RF output levels reach the active
                               device's saturation point. Nonlinear distortions generally have a nonlinear signal-to-
                               distortion amplitude relationship-for instance, 1:2, 1:3 or worse (increasing signal level by
                               1 dB causes distortion to increase by 2 dB, 3 dB, or more). The most common nonlinear
                               distortions are even order distortions such as composite second order, and odd order
                               distortions such as composite triple beat. Passive components can generate nonlinear
                               distortions under certain circumstances.
  Occupied Bandwidth           1) Downstream - The sum of the bandwidth in all standard channel frequency allocations
                               (e.g., 6 MHz spaced CEA channels) that are occupied by the OFDM channel. The CEA
                               channels which are occupied by the OFDM signal are those which contain any of the
                               Modulated Spectrum and/or taper region shaped by the OFDM channels' transmit
                               windowing, where the values for the taper regions are defined in Appendix V as a
                               function of the Roll-Off Period. It is possible, but not problematic, for a CEA channel to be
                               "occupied" by two OFDM channels.
                               2) Upstream - a) For a single OFDMA channel, the sum of the bandwidth in all the
                               subcarriers of that OFDMA channel which are not excluded. The upstream occupied
                               bandwidth is calculated as the number of subcarriers which are not excluded, multiplied
                               by the subcarrier spacing. b) For the transmit channel set, the sum of the occupied
                               bandwidth of all OFDMA channels plus the bandwidth of the legacy channels (counted as
                               1.25 times the modulation rate for each legacy channel) in a cable modem's transmit
                               channel set. The combined bandwidth of all the minislots in the channel is normally
                               smaller than the upstream occupied bandwidth due to the existence of unused
                               subcarriers. The bandwidth occupied by an OFDMA probe with a skip value of zero is
                               equal to the upstream occupied bandwidth.
  Orthogonal                   Distinguishable from or independent such that there is no interaction or interference. In
                               OFDM, subcarrier orthogonality is achieved by spacing the subcarriers at the reciprocal
                               of the symbol period (T), also called symbol duration time. This spacing results in the sinc
                               (sin x/x) frequency response curves of the subcarriers lining up so that the peak of one
                               subcarrier's response curve falls on the first nulls of the lower and upper adjacent
                               subcarriers' response curves. Orthogonal subcarriers each have exactly an integer
                               number of cycles in the interval T.
  Orthogonal Frequency         An OFDM-based multiple-access scheme in which different subcarriers or groups of
  Division Multiple Access     subcarriers are assigned to different users.
  OFDMA Channel Bandwidth      Occupied bandwidth of an upstream OFDMA channel. See also occupied bandwidth.
  Orthogonal Frequency         A data transmission method in which a large number of closely-spaced or overlapping
  Division Multiplexing        very-narrow-bandwidth orthogonal QAM signals are transmitted within a given channel.
                               Each of the QAM signals, called a subcarrier, carries a small percentage of the total
                               payload at a very low data rate.
  OFDM Channel Bandwidth       Occupied bandwidth of a downstream OFDM channel. See also occupied bandwidth.
  OFDMA Frame                  Group of a configurable number, K, of consecutive OFDMA symbols. A frame comprises
                               either a group of probing symbols or a column of minislots across the spectrum of the
                               OFDMA channel. Multiple modems can share the same OFDMA frame simultaneously by
                               transmitting data and pilots on allocated subcarriers within the frame.
  Phase Noise                  Rapid, short-term, random fluctuations in the phase of a wave, caused by time domain
                               instabilities.
  PHY Link Channel             A set of contiguous OFDM subcarriers (eight for 4K FFT and 16 for 8K FFT), constituting
                               a "sub-channel" of the OFDM channel, which conveys physical layer parameters from the
                               CMTS to cable modem.
  PHY Link Channel Frame       In downstream OFDM transmission, a group of 128 consecutive OFDM symbols, beginning
                               with the first OFDM symbol following the last OFDM symbol containing the PLC preamble.
  Physical Layer               Layer 1 in the Open System Interconnection architecture; the layer that provides services
                               to transmit bits or groups of bits over a transmission link between open systems and
                               which entails electrical, mechanical and handshaking procedures.



04/19/23                                         CableLabs                                                                    25

<!-- PDF_PAGE: 26 -->
CM-SP-PHYv3.1-I20-230419                                   Data-Over-Cable Service Interface Specifications


 Picosecond                One trillionth (10-12) of a second
 Pilot                     A dedicated OFDM subcarrier that may be used for such purposes as channel estimation
                           (measurement of channel condition), synchronization, and other purposes. See also
                           complementary pilots, continuous pilots and scattered pilots.
 Preamble                  A data sequence transmitted at or near the beginning of a frame, allowing the receiver
                           time to achieve lock and synchronization of transmit and receive clocks.
 Pre-equalizer             See adaptive pre-equalizer.
 Profile                   A set of parameters that defines how information is transmitted from a CMTS to a cable
                           modem, or from a cable modem to a CMTS. Examples of some of the parameters
                           defined in a profile include modulation order, forward error correction, preamble, and
                           guard interval.
 Protocol                  A description of a set of rules and formats that specify how devices on a network
                           exchange data.
 Pseudo Random Binary      A deterministic sequence of bits that appears to be random, that is, with no apparent
 Sequence                  pattern. Also called pseudo random bitstream.
 QAM Signal                Analog RF signal that uses quadrature amplitude modulation to convey information such
                           as digital data.
 Quadrature                The imaginary part of a vector that represents a signal, with 90 degrees phase angle
                           relative to a reference carrier. See also in-phase (I).
 Quadrature Amplitude      A modulation technique in which an analog signal's amplitude and phase vary to convey
 Modulation                information, such as digital data. The name "quadrature" indicates that amplitude and
                           phase can be represented in rectangular coordinates as in-phase (I) and quadrature (Q)
                           components of a signal.
 Quadrature Phase Shift    A form of digital modulation in which four phase states separated by 90 degrees support
 Keying                    the transmission of two bits per symbol. Also called 4-QAM.
 Radio Frequency           That portion of the electromagnetic spectrum from a few kilohertz to just below the
                           frequency of infrared light.
 Randomizer                A subset or layer of the forward error correction process, in which the data to be
                           transmitted is randomized using a PRBS scrambler. Randomization spreads out the
                           energy across the spectrum, ensures uniform population of all of the data constellation
                           points, and minimizes the likelihood of long strings of all zeros or ones.
 Receive Channel Set       The combination of legacy SC-QAM and OFDM channels that the cable modem has
                           been configured to receive by the CMTS
 Reed-Solomon              A class of error correction codes named after the inventors Irving Reed and Gustave
                           Solomon. The forward error correction in DOCSIS 3.0 and earlier uses Reed-Solomon
                           error correction codes.
 Reported Power            When referring to a cable modem upstream transmission, the reported power per
                           channel, P1.6r_n, follows these conventions:
                           1a) For a SC-QAM channel, the reported power is expressed in terms of P1.6r_n, i.e., the
                           actual power for a 6.4 MHz SC-QAM channel (with 64-QAM constellation) would be 6 dB
                           higher than the reported power (neglecting reporting accuracy); for a 1.6 MHz SC-QAM
                           channel (with 64-QAM constellation), the actual channel power would be equal to the
                           reported power (neglecting reporting accuracy).
                           1b) For SC-QAM signals with constellations other than 64-QAM, the reported power
                           differs from the actual power due to the constellation gain as defined in [DOCSIS
                           PHYv3.0].
                           2a) For an OFDMA channel, the reported power is also expressed as P1.6r_n, and for
                           OFDMA channels which do not use boosted pilots, is the average RF power of the CM
                           transmission in the OFDMA channel when transmitting in a grant comprised of 64 25 kHz
                           subcarriers or 32 50 kHz subcarriers.
                           2b) For OFDMA channels which have boosted pilots and 50 kHz subcarrier spacing,
                           reported power is 1 dB higher than the average RF power of the CM transmission with a
                           probe comprised of 32 subcarriers.
                           2c) For OFDMA channels which have boosted pilots and 25 kHz subcarrier spacing,
                           reported power is 0.5 dB higher than the average RF power of the CM transmission with
                           a probe comprised of 64 subcarriers.
                           For 2b and 2c, the additions to the probe power account for the maximum possible
                           number of boosted pilots in each OFDMA symbol when the OFDMA channel uses
                           boosted pilots.
 Return                    See upstream.




26                                            CableLabs                                                       04/19/23

<!-- PDF_PAGE: 27 -->
Physical Layer Specification                                                             CM-SP-PHYv3.1-I20-230419


  Return Loss                  The ratio of incident power PI to reflected power PR, expressed mathematically as
                               R = 10log10(PI/PR), where R is return loss in decibels.
  Reverse                      See upstream.
  RF Channel                   See channel.
  Roll-off Period              Duration in microseconds, or the equivalent number of IFFT output sample periods, used
                               for the ramping up (or ramping down) transition region of the Tukey raised-cosine
                               window, which is applied at the beginning (and end) of an OFDM symbol. A sampling rate
                               of 102.4 MHz is assumed for the upstream and 204.8 MHz is assumed for the
                               downstream. The roll-off period contains an even number of samples with weighting
                               coefficients between, but not including, 0 and 1. The rolloff, which ramps down at the end
                               of a symbol, overlaps the mirror-image rolloff which ramps up at the beginning of the
                               following symbol, and the two segments add to unity. In the case of no transmit
                               windowing, the roll-off duration is zero and there are no samples in the roll-off period.
  Root Mean Square             A statistical measure of the magnitude of a varying quantity such as current or voltage,
                               where the RMS value of a set of instantaneous values over, say, one cycle of alternating
                               current is equal to the square root of the mean value of the squares of the original values.
  Scattered Pilots             Pilots that do not occur at the same frequency in every symbol, and which may be used
                               for channel estimation. The locations of scattered pilots change from one OFDM symbol
                               to another.
  Scrambler                    See randomizer.
  Signal-To-Composite Noise    The ratio of signal power to composite noise power in a defined measurement bandwidth,
                               where composite noise is the combination of thermal noise and composite
                               intermodulation distortion (noise-like distortion).
  Single Carrier Quadrature    Data transmission method used in DOCSIS 1.x, 2.0 and 3.0, in which each downstream
  Amplitude Modulation         or upstream RF channel slot carries only one QAM signal. For the DOCSIS PHY v3.1
                               specification, SC-QAM pertains to either a) DOCSIS 3.0 or earlier downstream channels,
                               or b) TDMA, ATDMA, and S-CDMA, collectively, from DOCSIS 3.0 or earlier, for the
                               upstream channels.
  Society of Cable             A non-profit professional association that specializes in professional development,
  Telecommunications           standards, certification, and information for the cable telecommunications industry.
  Engineers
  Spectral Edge                For OFDM or OFDMA channel: The center frequency of the channel's lowest active
                               subcarrier minus half the subcarrier spacing; and the center frequency of the channel's
                               highest active subcarrier plus half the subcarrier spacing.
                               For OFDM or OFDMA exclusion band: The center frequency of the channel's highest
                               active subcarrier plus half the subcarrier spacing adjacent to the beginning of an
                               exclusion band, and the center frequency of the channel's lowest active subcarrier minus
                               half the subcarrier spacing adjacent to the end of the same exclusion band.
  Standard Frequencies         Method of spacing channels on a cable television system defined in [CEA-542]. Channels
                               2-6 and 7-13 use the same frequencies as over-the-air channels 2-6 and 7-13. Other
                               cable channels below Ch. 7 down to 91.25 MHz and above Ch. 13 are spaced in 6 MHz
                               increments.
  Subcarrier                   One of a large number of closely spaced or overlapping orthogonal narrow-bandwidth
                               data signals within an OFDM channel. Also called a tone. See also excluded subcarrier,
                               unused subcarrier, and used subcarrier.
  Subcarrier Clock Frequency   Frequency of the clock associated with the composite generation of all subcarrier signals
                               in an OFDM/OFDMA symbol transmission, nominally 25 kHz or 50 kHz; the subcarrier
                               clock frequency determines the subcarrier spacing (in the frequency domain).
  Subcarrier Spacing           The frequency spacing between centers of adjacent subcarriers in an OFDM/OFDMA
                               symbol, nominally equal to 25 kHz or 50 kHz.
  Sublayer                     A subdivision of a layer in the Open System Interconnection reference model.
  Subscriber                   End user or customer connected to a cable network.
  Subslot                      A subdivision or subunit of a minislot that fits within a minislot's boundaries, used to
                               provide multiple transmission opportunities for bandwidth requests. Data subcarriers
                               within a subslot use QPSK, and are not FEC encoded.
  Symbol Duration              Sum of the FFT duration and cyclic prefix duration. Symbol duration is greater than FFT
                               duration, because symbol duration includes a prepended cyclic prefix.
  Synchronous Code Division    A multiple access physical layer technology in which different transmitters can share a
  Multiple Access              channel simultaneously. The individual transmissions are kept distinct by assigning each
                               transmission an orthogonal "code." Orthogonality is maintained by all transmitters being
                               precisely synchronized with one another.




04/19/23                                         CableLabs                                                                27

<!-- PDF_PAGE: 28 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


 Tap                             (1) In the feeder portion of a coaxial cable distribution network, a passive device that
                                 comprises a combination of a directional coupler and splitter to "tap" off some of the
                                 feeder cable RF signal for connection to the subscriber drop. So-called self-terminating
                                 taps used at feeder ends-of-line are splitters only and do not usually contain a directional
                                 coupler. Also called a multitap.
                                 (2) The part of an adaptive equalizer where some of the main signal is "tapped" off, and
                                 which includes a delay element and multiplier. The gain of the multipliers is set by the
                                 equalizer's coefficients.
                                 (3) One term of the difference equation in a finite impulse response or an infinite impulse
                                 response filter. The difference equation of a FIR follows: y(n) = b0x(n) + b1x(n-1) +
                                 b2x(n-2) + ... + bNx(n-N).
 Thermal Noise                   The fluctuating voltage across a resistance due to the random motion of free charge
                                 caused by thermal agitation. Also called Johnson-Nyquist noise. When the probability
                                 distribution of the voltage is Gaussian, the noise is called additive white Gaussian noise
                                 (AWGN).
 Time Division Multiple Access   A multiple access technology that enables multiple users to access, in sequence, a single
                                 RF channel by allocating unique time slots to each user of the channel.
 Total Transmit Power            When referring to a cable modem upstream transmission, the cable modem total
                                 transmit power is the sum of the transmit power per channel of each channel transmitting
                                 a burst at a given time. See also Transmit Power Per Channel.
 Transit Delay                   The time required for a signal to propagate or travel from one point in a network to
                                 another point in the network, for example, from the CMTS to the most distant cable
                                 modem. Also called propagation delay.
 Transmit Channel Set            For a DOCSIS 3.1 CM cable modem, the combination of legacy SC-QAM channels and
                                 OFDMA channels that may be transmitted by a cable modem.
 Transmit Power Per Channel      When referring to a cable modem transmission, the cable modem transmit power per
                                 channel is the average RF power in the occupied bandwidth (channel width), assuming
                                 equally likely QAM symbols, measured at the F-connector of the CM. See also Reported
                                 Power.
 Transmit Pre-Equalizer          See adaptive pre-equalizer.
 Under-Grant Hold Bandwidth      The minimum grant bandwidth that can be allocated beyond which the spurious emissions
                                 limits (in dBc) are no longer relaxed as the grant size continues to decrease. Defined
                                 mathematically as UGHB = (100% grant spectrum)/(under-grant hold number of users).
 Under-Grant Hold Number of      The maximum number of equal-size grants that can be allocated beyond which the
 Users                           spurious emissions limits (in dBc) are no longer relaxed as the grants' size continues to
                                 increase. Defined mathematically as UGHU = floor[0.2 + 10((-44−SpurFloor)/10)].
 Unused Subcarrier               Subcarriers in an upstream OFDMA channel which are not excluded, but are not
                                 assigned to minislots. For example, unused subcarriers may occur when the number of
                                 subcarriers between excluded subcarriers is not divisible by the fixed number of
                                 consecutive subcarriers which comprise every OFDMA minislot. Thus, after constructing
                                 minislots from a group of consecutive non-excluded subcarriers, the remainder will
                                 become unused subcarriers. Unused subcarriers are not used for data transmission, but
                                 still carry power during probe transmission.
 Upstream                        1) The direction of RF signal transmission from subscriber to headend or hub site. Also
                                 called return or reverse. In most North American cable networks, the legacy upstream
                                 spectrum occupies frequencies from 5 MHz to as high as 42 MHz.
                                 2) The DOCSIS 3.1 upstream is 5-204 MHz, with support for 5-42 MHz, 5-65 MHz, 5-85
                                 MHz and 5-117 MHz.
 Upstream Channel                A portion of the electromagnetic spectrum used to convey one or more RF signals from
                                 the subscriber premises to the headend or hub site. For example, a commonly used
                                 DOCSIS 3.0 upstream channel bandwidth is 6.4 MHz. A DOCSIS 3.1 upstream OFDMA
                                 channel bandwidth may be as much as 95 MHz.
 Upstream Channel Descriptor     The MAC management message used to communicate the characteristics of the
                                 upstream physical layer to the cable modems.
 Used Subcarrier                 An upstream subcarrier that is part of a minislot. The cable modem transmits data, ranging,
                                 and probes on these subcarriers when instructed to do so by MAP messages. MULPI term.
 Useful Symbol Duration          See FFT duration.
 Vector                          A quantity that expresses magnitude and direction (or phase), and is represented
                                 graphically using an arrow.




28                                                 CableLabs                                                        04/19/23

<!-- PDF_PAGE: 29 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419


  Windowing                    A technique to shape data in the time domain, in which a segment of the start of the IFFT
                               output is appended to the end of the IFFT output to taper or roll-off the edges of the data
                               using a raised cosine function. Windowing maximizes the capacity of the channel by
                               sharpening the edges of the OFDM/A signal in the frequency domain.
  Word                         Information part of a codeword, without parity. See also codeword.
  Zero-Bit-Loaded-Subcarrier   A subcarrier with power but not carrying user data, although it could be modulated by a
                               PRBS.
  Zero-Valued Minislot         A minislot composed of zero-valued subcarriers and no pilots.
  Zero-Valued Subcarrier       A subcarrier with no power. See also excluded subcarrier.




04/19/23                                         CableLabs                                                              29
