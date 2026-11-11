# UI-DPD-VIEW-01: Visual modulation profile

Add View Profile to the DPD input editor, enabled only for a loaded draft.
Open a separate read-only profile window showing bit loading versus the exact
captured frequency grid, with profile selection, carrier-role legend, pointer
readout, mouse zoom and Full Span. Reuse MERLIN's PNM Viewer-derived
MeasurementPlot and pinned in-process decoder rather than launching a viewer
or introducing Python. Rust owns profile validation, coordinates and bounded
role-aware min/max grouping; QML owns drawing and transient viewport selection.

Hide capture filenames in this editor (including provenance tree rows), keeping
original names/bytes/hashes in WB persistence. View selection never applies a
draft, changes readiness or starts a simulation; Cancel retains accepted input.
Plot selections are transient and require no WB schema change. Unknown loading
remains absent; special roles without a loading value use explicit baseline
markers, not a claimed zero-bit assignment. Do not connect across role/gap groups.

Own implementation under inputs/profile_input in Rust and QML, one file per
window, with shared chart changes preserving existing RxMER/response rendering.
Verify binary/JSON parity, frequency endpoints, grouping/gaps and window
navigation, then build the Qt executable and capture the real dark VM windows.
Update the modulation-profile user guide, architecture and checklists.

Implemented in `inputs/profile_input/plot.rs`, the existing ProfileInput bridge
and a separate `ProfilePlotDialog.qml`. It composes the shared MeasurementPlot,
already adapted from PNM Viewer's RxMerBitLoadingChart, and ChartNavigation.
No additional viewer checkout, dependency, subprocess or Python runtime is used.
Each viewport has at most 512 frequency groups per carrier role (seven roles),
retains min/max loading and breaks joins across grouped/role boundaries.
Missing loading is preserved as null alongside an explicit baseline role marker.

Validation on the Ubuntu VM: all 27 adapter tests and 32 QML checks pass;
warnings-denied Qt Clippy, rustfmt and whitespace checks pass. The new test
compares binary/JSON plots, exact frequencies, role markers and range rejection;
the dialog test covers View Profile, zoom/Full Span and unchanged acceptance.
Actual native dark editor and profile windows were inspected and captured as
`dpd-input-dark.png` and `dpd-profile-plot-dark.png` in the user guide. Native
startup emitted no diagnostics. The development executable is rebuilt; this does
not replace the installed v0.1.1 application. No non-Qt workspace suite was run.
