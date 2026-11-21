# Upstream measurement admission

[Upstream guide](README.md) · [Synthetic fixtures](../../../examples/measurements/upstream/README.md)

The native `pnm-input::upstream` adapter owns raw upstream observations. It retains
original bytes and SHA-256 fingerprints, exact source identities, grids, timestamps
and missing readings. Decoding does not start a simulation or choose associations.

```mermaid
flowchart LR
    Bytes[Original bytes] --> Decode[Bounded format admission]
    Decode --> Observation[Immutable observation and provenance]
    Observation --> Context[Explicit association and model context]
    Context --> PHY[Typed upstream configuration]
```

UCD inspection accepts management envelopes starting at DA and excluding CRC.
Multipart reassembly requires a complete ordered set with consistent identity;
its canonical assembled view is derived data. Callers retain each physical
fragment. Semantic projection additionally requires explicit PHY bandwidth.

CCAP RxMER admission supports the documented major2 profile, quarter-dB readings
and `0xff` missing positions. Opaque flags, vendor revisions and identifier padding
remain inspectable. Inspection alone does not establish a CM/channel/time cohort
or a residual-noise model.

Pre-Eq admission distinguishes applied s2.13 banks from last-update s1.14 values.
For minor zero it delegates to the exact pinned PNM decoder; the approved opaque
minor profile uses that decoder's fixed-point kernel after bounded envelope
checks. No header bytes are rewritten. Update files still require external
operation, application status, corresponding probe and base-bank context.

Measured correction C is distinct from physical channel H and receive noise.
Whole-plan association and playback now pass local Rust acceptance.
`MeasuredExperiment::from_json` admits bounded plan JSON; `admit` prepares the
core-owned input for `Simulation::configure_measured`. Loading is idle and failed
admission preserves the previous session. Native Python and CLI exposure follow
the public API gates. Existing downstream capture
admission keeps its established contracts.


Plans require explicit CM/CMTS identity, CCAP-interface mapping when RxMER is used,
source-time bounds, exact UCD/grid compatibility and source interpretation evidence.
RxMER selection, equal-weight power average, hold or interpolation is explicit;
source epochs are retained separately from replay offsets. No frequency resampling
or missing-bin invention occurs. Updates require application status and their
corresponding probe/base; an older base can remain valid when that exact probe
observed it. Entire schedules are checked before a run begins.

The supported measured-RxMER model uses unity physical H and unity effective C
throughout, without added timing/noise impairments. Pre-Eq-only input does not need
invented CCAP or RxMER metadata. Original files, raw plan JSON and hashes survive
completed/cancelled reports; normalized effective banks remain derived values.
