# RXMER-DESIGN-01 validation

Local host: Ubuntu 22.04 x86_64, pinned Rust 1.85.1 and Qt 6.8.3 gcc_64.
Hosted/native Windows and macOS validation was not performed for this change.

- `scripts/test-local.sh all`: 386 workspace tests, 136 analysis-only tests and
  four example tests pass, with doctest phases preserved. The four new numerical
  cases independently anchor signed tilt, cosine crest spacing, distance units,
  inclusive 10–20-of-50 scope, notch edges, exact replacements, gaps and rejection.
- Workspace formatting and all-target warnings-denied Clippy pass.
- `scripts/qt-desktop.sh check` plus the final QML rerun: 73 adapter tests and
  63 QML checks pass, plus
  warnings-denied Qt Clippy and the latest executable build.
- Adapter coverage includes native-envelope and WB roundtrip, retained source
  identity/time, repeated Apply without compounding, synthetic provenance,
  cancellation of already completed success, stale draft/profile rejection,
  remapped selections and exact PLC-region clipping, including one-carrier zoom.
- A designed 48 dB flat frame produces `10^(-48/10)` residual error power through
  the existing playback planner. A real closed-loop FEC job compares recovered
  bits and reports zero bit errors; it does not substitute a chart-only model.

Native desktop interaction used a loaded channel 193 workbench. The Designer
created 50 flat 48 dB captures with −4 dB tilt, 2 dB peak-to-peak ripple at 20 MHz spacing,
and an 18 dB, 10 MHz-wide notch with 1 MHz cosine edges applied only to captures 10–20.
Capture 15 visibly contains the notch; capture 1 does not. The same single shaded
PLC region appears in Designer and main RxMER. Parent replay admission correctly
rejects 50 captures at 1000-symbol intervals in a 1 ms CWB; capacity advice remains
active. Applying Average instead permits the configured three-profile native
run: 4,710,792 compared bits, zero bit errors. This is a synthetic MERLIN
closed-loop check, not external modem interoperability evidence.

Dark/light native inspection and keyboard source selection cover the actual
window. Dark user-guide captures live under `docs/images/workbench/`; ignored
local screenshots, logs and the bounded native driver remain under
`docs/development/local/independent-channel-estimation/` and
`docs/development/local/rxmer-designer/`. No original fixture or source capture
was overwritten. Designer debug logs report stage counts without capture arrays.

## RXMER-DESIGN-02: zoom and PLC visibility

Qt check passes with 74 adapter tests and 64 QML checks, warnings-denied Clippy,
formatting and a current executable. New coverage verifies source-offset cropping,
shared original/designed axes, bounded PLC context, invalid viewport rejection,
Zoom to PLC, retained capture zoom, wheel-equivalent navigation, Home and Full Span.
The scientific core is unchanged; this change uses the Qt validation scope.

On the native example workbench the PLC spans first/last carrier centers at
932.800–933.175 MHz (16 carriers) within a 835.000–1024.975 MHz source grid. The
full-band drawing is approximately one pixel wide. Zoom to PLC frames
932.000–933.975 MHz and reveals one continuous shaded region with endpoint lines,
without inventing a wider PLC. Native Designer and main RxMER views, capture
changes and Home reset were inspected; the main PLC view was also checked in
light mode. Updated full-span and zoomed screenshots
are linked in the user guide.

## RXMER-DESIGN-03: tilt steps

All 65 QML checks pass, including typed negative/fractional values, up/down button
steps, Up/Down keys, bound-disabled controls and invalid/empty drafts. The native
Qt executable is rebuilt. Native dark inspection confirms −5 → −4 → −5 button
steps and immediate graph updates; the guide capture shows the editable field
and both compact controls. Light-mode layout was also inspected. No scientific core or persistence changes were needed.

## RXMER-DESIGN-04: button order

The seven Designer QML checks pass after ordering decrease before increase.
The native Qt application is rebuilt; the guide text and capture reflect the
low-to-high order. No numeric, validation or persistence behavior changes.
