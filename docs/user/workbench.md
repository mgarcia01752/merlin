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
