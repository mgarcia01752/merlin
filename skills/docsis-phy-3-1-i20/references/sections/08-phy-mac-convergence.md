# 8 - PHY-MAC Convergence

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 172-184
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 172 -->
CM-SP-PHYv3.1-I20-230419                                        Data-Over-Cable Service Interface Specifications


8 PHY-MAC CONVERGENCE
8.1     Scope
This specification defines the electrical characteristics and signal processing operations for a cable modem (CM) and
Cable Modem Termination System (CMTS). It is the intent of this specification to define an interoperable CM and
CMTS such that any implementation of a CM can work with any CMTS. It is not the intent of this specification to
imply any specific implementation.
This section describes CM and CMTS requirements for the convergence logical layer between the MAC and PHY
layers for OFDM downstream channels and OFDMA upstream channels. The primary roles of the convergence
layer are to map DOCSIS MAC frames into codewords and to map codewords into minislots for transmission from
the cable modem to the CMTS. Contents of the Next Codeword Pointer (NCP) message and PHY Link Channel
(PLC) are also defined in this section.

8.2     Upstream
8.2.1     Profiles
Upstream profiles are comprised of multiple minislots, and are characterized by bit loading and pilot pattern. Bit
loading and pilot patterns can vary between minislots within the profile. The bit loading and pilot pattern assignment
of minislots can also vary between profiles. An upstream profile maps to an Interval Usage Code defined in an
Upstream Channel Descriptor Message.
Different FEC codeword sizes may use portions of a single minislot. The use case for this is as follows: With a 17
KB grant, there needs to be a long codeword to cover the first 16200 bits and a 1 KB codeword to cover the rest of
the bits. The first long codeword can land in the middle of a minislot. In this situation, it does not make sense to
require a constant codeword size per profile, as the profile needs to cover a group of minislots.
FEC codewords can cross minislot and frame boundaries.

8.2.2     Upstream Subcarrier Numbering Conventions
Subcarriers are numbered from lower frequency to higher frequency within a FFT block. All subcarriers within the
102.4 MHz bandwidth are numbered, including the outside excluded subcarriers. Numbering starts at 0 and goes to
2047 for 2K FFT, and 0 to 4095 for 4K FFT.
Data codewords are mapped into minislots - prior to time and frequency interleaving - as described in Section 8.2.3.10.

8.2.3     Minislots
Minislots are defined by a size in terms of the number of symbols and number of subcarriers. They include data
carried on data subcarriers, pilots carried on pilot subcarriers and complementary pilots that can carry data but at a
lower modulation order.
In this section, Bandwidth (BW) is defined as the encompassed spectrum on a single OFDMA channel.

8.2.3.1    Minislot Parameters
The CMTS MUST define minislot parameters according to Table 51 - Minislot Parameters Table. The CMTS
communicates minislot definition to the CM in UCD messages as defined in [DOCSIS MULPIv3.1].
The CM MUST use the minislot structure defined by the UCD messages received from the CMTS.
The CMTS MUST be capable of receiving minislots structured according to Table 51 - Minislot Parameters Table.
The CMTS MUST apply any subcarrier exclusions to the entire channel independent of upstream profile
assignment.




172                                                 CableLabs                                                04/19/23

<!-- PDF_PAGE: 173 -->
Physical Layer Specification                                                                    CM-SP-PHYv3.1-I20-230419


                                            Table 51 - Minislot Parameters Table

                Parameter                          Minimum                  Maximum                    Recommended or
                                                    Value                    Value                       Typical Value
  K                                            6                   For BW ≥ 72 MHz
  Number of symbol periods per frame                               18 for 20 μs FFT duration         N/A

                                                                   9 for 40 μs FFT duration

                                                                   For 48 ≤ BW < 72 MHz
                                                                   24 for 20 μs FFT duration
                                                                   12 for 40 μs FFT duration

                                                                   For BW < 48 MHz
                                                                   36 for 20 μs FFT duration

                                                                   18 for 40 μs FFT duration
  Q                                            8 for 20 μs FFT duration 16 for 40 μs FFT duration
  Number of subcarriers per minislot
  Data bitloading                              QPSK                CMTS Mandatory:                   Plant-dependent
                                                                    1024-QAM

                                                                   CMTS Optional:
                                                                    2048-QAM
                                                                    4096-QAM

                                                                   CM Mandatory:
                                                                    4096-QAM
  Complementary-pilot bitloading               BPSK                256-QAM
  Modulation order for primary pilots          BPSK                BPSK
 Table Notes:
 Note 1. Data bitloading is constant within a minislot, excepting pilots and complementary pilots.
 Note 2. The bitloading of complementary pilots within a minislot is constant.

8.2.3.2     Minislot Structure
This specification defines several minislot structures with pilot patterns as described in Section 7.4.16.

8.2.3.2.1       Number of OFDMA Symbols per Frame
The CMTS MUST follow the rules listed below for the range of the frame size in number of symbols (K):
K is configurable between 6 (minimal value) and one of the following values:
    •     With 20 µs FFT duration (2K FFT)
          Kmax = 18 for BW≥ 72 MHz
          Kmax = 24 for 48 MHz ≤BW < 72 MHz
          Kmax = 36 for BW < 48 MHz
    •     With 40 µs FFT duration (4K FFT)
          Kmax = 9 for BW ≥ 72 MHz
          Kmax = 12 for 48 MHz≤ BW < 72 MHz
          Kmax = 18 for BW < 48 MHz




04/19/23                                                  CableLabs                                                     173

<!-- PDF_PAGE: 174 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications


8.2.3.2.2      Number of subcarriers per minislot
The CMTS signals the number of subcarriers per minislot to the CM in the UCD.
The CMTS MUST use 16-subcarrier minislots when the subcarrier spacing is 25 kHz.
The CMTS MUST use 8-subcarrier minislots when the subcarrier spacing is 50 kHz.

8.2.3.3     Modulation of Data Subcarriers
With the exception of pilots and complementary pilots, bit loading is constant within a minislot.
The CMTS MUST use the same modulation order for all data subcarriers in a minislot.

8.2.3.4     Location of Pilots
A set of pilot patterns is defined from which the CMTS or operator can select to match the frequency response of the
network. Pilot patterns are described in Section 7.4.16.

8.2.3.5     Modulation Order of Pilots
The CMTS MUST use BPSK modulation for pilots.

8.2.3.6     Location of Complementary Pilots
The CMTS MUST place complementary pilots as defined by the chosen minislot pattern.

8.2.3.7     Modulation Order of Complementary Pilots
The CMTS MUST use a modulation order equal to (data modulation order - 4) for complementary pilots.
The CMTS MUST use a minimum modulation order of BPSK for complementary pilots.

8.2.3.8     Pilot Overhead
Pilot overhead is dependent on the chosen minislot pattern. Capacity and pilot overhead vary with the length of the
minislot (number of symbols) and with the number of subcarriers (8 or 16). Minimum capacity and largest pilot
overhead occur with the shortest minislot length (8 symbols).

8.2.3.9     Mapping Minislots to Subcarriers
The CMTS MUST construct minislots using only contiguous subcarriers. There are no subcarrier exclusions or
unused subcarriers within a minislot.

8.2.3.10 Ordering of Data Bits within a Minislot
With the exception of initial ranging transmissions, the CM MUST fill minislots as follows: prior to interleaving,
data would be filled across all symbol periods, subcarrier by subcarrier, transmitted symbol period by symbol
period, with complementary pilots filled inline. The fill order is illustrated in Figure 85.
NOTE: The position of the pilots shown in Figure 85 is for illustrative purposes only and is not intended to be
      prescriptive.




174                                                 CableLabs                                               04/19/23

<!-- PDF_PAGE: 175 -->
Physical Layer Specification                                                            CM-SP-PHYv3.1-I20-230419




                                        Figure 85 - Minislot Data Bit Ordering

8.2.3.11 Modulation Order Variability
Different minislots are allowed to have different pilot patterns: pilot patterns are assigned at minislot granularity.
Different minislots are allowed to have different bit loading: bit loading is assigned at minislot granularity. This
allows bit loading to vary across the spectrum.

8.2.4     Subslots
This section specifies the subslot within the upstream data frame.

8.2.4.1    Subslot Structure
The minislot can be subdivided along time into multiple subslots to provide multiple transmission opportunities for
BW requests. The subslots fit within minislot boundary and can have leftover symbols in the end along time axis.
Leftover symbols are not a part of any subslot and are unused (zero valued subcarriers) when the minislot is granted
to IUCs 1 or 2. Gaps between subslots within a single minislot are not permitted.




                                             Figure 86 - Subslot Structure
The subslot length is fixed at 2 symbols with minislots that employ 16 subcarriers, and 4 symbols with minislots that
employ 8 subcarriers.

8.2.4.2    Data Mapping among Subslots
The data mapping within a subslot is as follows: the mapping starts from the lowest subcarrier's index, and lowest
symbol index, and first along symbol time index, and then goes up on subcarrier's index. The pilots are skipped
during data mapping.
Data mapping to subcarriers is implemented without time or frequency interleaving.
Figure 87 and Figure 88 illustrate the mapping process. Note that the positions of the pilots shown in Figure 87 and
Figure 88 are for illustrative purposes only and are not intended to be prescriptive.



04/19/23                                             CableLabs                                                        175

<!-- PDF_PAGE: 176 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications




                                    Figure 87 - Data Mapping for a 4x8 Subslot




                                   Figure 88 - Data Mapping for a 2x16 Subslot

8.3     Downstream
8.3.1    Operation
An example implementation of the downstream convergence layer for DOCSIS 3.1 and its association with the
stages before and after it is shown in Figure 89. This block diagram is intended to demonstrate functionality; while it
represents one style of implementation, there are no requirements that an implementation needs to adhere directly to
this example.




176                                                 CableLabs                                               04/19/23

<!-- PDF_PAGE: 177 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419




                           Figure 89 - Downstream Convergence Layer Block Diagram
The operation of the downstream can be split between the forwarding plane and the control plane. The forwarding
plane contains the data packets that are destined to the user. The control plane carries MAC management messages
and other types of control messages.
The forwarding engine in the CMTS forwards packets to a DOCSIS MAC Domain. The MAC Domain performs
QoS functions such as hierarchical QoS, per-user rate shaping, aggregate per-channel rate shaping, and aggregate
per-bonding-group rate shaping. The MAC engine can also hold packets in a buffer as part of the DOCSIS Light
Sleep (DLS) mode.
A profile is a list of modulations that are used for the subcarriers within an OFDM channel. The downstream can use
different profiles for different groups of CMs. Generally, a group of CMs that have similar SNR performance will be
grouped into the same profile. When packets are encoded into FEC codewords and transmitted into the OFDM
spectrum, a path in the downstream for that packet is created so the PHY layer uses a profile to create a path at the
MAC layer.
There can be multiple paths from the CMTS to the same CM. Each path has a different profile. Profiles are typically
given a letter designation such as Profile A. Profile A is the boot profile that CMs first begin receiving when they
initialize and register. Either the forwarding engine or the DOCSIS QoS engine keeps a lookup table of exactly to
what path and what profile each packet needs to be assigned. This profile assignment is used to pick a convergence
layer buffer.
There is one convergence layer buffer per profile. These are shallow buffers that hold only a few packets so as to not
build up any significant latency. The output of these buffers is fed to the codeword builder. The codeword builder is
responsible for mapping DOCSIS frames into codewords. It is also responsible for balancing out the traffic flow
between all the profiles so that the latency budgets are observed.
The codeword builder uses the same profile for an entire codeword. It can change profiles at each codeword
boundary. The convergence layer buffers do not have to be serviced in any particular order. The DOCSIS MAC
layer has already rate-shaped the packet flow to the size of the OFDM channel, so all packets will fit. It is up to the
codeword builder to schedule the packets into the FEC codewords as it deems appropriate. Although rate shaping or
packet drops are not intended to be performed at the convergence layer, some queues could be treated as low latency
while other queues could be treated as high latency.




04/19/23                                            CableLabs                                                     177

<!-- PDF_PAGE: 178 -->
CM-SP-PHYv3.1-I20-230419                                             Data-Over-Cable Service Interface Specifications


Since the codeword builder is multiplexing at the codeword level, the packets in the convergence layer buffers are
naturally split across codeword boundaries and multiplexed together. The convergence layer buffers are packets in –
bytes out. The codeword builder combines bytes from one buffer, adds FEC, and then using the profile modulation
vector, it maps the codeword onto one or more OFDM symbols (or partial symbols).

8.3.2     MAC Frame to Codewords
The downstream LDPC codeword shown in Figure 90 is referred to as (16200, 14400). This means that a full
codeword is 2025 bytes (16200 bits) that are divided into 225 bytes (1800 bits) of parity and 1800 bytes (14400 bits)
of LDPC payload. That payload is further divided into 21 bytes (168 bits) of BCH parity, a 2 byte fixed header, and
a variable 1777 byte maximum payload for DOCSIS frames. When the FEC codeword is shortened, only the
DOCSIS payload shrinks. All other fields remain the same size.




                                 Figure 90 - DOCSIS Frame to Codeword Mapping
Although the codeword is shown in Figure 90 as a collection of bytes, this has to be treated as a bitstream for FEC
encoding, and for subsequent mapping to OFDM subcarriers. Therefore, the CMTS MUST map the stream of bytes
into a stream of bits in the MSB-first order. For example, the MSB of the first byte of the codeword is to be mapped
to the leftmost bit of the codeword in Figure 90.
DOCSIS frames are sequentially mapped to codewords that are associated with a common profile. The codewords
do not need to be adjacent, although their order is guaranteed. If there is no DOCSIS frame available, the CMTS
MUST adopt one of the following two options until the next DOCSIS frame becomes available.
1.    Insert zero-bit-loaded filler subcarriers into OFDM symbols as described in Section 7.5.5.5.2.
2.    Insert stuffing pattern of 0xFF bytes into codewords.
The codeword header is defined in Table 52.
                                         Table 52 - Data Codeword Definition

          Name         Length                                                 Value
      Valid           1 bit         0 = PDU Pointer is not valid (ignore)
                                    1 = PDU pointer is valid
      Reserved        4 bits        Set to 0. Ignore on receive.
      PDU Pointer     11 bits       This points to the first byte of the first DOCSIS frame that starts in the payload. A value
                                    of zero points to the byte immediately following the codeword header.

When the codeword gets mapped across subcarriers within a symbol, there may be residual bits left over on the last
subcarrier within that symbol. Since the number of residual bits may be more or less than 8, the receiver cannot
simply round down to a byte boundary. To permit the downstream receiver to discard these residual bits properly,
the CMTS MUST make the codeword payload an odd number of bytes.
One potential set of algorithms for the CMTS codeword builder and the CM codeword receiver is as follows.
On the CMTS, the algorithm is:
      •   Number of total bytes is (header + payload + parity) and never exceeds 2025 bytes.
      •   IF (total bytes = odd), send to FEC engine.


178                                                    CableLabs                                                         04/19/23

<!-- PDF_PAGE: 179 -->
Physical Layer Specification                                                          CM-SP-PHYv3.1-I20-230419


    •   IF (total bytes = even), add a0xFF stuff byte to the payload if legal or change the number of bytes.
    •   CMTS Symbol mapper adds trailing bits (all 1s) to map codeword to a symbol boundary.
On the CM, the corresponding algorithm is:
    •   CM extracts total bits between two NCP pointers.
    •   IF total bits > 16200, use initial 16200 bits, and a full codeword is declared.
    •   IF total bits = 16200, and a full codeword is declared.
    •   IF total bits < 16200, round down to the nearest odd number of bytes.
    •   Discard [(total bits + 8) Modulo 16] bits.

8.3.3   Subcarrier Numbering Conventions
Subcarriers are numbered from lower frequency to higher frequency within a FFT block. All subcarriers within the
204.8 MHz bandwidth are numbered, including the outside excluded subcarriers. Numbering starts at 0 and goes to
4095 for 4K FFT, and 0 to 8191 for 8K FFT.
Data codewords are mapped to subcarriers–prior to time and frequency interleaving–from a lower number to a
higher number.

8.3.4   Next Codeword Pointer
When the data codewords are mapped to subcarriers within a symbol, a pointer is needed to identify where a data
codeword starts. This is known as the Next Codeword Pointer (NCP). The collection of NCP message blocks within
a symbol is known as the NCP field. There are a variable number of NCP message blocks (MBs) on each OFDM
symbol. To make sure that all subcarriers are used without reserving empty NCP MBs, the mapping of the NCP
occurs in the opposite direction of the mapping for data. The relationship of NCP message blocks to the data channel
is shown in Figure 91 (scattered pilots are not shown; last NCP MB is a CRC).




                                  Figure 91 - Data and NCP Prior to Interleaving
The CMTS MUST map data subcarriers within a symbol, starting from a lower subcarrier number and proceeding to
a higher subcarrier number.
The CMTS MUST map NCP message blocks starting at a higher subcarrier number and moving to a lower
subcarrier number.



04/19/23                                             CableLabs                                                179

<!-- PDF_PAGE: 180 -->
CM-SP-PHYv3.1-I20-230419                                          Data-Over-Cable Service Interface Specifications


8.3.4.1       NCP Data Message Block
The format of the NCP data message block is illustrated in Figure 92 and defined in Table 53.




                                       Figure 92 - NCP Data Message Block
Note that each three byte NCP MB is mapped into a unique FEC codeword that has a 3 byte payload with 3 bytes of
FEC. The last FEC codeword is then followed by a 3 byte CRC-24-D (refer to 24-bit Cyclic Redundancy Check
(CRC) Code) that is also placed in its own FEC block.
                                             Table 53 - NCP Parameters

          Field    Size                                             Description
      Profile ID   4 bits   Profile ID for the data channel
                            0 = Profile A
                            1 = Profile B
                            ...
                            15 = Profile P
      Z            1 bit    Zero bit-loading
                            0 = subcarriers follow profile
                            1 = subcarriers are all zero-bit-loaded
      C            1 bit    Data Profile Update
                            0 = use even profile
                            1 = use odd profile
                            This bit is equal to the LSB of the Configuration Change Count in the DPD message for the
                            profile listed in the Profile ID field.
      N            1 bit    NCP Profile Select
                            0 = use even profile
                            1 = use odd profile
                            This bit is equal to the LSB of the NCP profile change count.
      L            1 bit    Last NCP Block
                            0 = This NCP is followed by another NCP.
                            1 = This is the last NCP in the chain and is followed by an NCP CRC message block.
      T            1 bit    Codeword Tagging
                            0 = This codeword is not included in the codeword counts reported by the CM in the OPT-RSP
                            message.
                            1 = This codeword is included in the codeword counts reported by the CM in the OPT-RSP
                            message.

                            This bit is applicable only when Codeword Tagging is enabled in the CM for the CM's transition
                            profile. When Codeword Tagging is not enabled for the CM's transition profile, all codewords
                            are counted.
                            If the CM is not conducting OFDM Downstream Profile Usability Testing on the Profile ID of this
                            NCP, or if the CM is conducting testing but Codeword Tagging is disabled for the test, then the
                            Codeword Tagging bit is ignored and all codewords are counted.
                            Codeword Tagging is enabled or disabled in the CM for the CM's transition profile by the CMTS
                            through a parameter passed in the OPT-REQ message. Codeword Tagging enable/disable also
                            applies per profile.
                            Codeword Tagging can only be enabled for a CM's transition profile.
                            See [DOCSIS MULPIv3.1] for a more details about OFDM Downstream Profile Testing (OPT)
                            and Codeword Tagging.




180                                                  CableLabs                                                     04/19/23

<!-- PDF_PAGE: 181 -->
Physical Layer Specification                                                                    CM-SP-PHYv3.1-I20-230419


        Field      Size                                                Description
    U             1 bit      NCP Profile Update indicator
                             Indicates a change in the NCP bit-loading profile.
                             The CMTS sets this bit in each of the 128 symbols immediately preceding an NCP bit-loading
                             profile change. The 128 sequential "U" bits form a specific bit pattern as defined below to
                             indicate the NCP profile change.
                             The CMTS sets NCP Profile Update indicator to value 0 in all other symbols.
    R             1 bit      Reserved
    Subcarrier    13 bits    This is the number assigned to the first subcarrier used by the codeword. A value of zero points
    pointer                  to the first subcarrier of the symbol before interleaving, excluding locations reserved for
                             scattered pilots. For example, if the first subcarrier before interleaving is a location reserved for
                             a scattered pilot and the second subcarrier of the symbol is not, then the subcarrier pointer
                             value of zero points to the second subcarrier. The value of 0x1FFF is reserved as a null pointer.
                             The maximum value is 0x1FFE = 8190. The value 0x1FFF is reserved as a null pointer.

The NCP structure is predicated upon the following facts:
    •     FEC codewords are mapped continuously across successive symbols.
    •     The PHY can determine the first subcarrier of the first NCP message block.
    •     The PHY can determine the first subcarrier of the data field in the current symbol.
Based upon these facts and combined with the information in the NCP fields, then
    •     The PHY can determine the last subcarrier of the last NCP message block.
    •     The next subcarrier after the last NCP message block CRC is last subcarrier of the data field.
The main task of the NCP message block is to provide a reference to the appropriate profile and a start pointer for
codewords. The length of a codeword is determined by the difference between the subcarrier pointer in two
successive NCP message blocks.
Data subcarriers may contain FEC codewords or unused subcarriers. These functions are referred to as fields in the
NCP header.
The CMTS MUST include one NCP within the same symbol for each start of codeword or a group of unused
subcarriers that exists in that symbol. The CMTS MUST include a valid Profile ID when the field is a FEC
codeword field and no zero-bit-loading (Z bit not asserted).
The CMTS MUST assert the Zero load bit ("Z") to mark a set of subcarriers which are not used as described in
Section 8.3.4.3.
The CM MUST ignore the Profile ID when the Z bit is asserted.
The CM MUST use the Data Profile Update bit ("C") to select the odd or even data profile.
The CMTS MUST set the value of Data Profile Update bit ("C") to indicate whether the odd or even data profile is
in use in the current OFDM symbol.
The CM MUST use the NCP Profile Select bit ("N") to select the odd or even NCP profile in the current OFDM
symbol.
The CMTS MUST set the NCP Profile Select bit to the same value in all the NCP Message Blocks in any one
symbol.
The CMTS MUST use the NCP Profile Update bit ("U") to indicate a change in the NCP bit-loading profile.
The CMTS MUST set the NCP Profile Update bit to the same value in all NCP Message Blocks in any one symbol.
The CMTS MUST set the U-bits of the 128 symbols immediately preceding a NCP bit-loading profile change to the
pattern Hex "BCB240898BAD833539ED0ABE946E3F85", as illustrated in Figure 93 - NCP Profile Update Bit
Setting Immediately Preceding an NCP Bit Loading Profile Change.
The CMTS MUST set the NCP Profile Update Bit to zero in all other symbols, i.e., all symbols except the 128
symbols immediately preceding a NCP bit-loading profile change.



04/19/23                                                CableLabs                                                                   181

<!-- PDF_PAGE: 182 -->
CM-SP-PHYv3.1-I20-230419                                     Data-Over-Cable Service Interface Specifications




      Figure 93 - NCP Profile Update Bit Setting Immediately Preceding an NCP Bit Loading Profile Change
The CMTS MUST assert the Last bit ("L") if the NCP is the last NCP message block.
The CMTS MUST follow the NCP block that has its Last bit asserted with a CRC-24-D. The CRC-24-D is
calculated across all message blocks in a symbol exclusive of the FEC parity bits.
When a Downstream Profile Usability test is in progress with Codeword Tagging enabled, the CMTS assigns the
value of the Codeword Tagging bit ("T") to indicate whether the codeword is to be included in the codeword counts
reported in the OPT-REQ message [DOCSIS MULPIv3.1].
When a Downstream Profile Usability test is in progress and Codeword Tagging is enabled by the OPT-REQ
message for the CM's test profile, the CM is required to respect the Codeword Tagging ("T") bit [DOCSIS
MULPIv3.1].
A NULL NCP is defined as an NCP with the start pointer set to 0x1FFF. The usage of Null NCPs is defined in the
next section. An Active NCP is an NCP that points to valid FEC codeword. Therefore, an Active NCP is an NCP in
which Z-bit is Zero and the Start Pointer is not equal to 0x1FFF.

8.3.4.2    NCP Field with CRC and FEC
The last NCP message block is a dedicated CRC block. This block is shown in Figure 94.




                                     Figure 94 - NCP CRC Message Block
The CMTS MUST include a NCP CRC message block based upon the CRC-24-D calculation after the last NCP
data message.
The CM MUST check the NCP CRC field.
If the NCP CRC field indicates an error in the NCP field, then the CM MUST reject all NCP data message blocks in
the NCP field of the current symbol.
The CRC-24-D, defined in Annex A - QAM Constellation Mappings (Normative), is applicable to a bitstream.
Hence to work out this CRC the CMTS has to first map the NCP message blocks into a bit-stream. The CMTS
MUST implement this conversion to bit-serial format in MSB-first order. That is, the CMTS MUST place the first
bit of the bitstream as the leftmost bit of the Profile ID shown in Figure 92 - NCP Data Message Block, of the
topmost NCP message block shown in Figure 91 - Data and NCP Prior to Interleaving.
NCP has a specific method of mapping NCP message blocks with FEC that is different than the FEC used on the
main OFDM data channels. The complete NCP field with FEC parity is shown in Figure 95.




182                                               CableLabs                                            04/19/23

<!-- PDF_PAGE: 183 -->
Physical Layer Specification                                                        CM-SP-PHYv3.1-I20-230419




                                 Figure 95 - NCP Message Blocks Field with FEC
NOTE: Each three byte NCP MB is mapped into a unique FEC codeword that has a 3 byte payload with 3 bytes of
      FEC. The last FEC codeword is then followed by a 3 byte CRC-24-D (refer to 24-bit Cyclic Redundancy
      Check (CRC) Code) that is also placed in its own FEC block.

8.3.4.3    NCP Usage
The CMTS MUST NOT place more than 11 NCP data message blocks plus a CRC for a total of 12 NCP MBs in an
8K OFDM symbol. The CMTS MUST NOT place more than 12 NCP data message blocks plus two CRCs for a
total of 14 NCP MBs in any two successive 4K OFDM symbols.
In the case of an 8K FFT OFDM symbol, the 12 NCP MBs will be formed by a maximum of 10 active NCP MBs,
the NULL or zero-bit-loaded NCP MB (i.e., NCP with Z-bit set to ONE) and the NCP CRC MB.
In the case of a 4K FFT, in addition to the 10 maximum active NCP MBs over two successive symbols, each of
these symbols may have one additional NULL or zero-bit-loaded NCP MB, and each symbol will have an NCP
CRC MB. This brings the maximum number of NCP MBs over two successive symbols to 14.
If the data FEC blocks are small in one 8K FFT OFDM symbol, there could be data subcarriers left in the symbol
after the placement of 10 active NCPs and the corresponding data. In such a case the CMTS MUST include an NCP
describing the remaining subcarriers as zero-bit-loaded ("Z" bit asserted).
The CMTS MUST NOT place more than 10 active NCPs in any two consecutive 4K OFDM symbols, i.e., the
number of active NCPs in 4K FFT OFDM symbols n and n+1 MUST NOT exceed 10, for any value of n.
If the data FEC blocks are small, there could be data subcarriers remaining and unused after the placement of 10
active NCPs and the corresponding data in two consecutive 4K OFDM symbols. In such a case, the CMTS MUST
include an NCP describing the remaining subcarriers as zero-bit-loaded ("Z" bit asserted). Furthermore, in the case
of 4K FFT, if all of the 10 active NCPs are consumed by the symbol n, then the CMTS MUST place an NCP in
symbol n+1 indicating that the unused subcarriers are zero-bit-loaded. The symbol n+1 may contain a continuation
of a codeword from symbol n, but no new codeword can start in symbol n+1.
For small bandwidths it is possible that there may not be a beginning or an end of a FEC codeword in a symbol.
That is, a codeword may begin in the previous symbol and end in the following symbol. In such a case the CMTS
MUST insert a NULL NCP in the current symbol.
There may also be scenarios in which a FEC codeword may end within a symbol without leaving sufficient space to
include an NCP. In this case, the CMTS MUST insert a NULL NCP and move some of the data subcarriers of the
FEC codeword to next OFDM symbol.
The CMTS MUST NOT place more than one NCP with Z-bit set to one or with subcarrier pointer set to 0x1FFF, in
any OFDM symbol. That is, if a symbol contains an NCP with Z-bit set to one then there cannot be an NCP with
subcarrier pointer set to 0x1FFF in the same symbol. Similarly, if a symbol contains an NCP with subcarrier pointer
set to 0x1FFF then that symbol cannot contain an NCP with Z-bit set to one. It must be noted that there is no
requirement to have an NCP with Z-bit set to one or with subcarrier pointer set to 0x1FFF in every symbol.
However, if an NCP with either the Z-bit set to one or with subcarrier field set to 0x1FFF exists in a symbol, then
the CMTS MUST ensure that NCP is the last NCP of that symbol before the CRC NCP.
It is also possible that an FEC codeword may end within a symbol and leave sufficient space to include an NCP but
no more. That is, after inserting the NCP, there would be no available subcarriers for data. In this case, the CMTS


04/19/23                                           CableLabs                                                   183

<!-- PDF_PAGE: 184 -->
CM-SP-PHYv3.1-I20-230419                                            Data-Over-Cable Service Interface Specifications


MUST insert a NULL NCP. This is equivalent to the above paragraph, but with no data subcarriers of the FEC
codeword being moved to the next OFDM symbol.

8.3.4.4    NCP Examples
Figure 96 shows some examples of how the NCP field is used. This view is prior to interleaving. NCP blocks are
mapped to subcarriers starting with the first non-excluded subcarrier at the top of the spectrum and then down in
frequency. After the last NCP MB is a CRC-24-D.




                                                Figure 96 - NCP Examples
Data is mapped to the first non-excluded subcarrier at the bottom of the frequency range and then continuing
upwards in frequency.
  In symbol 1,   Codeword A starts at the beginning of the symbol and has a start pointer. Codeword B starts after codeword A
                 and has a start pointer. The length of codeword A is the difference between the codeword A start pointer and
                 the codeword B start pointer.
  In symbol 2,   Codeword C starts at the beginning of the symbol and has a start pointer. The length of the previous codeword
                 B is derived from the difference between the codeword B start pointer and the codeword C start pointer, taking
                 into account where the last data subcarrier was in symbol 1. Codeword D gets a start pointer.
  In symbol 3,   Codeword D continues from symbol 2 and finishes. Codeword A follows and is given a start pointer. The length
                 of codeword D is derived from the difference between the codeword C start pointer and the codeword D start
                 pointer, taking into account where the last data subcarrier was in symbol 2.
  In symbol 4,   Codeword A continues. Since there is no start pointer required, but at least one NCP block is required, an NCP
                 block with a null pointer is included.
  In symbol 5,   Codeword A ends. Codeword B begins and ends. A single NCP block is created with a start pointer to
                 codeword B.
  In symbol 6,   Codeword C both starts and ends. A single NCP block is created with a start pointer to codeword C.
  In symbol 7,   Codeword D starts and ends. There are no more data packets to send, so the remaining subcarriers are
                 unused. An NCP block is assigned for the codeword D start pointer. A second NCP block is assigned to the
                 start pointer of the unused subcarriers. This start pointer is used to determine the length of codeword D.
  In symbol 8,   Codeword A begins and ends. Codeword B begins and tried to end with a few subcarriers unused between the
                 end of the data codeword and the end of the NCP field. Since no subcarriers can be left unused, and since an
                 NCP would not fit, an NCP with a null pointer was inserted and some of the last few bytes of codeword B were
                 forced into the next symbol. There is an NCP message block for codeword A, codeword B, and the null NCP.
  In symbol 9,   Codeword C starts a few subcarriers into the symbol. There is one NCP block for codeword C.




184                                                     CableLabs                                                     04/19/23
