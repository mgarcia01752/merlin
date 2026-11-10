> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/waveform/`
> Logs, captures and specification images are not published.

# Waveform validation

Implemented WAVE-TX-01..03, WAVE-CH-01, WAVE-RX-01..04 and WAVE-QA-01..02.
Separate safe Rust transform, prefix, taper, overlap, framing, reference
response compensation and extraction owners feed thin TX/RX compositions.

## Checks

- Full scripts/check-before-push.sh: PASS after the final source/test changes.
  Includes formatting, warnings-denied all-target Clippy, workspace tests,
  analysis-only tests, release CLI build and JSON smoke simulation, version
  consistency, 32 release/Git helper tests, Bash syntax and ShellCheck.
- Fifteen new Rust tests across six component/integration files; every CP/RP
  pair and both FFT sizes, independent transform and window identities,
  overflow/bounds, arbitrary stream offsets/splits, continuation/tail reporting,
  known complex-gain equalization, protected extraction identity and rejected
  observation metadata covered.
- Thirteen TX/channel/RX producers: finite selection/budgets, preserved original
  coordinates, enabled/disabled/exhausted experiment equality, failed callback
  with subsequent overlap-state equality, and full producer registration checked.
- Integration: two actual PLC frames per spacing over a192MHz configured cap
  and wide synthetic active envelope with exclusions; raw FFT bins compared,
  actual PLC FEC/CRC/descriptors recovered, zero information-bit errors, Ready.
- Example: two frames per spacing over a24MHz synthetic envelope; zero errors,
  Ready; max complex bin errors about1.24e-15 and1.08e-15. JSON assertions pass.
- Analysis-only all-target Clippy: PASS; no simulation/native FFT dependency.
- 195 local documentation links/anchors, balanced fences and seventeen Mermaid
  blocks checked. Diagrams were not rendered; documentation is source-verified.
- scientific-testing skill: validates at canonical path and both discovery links.
- git diff --check: PASS.

## Simulation scope

Local PHY I20 PDF133 and69 visually verified; exact unitary centered transform
and half-sample Tukey taper implemented. Neighboring text/tables provide CP/RP
and overlap behavior; PHY4.0 FDD delegates as recorded in tracked evidence.
No external interoperability vectors or Windows/macOS runtime validation claimed.
RX timing/layout/response are explicit reference/bootstrap assumptions. No blind
acquisition, nonideal channel, production payload/NCP waveform decoding or full
one-second job is implemented here. WAVE-RX-05/06 and RUN remain unchecked.
Data cells are deterministic256-QAM references; actual PLC messages, pilots and
preambles traverse the complete sample path. No new dependencies or version bump.

## Implementation findings

The local IDFT is centered and unitary, not an unscaled library-default transform.
The window equation requires a half-sample offset. Initial integration and all
component tests pass. A compiler Option/Result adaptation and a test iterator
lint were corrected before the final gate; no outstanding failures remain.
