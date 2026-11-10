# MERLIN Simulation Workbench

The native Qt workbench starts with the single-channel system diagram. Click
OFDM Settings, PRBS-TX, Transmitter, Channel Simulator, Receiver, PRBS-RX or one
of the measurement-input blocks to open its section dialog. Close the dialog
or press Escape to return. Light mode/Dark mode changes the palette and icon.

This first increment is a **workbench preview**. Dialogs explain their section;
settings editors, file pickers and simulation jobs are not connected yet.
The lower-right Start, Stop and Pause controls are disabled. There are no UI
simulation results or automatically loaded measurements.

Development proceeds left to right from OFDM Settings. The PRBS-TX/PRBS-RX
connection means original/recovered data comparison, not receiver feedback.
Measurement input blocks will use supported decoded PNM JSON. The existing
[channel CLI](channel-runner/cli.md) remains available for actual simulations.

See [Qt setup and VM launch](../development/qt.md) and the
[UI checklist](../development/ui-checklist.md). The interface remains in the dev
phase; initial Ubuntu targets require separate native validation.

## Planned OFDM Settings editor

The next increment will replace the OFDM Settings preview with an editable
pop-up. Expand Spectrum for bandwidth and subcarrier spacing, Timing for cyclic
prefix and roll-off, and Interleaving for interleaver depth. Dropdowns will offer
supported values. Selecting a field will show its description, units, suggested
application default and PHY specification reference. Sample rate, FFT size and
symbol duration will be read-only values calculated by Rust.

Apply will validate changes for the next run; Cancel will discard edits.
The proposed JSON registry describes available fields, while a separate
`*.merlin.json` workbench file will store selected values and input references.
Neither the editor nor workbench persistence is implemented yet.

## Planned application navigation

- **Open:** restore a saved workbench configuration without starting a run.
- **Settings:** change application preferences such as theme.
- **About:** view MERLIN's expanded name, version, licenses and acknowledgements.

These actions are planned. Measurement-file selection will remain in its owning
workbench block. User-guide screenshots and graphics use the dark theme;
light-theme support remains available in the application.
