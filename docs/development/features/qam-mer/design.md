# UI-QAM-MER-01 — Calculated constellation MER

The later [run-specific QAM change](../qam-run-results/design.md) adds actual
TX-reference MER as the primary reading. This nearest-point calculator remains
as a separate estimate.

Bring the PNM Viewer Toolkit constellation MER implementation into MERLIN's
Apache-2.0 scientific library. Keep original copyright attribution and NOTICE.
The read-only source is `crates/core/src/pnm/analysis/constellation/mer.rs` in
the viewer working tree, SHA-256
`7f0063d40931217564bb4c0aae0253ec224ea22abca1f6fcf374bd055c1173c9`.
Its checkout HEAD is `5fbd5561f3e06b73486546b5ffa520279080a5eb`; the hash identifies
the inspected working-tree file rather than claiming that HEAD contains it.

## Calculation and ownership

`signal_processing::modulation::mer` owns the copied power accumulator,
structured MER/EVM/status model, paired-reference operation and indexed
nearest-reference search. `MerCalculator` binds MERLIN's existing actual
constellation table and explicit reference amplitude. It consumes bounded caller
streams without Qt, capture parsers, Python, paths or UI dependencies. Future
Rust and Python consumers can reuse the same typed operation. No binding is
added here. Row pruning and binary search avoid scanning every QAM reference
for every received sample; non-square tables remain exact.

MER is `10 log10(sum |reference|² / sum |received-reference|²)`.
Never average dB, rescale the received samples, fit gain/phase, or silently discard
invalid samples. Empty, invalid, unsupported, zero-power and perfect-reference
cases remain distinct. Perfect agreement has null dB and zero EVM rather than
an invented finite ceiling or non-JSON infinity. Pilot references retain their
known factor-of-two amplitude. This is a decision-directed estimate, distinct
from imported RxMER and the existing per-bin TX-truth diagnostic. Incorrect
decisions under severe noise can overstate estimated MER.

The receiver adapter uses every retained cell matching role/profile/order/bin/
RX-symbol filters, before the 2048-point display cap. Raw RX-input and equalized
planes report separate readings. Channel Output uses exactly the same raw
selection and estimate as Receiver Raw. Unknown mixed physical TX-bin snapshots
before/after the amplifier have no invented MER because their retained points
lack per-cell reference modulation/role. They still get smaller round markers.
All channel and receiver QAM markers use radius 0.85 logical pixels. Transmitter
stage plots retain their existing symbol-role markers.

QML displays Rust labels and typed results, with hover explanation. Filters clear
unavailable readings, and no query starts a simulation or changes WB JSON.

## Validation

Check independent analytic power/EVM vectors, every supported constellation
against exhaustive nearest-point search, exceptional JSON-safe states, boosted
pilots and measurements beyond the display cap. Adapter tests verify filtering,
raw/equalized separation and actual channel/receiver reading parity. QML checks
cover paired readings, empty filters and marker size. Run scoped core tests,
formatting and warnings-denied Clippy, Qt checks and latest native build. Review
actual dark/light views and update the user-guide captures. Full permutation
simulation sweeps and installed packages are outside this additive diagnostic
change; record actual validation results before marking complete.

## Completed validation

Eight scoped core cases pass: four new arithmetic/index/status cases plus the
existing full-table provenance case and three hard-slicer reference cases. The
new calculator also passes all four new cases without default features; three
existing slicer cases pass there as well. Core all-target Clippy passes with
warnings denied. Qt formatting, Clippy, 62 Rust adapter tests, latest native
build and 54 QML checks pass. The QML test traverses the visual delegate tree
when checking dynamic paired-panel labels and explicitly restores the view.

Owned native Ubuntu dark/light runs verify paired raw/equalized readings,
small markers and Channel Output parity. A pilot-estimated run reports 16.72 dB
Raw and 39.34 dB Equalized across 65,536 cells while displaying only 2,048.
Raw distortion is included, rather than relabeled as post-EQ RxMER. Native logs
are free of QML warnings. Both dark captures are in the user guides; scripts
and intermediate captures stay under ignored `docs/development/local/qam-mer/`.
No full permutation sweep, other platform or installed package was tested.
