# Qt UI checklist

This is the subordinate Qt detail checklist. The
[official developer checklist](checklist.md) owns overall delivery and the
[canonical requirements baseline](../requirements.md) owns requirements.
Upstream backend integration did not implement upstream UI. The separate
[unified Qt plan](features/unified-qt-workbench/design.md) now defines that UI
scope; its [official UIU tasks](checklist.md#unified-qt-workbench-delivery) own
completion status. The initial choices are one active simulation across both
tabs and Save for the active direction's project. No new Qt implementation is
accepted by this planning update. INT-DEFER-01's wider PHY deferrals remain.

The UIU task group covers public-API migration and boundary enforcement,
shared/downstream organization, isolated direction sessions and tabs, upstream
configuration/execution/PNM/resource-grid windows, proven shared instruments,
project compatibility, restart recovery and final evidence. See its direct
requirement/architecture links and dependency edges rather than duplicating
completion marks here. Existing UI-01 through UI-04 and UI-ORG history below
retain their original status owners and evidence.

The folder-style direction-tab refinement is tracked by
[UI-TABS-01](checklist.md#task-ui-tabs-01), with the existing shell/session
behavior preserved; its official completion mark remains in that checklist.
The matching upstream canvas framing and simplified CMTS control route follow
[UI-US-LAYOUT-01](checklist.md#task-ui-us-layout-01) under the same status owner.
The later reversed upstream orientation and receiver-side control connection
follow [UI-US-LAYOUT-02](checklist.md#task-ui-us-layout-02).
The subsequent shared diagram style and direct CMTS junction follow
[UI-US-LAYOUT-03](checklist.md#task-ui-us-layout-03).

The [downstream contract register](../requirements.md#detailed-downstream-contract-register)
adds requirement/task/evidence navigation without changing these detail marks.
Historical shell proposals and validation paragraphs describe their own milestone;
later named refinements govern current controls and defaults. In particular, the
old Python scheduling sentence below is superseded by completed API-01/API-PY-01
in the official checklist and the [native API evidence](features/high-level-api/validation.md).
Open UI umbrella/native acceptance tasks retain their existing status.

The [architecture](../architecture.md#ui-merlin-simulation-workbench) defines the
MERLIN Simulation Workbench. This checklist expands UI-01 through UI-04 in the
[main checklist](checklist.md); keep their milestone status consistent.
See the [task design](features/visual-workspace/design.md). The initial shell is implemented; unchecked tasks remain pending. Initial targets: Ubuntu 22.04/24.04/26.04 x64.

## UI-01: Workbench and section dialogs

<a id="task-ui-copy-01"></a>
- [x] **UI-COPY-01** Use user-focused text and concise essential notes; remove
  programming-language/framework commentary from normal UI.
  [Design](features/ui-copy/design.md).

  Requirements: [UI-COPY-01](../requirements.md#ds-ui-copy-01).


<a id="task-ui-examples-01"></a>
- [x] **UI-EXAMPLES-01** Curate single/multi-RxMER WB bundles with compatible
  originals, hashes and notices. [Design](features/workbench-examples/design.md).

  Requirements: [UI-EXAMPLES-01](../requirements.md#ds-ui-examples-01).

<a id="task-ui-examples-02"></a>
- [x] **UI-EXAMPLES-02** Add channel 193 `multi-rxmer-2`: 132 original RxMER
  files, companion DPD, flat response and bounded 256-file support.
  [Design](features/workbench-examples/design.md#ui-examples-02-channel-193-measured-rxmer-collection).

  Requirements: [UI-EXAMPLES-02](../requirements.md#ds-ui-examples-02).

<a id="task-ui-open-01"></a>
- [x] **UI-OPEN-01** Default native Open to relocatable example resources.
  [Design](features/workbench-examples/design.md).

  Requirements: [UI-OPEN-01](../requirements.md#ds-ui-open-01).

<a id="task-ui-wb-open-01"></a>
- [x] **UI-WB-OPEN-01** Filter main Open / Ctrl+O to `*.merlin.json` while
  retaining folders and independent input pickers; verify native VM behavior.
  [Design](features/qt-closed-loop/design.md#ui-wb-open-01-main-open-file-filter).

  Requirements: [UI-WB-OPEN-01](../requirements.md#ds-ui-wb-open-01).

<a id="task-install-examples-01"></a>
- [ ] **INSTALL-EXAMPLES-01** Ship and validate the runnable inventory in future
  native desktop installers, including installed-OS Open/restore behavior.

  Requirements: [INSTALL-EXAMPLES-01](../requirements.md#ds-install-examples-01).

<a id="task-ch-est-playback-01"></a>
- [ ] **CH-EST-PLAYBACK-01** Define varying channel-estimate collections before
  adding a runnable multi-RxMER/multi-channel-estimate WB.

  Requirements: [CH-EST-PLAYBACK-01](../requirements.md#ds-ch-est-playback-01).


<a id="task-ui-rxmer-04"></a>
- [x] **UI-RXMER-04** Remove RxMER demo/notch buttons and use Open file(s).
  [Design](features/rxmer-workbench/design.md#ui-rxmer-04-remove-demo-actions-from-rxmer).

  Requirements: [UI-RXMER-04](../requirements.md#ds-ui-rxmer-04).


<a id="task-ui-shell-11"></a>
- [x] **UI-SHELL-11** Remove the left icon/WB gutter and use equal side margins.
  [Design](features/verification-panel/design.md).

  Requirements: [UI-SHELL-11](../requirements.md#ds-ui-shell-11).

<a id="task-ui-verify-02"></a>
- [x] **UI-VERIFY-02** Restyle PRBS-RX with comparison cards, payload recovery
  and retained word-error charts. [Design](features/verification-panel/design.md).

  Requirements: [UI-VERIFY-02](../requirements.md#ds-ui-verify-02).


<a id="task-ui-shell-10"></a>
- [x] **UI-SHELL-10** Center diagram connections, refine the two-line branded
  header and show PNM type footnotes inside input blocks.
  [Design](features/visual-workspace/design.md#ui-shell-10-centered-connections-and-in-card-pnm-types).

  Requirements: [UI-SHELL-10](../requirements.md#ds-ui-shell-10).


<a id="task-ui-shell-09"></a>
- [x] **UI-SHELL-09** Group OCD/DPD under Transmitter Configuration and match
  the revised diagram/header styling. [Design](features/visual-workspace/design.md#ui-shell-09-group-transmitter-configuration).

  Requirements: [UI-SHELL-09](../requirements.md#ds-ui-shell-09).


<a id="task-ui-zoom-01"></a>
- [x] **UI-ZOOM-01** Replace all zoom buttons with shared mouse wheel/range
  navigation and retain Full Span. [Design](features/mouse-zoom/design.md).

  Requirements: [UI-ZOOM-01](../requirements.md#ds-ui-zoom-01).


<a id="task-ui-shell-07"></a>
- [x] **UI-SHELL-07** Place RxMER above Channel Simulator and split DS Channel
  Estimate into channel/receiver inputs; remove the top comparison connector.
  [Design](features/visual-workspace/design.md#ui-shell-07-measurement-input-placement).

  Requirements: [UI-SHELL-07](../requirements.md#ds-ui-shell-07).


Develop the workbench left to right; OFDM Channel Descriptor (OCD) is the first functional dialog.

<a id="task-ui-01"></a>
- [x] Inspect PNM Viewer Qt infrastructure read-only; select reusable components
  and record source revisions, adaptations, licenses and notices in the design.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-2"></a>
- [x] Add the native Qt Quick/QML shell and separate Rust presentation adapter.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-3"></a>
- [x] Render the user's high-level signal path and clickable input blocks.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-4"></a>
- [x] Open a focused section-preview pop-up for each block; keep the workbench
  visible behind it; accepted source/profile/measurement editors are available.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-5"></a>
- [ ] Implement OFDM Channel Descriptor (OCD): cyclic prefix, roll-off/windowing, interleaver
  depth and supported pilot settings, using Rust validation for the next run.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-6"></a>
- [x] Replace OFDM preview with an expandable settings tree, supported-value
  dropdowns, field descriptions/source references, and Apply/Cancel behavior.
  Use Parameter/Value columns with descriptions below; keep Modulation Profile
  values blank until selected, without displaying placeholder channel defaults.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-7"></a>
- [x] Add a bundled JSON field registry with pinned PHY references, separately
  labeled application defaults, and consistency checks against Rust settings.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-shell-02"></a>
- [x] **UI-SHELL-02** Add left-aligned File | Settings | About actions; move
  Light/Dark mode into Settings → Global; remove the instruction banner.
  Settings/About have separate QML files; explicit WB saves include the theme.

  Requirements: [UI-SHELL-02](../requirements.md#ds-ui-shell-02).

<a id="task-ui-wb-json-01"></a>
- [x] **UI-WB-JSON-01** Implement File Open/Save for JSON containing all PNM inputs
  and accepted settings of every block; define bounds, schema and input encoding.

  Requirements: [UI-WB-JSON-01](../requirements.md#ds-ui-wb-json-01).

<a id="task-ui-shell-03"></a>
- [x] **UI-SHELL-03** Use a standard application menu bar above WB branding,
  replacing the content-area navigation buttons.

  Requirements: [UI-SHELL-03](../requirements.md#ds-ui-shell-03).

<a id="task-ui-shell-06"></a>
- [x] **UI-SHELL-06** Add a shared circular X to all popup titles, retaining
  footer buttons, draft cancellation and nested-popup behavior.
  [Design](features/popup-close-controls/design.md).

  Requirements: [UI-SHELL-06](../requirements.md#ds-ui-shell-06).

<a id="task-ui-shell-05"></a>
- [x] **UI-SHELL-05** Add About icon/version/project and offline license links;
  standardize title capitalization and matched rounded light/dark icons.

  Requirements: [UI-SHELL-05](../requirements.md#ds-ui-shell-05).

<a id="task-ui-01-detail-8"></a>
- [x] Define and implement versioned workbench JSON persistence separately from
  registry metadata; Open restores configuration without starting a run.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-9"></a>
- [x] Show heading-only blocks and shared green/yellow/red readiness stripes,
  with status/reason tooltips and accessible descriptions.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-10"></a>
- [x] Keep OFDM Channel Descriptor (OCD) yellow while a validated modulation-profile input is
  unavailable; record WB-RULE-01/02 in the architecture and task design.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-11"></a>
- [x] Connect accepted/removed/rejected DPD input to Rust-backed DPD/OCD readiness.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-dpd-view-01"></a>
- [x] **UI-DPD-VIEW-01** Add View Profile with role colors, bounded bit-loading
  plot, mouse zoom and Full Span; hide editor filenames and preserve originals.
  [Design](features/pnm-measurement-inputs/profile-view.md).

  Requirements: [UI-DPD-VIEW-01](../requirements.md#ds-ui-dpd-view-01).

<a id="task-ui-01-detail-12"></a>
- [x] Connect accepted input/job states and dependency invalidation to readiness.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-13"></a>
- [x] Implement PRBS-TX configuration and supported Transmitter preview readiness.
  Full-loop Transmitter/job wiring uses the shared Rust engine. Source options are defined in the
  [architecture](../architecture.md#prbs-tx-source-configuration).

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-ch-01"></a>
- [x] **UI-CH-01** Implement Channel Input/Model/Output staging with direct
  RxMER/DS Channel Estimate frames and Spectrum/QAM chooser at each boundary.
  Separate additional-loss editor supplies manual tilt and full cable loss.
  [Design](features/channel-workbench/design.md).

  Requirements: [UI-CH-01](../requirements.md#ds-ui-ch-01).

<a id="task-ui-rx-config-01"></a>
- [x] **UI-RX-CONFIG-01 / UI-VERIFY-01** Implement Receiver / PRBS-RX configuration
  and actual recovered-data status.

  Requirements: [UI-RX-CONFIG-01](../requirements.md#ds-ui-rx-config-01).

<a id="task-ui-01-detail-14"></a>
- [x] Keep source/recovered comparison independent of receiver feedback.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-run-01"></a>
- [x] **UI-RUN-01** Add persistent lower-right Start, Stop and Pause/Resume icons with tooltips,
  accessible names and state-dependent availability.

  Requirements: [UI-RUN-01](../requirements.md#ds-ui-run-01).

<a id="task-ui-01-detail-15"></a>
- [x] Gate Start on every block being green; any yellow/red block disables it.
  The guard now admits only a complete supported request and an idle worker.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-16"></a>
- [x] Connect Start to a validated frozen request and a bounded Rust worker job.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-17"></a>
- [x] Connect Stop to cooperative cancellation, preserving completed partial results.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-18"></a>
- [x] Define Rust pause boundaries, paused-time budget accounting, Stop/close while
  paused, progress delivery and worker cancellation on close.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-01-detail-19"></a>
- [x] Implement pause/resume without resetting simulation state or RNG streams.

  Requirements: [UI-01](../requirements.md#ds-ui-01).


## UI-02: Input dialogs and source files

Integrate input blocks as their owning sections are developed left to right.

<a id="task-ui-02"></a>
- [x] Add the DPD Open File dialog, content detection, binary/JSON import and
  staged profile selection/removal with exact-byte provenance.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-2"></a>
- [x] Enforce and verify OS-native Open for PRBS-TX and DPD without Qt fallback.
  Native project Save is implemented; folder selection remains outside current dialogs.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-3"></a>
- [ ] Add OS file pickers to remaining applicable dialogs and show input roles/status.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-4"></a>
- [x] Support PRBS15 or exact text/binary payload bytes with Rust-derived size
  feedback and explicit optional append-PRBS15 fill.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

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
<a id="task-ui-02-detail-5"></a>
- [x] Accept modulation-profile capture inputs in DPD and show the OCD summary.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-6"></a>
- [x] Attach accepted DPD profiles to the actual closed-loop Transmitter request.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-7"></a>
- [x] Attach DS channel-estimation and RxMER configuration to Channel Simulator
  readiness and the frozen closed-loop request.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-8"></a>
- [x] Validate binary PNM through the pinned decoder and standalone decoded
  JSON through the shared Rust importers;
  preserve grids, missing samples, timestamps and provenance.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-9"></a>
- [x] Dispatch accepted multiple-RxMER configuration through validated full-run
  schedules; merely listing files must not activate playback.

  Requirements: [UI-02](../requirements.md#ds-ui-02).

<a id="task-ui-02-detail-10"></a>
- [ ] Apply replacements to the next run; preserve active inputs and original bytes.

  Requirements: [UI-02](../requirements.md#ds-ui-02).


## UI-03: Observations and analyzers

<a id="task-ui-rf-02"></a>
- [x] **UI-RF-02** Expand the channel workbench to available window space with
  balanced edge margins, equal growing gaps and matching measurement branches.
  [Design](features/rf-power-budget/design.md#ui-rf-02-responsive-channel-workbench-spacing).

  Requirements: [UI-RF-02](../requirements.md#ds-ui-rf-02).


<a id="task-ui-rf-01"></a>
- [x] **UI-RF-01** Implement amplifier/attenuator dialogs, scalar Rust conditioning,
  shared TP SA/QAM, optional electrical calibration/independent receiver noise and
  versioned WB persistence. Scoped tests and native VM checks pass.
  [Design](features/rf-power-budget/design.md#ui-rf-01-implemented-scalar-conditioning-and-channel-instruments).

  Requirements: [UI-RF-01](../requirements.md#ds-ui-rf-01).


<a id="task-rf-power-01"></a>
- [ ] **RF-POWER-01** Complete RF-port loading/capability, per-subband limits and
  physical receiver overload/AGC modeling. Scalar gain/attenuation, explicit power
  references, TP instruments and WB persistence are implemented by UI-RF-01.
  [Design](features/rf-power-budget/design.md).

  Requirements: [RF-POWER-01](../requirements.md#ds-rf-power-01).

<a id="task-ui-org-02"></a>
- [x] **UI-ORG-02** Group all QObject bridges, input models, workbench state,
  simulation lifecycle and platform integration; decouple saved-input restoration
  from Qt bridges. [Design](features/qt-source-organization/design.md#ui-org-02-organize-the-remaining-rust-qt-adapter).

  Requirements: [UI-ORG-02](../requirements.md#ds-ui-org-02).


<a id="task-ui-org-01"></a>
- [x] **UI-ORG-01** Group QML and Rust instrument providers by responsibility,
  preserving UI behavior and updating registration/imports/resource references.
  [Design](features/qt-source-organization/design.md).

  Requirements: [UI-ORG-01](../requirements.md#ds-ui-org-01).

<a id="task-ui-instr-01"></a>
- [x] **UI-INSTR-01** Share channel TP QAM presentation across sources with explicit tap
  providers. Folder organization is tracked separately under UI-ORG-01.
  [Plan](features/rf-power-budget/design.md#shared-instrument-organization-proposal).

  Requirements: [UI-INSTR-01](../requirements.md#ds-ui-instr-01).


<a id="task-req-ds-doc-terms-01"></a>
- [x] **UI-DOC-02** Add terminology Markdown and About menu online access.
  [Design](features/terminology/design.md).

  Requirements: [REQ-DS-DOC-TERMS-01](../requirements.md#ds-req-ds-doc-terms-01).

<a id="task-ui-tx-06"></a>
- [x] **UI-TX-06** Identify Source and Source Bits as originating in PRBS-TX.
  [Design](features/transmitter-workbench/design.md#ui-tx-06-source-stage-origin-label).

  Requirements: [UI-TX-06](../requirements.md#ds-ui-tx-06).


<a id="task-ui-tx-05"></a>
- [x] **UI-TX-05** Color Data/NCP QAM roles with a legend and retain actual
  TX role provenance through bounded observation/display selection.
  [Design](features/transmitter-workbench/design.md#ui-tx-05-qam-symbol-role-colors-and-legend).

  Requirements: [UI-TX-05](../requirements.md#ds-ui-tx-05).


<a id="task-ui-tx-04"></a>
- [x] **UI-TX-04** Replace message TLV buttons with a folding tree grid and
  registry-backed decoded fields, preserving exact packet bytes.
  [Design](features/transmitter-workbench/design.md#ui-tx-04-folding-descriptor-tree-grid).

  Requirements: [UI-TX-04](../requirements.md#ds-ui-tx-04).


<a id="task-ui-tx-03"></a>
- [x] **UI-TX-03** Implement staged Transmitter navigation, test points
  and actual OCD/DPD header/TLV/byte inspection, preserving emitted-message status.
  [Design and validation](features/transmitter-workbench/design.md).

  Requirements: [UI-TX-03](../requirements.md#ds-ui-tx-03).


<a id="task-ui-tx-01"></a>
- [x] **UI-TX-01** Derive green Transmitter readiness from accepted OCD/DPD/source
  through the supported Rust request; invalidate/cancel on applied input changes.

  Requirements: [UI-TX-01](../requirements.md#ds-ui-tx-01).

<a id="task-ui-tx-02"></a>
- [x] **UI-TX-02** Reuse a generic SA dialog for actual TX-output samples, with
  centered baseband default, optional capture mapping, normalized Hann PSD,
  bounded min/max viewports, zoom, progress and cooperative cancellation.
  Design: [Transmitter spectrum](features/transmitter-spectrum/design.md).

  Requirements: [UI-TX-02](../requirements.md#ds-ui-tx-02).

<a id="task-ui-sa-03"></a>
- [x] **UI-SA-03** Clear the SA lower text panel and reserve blank space.

  Requirements: [UI-SA-03](../requirements.md#ds-ui-sa-03).

<a id="task-ui-sa-04"></a>
- [x] **UI-SA-04** Fit symbol/window timing and nominal cable echo budgets below
  the PSD, using frozen result settings and Rust calculations.

  Requirements: [UI-SA-04](../requirements.md#ds-ui-sa-04).

<a id="task-ui-sa-06"></a>
- [x] **UI-SA-06** Replace the SA axis dropdown with radio buttons and remove
  the redundant caption/raw Tap text, preserving the selected bin window.
  [Design](features/transmitter-spectrum/design.md#ui-sa-06-concise-frequency-axis-controls).

  Requirements: [UI-SA-06](../requirements.md#ds-ui-sa-06).

<a id="task-ui-doc-01"></a>
- [x] **UI-DOC-01** Refresh native user-guide captures, remove obsolete
  overview copies and reconcile stale instructions.
  [Audit record](features/user-guide-captures/design.md).

  Requirements: [UI-DOC-01](../requirements.md#ds-ui-doc-01).


<a id="task-ui-sa-07"></a>
- [x] **UI-SA-07** Overlay plot metadata, move run/cancel actions to the footer,
  and clarify the cable echo estimate with a labeled grid.
  [Design](features/transmitter-spectrum/design.md#ui-sa-07-plot-metadata-footer-actions-and-cable-context).

  Requirements: [UI-SA-07](../requirements.md#ds-ui-sa-07).

<a id="task-ui-03"></a>
- [ ] Add actual IQ sample trace inspection, distinct from the window-coefficient view.

  Requirements: [UI-03](../requirements.md#ds-ui-03).

<a id="task-ui-03-detail-2"></a>
- [x] Connect the shared SA to retained channel output and RX framed/useful taps.

  Requirements: [UI-03](../requirements.md#ds-ui-03).


<a id="task-ui-03-detail-3"></a>
- [x] Present actual LDPC outcomes/iterations and independent BER/completion status.

  Requirements: [UI-03](../requirements.md#ds-ui-03).

<a id="task-ui-03-detail-4"></a>
- [x] Add bounded matched supported payload pre/post-equalization views.

  Requirements: [UI-03](../requirements.md#ds-ui-03).

<a id="task-ui-03-detail-5"></a>
- [x] Add separate continuous/scattered pilot and PLC data/preamble views.

  Requirements: [UI-03](../requirements.md#ds-ui-03).

<a id="task-ui-03-detail-6"></a>
- [x] Reuse existing Rust results and test points; label unsupported observations.

  Requirements: [UI-03](../requirements.md#ds-ui-03).

<a id="task-ui-03-detail-7"></a>
- [x] Preserve units, reference planes, supplied H versus estimated H-hat,
  missing values and shared retention limits across open panels.

  Requirements: [UI-03](../requirements.md#ds-ui-03).


## Receiver workbench navigation and instruments

<a id="task-ui-rx-wb-01"></a>
- [x] **UI-RX-WB-01** Replace the Receiver preview with its own workbench: processing
  stages, clickable test-point markers and instrument navigation; soft demapping
  and optional hard slicing remain separate branches.

  Requirements: [UI-RX-WB-01](../requirements.md#ds-ui-rx-wb-01).

<a id="task-ui-rx-wb-02"></a>
- [x] **UI-RX-WB-02** Align equal-width stage/instrument chips and annotation rows;
  use rounded edges, subtle chip pins and visible keyboard/hover feedback.

  Requirements: [UI-RX-WB-02](../requirements.md#ds-ui-rx-wb-02).

<a id="task-ui-qam-01"></a>
- [x] **UI-QAM-01 selectors** Add Data Profile / NCP / PLC and Raw / Equalized /
  Paired controls, separate PLC data/preamble and unavailable received-profile state.

  Requirements: [UI-QAM-01](../requirements.md#ds-ui-qam-01).

<a id="task-ui-qam-01-detail-2"></a>
- [x] **UI-QAM-01 data** Connect supported payload/PLC received-role collectors.

  Requirements: [UI-QAM-01](../requirements.md#ds-ui-qam-01).

<a id="task-ui-01-detail-20"></a>
- [x] Generalize available profile/modulation/bin/receive-symbol controls over
  retained data and preserve unsupported transport cases.

  Requirements: [UI-01](../requirements.md#ds-ui-01).

<a id="task-ui-qam-02"></a>
- [x] **UI-QAM-02** Add bounded matched received NCP I/Q classification/collection
  before enabling its constellation view; LLR/control events are not I/Q.

  Requirements: [UI-QAM-02](../requirements.md#ds-ui-qam-02).

<a id="task-ui-sa-05"></a>
- [x] **UI-SA-05 selector** Add independent SA Test Point routing with empty-state
  admission and a distinct FFT-bin-power label; disable receiver generation.

  Requirements: [UI-SA-05](../requirements.md#ds-ui-sa-05).

<a id="task-ui-sa-05-detail-2"></a>
- [x] **UI-SA-05 data** Connect retained RX time samples and FFT-bin power with
  correct units, boundary handling and normalization.

  Requirements: [UI-SA-05](../requirements.md#ds-ui-sa-05).

<a id="task-ui-01-detail-21"></a>
- [x] Verify selection does not rerun the simulation, hides stale results, preserves
  matched run/carrier/time identities and shares bounded retention across panels.

  Requirements: [UI-01](../requirements.md#ds-ui-01).


Design: [Receiver Workbench](features/receiver-workbench/design.md).
Navigation, matched NCP/payload/PLC/pilot plots and bounded retained-data selectors
are implemented. Unretained history and unsupported transport stay explicit.

<a id="task-ui-ch-est-rx-01"></a>
- [x] **UI-CH-EST-RX-01** Display supplied H and actual H-hat snapshots with
  response curves and band-limited impulse/echo diagnostics.

  Requirements: [UI-CH-EST-RX-01](../requirements.md#ds-ui-ch-est-rx-01).

<a id="task-ch-est-rx-11"></a>
- [x] **CH-EST-RX-11** Add Off-by-default MCE in Receiver Configuration and a
  small main Receiver chip without moving its centered heading. Keep waveform
  channel controls separate; preserve Apply/Cancel, WB settings and known-error
  help. 66 adapter/57 QML checks and native dark/light views pass.
  [Implementation](features/independent-channel-estimation/implementation.md).

  Requirements: [CH-EST-RX-11](../requirements.md#ds-ch-est-rx-11).

<a id="task-ch-est-rx-13"></a>
- [x] **CH-EST-RX-13** Use the agreed three-mode dropdown and conditional MCE
  averaging; document channel versus noise sources, unity/default behavior and
  saved/API mode names. Preserve Apply/Cancel and accepted-mode indicators.
  [Design](features/independent-channel-estimation/design.md#ch-est-rx-13-explicit-three-mode-receiver-wording-and-documentation),
  [guide](../user/receiver-estimation/modes.md).

  Requirements: [CH-EST-RX-13](../requirements.md#ds-ch-est-rx-13).

<a id="task-ui-slicer-01"></a>
- [x] **UI-SLICER-01** Show selected physical decisions/residuals and actual
  NCP/payload cell LLRs plus the retained LDPC-input window.

  Requirements: [UI-SLICER-01](../requirements.md#ds-ui-slicer-01).

<a id="task-ui-fec-01"></a>
- [x] **UI-FEC-01** Add dedicated FEC statistics, iteration distribution and
  bounded word outcomes, with a prominent Receiver header shortcut.

  Requirements: [UI-FEC-01](../requirements.md#ds-ui-fec-01).


[Receiver instrument completion design](features/receiver-instruments/design.md).

## UI-04: Validation and delivery evidence

<a id="task-ui-04"></a>
- [ ] Verify responsive worker execution, progress and state-dependent controls.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-2"></a>
- [ ] Test cancellation, runtime limits, pause/resume, Stop while paused and shutdown.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-3"></a>
- [ ] Verify simulation parity with the same Rust request and observation invariance
  when opening/closing dialogs and analyzers, separating host timing from science.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-4"></a>
- [ ] Test invalid input, repeated runs, partial results and resource cleanup.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-5"></a>
- [ ] Check dark/light readability, keyboard navigation and accessible controls.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-6"></a>
- [ ] Validate native builds and runtime separately on Ubuntu 22.04/24.04/26.04 x64.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-7"></a>
- [ ] Keep Rust/CLI builds independent of Qt and run applicable Rust and Qt checks.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-8"></a>
- [ ] Update user guides with every WB UI/graphic change; use dark-theme
  graphics and actual VM screenshots for implemented visual changes.

  Requirements: [UI-04](../requirements.md#ds-ui-04).

<a id="task-ui-04-detail-9"></a>
- [ ] Reconcile architecture, task design and both checklists with verified behavior.

  Requirements: [UI-04](../requirements.md#ds-ui-04).


Python API-01 remains deferred until UI-01 through UI-04 are complete.

Initial shell evidence and remaining validation: [Qt development](qt.md).

## Proposed RxMER editor and shell follow-up

<a id="task-ui-shell-04"></a>
- [x] **UI-SHELL-04** Use the requested main WB heading and conventional File labels.

  Requirements: [UI-SHELL-04](../requirements.md#ds-ui-shell-04).

<a id="task-ui-rxmer-01"></a>
- [x] **UI-RXMER-01** Stage single/multiple RxMER inputs with exact DPD association.

  Requirements: [UI-RXMER-01](../requirements.md#ds-ui-rxmer-01).

<a id="task-ui-rxmer-02"></a>
- [x] **UI-RXMER-02** Add static/average/hold/interpolated-power configuration and
  bounded previews with strict required-carrier validity.

  Requirements: [UI-RXMER-02](../requirements.md#ds-ui-rxmer-02).

<a id="task-ui-rxmer-03"></a>
- [x] **UI-RXMER-03 editor** Expose symbol intervals/order/end policy and previews.

  Requirements: [UI-RXMER-03](../requirements.md#ds-ui-rxmer-03).

<a id="task-ui-rxmer-03-detail-2"></a>
- [x] **UI-RXMER-03 dispatch** Integrate accepted configuration and interpolation
  into full channel jobs, including job-duration admission and continuous RNG.

  Requirements: [UI-RXMER-03](../requirements.md#ds-ui-rxmer-03).


See the [design and remaining scope](features/rxmer-workbench/design.md).

<a id="task-ui-qam-01-detail-3"></a>
- [x] **UI-QAM-01** Reuse paired RX FFT/equalized constellation observations before
  decisions, with carrier/modulation/time selection and explicit SA reference planes.

  Requirements: [UI-QAM-01](../requirements.md#ds-ui-qam-01).


<a id="task-ui-channel-01"></a>
- [x] **UI-CHANNEL-01** Require valid RxMER and response configurations for green
  Channel Simulator; provide explicit Flat/unity H=1+0i default on the DPD grid.

  Requirements: [UI-CHANNEL-01](../requirements.md#ds-ui-channel-01).


Closed-loop execution/persistence: [design](features/qt-closed-loop/design.md) · [validation](features/qt-closed-loop/validation.md).


## Ubuntu 22.04 desktop delivery

<a id="task-ui-deb-01"></a>
- [x] **UI-DEB-01** Build the amd64 Qt DEB with private runtime, examples,
  notices and local package validation. [Design](features/ubuntu-desktop/design.md).

  Requirements: [UI-DEB-01](../requirements.md#ds-ui-deb-01).

<a id="task-ui-update-01"></a>
- [x] **UI-UPDATE-01** Add background stable-release notification, About retry
  and global startup preference. [Design](features/ubuntu-desktop/design.md).

  Requirements: [UI-UPDATE-01](../requirements.md#ds-ui-update-01).

<a id="task-ui-deb-02"></a>
- [ ] **UI-DEB-02** Retain successful hosted Ubuntu 22.04 build and disposable
  installed/reinstall/remove results; verify this VM's interactive installed UI.

  Requirements: [UI-DEB-02](../requirements.md#ds-ui-deb-02).

<a id="task-ui-deb-03"></a>
- [x] **UI-DEB-03** Fix first-run scratch storage and SDK bootstrap failures;
  verify packaging tests in a fresh tracked checkout. Hosted rerun pending.

  Requirements: [UI-DEB-03](../requirements.md#ds-ui-deb-03).

<a id="task-security-deps-02"></a>
- [x] **SECURITY-DEPS-02** Patch rand/cxx advisories and audit both graphs
  without suppression. [Design](features/ubuntu-desktop/design.md).

  Requirements: [SECURITY-DEPS-02](../requirements.md#ds-security-deps-02).


<a id="task-ui-dpd-view-02"></a>
- [x] **UI-DPD-VIEW-02** Adapt the full PNM Viewer modulation view with
  profile overlays, dB/bit loading, carrier filters/lanes and gross Profile BW.
  [Design](features/pnm-measurement-inputs/profile-view.md#ui-dpd-view-02-full-pnm-viewer-profile-inspection).

  Requirements: [UI-DPD-VIEW-02](../requirements.md#ds-ui-dpd-view-02).


## Receiver profile performance

<a id="task-ui-rx-perf-01"></a>
- [x] **UI-RX-PERF-01** Add the Receiver Profile Performance dialog with profile
  selection, MER/profile overlays, TotalCW/CCW/UFEC, mouse zoom and Full Span.
  Requires RX-PERF-01 independent jobs and FEC-COUNTS-01 actual core counters.
  [Design](features/profile-performance/design.md).

  Requirements: [UI-RX-PERF-01](../requirements.md#ds-ui-rx-perf-01).


## Automatic Ubuntu desktop updates

<a id="task-ui-update-02"></a>
- [x] **UI-UPDATE-02** Right-aligned Update chip and automatic Ubuntu
  download/verify/polkit install/restart with WB recovery. Local adapter/QML,
  package identity and native recovery checks pass.
  [Design](features/automatic-updates/design.md).

  Requirements: [UI-UPDATE-02](../requirements.md#ds-ui-update-02).

<a id="task-ui-update-qa-01"></a>
- [ ] **UI-UPDATE-QA-01** Validate a real installed upgrade, authentication
  cancellation, APT failure and automatic restart on disposable Ubuntu 22.04.

  Requirements: [UI-UPDATE-QA-01](../requirements.md#ds-ui-update-qa-01).


## Main workbench configuration and shared CI

<a id="task-ui-wb-config-01"></a>
- [x] **UI-WB-CONFIG-01** Main Configure WB with Run/Profiles tabs, explicit
  New/Open activation, subset persistence/migration and Receiver-only controls.
  Adapter/QML checks and native visual/Save As/Open verification pass.

  Requirements: [UI-WB-CONFIG-01](../requirements.md#ds-ui-wb-config-01).

<a id="task-ci-consolidate-01"></a>
- [x] **CI-CONSOLIDATE-01** Implement two entry points sharing all validation,
  parallel native tests and release-only packages; static validation passes.

  Requirements: [CI-CONSOLIDATE-01](../requirements.md#ds-ci-consolidate-01).

<a id="task-ci-desktop-apt-01"></a>
- [x] **CI-DESKTOP-APT-01** Implement strict refresh, bounded retry and CI mirror
  preference; fake-command retry tests and shell validation pass.

  Requirements: [CI-DESKTOP-APT-01](../requirements.md#ds-ci-desktop-apt-01).

<a id="task-ci-desktop-qa-01"></a>
- [ ] **CI-DESKTOP-QA-01** Verify shared hosted validation, successful APT setup,
  release packaging/installed lifecycle and publication on a new authorized tag.

  Requirements: [CI-DESKTOP-QA-01](../requirements.md#ds-ci-desktop-qa-01).


[Design](features/workbench-configuration/design.md).

<a id="task-ui-dialog-actions-01"></a>
- [x] **UI-DIALOG-ACTIONS-01** Share lower-right Apply then Cancel actions
  across all nine draft editors; simplify labels and preserve cancellation.
  [Design](features/workbench-configuration/design.md#ui-dialog-actions-01-consistent-editor-actions).

  Requirements: [UI-DIALOG-ACTIONS-01](../requirements.md#ds-ui-dialog-actions-01).


<a id="task-ui-fec-02"></a>
- [x] **UI-FEC-02** Implement the approved FEC overview, actual iteration bars,
  per-profile selection and separate control/codeword tabs. Adapter/QML checks
  and native simulation/visual checks pass; compiled Qt executable is ready.
  [Design](features/fec-monitor/design.md).

  Requirements: [UI-FEC-02](../requirements.md#ds-ui-fec-02).


<a id="task-ui-run-time-01"></a>
- [x] **UI-RUN-TIME-01** Show live/final wall elapsed time, per-profile timing,
  actual phase and recovery progress. Clock/lifecycle, adapter and QML checks
  pass; native bounded Profile3 timing/CPU sample and screenshots verified.
  [Design](features/run-elapsed-time/design.md).

  Requirements: [UI-RUN-TIME-01](../requirements.md#ds-ui-run-time-01).


<a id="task-ui-terms-02"></a>
- [x] **UI-TERMS-02** Define interleaver depth/drain in the About-linked guide
  and require technical UI definitions through the UI-design skill.
  [Design](features/run-elapsed-time/design.md#ui-terms-02-processing-terminology).

  Requirements: [UI-TERMS-02](../requirements.md#ds-ui-terms-02).


## Shared action buttons

<a id="task-ui-buttons-01"></a>
- [x] **UI-BUTTONS-01** Apply the approved shared rounded action style across
  all frames, including standard dialog and Apply/Cancel buttons. Preserve
  roles, keyboard behavior, themes and specialized diagram controls; require
  the component through the UI skill.
  [Design](features/workbench-configuration/design.md#ui-buttons-01-shared-action-buttons-across-the-desktop).

  Requirements: [UI-BUTTONS-01](../requirements.md#ds-ui-buttons-01).


<a id="task-ui-help-01"></a>
- [x] UI-HELP-01: Stable playback diagnostic with actual minimum-duration
  advice, hover/focus tooltip, keyboard/click offline FAQ and About entry.

  Requirements: [UI-HELP-01](../requirements.md#ds-ui-help-01).

<a id="task-ui-sa-details-01"></a>
- [x] UI-SA-DETAILS-01: Acquisition readout below axes; Details toggle
  preserves spectrum data, channel power and warning visibility.

  Requirements: [UI-SA-DETAILS-01](../requirements.md#ds-ui-sa-details-01).


<a id="task-ui-about-01"></a>
- [x] UI-ABOUT-01: Plain dropdown labels and compact About actions, including
  manual update checks and issue-form link; dark/light native review complete.

  Requirements: [UI-ABOUT-01](../requirements.md#ds-ui-about-01).


## Responsive Open and RxMER selection

<a id="task-ui-wb-open-02"></a>
- [x] **UI-WB-OPEN-02** Background native Open/recovery, prepared-model adoption,
  loading stage/elapsed/Cancel, no repeated GUI imports; worker failure,
  cancellation and native responsiveness verified.
  [Design](features/responsive-workbench-open/design.md).

  Requirements: [UI-WB-OPEN-02](../requirements.md#ds-ui-wb-open-02).

<a id="task-ui-rxmer-select-01"></a>
- [x] **UI-RXMER-SELECT-01** Retain loaded originals and persist selected capture
  identities; Single Shot/Replay controls and CWB-derived capacity. Verify
  selection, legacy restore, strict boundaries and native layout.
  [Design](features/rxmer-selection/design.md).

  Requirements: [UI-RXMER-SELECT-01](../requirements.md#ds-ui-rxmer-select-01).

<a id="task-ui-rxmer-anim-01"></a>
- [x] **UI-RXMER-ANIM-01** PNM Viewer-style selected-capture animation,
  Play/Pause/step/slider/speed, bounded original viewports and lifecycle cleanup.
  [Design](features/rxmer-selection/design.md#ui-rxmer-anim-01--animated-capture-preview).

  Requirements: [UI-RXMER-ANIM-01](../requirements.md#ds-ui-rxmer-anim-01).


<a id="task-ui-tx-graph-01"></a>
- [x] **UI-TX-GRAPH-01** Graphical transmitter test-point cards and full stage
  graphs, optional exact Details, role legends and shared SA navigation.
  Verified bounded real graphs, missing history and native dark/light layouts;
  54 Rust adapter tests and 53 QML checks pass.
  [Design](features/transmitter-graphs/design.md).

  Requirements: [UI-TX-GRAPH-01](../requirements.md#ds-ui-tx-graph-01).


<a id="task-ui-wb-open-03"></a>
- [x] **UI-WB-OPEN-03** Show an animated loading activity bar and distinguish
  Time since Open from file progress and simulation time; retain current phase
  and cancellation. QML behavior, native rendering and build verified.
  [Design](features/responsive-workbench-open/design.md#ui-wb-open-03--clear-loading-activity).

  Requirements: [UI-WB-OPEN-03](../requirements.md#ds-ui-wb-open-03).


<a id="task-ui-ch-icon-01"></a>
- [x] **UI-CH-ICON-01** Shared graphical Spectrum Analyzer / QAM Analyzer choices
  for every channel input, output and amplifier TapPoint. Use matching graph
  cards with titles above and no TX Output caption. Verify equal icons,
  keyboard routing and native rendering; 53 QML checks pass.
  [Design](features/channel-workbench/design.md#ui-ch-icon-01--graphical-instrument-choices).

  Requirements: [UI-CH-ICON-01](../requirements.md#ds-ui-ch-icon-01).

<a id="task-ui-fec-table-01"></a>
- [x] **UI-FEC-TABLE-01** Compact Channel / Metric / Value grid for whole-run
  control statistics, with aligned values and bounded width.
  [Design](features/fec-monitor/design.md#ui-fec-table-01--compact-control-statistics).

  Requirements: [UI-FEC-TABLE-01](../requirements.md#ds-ui-fec-table-01).


<a id="task-ui-wb-open-04"></a>
- [x] **UI-WB-OPEN-04** Hide loading timing except --debug-ui; support startup
  workbench paths and redirected stdin through bounded background validation.
  Maintain debug-control documentation and lifecycle logging. Normal/debug
  rendering, startup, Save destinations, errors and cancellation verified;
  58 Rust tests, 53 QML checks and native Ubuntu launches pass.
  [Design](features/responsive-workbench-open/design.md#ui-wb-open-04--quiet-loading-and-command-line-workbenches).

  Requirements: [UI-WB-OPEN-04](../requirements.md#ds-ui-wb-open-04).


<a id="task-ui-sa-timing-02"></a>
- [x] **UI-SA-TIMING-02** Expand the compact timing plot into a resizable full
  two-symbol and overlap view. Distinguish overlap from usable guard and label
  CP end / FFT start. Use actual core window weights, stable time axes,
  explicit result/current-configuration source and accepted-setting refresh.
  Verified both FFTs, all CP/RP pairs, keyboard/resize and native dark/light UI;
  61 Rust tests, 54 QML checks, Clippy and native build pass.
  [Design](features/symbol-timing-popup/design.md).

  Requirements: [UI-SA-TIMING-02](../requirements.md#ds-ui-sa-timing-02).


<a id="task-ui-qam-mer-01"></a>
- [x] **UI-QAM-MER-01** Reuse PNM Viewer's MER in a typed core calculator,
  show separate raw/equalized estimates over every selected retained cell,
  and reduce channel/receiver QAM marker size. Numeric/reference tests, 62
  adapter tests, 54 QML checks, core/Qt Clippy and the native build pass.
  Actual Ubuntu dark/light views verify both reference planes.
  [Design](features/qam-mer/design.md).

  Requirements: [UI-QAM-MER-01](../requirements.md#ds-ui-qam-mer-01).


<a id="task-ui-input-restore-01"></a>
- [x] **UI-INPUT-RESTORE-01** Replace demo actions with Restore Workbench in
  DPD, Channel Estimate and RxMER; restore accepted data/options without Apply.
  Remove production demo invokables; verify drafts, empty states and WB reload.
  Qt Clippy, 62 Rust tests, 55 QML checks and native build pass; actual Ubuntu
  dark/light views verify restoration from an opened WB.
  [Design](features/restore-workbench-inputs/design.md).

  Requirements: [UI-INPUT-RESTORE-01](../requirements.md#ds-ui-input-restore-01).


<a id="task-ui-qam-runs-02"></a>
- [x] **UI-QAM-RUNS-02** Add captured Run Profile selection and bounded QAM
  history, keeping wire IDs distinct. Show matched TX-reference MER alongside
  nearest-point estimates; verify channel193 impairment, transport invariance,
  archive identity/bounds, missing states, filters and native dark/light views.
  14 scoped core cases, 65 adapter tests and 55 QML checks pass, with core/Qt
  Clippy, no-default-feature MER tests and the latest native build.
  [Design](features/qam-run-results/design.md).

  Requirements: [UI-QAM-RUNS-02](../requirements.md#ds-ui-qam-runs-02).
