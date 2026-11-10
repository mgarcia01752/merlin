# Receiver Workbench validation

Scope: UI-RX-WB-01 navigation, UI-QAM-01 selectors and UI-SA-05 selector/routing.
Actual receiver waveform/result dispatch, charts and matched NCP I/Q collection
remain pending. This is not evidence of a complete receiver analyzer.

## Automated checks

Qt 6.8.3 on Ubuntu 22.04 x64:

- `scripts/qt-desktop.sh check`: formatting, warnings-denied Clippy, 11 adapter
  tests, native executable build and 23 QML checks passed.
- The added receiver interaction test opens NCP, switches to PLC and Paired,
  opens RX Input SA, switches to FFT bins, verifies a stale viewport clears,
  verifies SA selection preserves the QAM signal, reopens Data Profile, and
  checks that received profiles/generation remain disabled and Receiver yellow.
- The same suite retains existing TX SA, measurement/editor, theme, About and
  Start-gating checks. No DSP/FEC source changed and no full non-Qt suite ran.
- A final QML-only run passed all 23 checks without warnings after completing
  the synthetic stale-viewport test record with its accessibility readout. The
  nested-popup test waits for the closing analyzer to render before clicking
  the parent test point; it also exercises actual combo popup clicks.

Local-only logs: `docs/development/local/receiver-workbench/`. The final Qt
executable is built using retained Cargo artifacts; no cleanup or installation
was performed. Other Ubuntu versions and package lifecycle remain unverified.

## Native review

The user guide contains actual dark VM screenshots of the receiver navigation,
NCP paired selection and SA Test Point selection. Empty states are intentional:
no received constellation points, spectral samples or received profile metadata
have been connected to Qt. The Rust adapter maps SA choices to core registered
IDs; QML does not calculate observations or substitute TX data. The actual native
review verified PLC signal switching outside the smaller receiver dialog,
returning to the workspace and opening RX Input SA. The workspace fits 960×640;
light-theme readability was checked, then dark mode restored. The running binary
hash matches the final compiled artifact, and its native log is empty.

## Remaining integration

Full-loop jobs must expose immutable bounded result sessions, RX time samples,
matching raw/equalized signal-role observations, received profile/carrier/time
metadata and correction mode. NCP needs its own received-role matched I/Q
collector. FFT-bin power needs its own frequency-domain normalization; useful
sample spectra must respect discontinuities. These data tasks stay unchecked.
Response/echo, slicer/LLR and FEC actions currently open explanatory instrument
panels, not implemented result renderers.

## UI-RX-WB-02 presentation follow-up

The shared rounded receiver chips preserve equal stage widths and aligned
annotation slots. Native dark review at normal size and 960×640 found no clipping;
QAM/instrument rows also use equal widths. The stage border pins are decorative.
The existing 23 QML checks passed again after styling, and the Qt executable was
rebuilt with retained artifacts. User screenshots were refreshed; no DSP/FEC
source or numerical tests changed for this styling follow-up.
