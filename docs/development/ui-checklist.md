# Qt UI checklist

The [architecture](../architecture.md#ui-merlin-simulation-workbench) defines the
MERLIN Simulation Workbench. This checklist expands UI-01 through UI-04 in the
[main checklist](checklist.md); keep their milestone status consistent.
See the [task design](features/visual-workspace/design.md). The initial shell is implemented; unchecked tasks remain pending. Initial targets: Ubuntu 22.04/24.04/26.04 x64.

## UI-01: Workbench and section dialogs

Develop the workbench left to right; OFDM Channel Descriptor (OCD) is the first functional dialog.

- [x] Inspect PNM Viewer Qt infrastructure read-only; select reusable components
  and record source revisions, adaptations, licenses and notices in the design.
- [x] Add the native Qt Quick/QML shell and separate Rust presentation adapter.
- [x] Render the user's high-level signal path and clickable input blocks.
- [x] Open a focused section-preview pop-up for each block; keep the workbench
  visible behind it. Other settings editors and file pickers remain pending.
- [ ] Implement OFDM Channel Descriptor (OCD): cyclic prefix, roll-off/windowing, interleaver
  depth and supported pilot settings, using Rust validation for the next run.
- [x] Replace OFDM preview with an expandable settings tree, supported-value
  dropdowns, field descriptions/source references, and Apply/Cancel behavior.
  Use Parameter/Value columns with descriptions below; keep Modulation Profile
  values blank until selected, without displaying placeholder channel defaults.
- [x] Add a bundled JSON field registry with pinned PHY references, separately
  labeled application defaults, and consistency checks against Rust settings.
- [ ] Add Open | Settings | About navigation and application-preference dialog.
- [ ] Define and implement versioned workbench JSON persistence separately from
  registry metadata; Open restores configuration without starting a run.
- [x] Show heading-only blocks and shared green/yellow/red readiness stripes,
  with status/reason tooltips and accessible descriptions.
- [x] Keep OFDM Channel Descriptor (OCD) yellow while a validated modulation-profile input is
  unavailable; record WB-RULE-01/02 in the architecture and task design.
- [ ] Connect Rust input/job state to automatic block readiness and diagnostics;
  verify accepted/removed/rejected profiles and dependency invalidation.
- [ ] Implement PRBS-TX / Transmitter configuration and stage details.
- [ ] Implement Channel Simulator configuration and stage details.
- [ ] Implement Receiver / PRBS-RX configuration and recovered-data status.
- [ ] Keep source/recovered comparison independent of receiver feedback.
- [ ] Add persistent lower-right Start, Stop and Pause/Resume icons with tooltips,
  accessible names and state-dependent availability.
- [x] Gate Start on every block being green; any yellow/red block disables it.
  Keep the run-integration guard closed until a worker action is connected.
- [ ] Connect Start to a validated frozen request and a bounded Rust worker job.
- [ ] Connect Stop to cooperative cancellation, preserving completed partial results.
- [ ] Define Rust pause boundaries, paused-time budget accounting, Stop/close while
  paused, progress delivery and worker cleanup before implementing Pause/Resume.
- [ ] Implement pause/resume without resetting scientific state or RNG streams.

## UI-02: Input dialogs and source files

Integrate input blocks as their owning sections are developed left to right.

- [ ] Add OS file pickers to applicable pop-up dialogs and show input roles/status.
- [ ] Support PRBS15 or exact text/binary payload bytes with Rust-derived size
  feedback and explicit optional append-PRBS15 fill.
- [ ] Attach modulation-profile inputs to Transmitter.
- [ ] Attach DS channel-estimation and RxMER inputs to Channel Simulator.
- [ ] Validate supported decoded PNM JSON using the existing Rust importer;
  preserve grids, missing samples, timestamps and provenance.
- [ ] Configure multiple RxMER inputs through explicit validated epoch schedules;
  merely listing files must not activate playback.
- [ ] Apply replacements to the next run; preserve active inputs and original bytes.

## UI-03: Observations and analyzers

- [ ] Present actual LDPC outcomes/iterations and independent BER/completion status.
- [ ] Add bounded matched payload pre/post-equalization views.
- [ ] Add separate continuous/scattered pilot and PLC data/preamble views.
- [ ] Reuse existing Rust results and test points; label unsupported observations.
- [ ] Preserve units, reference planes, supplied H versus estimated H-hat,
  missing values and shared retention limits across open panels.

## UI-04: Validation and delivery evidence

- [ ] Verify responsive worker execution, progress and state-dependent controls.
- [ ] Test cancellation, runtime limits, pause/resume, Stop while paused and shutdown.
- [ ] Verify scientific parity with the same Rust request and observation invariance
  when opening/closing dialogs and analyzers, separating host timing from science.
- [ ] Test invalid input, repeated runs, partial results and resource cleanup.
- [ ] Check dark/light readability, keyboard navigation and accessible controls.
- [ ] Validate native builds and runtime separately on Ubuntu 22.04/24.04/26.04 x64.
- [ ] Keep Rust/CLI builds independent of Qt and run applicable Rust and Qt checks.
- [ ] Update user guides with every WB UI/graphic change; use dark-theme
  graphics and actual VM screenshots for implemented visual changes.
- [ ] Reconcile architecture, task design and both checklists with verified behavior.

Python API-01 remains deferred until UI-01 through UI-04 are complete.

Initial shell evidence and remaining validation: [Qt development](qt.md).
