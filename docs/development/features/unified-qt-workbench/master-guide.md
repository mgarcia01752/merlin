# MERLIN — Unified DOCSIS OFDM/OFDMA Qt Workbench

## Master Architecture, Requirements, Reorganization and Implementation Prompt

### EXECUTION AUTHORIZATION

You are acting as the principal Qt/QML application architect, Rust integration engineer, DOCSIS PHY specialist, and software integration engineer responsible for MERLIN's unified desktop Workbench.

**Complete the authorized Qt Workbench integration end to end, following MERLIN's established engineering workflow and Agent Skills.**

The task encompasses:

1. Reviewing the current Qt application and accepted backend.
2. Updating canonical architecture.
3. Defining complete UI requirements.
4. Creating official implementation checklist tasks.
5. Producing a source ownership and migration inventory.
6. Reorganizing `apps/qt` around shared, downstream and upstream responsibilities.
7. Creating a unified tabbed application shell.
8. Preserving and adapting the existing downstream Workbench.
9. Building a new upstream OFDMA Workbench.
10. Sharing QAM, spectrum, FEC, PRBS and other compatible presentation components.
11. Implementing upstream configuration and instrument windows.
12. Integrating the accepted Rust simulation engines.
13. Preserving project-file compatibility.
14. Executing comprehensive Qt and regression testing.
15. Updating documentation and requirement traceability.
16. Automatically creating focused local Git commits after validated implementation units.

Do not repeatedly ask permission to proceed between completed tasks.

The user intends to refine visual layouts and instrumentation details as development progresses. Keep the design modular to accommodate those refinements without architectural rewrites.

---

# 1. Repository and accepted baseline

Repository:

`https://github.com/mgarcia01752/merlin`

Accepted backend integration branch:

`merge-merlin-morgana`

Backend acceptance checkpoint:

`23f63c9558ef659830338f6a61080705ce3d55f1`

The accepted backend supports:

- Downstream DOCSIS OFDM simulation.
- Upstream DOCSIS OFDMA simulation.
- Shared DSP primitives.
- Downstream and upstream FEC.
- Direction-specific Rust public APIs.
- Direction-specific native Python APIs.
- Unified CLI functionality.
- Upstream UCD, RxMER and Pre-Eq admission.
- Measured capture association and supported playback.
- Independent TX/channel/RX simulations.
- Validated local regression coverage.

The backend is authoritative for scientific processing.

Do not restart backend integration or reimplement accepted scientific algorithms.

### Initial Git checks

1. Verify repository location and active branch.
2. Inspect working-tree state.
3. Confirm the accepted backend checkpoint exists.
4. Preserve all unrelated user modifications.
5. Determine whether a Qt integration branch already exists.
6. If needed, create a dedicated feature branch from the accepted backend, following the repository Git skill.

Suggested branch name, if no branch already exists:

`feature/unified-qt-workbench`

Do not reset, recreate or overwrite an existing branch.

Do not modify or merge into `master`.

Do not push automatically.

Do not publish a release.

---

# 2. Mandatory MERLIN Agent Skills

Before any implementation, read the repository's `AGENTS.md` and relevant skills.

Required skills include:

- `skills/development-workflow/SKILL.md`
- `skills/ui-design/SKILL.md`
- `skills/coding-style/SKILL.md`
- `skills/public-api/SKILL.md`
- `skills/scientific-testing/SKILL.md`
- `skills/git-workflow/SKILL.md`
- `skills/docsis-signal-processing/SKILL.md`

Also consult DOCSIS 3.1/4.0 and PNM skills when addressing PHY behavior or measurement semantics.

Follow skill-specific conventions, including:

- Qt/QML file organization.
- Shared visual components.
- Dialog structure.
- `WorkbenchActionButton`.
- `WorkbenchDialog`.
- `ApplyCancelFooter`.
- Error handling.
- Keyboard navigation.
- Contextual help.
- Native file dialogs.
- Build validation.
- Focused commits.
- Scientific terminology.

Do not replace the established MERLIN UI design system with another framework.

---

# 3. Authoritative engineering workflow

All development follows:

**Architecture → Requirements → Checklist → Feature Design → Implementation → Validation → Focused Commit**

Canonical documents:

- `docs/architecture.md`
- `docs/requirements.md`
- `docs/development/checklist.md`

Feature documents live under:

`docs/development/features/`

Create a dedicated feature record:

`docs/development/features/unified-qt-workbench/`

Suggested contents:

```text
unified-qt-workbench/
├── architecture.md
├── requirements.md
├── design.md
├── source-inventory.json
├── migration-plan.md
├── component-ownership.md
├── compatibility-matrix.md
├── implementation-evidence.json
└── validation.md
```

These are subordinate records.

They must reference the canonical documents and must not create a competing requirements or checklist authority.

Use existing conventions for machine-readable evidence.

Preserve historical requirement and checklist identifiers.

Do not mark tasks complete before validation.

---

# 4. Comprehensive `apps/qt` audit

Before making changes, inspect the complete tracked Qt application.

The current structure includes:

```text
apps/qt/
├── Cargo.toml
├── Cargo.lock
├── build.rs
├── branding.qrc
├── qml/
│   ├── Main.qml
│   ├── about/
│   ├── components/
│   ├── inputs/
│   ├── instruments/
│   ├── shell/
│   ├── theme/
│   └── workbenches/
├── src/
│   ├── bridge/
│   ├── inputs/
│   ├── instruments/
│   ├── platform/
│   ├── simulation/
│   ├── workbench/
│   └── main.rs
└── tests/
```

The original Qt application was designed primarily for downstream OFDM.

A directory structure that mixes upstream and downstream scientific ownership must not become the permanent architecture.

### Review existing organization history

Read:

`docs/development/features/qt-source-organization/design.md`

The repository already completed:

- `UI-ORG-01`
- `UI-ORG-02`

Treat these as accepted historical work.

Extend their architecture rather than repeating the earlier reorganization.

### Build a complete file inventory

Inventory every tracked file under `apps/qt`.

For each relevant file, record:

- Original path.
- Language and file type.
- Current functionality.
- Public QML or Rust interface.
- Consumers/importers.
- Existing test coverage.
- Direction ownership.
- Shared functionality.
- Proposed destination.
- Required source changes.
- Required test changes.
- Move/keep/extract/adapt decision.
- Rationale.
- Acceptance criteria.

Classify files as:

- `SHARED`
- `DOWNSTREAM`
- `UPSTREAM`
- `PLATFORM`
- `BUILD`
- `TEST`

Where ownership is ambiguous, inspect the actual implementation rather than assigning ownership based on filenames.

Do not begin broad file moves before completing the inventory and migration design.

---

# 5. New Qt application architecture

Organize the application around three primary domains:

**SHARED — DOWNSTREAM — UPSTREAM**

The intent is to maximize reuse of compatible presentation components while keeping scientific configuration and session state independent.

### Proposed QML structure

```text
apps/qt/qml/
├── Main.qml
│
├── shared/
│   ├── shell/
│   │   ├── UnifiedWorkbench.qml
│   │   ├── WorkbenchTabs.qml
│   │   ├── WorkbenchMenuBar.qml
│   │   ├── WorkbenchStatusBar.qml
│   │   └── WorkbenchRunControls.qml
│   │
│   ├── components/
│   │   ├── WorkbenchBlock.qml
│   │   ├── Connection.qml
│   │   ├── WorkbenchDialog.qml
│   │   ├── WorkbenchActionButton.qml
│   │   ├── ApplyCancelFooter.qml
│   │   └── other shared controls
│   │
│   ├── instruments/
│   │   ├── qam/
│   │   ├── spectrum/
│   │   ├── channel/
│   │   ├── fec/
│   │   ├── prbs/
│   │   ├── timing/
│   │   └── plots/
│   │
│   ├── inputs/
│   ├── theme/
│   └── about/
│
├── downstream/
│   ├── DownstreamWorkbench.qml
│   ├── configuration/
│   ├── transmitter/
│   ├── channel/
│   ├── receiver/
│   ├── measurements/
│   └── instruments/
│
└── upstream/
    ├── UpstreamWorkbench.qml
    ├── configuration/
    ├── transmitter/
    ├── channel/
    ├── receiver/
    ├── measurements/
    └── instruments/
```

This is a logical target, not permission to create unused directories or duplicate existing files.

Retain established component names where compatible.

### Proposed Rust structure

```text
apps/qt/src/
├── main.rs
│
├── bridge/
│   ├── mod.rs
│   ├── existing QObject registrations
│   └── new QObject registrations
│
├── shared/
│   ├── instruments/
│   ├── inputs/
│   ├── workbench/
│   └── presentation/
│
├── downstream/
│   ├── inputs/
│   ├── instruments/
│   ├── simulation/
│   └── workbench/
│
├── upstream/
│   ├── inputs/
│   ├── instruments/
│   ├── simulation/
│   └── workbench/
│
└── platform/
```

Use cohesive modules.

Avoid duplicating simulation logic, UI state machines or scientific calculations.

Do not create universal data models merely because two directions have similarly named operations.

---

# 6. Critical CXX-Qt organization constraint

MERLIN currently uses CXX-Qt 0.7.3.

The accepted source-organization design documents that registered QObject bridge files for the existing QML module must reside in the same directory.

Preserve this constraint unless actual build evidence demonstrates a supported alternative.

Therefore:

- Keep registered QObject entry files together in `src/bridge/`.
- Organize backend adapter implementations under shared/downstream/upstream modules.
- Do not blindly relocate `#[cxx_qt::bridge]` files into direction subdirectories.
- Preserve the `Merlin.Workbench` QML module.
- Preserve the native application entry point and resource URL.
- Update `build.rs` explicitly when QML registrations change.

The application must remain one native Qt executable.

Do not create separate downstream and upstream Qt applications.

---

# 7. Shared presentation component strategy

This section is especially important.

MERLIN already has extensive scientific visualization functionality.

Do not duplicate these implementations merely to support OFDMA.

### 7.1 QAM and constellation instruments

Existing examples include:

- `ReceiverQamDialog.qml`
- `TransmitterQamDialog.qml`
- `TransmitterScatter.qml`
- `TapQamDialog.qml`
- `ReceiverCellControls.qml`

Separate reusable plotting primitives from direction-specific scientific semantics.

Share:

- Scatter-point rendering.
- Axes and grids.
- Zoom and pan.
- Marker selection.
- Color legends.
- Point inspection.
- Reference constellation overlays.
- Theme and visual controls.

Keep direction-specific:

- Received and transmitted sample ownership.
- Subcarrier or resource mapping.
- Modulation profile selection.
- Bit labeling.
- LLR attribution.
- Pilot/data interpretation.
- Error-vector calculations.

Do not calculate scientific MER or FEC outcomes independently in QML.

The shared QAM component receives validated, bounded plotting data from Rust.

### 7.2 Spectrum instruments

Existing components include:

- `SpectrumAnalyzerDialog.qml`
- `SpectrumPlot.qml`
- `SymbolTimingDialog.qml`
- `SymbolTimingPlot.qml`

Share:

- Spectrum plotting.
- Zoom/navigation.
- Frequency axes.
- Markers and cursors.
- Overlays.
- Legend presentation.
- Bounded data selection.

Preserve direction-specific spectrum interpretation.

Downstream PLC/NCP display logic must not be incorrectly reused for upstream OFDMA.

Upstream grant/resource boundaries should be displayed through upstream-specific overlays.

### 7.3 Channel visualization

Share graphical presentation for:

- Channel frequency response.
- Magnitude and phase.
- Impulse response.
- Group delay.
- Time-series response.
- Channel-model comparison.

Keep channel models and reference-plane interpretation direction-specific.

The downstream measured channel response is not interchangeable with upstream CM Pre-Eq.

Preserve the distinction between physical H, applied correction C and effective C×H.

### 7.4 FEC instruments

Existing components include:

- `ReceiverFecDialog.qml`
- `FecCodewordTable.qml`
- `FecControlTable.qml`
- `FecIterationChart.qml`
- `FecAllocationChart.qml`

Share presentation where scientific semantics genuinely match:

- Codeword outcome tables.
- Error counters.
- Decoder iterations.
- Success/failure states.
- Partial recovery.
- Bounded metric charts.

Keep direction-specific:

- Code families.
- Codeword allocations.
- Shortening and puncturing.
- Fill behavior.
- Normative mapping.
- Report interpretation.

Avoid displaying BCH stages where the corresponding upstream configuration does not actually use them.

### 7.5 PRBS and verification

Share:

- PRBS source controls.
- Seed presentation.
- Continuous-source state.
- Verification status.
- Progress presentation.
- Source comparison charts.

Keep source generation and independent verification in their respective Rust engines.

Do not reuse original source truth inside a normal receiver pipeline.

### 7.6 Measurement and PNM presentation

Share:

- Capture inventory views.
- Original-file metadata presentation.
- Time selection.
- Validated measurement plots.
- File selection.
- Coverage and missing-data presentation.
- Provenance details.

Keep direction-specific decoding, admission and scientific transformations in Rust.

### Shared-component acceptance

A component may be classified as shared only when:

1. Both directions have a genuine use for it.
2. The shared interface is well-defined.
3. It does not depend on a downstream-specific scientific model.
4. Its existing behavior is preserved.
5. Both directions have appropriate tests.

Do not prematurely abstract a component that has only one actual consumer.

---

# 8. Unified application shell

Retain the existing top menu:

- File
- Settings
- About

Retain update notifications and application-level functionality.

Remove the large internal MERLIN branding/title section from the Workbench canvas.

Use the available space for the block diagram.

Provide two large, persistent navigation tabs directly below the top menu:

**Downstream (OFDM)**

**Upstream (OFDMA)**

These must be real tabs, not dropdown controls.

Preserve MERLIN's existing dark/light theme behavior.

The selected tab determines which direction's Workbench is active.

### Tab-state rules

Maintain separate:

- Configuration.
- Loaded inputs.
- Simulation sessions.
- Run progress.
- Error state.
- Selected instruments.
- Unsaved-change status.

Switching tabs must not silently discard or overwrite state.

Running simulations must not be cancelled merely because the user changes tabs.

If concurrent execution is unsupported, enforce the real backend constraint and provide an accurate UI status.

Do not invent concurrency support.

### Shared shell controls

Continue using the existing application-wide interaction model:

- Configure Workbench.
- Start.
- Stop.
- Pause/resume where supported.
- Open/save projects.
- Status and progress information.
- Contextual help.

Actions must apply to the correct direction.

---

# 9. Preserve the downstream Workbench

The existing downstream block diagram and its sub-blocks are the functional baseline.

Retain the existing downstream workflow:

- OCD.
- DPD.
- PRBS-TX.
- Transmitter.
- RxMER.
- Channel Simulator.
- DS Channel Estimation.
- Receiver.
- PRBS-RX.

Preserve:

- Existing drill-in dialogs.
- Configuration behavior.
- Accepted state.
- Simulation controls.
- Native QAM and spectrum instruments.
- Measurement workflows.
- Project save/reopen.
- Update recovery.
- Accessibility.
- Existing test identifiers where possible.

### Downstream migration policy

Do not rebuild a working downstream dialog from scratch.

Use `git mv` where appropriate to preserve history.

Make only necessary adaptations for:

- The unified shell.
- The accepted backend API.
- Shared-component extraction.
- New source ownership.
- Updated QML registrations.
- Consistent visual layout.

Downstream regression tests must pass before declaring migration complete.

---

# 10. Upstream OFDMA startup Workbench

The user will supply the proposed upstream diagram.

The upstream startup view must use the same minimal block-diagram paradigm as downstream.

It must not become a crowded dashboard.

### CMTS configuration

The CMTS configuration section owns:

- UCD.
- OFDMA numerology.
- IUC profiles.
- MAP/grants.
- Resource allocation.
- Pilot-pattern configuration.
- Interleaving parameters.

These represent control/configuration of the upstream PHY.

They are not transmitted user-data sample stages.

### Physical upstream signal path

The correct physical sample flow is:

**CM Transmitter → Channel Simulator → CMTS Receiver**

The UI may show a CMTS configuration/control block separately, but it must not falsely imply that the CMTS transmits the upstream data waveform.

The layout can preserve the user's preferred CMTS-centric arrangement while clearly distinguishing configuration/control from the sample path.

### Pre-equalization

The user wants CM Pre-Eq visibly associated with the channel simulator.

Represent:

- Measured CM Pre-Eq coefficients.
- CM transmit-side correction.
- Physical channel.
- Effective channel after pre-equalization.
- CMTS receive estimation/equalization.

The startup diagram may simplify the relationship, but the actual engine must retain correct reference planes.

Do not implement CM Pre-Eq as a fictitious CMTS-side transmitted waveform.

### RxMER

The upstream RxMER PNM capture must feed the Channel Simulator's supported impairment model.

Keep the measurement source associated with the CMTS.

Do not confuse RxMER with CM transmit Pre-Eq or independent channel-estimation coefficients.

Apply the backend's strict compatibility and admission rules.

### Receiver

CMTS-side receiver processing includes:

- Framing.
- Timing.
- FFT.
- Pilot estimation.
- Equalization.
- Resource extraction.
- Soft demapping.
- LDPC decoding.
- Source recovery and reporting.

Do not assign these operations to the CM.

---

# 11. Upstream drill-in windows

Create new upstream windows following the existing downstream Workbench paradigm.

Use shared presentation components whenever appropriate.

### CMTS configuration window

Include:

- UCD.
- Numerology.
- IUC configuration.
- Grants.
- Resource plans.
- Pilot-pattern selection.

### CM transmitter window

Include:

- Source/PRBS.
- Coding and QAM.
- Resource-grid visualization.
- Pilot allocation.
- Interleaving.
- Waveform and spectrum instruments.

### Channel simulator window

Include:

- Supported channel impairments.
- AWGN.
- FIR/echo.
- Measured RxMER.
- Applied Pre-Eq context.
- Physical and effective response plots.

Respect declared backend restrictions.

### CMTS receiver window

Include:

- Received waveform.
- Timing/framing.
- Channel estimation.
- Equalization.
- Constellation.
- Soft decisions.
- LDPC outcomes.
- Recovery status.

### PNM input window

Include:

- UCD captures.
- Upstream RxMER.
- CM Pre-Eq captures.
- Capture association.
- Provenance.
- Coverage and validation errors.

### Verification window

Include:

- Source comparison.
- BER.
- Codeword recovery.
- Decoder iterations.
- Partial-result status.
- Simulation observations.

Only display outcomes the backend actually calculates.

---

# 12. OFDMA resource-grid viewer

The resource-grid viewer is a priority upstream instrument.

Provide a frequency-versus-symbol visualization.

X-axis:

OFDMA symbol index.

Y-axis:

Physical subcarrier index.

Distinguish:

- Data resource elements.
- Pilot elements.
- Guard/excluded resources.
- Unassigned elements where applicable.

Support all pilot patterns currently accepted by the Rust engine.

The actual resource plan must originate in Rust.

QML renders the result; it must not independently implement pilot/resource placement.

Allow:

- Zoom and pan.
- Pattern selection.
- Resource-element inspection.
- Grant highlighting.
- IUC context.
- Resource counts.
- Bounded viewport data.

Ensure efficient rendering for large OFDMA grids.

Do not create a separate QML rectangle object for every resource element when the data size would compromise performance.

Use an appropriate batched drawing strategy and verify it under the target Qt version.

---

# 13. Qt-to-Rust integration

The accepted backend exposes:

```rust
merlin::downstream
merlin::upstream
merlin::common
```

Use these public APIs or their validated underlying owners as appropriate.

Do not implement an additional simulation engine inside the Qt application.

### Direction-specific adapters

Each direction owns its:

- Configuration admission.
- Session lifecycle.
- Simulation dispatch.
- Data extraction.
- Report mapping.
- Scientific errors.

### Shared Qt presentation contracts

Share only compatible concepts such as:

- Bounded chart data.
- Generic plot axes.
- UI status.
- Progress presentation.
- Display metadata.
- Common error presentation.

Avoid universal configurations or reports that erase important differences.

### Concurrency

Heavy processing must not block the UI thread.

Preserve:

- Cancellation.
- Busy protection.
- Resource limits.
- Partial outcomes.
- Progress observations.
- Ownership and cleanup.

Do not invent unsupported pause/resume semantics.

---

# 14. Project files and compatibility

The current Qt project format is implemented in:

`apps/qt/src/workbench/project_file.rs`

The existing `Project` includes downstream-specific transmission settings.

Design a versioned direction-aware project format.

Requirements:

1. Preserve accepted legacy downstream files.
2. Preserve embedded original captures and hashes.
3. Add explicit direction metadata.
4. Support upstream configuration and measured inputs.
5. Preserve save/reopen equality.
6. Reject incompatible directions.
7. Prevent accidental overwrite of unsaved work.
8. Maintain update-recovery behavior.
9. Validate limits and malformed input.
10. Provide meaningful migration errors.

Do not silently reinterpret old downstream projects as upstream.

Do not change project serialization without corresponding regression fixtures.

---

# 15. Migration order

Perform implementation in the following dependency order.

### Phase 1 — Documentation and baseline

- Inspect the Qt tree.
- Establish a verified Qt regression baseline.
- Update architecture.
- Add stable UI requirements.
- Create the official checklist.
- Produce a full source ownership inventory.
- Finalize the migration design.
- Commit validated documentation.

### Phase 2 — Shared component organization

- Identify genuinely reusable UI primitives.
- Move selected shared QML components.
- Extract shared Rust presentation helpers.
- Update build registrations.
- Preserve downstream behavior.
- Test and commit.

### Phase 3 — Downstream organization

- Organize downstream-specific files.
- Preserve existing interfaces.
- Fix imports/resources.
- Run downstream QML and Rust tests.
- Commit.

### Phase 4 — Unified tabbed shell

- Add direction tabs.
- Create separate direction containers.
- Preserve menu, theme and status controls.
- Mount the downstream Workbench.
- Validate project/run state.
- Commit.

### Phase 5 — Upstream startup diagram

- Create the upstream block diagram.
- Implement functional block navigation.
- Integrate truthful readiness/status information.
- Validate Qt behavior.
- Commit.

### Phase 6 — Upstream configuration and execution

- Connect the Qt layer to upstream Rust configuration and sessions.
- Support validated start/stop/progress behavior.
- Implement real upstream configuration dialogs.
- Test and commit.

### Phase 7 — Shared scientific instruments

- Integrate reusable QAM.
- Integrate spectrum.
- Integrate channel visualization.
- Integrate FEC displays.
- Integrate PRBS presentation.
- Validate both directions.
- Commit each coherent instrument unit.

### Phase 8 — Upstream PNM and advanced instruments

- Implement measured capture admission views.
- Implement Pre-Eq plots.
- Integrate RxMER presentation.
- Implement resource-grid/pilot viewer.
- Integrate receiver observations.
- Validate and commit.

### Phase 9 — File persistence and final integration

- Complete direction-aware persistence.
- Verify legacy compatibility.
- Verify native lifecycle behavior.
- Test packaging.
- Complete documentation and traceability.
- Commit.

### Phase 10 — Final acceptance

- Run applicable complete Qt and backend integration regressions.
- Verify all accepted UI requirements.
- Verify native application startup.
- Validate dark/light UI.
- Review screenshot consistency.
- Complete the official checklist.
- Produce the final acceptance record.
- Commit.

Do not start a new implementation phase before satisfying its actual prerequisite acceptance gates.

---

# 16. Validation requirements

At minimum, cover:

### Rust

- Qt adapter unit tests.
- Shared visualization adapter tests.
- Downstream simulation integration.
- Upstream simulation integration.
- Project format validation.
- Formatting.
- Clippy.

### QML

- Component imports.
- QML module registration.
- Diagram rendering.
- Tab switching.
- Keyboard navigation.
- Focus states.
- Dialog opening/closing.
- Status changes.
- Responsive layouts.
- Dark/light themes.
- Native startup.

### Scientific presentation

- Actual upstream resource coordinates.
- Correct PHY direction.
- Pre-Eq reference planes.
- Measured RxMER restrictions.
- QAM point ownership.
- Spectrum normalization.
- FEC/report mappings.
- Partial results.
- Provenance.

### Compatibility

- Historical downstream project files.
- Existing saved data.
- Downstream run behavior.
- Existing UI tests.
- Existing CLI and Python API contracts where touched.

Do not modify scientific fixtures merely to make tests pass.

Do not claim execution for tests that were not run.

---

# 17. Automatic focused Git commits

The user explicitly authorizes focused local Git commits for this integration.

Follow:

`skills/git-workflow/SKILL.md`

For each independently completed and validated requirement or checklist implementation unit, execute:

```bash
tools/git/git-save.sh --commit-msg \
"Feature: Implement unified Qt tabs
- Preserve downstream Workbench behavior
- Add direction-aware Qt navigation
- Tests: Qt/QML and adapter checks"
```

The example is illustrative.

Codex must generate each commit message from the actual completed work.

Follow the repository's 60-character-per-line commit-message convention.

The existing helper stages all non-ignored changes, so inspect the full working tree before invoking it.

Do not include unrelated user changes.

Do not commit private captures, caches, secrets or unfinished unrelated functionality.

After a successful focused commit, continue automatically to the next eligible task.

Do not stop to request routine approval between commits.

Do not automatically push, merge or release.

---

# 18. Final acceptance conditions

The unified Qt application is complete only when:

1. Canonical architecture is current.
2. UI requirements are complete and traceable.
3. Official UI checklist tasks are accepted.
4. `apps/qt` uses coherent ownership boundaries.
5. Shared components have validated contracts.
6. Existing downstream interfaces work.
7. Downstream project files remain compatible.
8. Downstream and upstream tabs are functional.
9. Upstream block navigation is operational.
10. Upstream configuration uses the real Rust engine.
11. Upstream simulation executes through Qt.
12. Shared QAM and spectrum presentation is reused.
13. FEC, PRBS and channel visualizations are reused appropriately.
14. Upstream pilot/resource-grid data comes from Rust.
15. Pre-Eq and RxMER use correct reference planes.
16. Qt remains responsive during simulation.
17. Cancellation and partial results work correctly.
18. Project persistence is direction-aware.
19. Applicable Qt/QML regression tests pass.
20. Relevant scientific/backend regression checks pass.
21. Documentation matches actual functionality.
22. Every accepted UI task has implementation/test evidence.
23. Focused local commits exist for completed integration units.
24. The application remains one native MERLIN Qt executable.
25. No unauthorized merge, push or release occurred.

---

# 19. Final engineering report

At completion, provide:

### Architecture

- Final Qt directory tree.
- Shared/downstream/upstream ownership.
- QML dependency structure.
- Rust QObject/adapter ownership.
- Reusable instrument contract map.

### Requirements and checklist

- UI requirements implemented.
- Checklist tasks accepted.
- Implementation links.
- Validation evidence.
- Explicit deferrals.

### Functional results

- Downstream compatibility.
- Upstream simulation.
- Instrument availability.
- Project persistence.
- UI navigation.
- Screenshots or actual render checks where available.

### Validation

- Rust tests.
- Qt/QML tests.
- Native startup checks.
- Scientific presentation parity.
- Build and packaging checks.
- Known limitations.

### Focused commits

Provide a chronological table of actual commit SHAs, messages, checklist tasks and validation results.

---

# FINAL DIRECTIVE

**Complete the unified MERLIN Qt/QML Workbench modernization.**

Start with architecture, requirements, official checklist and the complete `apps/qt` source-ownership audit.

Reorganize the application around genuinely shared presentation infrastructure and separate downstream/upstream scientific responsibilities.

Preserve the current downstream Workbench and its validated sub-blocks.

Implement the persistent Downstream/Upstream tabbed shell.

Create new upstream OFDMA Workbench windows using the same block-based interaction model.

Reuse compatible QAM, spectrum, channel, FEC, PRBS and measurement visualization components.

Keep the accepted Rust backend authoritative for all scientific operations.

Implement in dependency order, validate each completed unit, update documentation and traceability, and automatically create focused commits through `tools/git/git-save.sh --commit-msg`.

Continue without repeated routine approvals.

Do not push, merge, publish or release without separate authorization.