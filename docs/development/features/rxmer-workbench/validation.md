# Shared measurement viewer validation

Validated locally on the Ubuntu 22.04 x64 VM with Qt 6.8.3. Ubuntu 24.04/26.04
and installed-package lifecycle checks were not run for this change.

## Targeted checks

- Qt warnings-denied Clippy, formatting, 11 Rust adapter tests and 22 QML tests
  passed. Coverage includes shared dialogs, exact profile association, binary
  RxMER and decoded response imports, atomic drafts, readiness, modes, bounded
  chart requests, About branding and the bundled offline license dialog.
- Three targeted core tests passed: standalone measurement import/chart checks
  and two independent playback tests. Playback checks use a 20/40 dB example
  with expected mean error power 0.00505, hold/interpolation boundaries, final
  hold, immutable epochs, incompatible inputs and application resource limits.
- The standalone import/chart test also passed with default features disabled
  and only `pnm-input` enabled. An existing unused `SymbolPacker.coded_bits`
  warning appears in that feature combination; no suppression was added.
- Core library warnings-denied Clippy and workspace formatting passed.
- The full DSP/FEC suite was intentionally not rerun; this change used scoped
  checks and retained Cargo build artifacts.

## Native presentation

The actual dark Qt workbench was reviewed on the VM. User-guide screenshots
record the shared response viewer, synthetic flat response, binary RxMER,
interpolated-power preview, accepted-input readiness and About.
The Ubuntu GTK native multi-file chooser was exercised through Open file(s)…:
selecting baseline and stress binary captures in one operation produced two
distinct source rows. This used the X11/XWayland display shared with Qt.
The GPL document opens from About without network access. The compiled version
is shown, and About/sidebar consume the active rounded application icon.
Both light/dark PNGs have identical alpha masks and transparent corners, rendered
from SVG sources sharing the same border-following clip. Interior artwork is
preserved. Light-theme About/sidebar were also reviewed; public documentation
remains dark. The workbench and populated RxMER editor fit the 960×640 minimum
window, with controls and chart visible. No desktop installation or global icon-cache mutation was performed.

## Boundaries

This is input configuration and Rust combination/preview integration. Accepted
RxMER/response are not yet dispatched through a full-loop UI run. Receiver QAM,
other SA taps, complete receiver/comparison readiness, run duration admission and
continuous interpolation dispatch remain pending. TX SA is still before channel
impairments. Workbench JSON persistence remains pending.

Parser evidence remains the pinned PNM decoder plus existing decoded examples;
this work does not claim newly rendered specification-PDF verification. Binary
RxMER and JSON response are directly exercised; broader real binary response
capture validation remains future evidence.

Logs are local-only under `docs/development/local/rxmer-workbench/`; public user
screenshots are linked in the [user guide](../../../user/channel-measurements.md).
