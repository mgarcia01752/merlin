# 7.5 - Downstream Transmit and Receive

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 108-171
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 108 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


The test modulator generates the second transmission using a compliant probe sent to both the spectrum analyzer
and the CMTS burst receiver (unit under test) with a CNR > 35 dB:
      •    The spectrum analyzer measures and records the amplitude variation over the spectrum of subcarriers (this
           is the :reference amplitude variation measurement" of the test).
      •    The CMTS burst receiver develops pre-equalizer coefficients.
      •    The CMTS formats and transmits compliant commands for the pre-equalizer coefficients.
      •    The downstream test receiver validates reception of pre-equalization coefficients.
Pre-equalization coefficients are implemented by the test modulator prior to the third transmission:
      •    The spectrum analyzer measures and records the amplitude variation over the spectrum of subcarriers for
           this third transmission from the test modulator, which has been pre-equalized.
      •    The reduction in this third amplitude variation measurement at the spectrum analyzer compared to the
           initial amplitude variation measurement of the second transmission is measured.
      •    The required minimum reduction in amplitude variation or better is observed.
The CM MUST normalize the new calculated coefficients as follows:
           mean (abs (Ck)^2 ) = 1
           (mean value computed over all pre-equalizer coefficients corresponding to the used and unused subcarriers).
The CM MUST pre-equalize all transmissions other than probe signals, as defined by the CMTS via the RNG_RSP
message. The CM MUST pre-equalize all probe transmissions unless the bit in the P-MAP message that defines the
presence or absence of pre-equalization, is set to "equalizer disabled".
The CM MUST be able to transmit a probe signal with or without pre-equalization (all coefficients are reset to
1+j*0) as instructed by the CMTS using the P-MAP message described in [DOCSIS MULPIv3.1].
The CM MUST reset all its pre-equalizer coefficients in the following cases:
1. Before its first transmission after receiving the first UCD message from the CMTS. See [DOCSIS MULPIv3.1].
2. Before its first transmission after a change in at least one of the following parameters: upstream channel
frequency (the frequency of subcarrier with index zero), subcarrier spacing, Cyclic Prefix size, Rolloff Period
duration, upstream channel change (CM is moved to a different upstream channel ID).

7.5       Downstream Transmit and Receive
7.5.1      Overview
This section specifies the downstream electrical and signal processing requirements for the transmission of OFDM
modulated RF signals from the CMTS to the CM.

7.5.2      Signal Processing
Serial data signals received from the PHY-MAC Convergence Layer are received and processed by the PHY as
illustrated in Figure 45.
This process yields OFDM symbols with 4096 subcarriers for the 4K FFT mode and 8192 subcarriers for the 8K
FFT mode, with each symbol consisting of:
      •    Data subcarriers
      •    Scattered pilots
      •    Continuous pilots
      •    PLC subcarriers
      •    Excluded subcarriers that are set to zero


108                                                    CableLabs                                             04/19/23

<!-- PDF_PAGE: 109 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419


This section briefly describes that process and provides links to the specific requirements for each process described
in this specification.




                                     Figure 45 - Downstream PHY Processing

7.5.2.1    Forward Error Correction (FEC) Encoding
The PHY begins processing incoming data by FEC encoding data bits to form encoded codewords. Forward error
correction adds redundancy to the transmitted data; these redundant bits can be used by the receiver to detect and
correct errors in the transmission. For DOCSIS 3.1, FEC encoding applies a concatenated BCH-LDPC encoder,
based on [DVB-C2], and then shuffling the bits in a codeword via bit interleaving. Downstream forward error
correction is described in detail in Section 7.5.4.

7.5.2.2    Symbol Mapping to QAM Constellations
Once FEC encoded codewords have been created, the codewords are placed into OFDM symbols. Because each
subcarrier in an OFDM symbol can have a different QAM modulation, the codewords are to be first demultiplexed
into parallel cell words; these cell words are then mapped into constellations based on the corresponding bit loading
pattern of the subcarrier's QAM constellation. In DOCSIS 3.1, QAM constellations for data subcarriers include zero-
bit-loaded subcarriers and 16-QAM and 64-QAM to 4096-QAM, with both square and non-square constellations.
8192-QAM and 16384-QAM are optional modulation orders for both the CM and CMTS. This process is described
in Section 7.5.4.1.1.

7.5.2.3    Scattered Pilot Placeholder Insertion
OFDM transmission requires the insertion of scattered pilots to enable channel estimation and equalization in the
receiver. While the insertion happens after time and frequency interleaving, since these pilots are not in the same
spectral location in every symbol, insertion of these scattered pilots disrupt the spectral location of the QAM data
subcarriers. To overcome this problem, place-holders for scattered pilot locations are inserted during the symbol
mapping process. When a particular subcarrier carries a scattered pilot, the phase of that scattered pilot on that
subcarrier is always the same, and is either 0 degrees or 180 degrees, depending on a pseudo-random sequence. The
pseudo-random sequence for defining the phase for the placeholders for both the scattered pilots and continuous
pilots is repeated every OFDM symbol, and the process is described in Section 7.5.15.




04/19/23                                            CableLabs                                                    109

<!-- PDF_PAGE: 110 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


7.5.2.4    Next Codeword Pointer Insertion
Detecting where the next codeword begins in an OFDM symbol can be difficult: more than one codeword may map
into one OFDM symbol, the number of codewords per OFDM symbol may not be an integer, a codeword can
overflow from one OFDM symbol to another, and the codeword could be shortened. Therefore, the transmitter is to
convey to the receiver all of the locations where a new codeword begins within an OFDM symbol. These Next
Codeword Pointers (NCPs) are encoded using another forward error correction method and are appended to OFDM
symbols. NCP subcarriers are modulated using QPSK, 16-QAM, or 64-QAM and this modulation is signaled by the
PLC. The process of encoding and inserting the NCP for DOCSIS 3.1 is discussed in Sections 7.5.5.5.2 and 8.3.4.

7.5.2.5    Interleaving
These OFDM symbols, comprised of data subcarriers, scattered pilot placeholders, and NCPs, are then subjected to
time and frequency interleaving. Time interleaving mitigates the impact of burst noise, while frequency interleaving
mitigates the effect of ingress.
Time interleaving disperses the subcarriers of an input symbol over a set of output symbols, based on the depth of
interleaving. Therefore, if an OFDM symbol is corrupted by a noise burst, this burst is spread over the symbols
when it is de-interleaved, thereby reducing the error correction burden on the decoder. The time interleaving process
is described in Section 7.5.6.
Frequency interleaving occurs after time interleaving. Frequency interleaving disperses subcarriers of the symbol
along the frequency axis; therefore, OFDM subcarriers impacted by narrowband ingress are distributed between
several codewords, reducing the number of errors in each codeword. The frequency interleaving process is described
in Section 7.5.6.

7.5.2.6    Insertion of Continuous Pilots and Exclusion Sub-bands
When interleaving is complete, placeholders for continuous pilots are inserted. These will be subject to modulation
later, together with the placeholders already inserted for scattered pilots. Continuous pilots are pilots that occur at
the same subcarrier location in every symbol. These are needed for receiver synchronization.
Exclusion bands and excluded subcarriers are inserted next. Nothing is transmitted at these subcarrier locations. The
contiguous block of subcarriers allocated to the PHY Link Channel is also treated as an exclusion band at this stage;
this is a placeholder for the PLC that is filled later. The regions outside the bandwidth of the OFDM signal may also
be treated as exclusion bands.
When a particular subcarrier carries a continuous pilot, the phase of that continuous pilot on that subcarrier is always
the same, and is either 0 degrees or 180 degrees depending on a pseudo-random sequence. The pseudo-random
sequence for defining the phase for the placeholders for both the scattered pilots and continuous pilots is repeated
every OFDM symbol, and the process is described in Section 7.5.15.

7.5.2.7    Encoding and Insertion of the PLC
The PLC is constructed within the convergence layer in parallel with the functions already discussed, relating to the
main data channel. The PLC occupies the same contiguous set of subcarriers in every OFDM symbol, as described
in Section 7.5.13.2. As further described in Section 7.5.13.2, the PLC subcarriers carry the PLC preamble for 8
consecutive OFDM symbols followed by 120 symbols of PLC data. The PLC data is encoded for error correction,
and then mapped into 16-QAM PLC data subcarriers. The PLC data is not subjected to the same time or frequency
interleaving as the data; however they are block interleaved. The PLC is then inserted in place of its placeholder in
each symbol. This PLC data block interleaving process is described in Section 7.5.6.3. The PLC preamble is BPSK
modulated as defined in Section 7.5.13.3.

7.5.2.8    IDFT Transformation and Cyclic Prefix Insertion
In this stage each OFDM symbol is transformed into the time domain using a 4096-point or 8192-point inverse
discrete Fourier transform (IDFT). This 4096 or 8192 sample sequence is referred to below as the IDFT output. This
process is described in Section 7.5.7.




110                                                  CableLabs                                               04/19/23

<!-- PDF_PAGE: 111 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


7.5.2.9    Cyclic Prefix and Windowing
A segment at the end of the IDFT output is prepended to the IDFT, and this is referred to as the Cyclic Prefix (CP)
of the OFDM symbol. There are five possible values for the length of the CP and the choice depends on the delay
spread of the channel - a longer delay spread requires a longer cyclic prefix.
For windowing purposes another segment at the start of the IDFT output is appended to the end of the IDFT output -
the roll-off period (RP). There are five possible values for the RP, and the choice depends on the bandwidth of the
channel and the number of exclusion bands within the channel. A larger RP provides sharper edges in the spectrum
of the OFDM signal; however, there is a time vs. frequency trade-off. Larger RP values reduce the efficiency of
transmission in the time domain, but because the spectral edges are sharper, more useful subcarriers appear in the
frequency domain. There is an optimum value for the RP that maximizes capacity for a given bandwidth and/or
exclusion band scenario.
These topics are discussed in detail in Section 7.5.8.

7.5.3     Time and Frequency Synchronization
This section specifies the timing and frequency synchronization requirements for DOCSIS 3.1 CMTS transmitters
and CM receivers.
The purpose of this section is to ensure that the CMTS transmitter can provide proper timing and frequency
references for DOCSIS 3.1 downstream OFDM operation and that the CM receiver can acquire the system timing
and subcarrier from the downstream for proper DOCSIS 3.1 operation.
The CMTS downstream OFDM symbol and subcarrier frequency and timing relationship is defined in Section 7.3.3.
Tolerances for the downstream subcarrier clock frequency are given in Sections 7.5.3, 7.5.3.1 to 7.5.3.3, and 7.5.9.1.
Functional requirements involving the downstream subcarrier clock frequency and downstream signal generation are
contained in Section 7.3.3, which couple the subcarrier clock frequency tolerance performance to the phase noise
requirements of Section 7.5.9.1 and the downstream OFDM symbol clock requirements of Section 7.5.3.
Each cycle of the downstream subcarrier clock is 4096 or 8192 cycles (50 kHz and 25 kHz subcarrier spacing,
respectively) of the downstream OFDM symbol clock (which is nominally 204.8 MHz), since the subcarrier clock
period is defined as the FFT duration for each OFDM symbol. Functional requirements on locking the downstream
waveform to the 10.24 MHz Master Clock (Sections 7.5.3.1 and 7.5.4.3.2) are then equivalently functional
requirements locking the downstream subcarrier clock to the Master Clock. Downstream OFDM symbol clock jitter
requirements (which are in the time domain) of Section 7.5.3.3 are equivalently requirements on the downstream
subcarrier clock (and its harmonics).
The requirements on the OFDM symbol clock are effectively measured on observables in the downstream
waveform, which include the downstream subcarrier clock frequency (manifested in the subcarrier spacing) and
downstream subcarrier frequencies.

7.5.3.1    Downstream Sampling Rate
The CMTS MUST lock the 204.8 MHz Downstream OFDM Clock to the 10.24 MHz CMTS Master Clock (see
Table 6 - Downstream OFDM Parameters).

7.5.3.2    OFDM RF Transmission Synchronization
The CMTS MUST lock the Downstream OFDM RF transmissions to the 10.24 MHz CMTS Master Clock (see
Table 6 - Downstream OFDM Parameters).

7.5.3.3    Downstream OFDM Symbol Clock Jitter
The CMTS MUST adhere to the following clock jitter requirements for the downstream OFDM symbol clock over
the specified frequency ranges:
    •     < [-21 + 20*log (fDS /204.8)] dBc (i.e., < 0.07 ns RMS) 10 Hz to 100 Hz
    •     < [-21 + 20*log (fDS /204.8)] dBc (i.e., < 0.07 ns RMS) 100 Hz to 1 kHz



04/19/23                                             CableLabs                                                  111

<!-- PDF_PAGE: 112 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


      •   < [-21 + 20*log (fDS /204.8)] dBc (i.e., < 0.07 ns RMS) 1 kHz to 10 kHz
      •   < [-4 + 20*log (fDS /204.8)] dBc (i.e., < 0.5 ns RMS) 10 kHz to 100 kHz
      •   < [2 + 20*log (fDS /204.8)] dBc (i.e., < 1 ns RMS) 100 kHz to (fDS /2), where fDS is the frequency of the
          measured downstream clock in MHz.
The CMTS MUST use a value of fDS that is an integral multiple or divisor of the downstream symbol clock. For
example, an fDS = 409.6 MHz clock may be measured if there is no explicit 204.8 MHz clock available.
In addition to meeting the clock jitter requirements given above, the CMTS is required to meet the phase noise
specifications defined in Table 41 of Section 7.5.9.1. In the event of a conflict between the clock jitter and the phase
noise requirement, the CMTS MUST meet the more stringent requirement.

7.5.3.4    Downstream Timing Acquisition Accuracy
The downstream clock timing is defined with respect to downstream PLC frame.
The CM MUST be able to adjust its clock to synchronize its own clock timing with PLC frame for proper operation.
The CM MUST be able to acquire downstream clock timing from downstream traffic (pilots, preambles, or mixed
pilots, preambles, and data).
The CM MUST have a timing acquisition accuracy better than 1 sample (4.8828125 ns).

7.5.3.5    Downstream Carrier Frequency Acquisition
The CM MUST be able to acquire the carrier frequency from downstream (pilots, preambles, or mixed pilots,
preambles and data).

7.5.3.6    Downstream Acquisition Time
The CM MUST achieve downstream signal acquisition (frequency and time lock) in less than 60s for a device with
no previous network frequency plan knowledge.
Nonetheless, it is expected that the CM would be able to achieve downstream acquisition in less than 30s.

7.5.4     Downstream Forward Error Correction
This section describes the downstream forward error correction scheme used for DOCSIS 3.1. It is based on [DVB-
C2] section 6.1, FEC Encoding; it is used here with the following modifications:
      •   A codeword will be the size of the short FEC Frame (16,200 bits); the "normal" FEC Frame (64,800 bits) is
          not used.
      •   Only the code rate 8/9 is used.
      •   Support for non-square constellations (128-QAM, 512-QAM, and 2048-QAM) is introduced.
      •   Support for mixed modulation codewords is introduced.
      •   Support for codeword shortening is introduced.
      •   Bit Interleaving for non-square constellations (128-QAM, 512-QAM, and 2048-QAM), mixed modulation
          mode constellations and for shortened codewords is introduced.
      •   Demultiplexing for non-square constellations (128-QAM, 512-QAM, and 2048-QAM), mixed modulation
          mode constellations, and for shortened codewords is introduced.
      •   Support for QPSK modulation is not required.
These changes are described in the following sections.




112                                                  CableLabs                                               04/19/23

<!-- PDF_PAGE: 113 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419


7.5.4.1     Definitions

7.5.4.1.1      Mixed-Modulation Codewords
Before downstream FEC can be defined, it is important to understand what a mixed-modulation codeword is, as
these codewords are handled differently. A mixed-modulation codeword belongs to a profile that does not use the
same modulation constellation for all subcarriers of the OFDM symbol. Note that subcarrier zero bit-loading is not
taken into account when determining if a codeword is a mixed-modulation codeword. In other words, if a profile has
the same modulation constellation (i.e., same bit loading profile) for all non-zero-bit-loaded subcarriers of the
OFDM symbol, then the codewords of that profile are not considered to be mixed modulation.
As an example consider a profile in which even numbered subcarriers, excluding zero-bit-loaded subcarriers (i.e.,
non-zero-bit-loaded subcarriers 0, 2, 4, 6, etc.), are modulated with modulation A, and odd numbered subcarriers
(i.e., non-zero-bit-loaded subcarriers 1, 3, 5, 7, etc.) are modulated with modulation B. This provides a bits/s/Hz
value that is the mean of the bits/s/Hz values of modulations A and B. Any codeword that belongs to that profile is a
mixed-modulation codeword.
In this example, if these subcarriers are modulated as shown in Table 22, the modulation combinations provide
approximately 1.5 dB SNR granularity of additional spectral efficiency:
                            Table 22 - Mixed Modulation with 1.5 dB SNR Granularity

                                        Modulation A           Modulation B
                                      128-QAM                256-QAM
                                      256-QAM                512-QAM
                                      512-QAM                1024-QAM
                                      1024-QAM               2048-QAM
                                      2048-QAM               4096-QAM

Another example of the use of mixed-modulation codewords can be applied to the case of an OFDM channel at the
high frequency end with a significant tilt in SNR. In this case, a modulation profile for this part of the spectrum
could use four different QAM constellations covering the OFDM symbol: 1024-QAM, 512-QAM, 256-QAM and
64-QAM. All of the codewords to which this profile is applied would be of the mixed-modulation type.
It is important to note that a codeword is treated as being a mixed-modulation type even if all of the subcarriers have
the same modulation order; being of the mixed-modulation type is determined by the profile. For example, consider
a codeword of the above profile in which all the subcarriers happen to be 256-QAM. Despite the fact that all
subcarriers of this codeword have the same modulation, this codeword is treated as the mixed-modulation type since
it belongs to a mixed-modulation profile. It is necessary to do this because the FEC encoder has no knowledge as to
which subcarriers the codeword is going to be mapped while the encoding is being performed. Therefore, FEC
encoder operations are determined by the profile applied to the codeword only.
As a final example, consider a profile that consists of 75% 1024-QAM subcarriers and 25% zero-bit-loaded
subcarriers. In this case the codewords of that profile are not of the mixed-modulation type, since zero-bit-loaded
subcarriers are ignored when determining mixed-modulation type.

7.5.4.1.2      Codeword versus FECFrame
[DVB-C2] uses the term FECFrame to refer to the bits of one LDPC encoding operation. In this specification, the
term codeword is used for the same concept.

7.5.4.2     FEC Encoding
[DVB-C2] section 6.1, FEC Encoding, describes the FEC encoding requirements for the CMTS transmitter. The
CMTS MUST meet the portion of [DVB-C2] section 6.1, FEC Encoding, as described below:
The CMTS MUST support the 8/9 code rate for the short codeword (Nldpc = 16,200 bits) only. Support for other code
rates and codeword sizes is not required.




04/19/23                                            CableLabs                                                     113

<!-- PDF_PAGE: 114 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications


The CMTS MUST support the FEC coding parameters specified in Table 23 - Coding Parameters (for Short
Codewords Nldpc = 16,200 and Code Rate 8/9). This table is based on Table 3(b), from [DVB-C2].
             Table 23 - Coding Parameters (for Short Codewords Nldpc = 16,200 and Code Rate 8/9)

          LDPC Code       BCH Uncoded             BCH Coded        LDPC Uncoded          LDPC Coded Block
             Rate         Block Size Kbch         Block Nbch       Block Size Kldpc          Size Nldpc
              8/9              14,232               14,400               14,400                 16,200


7.5.4.2.1      Outer Encoding (BCH)
[DVB-C2] section 6.1.1, Outer Encoding (BCH), details the outer encoding requirements for normal and short
codewords (FECFrames). For the CMTS, only short codewords are required. The CMTS MUST meet the outer
encoding requirements for short FECFrames specified in [DVB-C2] section 6.1.1, Outer Encoding (BCH).

7.5.4.2.2      Inner Coding (LDPC)
[DVB-C2] sections 6.1.2, Inner Encoding, and 6.1.2.2, Inner Coding for Short FECFrame, detail the inner coding
requirements for short codewords. For DOCSIS 3.1 codewords, the CMTS MUST meet the inner coding
requirements for short codewords and code rate 8/9 specified in [DVB-C2] sections 6.1.2, Inner Encoding, and
6.1.2.2, Inner Coding for Short FECFrame.

7.5.4.2.3      Support for Codeword Shortening
Codeword shortening is used for two purposes:
      •   Create shortened codewords when there is insufficient data to fill complete codewords.
      •   Achieve strong burst noise protection
The full FEC block size for the FEC code rate of 8/9 is provided in Table 23.
Codeword shortening is accomplished by shortening the uncoded block size in the BCH Uncoded Block Size column
of Table 23. Note that the number of parity bits remains the same; there is no shortening of the parity bits either in
the BCH or in the LDPC.
When a shortened codeword is needed, the CMTS MUST complete the codeword shortening process described here.
There are six overall steps to the codeword shortening process:
1.    Prepending zero bits (BCH) to the data
2.    BCH encoding
3.    Removing the prepended zero bits
4.    Appending zero bits (LDPC) to the data
5.    LDPC Encoding
6.    Removing the appended zero bits
This is done in both the BCH encoder and the LDPC encoder, as shown in Figure 46.




114                                                 CableLabs                                              04/19/23

<!-- PDF_PAGE: 115 -->
Physical Layer Specification                                                 CM-SP-PHYv3.1-I20-230419




                                   Figure 46 - Codeword Shortening Process
The zero bit padding process is shown in more detail in Figure 47.




                                           Figure 47 - Padding Process




04/19/23                                           CableLabs                                    115

<!-- PDF_PAGE: 116 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


7.5.4.2.3.1    Codeword Shortening for Strong Burst Noise Protection
Although the primary purpose of codeword shortening is to support scenarios in which there is insufficient data to
fill complete codewords, codeword shortening can also be used to provide signal protection in strong burst noise
conditions. A lower code rate such as 7/9 has better burst noise capabilities than the 8/9 code rate. Through
codeword shortening, it is possible to achieve the equivalent of a 7/9 code rate.
For example, the 16200-bit block can be shortened by 8096 bits. The number of parity bits remains unchanged at
1800. Hence, this shortened codeword will have a block size of 8104 with 6304 information bits and 1800 parity
bits; this produces an effective code rate of approximately 7/9 (6304/8104 = 0.777887463). When the receiver
receives this shortened codeword, it will pad the shortened 8096 bits with zeros to create a 16200-bit rate 8/9
codeword and decode it using the rate 8/9 decoder.

7.5.4.3     Bit Interleaving

7.5.4.3.1      Bit Interleaving for Non-Shortened Codewords
For non-shortened codewords that are not of the mixed-modulation type the CMTS MUST apply parity interleaving,
followed by column-twist interleaving as detailed in [DVB-C2] section 6.1.3, Bit Interleaver, with the number of
rows, columns and column twisting parameters specified in this section.
The number of rows and columns of the Bit Interleaver are specified by Table 24.
                                           Table 24 - Bit Interleaver Structure

                               Modulation                  Rows Nr                Columns Nc
                                  16-QAM                     2025                      8
                                  64-QAM                     1350                      12
                                 128-QAM                     2315                      7
                                 256-QAM                     2025                      8
                                 512-QAM                     1800                      9
                                 1024-QAM                    810                       20
                                 2048-QAM                    1473                      11
                                 4096-QAM                    675                       24
                                 8192-QAM                    1247                      13
                                16384-QAM                    1158                      14

Since 16,200 is not divisible by 7, 11, 13 and 14, for 128-QAM, 2048-QAM, 8192-QAM and 16384-QAM
constellations, the CMTS MUST append zeros after parity interleaving and prior to column-twist interleaving at the
end of the block: 5 zero bits for 128-QAM, 3 zero bits for 2048-QAM, 11 zero bits for 8192-QAM and 12 zero bits
are added after the 16,200th bit. Thus, an extended block of 16,205 bits, 16,203 bits, 16,211 bits and 16,212 bits will
be interleaved by the column-twist interleaver for 128-QAM, 2048-QAM, 8192-QAM and 16384-QAM,
respectively.
For non-shortened codewords that are not of the mixed-modulation type, the CMTS MUST serially write the data
bits into the column-twist interleaver column-wise, and serially read out row-wise, where the write start position of
each column is twisted by tc, as specified in Table 25 - Column Twisting Parameter tc (columns 0 - 11) and Table 26
- Column Twisting Parameter tc (columns 12-23).
                              Table 25 - Column Twisting Parameter tc (columns 0 - 11)

      Codeword       Columns                                     Twisting Parameter tc
      Modulation        Nc          Col. 0      1      2     3       4    5       6        7    8   9   10    11
        Type
          16-QAM         8            0         0      0     1       7    20      20    21      -   -   -      -
          64-QAM         12           0         0      0     2       2    2       3        3    3   6   7      7
          128-QAM        7            0         1      2     2       2    3       3         -   -   -   -      -
          256-QAM        8            0         0      0     1       7    20      20    21      -   -   -      -



116                                                  CableLabs                                              04/19/23

<!-- PDF_PAGE: 117 -->
Physical Layer Specification                                                                                            CM-SP-PHYv3.1-I20-230419


     Codeword          Columns                                                   Twisting Parameter tc
     Modulation           Nc        Col. 0            1            2         3         4         5            6        7         8      9       10        11
       Type
       512-QAM            9             0             1            2         3         5         6            7        9      11           -     -        -
       1024-QAM          20             0             0            0         2         2         2            2        2         5      5       5         5
       2048-QAM          11             0             0            0         0         0         3            3        4         4      4       4         -
       4096-QAM          24             0             0            0         0         0         0            0        1         1      1       2         2
       8192-QAM          13             0             0            0         0         0         0            2        3         5      8       8         9
      16384-QAM          14             0             0            2         2         2         2            2        4         4      6       6         8


                              Table 26 - Column Twisting Parameter tc (columns 12-23)

     Codeword          Columns                                                   Twisting Parameter tc
     Modulation           Nc        12           13           14        15        16        17           18        19        20        21       22        23
       Type
        16-QAM            8         -             -           -         -         -         -            -         -         -         -        -         -
        64-QAM           12         -             -           -         -         -         -            -         -         -         -        -         -
       128-QAM            7         -             -           -         -         -         -            -         -         -         -        -         -
       256-QAM            8         -             -           -         -         -         -            -         -         -         -        -         -
       512-QAM            9         -             -           -         -         -         -            -         -         -         -        -         -
       1024-QAM          20         5            7            7         7         7         8            8         10        -         -        -         -
       2048-QAM          11         -             -           -         -         -         -            -         -         -         -        -         -
       4096-QAM          24         2            3            7         9         9         9            10        10        10        10       10        11
       8192-QAM          13         9             -           -         -         -         -            -         -         -         -        -         -
      16384-QAM          14         9            9            -         -         -         -            -         -         -         -        -         -


7.5.4.3.2     Bit Interleaving for Non-Shortened Mixed Modulation Codewords
To support non-shortened mixed-modulation codewords, the bit interleaver specified in [DVB-C2] has been
modified. For non-shortened mixed-modulation codewords, the CMTS MUST apply parity interleaving followed by
column-twist interleaving as detailed in [DVB-C2] section 6.1.3, Bit Interleaver, and in the following discussion.
Because specific columns of the bit de-interleaver cannot be mapped to specific bits of the QAM constellation,
column twisting interleaving is used over all 24 columns.
For non-shortened mixed-modulation codewords, the CMTS MUST serially write the data bits into the column-twist
interleaver column-wise, and serially read out row-wise, where the write start position of each column is twisted by
tc, as specified in Table 27 - Mixed-Modulation Type - Column Twisting Parameter tc (columns 0-11) and Table 28
- Mixed-Modulation Type - Column Twisting Parameter tc (columns 12-23)
              Table 27 - Mixed-Modulation Type - Column Twisting Parameter tc (columns 0-11)

       Codeword          Columns                                                  Twisting Parameter tc
       Modulation           Nc          Col. 0        1            2         3         4         5            6         7         8         9       10     11
         Type
     Mixed-Modulation         24            0         0            0         0         0         0            0         1         1         1        2        2


              Table 28 - Mixed-Modulation Type - Column Twisting Parameter tc (columns 12-23)

      Codeword           Columns                                                  Twisting Parameter tc
      Modulation            Nc          Col. 12       13           14        15        16        17           18        19        20       21        22       23
        Type
    Mixed-Modulation          24            2             3        7         9         9             9        10        10        10       10        10       11




04/19/23                                                          CableLabs                                                                                       117

<!-- PDF_PAGE: 118 -->
CM-SP-PHYv3.1-I20-230419                                                 Data-Over-Cable Service Interface Specifications


7.5.4.3.3     Bit Interleaving for Shortened Codewords
Shortened codewords fall into one of the following three types:
1.    Square modulation
2.    Non-square modulation
3.    Mixed modulation
The CMTS MUST interleave all types of shortened codewords as described in this section.
The CMTS MUST interleave the 1800 parity bits as described in [DVB-C2] section 6.1.3, Bit Interleaver.
Because the shortened codeword can be quite small, it is possible that the entire codeword could map to one column
of the interleaver and hence not get interleaved. To avoid this, the CMTS MUST use the maximum number of
columns in the bit interleaver (24) on shortened codewords. The number of rows that would be occupied by the
shortened codeword is given by the following equation:
                                                                               𝐾𝐾+1968
                                               𝑅𝑅𝑅𝑅𝑤𝑤𝐶𝐶𝐶𝐶𝐶𝐶𝐶𝐶𝐶𝐶 = 𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐 �             �
                                                                                 24

If (K+1968) is not divisible by 24, then the last column will only be partially filled by the encoded shortened
codeword, as illustrated in Figure 48. For shortened codewords, the CMTS MUST fill the unfilled part of the last
column with bits that are labeled as "unused". For shortened codewords, the CMTS MUST discard these "unused"
bits in the memory read operation described below.



                                                   24 columns

                                                                                                     11


             Row_Count
                                                                                                          Unused




                          Figure 48 - Bit De-interleaver Block for a Shortened Codeword
For shortened codewords, the CMTS MUST serially write the data bits into the column-twist interleaver column-
wise, and serially read out row-wise, where the write start position of each column is twisted by tc, as specified in
Table 29 - Shortened Codeword Type Modulation - Column Twisting Parameter tc (columns 0 - 11) and Table 30 -
Shortened Codeword Type Modulation - Column Twisting Parameter tc (columns 12 - 23). For shortened
codewords, the CMTS MUST fill any unfilled bits of the last column with bits marked "unused". The CMTS MUST
write the first bit of the last column beginning from the 12th location since tc=11 for column 24. If there are any bits
left over after writing in the last location of the column, these bits are to be written beginning from the top of the
column. For shortened codewords, the CMTS MUST discard any bits labeled as "unused" during the process of
reading along the rows of the two-dimensional array.
       Table 29 - Shortened Codeword Type Modulation - Column Twisting Parameter tc (columns 0 - 11)

        Codeword          Columns                                         Twisting Parameter tc
          Type               Nc       Col. 0      1        2        3           4            5   6   7      8      9   10    11
         Shortened          24          0         0        0        0           0            0   0   1      1      1   2      2




118                                                     CableLabs                                                          04/19/23

<!-- PDF_PAGE: 119 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


        Table 30 - Shortened Codeword Type Modulation - Column Twisting Parameter tc (columns 12 - 23)

         Codeword       Columns                                  Twisting Parameter tc
           Type            Nc        Col. 12   13    14     15     16    17    18    19     20    21    22     23
          Shortened         24         2        3     7     9      9     9     10    10     10    10    10     11


7.5.4.4     Downstream Receiver FEC Processing
Downstream data is encoded for FEC by the CMTS. The CM MUST decode the FEC-applied codeword to correct
for any bit errors introduced by noise and interference in the transmission medium. This process is discussed in this
section.
The FEC decoder at the CM operates on the QAM subcarriers of OFDM symbols to generate an error corrected
bitstream. In addition, the decoder generates error statistics such as codeword error ratios.
The FEC decoding process is shown in Figure 49.




                                        Figure 49 - FEC Decoding Process
The receiver FEC decoder consists of the following components:
    •     The log-likelihood ratio (LLR) de-mapper processes one OFDM subcarrier at a time from the OFDM
          symbol and generates the LLRs for all bits of the QAM constellation, as defined by the bit-loading profile
          for the specific subcarrier. For example, if the subcarrier is 1024-QAM, the LLR de-mapper will generate
          10 LLRs for the subcarrier and the values of the LLRs are implementation specific. The LLR de-interleaver
          operates on the LLRs. This is the inverse of the bit-interleaver that has been applied by the CMTS
          transmitter, described in Section 7.5.2.5. Note that the receiver operates on LLRs and not on bits.
    •     The LDPC decoder decodes the 16200-bit LDPC codeword (or a shortened codeword). LDPC decoding is
          implemented using an iterative algorithm that uses message passing between the bit nodes and the check
          nodes of the Tanner graph of the LDPC code. If the CMTS has transmitted a shortened codeword (e.g.,
          when the payload is not large enough to fill a complete codeword), the receiver augments the shortened
          codeword to full size with LLRs corresponding to zero-valued bits. The receiver then decodes the
          codeword using the 16200-bit LDPC decoder, discarding the augmented bits.
    •     The BCH decoder generates an error corrected bitstream. The BCH decoder is also required to operate on
          shortened codewords.
    •     An error monitor determines codeword error ratios for reporting and troubleshooting.

7.5.5     Mapping Bits to QAM Constellations
This section describes the method used in DOCSIS 3.1 to map bits onto QAM constellations. It is based on [DVB-
C2] section 6.2, Mapping Bits onto Constellations, and is used here with the following modifications:
    •     Parameters for mapping bits onto non-square constellations have been added
    •     Parameters for mapping bits of shortened codewords onto all constellation types have been added
    •     Parameters for mapping bits of mixed-modulation codewords onto all constellation types have been added




04/19/23                                            CableLabs                                                      119

<!-- PDF_PAGE: 120 -->
CM-SP-PHYv3.1-I20-230419                                           Data-Over-Cable Service Interface Specifications


As described in [DVB-C2] section 6.2, Mapping Bits onto Constellations, the CMTS MUST map each codeword to
a sequence of QAM constellation values by:
      •   Demultiplexing the input bits into parallel cell words
      •   Mapping these cell words into constellation values
The mapping of bits to QAM constellation is carried out using the three sequential operations depicted in Figure 50.




                                   Figure 50 - Bits to QAM Constellation Mapping
The CMTS MUST use the number of bits per cell ηMOD, as defined in Table 31 - Parameters for Bit-Mapping onto
Constellations, when bit mapping codewords to constellations.
For non-shortened codewords that are not of the mixed-modulation type, the CMTS MUST use the Number of
Output Data Cells defined in Table 31 - Parameters for Bit-Mapping onto Constellations when bit mapping
codewords to constellations.
                            Table 31 - Parameters for Bit-Mapping onto Constellations

                             Modulation Mode                       Number of Output Data
                                                                          Cells
                                 16384-QAM             14                   1158
                                  8192-QAM             13                   1247
                                  4096-QAM             12                   1350
                                  2048-QAM             11                   1473
                                  1024-QAM             10                   1620
                                  512-QAM               9                   1800
                                  256-QAM               8                   2025
                                  128-QAM               7                   2315
                                   64-QAM               6                   2700
                                   16-QAM               4                   4050

For the cases of mixed-modulation codewords and shortened codewords, the number of output symbols per LDPC
block remains an integer. For both mixed-modulation codewords and shortened codewords, the CMTS MUST pad
the end of the LDPC block with zero bits to produce an integer number of bits in the final QAM symbol. The CM
MUST discard zero pad bits in the received symbol. This is described in further detail in the following sections.

7.5.5.1    Modulation Formats
The CMTS modulator MUST support 16-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM, 1024-QAM, 2048-
QAM, and 4096-QAM.
The CMTS modulator MAY support 8192-QAM and 16384-QAM.
The CM demodulator MUST support 16-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM, 1024-QAM, 2048-
QAM, and 4096-QAM.
The CM demodulator MAY support 8192-QAM and 16384-QAM.




120                                                  CableLabs                                            04/19/23

<!-- PDF_PAGE: 121 -->
Physical Layer Specification                                                                                     CM-SP-PHYv3.1-I20-230419


7.5.5.2     Bit-to-Cell Word Demultiplexer

7.5.5.2.1        Non-shortened Codewords
For non-shortened codewords that are not of the mixed-modulation type, the CMTS MUST demultiplex the
bitstream vi from the bit interleaver into Nsubstreams sub-streams, using the value of Nsubstreams as defined in Table 32 -
Number of Sub-Streams in Demultiplexer and the description following that table.
                                                 Table 32 - Number of Sub-Streams in Demultiplexer

                                                          Modulation Mode          Number of Sub-
                                                                                  Streams, Nsubstreams
                                                                16-QAM                       8
                                                                64-QAM                      12
                                                               128-QAM                       7
                                                               256-QAM                       8
                                                               512-QAM                       9
                                                              1024-QAM                      20
                                                              2048-QAM                      11
                                                              4096-QAM                      24
                                                              8192-QAM                      13
                                                              16384-QAM                     14

Bit-to-cell word demultiplexing is illustrated in Figure 51.


                                                                                         b0,0 b0,1 b0,2

                                                                                         b1,0 b1,1 b1,2
          v0 v1        v2                         Bit to Cell Word
                                                   Demultiplexer


                                                                                         bNsubstreams-1,0 bNsubstreams-1,1 bNsubstreams-1,2



                                                           Figure 51 - Bit-to-Cell Word Demultiplexer
For 16-QAM, 64-QAM, 256-QAM, 1024-QAM and 4096-QAM bit-to-cell word demultiplexing has to be carried
out as described in [DVB-C2] section 6.2.1, Bit to Cell Word Demultiplexer.
Bit-to-cell word demultiplexing is defined as a mapping of the bit-interleaved input bits, vdi, onto the output bits
be,do, where:
          vdi is the input to the demultiplexer;
          di is the input bit number;
          e is the demultiplexer sub-stream index (0 ≤ e < Nsubstreams), which depends on (di modulo Nsubstreams), as
          defined in Table 33 through
          Table 37;
                                         𝑑𝑑𝑑𝑑
          𝑑𝑑𝑑𝑑 = 𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓 �                            � is the output cell number from the demultiplexer;
                                𝑁𝑁𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

          be,do is the output from the demultiplexer.




04/19/23                                                                  CableLabs                                                          121

<!-- PDF_PAGE: 122 -->
CM-SP-PHYv3.1-I20-230419                                                                         Data-Over-Cable Service Interface Specifications


       Table 33 - Parameters for Demultiplexing of Bits to Sub-Streams for 8/9 Code Rate with 128-QAM
                                        Input bit number,                       0        1         2        3         4        5       6
                                        di mod Nsubstreams
                                        Output bit number, e                    6        5         4        1         2        3       0


       Table 34 - Parameters for Demultiplexing of Bits to Sub-Streams for 8/9 Code Rate with 512-QAM

                                Input bit number,                     0         1        2         3        4         5        6       7        8
                                di mod Nsubstreams
                                Output bit number, e                  8         7        6         1        2         3        4       5        0


      Table 35 - Parameters for Demultiplexing of Bits to Sub-Streams for 8/9 Code Rate with 2048-QAM

                       Input bit number,                   0          1         2        3        4         5        6        7        8        9         10
                       di mod Nsubstreams
                       Output bit number, e                10         9         8        7        2         3        4        5        6        1         0


      Table 36 - Parameters for Demultiplexing of Bits to Sub-Streams for 8/9 Code Rate with 8192-QAM
            Input bit number,                   0          1          2        3         4        5        6         7        8        9        10        11        12
            di mod Nsubstreams
            Output bit number, e                12         11         2        3         4        5        6         7        8        9        10        1         0


      Table 37 - Parameters for Demultiplexing of Bits to Sub-Streams for 8/9 Code Rate with 16384-QAM
         Input bit number,                0          1          2         3         4        5         6        7         8        9       10        11        12        13
         di mod Nsubstreams
         Output bit number, e             13         12         2         3         4        5         6        7         8        9       10        11        1         0

For example, in the case of 128-QAM there will be 7 substreams at the output of the bit-to-cell word demultiplexer.
The first 7 bits at the input to the demultiplexer are sent to sub-streams 6, 5, 4, 1, 2, 3, and 0, in that order. The next
7 input bits are also mapped in that order. The cell words are defined from the demultiplexer output as:
                                               �𝑦𝑦0,𝑑𝑑𝑑𝑑 . . 𝑦𝑦𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚 −1,𝑑𝑑𝑑𝑑 � = �𝑏𝑏0,𝑑𝑑𝑑𝑑 . . 𝑏𝑏𝑁𝑁𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠−1,𝑑𝑑𝑑𝑑 �
Note that the non-shortened LDPC codeword size is not divisible by 7. However, with reference to the section on bit
interleaving, it is seen that for 128-QAM the size of the non-shortened codeword has been extended to become a
multiple of 7 through zero-padding. The same comments are applicable to non-shortened 2048-QAM, 8192-QAM
and 16384-QAM codewords.

7.5.5.2.2       Shortened Codewords and Mixed-Modulation Codewords
It is important to emphasize that shortened codewords can have square modulation, non-square modulation or may
be of the mixed-modulation type. The CMTS MUST bypass the bit-to-cell demultiplexer and apply the bit-to-cell
mapping described in this section for all types of shortened codewords, as well as for non-shortened mixed
modulation codewords.
When the bit-to-cell word demultiplexer is bypassed, the bit-to-cell mapping becomes:
         Cell 0: �𝑦𝑦0,0 . . 𝑦𝑦𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚0 −1,0 � = �𝑣𝑣0 … 𝑣𝑣𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚0 −1 �

         Cell 1: �𝑦𝑦0,1 . . 𝑦𝑦𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚1 −1,1 � = �𝑣𝑣𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚0 … 𝑣𝑣𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚0 +𝜂𝜂𝑚𝑚𝑚𝑚𝑚𝑚1 −1 �
         etc.
The modulation assigned to cells 0 and 1 in the previous equations correspond to the ηmod values given by Table 31.
The first cell has the modulation corresponding to ηmod0 and the second cell has the modulation corresponding to
ηmod1. This modulation is defined by the bit loading pattern assigned to the profile to which this codeword belongs.



122                                                                           CableLabs                                                                                 04/19/23

<!-- PDF_PAGE: 123 -->
Physical Layer Specification                                                                          CM-SP-PHYv3.1-I20-230419


This mapping is simply a case of partitioning the interleaved bitstream to blocks of bits of size ηmod0, ηmod1, ...,
ηmodLAST, where the sequence {ηmod0, ηmod1, ..., ηmodLAST} is given by the bit loading pattern of the profile to which
this codeword belongs.
Let ηmodLAST correspond to the bit loading of the last cell of the sequence. It is possible that the shortened and/or
mixed-modulation codeword at the output of the bit interleaver might not have sufficient bits to complete this cell.
In this case zero-padding of the input bitstream has to be used for cell completion.

7.5.5.3    Randomization
The CMTS MUST randomize cell words of data subcarriers, NCP subcarriers and PLC subcarriers, just before
mapping these onto QAM constellations, as described in this section.
The CMTS MUST also introduce BPSK-modulated subcarriers for the following subcarriers during the
randomization process, as described in this section.
a) Zero-bit-loaded subcarriers of the codewords of individual profiles
b) Zero-bit-loaded subcarriers in the NCP segment
c) Zero-bit-loaded subcarriers that are introduced to complete the symbol
NCP and zero bit-loading are described in Section 7.5.5.5.
The wordlength (ηMOD) of a cell word ranges from 4 bits for 16-QAM to 14 bits for 16384-QAM.
For 16-QAM to 4096-QAM the CMTS MUST randomize each cell word through a bit-wise exclusive-OR operation
with the nMOD least significant bits (LSBs) of the 12-bit register D0 of the linear feedback shift register (LFSR)
shown in Figure 52 - Linear Feedback Shift Register for Randomization Sequence.
For 8192-QAM the CMTS MUST randomize the 13 bits of the cell word through a bit-wise exclusive-OR operation
with the 12 bits of register D0 and the LSB of register D1 of Figure 52 - Linear Feedback Shift Register for
Randomization Sequence, as given below:
                       (𝑧𝑧0 . . 𝑧𝑧12 ) = (𝑦𝑦0 . . 𝑦𝑦12 )𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏 (𝐷𝐷0 [0]. . D0 [𝜂𝜂𝑀𝑀𝑀𝑀𝑀𝑀 − 1] 𝐷𝐷1 [0])
For 16384-QAM the CMTS MUST randomize the 14 bits of the cell word through a bit-wise exclusive-OR
operation with the 12 bits of register D0 and the 2 LSBs of register D1 of Figure 52 - Linear Feedback Shift Register
for Randomization Sequence, as given below:
                   (𝑧𝑧0 . . 𝑧𝑧13 ) = (𝑦𝑦0 . . 𝑦𝑦13 )𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏𝑏 (𝐷𝐷0 [0]. . D0 [𝜂𝜂𝑀𝑀𝑀𝑀𝑀𝑀 − 1] 𝐷𝐷1 [0] 𝐷𝐷1 [1])
NCP subcarrier cell words are 2-bit for QPSK, 4-bit for 16-QAM or 6-bit for 64-QAM. The CMTS MUST
randomize these through bit-wise exclusive-OR operation with the 2, 4 or 6 LSBs of the 12-bit register D0.
The CMTS MUST set the zero-bit-loaded subcarriers in the data segment and NCP segment to the BPSK
modulation given by LSB of register D0.
                                                             𝑧𝑧0 = 𝐷𝐷0 [0]
The CMTS MUST clock the LFSR once, after each of the previous operations.




                    Figure 52 - Linear Feedback Shift Register for Randomization Sequence



04/19/23                                                   CableLabs                                                     123

<!-- PDF_PAGE: 124 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


The LFSR is defined by the following polynomial in GF[212].
          x2 + x + α11
The GF[212] is defined through polynomial algebra modulo the polynomial:
          α12 + α6 + α4 + α + 1
Each 12-bit GF[212]. element is a polynomial of α with a maximum degree of 11. The coefficient of α0 is referred to
as the LSB and the coefficient of α11 is referred to as the MSB.
This LFSR is initialized to the hexadecimal numbers given below:
          D0 = "555"
          D1 = "AAA"
This initialization is carried out at the beginning of an OFDM symbol, synchronized to the preamble of the PLC.
Since the PLC subcarriers are inserted after time and frequency interleaving and data subcarriers are randomized
before time and frequency interleaving, the following explanation is provided about how randomization is
synchronized to the PLC.
Note that the first subcarrier of an OFDM symbol passes through the time interleaver arm with zero delay. Therefore
the LFSR is initialized when this subcarrier is part of the OFDM symbol following the last OFDM symbol carrying
the PLC preamble. Hence LFSR is initialized once for every 128 OFDM symbols.
The first subcarrier referred to previously can be a data subcarrier or a scattered pilot placeholder because both of
these are time interleaved. If it is a data subcarrier then the cell word of that data subcarrier is randomized with the
initialized values of D0 and D1, namely hexadecimal "555" and "AAA". After that the LFSR is clocked once. If the
first subcarrier mentioned previously is a scattered pilot placeholder the LFSR is initialized but it is not clocked.
This is because the LFSR is clocked only after each data or NCP subcarrier (including zero-bit-loaded subcarriers).
To illustrate this by example, let the first five subcarriers of the symbol in which the randomizer is initialized be {A0,
SP, A1, Z, A2}, where Ai denotes a data subcarrier, SP denotes scattered-pilot and Z denotes zero-bit-loaded
subcarrier. The 24-bit randomizer concatenations "D1 & D0" corresponding to these five subcarrier locations are
{"AAA555", "FFFADF", "FFFADF", "520799", "2B9828"}. The randomizer contents are not used for scattered
pilot SP and hence randomizer linear feedback shift register is not clocked for this subcarrier. The randomizer values
used for the three data subcarriers A0, A1, and A2 are "AAA555", "FFFADF", and "2B9828", respectively. The
randomizer value used for the zero-bit-loaded subcarrier is "520799". This zero-bit-loaded subcarrier corresponds to
the BPSK constellation point (-1 + j0) since D0[0] = 1.

7.5.5.4    Cell Word Mapping into I/Q Constellations
The CMTS MUST modulate each randomized cell word (z0..znMOD-1), from the randomizer described in Section
7.5.5.3 using a BPSK, QPSK, 16-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM, 1024-QAM, 2048-QAM,
4096-QAM, 8192-QAM or 16384-QAM constellation as described in Annex A.

7.5.5.5    Transmitter Bit Loading for Symbol Mapping
All subcarriers of an OFDM symbol may not have the same constellation; the constellation for each subcarrier is
given in a table that details the bit loading pattern. This bit-loading pattern may change from profile to profile. This
section describes how the bits to symbol mapping is performed, with reference to a bit-loading pattern, in the
presence of interleaving, continuous pilots, scattered pilots and excluded subcarriers.
Excluded subcarriers are subcarriers that are forced to zero-valued modulation at the transmitter. Subcarriers are
excluded to prevent interference to other transmissions that occupy the same spectrum as the DOCSIS 3.1 OFDM
transmission, for example, to accommodate legacy channels. Subcarriers are also excluded outside of the active
OFDM bandwidth.
Excluded subcarriers are common to all profiles. The non-excluded subcarriers are referred to as active subcarriers.
Active subcarriers are never zero-valued. The notation S(E) is used here to define the set of excluded subcarriers.
This set will never be empty because there are always excluded subcarriers at the edges of the OFDM channel.



124                                                  CableLabs                                                04/19/23

<!-- PDF_PAGE: 125 -->
Physical Layer Specification                                                            CM-SP-PHYv3.1-I20-230419


Continuous pilots are pilots that occur at the same frequency location in every OFDM symbol. The notation S(C) is
used here to define the set of continuous pilots.
The PLC resides in a contiguous set of subcarriers in the OFDM channel. The CMTS adds the PLC to the OFDM
channel after time and frequency interleaving; the CM extracts the PLC subcarriers before frequency and time de-
interleaving. These subcarriers occupy the same spectral locations in every symbol. The notation S(P) is used here to
define the set of PLC subcarriers.
For bit loading, continuous pilots and the PLC are treated in the same manner as excluded subcarriers; hence, the set
of subcarriers that includes the PLC, continuous pilots and excluded subcarriers is defined as:
                                              S(PCE) = S(P) ∪ S(C) ∪ S(E)
The subcarriers in the set S(PCE) do not carry data (PLC carry signaling information). The other subcarriers that do
not carry data are the scattered pilots. However, scattered pilots are not included in the set S(PCE) because they do not
occupy the same spectral locations in every OFDM symbol.
The modulation order of the data subcarriers is defined using bit-loading profiles. These profiles include the option
for zero bit-loading. Such subcarriers are referred to as zero-bit-loaded subcarriers and are BPSK modulated using
the randomizer LSB, as described in Section 7.5.5.3.
All active subcarriers with the exception of pilots are transmitted with the same average power. Pilots are
transmitted boosted by a factor of 2 in amplitude (approximately 6 dB).
Scattered pilots do not occur at the same frequency in every symbol; in some cases scattered pilots will overlap with
continuous pilots. If a scattered pilot overlaps with a continuous pilot, then that pilot is no longer considered to be a
scattered pilot. It is treated as a continuous pilot.
Because the locations of scattered pilots change from one OFDM symbol to another, the number of overlapping
continuous and scattered pilots changes from symbol to symbol. Since overlapping pilots are treated as continuous
pilots, the number of scattered pilots changes from symbol to symbol.
The following notation is used here:
         N: The total number of subcarriers in the OFDM symbol, equaling either 4096 or 8192
         NC: The number of continuous pilots in an OFDM symbol
         NS: The number of scattered pilots in an OFDM symbol
         NE: The number of excluded subcarriers in an OFDM symbol
         NP: The number of PLC subcarriers in an OFDM symbol
         ND: The number of data subcarriers in an OFDM symbol
The values of N, NC, NE, and NP do not change from symbol to symbol for a given OFDM template; the values of NS
and ND change from symbol to symbol.
The following equation holds for all symbols:
                                          N = NC + NS + NE + NP + ND
The value of N is 4096 for 50 kHz subcarrier spacing and 8192 for 25 kHz subcarrier spacing. From this equation it
is clear that (NS + ND) is a constant for a given OFDM template. Therefore, although the number of data subcarriers
(ND) and the number of scattered pilots (NS) in an OFDM symbol changes from symbol to symbol, the sum of these
two numbers is invariant over all symbols. Interleaving and de-interleaving are applied to the set of data subcarriers
and scattered pilots of size NI = ND + NS.

7.5.5.5.1     Bit Loading
The bit loading pattern defines the QAM constellations assigned to each of the 4096 or 8192 subcarriers of the
OFDM transmission. This bit loading pattern can change from profile to profile. Continuous pilot locations, PLC
locations and exclusion bands are defined separately, and override the values defined in the bit-loading profile. Let
the bit loading pattern for profile i be defined as Ai(k), where:



04/19/23                                             CableLabs                                                      125

<!-- PDF_PAGE: 126 -->
CM-SP-PHYv3.1-I20-230419                                           Data-Over-Cable Service Interface Specifications


         k is the subcarrier index that goes from 0 to (N-1)
         N is either 4096 or 8192
         𝐴𝐴𝑖𝑖 (𝑘𝑘) ∈ {0,4,6,7,8,9,10,11,12,13,14} A value of 0 indicates that the subcarrier k is zero-bit-loaded. Other
         values indicate that the modulation of subcarrier k is QAM with order 2Ai(k).
Let the sequence {𝐴𝐴𝑖𝑖 (𝑘𝑘), 𝑘𝑘 = 0, 1, … , (𝑁𝑁 − 1), 𝑘𝑘 ∉ 𝑆𝑆 𝑃𝑃𝑃𝑃𝑃𝑃 } be arranged as NI consecutive values of another
sequence:
         Bi(k), k = 0, 1, ...,    (NI−1)
Given the locations of the excluded subcarriers, continuous pilots and the PLC in the OFDM template, it is possible
to obtain the bit-loading pattern Bi(k) that is applicable only to spectral locations excluding excluded subcarriers,
continuous pilots, and PLC subcarriers. However, note that Bi(k) does contain the spectral locations occupied by
scattered pilots; these locations change from symbol to symbol.
It is more convenient to define bit loading profiles in the domain in which subcarriers are transmitted. It is in this
domain that signal-to-noise-ratios of subcarriers are calculated. Furthermore, defining the bit-loading patterns in the
transmission domain allows significant data compression to be achieved, because a relatively large number of
contiguous spectral locations can share the same QAM constellation.
Although the bit loading pattern is defined in the domain in which subcarriers are transmitted, the bit loading is not
applied in that domain. Bit loading is applied prior to interleaving, as shown in Figure 53. Hence there is a
permutation mapping of subcarriers, defined by the interleaving function, between the domain in which bit loading
is applied to subcarriers and the domain in which subcarriers are transmitted.




                             Figure 53 - Bit Loading, Symbol Mapping, and Interleaving
The excluded subcarriers, PLC subcarriers, and continuous pilots are excluded from the processes of interleaving
and de-interleaving; scattered pilots and data subcarriers are subject to interleaving and de-interleaving. Hence, the
total number of subcarriers that pass through the interleaver and de-interleaver is NI = (ND + NS) and this number
does not change from symbol to symbol.
The interleaver introduces a 1-1 permutation mapping P on the NI subcarriers. Although interleaving consists of a
cascade of two components, namely time and frequency interleaving, it is only frequency interleaving that defines
the mapping P. This is because time interleaving does not disturb the frequency locations of subcarriers.
The corresponding permutation mapping applied at the receiver de-interleaver is P-1.
In order to perform bit-loading, it is necessary to work out the bit loading pattern at the node at which it is applied,
i.e., at the input to the interleavers. This is given by:
         Ci(k) = P-1(Bi(k))



126                                                    CableLabs                                                  04/19/23

<!-- PDF_PAGE: 127 -->
Physical Layer Specification                                                             CM-SP-PHYv3.1-I20-230419


Since the time interleaver does not change the frequency locations of subcarriers, the sequence Ci(k) is obtained by
sending {Bi(k), k = 1, 2, ..., NI−1} through the frequency de-interleaver.
Note that Ci(k) gives the bit-loading pattern for NI subcarriers. Yet, some of these subcarriers are scattered pilots that
have to be avoided in the bit-loading process. Hence, a two-dimensional binary pattern D(k, j) is used to identify
subcarriers to be avoided during the process of bit-loading. Because the scattered pilot pattern has a periodicity of
128 in the time dimension, this binary pattern also has periodicity 128 in the column dimension j.
         D(k, j) is defined for k = 0, 1, ..., (NI−1) and for j = 0, 1, ..., 127
The process to create the binary pattern D(k, j) begins with the transmitted scattered pilot pattern defined in Section
7.5.6. There are two scattered pilot patterns, one for 4K FFTs and the other for 8K FFTs; both patterns are defined in
reference to the preamble of the PLC and have a periodicity of 128 symbols.
The CMTS executes the following steps to obtain the pattern D(k, j):
1.   Define a two-dimensional binary array P(k, j) in the subcarrier transmitted domain that contains a one for each
     scattered pilot location and zero otherwise:
         P(k, j) for k = 0, 1, ..., (N−1) and for j = 0, 1, ..., 127
         Here, the value of N is either 4096 or 8192. The first column of this binary sequence corresponds to the
         first OFDM symbol following the preamble of the PLC.
2.   Exclude the rows corresponding to excluded subcarriers, continuous pilots, and PLC from the two-dimensional
     array P(k, j) to give an array Q(k, j). The number of rows of the resulting array is NI and the number of columns
     is 128.
3.   Pass this two-dimensional binary array Q(k, j) through the frequency de-interleaver and then the time de-
     interleaver, with each column treated as an OFDM symbol. After the 128 columns of the pattern have been
     input into the interleaver, re-insert the first M columns, where M is the depth of the time interleaver. This is
     equivalent to periodically extending Q(k, j) along the dimension j and passing (128+M) columns of this
     extended sequence through the frequency de-interleaver and the time de-interleaver.
4.   Discard the first M symbols coming out of the time de-interleaver and collect the remaining 128 columns into
     an array to give the binary two-dimensional array D(k, j) of size (NI × 128).
For bit loading the CMTS accesses the appropriate column j of the binary pattern bit D(k, j) together with the
appropriate bit loading profile Ci(k).
If the value of the bit D(k, j) is 1, the CMTS MUST skip this subcarrier k and move to the next subcarrier. This
subcarrier is included as a placeholder for a scattered pilot that will be inserted in this subcarrier location after
interleaving. After each symbol the column index j has to be incremented modulo 128.
The CMTS MUST use this binary two-dimensional array D(k, j) of size (N_I x 128) in order to do bit-loading of
OFDM subcarriers, as described earlier in this section. The corresponding operation in the CM is de-mapping the
QAM subcarriers to get Log-Likelihood-Ratios (LLRs) corresponding to the transmitted bits. This operation,
described below, is much simpler than the mapping operation in the transmitter.
The scattered pilots and data subcarriers of every received symbol are subjected to frequency and time de-
interleaving. The scattered pilots have to be tagged so that these can be discarded at the output of the time and
frequency de-interleavers. This gives NI subcarriers for every OFDM symbol. The CM accesses theses NI de-
interleaved subcarriers together with the bit-loading pattern Ci(k) to implement the de-mapping of the QAM
subcarriers into LLRs. If the subcarrier k happens to be a scattered pilot, then this subcarrier, as well as the
corresponding value Ci(k), is skipped and the CM moves to the next subcarrier (k+1).

7.5.5.5.2      NCP Insertion
Next Codeword Pointers (NCPs) point to the beginning of codewords in a symbol, counting only data subcarriers of
that symbol, including zero-bit-loaded subcarriers and not including the locations reserved for the scattered pilots.
The format of an NCP is described in Section 8.3.4, which also describes the FEC applied to the NCP. Each FEC
encoded NCP is 48 bits wide. NCPs may be modulated using QPSK, 16-QAM or 64-QAM and this modulation is
signaled by PLC. In addition to the NCPs carrying next codeword pointers, there will also be an NCP carrying the



04/19/23                                              CableLabs                                                        127

<!-- PDF_PAGE: 128 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


CRC for all the NCPs of the symbol. The CRC is generated as described in Annex E. As the NCPs are constructed
while the OFDM symbols are being constructed, the NCPs are inserted in the opposite direction to data and
beginning from the opposite end. Data is inserted beginning from the low frequency towards the high frequency end.
The NCPs are inserted from the high frequency end towards the low frequency end.
Note that NI subcarriers in each symbol are subjected to the data and NCP mapping operation. These subcarriers
consist of data subcarriers and scattered pilot place-holder subcarriers as described in the preceding section. During
the course of mapping data or NCP subcarriers, if a scattered pilot placeholder is encountered, this is skipped.
The figure given below shows an OFDM symbol comprising a Data segment, an NCP segment and a "Filler"
segment. "Filler" subcarriers have to be inserted into the OFDM symbol when the number of codewords in the
OFDM symbol has exceeded the upper limit or when it is not possible to begin a new codeword because of
insufficient space to include an NCP. These filler subcarriers are zero-bit-loaded.
The CMTS MUST only use zero-bit-loaded filler subcarriers when the number of codewords has exceeded the upper
limit or when it is not possible to begin a new codeword because of insufficient space to include a NCP, or when
there is no data to transmit. The CMTS MUST define the location of a segment of zero-bit-loaded subcarriers using
an NCP with Z-bit set to one as described in Section 8.3.4.1. The filler subcarriers always fill the remaining
subcarriers of the symbol.
If the CMTS has no data to transmit, the CMTS MUST adopt one of the following two options:
1. Insert zero-bit-loaded filler subcarriers into OFDM symbols as described in this section, or
2. Insert stuffing pattern of 0xFF bytes into codewords as described in Section 8.3.2.
Data segment contains codewords belonging to several profiles. Some of the subcarriers may be zero-bit-loaded in
some of the profiles. The NCP also has a profile. This profile allows some of the subcarriers in the NCP segment to
be zero-bit-loaded. Note that the NCP modulation is a constant for given OFDM transmission. It does not change
from subcarrier to subcarrier.
Note that throughout the symbol there can be scattered pilot placeholders. These have to be skipped during the
insertion of data subcarriers, NCP subcarriers or filler subcarriers. Moreover, these have to be tagged before sending
the NI subcarriers through the time and frequency interleavers. Scattered pilots will be inserted in their place with the
appropriate BPSK modulation before the data is transmitted.




                                              Figure 54 - NCP Insertion




128                                                  CableLabs                                               04/19/23

<!-- PDF_PAGE: 129 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419


7.5.6     Interleaving and De-interleaving
To minimize the impacts of burst noise and ingress on the DOCSIS signals, time and frequency interleaving are
applied to OFDM symbols in the following order: time interleaving, then frequency interleaving. These interleaving
methods are discussed in this section.
The time interleaver is a convolutional interleaver that operates in the time dimension on individual subcarriers of a
sequence of OFDM symbols. The time interleaver does not change the frequency location of any OFDM subcarrier.
A burst event can reduce the SNR of all the subcarriers of one or two consecutive OFDM symbols; the purpose of
the time interleaver is to disperse these burst-affected OFDM subcarriers between M successive OFDM symbols,
where M is the interleaver depth. This dispersion distributes the burst-affected subcarriers uniformly over a number
of LDPC codewords.
The frequency interleaver works along the frequency dimension. The frequency interleaver changes the frequency
locations of individual OFDM subcarriers; latency is not introduced, except for the data store and read latency. The
aim of frequency interleaving is to disperse ingress, e.g., LTE that affects a number of consecutive subcarriers over
the entire OFDM symbol. Frequency interleaving distributes the burst-affected subcarriers over a number of LDPC
codewords.
The CMTS first applies a time interleaver to an OFDM symbol worth of NI subcarriers to get a new set of NI
subcarriers. These NI subcarriers are made up of ND data subcarriers and NS scattered pilots.
                                                   NI = ND + NS
It is important to note that although ND and NS are not the same for every OFDM symbol, the value of NI is a
constant for all OFDM symbols in a given system configuration. The value of NI is a function of the channel
bandwidth, number of excluded subcarriers, number of PLC subcarriers and the number of continuous pilots. The
CMTS then subjects these NI subcarriers to frequency interleaving. The value of NI does not exceed 7537 for 8K
FFT mode and 3745 for the 4K FFT mode.
Note that both time and frequency interleaving are applied only to data subcarriers and scattered pilots. Continuous
pilot, subcarriers that have been excluded (used to support legacy channels in spectral regions, for example) and the
subcarriers of the physical layer link channel (PLC) are not interleaved. The CMTS MUST NOT interleave
continuous pilots, excluded subcarriers or the subcarriers of the PLC.

7.5.6.1    Time Interleaving
The CMTS MUST time interleave as described in this section. The CMTS MUST time interleave after OFDM
symbols have been mapped to QAM constellations and before they are frequency interleaved.
The time interleaver is a convolutional interleaver that operates at the OFDM subcarrier level. If the depth of the
interleaver is M, then there are M branches, as shown in Figure 55.




                                       Figure 55 - Time Interleaver Structure




04/19/23                                            CableLabs                                                        129

<!-- PDF_PAGE: 130 -->
CM-SP-PHYv3.1-I20-230419                                                Data-Over-Cable Service Interface Specifications


The CMTS MUST support a maximum value of M equal to 32 for 20 μs symbol duration (50 kHz subcarrier
spacing) and 16 for 40 μs symbol duration (25 kHz subcarrier spacing).
The CMTS MUST support all values of M from 1 to the maximum value of M (inclusive of both limits).
Each branch is a delay line; the input and output will always be connected to the same delay line. This delay line
will be clocked to insert a new subcarrier into the delay line and to extract a subcarrier from the delay line. Next, the
commutator switches at the input, and the output will move to the next delay line in the direction shown by the arrows
in Figure 55. After the delay line with the largest delay, the switch will move to the delay line with zero delay.
The lowest frequency subcarrier of an OFDM symbol always goes through the branch with zero delay. Then the
commutator switch at input and the corresponding commutator switch at output are rotated by one position for every
new subcarrier.
The value of J is given by the following equation:
                                                                       𝑁𝑁
                                                      𝐽𝐽 = 𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐 � 𝐼𝐼�
                                                                       𝑀𝑀

Here, NI is the number of data subcarriers and scattered pilots in an OFDM symbol. See Section 7.5.6.3 for details
on interleaving scattered pilots.
If NI were not divisible by M, all of the branches would not be filled. Therefore, "dummy subcarriers" are added to the
symbol to make the number of subcarriers equal to a multiple of M. The number of dummy subcarriers is given by:
                                                       𝐽𝐽 × 𝑀𝑀 − 𝑁𝑁𝐼𝐼
The dummy subcarriers are added for definition purposes only; at the output of the interleaver these dummy subcarriers
are discarded. An implementation will use a single linear address space for all the delay lines in Figure 55. Writing
and reading dummy subcarriers will not be needed.

7.5.6.2     Frequency Interleaving
The CMTS MUST frequency interleave OFDM symbols as described in this section. The CMTS MUST frequency
interleave after OFDM symbols have been time interleaved.
The frequency interleaver works on individual OFDM symbols. Each symbol to be interleaved consists of NI
subcarriers. These NI subcarriers are made up of ND data subcarriers and NS scattered pilot placeholders. Although
ND and NS are not the same for every symbol, the value of NI is a constant for all OFDM symbols in a given system
configuration. See Section 7.5.6.3 for details on interleaving scattered pilots.
There is a 2-D store comprising 128 rows and K columns. If the number of data subcarriers and scattered pilots in
the OFDM symbol is NI, then the number of columns, K, is given by the following equation:
                                                                       𝑁𝑁𝐼𝐼
                                                     𝐾𝐾 = 𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐 �          �
                                                                       128

If NI is not an integer multiple of 128, then the last column will only be partially filled during the frequency
interleaving process. The number of data subcarriers in the last column, C, is given by:
                                                 𝐶𝐶 = 𝑁𝑁𝐼𝐼 − 128(𝐾𝐾 − 1)
The frequency interleaver follows the following process; note that rows are numbered 0 to 127, and columns are
numbered from 0 to (K−1):
1.    Write the subcarriers along rows of the 2-D store. Rows are accessed in bit-reversed order. For example, after
      writing in row 0, the next writing operation will be in row 64. This will be followed by writing in row 32 and so
      on. If the row number is less than C, then K subcarriers will be written in the row. Otherwise only (K−1)
      subcarriers will be written. (If the number NI is an integer multiple of 128 then C will be zero. Then K
      subcarriers will be written in every row.)
2.    Rotate columns 0 to (K−2) by an amount given by a 6-bit shift linear feedback (maximal length) shift register.
      This shift register is initialized to a value of 17 at the start of each OFDM symbol. The final column, which may
      be partially full, is not rotated.




130                                                  CableLabs                                                04/19/23

<!-- PDF_PAGE: 131 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419


3.   Read the columns in bit-reverse order, starting at column 0, then column bit-reverse(1), then column bit-
     reverse(2), ..., ending at column bit-reverse(K−1). When K is not a power-of-2, bit-reverse(x), for x = 0, ...,
     K−1, is defined by:
         bit-reverse(x) = reverse_bits(x), if reverse_bits(x) < K; OR
         x, if reverse_bits(x) ≥ K
         where reverse_bits(x) is the number obtained by reversing the order of the bits in the m-bit representation
         of x, with m being the number of bits in K.
The structure of the two-dimensional store is shown in Figure 56.




                                   Figure 56 - Two-Dimensional Block Structure
The linear feedback shift register is defined using the following equation in Galois field GF[26]:
                              x(i) = αx(i−1), for i = 1, ..., 127, where x(0) = α5 + α
GF[26] is defined using the polynomial (α6 + α + 1). As this is primitive, powers of α will generate all 63 non-zero
elements of the field. This operation can be represented as the linear feedback shift register, depicted in Figure 57.


                            x5            +             x4      x3       x2      x1      x0
                            α0                                                           α5
                                     Figure 57 - Linear Feedback Shift Register
The binary number x[5:0] is used to rotate the columns. This number is initialized to 17 at the beginning of each
OFDM symbol. The column number 0 is rotated by 17; subsequent columns are rotated by values obtained by
clocking the shift register shown in Figure 57. The rotation applied to the first column is defined in Figure 58.
Subsequent rows are also rotated along the same direction.




04/19/23                                             CableLabs                                                        131

<!-- PDF_PAGE: 132 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications




                                Figure 58 - Frequency Interleaver Rotation Definition
Note that column (K−1) is not rotated, regardless of whether it is full: because all other columns are rotated by a
non-zero amount, there is no need to rotate column (K−1). The C code for interleaver implementation is given in
Appendix I.

7.5.6.3     Interleaving Impact on Continuous Pilots, Scattered Pilots, PLC and Excluded Spectral
            Regions
DOCSIS 3.1 transmissions contain continuous pilots for receiver synchronization and scattered pilots for channel
estimation. In addition, there could be excluded regions to accommodate legacy channels. There will also be a
physical layer link channel (PLC).
The CMTS interleaves scattered pilots and data subcarriers, but does not interleave continuous pilots, the PLC, and
subcarriers belonging to excluded regions. With respect to scattered pilots, it is noted here that CMTS actually
interleaves the subcarriers that are tagged to act as placeholders for scattered pilots, since at the time of interleaving
the scattered pilots have not yet been inserted. The actual BPSK modulation to these placeholder subcarriers is
applied after interleaving as described in Section 7.5.15.
The CMTS inserts scattered pilot placeholders prior to time and frequency interleaving such that when these
placeholders get time and frequency interleaved, the resulting placeholders conform to the required scattered pilot
pattern described in Section 7.5.15.
To accomplish this, the CMTS has to retain a reference pattern for inserting scattered pilot placeholders prior to
interleaving. Since the scattered pilot pattern repeats every 128 symbols, this pattern is a (NI × 128) two-dimensional
bit pattern. A value of one in this bit-pattern indicates the location of a scattered pilot. The CMTS inserts data
subcarriers where this reference pattern has a zero and scattered pilot placeholders where this pattern has a one.
This reference pattern may be derived from the following procedure:
1.    In the time-frequency plane, create a two-dimensional bit-pattern of zeros and ones from the transmitted
      "diagonal" scattered pilot patterns described in Section 7.5.15. This pattern has a periodicity of 128 symbols and
      has a value of one for a scattered pilot location and zero otherwise. Let the time axis be horizontal and the
      frequency axis vertical.
2.    Delete all horizontal lines containing continuous pilots, excluded subcarriers, and PLC from the above
      mentioned two-dimensional bit pattern; note that some scattered pilots could coincide with continuous pilots.
      These locations are treated as continuous pilot locations.
3.    Send the resulting bit-pattern through the frequency de-interleaver and the time de-interleaver in succession.
      This will give another two-dimensional bit pattern that has a periodicity of 128 symbols. The appropriate 128-
      symbol segment of this bit-pattern is chosen as the reference bit pattern referred to above.
Note that the CMTS has to synchronize the scattered pilot pattern to the PLC preamble, as described in Section
7.5.15. This uniquely defines the 128-symbol segment that has to be used as the reference pattern.


132                                                   CableLabs                                                 04/19/23

<!-- PDF_PAGE: 133 -->
Physical Layer Specification                                                            CM-SP-PHYv3.1-I20-230419


Scattered pilots are not in the same subcarrier location in every symbol; hence some scattered pilots can coincide
with continuous pilots in some OFDM symbols. The size of the overlap between the set of scattered pilots and the
set of continuous pilots will change from symbol to symbol. As a result, the number of data subcarriers in a symbol
will not be the same for all OFDM symbols. Note that in the nomenclature used below, when a scattered pilot
coincides with a continuous pilot, then that pilot is referred to as a continuous pilot.
Although the number of data subcarriers can change from symbol to symbol, the number of data subcarriers and
scattered pilots are the same for every symbol. This is referred to as NI in this section. Let ND denote the number of
data subcarriers in a symbol and NS denote the number of scattered pilots in a symbol. These two parameters, i.e., ND
and NS, will change from symbol to symbol. However, the sum of these two, i.e., NI is a constant for a given system
configuration.
                                                      NI = NS + ND
Hence the number of OFDM subcarriers that are interleaved does not change from symbol to symbol. This is
important, because if not for this, the output of the convolutional time interleaver may have dummy or unused
subcarriers in the middle of interleaved OFDM symbols.
The insertion of continuous pilots, PLC and excluded regions happens after both time and frequency interleaving.
Interleaving data and scattered pilots together has another important advantage. This is to do with bit loading. A
transmitted profile is said to have non-uniform bit loading if the QAM constellation that is applied to subcarriers is
not constant over the entire frequency band. If the data subcarriers are interleaved and scattered pilots are added
later, then the data subcarriers will have to be shifted to accommodate the scattered pilots. This shift will be different
from symbol to symbol, and this complicates non-uniform bit-loading. Hence, having the scattered pilots in-place
during the bit-loading process greatly simplifies the bit loading operation. The insertion of continuous pilots, PLC
and excluded regions also results in shift of data subcarriers, but this shift is the same for every symbol, and can
easily be accounted for in the bit loading process.
The CMTS only interleaves data subcarriers and scattered pilots, and therefore only needs information about the
number of data subcarriers and scattered pilots per symbol. In addition, the interleaver does not need to know what
modulation has been applied to an individual data subcarrier. Regardless of modulation scheme, all OFDM symbols
will have the same number of data subcarriers and scattered pilots, and the modulation pattern of these data
subcarriers may change from symbol to symbol.

7.5.7     IDFT

7.5.7.1    Downstream Transmitter Inverse Discrete Fourier Transform
The CMTS transmitter MUST use the IDFT definition and subcarrier referencing method described in this section.
This section defines the inverse discrete Fourier transform (IDFT) used in the CMTS transmitter for DOCSIS 3.1.
OFDM subcarrier referencing for other definitions such as PLC location, continuous pilots, exclusion bands and bit
loading is also described.
The OFDM signal assembled in the frequency domain consists of 4096 subcarriers for the 4K FFT and 8192
subcarriers for the 8K FFT. The OFDM signal is composed of:
    •     Data subcarriers
    •     Scattered pilots
    •     Continuous pilots
    •     PLC subcarriers
    •     Excluded subcarriers that are zero valued
This signal is described according to the following IDFT equation:


                                                                   , for i = 0, 1, …, (N−1)




04/19/23                                              CableLabs                                                     133

<!-- PDF_PAGE: 134 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


The resulting time domain discrete signal, x(i), is a baseband complex-valued signal, sampled at 204.8 Msamples
per second.
In this definition of the IDFT:
           X(0) is the lowest frequency component;
           X(N−1) is the highest frequency component.
The IDFT is illustrated in Figure 59.




                                    Figure 59 - Inverse Discrete Fourier Transform
The sample rate in the time domain is 204.8 Msamples/s. Hence, the N samples of the discrete Fourier transform
cover a frequency range of 204.8 MHz. This gives the subcarrier spacing shown in Table 38.
                                            Table 38 - Subcarrier Spacing

                                           IDFT Size N       Carrier Spacing
                                                 4096             50 kHz
                                                 8192             25 kHz

The maximum channel bandwidth is 192 MHz; this corresponds to 3841 subcarriers in 4K mode and 7681
subcarriers in 8K mode. The active bandwidth of the channel is expected to be 190 MHz; this corresponds to 3800
subcarriers in 4K mode and 7600 subcarriers in 8K mode.

7.5.7.2     Subcarrier Referencing
It is necessary to refer to specific OFDM subcarriers for several definitions:
      a.   Defining continuous pilot locations
      b.   Defining exclusion bands and excluded individual subcarriers
      c.   Defining bit loading profiles
Each of these definitions uses the index k of the equation defined in the preceding section to refer to a specific
subcarrier.




134                                                     CableLabs                                             04/19/23

<!-- PDF_PAGE: 135 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419


The subcarrier index goes from 0 to 4095 for the 4K FFT and from 0 to 8191 for the 8K FFT; each of these
definitions is limited to these subcarrier indices.
The PLC is also defined with reference to k=0. The OFDM template carried by the PLC defines the subcarrier index of
the lowest frequency subcarrier of the PLC. Hence, once the CM detects the PLC, the CM knows the location of k=0.
Since the FFT size is also known, it is possible to precisely compute the FFT of the data channel containing the PLC.
Note that scattered pilot placement is not referenced to k=0; instead, it is referenced directly to the PLC preamble.

7.5.8     Cyclic Prefix and Windowing
This section describes how cyclic prefixes are inserted and how a window is applied to the output of the IDFT at the
CMTS and how they are handled by the CM.
The addition of a cyclic prefix enables the receiver to overcome the effects of inter-symbol-interference caused by
micro-reflections in the channel. Windowing maximizes channel capacity by sharpening the edges of the spectrum
of the OFDM signal. Spectral edges occur at the two ends of the spectrum of the OFDM symbol, as well as at the
ends of internal exclusion bands.
The number of active OFDM subcarriers can be increased by sharpening these spectral edges. However, sharper
spectral edges in the frequency domain imply longer tapered regions in the time domain, resulting in increased
symbol duration and reduction in throughput. Therefore, there is an optimum amount of tapering that maximizes
channel capacity. This optimum is a function of channel bandwidth as well as the number of exclusion bands.

7.5.8.1    Cyclic Prefix Insertion and Windowing
The CMTS MUST follow the procedure described in Section 7.4.10.1 for cyclic prefix insertion and windowing,
using CMTS specific cyclic prefix and roll-off period values.
The CMTS MUST support cyclic prefix extension and windowing as described in Section 7.4.10.1.
The CMTS MUST support the cyclic prefix values defined in Table 39 - Downstream Cyclic Prefix (CP) Values for
both 4K and 8K FFTs.
The CM MUST support the cyclic prefix values listed in Table 39 - Downstream Cyclic Prefix (CP) Values for both
4K and 8K FFTs.
                                 Table 39 - Downstream Cyclic Prefix (CP) Values

                                Cyclic Prefix (μs)        Cyclic Prefix Samples (Ncp)
                                      0.9375                           192
                                       1.25                            256
                                        2.5                            512
                                       3.75                            768
                                        5.0                           1024

The cyclic prefix (in μs) are converted into samples using the sample rate of 204.8 Msamples/s and is an integer
multiple of: 1/64 * 20 μs.
The CMTS MUST support the five parameter values specified for this roll-off listed in Table 40 - Downstream Roll-
off Period (RP) Values.
                                Table 40 - Downstream Roll-off Period (RP) Values

                              Roll-Off Period (μs)       Roll-Off Period Samples (Nrp)
                                         0                              0
                                      0.3125                           64
                                       0.625                           128
                                      0.9375                           192
                                       1.25                            256




04/19/23                                             CableLabs                                                    135

<!-- PDF_PAGE: 136 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications


The CMTS MUST NOT allow a configuration in which the RP value is ≥ the CP value. The value 0 for the Roll-Off
Period is included for evaluation purposes and is not intended as an operational mode.

7.5.9     Fidelity Requirements
For the purposes of this specification, the number of occupied CEA channels of an OFDM channel is the occupied
bandwidth of the OFDM channel divided by 6 MHz.
CMTSs capable of generating N-channels of legacy DOCSIS plus NOFDM-channels of OFDM per RF port, for
purposes of the DRFI output electrical requirements, the device is said to be capable of generating Neq-channels per
RF port, where Neq = N + 32 * NOFDM "equivalent legacy DOCSIS channels."
An Neq-channel per RF port CMTS MUST comply with all requirements operating with all Neq channels on the RF
port, and MUST comply with all requirements for an Neq'-channel per RF port device operating with Neq' active
channels on the RF port for all values of Neq' less than Neq.
For an OFDM channel there is a) the occupied bandwidth, b) the encompassed spectrum, c) the modulated spectrum,
and d) the number of equivalent legacy DOCSIS channels.
The encompassed spectrum in MHz is 204.8 MHz minus the number of subcarriers in the Band edge Exclusion Sub-
band for the upper and lower band edges (combined) times the subcarrier spacing in MHz. For example, with
subcarrier spacing of 50 kHz and 150 lower band edge subcarriers and 152 upper band edge subcarriers for a total of
302 subcarriers in the two Band edge Exclusion Sub-bands, the encompassed spectrum = 204.8 − 302 * (0.05) =
189.7 MHz. The encompassed spectrum is also equal to the center frequency of the highest frequency modulated
subcarrier minus the center frequency of the lowest frequency modulated subcarrier in an OFDM channel, plus the
subcarrier spacing.
The modulated spectrum of an OFDM channel is the encompassed spectrum minus the total spectrum in the Internal
Excluded Sub-bands of the channel, where the total spectrum in the Internal Excluded Sub-bands is equal to the
number of subcarriers in all of the Internal Excluded Sub-bands of the OFDM channel multiplied by the subcarrier
spacing of the OFDM channel. In the previous example, if there are 188 subcarriers total in three Internal Exclusion
Sub-bands, then the total spectrum in the Internal Excluded Sub-bands (in MHz) is 188 * 0.05 = 9.4 MHz, and the
modulated spectrum is 189.7 MHz − 9.4 MHz = 180.3 MHz.
The occupied bandwidth is a multiple of 6 MHz, with a minimum of 24 MHz, and consists of all CEA channels
which include the modulated spectrum plus taper region shaped by the OFDM channels' transmit windowing; the
out-of-band spurious emissions requirements apply outside the occupied bandwidth. With a 1 MHz taper region on
each band edge of the OFDM channel, shaped by the transmit windowing function, encompassed spectrum of 189.7
MHz may provide 192 MHz of occupied bandwidth.
The number of equivalent active legacy DOCSIS channels in the OFDM channel Neq' is the ceiling function applied
to the modulated spectrum divided by 6 MHz. For the example, the number of equivalent legacy DOCSIS channels
in the OFDM channel is ceiling(180.3 MHz / 6 MHz) = 31.
For an Neq-channel per RF port device, the applicable maximum power per channel and spurious emissions
requirements are defined using a value of N* = minimum(4Neq', ceiling[Neq/4]) for Neq' < Neq/4, and N* = Neq'
otherwise.
These specifications assume that the CMTS will be terminated with a 75 Ohm load.

7.5.9.1    CMTS Output Electrical Requirements
For OFDM, all modulated subcarriers in an OFDM channel are set to the same average power (except pilots which
are boosted by 6 dB). For purposes of spurious emissions requirements, the "commanded transmit power per
channel" for an equivalent legacy DOCSIS channel is computed as follows:
      •   CMTS power is configured by power per CEA channel and number of occupied CEA channels for each
          OFDM channel.
      •   For each OFDM channel, the total power is Power per CEA channel + 10log10(Number of occupied CEA
          channels) for that OFDM channel.




136                                                CableLabs                                              04/19/23

<!-- PDF_PAGE: 137 -->
Physical Layer Specification                                                             CM-SP-PHYv3.1-I20-230419


    •    CMTS calculates power for data subcarrier and pilots (using total number of non-zero valued (non-
         excluded) subcarriers).
    •    CMTS calculates power in 6 MHz containing PLC.
    •    For the spurious emissions requirements, power calculated for the 6 MHz containing the PLC is the
         commanded average power of an equivalent DOCSIS legacy channel for that OFDM channel.
A CMTS MUST output an OFDM RF modulated signal with the characteristics defined in Table 41 - RF Output
Electrical Requirements, Table 42 - CMTS Output Power, and Table 43 - CMTS Output Out-of-Band Noise and
Spurious Emissions Requirements. Legacy DOCSIS RF modulated signal characteristics are provided in Section
6.2.22.
The condition for these requirements is all Neq' combined channels, legacy DOCSIS channels and equivalent legacy
DOCSIS channels, commanded to the same average power, except for the Single Channel Active Phase Noise,
Diagnostic Carrier Suppression, OFDM Phase Noise, OFDM Diagnostic Suppression, and power difference
requirements, and except as described for Out-of-Band Noise and Spurious Requirements.
                                     Table 41 - RF Output Electrical Requirements

                     Parameter                                                          Value
  Downstream Lower Band Edge of a CMTS                258 MHz.
                                                       (See Item #1 immediately following this table.)
                                                      108 MHz.
                                                       (See Item #2 following this table.)
  Downstream Upper Band Edge of a CMTS                1218 MHz.
                                                       (See Item #3 following this table.)
                                                      1794 MHz.
                                                       (See Item #4 following this table.)
  Level                                               Adjustable. See Table 42.
  Modulation Type                                     See Section 7.5.6.1
  OFDM channels' subcarrier spacing                   25 kHz and 50 kHz
  Inband Spurious, Distortion, and Noise
  576 MHz total occupied bandwidth,
  6 MHz gap (Internal Excluded subcarriers)
  88 equivalent legacy DOCSIS channels.
    See Notes 4, 6

  For measurements below 600 MHz                      ≤ -50 dBr   Average over center 400 kHz subcarriers within gap

  For measurements from 600 MHz to 1002 MHz           ≤ -47 dBr   Average over center 400 kHz subcarriers within gap

  For measurements 1002 MHz to 1218 MHz               ≤ -45 dBr   Average over center 400 kHz subcarriers within gap
  MER in 192 MHz OFDM channel occupied bandwidth

  576 MHz total occupied bandwidth,
  88 equivalent legacy DOCSIS channels. See Notes
  2, 4, 5, 6

  For measurements below 600 MHz                      ≥ 48 dB Any single subcarrier. See Note 1
                                                      ≥ 50 dB Average over the complete OFDM channel.
                                                      See Note 1

  For measurements from 600 MHz to 1002 MHz           ≥ 45 dB Any single subcarrier. See Note 1
                                                      ≥ 47 dB Average over the complete OFDM channel.
                                                      See Note 1

  For measurements 1002 MHz to 1218 MHz               ≥ 43 dB Any single subcarrier. See Note 1
                                                      ≥ 45 dB Average over the complete OFDM channel.
                                                      See Note 1

                                                      Minimal test receiver equalization: See Note 7
                                                        2 dB relief for above requirements
                                                        (e.g., MER > 48 dB becomes MER > 46 dB)




04/19/23                                            CableLabs                                                         137

<!-- PDF_PAGE: 138 -->
CM-SP-PHYv3.1-I20-230419                                                Data-Over-Cable Service Interface Specifications


                      Parameter                                                                 Value
  MER in 24 MHz OFDM channel occupied bandwidth,
  single OFDM channel only, 24 MHz total occupied
  bandwidth:
     See Notes 1, 2, 4, 8

  For measurements below 600 MHz                             ≥ 48 dB   Average over the complete OFDM channel.

  For measurements from 600 MHz to 1002 MHz                  ≥ 45 dB   Average over the complete OFDM channel.

  For measurements 1002 MHz to 1218 MHz                      ≥ 43 dB Average over the complete OFDM channel.
  Phase noise, double sided maximum, Full power CW           1 kHz - 10 kHz:   -48 dBc
  signal 1002 MHz or lower                                   10 kHz - 100 kHz: -56 dBc
                                                             100 kHz - 1 MHz:  -60 dBc
                                                             1 MHz - 10 MHz:   -54 dBc
                                                             10 MHz - 100 MHz: -58 dBc (Note 9)

  Full power 192 MHz OFDM channel block with 6               1 kHz - 10 kHz:        -48 dBc
  MHz in center as Internal Exclusion sub-band + 0           10 kHz - 100 kHz:      -56 dBc
  dBc CW in center, with block not extending beyond
  1002 MHz [CW not processed via FFT]


  Full power 192 MHz OFDM channel block with 24              100 kHz - 1 MHz:       -59 dBc
  MHz in center as Internal Exclusion sub-band +
  6 dBc CW in center, with block not extending beyond
  1002 MHz [CW not processed via FFT]


  Full power 192 MHz OFDM channel block with 30              1 MHz - 10 MHz:        -53dBc
  MHz in center as Internal Exclusion sub-band + 7
  dBc CW in center, with block not extending beyond
  1002 MHz [CW not processed via FFT]
  Output Impedance                                           75 ohms
  Output Return Loss (Note 3)                                > 14 dB within an active output channel from 88 MHz to 750 MHz
                                                             > 13 dB within an active output channel from 750 MHz to 870 MHz
                                                             > 12 dB within an active output channel from 870 MHz to 1218 MHz
                                                             > 12 dB in every inactive channel from 54 MHz to 870 MHz
                                                             > 10 dB in every inactive channel from 870 MHz to 1218 MHz
  Connector                                                  F connector per [ISO/IEC-61169-24] or [SCTE 02]
 Table Notes:
 Note 1. Receiver channel estimation is applied in the test receiver; test receiver does best estimation possible. Transmit windowing is
         applied to potentially interfering channel and selected to be sufficient to suppress cross channel interference.
 Note 2. MER (modulation error ratio) is determined by the cluster variance caused by the transmit waveform at the output of the ideal
         receive matched filter. MER includes all discrete spurious, noise, subcarrier leakage, clock lines, synthesizer products,
         distortion, and other undesired transmitter products. Phase noise up to ±50 kHz of the subcarrier is excluded from inband
         specification, to separate the phase noise and inband spurious requirements as much as possible. In measuring MER, record
         length or carrier tracking loop bandwidth may be adjusted to exclude low frequency phase noise from the measurement. MER
         requirements assume measuring with a calibrated test instrument with its residual MER contribution removed.
 Note 3. Frequency ranges are edge-to-edge.
 Note 4. Phase noise up to 10 MHz offset is mitigated in test receiver processing or by test equipment (latter using hardline carrier from
         modulator, which requires special modulator test port and functionality).
 Note 5. Up to 5 subcarriers in one OFDM channel can be excluded from this requirement.
 Note 6. The measured OFDM channel is allocated 204.8 MHz of spectrum which is free from the other OFDM channel and 32 SC-
         QAM channels which together comprise 576 MHz of occupied bandwidth.
 Note 7. The estimated channel impulse response used by the test receiver is limited to half of length of smallest transmit cyclic prefix.
 Note 8. A single subcarrier in the OFDM channel can be excluded from this requirement, no windowing is applied and minimum CP is
         selected.
 Note 9. Test limit includes 2 dB added to -60 dBc due to the contribution from the modulator noise floor which is allowed by spurious
         emissions.




138                                                        CableLabs                                                       04/19/23

<!-- PDF_PAGE: 139 -->
Physical Layer Specification                                                                   CM-SP-PHYv3.1-I20-230419


The following is an itemized list of RF Output Electrical Requirements based on Table 41 above.
1.     The CMTS MUST support a Downstream Lower Band Edge of 258 MHz.
2.     The CMTS SHOULD support a Downstream Lower Band Edge of 108 MHz.
3.     The CMTS MUST support a Downstream Upper Band Edge of 1218 MHz.
4.     The CMTS MAY support a Downstream Upper Band Edge of 1794 MHz.

7.5.9.1.1         Power per Channel for CMTS
A CMTS MUST generate an RF output with power capabilities as defined in Table 42 - CMTS Output Power.
The CMTS MUST be capable of adjusting channel RF power on a per channel basis as stated in Table 42 - CMTS
Output Power.
If the CMTS has independent modulation capability on a per channel basis for legacy DOCSIS channels, then the
CMTS MUST be capable of adjusting power on a per channel basis for the legacy DOCSIS channels, with each
channel independently meeting the power capabilities defined in Table 42 - CMTS Output Power.
                                                 Table 42 - CMTS Output Power


       for                                             Adjusted Number of Active Channels Combined per RF Port

                          Parameter                                                          Value
     Required power per channel for Neq' channels combined     Required power in dBmV per channel
     onto a single RF port:                                    60 - ceil [3.6*log2(N*)] dBmV
     Range of commanded transmit power per channel             ≥ 8 dB below required power level specified below maintaining full
                                                               fidelity over the 8 dB range
     Range of commanded power per channel; adjusted on a       0 dBc to -2 dBc relative to the highest commanded transmit power
     per channel basis                                         per channel, within an 8 dB absolute window below the highest
                                                               commanded power.
                                                               (See Item # 1 immediately following this table.)
                                                               Required power (in table below) to required power - 8 dB,
                                                               independently on each channel.
                                                               (See Item # 2 following this table.)
     Commanded power per channel step size                     ≤ 0.2 dB Strictly monotonic
     Power difference between any two adjacent channels in     ≤ 0.5 dB
     the 108-1218 MHz downstream spectrum (with
     commanded power difference removed if channel power
     is independently adjustable and/or accounting for pilot
     density variation and subcarrier exclusions)
     Power difference between any two non-adjacent             ≤ 1 dB
     channels in a 48 MHz contiguous bandwidth block (with
     commanded power difference removed if channel power
     is independently adjustable)
     Power difference (normalized for bandwidth) between       ≤ 2 dB
     any two channels OFDM channel blocks or legacy
     DOCSIS channels in the 108 - 1218 MHz downstream
     spectrum (with commanded power difference removed if
     channel power is independently adjustable)
     Power per channel absolute accuracy                       ±2 dB




04/19/23                                                   CableLabs                                                          139

<!-- PDF_PAGE: 140 -->
CM-SP-PHYv3.1-I20-230419                                           Data-Over-Cable Service Interface Specifications



       for                                          Adjusted Number of Active Channels Combined per RF Port

                             Parameter                                                     Value
     Diagnostic carrier suppression (3 modes)
     Mode 1: One channel suppressed                         1) ≥ 50 dB carrier suppression within the occupied bandwidth in
     (See Note 1)                                           any one active channel. (See Item #3 following this table below.)
                                                            When suppressing the carrier ≥ 50 dB within the occupied
                                                            bandwidth in any one active channel, the CMTS controls
                                                            transmissions such that no service impacting discontinuity or
                                                            detriment to the unsuppressed channels occurs. (See item #4
                                                            following this table)

     Mode 2: All channels suppressed except one             2) 50 dB carrier suppression within the occupied bandwidth in
     (See Note 1)                                           every active channel except one. The suppression is not required
                                                            to be glitchless, and the remaining unsuppressed active channel is
                                                            allowed to operate with increased power such as the total power of
                                                            the N' active channels combined. (See Item #3 following this table
                                                            below.)



     Mode 3: All channels suppressed                        3) 50 dB carrier suppression within the occupied bandwidth in
                                                            every active channel. (See Item #3 following the table.)

                                                            The CMTS controls transmissions such that in all three diagnostic
                                                            carrier suppression modes the output return loss of the suppressed
                                                            channel(s) complies with the Output Return Loss requirements for
                                                            active channels given in Table 44. (See item #4 following this table)

                                                            The total noise and spur requirement is the combination of noise
                                                            power from the 50 dBc suppressed channel and the normal noise
                                                            and spur requirement for the CMTS output when operating with all
                                                            channels unsuppressed.
     RF output port muting                                  ≥ 73 dB below the unmuted aggregate power of the RF modulated
                                                            signal, in every 6 MHz CEA channel from 54 MHz to 1218 MHz.

                                                            The specified limit applies with all active channels commanded to
                                                            the same transmit power level. Commanding a reduction in the
                                                            transmit level of any, or all but one, of the active channels does not
                                                            change the specified limit for measured muted power in 6 MHz.

                                                            When the CMTS is configured to mute an RF output port, the
                                                            CMTS is to control transmissions such that the output return loss of
                                                            the output port of the muted device complies with the Output
                                                            Return Loss requirements for inactive channels given in Table 44.
                                                            (See Item #5 following the table.)
 Table Notes:
 Note 1. "Channel" in mode 1 or mode 2 carrier suppression refers to an OFDM channel with at least 22 MHz of contiguous
         modulated spectrum or an SC-QAM channel.

The following is a list of CMTS Output Power Requirements based on Table 42 above.
1.     In a CMTS, the range of commanded power per channel, adjusted on a per-channel basis, MUST be 0 dBc to -2
       dBc relative to the highest commanded transmit power per channel, within an 8 dB absolute window below the
       highest commanded power.
2.     In a CMTS, the range of commanded power per channel, adjusted on a per-channel basis, MAY be according to
       the required power (in table below) to required power - 8 dB, independently on each channel.
3.     A CMTS supports the following 3 modes of diagnostic suppression:
       •     Mode 1: One channel suppressed - for this mode, the CMTS MUST support ≥ 50 dB carrier suppression
             within the occupied bandwidth in any one active channel, and when suppressing the carrier ≥ 50 dB within



140                                                    CableLabs                                                       04/19/23

<!-- PDF_PAGE: 141 -->
Physical Layer Specification                                                        CM-SP-PHYv3.1-I20-230419


         the occupied bandwidth in any one active channel, the CMTS MUST control transmissions such that no
         service impacting discontinuity or detriment to the unsuppressed channels occurs.
     •   Mode 2: All channels suppressed except one - for this mode, the CMTS MUST support 50 dB carrier
         suppression within the occupied bandwidth in every active channel except one. The suppression is not
         required to be glitchless, and the remaining unsuppressed active channel is allowed to operate with
         increased power such as the total power of the N' active channels combined.
     •   Mode 3: All Channels suppressed - for this mode, the CMTS MUST support 50 dB carrier suppression
         within the occupied bandwidth in every active channel.
4.   The CMTS MUST control transmissions such that in all three diagnostic carrier suppression modes the output
     return loss of the suppressed channel(s) complies with the Output Return Loss requirements for active channels
     given in Table 44 - CMTS OFDM Channel Characteristic.
5.   When the CMTS is configured to mute an RF output port, the CMTS MUST control transmissions such that the
     output return loss of the output port of the muted device complies with the Output Return Loss requirements for
     inactive channels given in Table 44 - CMTS OFDM Channel Characteristic.

7.5.9.1.2     Out-of-Band Noise and Spurious Requirements for CMTS
One of the goals of the DOCSIS DRFI specification is to provide the minimum intended analog channel CNR
protection of 60 dB for systems deploying up to 119 DRFI-compliant QAM channels. A new DOCSIS 3.1 PHY goal
is to provide protection for legacy DOCSIS channels and for high density constellations of OFDM channel blocks if
they are generated from another DRFI-compliant device.
The specification assumes that the transmitted power level of the digital channels will be 6 dB below the peak
envelope power of the visual signal of analog channels, which is the typical condition for 256-QAM transmission. It
is further assumed that the channel lineup will place analog channels at lower frequencies than digital channels, and
in systems deploying DOCSIS 3.1 modulators, analog channels will be placed at center frequencies below 600 MHz.
An adjustment of 10*log10 (6 MHz / 4 MHz) is used to account for the difference in a 6 MHz equivalent digital
channel, versus an analog channel's noise power bandwidth. With the assumptions above, for a 119-6 MHz
equivalent channel system, the specification in item 5 of Table 43 equates to an analog CNR protection of 60dB.
With all digital channels at the same equivalent power per 6 MHz channel, the specification provides for 58 dB SNR
protection for analog channels below 600 MHz (even with transmissions above 600 MHz) with 192 MHz occupied
bandwidth (one full OFDM channel) and 51 dB SNR protection for digital channels below 600 MHz with
transmission of 960 MHz modulated spectrum (160 equivalent legacy DOCSIS channels). The SNR protection
between 600 MHz and 1002 MHz is 55 dB for analog channels operating above a 192 MHz occupied bandwidth
generated by a DOCSIS 3.1 compliant device, and is 48 dB between 600 MHz and 1002 MHz for digital channels
operating above 960 MHz occupied bandwidth generated by a DOCSIS 3.1 compliant device.
Table 43 lists the out-of-band spurious requirements. In cases where the N' combined channels are not commanded
to the same power level, "dBc" denotes decibels relative to the strongest channel among the active channels. When
commanded to the same power level, "dBc" should be interpreted as the average channel power, averaged over the
active channels, to mitigate the variation in channel power across the active channels (see Table 42), which is
allowed with all channels commanded to the same power.
The CMTS modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - CMTS Output
Out-of-Band Noise and Spurious Emissions Requirements in measurements below 600 MHz and outside the
encompassed spectrum when the active channels are contiguous or when the ratio of modulated spectrum to gap
spectrum within the encompassed spectrum is 4:1 or greater.
The CMTS modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - CMTS Output
Out-of-Band Noise and Spurious Emissions Requirements, with 1 dB relaxation, in measurements within gaps in
modulated spectrum below 600 MHz and within the encompassed spectrum when the ratio of modulated spectrum
to gap spectrum within the encompassed spectrum is 4:1 or greater.
The CMTS modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - CMTS Output
Out-of-Band Noise and Spurious Emissions Requirements, with 3 dB relaxation, when the ratio of modulated
spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater, in measurements with 603 MHz ≤



04/19/23                                           CableLabs                                                   141

<!-- PDF_PAGE: 142 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications


center frequency ≤ 999 MHz, outside the encompassed spectrum or in gap channels within the encompassed
spectrum.
The CMTS modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - CMTS Output
Out-of-Band Noise and Spurious Emissions Requirements, with 5 dB relaxation, when the ratio of modulated
spectrum to gap spectrum within the encompassed spectrum is 4:1 or greater, in measurements with 999 MHz <
center frequency ≤ 1215 MHz, outside the encompassed spectrum or in gap channels within the encompassed
spectrum.
The CMTS modulator MUST satisfy the out-of-band spurious emissions requirements of Table 43 - CMTS Output
Out-of-Band Noise and Spurious Emissions Requirements, in addition to contributions from theoretical transmit
windowing, with permissible configurations of lower edge and upper edge sub-band exclusions of at least 1 MHz
each, FFT Size, cyclic prefix length (Ncp) and windowing roll-off period (Nrp) values. Recommendations for
configuration parameters are provided in Table 55 - CMTS Proposed Configuration Parameters. The test limit for
determining compliance to the spurious emissions requirements is the power sum of the spurious emissions
requirements taken in accordance with Table 43; and the contributions from the theoretical transmit windowing for
the configured transmissions.
When the Neq' combined active channels are not contiguous, and the ratio of modulated spectrum to gap spectrum
within the encompassed spectrum is 4:1 or greater, the spurious emissions requirements are determined by summing
the spurious emissions power allowed in a given measurement bandwidth by each of the contiguous subblocks among
the occupied bandwidth. In the gap channels within the encompassed spectrum and below 600 MHz there is a 1 dB
relaxation in the spurious emissions requirements, so that within the encompassed spectrum the spurious emissions
requirements (in absolute power) are 26% higher power in the measurement band determined by the summing of the
contiguous subblocks' spurious emissions requirements. In all channels above 600 MHz there is a 3 dB relaxation in
the spurious emissions requirements, so that the spurious emissions requirements (in absolute power) are double the
power in the measurement band determined by the summing of the contiguous subblocks' spurious emissions
requirements. The following three paragraphs provide the details of the spurious emissions requirements for non-
contiguous channel operation outside the encompassed spectrum; within the encompassed spectrum the same details
apply except there is an additional 1 dB allowance below 600 MHz; and 3 dB allowance is applied above 600 MHz
for all channels.
The full set of Neq' channels is referred to throughout this specification as the modulated channels or the active
channels. However, for purposes of determining the spurious emissions requirements for non-contiguous transmitted
channels, each separate contiguous subblock of channels within the active channels is identified, and the number of
channels in each contiguous subblock is denoted as Neqi, for i = 1 to K, where K is the number of contiguous
subblocks. Therefore,                 . Note that K=1 when and only when the entire set of active channels is
contiguous. Also note that an isolated transmit channel, i.e., a transmit channel with empty adjacent channels, is
described by Ni=1 and constitutes a subblock of one contiguous channel. Any number of the "contiguous subblocks"
may have such an isolated transmit channel; if each active channel was an isolated channel, then K=N'.
When Neq' ≥ Neq/4, Table 43 is used for determining the noise and spurious power requirements for each contiguous
subblock, even if the subblock contains fewer than Neq/4 active channels. When Neq' < Neq/4, Table 43 is used for
determining the noise and spurious power requirements for each contiguous subblock. Thus, the noise and spurious
power requirements for all contiguous subblocks of transmitted channels are determined from Table 43, where the
applicable table is determined by Neq' being greater than or equal to Neq/4, or not. The noise and spurious power
requirements for the ith contiguous subblock of transmitted channels is determined from Table 43 using the value Ni
for the "number of active channels combined per RF port", and using "dBc" relative to the highest commanded
power level of a 6 MHz equivalent channel among all the active channels, and not just the highest commanded
power level in the ith contiguous subblock, in cases where the Neq' combined channels are not commanded to the
same power. The noise and spurious emissions power in each measurement band, including harmonics, from all K
contiguous subblocks, is summed (absolute power, NOT in dB) to determine the composite noise floor for the non-
contiguous channel transmission condition.
For the measurement channels adjacent to a contiguous subblock of channels, the spurious emissions requirements
from the non-adjacent subblocks are divided on an equal "per Hz" basis for the narrow and wide adjacent
measurement bands. For a measurement channel wedged between two contiguous subblocks, adjacent to each, the
measurement channel is divided into three measurement bands, one wider in the middle and two narrower bands
each abutting one of the adjacent transmit channels. The wideband spurious and noise requirement is split into two


142                                                CableLabs                                             04/19/23

<!-- PDF_PAGE: 143 -->
Physical Layer Specification                                                                     CM-SP-PHYv3.1-I20-230419


parts, on an equal "per Hz" basis, to generate the allowed contribution of power to the middle band and to the
farthest narrowband. The ceiling function is applied to the resulting sum of noise and spurious emissions, per the
first Note in Table 43 to produce a requirement of ½ dB resolution.
Items 1 through 4 list the requirements in channels adjacent to the commanded channels.
Item 5 lists the requirements in all other channels further from the commanded channels. Some of these "other"
channels are allowed to be excluded from meeting the Item 5 specification. All the exclusions, such as 2nd and 3rd
harmonics of the commanded channel, are fully identified in the table.
Item 6 lists the requirements on the 2N' 2nd harmonic channels and the 3N' 3rd harmonic channels.
                Table 43 - CMTS Output Out-of-Band Noise and Spurious Emissions Requirements

                                                                                                  Adjusted Number of Active
                      for                                                                         Channels Combined per RF
                                                                                                            Port
                                               Band                                                       Requirement (in dBc)
  1    Adjacent channel up to 750 kHz from channel block edge                                     For N* = 1, 2, 3, 4: ≤ -58;

                                                                                                  For N* ≥ 5: ≤ 10*log10 [10-58/10
                                                                                                  +(0.75/6)*(10-65/10 + (N*-2)*10-
                                                                                                  73/10
                                                                                                       )]
  2    Adjacent channel (750 kHz from channel block edge to 6 MHz from channel block              For N* = 1: ≤ -62;
       edge)
                                                                                                  For N* ≥ 2: ≤ 10*log10 [10-62/10
                                                                                                  +(5.25/6)*(10-65/10 +(N*-2)*10-73/10)]
  3    Next-adjacent channel (6 MHz from channel block edge to 12 MHz from channel                ≤ 10*log10 [10-65/10 +( N*-1)*10-
       block edge)                                                                                   ]
                                                                                                  73/10


  4    Third-adjacent channel (12 MHz from channel block edge to 18 MHz from channel              For N* = 1: ≤ -73;
       block edge)                                                                                For N* = 2: ≤ -70;
                                                                                                  For N* = 3: ≤ -67;
                                                                                                  For N* = 4: ≤ -65;
                                                                                                  For N* = 5: ≤ -64.5;
                                                                                                  For N* = 6, 7: ≤ -64;
                                                                                                  For N'* ≥ 8: ≤-73 + 10*log10 (N*)
  5    Noise in other channels (47 MHz to 1218 MHz) Measured in each 6 MHz channel                For N* = 1: ≤ -73;
       excluding the following:                                                                   For N* = 2: ≤ -70;
       a) Desired channel(s)                                                                      For N* = 3: ≤ -68;
       b) 1st, 2nd, and 3rd adjacent channels (see Items 1, 2, 3, 4 in this table)                For N* = 4: ≤ -67;
       c) Channels coinciding with 2nd and 3rd harmonics (see Item 6 in this table)
                                                                                                  For N'* ≥ 5: ≤ -73 + 10*log10 (N*)
  6    In each of 2Neq' contiguous 6 MHz channels or in each of 3N' contiguous 6 MHz              ≤ -73 + 10*log10(N*) dBc, or -63,
       channels coinciding with 2nd harmonic and with 3rd harmonic components                     whichever is greater
       respectively (up to 1218 MHz)
  7    Lower out of band noise in the band of 5 MHz to 47 MHz                                     ≤ -50 + 10*log10(N*)
       Measured in 6 MHz channel bandwidth
  8    Higher out of band noise in the band of 1218 MHz to 3000 MHz                               For N* ≤ 8: ≤ -55 + 10*log10(N*)
       Measured in 6 MHz channel bandwidth
                                                                                                  For N* > 8: ≤ -60 + 10*log10(N*)
 Table Notes:
 Note 1. All equations are Ceiling(Power, 0.5) dBc. Use "Ceiling(2*Power) / 2" to get 0.5 steps from ceiling functions that return
         only integer values. For example Ceiling(-63.9, 0.5) = -63.5 dBc.
 Note 2. Add 3 dB relaxation to the values specified above for noise and spurious emissions requirements in all channels with 603
         MHz ≤ center frequency of the noise measurement ≤ 999 MHz. For example -73 dBc becomes -70 dBc.
         Add 5 dB relaxation to the values specified above for noise and spurious emissions requirements in all channels with 999
         MHz < center frequency of the noise measurement ≤ 1215 MHz. For example -73 dBc becomes -68 dBc.
 Note 3. Add 1 dB relaxation to the values specified above for noise and spurious emissions requirements in gap channels with
         center frequency below 600 MHz. For example -73 dBc becomes -72 dBc.




04/19/23                                                  CableLabs                                                                  143

<!-- PDF_PAGE: 144 -->
CM-SP-PHYv3.1-I20-230419                                             Data-Over-Cable Service Interface Specifications


7.5.10 Independence of Individual Channels Within Multiple Channels on a Single RF Port
The CMTS output OFDM channel characteristics are collected in Table 44.
                                         Table 44 - CMTS OFDM Channel Characteristic

                     Parameter                                                        Value
     Signal Type                                   OFDM
     Maximum Encompassed Spectrum                  190 MHz
     Minimum Modulated Spectrum                    22 MHz
     Subcarrier Spacing / FFT duration             25 kHz / 40 µs
                                                   50 kHz / 20 µs
     FFT Size                                      50 kHz: 4096 (4K FFT)
                                                   25 kHz: 8192 (8K FFT)
     Maximum Number of Subcarriers per FFT         4K: 3800
                                                   8K: 7600
     Maximum Number of Data Subcarriers per        4K: 3800 - number of pilot tones - 8 PLC subcarriers
     FFT                                           8K: 7600- number of pilot tones -16 PLC subcarriers
     Continuous Pilot Tones                        Continuous pilot placement is defined in Section 7.5.15.1.2. Minimum number
                                                   of continuous pilots is 16 and the maximum number is 128. Locations of 8
                                                   continuous pilots are pre-defined with reference to the PLC location. Locations
                                                   of remaining continuous pilots are defined using PLC messages.
     Scattered Pilot Tones                         4K FFT: One out of every 128 subcarriers, staggered by 1
                                                   8K FFT: One out of every 128 subcarriers, staggered by 2
     Cyclic Prefix Options                         Samples        µs
                                                   192            0.9375
                                                   256            1.25
                                                   512            2.5
                                                   768            3.75
                                                   1024           5.0
                                                   * sampling rate of 204.8 MHz
     OFDM Shaping                                  Raised cosine (Tukey) absorbed by CP
     Windowing Options                             Samples        µs
                                                   0              0
                                                   64             0.3125
                                                   128            0.625
                                                   192            0.9375
                                                   256            1.25
                                                   * sampling rate of 204.8 MHz

A potential use of a CMTS is to provide a universal platform that can be used for high-speed data services or for
video services. For this reason, it is essential that interleaver depth be set on a per channel basis to provide a suitable
transmission format for either video or data as needed in normal operation. Any N-channel block of a CMTS MUST
be configurable with at least two different interleaver depths, using any of the interleaver depths defined in Section
7.5.6.1. Although not as critical as per-channel interleaver depth control, there are strong benefits for the operator if
the CMTS is provided with the ability to set RF power, center frequency, and modulation type on a per-channel
basis.
1.     A multiple-channel CMTS MUST be configurable with at least two different legacy interleaver depths among
       the legacy channels on an RF output port in addition to each OFDM channel which is configurable
       independently.
2.     A multiple-channel CMTS MUST provide for 3 modes of carrier suppression of RF power for diagnostic and
       test purposes, see Table 42 - CMTS Output Power for mode descriptions and carrier RF power suppression
       level.
3.     A multiple-channel CMTS MAY provide for independent adjustment of RF power in a per channel basis for
       legacy channels with each RF carrier independently meeting the requirements defined in Table 42 - CMTS
       Output Power.




144                                                      CableLabs                                                     04/19/23

<!-- PDF_PAGE: 145 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


4.   A multiple-channel CMTS MAY provide for independent selection of center frequency on a per channel basis,
     thus providing for non-contiguous channel frequency assignment with each channel independently meeting the
     requirements in Table 42 - CMTS Output Power. A multiple-channel CMTS capable of generating nine or more
     channels on a single RF output port MUST provide for independent selection of center frequency with the ratio
     of number of active channels to gap channels in the encompassed spectrum being at least 2:1, and with each
     channel independently meeting the requirements in Table 42 - CMTS Output Power except for spurious
     emissions. A multiple-channel CMTS capable of generating nine or more channels on a single RF output port
     MUST meet the requirements of Table 42 - CMTS Output Power when the ratio of number of active channels
     to gap channels in the encompassed spectrum is at least 4:1. (A ratio of number of active channels to gap
     channels of at least 4:1 provides that at least 80% of the encompassed spectrum contains active channels, and
     the number of gap channels is at most 20% of the encompassed spectrum.)
5.   A multiple-channel CMTS MAY provide for independent selection of modulation order, either 64-QAM or
     256-QAM, on a per channel basis for legacy channels, with each channel independently meeting the
     requirements in Table 42 - CMTS Output Power.
6.   A CMTS MUST provide a test mode of operation, for out-of-service testing, configured for N channels but
     generating one-CW-per-channel, one channel at a time at the center frequency of the selected channel; all other
     combined channels are suppressed. One purpose for this test mode is to support one method for testing the
     phase noise requirements of Table 42 - CMTS Output Power. As such, the CMTS generation of the CW test
     tone SHOULD exercise the signal generation chain to the fullest extent practicable, in such manner as to exhibit
     phase noise characteristics typical of actual operational performance; for example, repeated selection of a
     constellation symbol with power close to the constellation RMS level would seemingly exercise much of the
     modulation and up-conversion chain in a realistic manner. The CMTS test mode MUST be capable of
     generating the CW tone over the full range of Center Frequency in Table 42 - CMTS Output Power.
In addition, the CMTS MUST be configurable in either one or both of the following conditions:
     •   Two CW carriers on a single out-of-service DS OFDM channel, at selectable valid subcarrier center
         frequencies 20 MHz to 100 MHz apart within the selected channel. All other subcarriers within the selected
         out-of-service DS OFDM channel are suppressed.
     •   One CW carrier on each of two separate but synchronized DS OFDM channels at selectable valid
         subcarrier center frequencies 20 MHz to 100 MHz apart within the selected channels. All other subcarriers
         within the selected out-of-service DS OFDM channel are suppressed.
The purpose of this test mode is to support the ability to measure the downstream Symbol Clock Jitter requirements
of Section 7.5.3.3, whereby the two CW carriers are mixed to create a difference product CW carrier at frequency
(F2-F1), for which the jitter is measure directly and compared to the requirements stated in that section.
7.   A CMTS MUST provide a test mode of operation, for out-of-service testing, generating one-CW-per-channel,
     at the center frequency of the selected channel, with all other N-1 of the combined channels active and
     containing valid data modulation at operational power levels. One purpose for this test mode is to support one
     method for testing the phase noise requirements of Table 42. As such, the generation of the CW test tone
     SHOULD exercise the signal generation chain to the fullest extent practicable, in such manner as to exhibit
     phase noise characteristics typical of actual operational performance. For example, a repeated selection of a
     constellation symbol, with power close to the constellation RMS level, would seemingly exercise much of the
     modulation and upconversion chain in a realistic manner. For this test mode, it is acceptable that all channels
     operate at the same average power, including each of the N−1 channels in valid operation, and the single
     channel with a CW tone at its center frequency. For example, a repeated selection of a constellation symbol,
     with power close to the constellation RMS level, would seemingly exercise much of the modulation and
     upconversion chain in a realistic manner. For this test mode, it is acceptable that all channels operate at the
     same average power, including each of the N−1 channels in valid operation, and the single channel with a CW
     tone at its center frequency. When operating in one-CW-per-channel test mode the CMTS MUST be capable of
     generating the CW tone over the full range of Center Frequency in Table 42 - CMTS Output Power.
8.   A CMTS MUST be capable of glitchless reconfiguration over a range of active channels from ceiling[7*Neq/8]
     to Neq. Channels which are undergoing configuration changes are referred to as the "changed channels." The
     channels which are active and are not being reconfigured are referred to as the "continuous channels."



04/19/23                                           CableLabs                                                    145

<!-- PDF_PAGE: 146 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


Glitchless reconfiguration consists of any of the following actions while introducing no discontinuity or detriment to
the continuous channels, where the modulator is operating in a valid DOCSIS 3.1-required mode both before and
after the reconfiguration with an active number of channels staying in the range {ceiling[7*Neq/8], Neq}: adding
and/or deleting one or more channels, and/or moving some channels to new RF carrier frequencies, and/or changing
the interleaver depth, modulation, power level, or frequency on one or more channels. Any change in the modulation
characteristics (power level, modulation density, interleaver parameters, center frequency) of a channel excuses that
channel from being required to operate in a glitchless manner. For example, changing the power per channel of a
given channel means that channel is not considered a continuous channel for the purposes of the glitchless
modulation requirements. Glitchless operation is not required when Neq is changed, even if no reconfigurations
accompany the change in Neq.

7.5.11 Cable Modem Receiver Input Requirements
The CM MUST be able to accept any range of OFDM subcarriers defined between Lower Frequency Boundary and
Upper Frequency Boundary simultaneously. Active subcarrier frequencies, loading, and other OFDM characteristics
are described by OFDM configuration settings and CM exclusion bands and profile definition. The OFDM signals
and CM interfaces will have the characteristics and limitations defined in Table 45.
The CM MUST support the requirements in Table 45 unless otherwise noted.
                                          Table 45 - Electrical Input to CM

                    Parameter                                                        Value
  Lower Band Edge                                258 MHz
                                                 should support 108 MHz - Note: applies if fumax is 85 MHz or less
                                                 (See Item #1 in the requirements list immediately following this table)
  Upper Band Edge                                1002 MHz
                                                 1218 MHz
                                                 1794 MHz
                                                 (See Item #2 in the list following this table)
  Frequency Boundary Assignment Granularity      25 kHz 8K FFT
                                                 50 kHz 4K FFT
  Signal Type                                    OFDM
  Single FFT Block Bandwidth                     192 MHz
  Minimum Contiguous-Modulated OFDM              22 MHz
  Bandwidth
  Number of FFT Blocks                           Support minimum of 2 FFT Blocks AND
                                                 32 SC-QAM Channels
  Subcarrier Spacing/FFT Duration                25 kHz / 40 µs
                                                 50 kHz / 20 µs
  Modulation Type                                QPSK, 16-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM, 1024-QAM,
                                                 2048-QAM, 4096-QAM
                                                 (See Item #3 in the list following this table)

                                                 may support:
                                                 8192-QAM, 16384-QAM
                                                 (See Item #4 in list following this table)
  Variable Bit Loading                           Support with subcarrier granularity
                                                 Support zero-bit-loaded subcarriers
  Total Input Power                              < 40 dBmV, 54 MHz to 1794 MHz
                                                 * Assuming negligible power outside this range
  Level Range (24 MHz min occupied BW)           -9 dBmV/24 MHz to 21 dBmV/24 MHz
  Equivalent PSD to SC-QAM of
  -15 dBmV to + 15 dBmV per 6 MHz




146                                                CableLabs                                                       04/19/23

<!-- PDF_PAGE: 147 -->
Physical Layer Specification                                                                  CM-SP-PHYv3.1-I20-230419


                       Parameter                                                       Value
     Maximum average power per MHz input to the      Let X = Average power of lowest power 24 MHz of modulated spectrum for
     CM from 54 MHz to 1218 MHz                      demodulation
     OR
     From 258 MHz to 1794 MHz                        Additional Demodulated Bandwidth, Bdemod:
     (dBmV/MHz)                                      ≤ Min [X - 10*log(24) + 10; 21 - 10*log(24)]

                                                     Additional Non-Demodulated Bandwidth, Bno-demod :
                                                     ≤ Min [X - 10*log(24) + 10; 26 - 10*log(24)]
                                                     For up to 12 MHz of occupied bandwidth (analog, OOB, QAM, OFDM)

                                                     ≤ Min [X - 10*log(24) + 10; 21 - 10*log(24)]
                                                     for all remaining bandwidth

                                                     NOTE: Level range does not imply anything about BER performance or
                                                     capability vs. QAM. CM BER performance is separately described.
     Input Impedance                                 75 ohms
     Input Return Loss                               > 6 dB (258 MHz - 1218 MHz)
                                                     > 6 dB (108 MHz - 1218 MHz)
                                                     Note: Applies when lower frequency boundary is 108 MHz
                                                     > 6 dB (258 MHz to 1794 MHz)
                                                     Note: Applies when upper frequency boundary is 1794 MHz
     Connector                                       F connector per [ISO/IEC-61169-24] or [SCTE 02]

The following is a list of CM Electrical Input requirements based on Table 45 above.
1a. The CM MUST support a Lower Band Edge of 258 MHz, and
1b. The CM SHOULD support a Lower Band Edge of 108 MHz if fumax is 85 MHz or less.
2.     The CM MUST support one or more of the following Upper Band Edges 1002 MHz, 1218 MHz, 1794 MHz.
3.     The CM MUST support Modulation Types QPSK, 16-QAM, 64-QAM, 128-QAM, 256-QAM, 512-QAM,
       1024-QAM, 2048-QAM, 4096-QAM.
4.     The CM MAY support Modulation Types 8192-QAM, 16384-QAM.

7.5.12 Cable Modem Receiver Capabilities
The required level for CM downstream post-FEC error ratio is defined as less than or equal to 10-6 PER (packet
error ratio) with 1500 byte Ethernet packets. This section describes the conditions at which the CM is required to
meet this error ratio.

7.5.12.1 CM Error Ratio Performance in AWGN Channel
Implementation loss of the CM MUST be such that the CM achieves the required error ratio when operating at a
CNR as shown in Table 46 - CM Minimum CNR Performance in AWGN Channel, under input load and channel
conditions as follows:
       •    Any valid transmit combination (frequency, subcarrier clock frequency, transmit window, cyclic prefix,
            pilot, PLC, subcarrier exclusions, interleaving depth, multiple modulation profile configuration, etc.) as
            defined in this spec.
       •    P6AVG (the measured channel power divided by number of occupied CEA channels) ≤ 15 dBmV.
       •    Up to fully loaded spectrum of 54 - 1218 MHz, including up to 48 analog channels placed lower in the
            spectrum than the digital channels.
       •    Power in (both above and below) 4 adjacent 6 MHz channels ≤ P6AVG+3 dB.
       •    Power in any 6 MHz channel over the spectrum ≤ P6AVG+6 dB.




04/19/23                                               CableLabs                                                         147

<!-- PDF_PAGE: 148 -->
CM-SP-PHYv3.1-I20-230419                                                   Data-Over-Cable Service Interface Specifications


      •      Peak envelope power in any analog channel over the spectrum P6AVG+6 dB.
      •      Average power per channel across spectrum P6AVG+3 dB.
      •      OFDM channel phase noise as in CMTS spec.
      •      No other artifacts (reflections, burst noise, tilt, etc.).
                                Table 46 - CM Minimum CNR Performance in AWGN Channel

               Constellation                 CNR1,2 (dB)                       CNR1,2 (dB)               Min P6AVG dBmV
                                            Up to 1002 MHz                1002 MHz to 1218 MHz
                     4096                          41.0                           41.5                            -6
                     2048                          37.0                           37.5                            -9
                     1024                          34.0                           34.0                           -12
                     512                           30.5                           30.5                           -12
                     256                           27.0                           27.0                           -15
                     128                           24.0                           24.0                           -15
                      64                           21.0                           21.0                           -15
                      16                           15.0                           15.0                           -15
          Table Notes:
          Note 1. CNR is defined here as total signal power in occupied bandwidth divided by total noise in occupied bandwidth.
          Note 2. Channel CNR is adjusted to the required level by measuring the source inband noise including phase noise
                  component along with transmitter noise and distortion and adding the required delta noise from an external
                  AWGN generator to achieve the desired CNR at the CM F-connector.
          Note 3. Applicable to an OFDM channel with 192 MHz of occupied bandwidth.

7.5.13 Physical Layer Link Channel (PLC)
This section contains the description of the Physical layer Link Channel (PLC) that the CMTS follows during the
construction of the PLC.
The aim of the PLC is for the CMTS to convey to the CM the physical properties of the OFDM channel. In a blind
acquisition, that is, in an acquisition without prior knowledge of the physical parameters of the channel, the CM first
acquires the PLC, and from this extract the parameters needed to acquire the complete OFDM channel.

7.5.13.1 PLC Placement
The CMTS MUST transmit a PLC for every downstream OFDM channel.
The CMTS MUST place the PLC at the center of a 6 MHz encompassed spectrum with no excluded subcarriers. For
4K FFT OFDM, this 6 MHz will contain 56 subcarriers followed by the 8 PLC subcarriers followed by another 56
subcarriers. For 8K FFT OFDM, this 6 MHz will contain 112 subcarriers followed by the 16 PLC subcarriers
followed by another 112 subcarriers.
The CMTS MUST place the 6 MHz encompassed spectrum containing the PLC on a 1 MHz grid, that is, the center
frequency of the lowest frequency subcarrier of the 6 MHz encompassed spectrum containing the PLC has to be an
integer when the frequency is measured in units of MHz.

7.5.13.2 PLC Structure
The CMTS MUST place the PLC so that it occupies the same set of contiguous subcarriers in every OFDM symbol.
The CMTS MUST place 8 OFDM subcarriers in the PLC of every OFDM symbol when using 4K FFT OFDM (i.e.,
a subcarrier spacing of 50 kHz).
The CMTS MUST place 16 OFDM subcarriers in the PLC of every OFDM symbol when using 8K FFT OFDM
(i.e., a subcarrier spacing of 25 kHz).




148                                                         CableLabs                                                     04/19/23

<!-- PDF_PAGE: 149 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419


                                           Table 47 - PLC components

                       DFT size           Subcarrier spacing                Number of
                                                                        PLC subcarriers (Np)
                         4096                    50 kHz                           8
                         8192                    25 kHz                           16

The CMTS MUST use a 16-QAM constellation for the PLC subcarriers.
The CMTS MUST construct the PLC as 8 symbols of preamble followed by 120 symbols of data subcarriers, as
shown in Figure 60 - Structure of the PLC.




                                         Figure 60 - Structure of the PLC
The CMTS MUST place the PLC at the center of a 6 MHz of active frequency range. For 4K FFT OFDM, this 6
MHz channel, in the increasing order of frequency, will consist of 56 subcarriers followed by the 8 PLC subcarriers
followed by another 56 subcarriers. For 8K FFT OFDM, this 6 MHz channel, in the increasing order of frequency,
will consist of 112 subcarriers followed by the 16 PLC subcarriers followed by another 112 subcarriers.
The CMTS MUST NOT insert any exclusion zones or excluded subcarriers within this 6 MHz band that contains the
PLC.
The CMTS MUST insert 8 continuous pilots in this 6 MHz bandwidth, 4 on each side of the PLC, as defined in the
section on downstream pilots.
The CMTS MUST interleave the PLC subcarriers on their own, as described in the section on "PLC Interleaving".
The CMTS MUST NOT interleave the PLC preamble.
The CMTS MUST synchronize the scattered pilot pattern to the PLC preamble as defined in Section 7.5.15 such that
in the OFDM symbol that follows the last symbol of the preamble sequence, the subcarrier next to the highest-
frequency subcarrier in the PLC is a scattered pilot.
The CMTS MUST NOT insert any scattered pilots or continuous pilots within the PLC frequency band.
The CMTS MUST synchronize the downstream data randomizer to the PLC preamble as described in Section
7.5.5.3. That is, the CMTS MUST initialize the downstream randomizer just before the lowest frequency data
subcarrier of the first OFDM symbol following the preamble.
The CMTS MUST synchronize the downstream PLC randomizer to the PLC preamble as described in
Section 7.5.13.8. That is, the CMTS must initialize the downstream PLC randomizer just before the lowest
frequency PLC subcarrier of the first OFDM symbol following the preamble.
The CMTS MUST place the 6 MHz bandwidth containing the PLC within the active bandwidth of the OFDM
channel.
Two possible locations for the PLC channel are illustrated in the example of Figure 61. In this example there are
three contiguous OFDM spectral bands in the 192 MHz channel, of width 22, 12 and 5 MHz. There are two
exclusion zones between these. The spectrum outside these bands is also excluded.



04/19/23                                           CableLabs                                                       149

<!-- PDF_PAGE: 150 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications


It is not necessary to place the PLC in the largest contiguous spectral segment of the OFDM channel. The 6 MHz
channel containing the PLC at its center may be anywhere provided it contains 6 MHz of spectrum without any
excluded subcarriers. In the example given the one possible location for the PLC channel is in the 12 MHz wide
segment.
Since the downstream channel will contain a minimum of 22 MHz of contiguous OFDM spectrum, there will always
be a spectral band to place the PLC. It may be noted that it not necessary to place the PLC at the center of the 22
MHz bandwidth.
The CMTS is expected to place the PLC in part of the spectrum that is less susceptible to noise and interference.




                                     Figure 61 - Examples of PLC Placement
The CMTS MUST generate the PLC as shown in Figure 62 - Physical Layer Operations for Forming the PLC
Subcarriers.




150                                                CableLabs                                              04/19/23

<!-- PDF_PAGE: 151 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419




                     Figure 62 - Physical Layer Operations for Forming the PLC Subcarriers

7.5.13.3 PLC Preamble Modulation
The CMTS MUST modulate the subcarriers in the PLC preamble using binary phase-shift keying (BPSK), as
described in this section.
For 4K FFT, the preamble size is 8 subcarriers. Thus, an array of size (8 x 8) is defined as follows:
                                           Table 48 - PLC Preamble for 4K FFT

                   Symbol      Symbol         Symbol        Symbol    Symbol       Symbol       Symbol   Symbol
                     1           2              3             4         5            6            7        8
  Subcarrier 1         0           0             1            0          1            1            0       1
  Subcarrier 2         1           0             0            0          1            1            1       0
  Subcarrier 3         0           1             1            1          1            0            0       1
  Subcarrier 4         0           1             0            0          0            1            1       0
  Subcarrier 5         1           1             1            0          1            1            1       1
  Subcarrier 6         1           0             0            0          0            0            0       1
  Subcarrier 7         0           1             0            1          0            0            1       1
  Subcarrier 8         0           0             1            0          0            0            1       1

                                           Table 49 - PLC preamble for 8K FFT

                    Symbol       Symbol         Symbol       Symbol    Symbol      Symbol       Symbol   Symbol
                      1            2              3            4         5           6            7        8
   Subcarrier 1            1           0             0            1       0            1           0       0
   Subcarrier 2            0           1             1            0       0            1           0       0
   Subcarrier 3            0           1             1            1       0            0           0       1
   Subcarrier 4            0           0             0            1       0            1           1       1
   Subcarrier 5            1           1             0            0       1            0           1       0
   Subcarrier 6            0           0             0            1       1            0           0       1
   Subcarrier 7            0           1             1            1       0            1           1       0



04/19/23                                                 CableLabs                                            151

<!-- PDF_PAGE: 152 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


                      Symbol       Symbol        Symbol     Symbol      Symbol     Symbol    Symbol      Symbol
                        1            2             3          4           5          6         7           8
   Subcarrier 8           1           1             1         0            0          0         1             0
   Subcarrier 9           0           1             1         1            1          0         0             1
  Subcarrier 10           1           1             1         1            0          1         1             1
  Subcarrier 11           1           1             1         0            0          0         0             0
  Subcarrier 12           1           1             0         1            0          1         0             1
  Subcarrier 13           1           1             0         0            1          1         0             0
  Subcarrier 14           1           0             1         1            1          0         1             0
  Subcarrier 15           0           1             0         1            1          0         0             0
  Subcarrier 16           0           0             1         0            0          0         0             1

The CMTS MUST map each of the above binary bits to a BPSK constellation point in the complex plane using the
following transformation:
          0 → (1 + j0)
          1 → (-1 + j0)

7.5.13.4 PHY Parameters Carried by the PLC
The PLC carries two sets of PHY parameters from the CMTS to cable modems: the Downstream Profile Descriptor
and the OFDM Channel Descriptor. Contents of each of these descriptors are described in [DOCSIS MULPIv3.1].
This section contains only a brief description of the physical layer parameters carried by the PLC. For formatting
and other details, reference is made to the MULPI specification.
The inverse discrete Fourier transform that defines the OFDM signal at the CMTS is given by the following
equation:

                                                                                                                  (1)

The sampling rate in the previous equation is 204.8 Msamples/s and the value of N is either 4096 or 8192. The
CMTS MUST specify this value of N via the PLC.
The CMTS MUST define, via the PLC, the frequency of the subcarrier X(0) in equation (1) as a 32-bit positive
integer in units of Hz.
The PLC subcarriers constitute a set of contiguous subcarriers given by:
                                              {X(k), k = L, L+1, …, L+Np−1}                                       (2)
The CMTS MUST define the value of L to define the location of the PLC within an OFDM channel.
The CMTS MUST define the locations of the continuous pilots, excluding the eight predefined ones, via the PLC.
The CMTS MUST define the locations of excluded subcarriers via the PLC.
The CMTS MUST define the bit loading profile for all 4096 or 8192 subcarriers of equation (1), excluding
continuous pilots and excluded subcarriers, via the PLC.
The CMTS MUST use the indices k of equation (1) to specify the locations of subcarriers in all of the above
definitions.
In addition to above, the CMTS MUST define the following physical parameters of the OFDM channel:
      •   Cyclic prefix length (five possible settings)
      •   Roll-off (five possible settings)
      •   Time interleaver depth (any integer from 1 to 32)
      •   Modulation of the NCP (QPSK, 16-QAM or 64-QAM)


152                                                     CableLabs                                         04/19/23

<!-- PDF_PAGE: 153 -->
Physical Layer Specification                                                               CM-SP-PHYv3.1-I20-230419


7.5.13.5 Mapping of Bytes to a Bitstream
The CMTS MUST convert the stream of bytes received by the PLC into a stream of bits, MSB first, as illustrated in
Figure 63 - Mapping Bytes into a Bitstream for FEC Encoding.




                           Figure 63 - Mapping Bytes into a Bitstream for FEC Encoding

7.5.13.6 Forward Error Correction Code for the PLC
The CMTS MUST encode the PLC data using (384,288) puncturing LDPC encoder, see Section 7.4.15.1.4 for the
definition of puncturing encoder.
The puncturing encoder uses the same mother encoder for fine ranging FEC (Section 7.4.15.2.3), that is the rate 3/5
(480,288) LDPC encoder listed by Table 20.
Denote the information bits sent to the mother code encoder by (a0, ..., a287) and let the encoder output being (a0, ...,
a287, b288, ..., b479), where b288, ..., b479 are parity-check bits. The coordinates to be deleted by the puncturing step are:
    •    Period 1: 48 consecutive coordinates a48, ..., a95
    •    Period 2: 48 consecutive coordinates b384, ..., b431
NOTE: Also see Figure 81.
The puncturing is described in Figure 64.




04/19/23                                               CableLabs                                                         153

<!-- PDF_PAGE: 154 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications




                                Figure 64 - Puncturing Encoder for the PLC FEC

7.5.13.7 Block Interleaving of the PLC Subcarriers
The preceding section shows 288 data bits entering the LDPC encoder and 384 encoded bits exiting the LDPC
encoder. This sequence is in effect is time-reversed order. The time-ordered sequence takes the form shown in the
figure below.
The CMTS MUST map these 384 encoded bits into 96 4-bit nibbles {ui, i = 0, 1, ... , 95} as described in Figure 65 -
Mapping Encoded Bitstream into a Stream of Nibbles before interleaving.




                       Figure 65 - Mapping Encoded Bitstream into a Stream of Nibbles
The CMTS MUST interleave this 96-nibble sequence {u0 u1 u2...u95} as described below.
For 4K FFT, the CMTS MUST use an (8x12) array.
The CMTS MUST write the values ui along the rows of this two-dimensional array, as shown in Figure 66 - Block
Interleaving of PLC Subcarriers for 4K FFT.




154                                               CableLabs                                              04/19/23

<!-- PDF_PAGE: 155 -->
Physical Layer Specification                                                    CM-SP-PHYv3.1-I20-230419




                          Figure 66 - Block Interleaving of PLC Subcarriers for 4K FFT
The CMTS MUST then read this two-dimensional array along vertical columns to form the two-dimensional
sequence {vt,f t = 0, 1,...,11 and f = 0, 1,...7}. This operation is mathematically represented as:
        vt,f = ut + 12f
The CMTS MUST map each of the 8-point sequence given below to the 8 successive PLC subcarriers of an OFDM
symbol after randomization described in the next section.
        Vt = {vt,f, f = 0, 1, ..., 7} for 12 successive OFDM symbols t = 0, 1, ..., 11
Therefore, each FEC codeword will occupy the PLC segment of twelve successive 4K FFT OFDM symbols. There
will be ten such codewords in a 128-symbol PLC frame, including the 8-symbol preamble.
The CMTS MUST map ten complete FEC codewords into one 4K FFT PLC frame.
For 8K FFT, the CMTS MUST use a (16 x 6) array. The CMTS MUST write the values ui along the rows of this
two-dimensional array, as shown in Figure 67 - Block Interleaving of PLC Subcarriers for 8K FFT.




04/19/23                                         CableLabs                                                155

<!-- PDF_PAGE: 156 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications




                          Figure 67 - Block Interleaving of PLC Subcarriers for 8K FFT
The CMTS MUST then read this two-dimensional array along vertical columns to form the two-dimensional
sequence {v_(t,f) ,t=0,1,...,5 and f=0,1,...,15}.This operation is mathematically represented as:
        vt,f = ut + 6f
The CMTS MUST map each of the 16-point sequence given below to the 16 successive PLC subcarriers of an
OFDM symbol after randomization described in the next section.
        Vt = {vt,f, f = 0, 1, ... , 15} for 6 successive OFDM symbols t =0, 1, ... , 5
Therefore, each FEC codeword will occupy the PLC segment of six successive 8K FFT OFDM symbols. There will
be twenty such codewords in a 128-symbol PLC frame, including the 8-symbol preamble.
The CMTS MUST map twenty complete FEC codewords into one 8K FFT PLC frame.


156                                                  CableLabs                                         04/19/23

<!-- PDF_PAGE: 157 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


7.5.13.8 Randomizing the PLC Subcarriers
The CMTS MUST randomize QAM symbols forming the data section of the PLC frame using a copy of the linear
feedback shift register in GF[212] used for randomizing the data subcarriers. This is shown in Figure 68.




                        Figure 68 - Linear Feedback Shift Register for PLC Randomization
The LFSR is defined by the following polynomial in GF[212].
         x2 + x + α11
The GF[212] is defined through polynomial algebra modulo the polynomial:
         α12 + α6 + α4 + α + 1
This LFSR is initialized to the hexadecimal numbers given below:
         D0 = "4A7"
         D1 = "B4C"
The CMTS MUST initialize the LFSR with the above two 12-bit numbers at the beginning of the first OFDM
symbol following the PLC preamble. The CMTS MUST clock the LFSR once after randomizing one PLC
subcarrier. The CMTS MUST randomize each subcarrier through an exclusive-OR operation of the 4 bits
representing the subcarrier (vt,f) with the four LSBs of register D0.
The first subcarrier to be randomized is the lowest frequency subcarrier of the PLC in the OFDM symbol
immediately after the preamble. This will be randomized using the four LSBs of the initialized D0, namely 0x7. The
LFSR will be clocked once after randomizing each PLC subcarrier of the OFDM symbol. After randomizing the
highest frequency PLC subcarrier of an OFDM symbol the CMTS MUST clock the LFSR before randomizing the
lowest frequency PLC subcarrier in the next OFDM symbol.
The CMTS MUST use the bit ordering given below to perform randomization. The four LSBs of D0 are defined as
the coefficients of {α3α2α1α0} of the Galois field polynomial representing D0. The LSB is defined as the coefficient
of α0 of the polynomial representing D0. The ordering of the four bits representing the subcarrier is defined with
reference to Figure 65. Assume that the FEC block shown in Figure 65 is the first FEC block in the PLC frame.
Then, since the location of the first nibble does not change as a result of interleaving:
         v0,0 = {a0 a1 a2 a3}
Then the randomization operation (i.e., exclusive-OR with 0x7) is given by:
         {y0, y1, y2, y3} = {a0 + 1, a1 + 1, a2 + 1, a3 + 0}
The addition operations in the above equation are defined in GF[2], that is, these are bit-wise exclusive-OR
operations. The LFSR is clocked once before randomizing the next nibble v0,1.
The CMTS MUST NOT randomize the PLC preamble.




04/19/23                                           CableLabs                                                    157

<!-- PDF_PAGE: 158 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications


7.5.13.9 Mapping to 16-QAM Subcarriers
The CMTS MUST map each randomized nibble {y0 y1 y2 y3} into a complex number using the 16-QAM
constellation mapping shown in Annex A.
The CMTS MUST multiply the real and imaginary parts by 1/√10 to ensure that mean-square value of the QAM
constellation is unity.

7.5.13.10 PLC Timestamp Reference Point
The PLC subcarriers following the preamble contain a timestamp. The timestamp is further described in [DOCSIS
MULPIv3.1].
The CMTS MUST define this timestamp with reference to the first OFDM symbol following the preamble if such a
timestamp exists. This OFDM symbol is indicated by an arrow in Figure 69.




            Figure 69 - Time - Frequency Plane Representation of PLC Timestamp Synchronization
Time domain version of the OFDM symbol is shown in Figure 69. The inverse discrete Fourier transform of the
symbol of Figure 69 results in the set of 4096 or 8192 samples occupying the FFT duration shown. After this the
CMTS will introduce a configurable cyclic prefix (CP), window the symbol and overlap successive symbols in the
time domain.
The CMTS MUST use the time of the first sample of the FFT duration as the timestamp.
To clarify this further, individual time domain samples are also shown in Figure 70.
(This is for illustration only; actual samples are complex-valued.) The sample rate is 204.8 Msamples/s. The dotted
arrow points to the first sample of the FFT symbol duration.




158                                                CableLabs                                              04/19/23

<!-- PDF_PAGE: 159 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419




                  Figure 70 - Time Domain Representation of PLC Timestamp Synchronization

7.5.14 Next Codeword Pointer

7.5.14.1 Mapping of Bytes to Bits
Each NCP consists of three bytes as defined in Section 8.3.4. The first byte (Byte 0) contains the profile identifier as
the four MSBs and four control bits as the four LSBs. The other two bytes (Byte 1 and Byte 2) contain the start
pointer.
The CMTS MUST map the three NCP bytes into 24-bit serial bitstream {a_23 a_22 ... a_0} for the purpose of
LDPC encoding, as shown in Figure 71 - Mapping NCP Bytes into a Bitstream for FEC Encoding. Note that the
LDPC encoder is also defined using the same bit pattern {a23 a22 ... a0}.




                       Figure 71 - Mapping NCP Bytes into a Bitstream for FEC Encoding
Figure 72 depicts the NCP bytes to input stream bits mapping after FEC encoding, including the FEC parity bits.
FEC parity bits are specified in Section 7.5.14.2.




04/19/23                                            CableLabs                                                      159

<!-- PDF_PAGE: 160 -->
CM-SP-PHYv3.1-I20-230419                                                Data-Over-Cable Service Interface Specifications




                              Figure 72 - Mapping FEC Encoded NCP Bytes into a Bitstream

7.5.14.1.1     CRC-24-D
The last NCP in the NCP field of each symbol contains a CRC-24-D message which is calculated across all NCPs in
the NCP field of the symbol as specified in 24-bit Cyclic Redundancy Check (CRC) Code (Normative).
         +           +                +                  +         +          +                +         +
                0         1      2         3    4    5         6          7         8     9        10        11
               >         >      >         >    >    >         >          >         >     >         >         >



                     +          +              +               +          +        +               +              +   din
               12        13          14   15        16   17        18         19        20    21        22   23
               >         >           >    >         >    >         >          >         >     >         >    >

             x^24 + x^22 + x^20 + x^19 + x^18 + x^16 + x^14 + x^13 + x^11 + x^10 + x^8 + x^7 + x^6 + x^3 + x + 1

                                Figure 73 - Polynomial Sequence for CRC-24-D Encoding


NCP data is fed into the CRC-24-D encoder in the same order as the FEC encoder, as depicted in Figure 74:




                                Figure 74 - Mapping NCP Data into the CRC-24-D Encoder




160                                                      CableLabs                                                     04/19/23

<!-- PDF_PAGE: 161 -->
Physical Layer Specification                                                           CM-SP-PHYv3.1-I20-230419


The 24-bit CRC output is represented as:




Figure 75 describes the mapping of the CRC-NCP bytes to input bitstream including the FEC parity bits.




                       Figure 75 - Mapping FEC Encoded CRC-NCP Bytes into a Bitstream

7.5.14.2 Forward Error Correction Code for the NCP
The CMTS MUST encode the 24 information bits of a Next Codeword Pointer using (48, 24) shortening and
puncturing LDPC encoder, see Section 7.4.15.1.4 for the definition of shortening and puncturing encoder.
The shortening puncturing encoder uses the same mother encoder for initial ranging FEC (Section 7.4.3.3), that is
the rate 1/2 (160, 80) LDPC encoder listed by Table 19.
Denote the information bits sent to the mother code encoder by (a0, ..., a79) and let the encoder output being (a0, ...,
a79, b80, ..., b159), where b80, ..., b159 are parity-check bits. Then the shortening and puncturing steps can be
described as follows; also see Figure 76:
The shortening step fills 0 to 56 consecutive coordinate starting at position 24, i.e., let a24 = a25 = ... = a79 = 0.
The rest 24 bits i.e., a0, ..., a23, are NCP information data.
The coordinates to be deleted by the puncturing step are:
    •    Period 1: 24 consecutive coordinates b80, ..., b103
    •    Period 2: 16 consecutive coordinates b112, ..., b127
    •    Period 3: 16 consecutive coordinates b144, ..., b159




                        Figure 76 - Shortening and Puncturing Encoder for the NCP FEC



04/19/23                                            CableLabs                                                      161

<!-- PDF_PAGE: 162 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


7.5.14.3 Mapping LDPC Encoded Bits into OFDM Subcarriers
The LDPC encoder outputs a stream of 48 bits:
        {b143 b142 ... b128 b111 b110 ... b104 a23 a22 ... a0}
The NCP QAM constellation can be a member of the set {QPSK, QAM-16, QAM-64}.
For QAM-64 the CMTS MUST map the LDPC encoded bits into eight 6-bit QAM constellation points as defined
below:
        {y0,0 y0,1 y0,2 y0,3 y0,4 y0,5} = {a5 a4 a3 a2 a1 a0}
        {y1,0 y1,1 y1,2 y1,3 y1,4 y1,5} = {a11 a10 a9 a8 a7 a6}



        {y7,0 y7,1 y7,2 y7,3 y7,4 y7,5} = {b143 b142 b141 b140 b139 b138}
The mapping of these 6-bit integers to points in the complex plane is given by the figure below. Hexadecimal
notation has been used to represent the 6-bit numbers {yi,0 yi,1 yi,2 yi,3 yi,4 yi,5}.
The CMTS MUST multiply the real and imaginary parts by 1/√42 to ensure that mean-square value of the QAM
constellation is unity.




                Figure 77 - 64-QAM Constellation Mapping of {y(i,0)y(i,1)y(i,2)y(i,3)y(i,4)y(i,5)}...
For QAM-16 the CMTS MUST map the LDPC encoded bits into twelve 4-bit QAM constellation points as defined
below:
        {y0,0 y0,1 y0,2 y0,3} = {a3 a2 a1 a0}
        {y1,0 y1,1 y1,2 y1,3} = {a7 a6 a5 a4}



        {y11,0 y11,1 y11,2 y11,3} = {b143 b142 b141 b140}
The mapping of these 4-bit integers to points in the complex plane is given by the figure below. Hexadecimal
notation has been used to represent the 4-bit numbers {yi,0 yi,1 yi,2 yi,3}.
The CMTS MUST multiply the real and imaginary parts by 1/√10 to ensure that mean-square value of the QAM
constellation is unity.



162                                                 CableLabs                                           04/19/23

<!-- PDF_PAGE: 163 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419




                       Figure 78 - 16 QAM Constellation Mapping of {y(i,0)y(i,1)y(i,2)y(i,3)}
For QPSK the CMTS MUST map the LDPC encoded bits into twenty four 2-bit QAM constellation points as
defined below:
         {y0,0 y0,1} = {a1 a0}
         {y1,0 y1,1} = {a3 a2}



         {y23,0 y23,1} = {b143 b142}
The mapping of these 2-bit integers to points in the complex plane is given by the figure below. Hexadecimal
notation has been used to represent the 2-bit numbers {yi,0 yi,1}.
The CMTS MUST multiply the real and imaginary parts by 1/√2 to ensure that mean-square value of the QAM
constellation is unity.




                             Figure 79 - QPSK Constellation Mapping of {y(i,0)y(i,1)}

7.5.14.4 Placement of NCP Subcarriers
The CMTS MUST place the NCP subcarriers beginning from the frequency location of the highest frequency active
data subcarrier of the OFDM symbol, and going downwards along active data subcarriers of the OFDM symbols
before they are time and frequency interleaved.
Therefore the first subcarrier of the first NCP occupies the frequency location of the highest frequency active data
subcarrier of the OFDM symbol. The term active data subcarrier is used to indicate a subcarrier that is neither




04/19/23                                            CableLabs                                                    163

<!-- PDF_PAGE: 164 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


excluded and that is neither a continuous pilot nor a scattered pilot. This highest frequency active subcarrier may not
occur at the same frequency in every symbol owing to the presence of scattered pilots.
The OFDM symbol, prior to time and frequency interleaving at the CMTS, will have subcarriers assigned to be
scattered pilot placeholders. Furthermore, the NCP profile may indicate subcarriers that are to be zero-bit-loaded.
The CMTS MUST skip both of these types of subcarriers during the placement of NCP subcarriers.

7.5.14.5 Randomization and Interleaving of NCP Subcarriers
The CMTS MUST randomize the NCP constellation points {yi,j} described in Section 7.5.14.3 using the algorithm
applied to the data subcarriers, described in Section 7.5.5.3.
The values of D0 and D1 are the ones that would be used to randomize data subcarriers at the same subcarrier index.
The CMTS MUST time and frequency interleave the NCP subcarriers using the algorithm applied to data
subcarriers and this is described in the interleaving section.

7.5.15 Downstream Pilot Patterns
Downstream pilots are subcarriers modulated by the CMTS with a defined modulation pattern that is known to all
the CMs in the system to allow interoperability.
There are two types of pilots: continuous and scattered. Continuous pilots occur at fixed frequencies in every
symbol. Scattered pilots occur at different frequency locations in different symbols. Each of these pilot types for
DOCSIS 3.1 is defined in the following sections.

7.5.15.1 Scattered Pilots
The main purpose of scattered pilots is the estimation of the channel frequency response for the purpose of
equalization. There are two scattered pilot patterns, one for 4K FFT and one for 8K FFT. Although these pilots occur at
different frequency locations in different OFDM symbols, the patterns repeat after every 128 OFDM symbols; in
other words, the scattered pilot pattern has a periodicity of 128 OFDM symbols along the time dimension.

7.5.15.1.1      Scattered Pilot Pattern for 4K FFT
The CMTS MUST create scattered pilots for 4K FFTs in the manner described in this section.
Figure 80 shows the 4K FFT scattered pilot pattern for OFDM transmissions.
The scattered pilot pattern is synchronized to the PLC as shown in Figure 80. The first OFDM symbol after the PLC
preamble has a scattered pilot in the subcarrier just after the highest frequency subcarrier of the PLC. Two such
scattered pilots that are synchronized to the PLC preamble are marked as red circles in Figure 83.
The remainder of the scattered pilot pattern is linked to the scattered pilot synchronized to the PLC preamble, using
the following rules:
1.    In each symbol scattered pilots are placed every 128 subcarriers.
2.    From symbol to symbol, scattered pilots are shifted by one subcarrier position in the increasing direction of the
      frequency axis. This will result in scattered pilots placed in the exclusion band and in the PLC band.
3.    Scattered pilots are zero-valued in the exclusion bands.
4.    Scattered pilots are zero-valued when these coincide with excluded subcarriers.
5.    In the PLC, normal PLC signals (i.e., PLC data or the PLC preamble) are transmitted instead of scattered pilots.
      The CMTS MUST NOT transmit scattered pilots in the PLC band.




164                                                  CableLabs                                               04/19/23

<!-- PDF_PAGE: 165 -->
Physical Layer Specification                                                              CM-SP-PHYv3.1-I20-230419




                                    Figure 80 - 4K FFT Downstream Pilot Pattern
There are 8 preamble symbols in the PLC; for 4K FFT, there are 8 PLC subcarriers in each symbol.
Mathematically, the scattered pilot pattern for a 4K FFT is defined as follows. Let a subcarrier (depicted in red in the
above figure just after the PLC preamble) be referred to as x(m, n), where:
         m is the frequency index
         n is the time index (i.e., the OFDM symbol number)
The scattered pilots in the 128 symbols following (and including symbol n) are given by:
         Symbol n:                  x(n, m±128i), for all non-negative integers i
         Symbol (n+1):              x(n+1, m±128i + 1), for all non-negative integers i
         Symbol (n+2):              x(n+2, m±128i + 2), for all non-negative integers i



         Symbol (n+127):            x(n+127, m±128i + 2), for all non-negative integers i


04/19/23                                              CableLabs                                                   165

<!-- PDF_PAGE: 166 -->
CM-SP-PHYv3.1-I20-230419                                         Data-Over-Cable Service Interface Specifications


Each of the above locations is a scattered pilot, provided that it does not fall on a continuous pilot, on the PLC, on an
exclusion zone or on a excluded subcarrier. If the scattered pilot coincides with a continuous pilot, it is treated as a
continuous pilot and not as a scattered pilot.
This pattern repeats every 128 symbols. That is, symbol (128+n) has the same scattered pilot pattern as symbol n.

7.5.15.1.2    Scattered Pilot Pattern for 8K FFT
The CMTS MUST create scattered pilots for 8K FFTs in the manner described in this section.
Figure 81 shows a scattered pilot pattern that may be used for OFDM transmissions employing 8K FFT. This is used
here for explanation purposes only and to help with the derivation of the scattered pilot pattern actually used in 8K
FFT OFDM transmissions depicted in Figure 82.




         Figure 81 - A Downstream Scattered Pilot Pattern for 8K FFT (for Explanation Purposes Only)
The scattered pilot pattern is synchronized to the PLC as shown in Figure 80. The first OFDM symbol after the PLC
preamble has a scattered pilot in the subcarrier just after the highest frequency subcarrier of the PLC. Two such
scattered pilots that are synchronized to the PLC preamble are marked as red circles in Figure 81.
In the case of an 8K FFT, pilots are stepped by two subcarriers from one OFDM symbol to the next. Since the pilot
spacing along the frequency axis is 128, this results in a pilot periodicity of 64 in the time dimension. When Figure 80


166                                                  CableLabs                                                04/19/23

<!-- PDF_PAGE: 167 -->
Physical Layer Specification                                                               CM-SP-PHYv3.1-I20-230419


and Figure 81 are compared, it is clear that the periodicity is half for the 8K scattered pilot pattern. However,
because an 8K symbol is twice as long as a 4K symbol, the scattered pilot periodicity in terms of actual time is
approximately the same for both the 4K and 8K FFTs. This allows channel estimates for 8K FFTs to be obtained in
approximately the same amount of time as for the 4K FFT. However, scattered pilots for 8K FFTs do not cover all
subcarrier locations and hence intermediate channel estimates have to be obtained through interpolation.
Noise can also be estimated using scattered pilots, and again, the noise at subcarrier locations not covered by
scattered pilots in the 8K FFT can be obtained through interpolation. Note that this interpolation operation could fail
in the presence of narrowband ingress; interpolation could also be problematic when there are excluded subcarriers.
To overcome these interpolation problems, the entire 8K scattered pilot location can be shifted by one subcarrier
location after 64 subcarriers, as illustrated in Figure 82. This may be treated as the interlacing of two identical scattered
pilot patterns. The set of purple scattered pilots are shifted one subcarrier space in relation to the set of green
scattered pilots. As a result, the scattered pilots cover all subcarrier locations; noise at every subcarrier location can
be estimated without interpolation. Note that periodicity of the 8K FFT scattered pilot pattern is now 128, not 64.




                               Figure 82 - 8K FFT Downstream Scattered Pilot Pattern




04/19/23                                               CableLabs                                                        167

<!-- PDF_PAGE: 168 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


Mathematically, the scattered pilot pattern for an 8K FFT is define d as follows. Let the subcarrier (depicted in red
in Figure 82 just after the PLC preamble) be referred to as x(m, n) where:
          m is the frequency index
          n is the time index (i.e., the OFDM symbol number)
The scattered pilots in the first 64 symbols following and including symbol n are given by:
          Symbol n:                  x(n, m±128i), for all non-negative integers i
          Symbol (n+1):              x(n+1, m±128i + 2), for all non-negative integers i
          Symbol (n+2):              x(n+2, m±128i + 4), for all non-negative integers i



          Symbol (n+63):             x(n+63, m±128i + 126), for all non-negative integers i
The scattered pilot sequence of the next 64 symbols is the same as above, but with a single subcarrier shift in the
frequency dimension.
          Symbol (n+64):             x(n+64, m±128i + 1), for all non-negative integers i
          Symbol (n+65):             x(n+65, m±128i + 3), for all non-negative integers i
          Symbol (n+66):             x(n+66, m±128i + 5), for all non-negative integers i



          Symbol (n+127):            x(n+127, m±128i + 127), for all non-negative integers i



Each of the above locations is a scattered pilot, provided that it does not fall on a continuous pilot, on the PLC, on an
exclusion band or on an excluded subcarrier. If the scattered pilot coincides with a continuous pilot it is treated as a
continuous pilot and not as a scattered pilot.
This pattern repeats every 128 symbols. That is, symbol (128+n) has the same scattered pilot pattern as symbol n.

7.5.15.2 Continuous Pilots
Continuous pilots occur at the same frequency location in all symbols and are used for receiver synchronization.
Placement of continuous pilots is determined in two ways:
a.    Predefined continuous pilot placement around the PLC
b.    Continuous pilot placement defined via PLC messages
Note that continuous and scattered pilots can overlap; the amount of overlap, in terms of number of carriers, changes
from symbol to symbol. Overlapping pilots are treated as continuous pilots.

7.5.15.2.1     Predefined Continuous Pilots around the PLC
As discussed in Section 7.5.13.1, the PLC is placed at the center of a 6 MHz spectral region. Four pairs of
predefined continuous pilots are placed symmetrically around the PLC as shown in Figure 83. The spacing between
each pilot pair and the PLC are different to prevent all pilots from being impacted at the same time by echo or
interference.




168                                                    CableLabs                                              04/19/23

<!-- PDF_PAGE: 169 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419




                    Figure 83 - Placement of Predefined Continuous Pilots around the PLC
The locations of the continuous pilots are defined with reference to the edges of the PLC band. Hence, once the PLC
has been detected, these continuous pilots also become known to the receiver.
Table 50 provides the values of d1, d2, d3, and d4, measured in number of subcarriers from the PLC edge. That is, dx
is absolute value of the difference between the index of the continuous pilot and the index of the PLC subcarrier at
the PLC edge nearest to the continuous pilot. The index of a subcarrier is the integer k of the IDFT definition given
in Section 7.5.2.8. For example, let the lowest frequency subcarrier of the PLC have the IDFT index k equal to 972.
Then according to Table 50 for the 4K FFT mode the continuous pilot nearest to this lowest frequency PLC subcarrier
will have the IDFT index k of (972 − 15) = 957. The index k of the highest frequency PLC subcarrier of this OFDM
channel is 979. Hence continuous pilot that is nearest upper frequency edge of the PLC has an index k of 994.
The table provides the number of subcarriers from the edge of the PLC to the placement of the pilot for the two FFT
sizes. For each distance (dx) defined in Table 50 - Subcarrier Distances for Placement of Predefined Pilots, the
CMTS MUST place two pilots: one dx subcarriers above and one dx subcarriers below the edge of the PLC band.
                       Table 50 - Subcarrier Distances for Placement of Predefined Pilots

                                                               d1          d2          d3         d4
                 4K FFT            PLC 8 subcarriers           15          24          35         47
                 8K FFT           PLC 16 subcarriers           30          48          70         94


7.5.15.2.2    Continuous Pilot Placement Defined by PLC Message
The CMTS MUST define a set of continuous pilots distributed as uniformly as possible over the entire OFDM
spectrum in addition to the predefined continuous pilots described in the preceding section.
The CMTS MUST ensure that there are no isolated active OFDM spectral regions that are not covered by
continuous pilots.



04/19/23                                           CableLabs                                                    169

<!-- PDF_PAGE: 170 -->
CM-SP-PHYv3.1-I20-230419                                                                         Data-Over-Cable Service Interface Specifications


It is not practical to predefine the locations of this set of continuous pilots because of exclusion bands and excluded
subcarriers.
The CMTS MUST provide the continuous pilot placement definition via the PLC in accordance with messaging
formats contained in the MULPI specification.
The CMTS MUST adhere to the rules given below for the definition of this set of continuous pilot locations
conveyed to the CM via PLC messaging. It is noted that these rules do not apply to the eight predefined pilots.
The CMTS MUST place the continuous pilots generated using these rules in every OFDM symbol, in addition to the
eight predefined continuous pilots.
The CMTS MUST obtain the value of NCP using the following formula:
                                                                                                    𝐹𝐹     −𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚
                                  𝑁𝑁𝐶𝐶𝐶𝐶 = min �max �8, 𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐 �𝑀𝑀 ∗ � 𝑚𝑚𝑚𝑚𝑚𝑚                                        ��� , 120�                        (1)
                                                                                                         190𝑒𝑒6

In this equation Fmax refers to frequency in Hz of the highest frequency active subcarrier and Fmin refers to frequency
in Hz of the lowest frequency active subcarrier of the OFDM channel. It is observed that the number of continuous
pilots is linearly proportional to the frequency range of the OFDM channel. It may also be observed that the
minimum number of continuous pilots defined using the PLC cannot be less than 8, and the maximum number of
continuous pilots defined using the PLC cannot exceed 120. Therefore, the total number of continuous pilots,
including the predefined ones, will be in the range 16 to 128, both inclusive.
The value of M in equation (1) is kept as a parameter that can be adjusted by the CMTS. Nevertheless, the CMTS
MUST ensure that M is in the range given by the following equation:
                                                                        120 ≥ M ≥ 48                                                                     (2)
The typical value proposed for M is 48.
The CMTS MUST use the algorithm given below for defining the frequencies for the location of these continuous pilots.
Step 1:
Merge all the subcarriers between Fmin and Fmax eliminating the following:
      •   Exclusion bands
      •   6 MHz band containing the PLC
      •   Known regions of interference, e.g., LTE
      •   Known poor subcarrier locations, e.g., CTB/CSO
Let the merged frequency band be defined as the frequency range [0, Fmerged_max].
Step 2:
Define a set of NCP frequencies using the following equation:
                                         𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚_𝑚𝑚𝑚𝑚𝑚𝑚       𝑖𝑖∗𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚_𝑚𝑚𝑚𝑚𝑚𝑚
                                𝐹𝐹𝑖𝑖 =                           +                              ; for 𝑖𝑖 = 0, 1, … , 𝑁𝑁𝐶𝐶𝐶𝐶 − 1                          (3)
                                               2𝑁𝑁𝐶𝐶𝐶𝐶                       𝑁𝑁𝐶𝐶𝐶𝐶

This yields a set of uniformly spaced NCP frequencies:
                             𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚_𝑚𝑚𝑚𝑚𝑚𝑚       3𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚_𝑚𝑚𝑚𝑚𝑚𝑚                                         𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚_𝑚𝑚𝑚𝑚𝑚𝑚
                            {                        ,                            , … , 𝐹𝐹𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚𝑚_𝑚𝑚𝑚𝑚𝑚𝑚 −                                 }        (4)
                                   2𝑁𝑁𝐶𝐶𝐶𝐶                       2𝑁𝑁𝐶𝐶𝐶𝐶                                                      2𝑁𝑁𝐶𝐶𝐶𝐶

Step 3:
Map the set of frequencies given above to the nearest subcarrier locations in the merged spectrum. This will give a
set of NCP approximately uniformly spaced subcarriers in the merged domain.
Step 4:
De-merge the merged spectrum through the inverse of the operations through which the merged spectrum was
obtained in step 1.



170                                                                        CableLabs                                                               04/19/23

<!-- PDF_PAGE: 171 -->
Physical Layer Specification                                                            CM-SP-PHYv3.1-I20-230419


Step 5:
If any continuous pilot is within 1 MHz of a spectral edge, move this inwards (but avoiding subcarrier locations
impacted by interferences like CSO/CTB) so that every continuous pilot is at least 1 MHz away from a spectral
edge. This is to prevent continuous pilots from being impacted by external interferences. If the width of the spectral
region does not allow the continuous pilot to be moved 1 MHz from the edge then the continuous pilot has to be
placed at the center of the spectral band.
Step 6:
Identify any spectral regions containing active subcarriers (separated from other parts of the spectrum by exclusion
bands on each side) that do not have any continuous pilots. Introduce an additional continuous pilot at the center of
every such isolated active spectral region.
In the unlikely event that the inclusion of these extra pilots results in the total number of continuous pilots defined
by PLC exceeding 120, return to step 1 and re-do the calculations after decrementing the value of NCP by one.
Step 7:
Test for periodicity in the continuous pilot pattern and disturb periodicity, if any, through the perturbation of
continuous pilot locations using a suitable algorithm. A simple procedure would be to introduce a random
perturbation of up to ±5 subcarrier locations around each continuous pilot location, but avoiding subcarrier locations
impacted by interferences like CSO/CTB.
The CMTS MUST transmit this continuous pilot pattern to the CMs in the system using the PLC.

7.5.15.3 Pilot Modulation
For both continuous and scattered pilots, the CMTS MUST modulate these subcarriers as described in the following
section.
Continuous and scattered pilots are BPSK modulated using a pseudo-random sequence. This pseudo-random sequence
is generated using a 13-bit linear feedback shift register, shown in Figure 84 with polynomial (x^13+x^12+x^11+x^8+1).
This linear feedback shift register is initialized to all ones at the k=0 index of the 4K or 8K discrete Fourier
transform defining the OFDM signal (refer to Section 7.5.7). It is then clocked after every subcarrier of the FFT. If
the subcarrier is a pilot (scattered or continuous), then the BPSK modulation for that subcarrier is taken from the
linear feedback shift register output.




     Figure 84 - 13-Bit Linear Feedback Shift Register for the Pilot Modulation Pseudo-Random Sequence
For example, let the output of the linear feedback shift register be wk. The BPSK modulation used for the pilot
would be:
                                     wk = 0: BPSK Constellation Point = 1 + j0
                                    wk = 1: BPSK Constellation Point = -1 + j0
To illustrate this by example, assume that there is a continuous pilot at subcarrier index k=1000. The output of the
linear feedback shift register for this index is 0 and hence the continuous pilot will correspond to the BPSK
constellation point (1 + j0). Now assume that there is scattered pilot at subcarrier index k=2999. The output of the
linear feedback shift register for this index is 1 and hence the scattered pilot will correspond to the BPSK
constellation point (-1 + j0).

7.5.15.4 Pilot Boosting
The CMTS MUST multiply the real and imaginary components of continuous and scattered pilots by a real-valued
number such that the amplitude of the continuous and scattered pilots is twice the root-mean-square value of the
amplitude of other subcarriers of the OFDM symbol. That is, continuous and scattered pilots are boosted by
approximately 6 dB with reference to other subcarriers.


04/19/23                                             CableLabs                                                      171
