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
- [x] **UI-SHELL-02** Add left-aligned File | Settings | About actions; move
  Light/Dark mode into Settings → Global; remove the instruction banner.
  Settings/About have separate QML files and session-only theme selection.
- [ ] **UI-WB-JSON-01** Implement File Open/Save for JSON containing all PNM inputs
  and accepted settings of every block; define bounds, schema and input encoding.
- [x] **UI-SHELL-03** Use a standard application menu bar above WB branding,
  replacing the content-area navigation buttons.
- [x] **UI-SHELL-05** Add About icon/version/project and offline license links;
  standardize title capitalization and matched rounded light/dark icons.
- [ ] Define and implement versioned workbench JSON persistence separately from
  registry metadata; Open restores configuration without starting a run.
- [x] Show heading-only blocks and shared green/yellow/red readiness stripes,
  with status/reason tooltips and accessible descriptions.
- [x] Keep OFDM Channel Descriptor (OCD) yellow while a validated modulation-profile input is
  unavailable; record WB-RULE-01/02 in the architecture and task design.
- [x] Connect accepted/removed/rejected DPD input to Rust-backed DPD/OCD readiness.
- [ ] Connect remaining input/job states and dependency invalidation to readiness.
- [x] Implement PRBS-TX configuration and supported Transmitter preview readiness.
  Full-loop Transmitter/job wiring remains pending. Source options are defined in the
  [architecture](../architecture.md#prbs-tx-source-configuration).
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

- [x] Add the DPD Open File dialog, content detection, binary/JSON import and
  staged profile selection/removal with exact-byte provenance.
- [x] Enforce and verify OS-native Open for PRBS-TX and DPD without Qt fallback.
  Save/folder selection remains pending with its owning future dialogs.
- [ ] Add OS file pickers to remaining applicable dialogs and show input roles/status.
- [x] Support PRBS15 or exact text/binary payload bytes with Rust-derived size
  feedback and explicit optional append-PRBS15 fill.
  - [x] Add a separate Transmit source (PRBS-TX) tree dialog with descriptions,
    staged Apply/Cancel and conditional PRBS15/File controls.
  - [x] Expose continuous PRBS15 seed 1–32767, initially 32767; no other polynomial.
  - [x] Read bounded exact-byte files through the OS-native picker; report Rust
    byte/bit counts and SHA-256 without serializing payload bytes to QML.
  - [x] Add opt-in finite fill, explicit total meaningful bits and independent
    fill seed; reuse core limits, including empty/equal-target/partial-byte cases.
  - [x] Drive PRBS-TX readiness from accepted Rust configuration; rejected drafts
    and Cancel preserve accepted state. Apply does not start or mutate a run.
  - [x] Verify UI lifecycle/native picker and adapter validation with existing
    PRBS/file tests reused; document actual dark VM screenshots.
- [x] Accept modulation-profile capture inputs in DPD and show the OCD summary.
- [ ] Attach accepted DPD profiles to the actual Transmitter run request.
- [x] Attach DS channel-estimation and RxMER configuration to Channel Simulator
  readiness; full-loop execution wiring remains pending.
- [x] Validate binary PNM through the pinned decoder and standalone decoded
  JSON through the shared Rust importers;
  preserve grids, missing samples, timestamps and provenance.
- [ ] Dispatch accepted multiple-RxMER configuration through validated full-run
  schedules; merely listing files must not activate playback.
- [ ] Apply replacements to the next run; preserve active inputs and original bytes.

## UI-03: Observations and analyzers

- [x] **UI-TX-01** Derive green Transmitter readiness from accepted OCD/DPD/source
  through the supported Rust request; invalidate/cancel on applied input changes.
- [x] **UI-TX-02** Reuse a generic SA dialog for actual TX-output samples, with
  centered baseband default, optional capture mapping, normalized Hann PSD,
  bounded min/max viewports, zoom, progress and cooperative cancellation.
  Design: [Transmitter spectrum](features/transmitter-spectrum/design.md).
- [x] **UI-SA-03** Clear the SA lower text panel and reserve blank space.
- [x] **UI-SA-04** Fit symbol/window timing and nominal cable echo budgets below
  the PSD, using frozen result settings and Rust calculations.
- [ ] Add actual IQ sample trace inspection, distinct from the window-coefficient view.
- [ ] Connect the shared SA to channel output and eligible RX time-sample taps.

- [ ] Present actual LDPC outcomes/iterations and independent BER/completion status.
- [ ] Add bounded matched payload pre/post-equalization views.
- [ ] Add separate continuous/scattered pilot and PLC data/preamble views.
- [ ] Reuse existing Rust results and test points; label unsupported observations.
- [ ] Preserve units, reference planes, supplied H versus estimated H-hat,
  missing values and shared retention limits across open panels.

## Receiver workbench navigation and instruments

- [x] **UI-RX-WB-01** Replace the Receiver preview with its own workbench: processing
  stages, clickable test-point markers and instrument navigation; soft demapping
  and optional hard slicing remain separate branches.
- [x] **UI-RX-WB-02** Align equal-width stage/instrument chips and annotation rows;
  use rounded edges, subtle chip pins and visible keyboard/hover feedback.
- [x] **UI-QAM-01 selectors** Add Data Profile / NCP / PLC and Raw / Equalized /
  Paired controls, separate PLC data/preamble and unavailable received-profile state.
- [ ] **UI-QAM-01 data** Connect supported received-role collectors and actual
  profile/carrier/symbol controls; preserve explicit unsupported cases.
- [ ] **UI-QAM-02** Add bounded matched received NCP I/Q classification/collection
  before enabling its constellation view; LLR/control events are not I/Q.
- [x] **UI-SA-05 selector** Add independent SA Test Point routing with empty-state
  admission and a distinct FFT-bin-power label; disable receiver generation.
- [ ] **UI-SA-05 data** Connect retained RX time samples and FFT-bin power with
  correct units, boundary handling and normalization.
- [ ] Verify selection does not rerun science, hides stale results, preserves
  matched run/carrier/time identities and shares bounded retention across panels.

Design: [Receiver Workbench](features/receiver-workbench/design.md).
Navigation is implemented; receiver chart wiring remains pending.

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

## Proposed RxMER editor and shell follow-up

- [x] **UI-SHELL-04** Use the requested main WB heading and conventional File labels.
- [x] **UI-RXMER-01** Stage single/multiple RxMER inputs with exact DPD association.
- [x] **UI-RXMER-02** Add static/average/hold/interpolated-power configuration and
  bounded previews with strict required-carrier validity.
- [x] **UI-RXMER-03 editor** Expose symbol intervals/order/end policy and previews.
- [ ] **UI-RXMER-03 dispatch** Integrate accepted configuration and interpolation
  into full channel jobs, including job-duration admission and continuous RNG.

See the [design and remaining scope](features/rxmer-workbench/design.md).

- [ ] **UI-QAM-01** Reuse paired RX FFT/equalized constellation observations before
  decisions, with carrier/modulation/time selection and explicit SA reference planes.

- [x] **UI-CHANNEL-01** Require valid RxMER and response configurations for green
  Channel Simulator; provide explicit Flat/unity H=1+0i default on the DPD grid.
