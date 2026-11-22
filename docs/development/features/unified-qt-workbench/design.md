# Unified Qt Workbench design

Status: accepted planning baseline; continuous task execution is now authorized.
The [execution authorization](execution-authorization.md) records the user scope.
The accepted choices are one active simulation across both tabs initially and
Save for the active direction's project. These choices do not couple the engines.

The branch is `feature/unified-qt-workbench`, created from accepted backend
`23f63c9558ef659830338f6a61080705ce3d55f1`. Completed backend, FEC and UI-ORG-01/02
tasks retain their acceptance states. This is a separate UI delivery scope;
INT-DEFER-01's wider PHY deferrals remain in force.

## Authority and navigation

- [Canonical architecture](../../../architecture.md#unified-qt-workbench)
- [Canonical requirements](../../../requirements.md#unified-qt-workbench-requirements)
- [Official tasks](../../checklist.md#unified-qt-workbench-delivery)
- [Complete source inventory](source-inventory.json)
- [Migration and ownership plan](migration-plan.md)
- [API gap and parity matrix](api-gap-matrix.md)
- [Compatibility matrix](compatibility-matrix.md)
- [Validation and acceptance plan](validation.md)
- [Original master guide](master-guide.md)
- [Additional high-level API contract](high-level-api-contract.md)

The feature files refine the three canonical documents. They are not alternative
requirements or completion registers. Proposed destination files are written as
code; current source links resolve to files inspected at this checkpoint.

## Decisions and implementation gates

One native executable and `Merlin.Workbench` module serve two persistent direction
tabs. Each direction owns its accepted configuration, inputs, session, worker
control, progress, immutable results, selected instruments, project path and dirty
state. A shell scheduler admits one active run across both tabs. Switching tabs
does not stop it; Stop/Pause target the active tab's own session, and the shell
identifies a run in the other tab without redirecting these controls silently.
Start in another tab reports the existing active run. A paused run still holds
the execution slot until completion or cancellation.

Future concurrent operation requires a dispatch-policy change, aggregate CPU and
memory admission, and concurrent isolation tests. It must not require merging
scientific configurations or replacing direction-specific public APIs. In
particular, the current thread-local latest-result and accepted-input stores
must become explicit session-owned state before two-tab execution is accepted.

Save/Save As write the active direction's project. Open detects and validates the
direction before applying it to the corresponding tab, then asks about that
tab's unsaved changes. Legacy schema 1/2 files remain downstream. Cancelling or
rejecting Open preserves both tabs. Closing the app handles both dirty states.
Updater recovery preserves both independently valid projects plus presentation
state; normal Save does not become a combined-workspace format.

The upstream logical flow is fixed: CM transmitter → physical channel → CMTS
receiver. UCD/IUC/MAP configuration is a control path. Pre-equalization C applies
at the CM transmitter; physical H and effective C×H keep their reference planes.
CMTS RxMER feeds only the backend's admitted residual model. The user's upstream
diagram is still needed to finalize visual placement; it does not block API,
inventory, boundary or downstream planning. Do not invent final visual approval.

No application source moves or API additions belong to the planning task.
Following planning acceptance, take the next eligible official task, establish a fresh
Qt baseline, then enforce the migration boundary before extending APIs and moving
their consumers. API extensions are focused, independently validated units before
Qt adoption. The [migration plan](migration-plan.md#dependency-order) refines the
master phase order where these prerequisites require it.

## Reuse review

### UIU-BASE-01: fresh downstream baseline

Reuse the accepted application and existing Qt test owners without changing
source or fixtures. Inputs are the pinned source inventory and four curated
workbenches; outputs are an exact pre-migration test/build/native-startup record.
Run `scripts/qt-desktop.sh check` with existing Rust 1.85.1 and Qt 6.8.3, retain
caches/profiles, then validate all four files and launch the fresh executable
both empty and with the single-channel project. Native checks use isolated
preferences and the current desktop display. Existing adapter tests exercise
actual bounded downstream runs; startup smoke does not claim manual UI execution.
The [baseline evidence](baseline-evidence.json) records all 87 Rust cases,
89 QML checks, six native checks and unchanged hashes for all 178 Qt files.

### UIU-BOUND-01: enforce scientific dependency ownership

Add `tools/validation/qt_api_boundary.py` and independent negative fixtures in
`tools/validation/test_qt_api_boundary.py`. Reuse the existing Rust comment/literal
masker from `implementation_evidence.py`. Cargo's locked no-dependencies metadata
identifies actual direct package names, renames, kinds and targets; local manifest
identity additionally detects a scientific package hidden behind a path alias.
Transitive scientific dependencies through the facade remain valid. Unknown
direct packages fail closed; known infrastructure packages have explicit
non-simulation role justifications in the guard.

The reviewed [legacy boundary record](legacy-qt-boundary.json) freezes existing scientific dependency
records and whole-file source hashes, with exact root-reference sites and sunset
task UIU-DS-01. Freezing the entire legacy file also detects a new invocation of
an already imported alias. It is deliberately stricter than import-line matching:
even presentation edits in such files require a focused migration/review update.
There is no automatic refresh flag. New direct scientific access is rejected;
final mode rejects every remaining exception. Source masking is a bounded lexical
check, not proof that arbitrary handwritten numerical code is presentation-only.
Manual scientific-owner review and API parity remain acceptance requirements.

Wire the check into Qt's existing check action and Master preflight, without a
Qt SDK requirement for the guard itself. Negative fixtures cover Cargo renames,
local paths, dev/target dependencies, qualified calls, extern aliases, mutation of
frozen legacy files, escaped module includes, stale exceptions and final mode.
No API or application implementation changes belong to this task.

### UIU-API-01: typed transmission and source admission

Add `crates/ofdm/src/api/admission.rs`, exported through the existing API/facade.
`TransmissionConfiguration::new(TransmissionSettings)` validates and owns an
immutable settings snapshot; `from_json(&[u8])` admits the same bounded 4096-byte
editor transport. `settings()` borrows accepted data, `timing(ObservationDuration)`
delegates to existing integer-clock timing, and `check_profile(&Dpd)` preserves
the current editor's spacing/envelope admission without claiming TX readiness.
No constructor starts a stream or changes a configured Simulation.

`SourceInput::prbs15(PrbsSeed)` and
`SourceInput::file(NamedCapture, Option<FileFillPolicy>)` own validated passive
source choices. FileFillPolicy contains target bits and checked independent fill
seed; existing FilePayload owns size/target admission. Read-only mode/seed/target/
name/hash/original-byte accessors support editors without exposing generators.
Actual source/verification execution remains API-02 work. PRBS seed and finite
file-fill seed keep their different meanings.

Expose matching immutable native Python TransmissionConfiguration, SourceInput
and FileFillPolicy classes through preserved root/compatibility/downstream
imports and stubs. Python converts arguments and returns Rust timing/metadata;
it does no timing, source generation or validation calculations. Rust and native
tests cover literal timing anchors, incompatible profiles, malformed/oversized
settings, arbitrary original bytes, empty files, fill bounds, bad seeds and
retained ownership. These operations are shared domain admission, not a second
Qt simulation interface. Further measurement/channel/Designer previews remain
open until their own typed API operations pass.

### UIU-API-01: native-input facade implementation unit

Reuse native admission rather than duplicate decoding or project validation.
Add optional facade feature `native-input`, enabling both direction namespaces
and the existing `pnm-input` adapter. It intentionally brings both PHY adapters;
base facade features and independent PHY crates retain their previous graph.
The facade contains only re-exports:

- `merlin::downstream::inputs::{CaptureLoader, Workbench, read_capture, save_json}`
  and `merlin::downstream::inputs::rxmer_export` retain their exact existing types,
  signatures, byte/work limits, errors and non-overwrite contracts.
- `merlin::upstream::inputs` exposes the existing native UCD/RxMER/Pre-Eq and
  measured-association contracts; it does not expose receiver/DSP implementations.

Edit `crates/merlin/Cargo.toml` and `src/lib.rs`; add facade-only consumer tests in
`crates/merlin/tests/native_inputs.rs`. Native Python enables the feature and
changes its existing admission imports to these facade paths without changing
public Python methods, defaults or classes. Remove the now-redundant direct
binding dependency only after all references are routed. Update the two affected
lockfiles without changing versions/pins. Qt remains unchanged until the complete
input/preview gap is accepted.

Validate binary and decoded DS identity/grids/provenance, idle WB loading, US
capture bytes and structured rejection via facade-only consumers, independent
feature checks, matching Python format/Clippy and fresh installed-wheel namespace,
input/workbench/Designer/export tests. This native-input unit alone does not
complete UIU-API-01: missing typed editor/idle-preview operations remain in the
API gap matrix until separately implemented and validated.

### UIU-API-01: original measurement preview unit

Reuse `scenario::measurement_view::{measurement_view,response_value}` unchanged.
Add `crates/ofdm/src/api/measurement.rs`; export `MeasurementViewport` (zero count
means the complete original grid, matching the existing viewer),
`MeasurementPreview` (bounded view, original grid/provenance and explicit
projection), `ChannelResponseMetric` and existing bounded view output models.
`RxMer::preview(MeasurementViewport)` and
`ChannelEstimate::preview(MeasurementViewport, ChannelResponseMetric)` return
`ApiResult<MeasurementPreview>` without changing captures, simulation state or RNG.
Response projections are original H gain dB, wrapped phase degrees, real or
imaginary, never inverse equalizer gain. Missing values and zero-magnitude gain
remain gaps. Existing owner validates checked viewport arithmetic, grid/value
limits and finite ranges; output stays at most 1024 min/max groups.

Edit native Python `inputs.rs` and root stub to expose `RxMer.preview(start=0,
count=0)` and `ChannelEstimate.preview(metric="gain_db", start=0, count=0)` as
owned dictionaries with the exact same fields. Python releases the GIL for Rust
projection/view construction and performs no numerical calculations. Unknown
metric names raise InvalidInput; typed Rust metric variants require no parsing.
Root/downstream/legacy capture class identity remains unchanged.

Add `crates/ofdm/tests/api_measurement_preview.rs` and
`bindings/python/tests/test_measurement_preview.py` with independent Rust and
installed-wheel Python cases for literal original
frequency centers, finite/gap buckets, zero/imaginary/negative H, phase and dB
units, full-grid bound, overflow/window rejection, provenance and unchanged
original bytes after success/failure. Reuse prior lower-owner tests; no GPL Qt
code or new scientific algorithm moves into Apache owners. This unit closes only
the original-capture preview gap; collection combination/playback, Designer,
channel composition and idle TX remain open before full UIU-API-01 acceptance.

### UIU-API-01: measured sequence admission unit

Reuse `simulation::channel::measurement_playback::{MeasurementPlayback,
validate_response}` for exact-grid/epoch/role/response admission and Single,
Average, Hold and Interpolate residual-power semantics. Add
`crates/ofdm/src/api/sequence.rs` and focused `sequence/queries.rs`, explicitly
export existing PlaybackMode as
`MeasurementMode` and typed `MeasurementSequenceOptions { mode,
interval_symbols, tolerance }`. `MeasurementSequence::new(&Dpd,
&TransmissionConfiguration, Vec<RxMer>, MeasurementSequenceOptions)` retains the
caller's original order and rejects duplicate original hashes, more than 256
captures/4,000,000 values/96 MiB original bytes before frame cloning.

The immutable sequence exposes `info()`, `capture(index)`, `capacity(duration)`,
`at(symbol)` and `preview(symbol, viewport)`. Info reports exact synthetic symbol
cadence/span; original epochs remain in every provenance. `at` returns dimensionless
normalized residual powers and all actual contributor indexes/weights/provenance:
average names every equal-weight source, interpolation names its actual endpoints,
static/held positions name one. Preview reports bounded equivalent RxMER dB,
explicitly distinct from original measured RxMER. Add a small finite/gap-preserving
power-to-MER conversion in existing `signal_processing/noise/residual.rs`; no
averaging or timeline algorithm moves into the adapter or Qt.

`TransmissionConfiguration::check_response(profile, response, tolerance)` reuses
existing response admission with explicit unity None and original epoch tolerance.
It creates no run or inferred measurement. Sequence construction/queries never
advance source/RNG state. Rejected admissions preserve existing captures/sequences.

Add native `bindings/python/src/sequence.rs`, aliases/stubs and
`bindings/python/tests/test_measurement_sequence.py`. Python constructor accepts
profile, transmission, capture sequence, typed MeasurementSequenceOptions (mode, interval_symbols and tolerance);
mode names are single/average/hold/interpolate. It converts to the same Rust
options, releases the GIL for admission/queries and returns owned dictionaries or
existing capture snapshots. Add `crates/ofdm/tests/api_measurement_sequence.rs`
with independent equal-power/half-step/last-hold anchors, clock literals,
null/coverage/epoch/grid/duplicate/count/overflow and ownership cases. Run matching
installed-wheel cases and existing playback tests before Qt adoption. Designer,
channel composition and idle TX gaps remain open.

### UIU-API-01: immutable RxMER Designer admission unit

Reuse the existing Apache native `workbench/designer.rs::load` and core
`RxmerRecipe` evaluation unchanged. Add focused
`crates/pnm-input/src/workbench/designer/admission.rs`; re-export `RxMerDesign`,
`RxMerDesignPosition` and curated existing recipe configuration types through
`pnm-input` and `merlin::downstream::inputs`. The facade still adds no science.
Add `Dpd::key()` and matching Python DPD.key for exact original association;
curated ScenarioKey/FrequencyGrid type re-exports preserve nominal ownership.
Expose exact existing `RxMer::source_kind`/`ChannelEstimate::source_kind` metadata
and matching Python properties so ordinary, designed and synthetic observations
keep their backend origin labels; do not infer origin from filenames.

`RxMerDesign::from_bytes(&NamedCapture)` validates a strict version-1 Designer
envelope and preserves exact original bytes/whitespace. Typed
`flat(&Dpd, MerDb, CaptureTime, RxmerRecipe, RxMerDesignPosition)` creates the
existing flat-baseline schema from the profile's original roles/grid;
`imported(&Dpd, &NamedCapture, RxmerRecipe, RxMerDesignPosition)` embeds exact
ordinary capture bytes and checks spatial identity. `check_profile(&Dpd)`
validates an independently loaded design before editor adoption. Position is
one-based inside 1..256, with existing count-times-grid <=4,000,000 and final
16 MiB envelope bounds. Imported baselines keep integrity checks, reject nested
recipes and reject flat-only random variation; no input is overwritten.

The immutable design exposes `materialize()` (owned RxMer), `original_bytes()`,
`recipe()`, `position()`, `is_synthetic()` and `with_recipe(RxmerRecipe)`.
Replacement always evaluates the retained original baseline, preventing compound
edits; it never advances a simulation source/noise stream. Original loaded
whitespace is retained until an explicit recipe replacement creates a new record.
Configuration type re-exports do not authorize direct recipe-kernel calls from Qt;
Qt consumers use these validated design operations.

Add `bindings/python/src/designer.rs`, aliases and stubs. Python exposes the
same immutable design/position classes; flat/imported/with_recipe accept bounded
UTF-8 recipe JSON bytes as configuration transport, while Rust retains a typed
recipe route. All evaluation/provenance/version checks remain in Rust with GIL
release. Add `crates/pnm-input/tests/designer_admission.rs` and
`bindings/python/tests/test_designer_admission.py`: exact saved fixture bytes,
role masks/epochs, literal tilt endpoints, noncompounding replacement, frozen
positions, malformed/version/nesting/random/work-limit rejection and owned
results. Re-run existing Designer numerical and saved-WB actual FEC parity tests
and fresh installed-wheel acceptance before Qt adoption. Channel composition,
profile scientific summaries and idle TX preview remain open afterward.

### UIU-API-01: profile capture inspection unit

The Qt inspector uses decoded compact source schemes, original u8 profile IDs,
per-role loadings, an uncoded data-capacity estimate and ideal Shannon bounds.
The existing simulation `ProfileId` remains 0..15. Preserve both contracts:
new read-only `ProfileCapture` retains the complete encoded source catalog,
including byte-sized IDs that cannot be passed to the legacy simulation API.
Do not widen that quantity, discard source IDs or infer simulation acceptance.
UIU-API-02 must reconcile actual full-loop source-ID behavior before Qt migration.

Move the existing Apache standalone profile parser into focused
`crates/ofdm/src/scenario/pnm_json/profiles.rs`. Keep `import_profiles` as its
compatibility entry and preserve validation order/messages, limits, assignment
roles, source grids, hashes and epochs. The shared parser optionally retains
validated compact schemes/header metadata for inspection; ordinary import callers
must not allocate inspection arrays. Add curated serialized metadata models in
`profile_metadata.rs`; reuse `ModulationAssignment` and `minimum_snr_db` unchanged,
including PLC data/preamble distinction, unknown/reserved codes and zero loading.

Add `crates/ofdm/src/api/inputs/profile_capture.rs`,
`profile_capture/models.rs` and `profile_capture/views.rs`; edit
`crates/ofdm/src/api/inputs.rs` for registration,
re-exported through existing API/facade. `ProfileCapture::from_bytes` admits decoded
JSON; `from_decoded` preserves original/intermediate bytes and decoder contract.
`CaptureLoader::profile_capture` in the existing native adapter handles binary/
JSON with the same pinned decoder; no second parser goes into Qt. Immutable
`profile_ids`, `metadata`, `provenance`, `summary(raw_id)`, `schemes(raw_id, start,
limit)` and `inspect(ProfileViewRequest)` expose bounded source-owned information.
The scheme window caps returned rows at 256 and preserves code, source order,
original offsets/frequencies, role, physical constellation and ideal bounds.

`ProfileViewRequest` owns optional selected raw IDs (None = complete inventory,
empty = no traces), selected roles, BitLoading/IdealShannonSnr metric, original
MeasurementViewport and total bucket budget 1..2048. Preserve sorted/deduplicated
selection and zero-count/full-grid behavior. Backend results carry exact grid,
raw metric extrema/gaps and role-specific spans/counts/loadings, bounded per
selected profile by the divided total budget. Qt retains pixel transforms/colors/
labels and existing components; it computes no capacity or Shannon reference.
A summary reports role counts and `sum(data bits)*spacing` as an uncoded nominal
capacity, separate from net simulated payload throughput and receiver MER limits.
`Dpd::summary`/`view` reuse the same numerical owners where semantically valid.

Edit `crates/pnm-input/src/lib.rs` for native loader dispatch.
Add native `bindings/python/src/profile_capture.rs`, register it in
`bindings/python/src/lib.rs`, and edit the existing root/downstream/legacy
aliases/stubs for immutable request/capture classes. Add tests in `bindings/python/tests/test_profile_capture.py`.
Rust/core/native cases cover binary/JSON catalog equality, complete source IDs,
exact compact schemes, independent capacity/Shannon/frequency anchors, protected
roles, missing markers, window/budget bounds, immutable results, original bytes
and unchanged legacy ProfileId behavior. Add the Rust cases under
`crates/ofdm/tests/api_profile_capture.rs` and `crates/merlin/tests/native_inputs.rs`;
rerun affected parser tests, Clippy and installed-wheel regressions before Qt
adoption. Existing scientific stage/FEC acceptance remains untouched.

### Existing owner decisions

| Inspected owner | Current contract/evidence | Gap | Decision and destination |
| --- | --- | --- | --- |
| [Facade](../../../../crates/merlin/src/lib.rs), public direction re-exports | Thin crate; both independent PHYs already usable; accepted backend compatibility | Qt still imports downstream internals | REUSE facade; ADAPT Qt call sites after typed API extensions, never add science to facade. |
| [Qt organization](../qt-source-organization/design.md), [build registration](../../../../apps/qt/build.rs) | Accepted UI-ORG-01/02; pinned CXX-Qt bridge directory constraint | No direction split | ADAPT group ownership; retain all registered bridges in `src/bridge/`, same QML URI, entry URL and executable. |
| [Closed loop](../../../../apps/qt/src/simulation/closed_loop.rs), `latest`/`Observations` | Bounded downstream worker and retained observations; inline tests | Private high-level-API gap; thread-local single result | SEPARATE per direction; scientific operations move behind public API, adapter retains dispatch and ownership only. |
| [Downstream inputs](../../../../apps/qt/src/inputs), [project file](../../../../apps/qt/src/workbench/project_file.rs) | Existing editors, capture admission, Designer and schema 1/2 persistence | Validation/preview still depends on lower scientific owners | ADAPT through public typed admission and backend project contracts before deleting legacy routes. |
| [Native input crate](../../../../crates/pnm-input/src/lib.rs), [workbench](../../../../crates/pnm-input/src/workbench) | Bounded binary/JSON, provenance, saved-workbench execution; Apache-2.0 | Initial facade exposure is now implemented; direction-aware unified project envelope remains pending | REUSE owners; optional facade input feature/re-exports, extend adapter-owned project contract, preserve existing imports. |
| [Upstream session](../../../../crates/ofdma/src/api/session.rs), `preview`/`run_with_control` | Validated configuration and bounded controlled runs already accepted | Grid coordinates and some instrument views not public API | REUSE session; small observation extensions in upstream API, scientific construction stays in owning modules. |
| [ResourcePlan](../../../../crates/ofdma/src/resources/grid.rs) | Authoritative physical cell/pilot allocation | High-level `ExperimentPreview` gives counts, not cell coordinates | ADAPT via bounded idle grid operation; no QML resource reconstruction. |
| [SpectrumPlot](../../../../apps/qt/qml/instruments/spectrum/SpectrumPlot.qml), [scatter](../../../../apps/qt/qml/instruments/qam/TransmitterScatter.qml), [response](../../../../apps/qt/qml/instruments/response/ReceiverResponseDialog.qml) | Existing downstream Canvas/viewport presentation and QtTests | No real upstream consumers; direction-specific labels/overlays mixed in | DEFER scientific shared acceptance; extract proven presentation only after two real API-backed consumers and tests. |
| [FEC presentation](../../../../apps/qt/qml/instruments/fec), [receiver providers](../../../../apps/qt/src/instruments/receiver) | Actual downstream counters/LLR/iteration views | Upstream LDPC data differs from downstream BCH/control chains | ADAPT chart/table widgets; SEPARATE semantics, code families and unavailable states. |
| [Platform](../../../../apps/qt/src/platform) | Native pickers, update HTTP/install/recovery, logging, theming | Recovery currently assumes one downstream project | REUSE infrastructure; adapt recovery after project API; no new install authorization. |

## Master-guide coverage

Every numbered master section is routed here; the additional API contract takes
precedence over master section 13's allowance to call underlying owners.

| Master sections | Canonical requirements | Official delivery tasks |
| --- | --- | --- |
| 1–3 baseline, skills, workflow | [01](../../../requirements.md#req-uiu-01), [26](../../../requirements.md#req-uiu-26), [27](../../../requirements.md#req-uiu-27) | UIU-PLAN-01, UIU-BASE-01, UIU-QA-01 |
| 4 complete audit and organization history | [01](../../../requirements.md#req-uiu-01), [04](../../../requirements.md#req-uiu-04) | UIU-PLAN-01, UIU-ORG-01, UIU-DS-01 |
| 5–6 source layout, bridge constraint | [03](../../../requirements.md#req-uiu-03), [04](../../../requirements.md#req-uiu-04) | UIU-BOUND-01, UIU-ORG-01, UIU-DS-01 |
| 7 shared QAM/spectrum/channel/FEC/PRBS/PNM presentation | [15–21](../../../requirements.md#req-uiu-15) | UIU-SHARED-01, UIU-SHARED-02, UIU-US-03 |
| 8 shell, menus and independent tabs | [05](../../../requirements.md#req-uiu-05), [06](../../../requirements.md#req-uiu-06), [25](../../../requirements.md#req-uiu-25) | UIU-SESSION-01, UIU-SHELL-01 |
| 9 downstream compatibility | [08](../../../requirements.md#req-uiu-08), [09](../../../requirements.md#req-uiu-09) | UIU-BASE-01, UIU-API-01/02, UIU-DS-01 |
| 10–11 upstream diagram and drill-ins | [10–13](../../../requirements.md#req-uiu-10), [20](../../../requirements.md#req-uiu-20) | UIU-US-01/02/03, UIU-SHARED-02 |
| 12 physical resource-grid viewer | [14](../../../requirements.md#req-uiu-14) | UIU-API-03, UIU-US-04 |
| 13 direction adapters, lifecycle and common presentation | [02](../../../requirements.md#req-uiu-02), [03](../../../requirements.md#req-uiu-03), [06](../../../requirements.md#req-uiu-06), [07](../../../requirements.md#req-uiu-07) | UIU-BOUND-01, UIU-API-01/02/03/04, UIU-SESSION-01, UIU-US-02 |
| 14 independent direction-aware projects | [22–24](../../../requirements.md#req-uiu-22) | UIU-API-05, UIU-FILE-01, UIU-RECOVERY-01 |
| 15 dependency order | [01](../../../requirements.md#req-uiu-01), [26](../../../requirements.md#req-uiu-26) | Official task dependencies and migration plan |
| 16 validation | [24–27](../../../requirements.md#req-uiu-24) | Per-unit acceptance and UIU-QA-01 |
| 17 focused local commits | [26](../../../requirements.md#req-uiu-26), [27](../../../requirements.md#req-uiu-27) | Validated implementation units; no push/release authorization |
| 18–19 acceptance and final report | [26](../../../requirements.md#req-uiu-26) | UIU-QA-01 |
| Additional mandatory API contract, all rules and acceptance clauses | [02](../../../requirements.md#req-uiu-02), [03](../../../requirements.md#req-uiu-03), [09](../../../requirements.md#req-uiu-09), [14](../../../requirements.md#req-uiu-14), [24](../../../requirements.md#req-uiu-24) | Boundary guard, API-first units, downstream migration and final zero-exception audit |

Ranges in this navigation table refer to the individual stable IDs in the
canonical baseline; exact per-task links and acceptance criteria are in the
official checklist. No range or planning record establishes runtime acceptance.
