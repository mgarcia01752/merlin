# Downstream OFDM scope and modulation coverage

DOCSIS OFDM Channel Sim is DOCSIS-focused. Build the downstream OFDM engine first; OFDMA is deferred.
The product target is profile evaluation and real FEC simulation across the data
constellations allowed by the pinned DOCSIS3.1/4.0 downstream definitions.

## Verified normative maximum

The local PHY4.0 I07 (CM-SP-PHYv4.0-I07-250627), section7.3.5.5, PDF134 delegates
FDD downstream bit-to-constellation mapping to PHY3.1 section7.5.5.
PHY3.1 I20 (CM-SP-PHYv3.1-I20-230419), section7.5.5.1, PDF120 identifies the
mandatory and optional endpoint modulation support below. The maximum listed
is **16384-QAM (16K-QAM), carrying14bits per symbol**. Both8192-QAM and16384-QAM
are optional for CM/CMTS; they already appear in the DOCSIS3.1 baseline.

| Data constellation | Bits/symbol | Pinned CM/CMTS requirement | Profile analysis | Mapping/soft demapping | CLI simulation job |
| --- | ---: | --- | --- | --- | --- |
| 16-QAM | 4 | Required | Implemented | Experimental | Initial experimental mode |
| 64-QAM | 6 | Required | Implemented | Experimental | Pending |
| 128-QAM | 7 | Required | Implemented | Experimental | Pending |
| 256-QAM | 8 | Required | Implemented | Experimental | Separate fixed-profile channel command |
| 512-QAM | 9 | Required | Implemented | Experimental | Pending |
| 1024-QAM | 10 | Required | Implemented | Experimental | Pending |
| 2048-QAM | 11 | Required | Implemented | Experimental | Pending |
| 4096-QAM | 12 | Required | Implemented | Experimental | Pending |
| 8192-QAM | 13 | Optional | Implemented | Experimental | Pending |
| 16384-QAM | 14 | Optional | Implemented | Experimental | Pending |

“Profile analysis” means role-preserving assignment handling, named analytical
MER targets and capacity accounting; it is not a verified mapper or decoder
simulation for that order. The typed `OfdmModulation` registry exposes this
separation. The default analytical cap is14; applications may choose a lower
cap for specific device capabilities. Pilot, PLC, NCP and zero-loaded roles are
separate from data modulation; recognizing a role does not make it user data.
Shared [mapping and reference max-log demapping](../user/qam-mapping.md) now use
embedded label-ordered resources through16K-QAM. Mapping is marked experimental
independently of complete coding-mode availability.

A separate [noiseless256-QAM cell loop](../user/qam256-coding-loop.md) now covers
BCH/LDPC, parity/twist/demultiplexing, randomization and soft inverse processing.
It reports iterations per codeword but does not yet have physical symbol packing
or integration into the general CLI `simulate` coding-mode selector. The separate
[channel CLI](../user/channel-runner/cli.md) now implements uniform16/256 and
non-shortened mixed-profile waveform transport. The final column and registry still describe the older generic
`simulate` selector, not the channel command.

## Coding work still required

Bit processing is implemented for uniform non-shortened short8/9 modes16and256.
For the other coding orders, verify column twists, demultiplexing, pad bits,
shortening and codeword placement. Extend independent high-order label vectors
and decoder validation beyond current diagram anchors and reference-table checks.
Non-square orders use their actual resources, not a square-QAM approximation.
High-order scrambling and bit ordering
must follow the pinned source. The [mixed-profile waveform path](../user/payload-transport/applied-profiles.md)
now implements24-column bit processing, inverse-frequency loading, variable-width
cells and final padding through actual FEC. Independent external high-order
interoperability vectors remain pending; this does not complete QAM-04.

DOCSIS4.0 FDX and FDD remain distinct applicability scopes. This table follows
I07's FDD delegation; it does not establish FDX sounding, echo cancellation or
receiver equivalence. I07/I20 are the local editions checked here, not a claim
that every later revision has been independently verified.

## FEC mother mode and shortened lengths

PHY I20 §7.5.4, PDF112–114 excludes normal64800-bit downstream frames and uses
only the short16200-bit rate8/9 mother code. PHY4.0 I07 §7.2.5.4/§7.3.5.4 delegates
FDX/FDD downstream FEC to that definition. FEC-03 now implements true shortened
codec boundaries, retaining168 BCH and1800 LDPC parity bits for transmitted
length K+1968. See [FEC shortening](../user/fec-shortening/README.md).

This does not complete the bit-interleaving/QAM/NCP/waveform work tracked in
QAM-03. The current channel still explicitly rejects shortened transport, and
SRC file EOF uses its documented full-information-word padding policy.
