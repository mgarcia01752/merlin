# Spectrum Analyzer validation

Validated locally on Ubuntu 22.04 x64 with Qt 6.8.3 on 2026-11-09.

- Scoped scientific tests: 7 passed (spectrum, actual transmitter preview and
  existing waveform transforms). Independent tone power, Parseval normalization,
  Hann bandwidth and both FFT spacings are covered.
- Generic spectrum without simulation features: 2 passed. The existing
  simulation-disabled SymbolPacker dead-code warning remains unchanged.
- Core library Clippy with warnings denied passed; Rust formatting passed.
- Qt check: 9 Rust adapter tests and 21 QML tests passed, including cancellation,
  stale result rejection, immutable axis mapping, bounded views, accepted-input
  readiness, zoom and invalidation. Native executable rebuilt from final source.
- Actual VM review: accepted default PRBS-TX and demo DPD Profile A, observed
  four green source/OCD/DPD/Transmitter blocks, opened SA and generated actual
  TX samples. Reviewed centered baseband and capture frequency axes, zoom and
  full span. Dark screenshots are published in the
  [user guide](../../../user/spectrum-analyzer.md). Start remains disabled while
  remaining workbench inputs and full-loop dispatch are pending.

The preview contains 135168 time samples, 32 averaged FFT windows and a 25 kHz
bin spacing for the demo. The displayed Hann equivalent noise bandwidth is
37.5 kHz. No RF upconversion or calibrated RF power is claimed. Source bit counts
include queued words and do not prove complete transmission or recovery.

Full workspace/DSP testing was not repeated. Ubuntu 24.04/26.04 runtime validation,
additional SA tap wiring and full-loop UI jobs remain pending. Qt now enables the
existing simulation feature and inherits its reviewed dependency graph. The known
rand 0.9.2 RUSTSEC-2026-0097 finding remains open under the security task; no
advisory suppression or new audit-scan result is claimed here.

UI-SA-03 follow-up: removed lower-panel text and updated actual baseband/mapped
VM screenshots. The button now reads Run simulation and its tooltip explains
the transmitter-only scope. The updated QML suite passes all 21 checks. The
latest Qt executable is rebuilt; no scientific code or dependency changes were
needed for this presentation revision. UI-SA-04's timing/cable graphic is labeled
a proposal and is not runtime evidence.

## UI-SA-04: Integrated timing/window and cable panel

- Two independent waveform-timing tests pass: known µs/feet results and half-path
  convention, invalid/unknown VF, both FFT spacings, all valid CP/RP pairs, actual
  half-sample window endpoints and bounded display data. The same two tests pass
  without simulation features; the unchanged SymbolPacker warning remains.
- Existing transmitter-preview test passes after retaining frozen settings in
  completed results. No full DSP/workspace suite was repeated.
- Rust formatting and warnings-denied core-library/Qt Clippy pass. Qt validation
  passes 9 Rust tests and 21 QML tests, including cable selection and timing
  invalidation. Accepted inputs and completed samples are unchanged by queries.
- Native Qt 6.8.3 on the Ubuntu 22.04 x64 VM: reviewed PSD plus compact lower
  window panel at 1210×736 and minimum 960×640. Verified preset distances, blank
  custom VF, and a custom VF entry. No native QML diagnostics were emitted.
- Updated actual dark screenshots in the user guide. Latest compiled executable
  is left running with the baseband SA open. Other Ubuntu runtimes remain untested.

The lower plot displays actual TX window coefficients with relative symbol time,
not an IQ amplitude trace or acquired echo. Ideal cable estimates assume a uniform
cable, nominal VF, two-way reflection for spacing and zero reserved timing margin.
