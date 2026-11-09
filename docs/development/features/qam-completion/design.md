# QAM-02..04: slicer, coding and profile transport

Status: QAM-02/03 implemented and validated; full pre-push gate passes. QAM-04 external
vectors deferred at the user's request; see [required evidence](external-vectors.md).

## QAM-02: diagnostic hard slicer

Own a focused `signal_processing/modulation/slicing/` module with a stateless
nearest-reference operation. Return the label, reference, received-minus-reference
error vector and squared distance. Equal computed distances select the lowest
label. Reject unsupported/nonfinite/overflowing inputs. Preserve soft LDPC paths.

## QAM-03: complete per-order bit processing and shortened mapping

Use PHY3.1 I20 §7.5.4.3 Tables24–30, PDF116–119 and §7.5.5.2 Tables32–37,
PDF121–123; square-mode demultiplexing delegates to ETSI EN302769 V1.2.1
Table10(a), PDF36. Original local PHY pages116–119/122 and ETSI36 were rendered and
visually checked. PHY4.0 I07 §7.3.5.5 delegates mapping to PHY3.1.

Full uniform modes retain parity ordering, append the specified zeros, apply
per-order twists and demultiplexing. Expose padded positions explicitly; reverse
soft processing discards these positions only after undoing their permutation.
Shortened and mixed modes use24 columns, bypass demultiplexing and group at the
actual per-cell loading. Shortened unused matrix positions are holes, not sent
zeros. Grouping pads only the last data cell. All work is bounded to one word.

```mermaid
flowchart LR
    FEC["Full or shortened BCH/LDPC word"] --> PARITY["Parity ordering"]
    PARITY --> TWIST["Mode-specific twist / explicit padding or holes"]
    TWIST --> GROUP["Uniform demux or mixed/shortened sequential grouping"]
    GROUP --> RANDOM["Shared cell randomizer"] --> QAM["Shared mapper"]
    QAM --> SOFT["Existing soft demapper"] --> INVERSE["Inverse randomization and bit ordering"]
    INVERSE --> DECODE["Existing full or shortened LDPC/BCH"]
    QAM -. "Diagnostic only" .-> SLICER["Hard slicer"]
```

Received boundaries must be discovered from accepted NCP pointers, not TX IDs or
lengths. PHY I20 §8.3.2 PDF178–179 constrains transmitted shortened codewords to
odd byte lengths. Its informative receiver algorithm rounds a terminated span
down to the nearest odd byte count. Keep bit-granular FEC admission separate
from byte-constrained transport. Do not silently change SRC padding or framing.

## QAM-04: frozen mixed profiles and external evidence

Retain current frozen ProfileA, inverse-frequency loading and live PLC/NCP path.
Expand tests to every supported order, zeros, final-cell padding, both spacings
and failed outcomes. Independently transcribed matrices are specification tests,
not external interoperability vectors. Search local resources for externally
produced high-order vectors; keep that subitem open if none is available.

## Owners, state and test points

Slicing is analysis-only and stateless. Bit ordering is immutable job-owned
geometry. Existing packer/collector own bounded cross-symbol state. FEC owns
mother-code operations; QAM modules never substitute expected bits on failure.
Expose permutation/padding coordinates, grouped labels, restored LLRs, actual
stage outcomes and original-bit comparisons. Unit tests cover independent table
anchors, ties, invalid dimensions, exact soft inverse and meaningful FEC trials.
Component guides and validation will record the delivered integration boundary.

## Delivered composition and policy choices

Full uniform profiles now carry their actual padded bit count through the existing
packer/collector. The default full-only receiver is preserved. Explicit shortened
TX uses the same profile-aware packer and sequential final-cell grouping; it never
passes lengths through the channel. `receiver/payload/variable.rs` is an opt-in
collector enabled before the session, holding at most16213 likelihoods until a
received start/filler boundary. It then chooses full or shortened inverse ordering,
restores mother known zeros and returns actual original-length decisions.

`simulation/payload/shortened/` owns the bounded reference composition and its typed
result. Actual PLC acquisition precedes independent RX construction. It provides
real trailing NCP filler before drain; EOF/NULL alone is not treated as a boundary.
All original bits stay with the experiment's comparison owner. General file/PRBS
jobs retain full-word padding and their existing JSON/cancellation policy.

The normative final-cell zero padding in §7.5.5.2.2 is used. The informative
§8.3.2 example's reference to trailing ones is not used to override that MUST.
The informative odd-byte receiver algorithm is an explicit transport policy, not
a new NCP length field or a claim of implemented DOCSIS MAC framing.

An initial alternating-per-carrier fixture exceeded the existing single management
packet limit; it was replaced with representable contiguous profile bands. No
packet-size bound was weakened. Large arbitrary DPD fragmentation remains outside
this task. Shared parity arithmetic now has one owner for full/shortened stages.
The code review preserved established APIs and kept new owners separate rather
than merging stateful collection with numerical permutations.

Component guides start at [QAM processing](../../../user/qam-processing/README.md).
The [validation record](validation.md) distinguishes independent specification
arithmetic from externally produced vectors and records the host/toolchain.
