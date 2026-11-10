# MERLIN Simulation Workbench

The native Qt workbench starts with the single-channel system diagram. Click
OFDM Channel Descriptor (OCD), PRBS-TX, Transmitter, Channel Simulator, Receiver, PRBS-RX or one
of the measurement-input blocks to open its section dialog. Close the dialog
or press Escape to return. Light mode/Dark mode changes the palette and icon.

The workbench includes an editable **OFDM Channel Descriptor (OCD)** dialog. Other blocks open
section previews; file pickers and simulation jobs are not connected yet.
The lower-right Start, Stop and Pause controls are disabled. There are no UI
simulation results or automatically loaded measurements.

Development proceeds left to right from OFDM Channel Descriptor (OCD). The PRBS-TX/PRBS-RX
connection means original/recovered data comparison, not receiver feedback.
Measurement input blocks will use supported decoded PNM JSON. The existing
[channel CLI](channel-runner/cli.md) remains available for actual simulations.

See [Qt setup and VM launch](../development/qt.md) and the
[UI checklist](../development/ui-checklist.md). The interface remains in the dev
phase; initial Ubuntu targets require separate native validation.

## Block status

Blocks show their name only. Their left stripe indicates status:

| Color | Meaning |
| --- | --- |
| Green | Complete: the block's requirements are satisfied. |
| Yellow | Needs attention: input, configuration or results are missing/pending. |
| Red | Error: an input, configuration or processing failure has a diagnostic. |

Hover over a block, or focus it with the keyboard, to see its status and reason.
Screen readers receive the same explanation. Click the block for its dialog.

![Heading-only blocks with yellow attention stripes](../images/workbench/workbench-status-dark.png)

**OFDM Channel Descriptor (OCD) stays yellow until a compatible modulation-profile PNM input has
been accepted and its settings are valid.** Applying timing settings alone does
not complete it. Profile import remains pending, so this requirement cannot yet
be satisfied in the UI. Missing input is yellow; an actual validation failure
will be red when import is connected. An invalid draft remains in its editor
with an error, preserving the last accepted configuration.

**Start requires every block to be green.** A yellow or red block disables it.
Green before a run means configured and ready; comparison results are not needed
to start the run. Start also remains disabled while simulation integration is
pending. Optional inputs may satisfy readiness through an explicit valid
bypass/default mode when those controls are implemented.

All current preview blocks are yellow because integration or inputs are pending. Optional measurement inputs will not be required for every future run.
Green/red presentation is available; automatic input/run-driven status changes
remain pending.

## Edit OFDM transmission settings

Click **OFDM Channel Descriptor (OCD)** in the diagram to open its Parameter/Value tree. Expand
or collapse **Symbol timing**, **Interleaving** and **Modulation Profile**. Edit
cyclic prefix, roll-off period and time-interleaver depth in the Value column.
Select a row or focus its dropdown to read its description and specification
reference below the tree.

![OFDM Channel Descriptor (OCD) in the native dark workbench](../images/workbench/ofdm-settings-dark.png)

The **Modulation Profile** branch shows Profile ID, Source file and Subcarrier
assignments. Their values stay blank when no profile is selected. Profile input
is not connected yet, so the application starts with all three blank. Application
defaults do not imply that a profile has been selected.

DOCSIS uses **OFDM Channel Descriptor (OCD)** for common channel parameters and
**Downstream Profile Descriptor (DPD)** for profile parameters, including modulation
assignments. They are separate concepts (MULPI 3.1 I25 §§6.4.40–6.4.41; PHY 3.1
I20 §7.5.13.4). The previous Channel context label has been removed. Bandwidth and
spacing remain in the core configuration but are not editable in this dialog;
channel/profile import integration remains pending. This dialog alone does not
configure a complete transmission or import/export a complete OCD message.

**Apply** validates the draft in Rust and retains it in this application session.
Invalid combinations remain open with an error and do not replace the last
accepted configuration. Roll-off must be shorter than the prefix. Interleaver
depth must be at most 16 for 25 kHz spacing or 32 for 50 kHz spacing. Zero roll-off
is evaluation-only. The interleaver-depth limit uses the channel spacing held by
Rust. Channel settings will need to be reconciled when profile input is connected.

**Cancel** or **Escape** discards edits. Reopening restores the last applied
values. **Reset defaults** changes the draft to MERLIN's application defaults;
press Apply to accept them. These defaults are not mandated by the specification.
See [transmission settings](transmission.md) for the supported values and units.

Settings are held in memory only and reset when the application closes. Run
integration and workbench-file saving remain pending. The bundled
`resources/registry/ofdm-settings.json` describes the fields; the proposed
`*.merlin.json` workbench format will separately store values and input references.

## Planned application navigation

- **Open:** restore a saved workbench configuration without starting a run.
- **Settings:** change application preferences such as theme.
- **About:** view MERLIN's expanded name, version, licenses and acknowledgements.

These actions are planned. Measurement-file selection will remain in its owning
workbench block. User-guide screenshots and graphics use the dark theme;
light-theme support remains available in the application.
