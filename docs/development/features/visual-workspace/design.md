# Qt workbench task design

Status: UI-01 shell implemented; section editors, file import, analyzers and
scientific job integration remain pending.
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

### Qt framework reuse

Inspect PNM Viewer Toolkit read-only for reusable shell, theme, dialogs,
navigation, adapter and build infrastructure. Adapt selected components only
inside MERLIN, preserving the standalone simulator and Rust scientific ownership.
The Qt application incorporating viewer code uses GPL-3.0-only; preserve copied
copyrights, third-party licenses and NOTICE content. Independent scientific
libraries stay Apache-2.0. Record each copied source path/revision and adaptation
here during implementation. The initial reuse record is below.

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

Initial planning was documentation-only. Shell build, QML rendering and test
evidence are now recorded in [validation.md](validation.md); no new scientific
lifecycle support is claimed.

## UI-01 initial increment: workbench shell

Implement the diagram and navigable section dialogs first, with a reusable block
component, scalable connectors and lower-right controls. Controls remain disabled
until real Rust lifecycle integration exists; no simulated progress or results.
Dialogs explain each section and clearly identify pending editors/file pickers.
Use Qt 6.8.3 and the viewer's pinned CXX-Qt 0.7.3 bootstrap in an independent
apps/qt Cargo workspace. Reuse its theme palette; keep scientific crates unchanged.
The VM is the interactive rendering/review target. Verify the native build and
QML navigation tests, then inspect actual rendering. This increment does not
complete UI-01 or UI-04.

### Reuse provenance for the initial shell

Read-only reference HEAD: `5fbd5561f3e06b73486546b5ffa520279080a5eb`.
The selected source files came from the local working tree; these SHA256 values
identify inspected bytes rather than asserting they match that commit.

| Reference path | SHA256 |
| --- | --- |
| `apps/qt/Cargo.toml` | `442a69c8ae281abe71856aba46a796a9d90527fde64ff19ac21dd5e1999a57d7` |
| `apps/qt/build.rs` | `7f7a8ef0a19d26ee2defe71c423c41087ca1259cfcb3b17e53ff62cfbca2b6fd` |
| `apps/qt/src/main.rs` | `9199af55ed2993cccc24280529ffa191e6605c4420863646eba721cb37ca858f` |
| `apps/qt/src/branding.h` | `9d0ae4946c884d0c12f12db9a36f24455b2266d5f21024d43a4df654a58204f3` |
| `apps/qt/qml/Theme.qml` | `035fa925eb5b73a925ad190f56d5523e5ece61ec3821043bb67b93b74e83f680` |
| `apps/qt/qml/ThemePalette.qml` | `a4a694c742bbf2a3894d0df34ea3a87699523538937fbb33ad0b9fc37fd040ad` |
| `scripts/qt-desktop.sh` | `7241e3566a07e5bec3696c109a5b6d5d532f897d07782a5b3dc5c8864cd10958` |

Adaptations: independent Cargo workspace and pinned Qt bridge dependencies;
minimal MERLIN bootstrap/resource module; extracted dark/light palette; unchanged
ThemePalette bindings; renamed embedded-icon wrapper; simplified Apache-2.0
build/run helper with independent Cargo cache. New workbench components own only
layout/navigation. GPL notices and full license are retained under apps/qt.
The native icon wrapper contains Qt calls only, no DSP or simulation code.

The approved light icon now matches the dark design. Dark icon bytes were
preserved. Desktop entry generation is checkout-local; no system installation
or application-menu registration was performed. See [Qt development](../../qt.md)
and the [user guide](../../../user/workbench.md).
