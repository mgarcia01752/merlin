# UI-TX-03: Transmitter workbench and message inspection

## Approved implementation scope

The user approved implementation, placing the shared SA immediately below the
final TX Output test point and removing the small MERLIN eyebrow from the main
header. The full toolkit title and logo remain. Replace Transmitter's direct SA
entry with a staged dialog; retain the existing transmit-only SA preview fallback.

Collect bounded actual TX snapshots in the existing closed-loop observer. Reuse
registered payload/waveform producers; add the missing PLC-frame and physical-bin
observation calls to the channel link without modifying stage calculations.
Allow a per-stage probe phase override so PLC acquisition messages can be observed
without spending RX observation-only budgets on acquisition. Reassemble actual
observed PLC information blocks using the existing reassembler and packet/TLV
parser. Label completed versus incomplete scheduled PLC frames from the actual
transmitted-sample total, not from configured duration. No PNM bytes are substituted.

Use separate workbench, stage, QAM and message dialogs with shared chip/SA controls.
The new adapter remains bounded and reads immutable results. The bit inspector
exposes the supported LDPC boundary; do not claim a separately retained BCH word.
The QAM inspector preserves actual cell roles and unavailable history. Cancellation
or input invalidation must not leak stale results. Scoped adapter/QML tests plus
focused core observer/channel tests cover the necessary integration changes.

## Implementation

`TransmitterWorkbenchDialog.qml` replaces the direct SA entry and reuses shared
chip/SA controls. Stage, QAM, scatter and message views are separate QML files.
The existing TransmitterAnalyzer bridge adds bounded query methods; focused
`transmitter_instruments/{capture,views,messages}.rs` modules own presentation
admission and finite copied data. `closed_loop/model.rs` composes TX capture into
the existing immutable result, discarded on input changes or a new run.

The core channel link now invokes the existing physical-bin and PLC observation
functions after stage calculations. The nonbreaking per-stage phase-filter API
allows acquisition PLC frames while other snapshots remain observation-only.
Existing global filters and budgets retain their behavior. There are no new
DSP algorithms, wire formats or packet serializers.

Capture limits: one16K-element snapshot per selected bit/cell/bin boundary,
one symbol for IFFT/prefix/window,32768 contiguous overlap-add samples,64 complete
generated PLC information frames and32 inspected management packets. Presentation
pages contain64 rows (32bits per row for bit streams); QAM has at most2048 actual
points. Missing cells remain null/placeholder values. Profile-mode labels may be
unavailable because that path emits interleaved bits rather than uniform labels.

The core observer's `tx.plc.message` contains complete generated TS+MC frames.
Their MAC packets are independently reassembled and validated by existing APIs.
Generated frame bytes do not alone establish emission: a packet is labeled carried
only when the actual transmitted-sample total covers the last sample of its
completion frame. Partial frame/packet and capture-limit notes remain visible.
OCD and both data/NCP DPDs retain separate profile identity and TLV offsets into
the complete packet. Field labels follow the existing fixed-subset implementation
in `transmission/plc/descriptors.rs`; raw values remain exact hex, not a newly
invented semantic decoder or claim of expanded MULPI support.

Mixed Data/NCP/filler QAM is labeled explicitly; PLC data/preamble are separate.
The coding chip groups BCH/LDPC processing but its retained boundary is LDPC.
NCP/pilots share physical-bin inspection; independent NCP-only QAM and separate
BCH/individual permutation snapshots are not claimed. SA defaults to closed-loop
TX output after Start, with IFFT/prefix/window choices. Before any completed run,
TX Output retains the existing independent preview with timing/echo display.
Opening the workbench never starts that preview. Closing an inspector does not
stop the main worker or alter accepted settings.

```mermaid
flowchart LR
    SRC[Source] --> FEC[BCH / LDPC]
    FEC --> BIT[Bit processing / randomizer]
    BIT --> QAM[QAM] --> INT[Cell interleavers] --> FRAME[Frame assembly]
    MSG[OCD / DPD] --> PLC[PLC coding / mapping] --> FRAME
    PROTECTED[NCP / pilots] --> FRAME
    FRAME --> WAVE[IFFT / CP / window / overlap] --> OUT[TX output]
    OUT -.-> SA[Spectrum Analyzer]
    MSG -.-> INSPECT[Header / TLV / bytes]
```

## Historical proposal

The initial proposal requested layout review before implementation. Clicking Transmitter would
open a staged workbench matching the Receiver, with rounded processing chips,
selectable test points and shared instruments. Existing green readiness remains
derived from accepted OCD, DPD and source; instrument selection does not gate Start.

Payload path: source → BCH/LDPC → bit processing/randomization → QAM → time/frequency
cell interleaving → physical frame assembly → IFFT/CP/window/overlap-add.
PLC message coding, NCP generation/coding and pilot generation are separate paths
feeding frame assembly; they do not pass through the payload coding chain.

Reuse the existing stage APIs and test points listed in architecture. Proposed
instruments: bits/codewords, QAM by signal role, physical bins and shared SA at
waveform boundaries. Bits and bins must not be presented as time-domain I/Q.
Stage availability needs an adapter audit: a core direct output does not imply
it is already retained by the desktop run. Bound snapshots and report unavailable
data rather than manufacturing observations or silently rerunning a simulation.

Message Inspector would expose actual generated/scheduled OCD and DPD management
packets carried by PLC: message tree, header, TLV type/length/value/offset and exact
bytes. Do not confuse imported modulation-profile PNM bytes with transmitted DPD
MAC bytes. Distinguish configured, generated/scheduled and actually emitted data;
a short run may not emit the complete descriptor cycle. Preserve message profile
identity, change counts, timing and packet integrity results where available.
Do not invent TLV numbers or values. Use existing packet serializers/decoders and
pinned MULPI evidence when field interpretation needs extending.

Presentation would live in separate TransmitterWorkbenchDialog and message/bit
inspector dialogs, sharing chip, QAM and SA controls where applicable. Observation
adapters own typed bounded data; QML only presents it. No DSP changes are planned.
Applied input changes invalidate stale observations; opening inspectors is read-only.

The dark interactive proposal is
`docs/images/workbench/transmitter-workbench-proposal.html`, linked in the user
guide. It contains field labels, no captured values or invented hex. It is a design
preview, not runtime evidence. The initial proposal changed no application code.
Implementation validation will cover tap identity, limits, message-byte round trips,
read-only behavior, short-run availability and actual dark VM screenshots.

Proposal validation: JavaScript syntax and all stage/test-point/message controls
checked with a local DOM harness; documentation links and diff whitespace pass.
No Qt build or processing tests are needed for this documentation-only preview.

A static PNG of the same proposal was rendered with headless Chrome and visually
inspected for chat and the user guide. It remains a mockup, not a Qt screenshot.

## Implementation validation

UI-TX-03 is complete within the bounded stage set above. Formatting, warnings-denied
Clippy,17 Qt adapter tests and27 QML checks pass. The adapter's real closed-loop
case verifies TX source/coded/cell/bin pages, all four waveform spectra, actual OCD
and both DPDs, exact packet-byte round trips through CRC validation, TLV offsets,
invalid selections and stale-result clearing. A focused partial-frame test checks
the last-sample emission threshold and checked arithmetic.

Thirteen focused core tests pass (`channel_job` and `run_infrastructure`). Enabled
PLC/frame-bin observation preserves the complete channel summary and recovered
words versus disabled observation. Per-stage phase overrides preserve other-stage
filters, budgets and rejection rules. No full non-Qt workspace suite was run.
Final QML checks also cover the SA's output connection, dialogs/X controls,
page navigation, message/byte selection, control-path widths and prevention of a
separate preview while the main worker is busy. Retained Cargo artifacts are kept.

Actual Ubuntu22.04/Qt6.8.3 VM review: the single bundled WB restores all green
inputs; a1ms run compares1,223,952 bits with zero errors. Native inspection shows
actual mapped I/Q,16200 LDPC bits, source/channel identity, OCD TLV/hex and NCP DPD
profile255/type50. TX output SA shows a real8192-point baseband spectrum and symbol
timing. The staged workbench fits both1210x736 and960x640; the redundant header
eyebrow is absent. Dark screenshots in the user guide are actual Qt captures,
separate from the historical mockup. Native logs contain no QML warnings.
The latest executable is built and restarted in the VM; other OS runtime and
installer validation remain unverified. Local logs use the `tx-staging-*` prefix
under ignored `docs/development/local/visual-workspace/`.
