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
- [Approved upstream startup layout](upstream-startup-layout.md)

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
CMTS RxMER feeds only the backend's admitted residual model. On 2026-11-22 the
user approved the [upstream startup layout](upstream-startup-layout.md), satisfying
the REQ-UIU-10 visual-input gate. Implement UIU-US-01 when its shell prerequisites
pass; no further layout approval is required. Runtime acceptance remains pending.

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

### UIU-API-01: configured channel composition preview unit

Reuse `ChannelImpairments`, `prepare_channel`, original captured-layout admission,
AdditionalLoss, PowerConditioning and ChannelFilterSettings unchanged. Add
`crates/ofdm/src/api/channel.rs` and focused `channel/preview.rs`/`models.rs`.
`ChannelConfigurationInput` owns explicit additional_loss/power/filter settings;
`ChannelConfiguration::new` validates all existing parameter bounds and freezes
an immutable snapshot. `from_json(&[u8])` adds a 64 KiB application transport bound
and strict fields. Curated configuration type re-exports retain their original
scientific owners; no filter/DSP engine moves into the facade.

`preview(&TransmissionConfiguration, &Dpd, Option<&ChannelEstimate>,
AssociationTolerance)` and `preview_capture(&TransmissionConfiguration,
&ProfileCapture, raw_id, Option<&ChannelEstimate>, AssociationTolerance)` call the
same layout/response/epoch/composition owners. Add a crate-private source lookup
in `api/inputs/profile_capture.rs`; factor the existing profile-grid check inside
`api/admission.rs` and shared response/epoch admission inside `api/sequence.rs`
without changing their public contracts. Raw source IDs are retained; neither this
preview nor its successful channel plan establishes full-loop acceptance.

`ChannelPreview` owns the original key/profile ID/sources, full original-grid
optional effective I/Q, active offsets/physical band, at most 1024 gain-view
min/max groups, at most 128 additional/effective gain points, endpoint additional
loss, configured power summary and optional actual filter coefficients/diagnostics.
Gain views cover the accepted active band while the full response retains excluded
gaps on the original grid. This total configured transfer includes conditioning,
additional loss and optional continuous filter, before receiver correction;
it remains separate from original measured H. Exact input H is never normalized
or replaced. Filter/echo ambiguity and fit/null/coverage restrictions stay with
existing owners. No stream, source/noise RNG or PHY simulation is created.

Add native `bindings/python/src/channel.rs`, registration, aliases/stubs and
`bindings/python/tests/test_channel_configuration.py`. Python constructs defaults
or uses bounded JSON to the typed Rust input; settings/power snapshots are owned
copies. It exposes preview/preview_capture as owned dictionaries with the same
fields and shared errors, releasing the GIL for planning. Add
`crates/ofdm/tests/api_channel_configuration.rs` for literal unity/manual-tilt/
conditioning gain anchors, original/excluded grids, configured electrical scale,
raw source identity, actual echo coefficients and invalid field/parameter/size/
H/echo/epoch/fit rejection. Re-run affected channel composition/filter/power tests,
Clippy and installed-wheel regressions before Qt consumers change. Idle TX preview
remains open after this unit.

### UIU-API-01: bounded idle transmitter preview unit

REUSE `simulation::transmitter_preview::TransmitterPreviewRequest` and its actual
TX/PSD pipeline; keep original `generate(&AtomicBool, progress)` usable and add a
`generate_with_cancellation(&impl Cancellation, progress)` delegate for the shared
one-way token. No DSP, coding, scheduling or source algorithm changes.
ADAPT under new `crates/ofdm/src/api/transmitter.rs` and focused `transmitter/`
modules: immutable `TransmitterPreviewRequest::new(&TransmissionConfiguration,
&Dpd, &SourceInput)` and `from_capture(..., &ProfileCapture, raw_id, ...)` admit
through the existing owner. The existing preview supports source IDs 0..15 and
bootstraps Profile A; source ID 16+ continues to reject. It is separate from the
later full-loop source-ID compatibility gate. Freeze original provenance/source
and settings; no source advances until explicit `generate(&TransmitterPreviewControl)`.

A cloneable one-shot control owns cancellation and atomic bounded progress only.
Its `snapshot` reports idle/running/completed/cancelled/failed and completed/total
symbol counts. Concurrent/repeated execution with the same control rejects Busy;
fresh controls permit deterministic independent previews. Cancel is one-way;
no pause is promised. Errors/cancellation retain caller inputs and prior results;
failed generation returns no accepted new preview. The request starts no worker;
callers dispatch synchronous generation outside their UI/interpreter locks.

The owned result retains actual bounded I/Q/PSD with immutable metadata/key/sources,
post-overlap/pre-channel reference plane and source-bit/EOF/warmup accounting.
`sample_view(start,count)` returns at most 4096 contiguous actual samples with
absolute sample origin (zero count selects up to this limit); `spectrum(start,count)`
returns existing min/max buckets (at most 2048; zero count selects remaining FFT),
original frequency metadata and normalized-unit density semantics. Keep numerical
FFT/PSD/grouping in the existing waveform owner. No fabricated/reconstructed
physical stages or messages; those remain API-02. Add `TransmissionConfiguration`
`symbol_timing(Option<velocity>)` and `window()` through existing geometry/window
owners so Qt timing graphics need only position/format authoritative coefficients.

Add `bindings/python/src/transmitter.rs` and focused native result module, register
classes/aliases/stubs; frozen requests/results release the GIL for generation and
queries, expose owned dictionaries, and permit another Python thread to poll/cancel
the control. Rust operations map directly to matching Python methods, with `None`
for unknown velocity and optional fresh control for explicit generation. Add
`crates/ofdm/tests/api_transmitter_preview.rs` and native
`bindings/python/tests/test_transmitter_preview.py`; compare actual old-owner
samples/PSD, independent sample-count/timing/FFT anchors, deterministic regeneration,
finite file/EOF, invalid raw IDs/windows/settings and cross-thread cancellation/
Busy/progress/owned-result behavior. Extend facade tests and installed-wheel parity
before parent API-01 acceptance. Add the direct-Rust example at
`crates/ofdm/examples/api_transmitter_preview.rs`, register its required feature in
`crates/ofdm/Cargo.toml`, and extend `tools/testing/native_api.py` with its separate
reference process. The installed native test compares actual selected samples,
PSD extrema, timing and progress against this generated reference. No Qt source
or historical acceptance changes.

Final API-01 inventory reconciliation is part of this acceptance gate. Refresh
compiler/native inventories without replacing historical run records. Extend
`tools/validation/rust_api_inventory.py` only where actual new explicit scientific
re-exports omit source links: resolve the exact declared import through its freshly
built canonical rustdoc module, preserving kind/method/signature/source identity.
Add independent provenance/collision/signature rejection cases in
`tools/validation/test_rust_api_inventory.py`. Append exact operation/export rows
and focused test/evidence links to the existing authoritative operation matrix;
retain every original row and historical evidence. Compiler discovery and actual
native exports must both match before the parent task can be checked.

### UIU-API-02: downstream controlled-request unit

Dependencies: accepted UIU-API-01. This first unit preserves the existing
Simulation/Merlin evaluation API and adds standalone typed full-loop admission;
retained stage/query and complete source-catalog/sweep parity remain separate
mandatory parts of UIU-API-02 before Qt adoption.

Add `crates/ofdm/src/api/full_loop.rs` and focused `full_loop/` options, control,
request and result modules. `FullLoopInputs` owns accepted TransmissionConfiguration,
Dpd, SourceInput, MeasurementSequence, ChannelConfiguration and optional original
ChannelEstimate. `FullLoopOptions` owns explicit bounded duration/iteration/runtime/
retained-word limits, noise seed, receiver mode and existing MCE/QAM capture policies.
Default values preserve the current supported downstream workbench settings;
legacy Simulation options/defaults/behavior do not change. Add a bounded JSON
options adapter, retaining a typed Rust input/constructor.

`Simulation::prepare_full_loop(inputs, options)` returns immutable reusable
`FullLoopRequest` only after existing captured-layout, MeasurementSelection,
PlaybackConfiguration and final ChannelRunSettings validation. Add a private
measurement-sequence export/revalidation operation in `api/sequence.rs` to retain
original frames, order, mode, tolerance and clock for the selected profile and
numerology. Do not silently accept a sequence planned for different timing.
Neither loading nor preparation advances a source/RNG or starts a job. Configured
H, loss/power/filter and receiver policy remain separate; originals and provenance
are preserved. Recognized raw byte IDs do not broaden existing simulation IDs.

`FullLoopRequest::run(&FullLoopControl)` synchronously reuses the existing bounded
channel job. The cloneable one-shot direction-specific control supports pause,
resume, cancel, scalar snapshot and Busy rejection. Scientific safe-boundary pause
acknowledgement comes from the core wait loop, not an optimistic UI flag. Add a
crate-private controlled job entry with pause-entry/exit notifications; preserve
all existing public/old cancellation entries through a no-op notification delegate.
Active runtime continues to exclude actual paused time. Polling/control calls take
no scientific-result lock and no Python callbacks execute in the core.

The owned `FullLoopResult` retains the actual existing typed complete/partial job
report, selected input provenance and immutable accepted settings. Completed FEC
comparisons and recovered-file outcomes survive cancel/runtime/failure. This unit
uses a disabled passive probe; retained stage/message/spectrum/response/soft-decision
query acceptance remains pending for the next unit. Re-export only named existing
configuration/result models, keeping all kernels/engine calls in the core owner.

Add matching immutable native request/options/control/result wrappers under
`bindings/python/src/full_loop/`, registration and aliases/stubs. Python prepares
from the same typed inputs and releases the GIL for admission/run; another thread
can poll/pause/resume/cancel. Return owned report/provenance snapshots, preserve
structured errors and source ownership. Add focused Rust/core/facade and installed
native tests for actual old-owner settings/output equivalence, independent clean
closed-loop FEC/PRBS outcomes, pause-at-boundary/resume/cancel/Busy/partial retention,
invalid limits/sequence/identity and no mutation. Extend exact operation/test
traceability before committing this implementation unit; parent stays unchecked.
Add a fresh direct-Rust `crates/ofdm/examples/api_full_loop.rs` reference and its
Cargo feature registration, plus the corresponding native-runner reference process
in `tools/testing/native_api.py`. Core tests live in
`crates/ofdm/tests/api_full_loop.rs`; native tests in
`bindings/python/tests/test_full_loop.py`. Include actual interpreter-interrupt
cancellation during an acknowledged pause on the locally validated Linux host.
Native reference comparisons preserve each input's actual original encoding/hash;
compare scientific configuration/counts/words, not hashes of differently encoded
but equivalent synthetic JSON observations.


### UIU-API-02: retained test-point ownership unit

This unit follows the committed controlled-request unit. Add focused retention
policy, owned data/model, collector and bounded query modules under
`crates/ofdm/src/api/full_loop/observations/`, assembled by
`api/full_loop/observations.rs`. Reuse existing TestPointProbe selection/phase/
per-stage/global budgets and borrowed ProbeEvent producers without altering
source, RNG, DSP or FEC execution. Inspect every selected producer representation
before defining the owned data variants. No GPL Qt collector implementation is
copied into the Apache core.

Add immutable `FullLoopObservationPolicy` over typed finite stage-retention inputs,
with strict bounded JSON as an optional transport. The supported initial set is
the existing TX source/coded/interleaved/label/mapped/physical/PLC/pilot/waveform
stages plus channel output, receiver framed/useful/FFT/H and LDPC-input LLR taps.
Reject duplicate/unsupported stage IDs and work/byte limits before retention.
Workbench policy preserves the existing finite per-stage windows and actual
observation-vs-acquisition phase distinctions; no capture is enabled by default.

`FullLoopRequest::with_observations(policy)` returns a fresh validated request,
leaving existing FullLoopOptions fields/defaults and original requests compatible.
The private prepared request stores optional policy; run creates the existing
passive probe and an owned bounded collector. `FullLoopResult` privately owns
retention alongside the actual report/inputs. Exhausted byte retention drops
further copies with explicit statistics rather than failing or altering science.
Preserve actual metadata, original element offsets/counts, physical/sample/grid
coordinates, roles, reference planes and phase; scheduled PLC data is not claimed
physically emitted solely because it was observed at construction.

Expose typed stage inventory/statistics and per-stage snapshot/window queries;
copy at most 4096 actual contiguous elements per query, retaining original origins
and explicit unavailable states. Numerical FFT/PSD/MER/decoding queries belong to
subsequent scientific-owner extensions, not this ownership unit. Native Python
policy/request/result conversions and exact stubs mirror these operations, release
the GIL for work/copies and return owned values. Existing no-observation reports
and all old imports remain compatible.

Add targeted core/facade and native cases comparing actual scientific reports/
retained words with capture enabled/disabled, original data/phase/grid literals,
retention exhaustion, missing/outside/overflow windows, owned results and
cancellation. Update operation/test tracing and exact native wheel acceptance
before committing. Parent UIU-API-02 stays unchecked for numerical/message queries
and complete source-catalog/sweep compatibility before any Qt consumer migration.

The first actual run exposed a producer gap: the standalone pilot generator emits
pilot records, but closed-loop frame assembly previously consumed its actual
continuous/scattered vectors without emitting them. Add a crate-private borrowed
pilot notification alternative in `transmission/frame/assembly.rs`; preserve the
old public assemble signature through a no-op delegate and keep the same existing
pilot generators/output bins. Add a private budget/phase-interest query in
`simulation/test_points/observer.rs` so disabled/exhausted/unselected paths avoid
extra copies. `simulation/channel/link.rs` invokes the alternative only when
requested, using a crate-private values observer in
`simulation/pilots/observation.rs` (registered from `simulation/pilots/mod.rs`).
This emits the original already-computed vectors; it does not regenerate pilots,
infer them from received data or alter scientific output. Validate their actual
bin/IQ equality, fourfold power anchor, phase metadata and enabled/disabled
closed-loop equivalence. Known excluded FFT bins stay explicit zeros; unavailable
history stays None, as defined by the existing frame owner.

### UIU-API-02 passive numerical query unit

Implement the [exact query plan](api-gap-matrix.md#uiu-api-02-numerical-query-unit)
under `crates/ofdm/src/api/full_loop/instruments`, preserving retained ownership
and all original scientific producers. `FullLoopResult::spectrum` accepts a typed
time-stage/event/work/FFT-bin query. Reuse the existing normalized Welch owner;
only matching adjacent absolute sample origins may join, with an explicit bound
of 32 overlapping windows. Keep actual used/available samples and original
reference/phase/grid metadata. Frequency-domain stages and missing windows reject.

`response` selects configured composition H or an actual retained receiver H-hat
event. Existing layout/composition and diagnostic owners supply masked original
4K/8K response curves and signed band-limited impulse diagnostics. Missing or
truncated H-hat cannot select supplied H as a fallback. `llr_statistics` delegates
the actual retained coding-order LDPC-input window to the existing likelihood
owner, with original total/offset/count and sign convention. Existing output
models are re-exported only where appropriate; no whole engine modules are exposed.

Native methods under `bindings/python/src/full_loop/instruments.rs` translate
explicit parameters into typed queries, release the GIL and own returned data.
Core tests compare actual sample/value queries with the existing numerical
owners and independent tone/impulse/likelihood vectors. Validate missing/gapped/
truncated data, work/viewport/nonfinite rejection, repeated immutable queries,
real pilot-estimated H-hat and separate direct-process Rust/native results.
Selected-cell/message and complete source-catalog/sweep parity keep API-02 open.

### UIU-API-02 retained PLC message unit

Add `FullLoopResult::plc_messages()` over actual retained `tx.plc.message` events,
reusing the existing frame parser, CRC-validated MAC reassembler and descriptor
decoder. Bound processing to 64 original frames and 32 packets. Preserve complete
packet bytes, source address, descriptor identity, TLV offsets/encoded bytes and
typed field meanings. Semantic field decoding belongs to the existing descriptor
owner, reusing received-channel settings and accepted modulation tables; Qt only
formats those typed values. Do not copy GPL presentation algorithms into the core.

Original construction capture does not establish physical transmission. Report
whether the final carrying frame's end is within actual transmitted sample count,
using the existing 128-symbol frame constant and accepted symbol stride with
checked arithmetic. Preserve each packet's last carrying original context. Stop
at missing, truncated, nonconsecutive or invalid frames, retaining earlier complete
packets and an explicit structured issue/pending-byte count. Never bridge missing
segments or silently substitute reconstructed messages.

Native Python converts the same bounded owned report with the GIL released. Test
both numerologies' descriptor meanings and byte offsets through independent fixed
anchors, normative existing packet/CRC/reassembly vectors, actual scheduled-versus-
transmitted coverage, unavailable/truncated/gapped data and direct-process parity.
This closes message inspection only; selected-cell and complete source-catalog/
sweep parity still keep API-02 open before Qt adoption.

### UIU-API-02 selected receiver cell unit

Add bounded `FullLoopResult::qam(FullLoopCellQuery)` and
`cell_decisions(FullLoopCellQuery)` over the actual immutable payload/NCP and
protected-stream report owners. Explicit streams are payload, NCP, PLC data,
PLC preamble, continuous pilots and scattered pilots. Preserve matched physical
bin/frequency/channel-symbol/source-symbol identities, original before/after
correction I/Q, optional diagnostic TX reference and actual receiver LLRs.
Pilot reference amplitude is explicitly two; other accepted streams use one.

Queries retain optional profile/order and inclusive physical-bin/channel-symbol
windows. Automatic order selects the first actually available order after profile
filtering. Metadata describes the original retained population and actual filtered
count, including honest empty selections. A contiguous display window copies at
most 2048 cells; nearest-decision rows are limited to 128. Work remains bounded by
the existing finite capture owners. Never synthesize missing roles or pair cells
from independent runs. Each owned result identifies one run; history/session
routing belongs to the later adapter/batch owners.

Nearest-reference and matched-TX MER use the existing modulation owners over all
filtered cells, independently of display pagination. Keep raw/corrected and
nearest/TX-reference metrics distinct. Receiver diagnostic normalization/slicing
delegates to the existing scientific slicing owner, never Qt or Python, and never
feeds production FEC. Summarize actual soft values from at most 2048 selected cells
through the existing likelihood owner, separately from LDPC coding-order data.

Native Python uses an immutable `FullLoopCellQuery` wrapper with strict bounded
JSON and owned query results. Validate literal role/power/coordinate identities,
all-selected versus display-window MER, missing references/empty selections,
unsupported orders/ranges/work limits, exact slicing/soft values, independent
result ownership and separate direct Rust-process parity. Decision-region geometry,
ideal-reference and detailed soft-display projection remain explicit follow-up API
work. The legacy spectrum/physical views also require an explicit audit of bounded
instantaneous FFT-bin power and authoritative physical-role views before their Qt
calls migrate; raw I/Q does not authorize Qt to compute scientific observations.
No Qt presentation source changes during this unit.

### UIU-API-02 slicer reference and soft-view unit

Add passive `Simulation::constellation_reference(order, amplitude, start, count)`
and `Simulation::decision_region(order, label, amplitude)` operations. Their
scientific owner is a focused module under existing modulation/slicing, using the
original label-ordered reference tables. Return owned labelled reference points
(at most 4096 per query) or a finite local nearest-reference region, at most 49
neighbor points and 64 polygon vertices. The region is the intersection of all
reference half planes with an explicit finite viewport; it does not claim a
bounded outer Voronoi cell. Validate order/label, positive finite amplitude,
coordinate overflow and work limits. Preserve unit-energy versus known physical
reference amplitude explicitly. These static operations never configure a session
or run a PHY/source.

Add `FullLoopResult::cell_soft_view(query)` and `decoder_soft_view(snapshot)` over
actual existing selected-cell and retained coding-order LDPC observations. The
existing receiver diagnostic owner supplies finite LLR statistics, two explicit
40-bin display histograms spanning (-100,100) and (-10,10), endpoint counts and
at most 1024 original-index min/max groups. Bound input to 32768 values, preserve
original offsets/context and distinguish selected physical cells from coding
order. No probabilities, empirical BER or new decoder inputs are inferred.

Implement numerical preparation independently in Apache Rust owners; do not copy
GPL Qt code. The backend returns scientific numbers and geometry; Qt retains pixel
coordinates, labels, colors and chart layout. Python converts typed public calls
to owned dictionaries and releases the GIL. Add independent BPSK/scale/region
constraints, every accepted constellation geometry, literal histogram boundary/
endpoint/index vectors, finite-work rejection and actual/direct-process parity.
Trace through existing REQ-UIU-02/07/09/16/19/24 and UIU-API-02; physical-spectrum/
role audit and complete catalog/sweep compatibility remain subsequent gates.

### UIU-API-02 physical-bin and transmitted-role unit

Add passive `TransmissionConfiguration::physical_layout` and
`physical_layout_capture` queries over accepted Dpd or original ProfileCapture
inputs. Reuse captured-layout admission and `CarrierLayout::symbol_roles`; return
the authoritative original FFT grid, requested physical carriers/roles, fixed PLC
bounds and at most 1024 role spans. Keep source-capture identity/grid separate from
the full physical FFT layout. This is configured geometry, not an observed run or
proof that an arbitrary encoded profile ID supports full simulation.

Add `FullLoopResult::physical_bins(stage, snapshot, start, count)` over actual
retained TX physical bins or raw RX FFT bins. Return original context/extent,
authoritative phase-specific roles and actual I/Q, linear bin power and dB power.
Preserve known zero bins separately from unavailable history. Do not transform
frequency bins again or label instantaneous bin power as PSD/Hz. Bound a query
to 8192 original bins; compute power through a focused waveform numerical owner
and reuse the layout-view owner. No QML reconstruction of pilot/PLC positions.

Add `FullLoopResult::transmit_cells(stage, snapshot, start, count)` for original
mapped/interleaved transport-cell observations. Preserve the typed original value,
absolute cell index and optional actual packing role. A focused payload diagnostic
owner may invoke the existing generic SymbolInterleaver on retained role metadata
only for the first demonstrably aligned source symbol. Missing, misaligned or
later history remains unknown; roles are never inferred from I/Q positions and no
live transmitter/source/interleaver advances. PLC roles come from their actual
stage identity, not inferred payload labels.

Validate both numerologies' existing layout/pilot vectors, dynamic role positions,
original offsets, exact 3+4j power and zero/missing distinctions, invalid windows/
overflow, independent role-interleaver vectors and actual/direct-process parity.
Native Python mirrors all queries and releases the GIL. Trace through existing
REQ-UIU-02/07/09/16/17/24 and UIU-API-02. Subsequent spectrum work must explicitly
cover existing channel-power calibration and legacy ordered per-frame stage PSD
observations without changing the accepted contiguous-sample spectrum operation.
Complete catalog/sweep and final API-02 acceptance remain separate gates.

The focused file/owner map for this unit is:

| Disposition | Owner/files | Contract |
| --- | --- | --- |
| Reuse | `transmission/layout/physical.rs`, `simulation/channel/measurement.rs` | Existing captured-layout admission and channel-symbol roles; no new pilot placement. |
| Add | `transmission/layout/view.rs`, `signal_processing/waveform/bin_power.rs` | Original FFT layout window/spans and instantaneous complex-bin power, separate from PSD. |
| Add | `simulation/payload/role_projection.rs` | Metadata-only first-aligned-symbol projection through the existing generic interleaver. |
| Add | `api/physical.rs`, `api/full_loop/physical.rs`, `api/full_loop/transmit_cells.rs` | Immutable configured and actual retained typed public queries. |
| Add | `bindings/python/src/physical.rs`, `bindings/python/src/full_loop/physical.rs` | Thin owned conversions with the GIL released. |
| Edit | `crates/ofdm/tests/api_full_loop_observations.rs`, `crates/ofdm/examples/api_full_loop.rs`, `bindings/python/python/merlin/__init__.pyi` | Original-value/geometry/bounds tests, direct-process reference and matching native signatures. |
| Add | `bindings/python/tests/test_physical_views.py` | Installed-wheel originals, phase/role provenance, limits, ownership and direct Rust parity. |
| Edit | Existing parent module registrations | Register the focused modules and public re-exports; no new dependency or Qt source change. |

Paths in the first five rows are relative to `crates/ofdm/src/` unless prefixed
with `bindings/`. Existing public methods/defaults and raw-stage query limits are
preserved. Source-only and planning passes cannot establish Qt acceptance.

### UIU-API-02 ordered frame-stage spectrum unit

Add `FullLoopResult::spectrum_sequence(FullLoopSpectrumQuery)` and matching native
`FullLoopResult.spectrum_sequence(stage="tx.ifft.samples", snapshot=0,
max_averages=32, start=0, count=0)`. Only actual IFFT, prefixed, windowed and
receiver useful-frame stages are admitted. Existing `spectrum` continues to join
only physically contiguous origins at its admitted boundaries; its signatures,
defaults and outputs stay compatible.

The new diagnostic concatenates retained consecutive stage frames on an explicit
ordered-frame sample clock. Preserve each contributing channel symbol, original
sample origin, element offset/total, retained extent and sequence offset/count.
Report true physical contiguity and why collection stopped. Missing phase/run/
configuration/geometry/reference continuity, skipped channel symbols, unexpected
original stride, partial-frame history or finite work limits stop collection.
Never fill a gap, overlap-add another signal, rerun a source or silently call this
a continuous waveform. First-frame selected offsets are preserved; later frames
must begin at their original element zero. Only fully retained earlier frames
may precede another segment; the final segment may be clipped by the work bound.

Reuse the existing Welch owner (periodic Hann, 50% overlap, at most 32 averages,
2048 buckets). Bound assembled work to N + 31*N/2 and 64 contributing frames;
retain collected versus actually analyzed sample counts and segment coverage.
The API supplies matching-context admission; a focused waveform owner handles
sequence/stride/window assembly. No scientific implementation belongs in Qt or
Python. Observation defaults stay unchanged: a caller explicitly requests more
than one frame to retain legacy multi-frame diagnostic views.

| Disposition | Owner/files | Contract and acceptance |
| --- | --- | --- |
| Reuse | `crates/ofdm/src/simulation/waveform/observation.rs`, `crates/ofdm/src/signal_processing/waveform/spectrum.rs` | Actual frame origins/geometry and existing normalized Welch computation; original source remains unchanged. |
| Add | `crates/ofdm/src/signal_processing/waveform/stage_sequence.rs` | Bounded original-index segment assembly; independent literal gap/overlap/stride/truncation and overflow vectors. |
| Add | `crates/ofdm/src/api/full_loop/instruments/sequence.rs` | Typed public query/result, explicit sampling basis, matching contexts and analyzed segment accounting. |
| Edit | `crates/ofdm/src/api/full_loop/instruments/spectrum.rs` | Reuse one existing PSD-to-public-viewport projection without changing normal spectrum behavior. |
| Add | `bindings/python/src/full_loop/sequence.rs`, `bindings/python/tests/test_spectrum_sequence.py` | Thin GIL-released operation and installed-wheel source/limits/ownership/direct-Rust parity. |
| Edit | Parent module registrations, root native stub, actual full-loop example and `crates/ofdm/tests/api_full_loop_observations.rs` | Public re-exports/signature, separate process reference and comparisons against literal concatenation of actual retained frames; existing contiguous query remains distinct. |

The user reinforced the coding-style/no-magic-number rule during this unit.
Derive the FFT work ceiling from existing SubcarrierSpacing numerology and
MAX_SPECTRUM_AVERAGES; share the existing 50-percent hop through a named waveform
constant. Name application frame/role-span/identifier limits separately from PHY
dimensions. Apply the same additive style correction to the preceding physical
views without changing its accepted numeric limits or behavior. Keep independent
literal numerical test anchors distinct from production-derived constants.
The actual audit keeps FRAME_EVENTS_PER_STAGE and FRAME_SEQUENCE_WELCH_AVERAGES
separate, as well as reference-page offsets, decision rows and selected soft-cell
counts. MAX_STAGE_SEQUENCE_FRAMES is application retention policy;
MAX_PHYSICAL_ROLE_SPANS is overlay policy; MAX_STAGE_IDENTIFIER_BYTES is native
transport policy. None is reused as a constellation order or PHY dimension merely
because a value coincides. The example's MER, capture epoch/grid, source seed and
host/decoder limits also have explicit fixture names. No values change.


Trace through REQ-UIU-02/07/09/16/17/24 and UIU-API-02. Calibrated channel-power/
spectrum, scalar amplifier-boundary and complete source-catalog/sweep compatibility
remain subsequent gates; this unit does not adopt the API in Qt.

### UIU-API-02 calibrated channel and amplifier unit

Requirements REQ-UIU-02/07/09/16/17/24 and UIU-API-02 retain the existing
scientific reference planes. Add passive `FullLoopResult::calibrated_spectrum`
with the unchanged FullLoopSpectrumQuery for actual contiguous time boundaries.
Add `FullLoopResult::channel_spectrum(tap, query)`, where FullLoopChannelTap is
BeforeAmplifier, AfterAmplifier or ReceiverInput; the tap fixes TX-overlap or
actual channel-output selection and a mismatched query stage rejects. Before/
after amplifier views use the same retained TX samples; only the latter applies
the accepted amplifier scalar. It excludes CPA, physical H, additional loss and
receiver noise. ReceiverInput uses actual modeled output, including admitted
channel effects. Metadata distinguishes direct observations from scalar-derived
views and preserves the original context, sample origins and finite coverage.

Add `FullLoopResult::amplifier_bins(boundary, snapshot, start, count)` with a
separate Before/After AmplifierBoundary enum. Reuse actual TX physical bins,
original roles and missing history; the after view applies the same scalar and
recomputes instantaneous bin power through the existing numerical owner. Raw RX
QAM continues through existing received-cell queries rather than substituting
transmitter truth. No channel output is synthesized by this operation.

Add `TransmitterPreview::calibrated_spectrum(channel, start, count)` for the actual
idle TX PSD before the amplifier. A supplied validated ChannelConfiguration sets
only electrical units; it does not execute a channel, apply gain or replace actual
TX generation. Original normalized spectrum methods remain unchanged.

The existing channel_power owner integrates the full inclusive first-to-last
nonexcluded physical band, including internal holes, before display reduction.
Zoom never changes that reading. A focused channel spectrum owner copies and
calibrates density only when an explicit input reference exists; relative values
otherwise remain relative. Zero density retains absent finite dB values. Return
explicit normalized versus millivolt-squared density units, bucket dB reference,
scale, full-band power and original FFT metadata. Keep the existing complex-envelope
normalization and six-MHz reference; do not introduce a real-carrier factor of two.

| Disposition | Owner/files | Contract and acceptance |
| --- | --- | --- |
| Reuse | `signal_processing/channel/power.rs`, `waveform/spectrum.rs`, `transmission/layout` | Existing electrical calibration, full-resolution integration, Welch/window/buckets and occupied physical band. |
| Add | `signal_processing/channel/spectrum.rs` | Bounded calibrated spectrum projection and validated scalar I/Q operations; independent flat-density, zero/missing, gain, overflow and zoom-invariance vectors. |
| Edit | `api/full_loop/instruments/spectrum.rs` | Extract unchanged contiguous sample preparation for both original and calibrated queries; no default or retention changes. |
| Add | `api/full_loop/instruments/channel.rs`, `api/transmitter/calibrated.rs` | Typed public operations and explicit direct/derived planes over owned immutable results. |
| Edit | API module registrations and channel model re-exports | Public result/enum ownership, without exposing the scientific implementation module. |
| Add | `bindings/python/src/full_loop/channel.rs`, `bindings/python/tests/test_channel_spectrum.py` | GIL-released argument conversion, installed-wheel bounds/ownership/units and separate Rust-process parity. |
| Edit | `bindings/python/src/transmitter/result.rs`, native stub, Rust examples and focused integration tests | Matching idle preview operation and actual reference processes; preserve existing outputs. |

Paths in this table are relative to crates/ofdm/src unless prefixed with bindings.
Limits reuse accepted numerology, Welch work and existing public query bounds.
Keep conversion factors, calibration bandwidth, application budgets and independent
fixture anchors named by purpose even when values coincide. Missing observations
reject without a hidden run; every query leaves retained source/RNG/results intact.
This is an API prerequisite unit, with Qt files unchanged. Complete catalog/sweep
compatibility and final API-02 acceptance remain after this unit.
The [focused acceptance](validation.md#uiu-api-02-calibrated-channel-and-amplifier-unit)
records passing Rust/native evidence and the [user contract](../../../user/api/channel-spectrum.md).

### UIU-API-02 complete catalog and bounded sweep unit

REQ-UIU-02/07/09/16/17/18/19/20/24 and UIU-API-02 require the existing Qt
single/current, all-captured and explicit-selection admission behavior before
UIU-DS-01 adopts the public API. Preserve the complete byte-sized ProfileCapture
catalog and the original ProfileId 0..15 domain. All-captured selection rejects
unsupported run IDs rather than dropping them or inventing a coding capability.
A valid selected ID can be admitted from a mixed catalog without rewriting its
original bytes or rejecting unrelated read-only source entries.

Add `ProfileCapture::simulation_profile(id: ProfileId) -> ApiResult<Dpd>`.
The returned immutable Dpd retains the original selected assignment, captured
origin and complete original-byte provenance. Its legacy `profile_ids()` lists
only the catalog IDs that pass the existing ProfileId validator; the unchanged
ProfileCapture retains every raw ID. Existing Dpd loaders and ProfileId behavior
remain unchanged. Add a private Dpd factory in `api/inputs.rs` because that owner
controls its catalog/origin fields; the capture operation delegates to it.

Add `FullLoopSweepPlan::new(request: &FullLoopRequest, catalog: &ProfileCapture,
selection: FullLoopProfileSelection) -> ApiResult<Self>`. The explicit selection
is Current, AllCaptured or Selected(Vec<ProfileId>). Current preserves the exact
accepted request, including edited-hypothesis loading. Captured selections use
original source profiles, sorted by original ID, with no duplicates or empty
selection. Preserve the legacy 32-profile/262144-assignment application limits
as separately named policies; they are not the simulation ID domain. Require
matching original ScenarioKey and validate every selected profile through the
same FullLoopRequest preparation before returning a plan. Failure is atomic
and identifies the failing original profile.

`plan.info() -> FullLoopSweepInfo` reports full raw catalog IDs, selected checked
IDs, assignment count and finite sequential duration/host/detail ceilings.
`plan.request(index: usize) -> ApiResult<FullLoopRequest>` returns one immutable
fully validated request, preserving original source, measurements, channel,
options and observation policy. Preflight constructs and drops one request at a
time; the plan shares catalog/base ownership instead of retaining a separate
copy of all measurement frames per profile. Request creation starts no run.
Scientific selection/admission and execution stay in Rust; clients dispatch the
existing independently controlled `run` operation serially. Reusing a completed
control still rejects, and cancellation/partial reports retain their existing
per-profile contract. The later Qt session task owns aggregate retained-result
budgets and UI dispatch, not scientific profile selection or validation.

Native Python adds `capture.simulation_profile(profile_id)` and a genuine
importable FullLoopSweepPlan class with constructor `(request, catalog,
selection="current", profile_ids=None)`, owned `info` property and `request(index)`.
Validate sequence length before extracting selected IDs, then use Rust ProfileId
admission. `selected` requires a nonempty ID sequence; other modes reject an
extraneous list. All work releases the GIL. Existing Rust/Python class/import
identity and behavior remain compatible.

| Disposition | Owner/files | Contract and acceptance |
| --- | --- | --- |
| Reuse | `api/full_loop/request.rs`, `api/sequence.rs`, `api/inputs/profile_capture.rs`, `simulation/channel/measurement` | Existing immutable admission, supported coding domain, exact source epochs/grids and measured playback. |
| Edit | `api/inputs.rs`, `api/inputs/profile_capture.rs` | Private selected-profile factory and additive public capture admission; raw inventory/provenance remains complete. |
| Add | `api/full_loop/sweep.rs` and parent registration | Typed finite plan, explicit selection, original-key validation, all-before-run preflight and lazy per-profile request construction. |
| Edit/Add | `bindings/python/src/profile_capture.rs`, new `bindings/python/src/full_loop/sweep.rs`, registration/import modules/stub | Thin checked conversion, public class identity and installed-wheel operations. |
| Add/Edit | Focused Rust/native tests, actual Rust example and user guide | Mixed raw IDs, original selected loading, sorted/deduplicated/current/empty/invalid/mismatched cases, finite budgets, rejected-call invariance, per-profile actual runs/cancellation and direct-process parity. |

Paths are relative to crates/ofdm/src unless prefixed with bindings. Retain
historical requirements, original normalized observations and completed tasks.
This closes only the catalog/sweep API gap after focused acceptance; the full
API-02 regression gate is now satisfied by the
[complete acceptance](validation.md#uiu-api-02-public-full-loop-and-observations-accepted);
Qt adoption remains a separate task. The [unit evidence](validation.md#uiu-api-02-complete-catalog-and-bounded-sweep-unit)
records exact source/wheel hashes and independent selection/control cases.

### UIU-API-03 idle upstream resource-grid observation

REQ-UIU-02/14/24 and UIU-API-03 add
`Simulation::resource_grid(&self, request: ResourceGridRequest) -> ApiResult<ResourceGridSnapshot>`.
The accepted session stays idle and its configuration generation is unchanged.
This is the configured base grant's allocation, not actual received/transmitted
I/Q and not a reconstruction in Qt. Reuse ResourcePlan::new, its original visits,
CellRole/loading/gain/bit offsets, immutable UsConfiguration and grant::select.
No new pilot/interleaving algorithm or supported mode is introduced.

ResourceGridRequest has frame_index, first_symbol, symbol_count, first_carrier,
carrier_count and max_cells (all usize). Default selects frame zero, symbol zero,
one symbol, the full original FFT carrier span and a 16384-cell budget. Zero
symbol/carrier count selects the remaining original frame/FFT extent. Work must
be positive and at most MAX_RESOURCE_GRID_CELLS=16384; validate frame, extents,
checked products and caller budget before constructing a ResourcePlan or copying
cells. The separate JSON transport ceiling is eight MiB. Both are application
policies, unrelated to normative minislot sizes or execution resource limits.

ResourceGridSnapshot contains the merlin.upstream.resource_grid export format,
independent schema version 1, configuration_generation, transmission_planned
(from the existing grant selector) and an owned ResourceGridRegion. The region
retains grant ID/epoch/first-frame/frame-count, selected profile/IUC, numerology,
FFT/sample-rate/spacing, CP/RP/stride, original frame index/identity/prefix origin,
resolved original symbol/carrier viewport, full frame/grant coded capacity and
visit count, per-window role counts and bounded cells. Sample/frame arithmetic
is checked and never infers sample time from an arbitrary absolute frame ID.
A snapshot remains usable after session reconfiguration/destruction.

Cells are ordered by increasing frame symbol then physical carrier, matching
symbol X and physical-carrier Y in the future viewer. Each cell keeps frame and
grant-relative symbol, physical carrier, prefix/useful sample origins, role,
original minislot index/pattern/configured loading and grant-selection flag.
For actual ResourcePlan visits, preserve original joint-interleaver visit index,
cell loading, amplitude gain, frame-relative and grant-relative coded-bit offsets.
Do not compact physical holes or derive roles from zero-valued samples. Excluded,
zero-loaded selected minislots and unallocated locations stay distinct; absent
visits have no invented gain or bit position. Unselected profile minislots may
retain configured metadata, while their resource role stays unallocated.
Complementary cells carry data at the existing lower loading; known pilots carry
no coded bits. Capacity comes from the full plan independently of the viewport.

`ResourceGridSnapshot::to_json() -> ApiResult<String>` reuses bounded_json with
MAX_RESOURCE_GRID_JSON_BYTES, keeping serialization separate from science.
Native Python adds an actual importable merlin.upstream.ResourceGridSnapshot with
to_json(), and Simulation.resource_grid(frame_index=0, first_symbol=0,
symbol_count=1, first_carrier=0, carrier_count=0, max_cells=16384). Defaults reuse
the Rust request; explicit introspection metadata is pinned by an installed test.
The adapter uses the existing Busy/session lease guard, releases the GIL for
work and returns an owned snapshot. No Python/Qt scientific calculations.

| Disposition | Owner/files | Contract and acceptance |
| --- | --- | --- |
| Reuse | `crates/ofdma/src/resources/grid.rs`, `crates/ofdma/src/resources/pilot_patterns.rs`, `crates/ofdma/src/configuration/` and `crates/ofdma/src/grant/` | Authoritative allocation, original physical visits/roles/gains, unchanged mode admission and full-plan coding decision. |
| Add | `crates/ofdma/src/resources/view.rs` | Bounded rectangular projection of the existing plan, original coordinates and distinct holes; no pilot placement algorithm. |
| Add/Edit | `crates/ofdma/src/api/resources.rs`, `crates/ofdma/src/api/mod.rs`, `crates/ofdma/src/api/session.rs`, `crates/ofdma/src/resources/mod.rs` | Typed re-exports, generation/plan wrapper and bounded export; public operation remains read-only. |
| Add/Edit | `bindings/python/src/upstream/simulation/grid.rs`, `bindings/python/src/upstream/simulation.rs`, `bindings/python/python/merlin/upstream.py`, `bindings/python/python/merlin/upstream.pyi` | Existing access_long guard and typed snapshot conversion, exact defaults and installed namespace. |
| Add | `crates/ofdma/tests/api_resource_grid.rs`, `bindings/python/tests/test_upstream_resource_grid.py` | Independent literal pilot/edge/zero/exclusion coordinates, all admitted patterns/frame sizes/numerologies, bounds, timing, ownership, generation and actual-run invariance, installed-wheel direct-process parity. |
| Add/Edit | `crates/ofdma/examples/upstream_resource_grid.rs`, `tools/testing/native_api.py`, `docs/user/upstream/resource-grid.md`, `docs/user/upstream/README.md`, `docs/user/python/upstream.md` | Real separate-process reference and a focused module guide, without changing prior example outputs. |

Pinned evidence remains CM-SP-PHYv3.1-I20-230419 section 7.4.16, PDF pages
101-104, Figures 38-42 and the already accepted independent upstream pilot tests.
The current extraction was inspected for CM edge/body and complementary semantics;
this unit does not claim a new rendered-PDF verification or change normative
support. All new observations require passing MERLIN tests; Qt viewer acceptance
remains UIU-US-04 after this focused public API commit.

The [backend acceptance](validation.md#uiu-api-03-idle-upstream-resource-grid-accepted)
records 88 upstream/16 facade/114 installed native passes, core/native Clippy,
77 tooling passes and exact source/wheel hashes. All prior API rows, alias history
and Qt source hashes are preserved. UIU-API-03 is complete; UIU-US-04 still owns
actual viewport rendering, interaction and bounded Qt object acceptance.

### UIU-API-04 upstream instrument gaps and report views

REQ-UIU-02/12/16/17/18/19/20/24 require real upstream observations before Qt
adoption. Existing `BatchReport::qam` and `spectrum` already expose selected actual
I/Q, original coordinates, normalized rectangular PSD and explicit unavailable
history. Existing GrantReport/DecodedWord preserve actual decoder decisions,
iterations and independent source comparison. Reuse these owners and their
existing tests; do not add a second simulation interface or algorithms in Qt.

The audit identifies three separate gaps. First, the UI needs bounded typed FEC
paging and scalar verification summaries without copying/parsing an entire report
or calculating scientific aggregates in the adapter. Second, receiver pilot
estimates and per-cell/word soft observations are not retained by the existing
report; they cannot be recovered by reading the final decoded bits. Third,
response/impulse and constellation reference queries need authoritative Rust
operations with explicit C, H and estimated C×H planes. The latter gaps remain
pending; acceptance of the report-view unit alone does not complete UIU-API-04.

The first implementation unit adds these exact Rust operations on the existing
owned BatchReport, through `merlin::upstream`:

- `verification(&self) -> PhyResult<VerificationSnapshot>` copies actual status,
  source seed, planned/generated/compared/unverified/error counts, optional BER,
  recovery flag and grant/codeword/coverage counts. PRBS15 truth remains the
  independent verifier's result; parity/convergence never implies source recovery.
- `fec_summary(&self) -> PhyResult<FecSummary>` counts planned/completed/pending
  words, independent convergence/parity/shortening/fill failures, actual iteration
  sum/maximum and hard-decision changes. Zero completed words have no maximum.
- `fec_words(&self, request: FecWordRequest) -> PhyResult<FecWordPage>` selects one
  started grant by original index and a completed-word ordinal window. It returns
  original grant identity, word ordinal/source offsets, upstream code-family and
  shortening dimensions, unchanged decoder flags/iterations and actual decisions.
  No BCH stage or unavailable soft metrics are invented.

FecWordRequest has grant_index, first_word, maximum_words and max_decision_bits.
Defaults select grant/word zero, one word and 262144 copied decision bits. The
independent application ceilings are 32 words and 262144 decision bits; count all
copied mother, information and payload vectors before allocating the page.
Out-of-range grants/starts, zero/oversized budgets, inconsistent original word
metadata and arithmetic overflow reject. A start exactly at the completed-word
end returns an empty page; partial/unprocessed words are not substituted.
FecWordPage retains the selected grant identity, total completed/pending word
counts, returned first position and optional next position. Scalar totals remain
independent of page size and detail budgets. Querying never runs a source, decoder
or random generator, and returned owned data outlives the report/session.

Python mirrors `report.verification() -> dict`, `report.fec_summary() -> dict`
and `report.fec_words(grant_index=0, first_word=0, maximum_words=1,
max_decision_bits=262144) -> dict`. Native conversion releases the GIL for Rust
query/serialization, uses the same validation/errors and returns owned ordinary
Python data. No extra Python scientific calculation or whole-report fallback.

| Inspected owner / files | Current contract and gap | Decision / destination |
| --- | --- | --- |
| `crates/ofdma/src/simulation/report.rs`, `crates/ofdma/src/receiver/framing.rs`, `crates/ofdma/src/grant/plan.rs` | Actual completed/partial decisions and independent verifier counts already exist; no bounded detail projection | Reuse original data and dimensions; add focused `crates/ofdma/src/instruments/recovery.rs`; do not change receiver behavior or report schema. |
| `crates/ofdma/src/instruments/observer.rs`, `crates/ofdma/src/instruments/spectrum.rs` | Accepted bounded actual QAM/PSD owners; no retained receiver estimates or soft words | Reuse QAM/PSD; keep new response/soft retention as subsequent API-04 units before Qt exposure. |
| `crates/ofdma/src/api/mod.rs`, `crates/ofdma/src/instruments/mod.rs` | Direction public export/ownership | Edit focused module registration and typed high-level re-exports. |
| `bindings/python/src/upstream/reports.rs`, proposed `bindings/python/src/upstream/reports/recovery.rs`, `bindings/python/python/merlin/upstream.pyi` | Existing owned report and shared JSON-to-object conversion | Add thin methods on the existing report, exact installed defaults and dictionary parity. |
| Proposed `crates/ofdma/tests/api_recovery_views.rs`, `bindings/python/tests/test_upstream_recovery_views.py`, `crates/ofdma/examples/upstream_recovery_views.rs`; `tools/testing/native_api.py` | Existing source/codec/experiment cases remain; focused new projections need acceptance | Add independent completed/empty/partial/failed flag, family/shortening/offset, budget/paging/ownership tests and a separate Rust-process reference. |
| Proposed `docs/user/upstream/recovery.md`; upstream/Python guide indexes | Document actual projection scope separately from missing LLR/response views | Add focused user guide and retain explicit parent-task gaps. |

This unit changes no scientific algorithm or normative mode. Existing upstream
code definitions retain PHY I20 section 7.4.3.2/source vectors and original notices.
New projection tests must not relabel fabricated failure fixtures as actual decoder
experiments: use them only to prove that the projection preserves independent flags.
Actual completed and cooperatively cancelled MERLIN runs establish runtime evidence.

The [recovery-view unit acceptance](validation.md#uiu-api-04-bounded-recovery-view-unit)
records seven focused Rust cases, the complete 95-case upstream package, 16 facade
cases, 119 installed native cases, core/native Clippy and 77 tooling passes. Actual
destructive-channel outcomes remain failed, and deliberately edited flag fixtures
remain labelled projection tests. Original API rows, report schema, scientific
algorithms and all Qt sources remain unchanged. UIU-API-04 stays unchecked for its
received-estimate/response/soft and remaining instrument gaps.

### UIU-API-04 configured response and periodic impulse unit

REQ-UIU-02/12/18/24 next expose configured deterministic C, H and C×H through
typed Rust operations, separately from the still-pending actual received-pilot
estimate. A configured response is a diagnostic of admitted settings, not a
measured channel, an actual received observation or a normal receiver input.

The exact proposed operations are
`Simulation::configured_response(&self, request: ConfiguredResponseRequest) -> ApiResult<ConfiguredResponseSnapshot>`
and `BatchReport::configured_response(&self, request: ConfiguredResponseRequest) -> PhyResult<ConfiguredResponseSnapshot>`.
The idle operation requires accepted configuration and preserves generation;
the report operation uses its frozen original configuration. Neither executes a
waveform, channel stream, source, receiver or RNG. The existing native Busy guard
applies to the idle session, while report queries remain independent/owned.

ConfiguredResponseSource has CmPreEq, PhysicalChannel and Composite, serialized
as cm_pre_eq, physical_channel and composite. ConfiguredResponseRequest contains
source, grant_symbol, first_carrier, carrier_count, maximum_points and
include_impulse. Default is PhysicalChannel, original base-grant symbol/carrier
zero, remaining full FFT, maximum_points=4096 and include_impulse=false. Validate
the original symbol/carrier extents and positive frequency-point budget before
numerical work. MAX_CONFIGURED_RESPONSE_POINTS=4096 is a frequency-display
application policy. The separate MAX_CONFIGURED_IMPULSE_SAMPLES=4096 bounds the
complete periodic impulse; equal values have different purposes. No RF frequency
origin is inferred from a physical FFT index.

Reuse PreEqPlayback::at(grant_symbol), or the original static PreEqBank, for C.
Retain its actual bank generation, independently of replay event/probe IDs. Reuse
the centered unitary upstream Transform (shared merlin-dsp kernel) to derive the
configured FIR transfer from original causal taps. Include the admitted integer
delay and constant phase from ChannelConfig, including the actual channel owner's
initial phase reduction modulo one turn before trigonometry. Explicit nonzero frequency offset
makes H time varying, so PhysicalChannel/Composite static queries reject with an
unsupported diagnostic; an independent C-only query remains valid. Noise and
RxMER residual effects are excluded from this deterministic transfer and remain
explicit metadata. Existing measured-composition restrictions remain unchanged.

For N original centered bins, deterministic transfer is

\[
H[k]=e^{j\phi}e^{-j2\pi f_k d/f_s}\sqrt{N}\,\mathrm{FFT}_{u}(h)[k],
\qquad G[k]=C[k]H[k].
\]

The optional impulse is the complete periodic diagnostic
`IFFT_u(G)/sqrt(N)` (or selected C/H), computed before frequency viewport
selection. Its period is N sample ticks; delays wrap modulo N. It must never be
labelled an unaliased causal echo or a measured impulse. Excluded zeros in C are
preserved, so they contribute their actual periodic mask response. H remains the
configured physical transfer even on unused/excluded signal carriers.

ConfiguredResponseSnapshot retains source/reference-plane/units, optional
configuration generation and run ID, original base grant/profile/IUC and selected
grant symbol, C bank generation when applicable, FFT/sample clock/spacing,
original viewport, physical delay/phase/frequency-offset/noise settings, whether
RxMER residual impairment is configured, frequency points and optional periodic
impulse. A report with no started grant has no fabricated retained run/generation
identity. Each frequency point has original carrier, centered baseband Hz,
excluded flag, complex transfer, optional amplitude dB and phase radians. Impulse
points retain original periodic sample index/time seconds, complex value and
optional dB relative to the complete-period peak. Exact zeros have no logarithmic
or phase reading; use no arbitrary floor. Reject nonfinite/overflowing numerical
results. Zoom cannot alter the complete impulse, source settings or identities.

Native Python mirrors both methods as
`configured_response(source="physical_channel", grant_symbol=0, first_carrier=0,
carrier_count=0, maximum_points=4096, include_impulse=False) -> dict`.
Checked strings select the Rust enum; native conversion releases the GIL, reuses
Rust validation and returns owned ordinary data. Introspection defaults must be
tested against actual installed signatures. No response/phase/impulse math in
Python or Qt, and no new scientific dependency between PHY crates.

| Inspected owner/files | Contract / missing observation | Decision / destination |
| --- | --- | --- |
| `crates/ofdma/src/signal_processing/pre_eq.rs`, `crates/ofdma/src/measurements/pre_eq.rs` | Original normalized corrective C, excluded zeros and admitted per-symbol replay | Reuse coefficients/at/generation; no event or inverse-H reconstruction. |
| `crates/ofdma/src/channel/stream.rs` | Actual causal FIR, independent noise, delay FIFO and emitted-clock phase rotation; no static transfer observation | Adapt deterministic settings in proposed `crates/ofdma/src/instruments/response.rs`; exclude noise/residual and reject time-varying H. |
| `crates/ofdma/src/signal_processing/transform.rs`, `crates/dsp/src/transform.rs` | Shared centered unitary FFT/IFFT with independent existing vectors | Reuse plans/normalization; keep upstream geometry and reference semantics separate from downstream response owners. |
| `crates/ofdma/src/api/session.rs`, `crates/ofdma/src/instruments/mod.rs`, `crates/ofdma/src/api/mod.rs` | Idle/frozen configuration and high-level export owners | Add typed projection wrapper/re-exports; no receiver truth input or existing report/schema changes. |
| `bindings/python/src/upstream/simulation.rs`, `bindings/python/src/upstream/reports.rs`, `bindings/python/src/upstream/mod.rs`, `bindings/python/python/merlin/upstream.pyi`; proposed `bindings/python/src/upstream/simulation/response.rs`, `bindings/python/src/upstream/reports/response.rs`, `bindings/python/src/upstream/response.rs` | Existing session guard and owned report/native dictionaries | Add thin dispatch and one shared string/request converter for two real consumers, with identical checked defaults and errors. |
| Proposed `crates/ofdma/tests/api_configured_response.rs`, `bindings/python/tests/test_upstream_configured_response.py`, `crates/ofdma/examples/upstream_configured_response.rs`; `tools/testing/native_api.py` | New numerical/public/native contract requires independent acceptance | Literal unity/complex FIR/delay/two-tap phase and periodic impulse, C normalization/replay/masks, all planes/both numerologies, zoom/zero/missing/bounds/overflow/unsupported-CFO, source/noise invariance, Busy/ownership and separate-process parity. |
| Proposed `docs/user/upstream/response.md`; upstream/Python guide indexes | New configured diagnostic versus unavailable estimated/soft observations | Add a focused guide; preserve API-04 pending estimate/soft/constellation gaps. |

The [unit acceptance](validation.md#uiu-api-04-configured-response-and-periodic-impulse-unit)
records eight focused cases, 103 complete upstream/16 facade/124 installed native
passes, core/native Clippy and 77 tooling passes. The first complete native run
had an existing second-pause timeout; the unchanged case passed alone and in 50
diagnostic repetitions, followed by a passing full-suite retry of the exact wheel.
Original failure/reference hashes remain recorded; no timeout or test was removed.
Actual receiver-estimate/soft retention remains subsequent API-04 work, and the
parent task stays unchecked. The [user guide](../../../user/upstream/response.md)
distinguishes configured deterministic and periodic data from actual received
observations and unaliased causal echoes.

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
