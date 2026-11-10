# Qt workbench task design

Status: discussion decisions recorded; no Qt implementation yet.
The [UI architecture](../../../architecture.md#ui-merlin-simulation-workbench)
owns the layout and signal-flow diagrams. The [UI checklist](../../ui-checklist.md)
expands the main milestones. The [earlier plan](plan.md) retains detailed scientific
input and observation boundaries.

## UI-01: Shell, dialogs and lifecycle

Use the name MERLIN Simulation Workbench. Implement native Qt Quick/QML with a
separate Rust adapter, proposed under apps/qt/, consuming the existing public
scientific APIs. Keep Qt dependencies out of the scientific libraries and all
DSP, modulation, FEC and derived scientific values in Rust.

Follow the user's diagram left to right: OFDM Settings first, then PRBS-TX /
Transmitter and profile input, Channel Simulator and measurement inputs, and
Receiver / PRBS-RX. Each block opens a focused pop-up for editing its section,
selecting an applicable input or viewing status. Keep the path visible behind it.
The PRBS comparison is independent verification, not a feedback link.

Place persistent Start, Stop and Pause/Resume icons in the lower-right corner.
Adapt TransmissionSettings, ChannelRunSettings, ChannelJobLimits, run_channel,
ChannelProgress and ChannelJobResult rather than recreating orchestration or
calculations in QML. Freeze requests at Start, run away from the UI thread,
and deliver bounded progress/results. Stop preserves completed partial results.
Borrowed probe events must be copied within their callback into bounded owned
snapshots before delivery to the UI; never retain their borrowed buffers.

Pause is not implemented by the existing job API. Design its safe step boundary,
paused-time treatment in the host-runtime budget, Stop/close handling and worker
cleanup before coding it. Resume must continue the same scientific state, not
restart or reseed. Settings edits apply only to the next run.

## UI-02: Input ownership

Use OS pickers for applicable files. Payloads are exact bytes with explicit fill;
measurement files initially use supported decoded PNM JSON. Reuse source and
scenario validation, including bounds, provenance, grids and explicit epoch
schedules. File replacement must not mutate an active job. Rust supplies size,
fill and validation feedback; QML presents it.

## UI-03: Test points and results

Expose existing bounded Rust observations through separate LDPC, BER, payload
pre/post-CE, continuous/scattered pilot and PLC data/preamble views. Preserve
units, reference planes, missing values and honest partial outcomes. Distinguish
supplied-H correction from pilot-derived H-hat. An open panel must not change
RNG draws, source bits or decoding decisions. Share observation retention budgets.

## UI-04: Acceptance

Initial native targets are Ubuntu 22.04, 24.04 and 26.04 x64. Verify navigation,
input rejection, Rust/UI scientific result parity, responsive progress, cancel,
pause/resume, runtime accounting, shutdown and repeated-run cleanup. Test
observation invariance independently of host timing. Check dark/light readability
and accessible controls; capture actual screenshots after implementation.
Keep platform validation and UI milestones unchecked until evidence exists.

Documentation-only validation for this record: check local links and whitespace.
No native build, Qt test, screenshot or new lifecycle support is claimed.
