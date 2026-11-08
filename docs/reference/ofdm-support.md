# Downstream OFDM scope and modulation coverage

DOFT is DOCSIS-focused. Build the downstream OFDM engine first; OFDMA is deferred.
The product target is profile evaluation and real FEC simulation across the data
constellations allowed by the pinned DOCSIS3.1/4.0 downstream definitions.

## Verified normative maximum

The local PHY4.0 I07 (CM-SP-PHYv4.0-I07-250627), section7.3.5.5, PDF134 delegates
FDD downstream bit-to-constellation mapping to PHY3.1 section7.5.5.
PHY3.1 I20 (CM-SP-PHYv3.1-I20-230419), section7.5.5.1, PDF120 identifies the
mandatory and optional endpoint modulation support below. The maximum listed
is **16384-QAM (16K-QAM), carrying14bits per symbol**. Both8192-QAM and16384-QAM
are optional for CM/CMTS; they already appear in the DOCSIS3.1 baseline.

| Data constellation | Bits/symbol | Pinned CM/CMTS requirement | Profile analysis | Full coding simulation |
| --- | ---: | --- | --- | --- |
| 16-QAM | 4 | Required | Implemented | Initial experimental mode |
| 64-QAM | 6 | Required | Implemented | Pending |
| 128-QAM | 7 | Required | Implemented | Pending |
| 256-QAM | 8 | Required | Implemented | Pending |
| 512-QAM | 9 | Required | Implemented | Pending |
| 1024-QAM | 10 | Required | Implemented | Pending |
| 2048-QAM | 11 | Required | Implemented | Pending |
| 4096-QAM | 12 | Required | Implemented | Pending |
| 8192-QAM | 13 | Optional | Implemented | Pending |
| 16384-QAM | 14 | Optional | Implemented | Pending |

“Profile analysis” means role-preserving assignment handling, named analytical
MER targets and capacity accounting; it is not a verified mapper or decoder
simulation for that order. The typed `OfdmModulation` registry exposes this
separation. The default analytical cap is14; applications may choose a lower
cap for specific device capabilities. Pilot, PLC, NCP and zero-loaded roles are
separate from data modulation; recognizing a role does not make it user data.

## Coding work still required

For each pending order, verify and test normative constellation labels,
normalization, soft demapping, column twists, demultiplexing, pad bits,
shortening and codeword placement. Non-square orders need their actual mappings,
not a generic square-QAM approximation. High-order scrambling and bit ordering
must follow the pinned source. Mixed profiles require verified per-carrier
placement before claiming a complete mixed-modulation FEC chain.

DOCSIS4.0 FDX and FDD remain distinct applicability scopes. This table follows
I07's FDD delegation; it does not establish FDX sounding, echo cancellation or
receiver equivalence. I07/I20 are the local editions checked here, not a claim
that every later revision has been independently verified.
