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
[channel CLI](../user/channel-runner/cli.md) now implements all ten uniform orders and
non-shortened mixed-profile waveform transport. The final column and registry still describe the older generic
`simulate` selector, not the channel command.

## Implemented coding and remaining evidence

QAM-02 provides diagnostic hard decisions without replacing soft LDPC inputs.
QAM-03 implements all ten full uniform short8/9 coding orders, including their
per-order twists/demultiplexing and explicit padding. Non-square modes retain
their actual resources. The channel command supports those frozen uniform
profiles and the existing mixed path. The old general `simulate` selector and
its capability field remain explicitly16-QAM-only.

The [QAM processing guides](../user/qam-processing/README.md) explain exact stages,
padding coordinates, shortened matrix holes and bounded received-length discovery.
Explicit shortened transport is available through public Rust APIs and the
`shortened_waveform` JSON example. General PRBS/file channel jobs keep full-word
EOF padding. There is no automatic MAC framing or new generic shortening option.

The [mixed-profile path](../user/payload-transport/applied-profiles.md) uses its
own24-column rule, inverse-frequency loading and sequential cell grouping.
QAM-04 checks every data label against pinned PyPNM text rows and verifies the
supported coding/profile paths through MERLIN's own TX/channel/RX. It is complete
for the documented closed-loop minimum. External interoperability is optional
future scope; its [reference plan](../development/features/qam-completion/external-vectors.md)
is retained without making it an acceptance prerequisite.

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

QAM-03 connects this codec to explicit bit-interleaving/QAM/NCP/waveform
transport. Pointer-based shortening imposes its separate odd-byte wire-size
admission rule. SRC file EOF still uses its documented full-information-word
padding policy. No64800-bit downstream mother mode is introduced.
