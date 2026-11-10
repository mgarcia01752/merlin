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

## OFDM editor and planned navigation

The OCD block opens the Parameter/Value tree editor described below. Selecting
a field shows its description, units, supported choices, application default and
pinned specification source. Apply validates staged changes in Rust and stores
them in memory; Cancel leaves the accepted configuration unchanged.

Use `resources/registry/ofdm-settings.json` for the bundled field-description
registry. Follow the OSCAR registry concept, not its TLV wire semantics. Entries
carry stable IDs, groups, labels, choices with display units, descriptions,
application defaults where applicable and versioned section/table provenance. Mark
paraphrases explicitly. Do not invent MULPI provenance or normative defaults.
Rust validation remains authoritative; verify registry choices against Rust.

Propose separate versioned `*.merlin.json` files for workbench values and input
references. Registry metadata is not duplicated in saved workbenches. File
persistence and registry loading belong in Rust; QML owns editing presentation.

Top-level Open selects a workbench; Settings opens application preferences
(including theme); About shows the expanded MERLIN name, version, licensing and
acknowledgements. Input capture selection stays with the owning diagram block.
Application navigation and workbench persistence remain unimplemented.

## UI-01 implementation: transmission editor

Implement three registry-driven dropdown fields with expandable groups and a
selected-field description. A Qt Rust object owns accepted TransmissionSettings;
QML holds only a draft. Apply deserializes and validates using the existing core
model before replacing accepted state. Rejection preserves previous state. Escape
and Cancel discard the draft. Reopening reads accepted values. The adapter also exposes core timing; the current tree does not display it. Link the core without simulation features; no DSP changes.
Test invalid CP/RP, spacing-dependent depth, malformed input, rejection atomicity,
registry choices/default consistency, and QML reopen/cancel interactions. Capture
the native dark popup on the VM and update the user guide. Persistence, other
editors and execution remain pending.

Implemented editor boundary: `OfdmSettingsDialog.qml` owns transient draft
presentation; `src/ofdm_settings.rs` owns accepted in-memory settings and delegates
to core validation/timing. `SectionPreviewDialog.qml` owns remaining generic
previews. The registry is embedded by Rust with `include_str!`; no runtime source
checkout or registry filesystem access is required. Other navigation, persistence,
carrier/pilot editors and job integration remain pending.

## UI-01 revision: selected tree layout

Replace the side-by-side form with the approved Parameter/Value tree layout and
a selected-field description below. Edit cyclic prefix, roll-off and interleaver
depth only. Replace the invented Channel context branch with Modulation Profile;
its Profile ID, Source file and Subcarrier assignments values remain blank until
profile selection is implemented. Do not derive a selected profile from core
application defaults. The accepted core configuration retains bandwidth/spacing
internally; profile loading must reconcile channel settings separately.

Pinned terminology: PHY I20 §7.5.13.4 distinguishes OFDM Channel Descriptor and
Downstream Profile Descriptor. MULPI I25 §6.4.40 (PDF 197–199) describes static
channel parameters common to profiles; §6.4.41 (PDF 199 onward) describes downstream
profiles and subcarrier assignments. Evidence is a local text extraction under
ignored docs/development/local/plc-completion/, absent from fresh clones; no
rendered-PDF verification is claimed. A modulation profile does not itself define
the sample clock/FFT. Blank profile rows are presentation of unavailable input,
not a valid channel configuration or evidence of profile-import support.

## UI-01 revision: block labels and readiness stripes

Remove secondary text from every diagram block. Preserve the heading, centered
within the box, and the left stripe. A common presentation component maps complete
(green), attention (yellow), and error (red) to palette colors. Require an explicit
status and reason for each block; expose both through tooltip/accessibility.

The architecture owns the readiness rules. In this preview all blocks need
attention because their inputs, run integration or results are unavailable.
OFDM Settings specifically requires valid accepted settings AND an accepted,
compatible modulation-profile PNM input. Its current values alone cannot make it
green. Profile import is pending, so OFDM remains yellow even after Apply.
Scientific readiness and error classification must come from Rust when connected;
QML renders status and must not infer successful import from a selected filename.
An invalid draft keeps the previous accepted state and shows its error locally.
No new synthetic errors or implicit requirements for optional inputs are added.

Verify common stripe colors/reasons, initial yellow status, unchanged yellow
OFDM status after Apply, native rendering and current block interactions. Use
Qt-only checks; update the dark native overview and dialog screenshots.

## UI-01 revision: Start readiness gate

WB-RULE-03 requires every diagram block to be green before Start can be enabled.
Aggregate the shared block statuses reactively; any yellow/red block closes the
gate. This is a presentation prerequisite, not a substitute for Rust validating
and freezing a run request. Keep Start disabled while the run action is not
connected, even if a presentation test marks every block complete. Before a run,
green means ready/configured, including the PRBS-RX comparison setup; it must not
require results from that run. Explicit valid optional-input bypass/default modes
can satisfy their block without a measurement file. Test each block independently
blocking the gate, recovery to all-green, and the pending-integration guard.

## UI-01 naming: OFDM Channel Descriptor (OCD)

The user selected the specification's full OFDM Channel Descriptor (OCD) name
for the block and popup. Wrap the heading across lines to fit the diagram.
Keep stable internal IDs and source filenames to avoid unrelated renames. This
is an editor label, not a claim that the dialog imports/exports complete OCD
messages; the modulation-profile summary remains separate from OCD semantics.
