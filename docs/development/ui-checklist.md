# Qt UI checklist

The [architecture](../architecture.md#ui-merlin-simulation-workbench) defines the
MERLIN Simulation Workbench. This checklist expands UI-01 through UI-04 in the
[main checklist](checklist.md); keep their milestone status consistent.
See the [task design](features/visual-workspace/design.md). The initial shell is implemented; unchecked tasks remain pending. Initial targets: Ubuntu 22.04/24.04/26.04 x64.

## UI-01: Workbench and section dialogs

- [x] **UI-COPY-01** Use user-focused text and concise essential notes; remove
  programming-language/framework commentary from normal UI.
  [Design](features/ui-copy/design.md).

- [x] **UI-EXAMPLES-01** Curate single/multi-RxMER WB bundles with compatible
  originals, hashes and notices. [Design](features/workbench-examples/design.md).
- [x] **UI-EXAMPLES-02** Add channel 193 `multi-rxmer-2`: 132 original RxMER
  files, companion DPD, flat response and bounded 256-file support.
  [Design](features/workbench-examples/design.md#ui-examples-02-channel-193-measured-rxmer-collection).
- [x] **UI-OPEN-01** Default native Open to relocatable example resources.
  [Design](features/workbench-examples/design.md).
- [x] **UI-WB-OPEN-01** Filter main Open / Ctrl+O to `*.merlin.json` while
  retaining folders and independent input pickers; verify native VM behavior.
  [Design](features/qt-closed-loop/design.md#ui-wb-open-01-main-open-file-filter).
- [ ] **INSTALL-EXAMPLES-01** Ship and validate the runnable inventory in future
  native desktop installers, including installed-OS Open/restore behavior.
- [ ] **CH-EST-PLAYBACK-01** Define varying channel-estimate collections before
  adding a runnable multi-RxMER/multi-channel-estimate WB.

- [x] **UI-RXMER-04** Remove RxMER demo/notch buttons and use Open file(s).
  [Design](features/rxmer-workbench/design.md#ui-rxmer-04-remove-demo-actions-from-rxmer).

- [x] **UI-SHELL-11** Remove the left icon/WB gutter and use equal side margins.
  [Design](features/verification-panel/design.md).
- [x] **UI-VERIFY-02** Restyle PRBS-RX with comparison cards, payload recovery
  and retained word-error charts. [Design](features/verification-panel/design.md).

- [x] **UI-SHELL-10** Center diagram connections, refine the two-line branded
  header and show PNM type footnotes inside input blocks.
  [Design](features/visual-workspace/design.md#ui-shell-10-centered-connections-and-in-card-pnm-types).

- [x] **UI-SHELL-09** Group OCD/DPD under Transmitter Configuration and match
  the revised diagram/header styling. [Design](features/visual-workspace/design.md#ui-shell-09-group-transmitter-configuration).

- [x] **UI-ZOOM-01** Replace all zoom buttons with shared mouse wheel/range
  navigation and retain Full Span. [Design](features/mouse-zoom/design.md).

- [x] **UI-SHELL-07** Place RxMER above Channel Simulator and split DS Channel
  Estimate into channel/receiver inputs; remove the top comparison connector.
  [Design](features/visual-workspace/design.md#ui-shell-07-measurement-input-placement).

Develop the workbench left to right; OFDM Channel Descriptor (OCD) is the first functional dialog.

- [x] Inspect PNM Viewer Qt infrastructure read-only; select reusable components
  and record source revisions, adaptations, licenses and notices in the design.
- [x] Add the native Qt Quick/QML shell and separate Rust presentation adapter.
- [x] Render the user's high-level signal path and clickable input blocks.
- [x] Open a focused section-preview pop-up for each block; keep the workbench
  visible behind it; accepted source/profile/measurement editors are available.
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
  Settings/About have separate QML files; explicit WB saves include the theme.
- [x] **UI-WB-JSON-01** Implement File Open/Save for JSON containing all PNM inputs
  and accepted settings of every block; define bounds, schema and input encoding.
- [x] **UI-SHELL-03** Use a standard application menu bar above WB branding,
  replacing the content-area navigation buttons.
- [x] **UI-SHELL-06** Add a shared circular X to all popup titles, retaining
  footer buttons, draft cancellation and nested-popup behavior.
  [Design](features/popup-close-controls/design.md).
- [x] **UI-SHELL-05** Add About icon/version/project and offline license links;
  standardize title capitalization and matched rounded light/dark icons.
- [x] Define and implement versioned workbench JSON persistence separately from
  registry metadata; Open restores configuration without starting a run.
- [x] Show heading-only blocks and shared green/yellow/red readiness stripes,
  with status/reason tooltips and accessible descriptions.
- [x] Keep OFDM Channel Descriptor (OCD) yellow while a validated modulation-profile input is
  unavailable; record WB-RULE-01/02 in the architecture and task design.
- [x] Connect accepted/removed/rejected DPD input to Rust-backed DPD/OCD readiness.
- [x] **UI-DPD-VIEW-01** Add View Profile with role colors, bounded bit-loading
  plot, mouse zoom and Full Span; hide editor filenames and preserve originals.
  [Design](features/pnm-measurement-inputs/profile-view.md).
- [x] Connect accepted input/job states and dependency invalidation to readiness.
- [x] Implement PRBS-TX configuration and supported Transmitter preview readiness.
  Full-loop Transmitter/job wiring uses the shared Rust engine. Source options are defined in the
  [architecture](../architecture.md#prbs-tx-source-configuration).
- [x] **UI-CH-01** Implement Channel Input/Model/Output staging with direct
  RxMER/DS Channel Estimate frames and Spectrum/QAM chooser at each boundary.
  Separate additional-loss editor supplies manual tilt and full cable loss.
  [Design](features/channel-workbench/design.md).
- [x] **UI-RX-CONFIG-01 / UI-VERIFY-01** Implement Receiver / PRBS-RX configuration
  and actual recovered-data status.
- [x] Keep source/recovered comparison independent of receiver feedback.
- [x] **UI-RUN-01** Add persistent lower-right Start, Stop and Pause/Resume icons with tooltips,
  accessible names and state-dependent availability.
- [x] Gate Start on every block being green; any yellow/red block disables it.
  The guard now admits only a complete supported request and an idle worker.
- [x] Connect Start to a validated frozen request and a bounded Rust worker job.
- [x] Connect Stop to cooperative cancellation, preserving completed partial results.
- [x] Define Rust pause boundaries, paused-time budget accounting, Stop/close while
  paused, progress delivery and worker cancellation on close.
- [x] Implement pause/resume without resetting simulation state or RNG streams.

## UI-02: Input dialogs and source files

Integrate input blocks as their owning sections are developed left to right.

- [x] Add the DPD Open File dialog, content detection, binary/JSON import and
  staged profile selection/removal with exact-byte provenance.
- [x] Enforce and verify OS-native Open for PRBS-TX and DPD without Qt fallback.
  Native project Save is implemented; folder selection remains outside current dialogs.
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
- [x] Attach accepted DPD profiles to the actual closed-loop Transmitter request.
- [x] Attach DS channel-estimation and RxMER configuration to Channel Simulator
  readiness and the frozen closed-loop request.
- [x] Validate binary PNM through the pinned decoder and standalone decoded
  JSON through the shared Rust importers;
  preserve grids, missing samples, timestamps and provenance.
- [x] Dispatch accepted multiple-RxMER configuration through validated full-run
  schedules; merely listing files must not activate playback.
- [ ] Apply replacements to the next run; preserve active inputs and original bytes.

## UI-03: Observations and analyzers

- [x] **UI-RF-02** Expand the channel workbench to available window space with
  balanced edge margins, equal growing gaps and matching measurement branches.
  [Design](features/rf-power-budget/design.md#ui-rf-02-responsive-channel-workbench-spacing).

- [x] **UI-RF-01** Implement amplifier/attenuator dialogs, scalar Rust conditioning,
  shared TP SA/QAM, optional electrical calibration/independent receiver noise and
  versioned WB persistence. Scoped tests and native VM checks pass.
  [Design](features/rf-power-budget/design.md#ui-rf-01-implemented-scalar-conditioning-and-channel-instruments).

- [ ] **RF-POWER-01** Complete RF-port loading/capability, per-subband limits and
  physical receiver overload/AGC modeling. Scalar gain/attenuation, explicit power
  references, TP instruments and WB persistence are implemented by UI-RF-01.
  [Design](features/rf-power-budget/design.md).
- [x] **UI-ORG-02** Group all QObject bridges, input models, workbench state,
  simulation lifecycle and platform integration; decouple saved-input restoration
  from Qt bridges. [Design](features/qt-source-organization/design.md#ui-org-02-organize-the-remaining-rust-qt-adapter).

- [x] **UI-ORG-01** Group QML and Rust instrument providers by responsibility,
  preserving UI behavior and updating registration/imports/resource references.
  [Design](features/qt-source-organization/design.md).
- [x] **UI-INSTR-01** Share channel TP QAM presentation across sources with explicit tap
  providers. Folder organization is tracked separately under UI-ORG-01.
  [Plan](features/rf-power-budget/design.md#shared-instrument-organization-proposal).

- [x] **UI-DOC-02** Add terminology Markdown and About menu online access.
  [Design](features/terminology/design.md).
- [x] **UI-TX-06** Identify Source and Source Bits as originating in PRBS-TX.
  [Design](features/transmitter-workbench/design.md#ui-tx-06-source-stage-origin-label).

- [x] **UI-TX-05** Color Data/NCP QAM roles with a legend and retain actual
  TX role provenance through bounded observation/display selection.
  [Design](features/transmitter-workbench/design.md#ui-tx-05-qam-symbol-role-colors-and-legend).

- [x] **UI-TX-04** Replace message TLV buttons with a folding tree grid and
  registry-backed decoded fields, preserving exact packet bytes.
  [Design](features/transmitter-workbench/design.md#ui-tx-04-folding-descriptor-tree-grid).

- [x] **UI-TX-03** Implement staged Transmitter navigation, test points
  and actual OCD/DPD header/TLV/byte inspection, preserving emitted-message status.
  [Design and validation](features/transmitter-workbench/design.md).

- [x] **UI-TX-01** Derive green Transmitter readiness from accepted OCD/DPD/source
  through the supported Rust request; invalidate/cancel on applied input changes.
- [x] **UI-TX-02** Reuse a generic SA dialog for actual TX-output samples, with
  centered baseband default, optional capture mapping, normalized Hann PSD,
  bounded min/max viewports, zoom, progress and cooperative cancellation.
  Design: [Transmitter spectrum](features/transmitter-spectrum/design.md).
- [x] **UI-SA-03** Clear the SA lower text panel and reserve blank space.
- [x] **UI-SA-04** Fit symbol/window timing and nominal cable echo budgets below
  the PSD, using frozen result settings and Rust calculations.
- [x] **UI-SA-06** Replace the SA axis dropdown with radio buttons and remove
  the redundant caption/raw Tap text, preserving the selected bin window.
  [Design](features/transmitter-spectrum/design.md#ui-sa-06-concise-frequency-axis-controls).
- [x] **UI-DOC-01** Refresh native user-guide captures, remove obsolete
  overview copies and reconcile stale instructions.
  [Audit record](features/user-guide-captures/design.md).

- [x] **UI-SA-07** Overlay plot metadata, move run/cancel actions to the footer,
  and clarify the cable echo estimate with a labeled grid.
  [Design](features/transmitter-spectrum/design.md#ui-sa-07-plot-metadata-footer-actions-and-cable-context).
- [ ] Add actual IQ sample trace inspection, distinct from the window-coefficient view.
- [x] Connect the shared SA to retained channel output and RX framed/useful taps.

- [x] Present actual LDPC outcomes/iterations and independent BER/completion status.
- [x] Add bounded matched supported payload pre/post-equalization views.
- [x] Add separate continuous/scattered pilot and PLC data/preamble views.
- [x] Reuse existing Rust results and test points; label unsupported observations.
- [x] Preserve units, reference planes, supplied H versus estimated H-hat,
  missing values and shared retention limits across open panels.

## Receiver workbench navigation and instruments

- [x] **UI-RX-WB-01** Replace the Receiver preview with its own workbench: processing
  stages, clickable test-point markers and instrument navigation; soft demapping
  and optional hard slicing remain separate branches.
- [x] **UI-RX-WB-02** Align equal-width stage/instrument chips and annotation rows;
  use rounded edges, subtle chip pins and visible keyboard/hover feedback.
- [x] **UI-QAM-01 selectors** Add Data Profile / NCP / PLC and Raw / Equalized /
  Paired controls, separate PLC data/preamble and unavailable received-profile state.
- [x] **UI-QAM-01 data** Connect supported payload/PLC received-role collectors.
- [x] Generalize available profile/modulation/bin/receive-symbol controls over
  retained data and preserve unsupported transport cases.
- [x] **UI-QAM-02** Add bounded matched received NCP I/Q classification/collection
  before enabling its constellation view; LLR/control events are not I/Q.
- [x] **UI-SA-05 selector** Add independent SA Test Point routing with empty-state
  admission and a distinct FFT-bin-power label; disable receiver generation.
- [x] **UI-SA-05 data** Connect retained RX time samples and FFT-bin power with
  correct units, boundary handling and normalization.
- [x] Verify selection does not rerun the simulation, hides stale results, preserves
  matched run/carrier/time identities and shares bounded retention across panels.

Design: [Receiver Workbench](features/receiver-workbench/design.md).
Navigation, matched NCP/payload/PLC/pilot plots and bounded retained-data selectors
are implemented. Unretained history and unsupported transport stay explicit.

- [x] **UI-CH-EST-RX-01** Display supplied H and actual H-hat snapshots with
  response curves and band-limited impulse/echo diagnostics.
- [x] **UI-SLICER-01** Show selected physical decisions/residuals and actual
  NCP/payload cell LLRs plus the retained LDPC-input window.
- [x] **UI-FEC-01** Add dedicated FEC statistics, iteration distribution and
  bounded word outcomes, with a prominent Receiver header shortcut.

[Receiver instrument completion design](features/receiver-instruments/design.md).

## UI-04: Validation and delivery evidence

- [ ] Verify responsive worker execution, progress and state-dependent controls.
- [ ] Test cancellation, runtime limits, pause/resume, Stop while paused and shutdown.
- [ ] Verify simulation parity with the same Rust request and observation invariance
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
- [x] **UI-RXMER-03 dispatch** Integrate accepted configuration and interpolation
  into full channel jobs, including job-duration admission and continuous RNG.

See the [design and remaining scope](features/rxmer-workbench/design.md).

- [x] **UI-QAM-01** Reuse paired RX FFT/equalized constellation observations before
  decisions, with carrier/modulation/time selection and explicit SA reference planes.

- [x] **UI-CHANNEL-01** Require valid RxMER and response configurations for green
  Channel Simulator; provide explicit Flat/unity H=1+0i default on the DPD grid.

Closed-loop execution/persistence: [design](features/qt-closed-loop/design.md) · [validation](features/qt-closed-loop/validation.md).


## Ubuntu 22.04 desktop delivery

- [x] **UI-DEB-01** Build the amd64 Qt DEB with private runtime, examples,
  notices and local package validation. [Design](features/ubuntu-desktop/design.md).
- [x] **UI-UPDATE-01** Add background stable-release notification, About retry
  and global startup preference. [Design](features/ubuntu-desktop/design.md).
- [ ] **UI-DEB-02** Retain successful hosted Ubuntu 22.04 build and disposable
  installed/reinstall/remove results; verify this VM's interactive installed UI.
- [x] **UI-DEB-03** Fix first-run scratch storage and SDK bootstrap failures;
  verify packaging tests in a fresh tracked checkout. Hosted rerun pending.
- [x] **SECURITY-DEPS-02** Patch rand/cxx advisories and audit both graphs
  without suppression. [Design](features/ubuntu-desktop/design.md).

- [x] **UI-DPD-VIEW-02** Adapt the full PNM Viewer modulation view with
  profile overlays, dB/bit loading, carrier filters/lanes and gross Profile BW.
  [Design](features/pnm-measurement-inputs/profile-view.md#ui-dpd-view-02-full-pnm-viewer-profile-inspection).

## Receiver profile performance

- [x] **UI-RX-PERF-01** Add the Receiver Profile Performance dialog with profile
  selection, MER/profile overlays, TotalCW/CCW/UFEC, mouse zoom and Full Span.
  Requires RX-PERF-01 independent jobs and FEC-COUNTS-01 actual core counters.
  [Design](features/profile-performance/design.md).

## Automatic Ubuntu desktop updates

- [x] **UI-UPDATE-02** Right-aligned Update chip and automatic Ubuntu
  download/verify/polkit install/restart with WB recovery. Local adapter/QML,
  package identity and native recovery checks pass.
  [Design](features/automatic-updates/design.md).
- [ ] **UI-UPDATE-QA-01** Validate a real installed upgrade, authentication
  cancellation, APT failure and automatic restart on disposable Ubuntu 22.04.

## Main workbench configuration and shared CI

- [x] **UI-WB-CONFIG-01** Main Configure WB with Run/Profiles tabs, explicit
  New/Open activation, subset persistence/migration and Receiver-only controls.
  Adapter/QML checks and native visual/Save As/Open verification pass.
- [x] **CI-CONSOLIDATE-01** Implement two entry points sharing all validation,
  parallel native tests and release-only packages; static validation passes.
- [x] **CI-DESKTOP-APT-01** Implement strict refresh, bounded retry and CI mirror
  preference; fake-command retry tests and shell validation pass.
- [ ] **CI-DESKTOP-QA-01** Verify shared hosted validation, successful APT setup,
  release packaging/installed lifecycle and publication on a new authorized tag.

[Design](features/workbench-configuration/design.md).

- [x] **UI-DIALOG-ACTIONS-01** Share lower-right Apply then Cancel actions
  across all nine draft editors; simplify labels and preserve cancellation.
  [Design](features/workbench-configuration/design.md#ui-dialog-actions-01-consistent-editor-actions).

- [x] **UI-FEC-02** Implement the approved FEC overview, actual iteration bars,
  per-profile selection and separate control/codeword tabs. Adapter/QML checks
  and native simulation/visual checks pass; compiled Qt executable is ready.
  [Design](features/fec-monitor/design.md).

- [x] **UI-RUN-TIME-01** Show live/final wall elapsed time, per-profile timing,
  actual phase and recovery progress. Clock/lifecycle, adapter and QML checks
  pass; native bounded Profile3 timing/CPU sample and screenshots verified.
  [Design](features/run-elapsed-time/design.md).

- [x] **UI-TERMS-02** Define interleaver depth/drain in the About-linked guide
  and require technical UI definitions through the UI-design skill.
  [Design](features/run-elapsed-time/design.md#ui-terms-02-processing-terminology).

## Shared action buttons

- [x] **UI-BUTTONS-01** Apply the approved shared rounded action style across
  all frames, including standard dialog and Apply/Cancel buttons. Preserve
  roles, keyboard behavior, themes and specialized diagram controls; require
  the component through the UI skill.
  [Design](features/workbench-configuration/design.md#ui-buttons-01-shared-action-buttons-across-the-desktop).

- [x] UI-HELP-01: Stable playback diagnostic with actual minimum-duration
  advice, hover/focus tooltip, keyboard/click offline FAQ and About entry.
- [x] UI-SA-DETAILS-01: Acquisition readout below axes; Details toggle
  preserves spectrum data, channel power and warning visibility.

- [x] UI-ABOUT-01: Plain dropdown labels and compact About actions, including
  manual update checks and issue-form link; dark/light native review complete.

## Responsive Open and RxMER selection

- [x] **UI-WB-OPEN-02** Background native Open/recovery, prepared-model adoption,
  loading stage/elapsed/Cancel, no repeated GUI imports; worker failure,
  cancellation and native responsiveness verified.
  [Design](features/responsive-workbench-open/design.md).
- [x] **UI-RXMER-SELECT-01** Retain loaded originals and persist selected capture
  identities; Single Shot/Replay controls and CWB-derived capacity. Verify
  selection, legacy restore, strict boundaries and native layout.
  [Design](features/rxmer-selection/design.md).
- [x] **UI-RXMER-ANIM-01** PNM Viewer-style selected-capture animation,
  Play/Pause/step/slider/speed, bounded original viewports and lifecycle cleanup.
  [Design](features/rxmer-selection/design.md#ui-rxmer-anim-01--animated-capture-preview).

- [x] **UI-TX-GRAPH-01** Graphical transmitter test-point cards and full stage
  graphs, optional exact Details, role legends and shared SA navigation.
  Verified bounded real graphs, missing history and native dark/light layouts;
  54 Rust adapter tests and 53 QML checks pass.
  [Design](features/transmitter-graphs/design.md).

- [x] **UI-WB-OPEN-03** Show an animated loading activity bar and distinguish
  Time since Open from file progress and simulation time; retain current phase
  and cancellation. QML behavior, native rendering and build verified.
  [Design](features/responsive-workbench-open/design.md#ui-wb-open-03--clear-loading-activity).

- [x] **UI-CH-ICON-01** Shared graphical Spectrum Analyzer / QAM Analyzer choices
  for every channel input, output and amplifier TapPoint. Use matching graph
  cards with titles above and no TX Output caption. Verify equal icons,
  keyboard routing and native rendering; 53 QML checks pass.
  [Design](features/channel-workbench/design.md#ui-ch-icon-01--graphical-instrument-choices).
- [x] **UI-FEC-TABLE-01** Compact Channel / Metric / Value grid for whole-run
  control statistics, with aligned values and bounded width.
  [Design](features/fec-monitor/design.md#ui-fec-table-01--compact-control-statistics).

- [x] **UI-WB-OPEN-04** Hide loading timing except --debug-ui; support startup
  workbench paths and redirected stdin through bounded background validation.
  Maintain debug-control documentation and lifecycle logging. Normal/debug
  rendering, startup, Save destinations, errors and cancellation verified;
  58 Rust tests, 53 QML checks and native Ubuntu launches pass.
  [Design](features/responsive-workbench-open/design.md#ui-wb-open-04--quiet-loading-and-command-line-workbenches).

- [x] **UI-SA-TIMING-02** Expand the compact timing plot into a resizable full
  two-symbol and overlap view. Use actual core window weights, stable time axes,
  explicit result/current-configuration source and accepted-setting refresh.
  Verified both FFTs, all CP/RP pairs, keyboard/resize and native dark/light UI;
  61 Rust tests, 54 QML checks, Clippy and native build pass.
  [Design](features/symbol-timing-popup/design.md).
