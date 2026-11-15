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

## RXMER-DESIGN-05: Flip action

All 66 QML checks pass. The Flip case covers −5 ↔ +5 dB, fractional magnitude,
limit values, disabled zero/invalid values, keyboard activation and retained
zoom. The native Qt executable is rebuilt; the guide explains the visible Flip
button and signed slope direction. The scientific recipe and persistence format
are unchanged. Native inspection confirms the positive and negative slopes
reverse when Flip is clicked.

## RXMER-DESIGN-06: inactive tilt controls

All 67 QML checks pass. The new interaction test covers initial disabled state,
mouse toggling, retained tilt and capture range, omitted/restored preview recipe,
blocked stepping and independent sibling effects. The native Qt build passes;
native dark inspection confirms the unchecked controls are dimmed and disabled,
and checking tilt restores the controls. The guide capture is updated.

## RXMER-DESIGN-07: direction button

All 67 QML checks and the native Qt build pass. Direction tests verify both
arrow labels follow the signed value, mouse and keyboard reversal retain
magnitude and zoom, and inactive/zero/invalid drafts remain guarded. Native
dark inspection confirms Low ← High at −4 dB and Low → High at +4 dB,
with the corresponding preview slopes. The guide capture is updated.

## RXMER-DESIGN-11: binary and JSON save

Validated locally on Ubuntu 22.04 x86_64, Qt 6.8.3 and CPython 3.10. PNM field
provenance was recorded under ignored
`docs/development/local/rxmer-pnm-export/field-evidence.md`: clear I27 text and
pinned Apache decoder evidence, without a rendered-PDF claim.

- `scripts/test-local.sh all`: 407 workspace + 4 preparation-example + 136
  analysis-only tests passed; no skips. Total including compilation/doctests:
  **12m 44s**. The new exporter has four cases, including independent literal
  header/payload anchors, 25/50 kHz pinned-reader roundtrips, half-step/range/null
  rules, invalid/nonfinite inputs, original ownership, manifests, no-overwrite,
  cancellation, all-or-nothing collection publication and full-precision JSON.
  After final early size-admission hardening, all four exporter cases and
  workspace Clippy were rerun successfully; native UI and wheel were rebuilt.
- Workspace formatting/Clippy passed. Full `pnm-input` tests separately passed
  21 cases in 63.05s including compilation; binding formatting/Clippy passed.
- `scripts/qt-desktop.sh check`: 82 Rust adapter tests passed (125.01s), native
  build passed and all 76 QML checks passed (26.286s). Focused Designer suite
  passed 13 checks, including Rust-authoritative binary admission, JSON while
  binary is blocked, capture range, stale preflight and unapplied draft state.
  A final native build includes equal-width save actions; the latest executable
  is available for VM review.
- Local native Linux cp310 wheel built with notices/checksum and was installed
  only in the project test environment. Eight lifecycle/API tests, one export
  test and parity across Each/Individual/AverageAll/InterpolateAt passed.
  Parity uses the preserved Rust policy baseline; engine calculations did not
  change. Export tests independently assert payload bytes and structured guards.
- Native VM dark/light inspection confirmed the popup and controls are readable.
  Actual JSON and binary single-file saves created separate manifests without
  applying the draft. Native All captures saved two complete 7600-value binaries
  in a new folder. Installed `pnm-decode 0.3.6` decoded all generated binaries;
  single JSON/binary identity, time, grid and samples matched exactly for the
  quarter-dB fixture. Dark screenshots were updated. Review processes used
  isolated configuration/data and were closed without accepting inputs.

No Windows native compilation/runtime, MSI lifecycle, macOS or other Python ABI
was tested. Collection publication returns unsupported outside Linux/Windows.
The passing local checks do not establish modem interoperability or DOCSIS 4.0
file-format equivalence. Nothing was committed, published or installed system-wide.

## RXMER-DESIGN-12: native-font clipping regression

[v0.3.0 release run](https://github.com/mgarcia01752/merlin/actions/runs/43696778449)
failed the Windows notch width mouse step. Local Qt 6.8.3 reproduction with an
18 px dialog font likewise left width at 10: its arrow was outside the clipped
400 px editor. The responsive grid now selects one or two groups per row from
actual available width and native font metrics. Its parent column uses the
scroll view's available width directly, avoiding implicit-width feedback.

The final `scripts/qt-desktop.sh check` passes formatting, warnings-denied
Clippy, 82 Rust adapter tests (54.49 s), native build and 77 QML checks (13.27 s).
Designer-only coverage passes all 14 cases, including both column counts,
large-font/narrow-editor clip bounds and mouse stepping for center, width,
depth and edge. Scientific calculations and all disabled/input guards are unchanged.
A rebuilt native VM window also accepted width 10 → 11 → 10 using the mouse.
Refreshed full-span and PLC-zoom dark screenshots show actual imported captures,
−4 dB tilt, 2 dB ripple and an 18 dB/10 MHz notch. Originals were not overwritten;
the review process closed without applying or saving the draft. Local logs are
ignored under `docs/development/local/release-monitor/`. Hosted Windows validation
requires the pending patch release; Linux success is not Windows verification.
