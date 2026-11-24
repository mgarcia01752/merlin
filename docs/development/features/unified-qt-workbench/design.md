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
- [Parallel implementation ownership](parallel-development.md)

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

The initial audit identified three separate gaps. First, the UI needs bounded typed FEC
paging and scalar verification summaries without copying/parsing an entire report
or calculating scientific aggregates in the adapter. Second, receiver pilot
estimates and per-cell/word soft observations were not retained by the original
report; they cannot be recovered by reading the final decoded bits. Third,
response/impulse and constellation reference queries need authoritative Rust
operations with explicit C, H and estimated C×H planes. The configured-response
and actual estimate-retention, soft and display units below have scoped acceptance.
The [final backend gate](validation.md#uiu-api-04-upstream-instrument-backend-accepted)
also passes, completing API-04. Per-unit statements about remaining gaps below
record their historical acceptance boundaries; the official checklist owns
current status. Qt adoption remains separate.

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
algorithms and all Qt sources remain unchanged. At that unit's acceptance,
UIU-API-04 stayed unchecked for subsequent instrument gaps, now accepted above.

### UIU-API-04 constellation, pairing and received-response display unit

REQ-UIU-02/12/16/18/24 require the remaining authoritative display contracts.
This design preceded implementation; [scoped acceptance](validation.md#uiu-api-04-qam-pairing-and-received-response-display-unit)
now passes, followed by the final parent backend gate linked above. Existing raw QAM/PSD, actual pilot
estimates and soft/FEC views stay authoritative; no new simulation interface.

Add `BatchReport::qam_constellation(QamConstellationRequest)` for actual retained
`TxGrid` or `RxEqualized` cells at one original payload loading. Loading zero
selects known pilots and uses BPSK reference geometry without relabelling pilot
payload bits. Retain the original canonical Cell.gain alongside each point in a
private bounded trace ledger omitted from serialization. The existing recorder
already receives this authoritative ResourcePlan cell; no resource plan is rebuilt
by a query and no configured replacement can invent an original gain. Return each original QamPoint paired with its backend
unit-reference value; TX removes its actual prescribed role gain, while RX
equalized cells are already at unit reference. Reuse upstream `map_symbol` for
every labelled ideal reference and explicit MSB-first labelling/normalization.

The new request has tap, loading, maximum_points and maximum_references. Default
selects RxEqualized, loading four and independent 4096 observed/reference-point
budgets. Name the existing raw-QAM copied-point ceiling once and reuse it for the
same observed-point purpose; keep the reference-geometry budget separate despite
its equal value. Validate both limits and supported order before copying.
Return original identities/coordinates, actual tap, exact normalization plane,
whole retained matching-point count and whether the display is capped. Existing
raw QAM remains unchanged. TxPreEq and RxFft have different physical planes;
unit-reference overlays reject there rather than borrowing configured H/C or
assuming a stationary correction. Their actual raw observations remain available.

Add `BatchReport::qam_pairing(QamPairRequest)` to match two actually retained
stage traces by exact run/generation/grant/frame/symbol/physical carrier. Request
fields are left/right taps, optional loading and maximum_pairs (default TxGrid,
RxEqualized, all loadings and 4096 pairs). Use a separate named pair-copy policy.
Scan the already bounded histories, validate duplicate coordinates and return
original point pairs in left-record order, whole selected/matched/unpaired counts
and explicit display capping. No source-truth replay, interpolation or waveform
matching. Unretained taps are unavailable; retained empty/partial observations
can return empty pairs without claiming successful reception. The raw stage
planes remain distinct; pairing does not normalize channel effects.

Extend non-exhaustive ReceiverEstimatePoint with optional amplitude_db and
phase_radians from the actual optional estimate. Missing or exact-zero values
have no log/phase reading. Share the existing configured-response robust complex
magnitude/log/phase/frequency projection in a focused `instruments/transfer.rs`
owner; keep its twenty-log amplitude convention and configured error fields.
Actual received queries use their own error context. Do not infer a complete
impulse from missing frequency support. Configured periodic impulses retain their
existing explicit aliasing contract.

Python mirrors the two typed report operations as owned dictionaries with checked
tap names and matching defaults. Rust projection/query/serialization releases
the GIL; no adapter role normalization, reference construction, association or
response math. Existing query signatures and default whole-report JSON remain
unchanged. Re-export actual QamPoint/CellRole under the high-level upstream API
so consumers need no internal scientific module imports.

| Inspected owner / files | Current contract and gap | Decision / concrete destination |
| --- | --- | --- |
| `crates/ofdma/src/instruments/observer.rs::qam`, `QamPoint`, private Trace | Bounded actual stage points; no authoritative unit reference or pairing | REUSE original observations/coordinates; retain original Cell.gain privately without changing serialized defaults, name the existing query ceiling and permit focused sibling views to read bounded traces. |
| `crates/ofdma/src/resources/grid.rs::ResourcePlan`, `signal_processing/qam.rs::map_symbol` | Authoritative role gains, source geometry/labels and unit energy | REUSE unchanged owners; no duplicate pilot/resource or constellation algorithm. |
| Proposed `crates/ofdma/src/instruments/constellation.rs`, `pairing.rs`; `instruments/mod.rs`, `api/mod.rs` | Missing typed bounded display/association operations | ADD focused queries/models and EDIT high-level exports. |
| `crates/ofdma/src/instruments/response.rs`, `receiver.rs`; proposed `transfer.rs` | Configured amplitude/phase projection already exists; received view lacks it | SHARE identical numerical projection for two real consumers, preserving source/error/reference-plane distinctions. |
| `bindings/python/src/upstream/reports.rs`; proposed `upstream/reports/qam_views.rs`; upstream stubs | Existing Rust-owned report conversion and tap-name admission | ADD thin GIL-released dispatch and matching declarations; validate actual estimate fields through the new paired native cases. |
| Proposed `crates/ofdma/tests/api_qam_views.rs`, `crates/ofdma/examples/upstream_qam_views.rs`, `bindings/python/tests/test_upstream_qam_views.py`; existing estimate/response tests | Remaining display acceptance absent | ADD independent labels/gains/complex dB/phase anchors, actual coordinate pairings, original counts/caps, both modes, unavailable/unsupported/partial states, passive noisy outcomes and installed Rust/native parity. |
| Proposed `docs/user/upstream/qam-views.md`; existing response/estimate/Python guides | Scientific display planes and unsupported overlays need explicit user documentation | ADD focused guide and reconcile current acceptance without restructuring historical records. |

Validate the changed scientific/API owners and installed namespace/observation
consumers. Preserve original operation rows when appending the two methods;
archive the previous received-query row before adding new field-level evidence.
After all required instrument gaps close, perform the API-04 integration gate
before checking the parent task. Full regression evidence remains separate from
the per-unit scoped passes and from future Qt consumer acceptance.

### UIU-API-04 actual receiver soft-metric retention unit

REQ-UIU-02/12/19/24 next require actual normal-receiver demapper and LDPC-input
metrics. This design preceded implementation; its
[scoped acceptance](validation.md#uiu-api-04-actual-receiver-soft-observation-unit)
now passes. The API-04 parent remains open for remaining instrument contracts.
Decoded decisions cannot reconstruct these observations. Existing decoding,
mapping, randomizer recurrence and source-verification behavior stay authoritative.

Add immutable `SoftObservationPolicy::new(maximum_cells, maximum_words,
maximum_word_llrs)` and `info()`. Independent application ceilings are 1024
computed payload/complementary cells, 32 completed words and 262144 retained
word-input LLRs across all grants. Cell count separately bounds the two actual
per-bit vectors at admitted loading. These limits do not borrow equal-valued FEC
page or estimate-budget constants. Zero word count requires zero word-LLR budget;
positive word retention requires a positive budget. All-zero policy disables it.

`ReceiverObservationPolicy::with_soft_metrics(policy)` returns a new immutable
policy, preserving its existing estimate limits. Default estimate policy can
enable soft-only observation. Private optional fields and omitted disabled JSON
fields preserve existing constructors, policy info and default report shape.
The existing session setter/getter admits the combined policy; no parallel run API.

At the normal receiver's already computed max-log and soft derandomization
boundary, retain original identity/frame/symbol/carrier/role/loading, corrected
optional unit-reference I/Q, declared complex reference variance, original coded
offset, actual MSB-first demapper LLRs and actual derandomized LLRs. Positive LLR
favors zero and the values are natural-log P(0)/P(1). Report the existing Rust
max-log clipping limit from its owner, rather than duplicating it in adapters.
Pilot cells are excluded; missing cells retain actual zero-erasure metrics.
Computed-cell observations do not claim every bit reached a completed codec call
if cancellation occurs inside the cell.

After an actual codec call completes, retain the exact transmitted-position
input LLRs before the receiver clears its word buffer. Preserve original word
ordinal/coded offset, completion frame, actual code family/shortening dimensions,
iterations and independent parity/convergence/shortening/fill outcomes. Do not
claim posterior LLRs, insert shortened known-zero priors into observed inputs or
rerun a decoder. Reuse existing code-family naming for recovery and soft views.

Extend the existing private aggregate observer, sharing immutable retained records
for transactional clones. Transfer it across grants as today. Rejection rolls back
observations with science; cancellation retains computed cells and completed
words while leaving an incomplete word unavailable. Exhaustion never changes
randomizer state, confidence, source/RNG consumption or decoding.

Exact new report operations are `soft_observations() -> SoftObservationSummary`,
`soft_cells(SoftCellRequest) -> PhyResult<SoftCellPage>` and
`soft_word(SoftWordRequest) -> PhyResult<SoftWordSnapshot>`. Summary distinguishes
observed/retained/omitted counts and inventories bounded original word identities.
Cell pages default to first index zero, 32 cells and 4096 copied LLRs; application
ceilings are 128 cells and 4096 copied LLRs. Count both original per-bit vectors
before copying. End positions return an empty page; missing history is unavailable.
Word queries default to retained index zero and 32768 copied input LLRs, with that
independent finite ceiling; insufficient copy budgets reject before allocation.
These query defaults do not share policies with retention or other instruments.

Python mirrors policy construction/info, immutable policy composition and all
three report queries, releasing the GIL for Rust query/serialization. Native
adapters perform no demapping, mask inversion or scientific aggregation.

| Inspected owner / files | Existing contract and gap | Decision / concrete destination |
| --- | --- | --- |
| `crates/ofdma/src/receiver/framing.rs::decode_frame` | Actual raw/derandomized cell and transmitted word-input LLRs are transient | EDIT passive opt-in recording at these exact boundaries; preserve processing order and cancellation. |
| `crates/ofdma/src/signal_processing/demapping.rs::max_log`, `signal_processing/mod.rs` | Existing finite natural-log metrics clamp at a literal limit | REUSE unchanged numerical convention; name the existing limit once for kernel and report metadata. |
| `crates/ofdma/src/instruments/receiver.rs`, `observer.rs` | Accepted opt-in aggregate estimate owner and transactional shared records | ADAPT private observer to optional soft-only or combined retention, preserving existing public fields/default serialization. |
| `crates/ofdma/src/instruments/recovery.rs::fec_words` | Existing code-family/dimension projection | SHARE actual family naming with the soft-word owner; preserve recovery behavior. |
| Proposed `crates/ofdma/src/instruments/soft/{mod,policy,store,views}.rs`; `instruments/mod.rs`, `api/mod.rs` | Missing typed soft retention/query models | ADD focused actual policy, storage and view owners; EDIT public re-exports. |
| `bindings/python/src/upstream/receiver_observations.rs`, `upstream/mod.rs`, `bindings/python/python/merlin/upstream.py`, `upstream.pyi`; proposed `upstream/reports/soft.rs` | Existing immutable policies and native report conversion | EDIT policy composition/registration and ADD thin report dispatch with installed signatures/ownership parity. |
| Proposed `crates/ofdma/tests/api_soft_observations.rs`, `crates/ofdma/examples/upstream_soft_observations.rs`, `bindings/python/tests/test_upstream_soft_observations.py`; existing receiver-boundary tests | Retention and query acceptance absent | ADD independent sign/scale/erasure anchors, reconstructed actual word-input continuity from observed cells, code families, cancellation, limits, passive noisy outcomes and separate Rust/native parity. |
| Proposed `docs/user/upstream/soft-observations.md`; upstream/Python indexes | LLR support still described as pending | ADD actual scientific planes and limitations, then reconcile scoped acceptance. |

Run changed soft and receiver/experiment owners, installed observation/namespace
cases, formatting, scoped Clippy and exact API tracing. Keep full final integration
regressions distinct from this unit. Remaining constellation/reference and
response-display gaps keep API-04 open until their own acceptance passes.

### UIU-API-04 received-pilot estimate retention unit

REQ-UIU-02/12/18/24 require actual normal receiver estimates, separately from
configured-response diagnostics. The design preceded implementation; the
[scoped acceptance](validation.md#uiu-api-04-actual-received-pilot-estimate-unit)
now passes. API-04 remains open for remaining response-display and constellation references.

Add `ReceiverObservationPolicy::new(maximum_frames, maximum_scalar_slots)`
with private fields, `Default` disabled and an `info()` projection. Independent
application ceilings are eight retained frames and 262144 scalar slots. Charge
two scalar slots per complete physical FFT bin, including unavailable bins.
These limits have separate owners from equal-valued FEC page limits. Enabled
retention requires space for one estimate in the accepted numerology.

`Simulation::set_receiver_observation_policy(policy)` and
`receiver_observation_policy()` replace/read the opt-in atomically. Validate all
configuration entrypoints before adoption. Preserve configuration generation,
source, options, existing constructors and public ExperimentOptions fields.
Existing public `simulation::run` remains disabled; the high-level session uses
the same loop through internal dispatch with its explicit observation policy.

At each completed receive frame, move the already calculated optional estimate
vector into a bounded owner. Transactional receiver clones share immutable
retained vectors. No extra FFT, fit, random draw or sample replay. Transfer the
observer across grants so its budget is global. Capture includes completed
frames during channel drain and receiver finish. Cancellation preserves completed
frames; interrupted partial frames claim no complete estimate. Invalid receive
transactions preserve accepted history.

`BatchReport::receiver_observations()` returns policy, completed/retained/dropped
counts, used scalar slots and retained identities.
`receiver_estimate(ReceiverEstimateRequest)` selects retained index and original
carrier viewport with a separate 4096-point copy ceiling; zero count means the
remaining FFT. Return owned optional coefficients, original run/grant/generation/
frame, estimator, FFT/sample-clock/frequency metadata and explicit
`received_pilot_effective_c_times_h` plane. Unknown bins stay unavailable; zero
estimates stay actual zeros. Do not infer an impulse from incomplete frequency
support or fill holes with configured C/H. Validate bounds before copying;
unretained frames return unavailable.

Python mirrors policy construction/info, session setter/getter and report queries
through the installed native extension, with GIL release for Rust work. Default
reports omit the optional observation field, preserving the old JSON shape.
Project persistence remains API-05.

| Inspected path/symbol | Current contract / gap | Decision and owner |
| --- | --- | --- |
| `crates/ofdma/src/signal_processing/estimation.rs::estimate` | Actual optional C×H bins; discarded after frame | REUSE unchanged algorithm; move completed values from `receiver/framing.rs`. |
| `crates/ofdma/src/receiver/framing.rs::decode_frame`, `push_with_control` | Frame completion and transactional rollback | EDIT optional recorder; immutable shared frames keep transactional clones bounded. |
| `crates/ofdma/src/instruments/observer.rs::ObservationStore` | Existing I/Q budget and report shape | EDIT optional private store; ADD `crates/ofdma/src/instruments/receiver.rs` for policy, recording and queries. |
| `crates/ofdma/src/simulation/run.rs::run`, `api/session.rs::Simulation` | One loop, stable constructors/options | ADAPT internal dispatch and observer transfer; preserve public disabled run. |
| `crates/ofdma/src/instruments/mod.rs`, `crates/ofdma/src/api/mod.rs` | Public export registration | EDIT typed exports; retain facade re-exports. |
| `bindings/python/src/upstream/mod.rs`, `upstream/simulation.rs`, `upstream/reports.rs`, `bindings/python/python/merlin/upstream.pyi` | Registration, Busy guard and owned reports | EDIT wiring; ADD `upstream/receiver_observations.rs`, `simulation/receiver_observations.rs`, `reports/receiver_observations.rs`. |
| Proposed `crates/ofdma/tests/api_receiver_estimates.rs`, `crates/ofdma/examples/upstream_receiver_estimates.rs`, `bindings/python/tests/test_upstream_receiver_estimates.py` | No retention acceptance yet | ADD independent complex-gain/plane, both-mode, disabled/passive, budget, identity, partial, rejection and Rust/native parity cases. |
| Proposed `docs/user/upstream/receiver-estimates.md`; upstream/Python indexes | Estimate support currently pending | ADD focused guide; reconcile after actual validation. |

Validate new cases and existing receiver/experiment owners, installed native
namespace/estimate cases, formatting and scoped Clippy. Reuse proven unchanged
reference outputs with hashes rather than regenerating downstream pipelines for
this unit. Full regression remains the final integration gate. Earlier passes
do not establish acceptance of this new unit.

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
At this unit acceptance, receiver-estimate/soft retention was subsequent API-04 work; the
parent task stays unchecked. The [user guide](../../../user/upstream/response.md)
distinguishes configured deterministic and periodic data from actual received
observations and unaliased causal echoes.

### Existing owner decisions

| Inspected owner | Current contract/evidence | Gap | Decision and destination |
| --- | --- | --- | --- |
| [Facade](../../../../crates/merlin/src/lib.rs), public direction re-exports | Thin crate; both independent PHYs already usable; accepted backend compatibility | Qt still imports downstream internals | REUSE facade; ADAPT Qt call sites after typed API extensions, never add science to facade. |
| [Qt organization](../qt-source-organization/design.md), [build registration](../../../../apps/qt/build.rs) | Accepted UI-ORG-01/02; pinned CXX-Qt bridge directory constraint | No direction split | ADAPT group ownership; retain all registered bridges in `src/bridge/`, same QML URI, entry URL and executable. |
| [Closed loop](../../../../apps/qt/src/downstream/simulation/closed_loop.rs), `latest`/`Observations` | Bounded downstream worker and retained observations; inline tests | Private high-level-API gap; thread-local single result | SEPARATE per direction; scientific operations move behind public API, adapter retains dispatch and ownership only. |
| [Downstream inputs](../../../../apps/qt/src/downstream/inputs), [project file](../../../../apps/qt/src/downstream/workbench/project_file.rs) | Existing editors, capture admission, Designer and schema 1/2 persistence | Validation/preview still depends on lower scientific owners | ADAPT through public typed admission and backend project contracts before deleting legacy routes. |
| [Native input crate](../../../../crates/pnm-input/src/lib.rs), [workbench](../../../../crates/pnm-input/src/workbench) | Bounded binary/JSON, provenance, saved-workbench execution; Apache-2.0 | Initial facade exposure is now implemented; direction-aware unified project envelope remains pending | REUSE owners; optional facade input feature/re-exports, extend adapter-owned project contract, preserve existing imports. |
| [Upstream session](../../../../crates/ofdma/src/api/session.rs), `preview`/`run_with_control` | Validated configuration and bounded controlled runs already accepted | Grid coordinates and some instrument views not public API | REUSE session; small observation extensions in upstream API, scientific construction stays in owning modules. |
| [ResourcePlan](../../../../crates/ofdma/src/resources/grid.rs) | Authoritative physical cell/pilot allocation | High-level `ExperimentPreview` gives counts, not cell coordinates | ADAPT via bounded idle grid operation; no QML resource reconstruction. |
| [SpectrumPlot](../../../../apps/qt/qml/downstream/instruments/spectrum/SpectrumPlot.qml), [scatter](../../../../apps/qt/qml/downstream/instruments/qam/TransmitterScatter.qml), [response](../../../../apps/qt/qml/downstream/instruments/response/ReceiverResponseDialog.qml) | Existing downstream Canvas/viewport presentation and QtTests | No real upstream consumers; direction-specific labels/overlays mixed in | DEFER scientific shared acceptance; extract proven presentation only after two real API-backed consumers and tests. |
| [FEC presentation](../../../../apps/qt/qml/downstream/instruments/fec), [receiver providers](../../../../apps/qt/src/downstream/instruments/receiver) | Actual downstream counters/LLR/iteration views | Upstream LDPC data differs from downstream BCH/control chains | ADAPT chart/table widgets; SEPARATE semantics, code families and unavailable states. |
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


## UIU-API-05 direction-aware public project admission

REQ-UIU-02/13/22/23/24 require idle, self-contained, bounded projects through both
APIs. API-01 and API-04 are accepted prerequisites. This design precedes API-05
implementation; its parent remains unchecked until both directions, installed
Python, compatibility and aggregate-input gates pass. Qt chooser/dirty/write
and recovery transport remain FILE-01/RECOVERY-01.

The Apache-2.0 native input adapter owns project admission. The facade only
re-exports it under optional `native-input`; the `merlin-api` leaf gains no PHY
or adapter dependency. Independent PHYs and old imports remain unchanged.

### Selected envelope and limits

Schema 1/2 bytes retain downstream interpretation and exact original JSON.
Schema 3 has exactly `schema_version: 3`, `application: "MERLIN"`,
`direction: "downstream" | "upstream"` and direction-specific `contents`.
Unknown typed fields/versions and mismatched payloads reject. Downstream contents
hold `workbench` (the exact original UTF-8 schema 1/2 document string) and optional
`observations` (the existing typed FullLoopObservationInput). This keeps original
embedded bytes/recipes/settings without a second base64 layer or changing the
legacy schema. No automatic migration rewrites user files.

Upstream contents hold `input` with exactly `kind: "configuration" | "measured"`
and `document` (exact original configuration/measured-plan JSON string), explicit
`options` (grants, maximum_samples, chunk_samples, timeout_ms, retained_points,
retained_taps) and `receiver_observations` (maximum_frames,
maximum_scalar_slots and optional soft maximum_cells/maximum_words/
maximum_word_llrs). Existing constructors validate every execution/observation
setting, including mode-specific admission. C, H, synthetic plans and captured
source lineage remain owned by their existing documents, never reconstructed by
Qt. Loading builds an idle accepted candidate; an explicit public request/session
run remains necessary. Saving never serializes results, live controls or RNG state.

The per-document budget remains 144 MiB, embedded originals 16 MiB each and total
originals 96 MiB. The existing upstream input document limit remains 16 MiB.
A collection admits at most two distinct directions, at most 288 MiB serialized
project bytes and at most 96 MiB combined retained original inputs. These are
separate named application policies, independent of decoder/sample/result bounds.
All additions/counts are checked before decoding/retention. Serialization is
bounded and returns bytes/string only; safe destination replacement belongs to
Qt. A maximal legacy input may fail schema-3 export if wrapper overhead exceeds
the unchanged file budget; it remains readable without truncation.

### Operations and implementation units

First add `Workbench::prepare_full_loop(bytes: &[u8]) -> ApiResult<FullLoopSweepPlan>`
in `crates/pnm-input/src/workbench/full_loop.rs`. Its native static mirror is
`Workbench.prepare_full_loop(data: bytes) -> FullLoopSweepPlan`. The existing
Workbench loaders/runs retain their legacy contract. This new entry point uses
the same strict legacy DTO/integrity decoder and Designer admission, but builds
the accepted FullLoopInputs/MeasurementSequence/SourceInput/ChannelConfiguration
and sweep owners. It supports actual UI file/fill sources and single/average/
hold/interpolate modes without a second engine. Capture selection retains stored
inventory order; profile selection retains schema-version precedence and sorted
checked IDs. Profile/H time tolerance stays distinct from RxMER/profile tolerance.
The audit also finds that the existing full-loop constructor uses the RxMER
association tolerance for H. Add the compatible public
`Simulation::prepare_full_loop_with_response_tolerance(inputs, options, tolerance)`
and native static mirror, plus `FullLoopRequest::response_tolerance()` / native
property. The existing constructor keeps its behavior. The frozen request and
sweep carry the separately accepted H tolerance through every selected profile;
no tolerance is widened or silently substituted. This is an admission-only
extension in `crates/ofdm/src/api/full_loop/{request,sweep}.rs` and `full_loop.rs`,
with native conversions in `bindings/python/src/full_loop/request.rs`.
All selected/unselected original hashes are checked before admission. Run options
retain duration, noise seed, iterations, receiver mode, MCE, filter, loss and power.

Then add adapter-owned `Project`, `ProjectDirection`, `ProjectError` and resource
metadata in `crates/pnm-input/src/project/`. Public operations are `from_bytes`,
`from_file`, `direction`, `original_document`, `to_json`, `downstream_plan`,
`upstream_simulation` and `collection_info`; they expose existing owned typed
requests/sessions, not a generic simulation interface. `merlin::common` re-exports
these optional native-input contracts. Python `merlin.common.Project` mirrors
them using existing native request/session classes, structured shared categories,
owned copies and GIL release for admission. Wrong-direction access fails without
mutating the accepted project. Per-direction API configuration stays separate.

### Reuse and file map

| Path and symbol | Inspected contract / gap | Decision and destination |
| --- | --- | --- |
| `crates/pnm-input/src/workbench.rs` Document/Stored/Workbench | Strict legacy integrity, original ownership, Designer; old execution omits file/timeline modes | REUSE DTO/decoding in child `workbench/full_loop.rs`; preserve old loader/run behavior. |
| `crates/ofdm/src/api/full_loop.rs`, `full_loop/sweep.rs`, `sequence.rs`, `admission.rs`, `channel.rs` | Accepted typed source, playback, channel, request and complete-catalog preflight | REUSE scientific admission and execution; no new scientific calculations. |
| `crates/pnm-input/src/upstream/association.rs`, `crates/ofdma/src/configuration/document.rs`, `api/session.rs` | Exact original documents, measured association/lineage and atomic idle configuration | REUSE for upstream project candidates, explicit options and observation policies. |
| `crates/merlin/src/lib.rs` | Re-export-only optional input feature | EDIT curated common project exports after implementation; preserve all old exports. |
| `bindings/python/src/workbench.rs`, `full_loop/sweep.rs`, proposed `project.rs`; upstream simulation wrapper | Existing native objects and Busy/error conversion | ADAPT ownership conversion only; import-only modules/stubs remain aligned. |
| Proposed `crates/pnm-input/tests/workbench_full_loop.rs`, `project.rs`; native project tests and Rust example | Direction/schema/idle/round-trip and actual-run evidence missing | ADD targeted independent input/settings/bounds/cancelled and actual engine parity cases. |
| `apps/qt/src/downstream/workbench/project_file.rs` | GPL presentation file lifecycle and UI-produced legacy schema | REUSE schema fixtures as compatibility evidence; no Qt code migration in this API unit. |

Acceptance covers old fixtures and a saved Designer recipe, exact original bytes,
file fill/bit order, temporal order/capacity, receiver/channel settings, unsupported
or corrupt/version/hash/aggregate bounds, atomic rejection, owned snapshots and
bounded deterministic actual runs. Rust/native comparisons use a separate Rust
process. Record actual elapsed times. Preserve every prior matrix row/assertion,
append new operations and keep final installed/Qt acceptance distinct.

### UIU-API-05 saved downstream full-loop unit

The first implementation unit adds idle `Workbench::prepare_full_loop` and the
independent response tolerance operation described above. Existing legacy loaders
and constructors retain their original behavior. Native document versions and
shared saved-input budgets now belong to
`crates/pnm-input/src/common/constants.rs`; `workbench::MAX_WORKBENCH_BYTES`
remains a public re-export. Channel model and channel power versions have separate
names because their formats evolve independently. These input limits are separate
from scientific sample, observation and result budgets.

The [scoped acceptance record](validation.md#uiu-api-05-saved-downstream-full-loop-unit)
closes only this prerequisite. Schema 3, upstream projects, aggregate admission and
Qt file/recovery adoption remain pending under the parent task.

### UIU-API-05 project envelope implementation details

`Project` retains exact admitted document bytes and direction-specific original
contents; it does not retain or advance a running simulation. `to_json()` emits
schema 3 while keeping the embedded legacy/downstream or upstream document string
unchanged. Downstream observation input is optional and validated through the
existing public policy before complete-catalog preflight. Upstream options and
receiver/soft policies use explicit strict transport DTOs and the existing public
constructors; configuration/measured admission precedes applying saved overrides.
Returned plans/sessions own their inputs independently of the project.

The collection operation accepts at most two references with distinct directions.
It reports project count, original serialized document bytes and original-input
bytes. Downstream original-input accounting includes every embedded original,
including dormant/unselected inputs. Upstream accounting includes exact original
configuration/measured document bytes plus each captured source inventory once;
project wrapper bytes have their separate serialized budget. Checked arithmetic
and the combined 96 MiB original-input ceiling precede any collection acceptance.
Scientific runtime/result memory is a separate SESSION-01 contract.

`ProjectError` retains the original directional error and exposes the shared
`ErrorCode`; the native adapter forwards it through the existing structured
exception converters. Native `Project.direction` returns the language-appropriate
string `downstream` or `upstream`; `collection_info` returns an owned dictionary.
No new native engine, generic configuration or Qt-specific scientific interface
is introduced. Serialization remains bounded and writes no destination.

The envelope parser uses Serde's borrowed/raw JSON boundary for `contents`, then
per-direction strict DTOs. The `raw_value` feature on the already pinned serde_json
preserves duplicate-field rejection in nested contents rather than first folding
objects into a last-value-wins dictionary. No new package/version is introduced.
The native collection converter checks length first and accesses only that bounded
index range, independently of a Python object's iterator implementation.

Upstream generic configuration JSON uses its existing 16 MiB ceiling from
`crates/ofdma/src/common/constants.rs`. Captured-plan input and measured provenance
output have separate named policies in the native adapter's common constants;
their equal current byte values do not couple their formats or evolution.

### UIU-API-05 as-built ownership and acceptance

The [accepted public project backend](validation.md#uiu-api-05-project-backend-accepted)
implements the selected contract above. `crates/pnm-input/src/project/mod.rs`
owns the strict envelope, original bytes, direction access and collection metadata;
`project/upstream.rs` composes existing upstream scientific owners and
`project/error.rs` preserves original directional causes. `project/tests.rs`
checks aggregate cached-byte boundary/overflow without large payload allocation.
`workbench/full_loop.rs` composes downstream admission with its optional existing
observation policy before complete-catalog preflight.

`crates/merlin/src/lib.rs` only re-exports optional native-input Project contracts.
`bindings/python/src/project.rs` owns checked conversion and independent native
handles; the existing upstream session slot supplies Busy/run/cancellation behavior.
Import-only `common.py` and `common.pyi` expose the real class and matching signatures.
`crates/pnm-input/tests/project.rs`, the facade namespace test and installed
`bindings/python/tests/test_project.py` cover both directions and compatibility.
`examples/api/rust/run_project.rs` supplies an independently executed reference.

Native input and upstream configuration limits use their owning common constants
modules. The existing pinned serde_json raw-value feature preserves strict nested
field admission. No scientific implementation, new crate, dependency cycle or
Qt component was introduced. The neutral API leaf and independent PHY contracts
remain unchanged. The user guide is [Direction-specific projects](../../../user/api/projects.md).
Qt file/dirty/recovery adoption and final integration remain pending tasks.

## UIU-ORG-01 shared presentation organization

REQ-UIU-04/25 and the canonical source-ownership section authorize the next
presentation-only move. Follow the pinned inventory: move the fifteen QML files
assigned to ORG-01 from `qml/{components,theme,about}` into the corresponding
`qml/shared/` folders. Their existing type names, properties, signals, handlers,
styles, geometry and notices remain unchanged. Rebase relative imports and dynamic
component URLs at all existing consumers and tests, then update explicit build
registration. The module remains `Merlin.Workbench`, with the same Main entry URL,
QObject entry files, executable, branding resources and native launch identity.

| Inspected owner and contract | Decision / exact destination |
| --- | --- |
| `qml/components/` WorkbenchDialog/ActionButton/ApplyCancelFooter, blocks/connections/chips/status, ChartNavigation and TlvTreeGrid | SHARE only generic presentation under `apps/qt/qml/shared/components/`; inputs are caller-provided theme/rows/viewports, with no scientific decoding or engine state. |
| `qml/theme/` Theme/ThemePalette | SHARE existing colors, sizing and one tooltip-delay owner under `apps/qt/qml/shared/theme/`; preserve dark/light propagation and current values. |
| `qml/about/` About/Faq/License dialogs | SHARE application information/help presentation under `apps/qt/qml/shared/about/`; retain actual bridge providers and notices. |
| `src/bridge/` entry modules and `src/platform/` native infrastructure | REUSE in place for pinned CXX-Qt registration and current platform ownership; no source move or science extraction. |
| `qml/shell/` coupled workbench/menu/settings/update views | DEFER their inventory-assigned moves to UIU-SHELL-01; current downstream and update lifecycle remain unchanged here. |
| `apps/qt/build.rs`, QML consumers and `apps/qt/tests/` | EDIT only resource paths/imports needed by these moves; no URI/type/executable or source-feature change. |
| Canonical skills and maintained source links | EDIT discovery/navigation paths to moved files; preserve original pinned inventory/evidence as historical records. |

Keep scientific inputs/instruments in their current direction until DS-01 and the
shared-instrument tasks establish their separate API boundaries and consumers.
No empty symmetric source trees are needed. Record exact original/destination
hashes and every adapted consumer in the organization evidence. Existing Qt Rust
and QML cases, the ordinary dependency guard, native build and fresh native startup
must pass. This organization has no visible UI behavior change; existing user-guide
visuals remain applicable. Native smoke must retain real dialog and theme loading.

The initial move preserves all fifteen QML file contents byte-for-byte. Existing
consumers/tests require only import-path changes, and build registration retains
all type names, the module URI and Main resource entry. Six downstream backfill
route owners are current locators and move to the new directories; their IDs,
tasks, historical acceptance and original pinned evidence are unchanged. Empty
former directories are removed so link checks also reflect a clean checkout.
The initial standalone boundary invocation lacked the Qt Cargo environment; the
supported Qt check wrapper supplies its pinned cache/toolchain and passes that
boundary check. This is an environment invocation correction, not a bypass.

## UIU-DS-01 downstream placement prerequisite

REQ-UIU-02/03/04/08/09 and the accepted inventory govern downstream
adoption. First isolate existing downstream owners without changing scientific
operations: the inventory has 104 actual moves and nine bridge entries retained
in place. Rust inputs/instruments/simulation/workbench/help move under
`apps/qt/src/downstream/`; QML moves to its approved direction-specific destinations,
including `downstream/DownstreamWorkbench.qml`. Root Main and tests instantiate
that renamed presentation type while keeping the module and Main entry URL.
CXX-Qt QObject entry files stay together. A small `downstream/mod.rs` declares
existing child owners; there is no new engine or duplicate scientific logic.

Rebase internal Rust module paths and literal compile-time resource paths, relative
QML imports, explicit build registrations, live owner/test locators and skills.
Do not change scalar values, handlers, settings, source vectors, scientific crate
calls or serialized contracts in this placement unit. Merge only identical QML
imports when approved destinations converge. All original tests remain exercised
through their new module/file locations, with original function names preserved.

The accepted boundary plan explicitly permits reviewed file moves to update exact
legacy locators without expanding operations. Preserve the [original guard record](legacy-qt-boundary-initial.json)
verbatim as a historical initial record, alongside the [current locators](legacy-qt-boundary.json). Update each current legacy path/hash only
after verifying equivalent scientific access roots/counts and path-only source
changes; keep all three existing dependency records and 45 legacy entries. The
guard remains strict for every current whole-file hash and rejects new accesses.
No new scientific import or engine operation is authorized by a relocation.

| Existing owner | Placement decision / acceptance |
| --- | --- |
| `src/{inputs,instruments,simulation,workbench}` and `src/help.rs` | SEPARATE under `src/downstream/`; existing scientific callers remain frozen until the subsequent public-API conversion. |
| `qml/inputs`, `qml/workbenches`, `qml/instruments`, selected shell views | SEPARATE using every exact approved inventory destination; retain shared presentation imports from ORG-01. |
| `src/bridge` | REUSE entries in place; adapt private Rust references only. |
| `build.rs`, Main, Qt tests and compile-time fixture references | ADAPT paths and the renamed direction workbench type; unchanged resource/module/native identity. |
| Active downstream requirement routes and skills | ADAPT current locators; retain original IDs, historical states and original evidence. |
| Scientific call adoption | DEFER to the next focused DS-01 units using the already accepted public APIs; the parent remains unchecked until no direct scientific dependencies/exceptions remain and actual compatibility passes. |

Acceptance for placement is Qt formatting/Clippy, the unchanged strict dependency
set and scientific operation count, all existing Qt Rust/QML cases, fresh native
build/startup/project smokes and documentation traceability. No non-Qt source or
public API changes are required. This unit does not accept the final API boundary,
scientific instrument sharing, new session isolation or upstream UI behavior.

The [placement prerequisite is accepted](validation.md#uiu-ds-01-downstream-placement-prerequisite-accepted)
with exact source/case preservation and current Qt/native evidence. One relative
bridge path and one implicit QML shell-directory dependency were corrected without
changing their original targets or assertions. The aggregate QML failure remains
recorded separately from the passing Rust phases and later QML/native commands.
The next scientific adoption units consume the already accepted public APIs;
placement does not remove or accept any frozen scientific migration debt.

## UIU-DS-01 OFDM settings facade unit

Migrate `apps/qt/src/bridge/ofdm_settings.rs` to existing accepted operations:
`merlin::downstream::TransmissionConfiguration::from_json`, `symbol_timing` and
the curated `TransmissionSettings::one_second_timing` metadata query. The public
operation matrix already covers these methods. No new Rust/Python capability or
scientific implementation is needed. Preserve the QObject, bundled registry,
accepted five-field JSON, defaults, validation limits, atomic rejection and exact
visible timing-summary wording. Configuration/time calculations stay in the backend;
Qt scales the reported sample rate only for its existing Msamples/s display.

Add the existing `merlin` facade to Qt with `native-input`, preserving all three
legacy dependency records for not-yet-migrated owners. Update only the local lock
entry and verify no existing dependency version changes. After this file contains
no direct scientific imports, remove its single active legacy exception and retain
that exact prior record as retired history. Other current whole-file guards remain
unchanged. Parent DS-01 remains unchecked.

Reuse the backend's `MAX_TRANSMISSION_DOCUMENT_BYTES` rather than copying 4096.
Introduce the actual UI unit-scaling consumer under
`apps/qt/src/shared/presentation/constants.rs`: `SAMPLES_PER_MEGASAMPLE` is a display
unit conversion, independent of waveform clocks, PSD normalization and time-unit
factors that happen to have the same number. `shared/mod.rs` and
`shared/presentation/mod.rs` wire only this real consumer; no placeholder tools or
scientific interfaces are added.

Acceptance uses the existing settings/registry cases plus an independent literal
summary compatibility case for both FFTs, structured invalid-draft ownership,
unchanged QML editor interactions and the current boundary/Clippy/native build.
Because backend/public API code is unchanged, Rust/Python operation and scientific
acceptance remains its existing evidence rather than a redundant wheel rebuild.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `src/bridge/ofdm_settings.rs`, `parse` / `describe` | Five-field editor, atomic Apply and passive timing summary; direct legacy crate access | ADAPT to the already accepted facade operations; retain QObject and registry contracts. |
| `src/shared/presentation/constants.rs`, `SAMPLES_PER_MEGASAMPLE` | SI display scaling needs an explicit semantic owner | ADD a real presentation conversion; wire `shared/mod.rs` and `shared/presentation/mod.rs` from `src/main.rs`. |
| `apps/qt/Cargo.toml` / `Cargo.lock` | Existing legacy consumers remain | ADD only the facade and its local lock entry; preserve every existing package/version. |
| `legacy-qt-boundary.json` | Whole-file exception for this adapter | RETIRE that exact entry with history; leave the other 44 entries and all three dependency exceptions frozen. |

This unit delivers part of REQ-UIU-02/03/08/09 under UIU-DS-01, with the canonical
[public API boundary](../../../architecture.md#unified-qt-public-api-boundary)
and [task](../../checklist.md#task-uiu-ds-01) retaining ownership. No new public API,
persisted setting, UI control, scientific mode or user-visible wording is introduced.

## UIU-DS-01 receiver decision-region facade unit

The next independent consumer is
`apps/qt/src/downstream/instruments/receiver/soft_display.rs::neighborhood`.
Replace its local nearest-reference sorting and half-plane clipping with accepted
`merlin::downstream::Simulation::decision_region`. Keep the existing normalized
unit-amplitude display plane, `center`, `radius`, `references` and `polygon` JSON
fields and empty unsupported result. Configuration, waveform, receiver decisions
and decoder inputs remain unchanged. The backend supplies all geometry; Qt maps
owned fields and reference coordinates only. No new public API is required.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `receiver/soft_display.rs::neighborhood` | Existing private table access and duplicate clipping | ADAPT to the public decision-region query and retire the file's direct-access guard entry. |
| `Simulation::decision_region` / `constellation_reference` | Accepted Rust/Python geometry and independent scientific tests | REUSE unchanged; use public reference pages in the existing Qt geometric containment test. |
| `receiver/soft_display.rs::confidence` | Existing retained-value histogram with unchanged caller contract | DEFER its result-backed projection to the coupled receiver-result adoption; no new histogram algorithm or scientific acceptance in this unit. |
| `legacy-qt-boundary.json` | One whole-file source exception | RETIRE the exact prior entry; preserve its history and every remaining exception unchanged. |

This is partial UIU-DS-01 delivery for REQ-UIU-02/03/08/09 under the
[public API boundary](../../../architecture.md#unified-qt-public-api-boundary).
Preserve all existing test assertions and square/cross-QAM parameter coverage.
Add rejected-order/label/width handling, keeping valid view geometry compatible.
Run targeted Qt Rust geometry/histogram and QML soft-display cases, Qt Clippy,
the strict boundary and its negative cases, then build and smoke the optimized
native executable. Backend, native Python and saved-project owners are unchanged.
The parent remains unchecked; histogram/result migration and final numeric review
retain their separate acceptance obligations.

## UIU-DS-01 lifecycle and receiver contract facade unit

Adopt existing public configuration/stage contracts in
`apps/qt/src/bridge/{closed_loop,receiver_instruments}.rs` without changing their
QObject identities or the still-legacy directional providers. This is partial
REQ-UIU-02/03/08/09 delivery under UIU-DS-01; worker and instrument implementation
adoption remains subsequent work. Preserve configure's invalidation timing,
atomic Apply and original error messages, selector JSON, tap identities and
existing provider calls. No new scientific operation or Rust/Python API is needed.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `bridge/closed_loop.rs`, `apply` / `channel_preview` | Direct transmission model admission | ADAPT to `TransmissionConfiguration::from_json` and its owned settings; preserve `.message()` wording and existing later request validation. |
| `bridge/closed_loop.rs`, document guards | OCD and receiver-option documents share the number 4096 for different contracts | REUSE backend `MAX_TRANSMISSION_DOCUMENT_BYTES` for OCD; ADD the independent presentation policy `MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES` under `src/downstream/common/constants.rs`. |
| `src/downstream/{mod.rs,common/mod.rs}` | Direction-owned common policies | WIRE only the actual receiver-options constant consumer; no empty placeholder owners. |
| `bridge/receiver_instruments.rs`, `TestPointId` | Existing serialized stage identity, no direct engine operation | REUSE the curated nominal `merlin::downstream::FullLoopStage`; keep all enum values and displayed strings unchanged. |
| `legacy-qt-boundary.json` | Two adapter exceptions | RETIRE only those exact entries with history; keep all other guards and dependencies unchanged. |

Acceptance uses scoped Qt settings and closed-loop compatibility tests, existing
workbench/soft-display QML interactions, formatting/Clippy, boundary regression
cases, optimized build and native project/startup checks. Since no backend or
persisted schema changes, retain current paired API/scientific evidence. This unit
does not accept the legacy worker/result providers or session isolation, and keeps
the parent unchecked. Final numeric review must still inspect all changed owners.

## UIU-DS-01 recovered-data chart contract facade unit

Migrate `apps/qt/src/downstream/instruments/receiver/verification.rs` to the
accepted `merlin::downstream::FullLoopReport` nominal report. Existing complete
and partial job fields remain unchanged. This helper formats supplied comparison
counts into presentation coordinates; it never executes or reconstructs a
receiver. Keep recovery totals, zero bars, per-word order, incomplete comparisons,
omitted counts and the original independent coordinate assertions.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `receiver/verification.rs::charts` | Direct nominal engine report import; existing observed counters | REUSE public `FullLoopReport` without changing fields or callers. |
| `receiver/verification.rs` repeated word limit | Three uses of the same 64-bar presentation cap | SHARE `MAX_VERIFICATION_WORD_BARS` under `src/downstream/common/constants.rs`; distinguish display rows from the independent backend retention budget. |
| Existing `zero_counts_and_mixed_outcomes_keep_honest_coordinates` | Independent 12:3 ratio, exact zero and attention flags | RETAIN assertions and explain fixture meanings; do not derive expected values from production policy. |
| `legacy-qt-boundary.json` | One report-type exception | RETIRE the exact entry with history, preserving all other exceptions. |

This is partial UIU-DS-01 delivery for REQ-UIU-02/03/08/09 through the
[public API boundary](../../../architecture.md#unified-qt-public-api-boundary).
Run the scoped chart test and current soft-display QML interactions, strict
boundary/regression checks, formatting/Clippy, optimized build and native smokes.
No new API, scientific calculation, saved schema, QML control or shared-direction
acceptance is introduced. Backend and Python evidence remains unchanged; the
parent, result-provider migration and final numeric review remain pending.

## UIU-DS-01 source-catalog OCD compatibility API prerequisite

The next profile-admission consumer exposes an actual gap: Qt's existing
`profile_input/document.rs::Capture::compatible(profile_id: u8, settings)` checks
any original source identity, while public `TransmissionConfiguration::check_profile`
takes `Dpd`, whose checked run identity is limited to 0..15. A source catalog can
contain IDs through 255. Constructing a run profile just to validate editor
geometry would reject previously inspectable inputs early. Do not duplicate the
existing grid checks in a new Qt adapter or widen the run-ID domain.

Implement one passive operation before migrating its Qt consumer:

```rust
// Implemented and independently validated; no Qt consumer in this unit.
pub fn check_profile_capture(
    &self,
    capture: &ProfileCapture,
    profile_id: u8,
) -> ApiResult<()>;
```

Python mirrors this as
`TransmissionConfiguration.check_profile_capture(capture: ProfileCapture, profile_id: int) -> None`.
The selected captured profile supplies its existing validated original grid to
the same private `check_profile_grid` owner used by `check_profile`. Unknown IDs
retain `InvalidIdentifier`; spacing/FFT/occupied-band failures retain
`IncompatibleGrid`. Python integer-width errors remain language conversions.
The operation borrows immutable inputs, allocates no scientific state, runs no
source/PHY and never mutates settings, captures or provenance. It does not admit
the selected source ID for simulation. Existing methods/defaults remain unchanged.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `crates/ofdm/src/api/admission.rs::check_profile_grid` | Existing editor geometry rules, currently callable publicly only through Dpd | REUSE without new rules through a typed source-catalog selection. |
| `crates/ofdm/src/api/inputs/profile_capture.rs::profile_ref` | Bounded original byte-sized ID lookup with structured errors | REUSE internally; expose no engine/profile implementation. |
| `bindings/python/src/admission.rs`, native `PyProfileCapture` | Existing native owned immutable source catalog | ADD a thin delegation; update the shipped root stub and inherited downstream namespace. |
| `crates/ofdm/tests/api_profile_capture.rs`, `bindings/python/tests/test_profile_capture.py` | Independent source-ID/geometry fixtures and ownership checks | EXTEND with both numerologies, byte-ID boundaries, absent IDs, incompatible grids and unchanged run-ID rejection. |
| Canonical architecture, REQ-UIU-09, UIU-DS-01 and single operation matrix | Missing consumer-specific compatibility observation | LINK the exact operation, owners, paired tests and evidence; preserve accepted API-01/02 history. |

This is a focused UIU-DS-01 prerequisite under REQ-UIU-02/09 and the
[public API boundary](../../../architecture.md#unified-qt-public-api-boundary),
not a reopened planning/FEC/API milestone. Validate Rust first, then the installed
native wheel with actual callable metadata, existing admission/catalog regressions,
Clippy and exact operation tracing. Commit the API extension independently before
any Qt capture-admission replacement. Original Qt sources/guard debt stay frozen
through this prerequisite; no new operating mode, resource policy or schema.

The [paired API prerequisite is accepted](validation.md#uiu-ds-01-source-profile-compatibility-api-accepted)
with exact callable metadata and preserved prior rows. Eleven Rust and thirteen
installed-native cases pass; the source/test-style assertion corrections remain
recorded separately. The public extension is ready for its focused commit before
any Qt source-catalog consumer changes.

## UIU-DS-01 idle replay-capacity API prerequisite

The live `bridge/measurement_input.rs::selection_status` calls
`measurement_input/model.rs::Options::capacity` once CWB timing is accepted,
including when no profile or measurement sequence can yet be admitted. The
existing public `MeasurementSequence::capacity` requires that admitted sequence.
Do not invent dummy scientific inputs or keep a second temporal-capacity formula
inside Qt. This is the next actual UIU-DS-01 admission-observation gap, under
REQ-UIU-02/07/09; completed planning/FEC/API milestones remain unchanged.

Independently validated Rust operation on the existing immutable options model:

```rust
pub fn capacity(
    &self,
    transmission: &TransmissionConfiguration,
    duration: ObservationDuration,
) -> ApiResult<usize>;
```

Python mirrors this as
`MeasurementSequenceOptions.capacity(transmission: TransmissionConfiguration, duration: ObservationDuration) -> int`.
Move the existing public sequence query's clock/strict-end capacity calculation
to this options operation, then delegate the original method to it. Preserve the
original API's accepted observation-duration domain and Single/Average/temporal
results. Zero temporal intervals receive a structured InvalidInput rather than
division by zero; static modes continue to ignore dormant intervals. The shared
capture-count bound stays authoritative. Association tolerance remains separate
from the synthetic replay clock. No source, capture, profile, RNG or run is needed.
Qt's existing narrower CWB duration admission remains its consumer policy; this
query must not narrow the pre-existing sequence API.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `crates/ofdm/src/api/sequence/queries.rs::MeasurementSequence::capacity` | Existing exact public clock query, needs admitted inputs | SHARE its calculation through `MeasurementSequenceOptions::capacity` in the same owner and delegate the unchanged method. |
| `bindings/python/src/sequence.rs::PySequenceOptions` | Existing native immutable options, deferred interval admission | ADD thin Rust delegation and matching shipped stub; retain constructor/defaults and argument conversions. |
| `crates/ofdm/tests/api_measurement_sequence.rs`, native counterpart | Original admitted-sequence timing/power/ownership cases | RETAIN and extend with literal both-spacing strict boundaries, zero/static/maximum intervals, resource saturation and unchanged timing/options. |
| Current architecture, checklist and operation matrix | Idle capacity currently exists only in Qt | LINK the exact public prerequisite and paired evidence; keep Qt sources and guard debt frozen until separately validated/committed. |

Validate Rust first, then a fresh installed native wheel, existing sequence and
admission cases, Clippy/format/analysis-only checks and exact API metadata. Commit
this API prerequisite independently before the Qt consumer migration. This adds
no resource budget, waveform mode, schema, dependency or scientific algorithm.

The [idle-capacity API prerequisite is accepted](validation.md#uiu-ds-01-idle-replay-capacity-api-accepted)
with six Rust and thirteen installed native cases, exact operation tracing and
preserved prior exports. The calculation now has one backend implementation.
Qt's consumer remains unchanged until the independent API commit succeeds.

## UIU-DS-01 minimum observation timing API prerequisite

`apps/qt/src/downstream/help.rs::playback_help` still derives the minimum whole
milliseconds needed to include the final zero-based source symbol. Its registered
diagnostic and presentation templates belong in Qt, while the quantitative clock
threshold must come from the backend. The accepted public timing queries expose
complete-symbol counts but no inverse minimum-window query. Add that specific
observation before replacing the local calculation; do not reconstruct it in QML.

Proposed operation on the existing immutable transmission configuration:

```rust
pub fn minimum_observation_milliseconds_after(
    &self,
    last_symbol_index: u64,
) -> ApiResult<u64>;
```

Python mirrors the same name and returns an owned integer. The zero-based index
belongs to the source observation, excluding acquisition, interleaver drain and
window tails. Require enough sample ticks for `last_symbol_index + 1` complete
periods, then round up to whole milliseconds using checked integer arithmetic.
The returned threshold may exceed the checked observation-duration domain or
Qt's narrower CWB budget; it reports required source time and admits no run.
No source, waveform, receiver or RNG is created or advanced.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `crates/ofdm/src/transmission/timing.rs` | Existing Apache-2.0 integer-clock geometry/cadence owner | ADD a private checked minimum-window calculation using its existing validated timing. Preserve duration/sample/one-second operations and their outputs. |
| `crates/dsp/src/common/constants.rs`, downstream re-exports | Generic exact integer SI time scales need one owner | SHARE milliseconds/second and nanoseconds/second here; replace the timing owner's identical local nanosecond conversion. Keep unit scales separate from app/runtime budgets with equal numbers. |
| `crates/ofdm/src/api/admission.rs` | Immutable validated timing configuration | ADD the small typed query delegating to the timing owner. |
| `bindings/python/src/admission.rs`, root stub | Existing thin native transmission wrapper | ADD exact delegation and signature; retain constructors/defaults and argument conversion semantics. |
| `api_admission.rs`, native admission cases | Existing independent transmission/source cases | RETAIN and extend with both clocks, alternate CP, strict rounding, zero/upper symbol indices and required thresholds above run-policy limits. |
| Qt help/templates and guard record | Existing GPL presentation and compatibility behavior | RETAIN unchanged in this prerequisite; consume the validated API only after its independent commit. |

The compatibility reference is `apps/qt/src/downstream/help.rs` at MERLIN commit
`61d69090c2effede83943148cc5c2e40477de83d`, copyright 2026 MERLIN contributors,
GPL-3.0-only. Its numerical expectations are behavioral facts, not an imported
implementation. Implement independently from the existing Apache timing primitive
and sample-clock equation; copy no GPL function into Apache owners and change no
license/NOTICE. Existing PHY I20 sample-clock/CP references stay unchanged; this is
a project timing query, not a new specification requirement or interoperability
claim. No third-party material or dependency is introduced.

Trace this focused prerequisite through REQ-UIU-02/07/09 and UIU-DS-01. Validate
Rust first, then a fresh installed native wheel, existing timing/admission cases,
Clippy, formatting, analysis-only compatibility and exact operation tracing.
Commit the API independently before changing the Qt help consumer. Completed
planning/FEC/API milestones and the parent task's pending status remain unchanged.

Pinned clock context was checked in the local text extraction of
CM-SP-PHYv3.1-I20-230419, section 7.3.1/Table 6 (PDF page 47) and
section 7.5.7/Table 38 (PDF page 134). No rendered-PDF inspection is claimed.
The inverse minimum-window result is derived from the existing library clock,
not presented as an additional normative DOCSIS requirement.

Scoped backend prerequisite accepted: [minimum-window validation](validation.md#uiu-ds-01-minimum-observation-timing-api-accepted) and [exact evidence](minimum-observation-timing-evidence.json) record ten Rust/fourteen installed native cases, original timing compatibility and unchanged earlier API exports. The Qt consumer and parent DS-01 remain pending.

## UIU-DS-01 relative channel power API prerequisite

Idle TX preview can reuse `TransmitterPreviewRequest`, its one-shot control,
`TransmitterPreview::calibrated_spectrum`, and public timing/window snapshots.
The old Qt channel formatter still computes `10 log10(normalized_power)` for
its relative Channel Power label. The existing public spectrum carries linear
integrated power and optional electrical dBmV, but lacks that relative dB value.
Add an optional `normalized_power_db` field to the existing non-exhaustive
`ChannelSpectrum`, exposed unchanged by native Python's Rust serialization.
It means dB re one normalized sample unit squared, not PSD per Hz or dBmV.
Zero integrated power yields None/null rather than an invented finite floor.
The value remains independent of viewport, calibration, gain application and run
state. No new method, constructor, type, dependency, operating mode or admission.

| Path and symbol | Existing contract / gap | Decision and owner |
| --- | --- | --- |
| `signal_processing/channel/spectrum.rs::view` | Existing full-band power owner supplies validated nonnegative finite integration | ADD the optional logarithmic observation after integration; retain every existing value and field. |
| DSP/downstream common constants; `waveform/spectrum.rs::density_db` | Total power and power-density ratios use the same decibel factor | SHARE one named power-ratio factor; keep amplitude factor and logarithm base semantically separate. |
| `api/channel/models.rs`, native spectrum bindings | Curated non-exhaustive result and owned serialized dictionaries | REUSE unchanged delegation; the additional field appears in Rust/native results together. |
| Channel-spectrum owner tests, `api_transmitter_preview.rs`, native `test_channel_spectrum.py` | Independent flat PSD, zero, original-owner and separate-process parity cases | EXTEND assertions for literal relative power, zero/null, zoom/calibration independence and immutable original results. |
| Qt preview/spectrum consumers | Existing legacy formatter retains behavior and remains frozen | DEFER edits until this API prerequisite has a focused accepted commit. |

Behavioral reference: GPL-3.0-only MERLIN
`apps/qt/src/downstream/instruments/channel/mod.rs::channel_viewport` at commit
`653bbe05cb4237500ddc34908d29125769b0e179`, copyright 2026 MERLIN contributors.
Implement the standard power-ratio equation independently in the existing Apache
scientific owner; copy no GPL implementation. The existing waveform density
conversion proves the common factor's in-tree meaning. Preserve headers/NOTICE;
no third-party material, new normative claim or external interoperability claim.

Trace REQ-UIU-02/08/09/17/27 and UIU-DS-01. Run scoped owner/API tests and a fresh
installed wheel against separately executed Rust full-loop/TX references. Preserve
every old operation row and native signature; validate metadata, docs and links.
Parent DS-01 and final numeric/integration gates remain unchecked.

Scoped prerequisite accepted: [relative-power validation](validation.md#uiu-ds-01-relative-channel-power-api-accepted) and [exact evidence](relative-channel-power-evidence.json) record ten Rust/twelve installed native cases, independent zero/flat-density anchors and fresh separate-process parity. Qt preview adoption and the parent remain pending.

## UIU-DS-01 unnamed file source compatibility API prerequisite

Compatibility review of the idle TX consumer found an actual input distinction:
the existing Qt release accepts a saved finite file whose original display name
is empty. `NamedCapture::new` intentionally rejects empty capture identities. The
new Qt adapter must not narrow legacy file support or fabricate a replacement
name. Keep the named constructor unchanged and add a specific passive file source
operation for an explicitly absent display label:

```rust
pub fn unnamed_file(bytes: &[u8], fill: Option<FileFillPolicy>) -> ApiResult<Self>;
```

Python exposes `SourceInput.from_unnamed_bytes(data: bytes, fill=None)`. Preserve
`kind == file`, name `Some("")`/empty string, exact bytes/SHA-256, the existing
finite source bounds, independent fill seed/target and no source/RNG advancement.
NamedCapture/new and SourceInput.file/from_bytes retain their original validation.
The saved full-loop adapter must use this operation only for an empty file-source
label, preserving capture-name/provenance checks for profiles, RxMER and H.

The prior optimized Qt executable accepted the private modified single-workbench
fixture with an empty file label (`--check-workbench`, exit 0). Its SHA-256 was
`9eaa52d7eb32ef9630d20cad9b29ae5d649e7932f50e37ee764d6eafa78700ee`;
the exact input digest was
`b51496051c310bb133cec385864e15659894a4047ffd893aa070bc613d4c64d8`.
The same executable rejected an empty profile-capture name, exit 1 with invalid
source provenance. This is a file-label compatibility gap, not permission to
relax scientific capture identities. Private receipts/inputs are under
`docs/development/local/unified-qt-workbench/ds-01-preview-unlabeled-*`.

| Path and symbol | Current contract / gap | Decision / destination |
| --- | --- | --- |
| `crates/ofdm/src/api/admission.rs::SourceInput::file` | Existing validated immutable FilePayload/fill/hash owner, named input only | SHARE its private construction helper; ADD explicit unnamed_file without a duplicate source algorithm or altered named constructor. |
| `bindings/python/src/admission.rs`, root stub | Existing native passive source wrapper | ADD from_unnamed_bytes, identical Rust validation/ownership and GIL release. |
| `crates/pnm-input/src/workbench.rs::Stored`, full-loop source loader | Shared integrity/budget checking currently also forces NamedCapture for file labels | EXTRACT private verified-byte decoding. Existing capture decoding still calls NamedCapture::new; only file-source loading chooses the explicit unnamed operation. |
| `crates/pnm-input/src/common/constants.rs` | Existing name/base64 safety endpoints are inline literals | CENTRALIZE their unchanged inclusive byte bounds by saved-input purpose. Keep aggregate/original/encoded budgets independent. |
| Rust admission/workbench and native equivalents | Existing exact bytes, fill, real FEC, provenance and bounds cases | RETAIN and add empty-label/empty-file, caller mutation, bounds, named rejection, saved original/fill recovery and corrupt-source cases. |
| Qt work in progress and guard record | Consumer not yet accepted/committed | PRESERVE exact WIP in local stash c0224b5c963e175ccd2eb68c055c9404ca4cc7fa while this API commits independently; restore and verify hashes afterward. |

Reuse audit: the Apache SourceInput/FilePayload/saved-workbench owners at
`4c208860fa9baec12779562a77e501182af2bafa` retain algorithms and licenses. The
GPL Qt release is a behavioral compatibility reference only; copy no GPL code
into Apache crates. No new normative mode, scientific algorithm or dependency.
Trace REQ-UIU-02/08/09/24/27 and UIU-DS-01. Validate Rust before a fresh installed
wheel, scoped original-owner cases and fresh saved-workbench process parity.
Record exact additive method signatures/operation rows and unchanged earlier
exports. API/FEC/planning milestones remain completed; DS-01/QA-01 stay open.

Scoped paired prerequisite accepted: [unnamed-source validation](validation.md#uiu-ds-01-unnamed-file-source-api-accepted) and [exact evidence](unnamed-file-source-evidence.json) record20 Rust/16 installed native cases, prior constructor preservation and actual original-file recovery. Restore/verify the preserved Qt WIP after this focused API commit; DS-01 stays open.

## UIU-DS-01 idle transmitter preview facade

Implement the next DS-01 consumer after the accepted public preview/timing,
physical-layout and relative-power prerequisites. Preserve readiness, idle-only
generation, one worker, original input bytes, source/fill separation, stale-result
rejection, cancellation, spectrum navigation/PLC shading and stable timing axes.
Long operations remain on the existing bounded Rust worker. No QML rewrite or
saved schema/default change. REQ-UIU-02/07/08/09/17/27 and UIU-DS-01 own acceptance.

| Path and symbol | Existing contract / gap | Decision / destination |
| --- | --- | --- |
| `apps/qt/src/downstream/inputs/public_inputs.rs` (new), inputs module wiring | Saved source/profile sections already retain originals, integrity and accepted options | ADD a thin adapter translating those sections through NamedCapture, CaptureLoader, SourceInput (including explicit unnamed files), PrbsSeed and FileFillPolicy. No private scientific type, source stream or parameter calculation. Null sections preserve incomplete readiness. File-mode dormant continuous seed and disabled fill remain dormant. |
| `instruments/transmitter/preview.rs::Analyzer` | Legacy direct preview engine, atomics, geometry and PSD access | ADAPT to public TransmissionConfiguration, TransmitterPreviewRequest/Control/result and configured physical-layout query. Preserve generations, busy state, cancellation and result tokens; public progress replaces private engine callbacks. |
| `bridge/transmitter_analyzer.rs`, `workbench/project_load.rs` | GUI-thread and independent Open preparation | EDIT callers to pass their own accepted original sections; no new global scientific store or dependency. |
| `instruments/spectrum/symbol_timing.rs` | Adjacent-window drawing currently generates coefficients from private waveform geometry | ADAPT drawing to owned public SymbolWindow coefficients. Keep only coordinate/axis/label projection in Qt and preserve every existing independent numerical assertion/parameter combination. |
| `shared/presentation/spectrum.rs` (new), module wiring | Idle preview needs the existing SA shape from typed public buckets | ADD shared presentation over public ChannelSpectrum and PhysicalLayoutView. Reuse original graph arithmetic/strings; backend owns PSD, calibration and integrated relative dB. Legacy channel/receiver formatters remain pending, to converge through this owner during DS-01/SHARED-01. |
| Shared presentation and downstream common constants | Existing SI labels and independent chart policies have unexplained literals | SHARE unit-specific display conversions and purpose-named chart/timing policies. Equal-valued headroom/grid steps, prefix/taper allowances and sample-count budgets remain distinct. Stable graph padding is a display policy, not a new PHY admission bound. |
| Boundary record, current docs and user guides | Two fully migrated files can retire exact original exceptions | RETIRE only preview and symbol-timing entries after public adoption; preserve all other entries/initial snapshot verbatim. No guard relaxation or hash refresh for remaining private consumers. |

Reuse provenance: GPL-3.0-only MERLIN files at
`4c208860fa9baec12779562a77e501182af2bafa`, copyright 2026 MERLIN contributors:
`instruments/transmitter/preview.rs`, `instruments/spectrum/{mod,symbol_timing,plc}.rs`
and `instruments/channel/mod.rs`. Adapt only within the GPL Qt adapter, preserving
headers, original expected vectors and license/NOTICE. Apache API owners remain
unchanged in this consumer unit. New presentation modules require no package
resource registration; QObject/QML URIs and current presentation files remain.

Acceptance: scoped Rust preview, independent project-load, original timing-window
matrix, original-byte translation and spectrum formatting cases; relevant QML
navigation/timing cases; 16 boundary regressions and current guard; formatting,
Qt Clippy, optimized executable and native curated-project/startup smokes. Record
changed assertion construction separately from preserved scientific oracles.
Backend/native API tests are reused from the prerequisite commits since this unit
changes only Qt adapters/presentation; final full integration remains QA-01.

Scoped consumer accepted: [preview validation](validation.md#uiu-ds-01-idle-transmitter-preview-facade-accepted)
and [exact evidence](downstream-preview-evidence.json) record11 targeted Rust,
89 QML, 16 boundary and six native checks, final Clippy and optimized build.
The original 44-case timing matrix and every original preview assertion remain.
The unnamed-file compatibility API was committed independently before consumption;
twelve WIP files restored exactly and four canonical docs reconciled both records.
Two original guard entries retire without modifying the other38 entries or initial
snapshot. Backend/binding/lock/QML files stay unchanged. A pinned RCC list parser
emits an empty resource watch, causing redundant Cargo rebuilds; fix that separately
through supported build-time Qt APIs after this focused consumer commit. Keep
DS-01, session/isolation, remaining scientific consumers and final numeric audit open.

## UIU-DS-01 resource build cache follow-up

The preview consumer exposed a reproducible build-only defect in the pinned
qt-build-utils0.7.3/CXX-Qt0.7.3 resource-list path. RCC's trailing newline becomes
an empty PathBuf; CXX-Qt emits `cargo::rerun-if-changed=`. The current Cargo
fingerprint records that empty path, recursively watching the Qt package and its
target artifacts. Consecutive scoped tests therefore rebuild the Qt adapter.
Preserve artifact caches and fix the watch owner rather than deleting outputs.
Trace REQ-UIU-25/27 and UIU-DS-01; runtime/scientific/session/API tasks stay unchanged.

| Path and symbol | Current contract / gap | Decision / destination |
| --- | --- | --- |
| `apps/qt/build.rs`, `branding.qrc` | Builder's explicit branding QRC path emits an empty watch; QML module resource generation is separate | ADAPT branding embedding through a focused build-support owner. Retain exact QRC, aliases, source assets, QML URI and all remaining builder registrations. |
| `apps/qt/build_support/branding.rs` (new) | Build-time resource generation and initializer need finite actual file watches | REUSE pinned QtBuild::qrc_list/qrc and cc::Build::compile_intermediates. Exclude only empty paths; retain all real watches and link the initializer object directly so static registration survives. No SDK/cache patch or vendoring. |
| Qt manifest/lock | cc1.11.1 and qt-build-utils0.7.3 already exist transitively | ADD exact direct build-only pins for their supported APIs. Preserve every other locked package record/version/checksum; no new registry package or scientific dependency. |
| `tools/validation/qt_api_boundary.py` and its fixtures | New resource build tools and a focused build-support module must remain audited | ADMIT only build-kind tool dependencies and build-script-owned includes under `apps/qt/build_support`; reject runtime/dev widening and scientific access/escaping includes inside helpers. Retain exact legacy science records. |
| Current build fingerprints, native smokes and source-resource catalog | Existing observed empty watch and actual branded startup | RETAIN baseline receipt; prove no empty/package/target watches afterward, all five real assets remain watched/unchanged, target-artifact mutation does not rebuild, actual QRC mutation does invalidate, and optimized executable/native checks still pass. |

Reuse/licensing audit: inspected the pinned registry source at MERLIN commit
`bd279075c305facc4dd18a2793c2bc2f5e3ba3ff`: qt-build-utils0.7.3 qrc_list/qrc,
CXX-Qt0.7.3 resource/object setup and cc1.11.1 intermediate compilation. The two
direct build tools are already pinned MIT OR Apache-2.0 dependencies. Use public
interfaces in new GPL Qt build-support code; copy no dependency source, change no
copyright/NOTICE or asset, and add no scientific/native DSP implementation. The
C++17 setting is a named Qt resource-compiler contract, not a PHY parameter.
Direct initializer-object linking preserves the existing non-archive linkage.
Core Qt includes and the existing compiler's supported platform flags remain
build-only. Linux runtime proof does not establish Windows/installed lifecycle
acceptance; retain those existing platform gates.

Acceptance: exact lock/asset preservation, filtered actual RCC list, final
fmt/Clippy and boundary checks; consecutive stable-profile Cargo builds with
unchanged build-script output/object fingerprints after an ignored target probe;
explicit real-QRC invalidation; optimized build and six native checks. Do not
repeat unchanged backend/native simulation suites for this build-only change.
Source package allowlists must retain the new build-support source. Record
actual time/build-output comparisons and keep parent DS-01/QA-01 unchecked.

Traceability reconciliation also adds DS-01 delivery/reverse links for the existing cross-cutting requirements07/17/24/25/27 already used by accepted DS-01 unit evidence. IDs and historical completion states remain unchanged; the planning inventory remains a historical snapshot.

Scoped follow-up accepted: [resource-cache validation](validation.md#uiu-ds-01-resource-build-cache-accepted)
and [exact evidence](resource-build-cache-evidence.json). All 210 locked package
records and five branding assets remain unchanged. The current fingerprint has 114
existing-file watches and no empty/output paths. Two no-op builds, an ignored
target write and a final no-op build take 0.13–0.21 seconds with exact output hashes
and timestamps preserved; touching the real unchanged QRC triggers the expected
98.23 second rebuild. Twenty boundary regressions, current guard, formatting,
Clippy, optimized build and six native checks pass. Resource initializer symbols
remain linked. No SDK, cache, backend, Python, QML or original boundary snapshot
is modified. Parent DS-01, final numeric audit and platform lifecycle gates remain
open. No independent extracted source-package build was executed in this unit;
the new source is explicitly tracked and not ignored.

## UIU-DS-01 public progress phase prerequisite

The existing public `ChannelProgress::phase` field has the engine's ChannelPhase
type, but that enum cannot be named from `merlin::downstream`. The wall-time
adapter must match its actual typed phases without importing an engine module or
parsing serialized/debug text. Re-export the same existing nominal enum through
`api::full_loop` alongside ChannelProgress/ChannelJobStatus. This adds type
reachability only: no method, algorithm, enum variant, serialization, default or
lifecycle change. REQ-UIU-02/07/09/24/27 and UIU-DS-01 own the prerequisite.

| Path and symbol | Decision and acceptance |
| --- | --- |
| `crates/ofdm/src/api/full_loop/control.rs` | EDIT the existing curated re-export to include ChannelPhase. REUSE the scientific owner's exact enum and serde implementation, preserving old imports. |
| `crates/merlin/tests/namespaces.rs` | EXTEND existing nominal-type compatibility assertions and compile a typed public ChannelProgress field projection through the facade. |
| `crates/ofdm/tests/api_full_loop.rs`, native `test_full_loop.py` | REUSE actual completion, acknowledged pause/resume/cancel and retained-comparison regressions. Native `FullLoopControl.snapshot()` already serializes this same phase as a snake-case string; no new Python class or signature is needed. |
| Full-loop user/API docs and canonical traceability | DOCUMENT Rust typed phase access and the existing native snapshot representation; retain all operation matrix rows because no callable operation changes. |
| Qt timing adapter and constants | DEFER consumption to a separate focused unit after this prerequisite commit. Existing frozen source and boundary records remain untouched here. |

Reuse/license: existing MERLIN Apache-2.0 public control and scientific enum,
inspected at `ea6e2c7b66d73e019ad33ebe846a30b7e70eaa5c`; copy no implementation
and change no scientific owner/dependency. Run focused Rust facade/control cases,
native installed control regressions, current metadata/inventory checks and
formatting/Clippy. Preserve source-only versus current installed evidence.

Prerequisite accepted: [progress-phase validation](validation.md#uiu-ds-01-public-progress-phase-accepted)
and [exact evidence](progress-phase-evidence.json) record four Rust and ten
installed native compatibility cases on the prior runtime wheel, 38 metadata cases, Clippy and strict compiler/installed
inventory reconciliation. The exact enum export is additive; every callable row
and native class/signature is unchanged. The historical direct Rust full-loop
reference is explicitly reused with its hash; no new reference run is claimed.
The native runner used the prior unnamed-file archive; its earlier fresh-source
wheel claim is corrected in the evidence. Current worker-prerequisite acceptance
revalidates these cases against its exact new archive/installed-module checksum.
No Qt source, dependency, frozen record or scientific implementation is changed.
Commit this prerequisite before the wall-time consumer. Parent DS-01 stays open.

## UIU-DS-01 wall-time adapter facade

Consume the separately committed public ChannelPhase enum in the existing
RunTiming presentation owner. Preserve monotonic wall-clock semantics, paused
elapsed time, completed-profile timing, progress age, poison recovery and frozen
completion. The worker remains the legacy owner pending its coherent migration;
do not change it or claim full-loop facade acceptance. REQ-UIU-02/07/08/27 and
UIU-DS-01 own this scoped consumer.

| Path and symbol | Reuse and implementation |
| --- | --- |
| `apps/qt/src/downstream/simulation/run_timing.rs` | ADAPT only phase import/matching to `merlin::downstream::ChannelPhase`; retain the same nominal type, strings and callbacks. REUSE standard Duration conversion for milliseconds to seconds. Preserve all original time/profile assertions and add all-six-phase label coverage. |
| `apps/qt/src/shared/presentation/constants.rs` | SHARE exact seconds/minute and minutes/hour units with separate purpose/unit names despite equal values; derive seconds/hour from those definitions. These are display units, separate from simulation budgets and sample clocks. |
| Legacy boundary record | RETIRE only the original run-timing entry after all private imports are removed. Preserve every other frozen entry and the immutable initial record byte for byte. |
| Current docs/evidence | RECORD scoped Rust/QML control checks, boundary regressions, Clippy, optimized executable and six native checks. Parent DS-01, the worker and final numeric audit stay open. |

Reuse/license: existing GPL-3.0-only MERLIN RunTiming and common presentation
constants at `d5638acb19a57c8a71e20934ec0b878ef3bbac2d`. No third-party code,
new dependency, scientific algorithm, saved setting or QML presentation is added.
Retain the named existing profile-timing bound from its present owner until that
frozen profile-result owner migrates; do not duplicate the policy just to relocate
an equal number. The final numeric audit reconciles those remaining common uses.

Scoped consumer accepted: [timing validation](validation.md#uiu-ds-01-wall-time-adapter-facade-accepted)
and [exact evidence](downstream-wall-time-evidence.json) record three Rust cases,
the targeted QML clock/pause/completion case, 20 boundary regressions, Clippy,
optimized executable and six native checks. All original timing assertions stay
exact. Only the original timing boundary entry retires; 37 frozen files and the
three scientific dependencies remain. No worker/API/backend/QML behavior change
or full parent/final numeric acceptance is claimed.

## UIU-DS-01 worker budget and progress-age prerequisite

The legacy serial worker gives each profile the remaining active-runtime budget,
and RunTiming ages the actual last completed-step notification. Public requests
currently freeze their host limit, while public controls expose the last progress
but no notification age. Reconstructing every scientific request or estimating
notification time from UI poll cadence would weaken those contracts. Add two
small operations before migrating the coherent worker. REQ-UIU-02/07/08/09/24/27
and UIU-DS-01 own this prerequisite; completed API/planning tasks stay closed.

| Owner / disposition | Exact operation and contract | Native Python mapping |
| --- | --- | --- |
| EDIT `crates/ofdm/src/api/full_loop/request.rs`, REUSE FullLoopOptions validation | `FullLoopRequest::with_runtime_limit(&self, max_runtime_ms: u64) -> ApiResult<Self>` returns an idle independent clone with a positive admitted limit no larger than its original limit. Preserve settings, source, RNG, observation policy, tolerance and original request. Shared runtime validation rejects zero/above-owner bounds; widening an otherwise valid parent budget rejects ResourceLimit. | `FullLoopRequest.with_runtime_limit(self, max_runtime_ms: int) -> FullLoopRequest`; checked native integer conversion then identical Rust errors. |
| EDIT `crates/ofdm/src/api/full_loop/control.rs` | `FullLoopControl::last_progress_age_ms(&self) -> Option<u64>` reads actual monotonic time since the last backend notification, saturated only at the native u64 representation. None before a notification; age continues through pauses and terminal states. Store the timestamp privately under the same scalar-progress mutex; do not change FullLoopProgress fields/serialization or scientific timing. | `FullLoopControl.last_progress_age_ms(self) -> int | None`; direct Rust query, no Python clock or callback. |
| EDIT `bindings/python/src/full_loop/{request,control}.rs` and existing `.pyi` | Thin native conversion only; retain method signatures/defaults and one distribution. | Fresh installed-wheel operations and original namespaces. |
| EDIT existing Rust/native full-loop tests | Preserve existing control/FEC assertions. Add immutable budget narrowing/boundaries, actual runtime-limited partial outcomes, idle/terminal age, paused age and no implicit source advancement. Keep numerical/time fixtures independent and named or explained. | Exercise actual native workers/GIL/control behavior, not mocked clocks. |
| API row matrix / canonical docs / user full-loop guide | Add two exact operations per language with named cases and executed evidence; preserve every existing row and class export. | Strict fresh compiler/installed inventory and targeted controls/namespace acceptance. |

Reuse/license: the MERLIN Apache-2.0 request/control, existing ChannelJobLimits
validator and native adapter at `85955b0439ca934cbe4d9328a6db9a5fab6f7da8`.
No scientific implementation, domain endpoint, dependency, saved schema, Qt
consumer or frozen boundary entry changes. Internal timestamp ownership is a
bounded lifecycle observation, distinct from active host time and simulation time.
Run focused Rust controls/new boundaries, native installed counterparts, relevant
formatting/Clippy, metadata and strict operation tracing. Retain existing full
historical gates and explicitly scope any reused direct-process reference.

Prerequisite accepted: [runtime/age validation](validation.md#uiu-ds-01-worker-budget-and-progress-age-accepted)
and [exact evidence](worker-budget-progress-evidence.json). Six Rust/twelve
exact-archive installed native cases, root/native Clippy, formatting, 38 metadata
cases and strict 569/542 operation tracing pass. All earlier callable rows and
class exports remain exact. Notification timestamps remain private; default
snapshot shape and scientific settings are unchanged. The native artifact
preflight corrects a stale-wheel local runner and the earlier phase record's
fresh-source claim without discarding either historical receipts or valid cases.
No Qt or scientific algorithm changes; commit this gate before worker adoption.
Parent DS-01 and final numeric/runtime acceptance remain incomplete.

## UIU-DS-01 structured engine diagnostic prerequisite

The legacy help owner chooses its bundled guidance from SimulationError.code.
Public ApiError already retains that original structured cause, but Rust callers
cannot read its code without serializing the error. Add a typed optional getter
before removing the worker/help engine imports. Preserve categories, messages,
source error codes and serialized errors. REQ-UIU-02/07/08/09/24/27 and UIU-DS-01
own this additive observation; no new scientific capability or error category.

| Path / disposition | Exact contract and acceptance |
| --- | --- |
| EDIT `crates/ofdm/src/api/error.rs` | `ApiError::engine_code(&self) -> Option<&str>` borrows the retained original backend code, or None for API-only diagnostics. Never infer it from message text or the generic category. |
| EDIT `bindings/python/src/lib.rs`, `upstream/mod.rs`, root `.pyi` | Add optional `ApiError.engine_code` metadata to the existing shared native exception. Downstream calls the new Rust getter; upstream reuses its existing public ApiError.code() source-code operation. Existing code/message/field/offset/legacy_code stay unchanged. Defaults are None for manually constructed/unclassified exceptions. |
| EDIT existing admission tests and namespace test | Actual zero-runtime backend validation exposes InvalidLabInput without changing its message/category; API-only errors with misleading display text retain None. Verify borrowed/clone/serde ownership, both native directions and existing exception/import behavior. |
| API operation matrix, canonical docs and full-loop user guide | One Rust getter and two native namespace field projections with exact provenance, tests and additive rows. Preserve every previous operation/class/member except the explicitly new field. |
| Qt worker/help | Consume only after a focused prerequisite commit; no private field or serialized/debug-text workaround. |

Reuse/license: MERLIN Apache-2.0 ApiError, its existing retained SimulationError,
the native exception converters and public upstream code() at
`5fd139491f1f5ad9f9f5c5a9cccb10c3845fce5d`. No new dependency/type/algorithm,
domain bound, saved schema, Qt source or frozen entry. Validate focused Rust and
exact-archive installed native error cases, namespace imports, relevant Clippy/
formatting, metadata and complete strict operation tracing. Keep runtime/kernel
and historical full acceptance separate from this passive diagnostic change.

Prerequisite accepted: [diagnostic validation](validation.md#uiu-ds-01-structured-engine-diagnostic-accepted)
and [exact evidence](engine-diagnostic-evidence.json) record ten Rust/thirteen
archive-matched installed native cases, root/native Clippy, formatting,
analysis-only compilation, 38 metadata cases and strict 570/544 operation tracing.
All prior callable rows, types and native members are exact after removing only
the new optional field. Generic category/message and existing metadata remain
unchanged. Qt worker/help adoption and final numeric/integration gates stay open.

## UIU-DS-01 controlled worker facade migration

This scoped consumer implements REQ-UIU-02/07/08/09/17/24/27 under
the existing downstream migration task. Preserve the UI's accepted options,
profile ordering, partial results, stage windows and diagnostic wording.
Architecture ownership remains the public downstream full-loop API and the thin
Qt adapter; this work does not implement the later global session arbiter.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| `apps/qt/src/downstream/simulation/closed_loop.rs` | Serial profile worker calls the private controlled job and probe dispatcher; five existing regression cases cover measurement runs, MCE, MER and sweeps. | Execution must use accepted public requests and controls. | ADAPT admission, dispatch and read-only result ownership into focused `closed_loop/` modules; preserve the existing module entry points. |
| `apps/qt/src/downstream/inputs/public_inputs.rs`, `workbench/project_file.rs` | Existing integrity-checked original source/profile transport and accepted measurement snapshots. | Pair observations with their unchanged originals without Qt recipe calculations. | REUSE public source/profile admission; admit actual accepted RxMER frames with `RxMer::from_frame`, matching the original SHA-256; load H from original bytes. Pass project sections explicitly during background Open. |
| Public `FullLoopRequest`, `FullLoopSweepPlan`, `FullLoopControl` | Validated immutable requests, complete-catalog preflight, one-shot cancellation/pause and actual notification age. | Replace private callbacks while preserving one active serial worker and aggregate active-runtime budget. | REUSE public contracts, one control per profile; keep application cancellation/pause state across profile boundaries and narrow each runtime to the remaining budget. |
| `simulation/run_timing.rs`, `downstream/help.rs` | Monotonic UI wall clock and stable-code contextual help. | Public progress polling must not reset notification age; duration advice must use backend timing. | ADAPT actual public snapshots/age and public minimum-observation operation. Generic diagnostic conversion preserves old editor callers without private imports. |
| `instruments/transmitter/capture.rs`, new `closed_loop/observations.rs` | Finite original TX/RX presentation copies and exact metadata. | Existing instruments still expect their current data containers. | ADAPT bounded public stage pages into those containers, without numerical reconstruction. Keep the public result as the authoritative immutable owner; absent observations remain absent. |
| `downstream/common/constants.rs` | Purpose-specific presentation policies. | Retention and dispatch values must identify meaning and reuse backend bounds. | SHARE genuine display policies; derive stage-array dimensions, reuse backend event/query limits and default observation policy. Keep independent numeric test vectors literal and explained. |

Reuse/licensing: these are MERLIN GPL Qt presentation adapters consuming the
existing Apache public API; no copied third-party material, new dependency,
scientific algorithm, Python change or persisted schema change. Preserve old
source/profile/MCE assertions; replace test mutation of immutable completed
reports with separate actual runs. Test-only changed captures retain their own
correct hashes and do not alter repository originals. Acceptance requires the
five existing worker cases plus focused lifecycle/window/help checks, relevant
QML checks, dependency-boundary tests, Clippy/formatting, latest optimized Qt
build and native startup/Open checks. Retire only fully migrated frozen files.
The parent DS task and final numeric audit remain unchecked until their full
acceptance criteria pass.

The first compile identified two additional callers. Migrate
`workbench/project_file.rs` admission checks and public settings aliases as part
of this consumer, preserving its existing persistence tests and bytes. In
`inputs/measurement_input/designer.rs`, change only the existing
`designed_flat_mer_drives_residual_power_and_real_fec` test to pass its newly
built recipe originals and execute the public request. All Designer production
code and other tests must remain byte-identical. Record this narrowly reviewed
frozen-file update with before/after hashes and exact outside-test preservation;
remove its one old private test import and add no private invocation. The
remaining Designer production debt stays frozen and unaccepted. Its scientific
oracles, one-millisecond source window and 60-second host ceiling remain;
retained word details now use the accepted Qt public default instead of the
old test-only one-word detail budget, and passive stage capture follows the
same accepted workbench policy. Neither affects its residual-power or FEC
assertions; do not describe this as unchanged retention policy.

The worker now passes the separately accepted H/profile tolerance to the public
backend instead of reusing RxMER's tolerance. This corrects the old worker's
inconsistent rejection of a response admitted by its own editor policy. A
focused fixture admits H at its independent 100000-second boundary while
retaining RxMER's 90000-second policy, then rejects H one second beyond its
own tolerance. No waveform combination or scientific algorithm is added.

Scoped consumer accepted: [validation](validation.md#uiu-ds-01-controlled-worker-facade-accepted)
and [exact evidence](downstream-worker-evidence.json) record 28 targeted Rust,
eight QML behavior cases, twenty boundary regressions, Clippy, formatting,
latest optimized Qt build and six native VM checks. The parent remains open
with 33 frozen files/105 reference sites/three scientific dependencies.

## UIU-DS-01 transmitted PLC message viewer facade

REQ-UIU-02/08/09/24/27 and the existing DS task own this consumer. Replace Qt
frame/MAC/descriptor decoding with `FullLoopResult::plc_messages()`. Preserve
actual bytes, wire offsets, source/profile identity, completed-versus-scheduled
status, bounded trees and existing navigation. Retain the public report's first
issue and pending-byte count; no received-message claim is introduced.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| `instruments/transmitter/messages.rs` | Reassembles PLC frames and parses descriptors privately; duplicates the frame-end calculation. | Public result must own all scientific/message interpretation. | ADAPT to the existing public message report and format its actual packet/context/status fields. Keep 32-record presentation bounds and exact empty states. |
| `instruments/transmitter/tlv_fields.rs` | Qt decodes numeric wire codes, CP/RP tables and modulation; three tests cover labels/raw fallback/offsets. | Render backend-provided meanings without a second decoder. | ADAPT to FullLoopDescriptorField/Meaning; preserve known labels, children and byte rows. Raw/unavailable future meanings stay unsupported. Backend zero loading retains the legacy raw-only tree state; QPSK retains its existing label. This does not imply a backend admission limit. |
| `crates/ofdm/src/api/full_loop/messages.rs` existing tests | Already covers the exact former Qt 128-symbol/8704-sample last-sample, zero-stride and overflow anchors, plus both spacings, gaps, truncation and budgets. | Retiring the duplicate Qt calculation must preserve those oracles. | REUSE and execute the existing three backend tests; no production/test rewrite or completed API-task reopening. |
| `crates/ofdm/src/transmission/plc/descriptors/inspection.rs` existing tests | Scientific owner supplies numerology, timing, carrier roles, loading and original offsets. | Qt fixture formatting alone cannot establish scientific decoding. | REUSE the existing two independent owner tests and actual public message integration case. Preserve Qt raw unsupported vectors as formatting coverage without claiming it validates wire semantics. |
| `simulation/closed_loop/tests.rs` actual instrument case | Existing real-run packet/type/profile/byte-offset assertions remain. | Public completion status must preserve the original visible label. | EDIT one additional independent literal status assertion; keep every existing scientific/byte assertion. |
| New `shared/presentation/bytes.rs`, existing presentation registration/constants | Two message/tree consumers duplicate hex formatting. | Shared byte layout and distinct equal-valued policies need one presentation owner. | SHARE byte/row formatting and named display bounds; reuse existing Hz/kHz conversion. Keep TLV byte-layout annotation and NCP/QPSK labels separate from validation/capability policy. |

Reuse/license: existing MERLIN GPL presentation code consumes Apache public
models and operations; no imported source or dependency/API/schema change.
Accept only after focused owner/API and Qt message/tree/scientific regression
cases, affected QML navigation, boundary/fmt/Clippy, optimized build and native
checks. Retire only these two whole legacy files after acceptance. The final
numeric audit and parent downstream migration remain incomplete.

Scoped viewer accepted: [validation](validation.md#uiu-ds-01-transmitted-plc-message-viewer-facade-accepted)
and [evidence](plc-message-viewer-evidence.json) record eleven targeted Rust cases,
one QML behavior case, twenty boundary regressions, Clippy, formatting, canonical
document checks, optimized build and six native checks. All remaining frozen
entries are exact; the parent retains 31 files/103 sites/three scientific dependencies.

## UIU-DS-01 pre-channel physical-bin scatter facade

REQ-UIU-02/08/09/27 and the existing downstream migration task own this focused
consumer. Migrate `instruments/transmitter/channel_input.rs` to the existing
public `FullLoopResult::physical_bins(PhysicalBins, ...)`. Qt must not rebuild
CarrierLayout or derive frequencies/roles. Preserve open range selectors, the
global checked bin domain, excluded/missing-value filtering, original I/Q and
symbol coordinates, unknown transported-role labels and existing plot scaling.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| `instruments/transmitter/channel_input.rs::qam` | Private CarrierLayout rebuild and frequency/role queries; existing real-worker case checks selection, exact I/Q and amplifier behavior. | All scientific geometry must come through the accepted API. | ADAPT to public physical-bin observation/carrier metadata; public SubcarrierIndex validates explicit selector bounds. Qt retains only filtering, finite point selection and drawing coordinates. |
| Existing `FullLoopResult::physical_bins` and `api_full_loop_observations` actual-bin test | Actual I/Q, power, original coordinates/phase, roles, missing history and invalid windows already implemented. | Consumer needs integrated acceptance, not another geometry implementation. | REUSE unchanged API; run existing actual FFT-bin/transmit-cell regression and the real Qt worker/instrument case. |
| `downstream/common/constants.rs`, `shared/presentation/constants.rs` | Directional input policies and generic drawing conventions. | Existing -1 selector, 2048-point cap, unit minimum extent and 10% margin need explicit meanings. | SHARE genuine scatter drawing policies; keep the TX physical minimum extent distinct from receiver scales, and reuse the public bin-domain validator rather than creating a second domain maximum. |
| `simulation/closed_loop/tests.rs`, new local channel-input tests | Existing real capture assertions remain independent. | Preserve availability, invalid-range ordering and exact readouts. | REUSE all old oracles and add focused pre-result selector-domain checks. No scientific expected value derives from a production constant. |

No backend/API/binding/schema/dependency or UI layout change. Existing GPL Qt
code consumes Apache public observations; no new scientific algorithm or copied
material. Physical power graphs and waveform-spectrum views remain separate
frozen migration work, including preservation of their legacy sample-selection
budgets. Accept this consumer only after focused Rust/API/QML, boundary,
formatting/Clippy, optimized build and native checks; retire only its whole file.

Scoped scatter consumer accepted: [validation](validation.md#uiu-ds-01-pre-channel-physical-bin-scatter-facade-accepted)
and [evidence](physical-input-scatter-evidence.json) record three targeted Rust
cases, the QAM/navigation QML case, twenty boundary regressions, all-target
Clippy, formatting, canonical documentation/traceability, optimized build and
six native checks. The parent retains 30 frozen files/102 sites/three direct
scientific dependencies; the final numeric audit remains open.

## UIU-DS-01 spectrum sample-budget prerequisite

The physical/scatter review found a real remaining spectrum compatibility gap.
Legacy Qt retains at most 32768 contiguous time samples. The public spectrum
query instead admits up to 32 Welch averages. At the supported 25 kHz spacing
and 1024-sample CP, four TX-overlap events contain 36864 samples: the default
query analyzes eight windows, while the legacy retained prefix admits seven.
Qt must not reconstruct the Welch hop/average calculation to preserve its budget.

Select one additive passive operation; do not add fields to the existing public
FullLoopSpectrumQuery or change its defaults, serialized shape or other methods.
REQ-UIU-02/09/24/27 and UIU-DS-01 own this prerequisite. Completed API/FEC/planning
tasks remain closed, and the Qt spectrum consumers remain frozen until acceptance.

| Rust / Python operation | Contract and ownership |
| --- | --- |
| `TransmissionConfiguration::spectrum_average_capacity(&self, max_samples: usize) -> ApiResult<usize>` | Accepted FFT geometry determines the largest complete Welch-window count fitting the sample budget, capped by the existing MAX_SPECTRUM_AVERAGES work policy. Fewer than one FFT rejects; oversized positive budgets never widen work. No source, observation, FFT, receiver, RNG or live session advances, and actual retained availability remains a separate query. |
| `TransmissionConfiguration.spectrum_average_capacity(self, max_samples: int) -> int` | Native Rust delegation with matching domain errors. Nonrepresentable native integer conversion remains a Python error. One distribution, both root and downstream class aliases unchanged. |

| Path / disposition | Exact reuse and validation |
| --- | --- |
| EDIT `crates/ofdm/src/signal_processing/waveform/spectrum.rs` | Own complete-window capacity beside the existing hop/work constants; reuse it in estimate after the unchanged length/sample/frequency validation. Preserve numerical processing and old error ordering/messages. |
| EDIT `crates/ofdm/src/api/admission.rs`, `bindings/python/src/admission.rs`, root `.pyi` | Add only the passive method and direct native delegation; no scientific code in Python or facade. |
| EDIT `crates/ofdm/tests/api_admission.rs`, existing observation fixture helper and `api_full_loop_observations.rs` | Independent two-spacing sample boundaries, work saturation, unchanged input snapshots and a real 1024-CP retained-prefix PSD comparison. Preserve existing default fixture behavior and assertions. |
| EDIT `bindings/python/tests/test_admission.py`; ADD `crates/ofdm/examples/api_spectrum_average_capacity.rs` with its required-feature manifest entry | Installed-wheel integer/domain/ownership checks and a separate bounded Rust-process capacity reference; no simulation is needed for the passive geometry query. |
| REUSE existing `waveform_spectrum.rs`, facade namespace cases and executed operation matrix | Independent tone, power integral, bandwidth, silence and nonfinite oracles, legacy imports and exact additive operation tracing. |

Reuse/license: existing MERLIN Apache scientific/API/native owners and their
fixed sample-clock/FFT geometry; no new normative capability, dependency,
cross-direction consolidation, schema, UI source or FEC implementation. Scientific
32-window work policy stays separate from Qt's 32768-sample presentation budget.
Validate Rust first, then a unique source-built wheel installed outside the
checkout with archive-to-module hash verification. Preserve every prior API row,
class/member and executed record; accept the new operation before Qt adoption.

This prerequisite is [accepted independently](validation.md#uiu-ds-01-spectrum-sample-budget-api-accepted):
thirteen Rust and thirteen exact-archive installed native cases pass, alongside
38 metadata cases, Clippy and strict 571/545 operation tracing. Every prior row,
type and native member is preserved. Qt adoption, the parent migration and final
numeric audit remain separate unfinished work.

## UIU-DS-01 retained transmitter views facade unit

After the separately accepted sample-budget API, migrate retained TX stage
presentation, QAM roles and physical graphs together. REQ-UIU-02/03/09/24/27 and
UIU-DS-01 own this unit; the parent stays open. No QML controls, saved settings,
source generation, channel processing or decoder behavior changes.

| Inspected owner / disposition | Contract and selected replacement |
| --- | --- |
| EDIT `apps/qt/src/downstream/instruments/transmitter/views.rs` | Preserve stage registration, pages, cards, labels and absent states. REUSE public stage identifiers and already retained public snapshots for these views. Keep spectrum_state and spectrum byte-for-byte unchanged under a focused reviewed frozen-record update; their remaining scientific imports are not retired. |
| EDIT `apps/qt/src/downstream/instruments/transmitter/qam_points.rs` | REUSE FullLoopResult.transmit_cells for original mapped/interleaved roles. Remove the Qt metadata interleaver; reduce only already supplied finite I/Q and role values. Keep rare-role quotas, missing history and absolute displayed indices. |
| EDIT `apps/qt/src/downstream/instruments/transmitter/graphs/mod.rs`, `graphs/physical.rs` | REUSE FullLoopResult.physical_bins for actual power, frequency, roles and bounded role spans. No Qt I²+Q² or CarrierLayout construction. Min/max display reduction, axis scaling and ordered-cell drawing remain presentation. |
| EDIT directional/common presentation constants | Name inspector row/page widths, raster columns, bit/label drawing positions, cell extent and physical envelope bucket policy separately even where their numeric values match. Reuse shared scatter/headroom/unit definitions with real existing consumers. |
| EDIT `crates/ofdm/src/simulation/payload/role_projection.rs` tests only | Preserve the retired Qt 128-cell/depth-four role fixture, actual interleaver comparison, one NCP, 96 missing-history positions and later-symbol rejection at the existing Rust role owner. No production backend or public contract changes. |
| EDIT physical/QAM presentation tests and existing closed-loop acceptance test | Keep original graph assertions and independent 3+4i→25 backend anchor. Physical formatting takes supplied power and public layout; it does not fabricate a public result. Check actual backend-to-UI values, all four unchanged spectrum taps and bounded windows. |

Acceptance requires targeted Qt/core cases, QML navigation, boundary regressions,
all-target Qt Clippy/formatting, optimized build and native startup/Open checks.
Retire only the three graph/QAM whole-file exceptions after compatibility passes; preserve
all other frozen hashes and historical records. No fresh wheel is required for
unchanged backend production/bindings; preceding paired API acceptance remains
separate. The final line-by-line numeric audit is still UIU-QA-01.

The source review found a separate spectrum prerequisite: legacy IFFT/prefix/
window plots concatenate up to 16384 retained frame-stage samples. Those frames
are not necessarily physically continuous. Existing spectrum_sequence exposes
the normalized ordered-frame diagnostic, whereas calibrated_spectrum deliberately
uses one frame at these boundaries. The accepted sample-capacity operation alone
cannot bridge electrical calibration while preserving this sampling basis.
Retain the original spectrum functions until a focused typed API extension is
independently accepted. Do not silently replace three-window legacy results with
one physical-frame window, or label an ordered-frame diagnostic as a continuous
waveform. TX overlap remains a separate 32768-sample continuous-output budget.

## UIU-QA-01 validation scheduling refinement

The user authorizes a [three-level testing strategy](testing-strategy.md) to
reduce repeated unrelated execution while preserving every existing test/vector,
downstream contract and official acceptance criterion. REQ-UIU-24/26 and the
existing UIU-QA-01 acceptance owner trace this workflow refinement; no task is
added, reopened or marked complete, and the implementation DAG is unchanged.

| Inspected owner | Decision / file disposition |
| --- | --- |
| `scripts/test-local.sh`, bounded nextest configuration, quick-test policy and Qt build/check helpers | REUSE unchanged. Existing focused filters, complete phases and stable caches already support the three levels. Do not introduce a second runner, skip-list change or test-result cache. |
| Existing Python wheel/import/parity workflow and API matrix | REUSE unchanged when code/contracts require native coverage. Avoid new wheels for unrelated Qt/docs commits; a changed native owner still requires the matching tested artifact. |
| Existing feature validation/task acceptance records | ADD testing-strategy.md as the scheduling reference; preserve historical evidence and all task criteria. Record executed/not-required/deferred scope for new units. |
| `AGENTS.md`, canonical development-workflow/scientific-testing/git-workflow skills | EDIT only validation routing/scope; preserve skill discovery, ownership, scientific rules and authorized local save/push boundaries. |
| Canonical architecture/requirements/checklist and development/feature indexes | EDIT focused references to the strategy with stable IDs/statuses and no restructuring. |

Validate this documentation-only unit with link/traceability checks, modified
skill validation, diff formatting and an exact task-state/source/gate-preservation
audit. Scientific/runtime/Clippy/package suites are not applicable because no
production, test, dependency, runner or build configuration is changed. Preserve
the isolated unfinished calibrated-spectrum implementation and resume it after
the separate focused workflow commit.

## UIU-DS-01 calibrated ordered-frame API prerequisite

Select one additive operation to close the retained-frame calibration gap before
Qt adoption. REQ-UIU-02/09/24/27 and UIU-DS-01 own this prerequisite. Completed
planning/FEC/API milestones stay closed. Existing normalized sequence and
physically continuous spectrum operations, queries and defaults stay unchanged.

| Public contract | Ownership and behavior |
| --- | --- |
| Rust `FullLoopResult::calibrated_spectrum_sequence(&self, query: FullLoopSpectrumQuery) -> ApiResult<FullLoopCalibratedSequenceSpectrum>` | New typed result contains `sequence: FullLoopSequenceSpectrum` and `power: ChannelSpectrum`: normalized and calibrated projections of the same single estimate. All original segment/context/gap/stop/contiguity metadata remains explicit in sequence. Calibration does not make the diagnostic physically continuous. |
| Python `FullLoopResult.calibrated_spectrum_sequence(self, stage='tx.ifft.samples', snapshot=0, max_averages=32, start=0, count=0) -> dict` | One thin native call, releasing the interpreter lock for Rust work and returning owned data. Same stage/work/viewport errors and electrical/relative semantics. Existing class/root aliases remain. |

| Owner / disposition | Exact reuse and acceptance |
| --- | --- |
| EDIT `crates/ofdm/src/api/full_loop/instruments/sequence.rs` | Extract the existing sequence preparation into a private helper returning its existing normalized view plus full-resolution estimate. Old spectrum_sequence returns the same view; the new operation adds calibration without repeating sequence stitching or FFTs. |
| EDIT `crates/ofdm/src/api/full_loop/instruments/channel.rs` | Share its existing accepted-layout/grid/electrical projection as a private composition helper. Existing continuous/channel/amplifier scientific behavior and error order remain. No new gain application to TX observations. |
| EDIT existing native sequence module/root `.pyi` | Direct public delegation only. New Rust report is represented as an owned dictionary, not an artificial Python simulation class. |
| EDIT existing observation and native sequence tests; ADD bounded `api_calibrated_sequence` reference example with feature registration | Compare normalized legacy output exactly, calibrated output against the existing full-resolution electrical owner, literal sample/average anchors, zoom-independent channel power, incomplete-frame/missing/invalid-work/stage rejection, immutable results and exact installed native/direct-Rust-process parity. Preserve all earlier tests. |
| REUSE owner/namespace tests and operation matrix | Original sequence context/gap/partial-frame oracles and tone/power/ENBW numerical vectors remain independent. Add only the new operation and Rust typed report export, preserving every previous operation/member. |

This is scientific composition in the existing Apache Rust owner, with no new
algorithm, dependency, Qt source, saved schema, protocol capability or work limit.
The nested normalized view intentionally preserves its original metadata and
units; ChannelSpectrum supplies the alternate calibrated rendering and fixed
occupied-band power. Both remain bounded by the unchanged 32-average/2048-bucket
policies. The supported UI frame prefix remains a separate caller policy.
Accept Rust first, then an exclusively built wheel installed outside the checkout
with archive/module hash verification, direct-process parity and strict complete
inventory tracing. Only a subsequent focused Qt commit may consume the operation.

This [Level 1 prerequisite is accepted](validation.md#uiu-ds-01-calibrated-ordered-frame-api-accepted):
ten Rust/eleven exact-wheel native cases, scoped Clippy, analysis-only build and
strict 572/546 operation tracing pass. Existing normalized/channel consumers and
all previous operation rows/types/native members are preserved. The evidence
records the stale-reference failure, current Rust reference refresh, unchanged
wheel reuse and explicit executed/not-required/deferred scopes. No Qt entry is
retired; parent Level 2 and milestone/final Level 3 remain pending.

## UIU-DS-01 TX spectrum facade adoption

After API prerequisite 8cf1f7e, migrate the remaining TX spectrum functions in
views.rs without changing QML controls, axis conventions or accepted samples.
REQ-UIU-02/08/09/24/27 and UIU-DS-01 own this Level 1 unit; the parent stays open.

| Owner / disposition | Reuse and compatibility |
| --- | --- |
| EDIT `apps/qt/src/downstream/instruments/transmitter/views.rs` | REUSE public stage inventory for readiness, calibrated_spectrum_sequence for IFFT/prefix/window and calibrated_spectrum for continuous TX overlap. Existing capacity helper preserves separate 16384/32768 sample prefixes. Remove private estimator and CarrierLayout calls. Reuse the accepted shared presentation viewport, public configured PLC interval and unchanged labels/full-span/zoom/power behavior. |
| EDIT `capture.rs` and directional common constants | SHARE the actual four-stage selector order between capture and viewer. Name the legacy frame-sequence sample policy separately from the backend event budget and continuous-output policy; retain equal-valued policies independently. |
| EDIT existing Qt closed-loop tests | Preserve every existing stage/SA assertion. Add a three-frame, supported 1024-CP actual run to verify original frame-prefix averages, seven-window bounded continuous output, calibrated power/zoom, invalid selections and unchanged report/retention. |
| EDIT spectrum/transmitter user guides | Explain existing ordered-frame versus continuous-output sampling, public ownership and retained sample budgets. No visual layout/control or rendered-value change requires replacement screenshots. |

Level 1 runs affected Qt Rust and QML navigation/SA cases, Qt formatting/Clippy,
actual dependency-boundary audit, documentation/traceability and the latest
optimized Qt build. Backend/binding code and accepted APIs are unchanged; another
wheel, full scientific/native suite and unrelated native startup/project probes
are not required. Level 2 task and Level 3 milestone/final gates remain pending.
Retire only the entire views.rs exception after compatibility passes; preserve
every other frozen hash and the original history.

This [Level 1 consumer is accepted](validation.md#uiu-ds-01-tx-spectrum-facade-accepted):
two actual Qt Rust cases, two QML behavior cases, actual boundary audit, scoped
Clippy/formatting, optimized build and documentation/traceability checks pass.
The explicit three-frame/long-CP fixture preserves independent three-/seven-average
prefix anchors and public power/zoom/error behavior. One full exception retires;
26 files/94 sites/three dependencies remain. Backend/native code is unchanged,
and the evidence identifies not-required suites and pending Level 2/3 gates.

## UIU-DS-01 RxMER animation preview facade

Migrate the complete measurement_input/preview.rs exception through the already
accepted RxMer.preview operation. REQ-UIU-02/03/08/24/27 and UIU-DS-01 own this
Level 1 unit; other measurement admission/editor files remain frozen.

| Inspected owner / disposition | Selected reuse and acceptance |
| --- | --- |
| EDIT `apps/qt/src/downstream/inputs/measurement_input/preview.rs` | ADAPT the selected original bytes/name and its already decoded immutable frame to public NamedCapture and RxMer.from_frame, then REUSE RxMer.preview(MeasurementViewport). Preserve sparse selection/order, original index/time/hash, synthetic marker and the exact bounded MeasurementView shape. No replay plan, averaging, RNG or simulation is introduced. |
| REUSE document.rs Capture and model.rs selection | Existing original bytes and decoded frame are read only. Their decoder/Designer internals remain separate frozen work; do not widen this unit into an admission rewrite. Only the selected capture is copied into the existing public immutable owner, not the capture collection. |
| EDIT the existing preview regression | Keep all sparse-animation original-order, 132-input, viewport and failure assertions. Add original-byte/frame/hash preservation across calls without deriving expected original indices from implementation constants. |
| EDIT measurement user guide and canonical trace links | Describe unchanged offline preview behavior with authoritative public ownership; no visible layout, control or saved format changes. |

Validate targeted Qt Rust and affected QML animation/viewer cases, Qt format/
Clippy, actual boundary, documentation/traceability and current optimized build.
No backend/binding/scientific kernel, CXX registration, project or lifecycle
change requires another wheel or unrelated complete suites/native Open probes.
Level 2/3 gates stay pending; retire only this whole file after acceptance.


[Accepted focused evidence](validation.md#uiu-ds-01-rxmer-animation-preview-facade-accepted)
preserves every existing animation assertion and adds immutable originals/frames/
hashes. The public preview replaces the sole private call; 25 files/93 sites/
three dependencies remain. Level 2/3 gates and all other measurement files remain
separate work.

## UIU-DS-01 source-verified FEC recovery prerequisite

REQ-UIU-02/03/08/09/19/20/24/27 and UIU-DS-01 own this additive prerequisite.
The existing Qt recovery chart subtracts rejected and independently mismatched
words from completed decoder attempts. Move that exact finite accounting to the
downstream simulation result owner before migrating its Qt consumer. Decoder
success, source agreement and unfinished words remain distinct; this changes no
decoder, receiver input, observation window or historical FEC acceptance.

| Path / disposition | Reuse and exact contract |
| --- | --- |
| EDIT `crates/ofdm/src/simulation/channel/results.rs` | ADAPT the existing Qt recovery subtraction into a constant-size PayloadRecovery result constructed from existing FecCounters, generated and unrecovered totals. Preserve saturating subtraction and the independent five-word oracle: one recovered, one rejected, one mismatch, two unrecovered. |
| EDIT `crates/ofdm/src/api/full_loop/result.rs` | REUSE that owner through `FullLoopResult::fec_recovery(&self) -> FullLoopFecRecovery`, re-exporting the result type. Fields are generated, recovered, rejected, mismatched and unrecovered u64 counts for the whole actual complete/partial run, independent of retained detail limits. No mutable state, allocation or scientific execution. |
| EDIT `bindings/python/src/full_loop/result.rs` and root typing declaration | Matching `FullLoopResult.fec_recovery() -> dict` delegates only to Rust and returns an owned five-field dictionary; root/downstream/legacy identities remain exact. |
| EDIT existing full-loop Rust/native tests, facade compile contract and Rust reference example | Keep every original assertion; exercise complete, cancelled and pre-cancelled results, empty outcomes, independent decoder/source classification, source/result immutability, owned native dictionaries and separate Rust-process parity. |
| DEFER `apps/qt/src/downstream/instruments/receiver/fec.rs` | Remains frozen until this paired prerequisite passes and is committed. Qt adoption is a separate focused unit. |

Level 1 expands to affected downstream Rust, installed native Python and facade
contracts, scoped Clippy/format, analysis-only compilation and strict additive
operation inventories. No upstream owner, CLI handler, Qt source/registration,
project schema, cache/profile or packaging contract changes. Existing non-Qt
full suites and Level 2/3 parent/final gates remain pending. No new specification
claim, numerical vector replacement or completed-task reopening is introduced.


[Paired Level 1 acceptance](validation.md#uiu-ds-01-source-verified-fec-recovery-api-accepted)
passes six Rust/eight exact-archive native cases, scoped contract checks and
strict additive inventories. Qt adoption remains separate. Origin review uses
existing Apache-2.0 FecCounters and their public getters; the five-word expected
classification is factual validation data retained from the project-owned
GPL-3.0-only Qt recovery test at checkpoint 7428b33. The new typed projection is
backend code; no Qt chart implementation, geometry, dependency or notice is
imported into the Apache backend. Existing license files/headers remain intact.

## UIU-DS-01 FEC monitor recovery facade

Consume the paired recovery prerequisite from commit 7752a02a for
REQ-UIU-02/03/08/19/20/24/27 and UIU-DS-01. Edit the complete receiver/fec.rs
exception: replace the private counter-based subtraction with a drawing-only
formatter over FullLoopFecRecovery from ResultData.backend().fec_recovery().
Preserve every JSON key, label, segment order/fraction, histogram gap, observed
decoder category, retained-word/partial state and verification chart. No Qt
scientific recovery calculation remains.

The independent five-word classifier regression is already preserved and passed
in the backend prerequisite; retire its Qt duplicate when retiring the private
import. Keep the existing chart allocation and histogram assertions and extend
the actual worker regression to compare every emitted recovery count with the
authoritative public result, retaining the separate four-profile archival case. REUSE existing FEC QML overview,
partial/empty and receiver navigation cases without changing presentation.

Clarify the backend field/API guide's wording: no detected source mismatch is
different from positive verification when no source bits were compared. This
is a documentation correction to the existing subtraction, not a decoder/count
behavior change. The monitor keeps compared bits and BER visible separately.

Level 1 uses targeted Qt Rust/QML, format/Clippy, actual boundary, traceability
and an optimized build. The preceding exact-wheel API evidence is historical
prerequisite evidence; no unchanged wheel/scientific/full suite is rerun or
claimed as a new Qt pass. Parent Level 2 and milestone/final Level 3 stay open.

## UIU-DS-01 profile plot inspection facade

REQ-UIU-02/03/08/24/27 and UIU-DS-01 own this existing-API consumer.
EDIT the complete inputs/profile_input/plot.rs exception to REUSE public
CaptureLoader.profile_capture on the exact original bytes and ProfileCapture.inspect
for all role/loading/coordinate/extrema, ideal Shannon and useful-rate data.
Qt retains only selection, labels, ticks, normalized drawing coordinates and
existing output keys. Separate single-profile 512-column and multi-profile public
bucket policies preserve sampling; 14-bit single-view, 15-bit overlay and 45-dB
overlay axes remain independent named presentation choices in the existing
direction constants owner. Preserve full-span reset, original byte IDs, all/empty
selections, missing/zero role markers and original source provenance.

REUSE profile_input/document.rs and admission/state owners unchanged. The old
marker regression mutated an internal profile independently of its original
bytes; adapt only that fixture preparation in profile_input/mod.rs to an admitted
decoded capture with the same first-carrier exclusion. Keep every assertion and
all other production/test code exact; record a focused whole-file review of that
still-frozen owner, preserving its remaining private sites. This does not change
saved workbench/project contracts or add a public operation for a test shortcut.

Run the existing profile module tests and affected DPD QML cases, Qt format/Clippy,
actual boundary, traceability and optimized build. Backend/bindings, CXX/resource
registration, lifecycle and serialization are unchanged; no fresh wheel or
unrelated full suite is required. Parent Level 2/final Level 3 remain pending.


[Focused acceptance](validation.md#uiu-ds-01-profile-plot-inspection-facade-accepted)
passes all four original profile tests and one new raw-ID/boundary/immutability
case, three QML behaviors, scoped Qt checks and a current optimized build. The
first-carrier exclusion is now represented by original decoded scheme data;
all original assertions and every other model production/test byte remain exact.
The whole plot exception retires, leaving 23 files/88 sites/three dependencies.
One bounded public original-capture admission occurs per view; retaining its
immutable public catalog belongs to the remaining input-owner migration. No
new caching/session or scientific API contract is introduced in this unit.

## UIU-DS-01 source-file admission metadata prerequisite

REQ-UIU-02/03/08/09/20/24/27 and UIU-DS-01 own this paired additive prerequisite.
The source editor still uses private file/generator types for finite counts and
the native chooser's limit; the worker compares accepted source metadata with
the prepared request. Expose the related passive metadata and immutable fill
operations before changing these Qt consumers. Preserve exact bytes, unnamed/empty files, dormant continuous
seeds, independent fill seeds/partial-byte targets and current source limits.

| Owner / disposition | Exact contract and validation |
| --- | --- |
| ADD `crates/ofdm/src/api/admission/source_info.rs`; EDIT admission/root re-exports | `SourceInput::file_limits() -> SourceFileLimits` returns existing authoritative max_original_bytes/max_total_bits. `file_info(&self) -> Option<SourceFileInfo>` returns original_bytes, original_bits, target_bits, fill_bits and optional typed FileFillPolicy. None means continuous PRBS, never empty-file EOF. Values derive solely from the existing validated FilePayload. `SourceInput::with_fill(&self, Option<FileFillPolicy>) -> ApiResult<SourceInput>` reuses FilePayload.with_fill and shares its original byte buffer; it rejects continuous inputs. No generator/bit stream or source mutation. |
| EDIT native source adapter/root typing | Matching static file_limits() -> dict, file_info() -> dict or None and with_fill(fill=None) -> SourceInput. Only Rust calculates source counts/validates replacement; Python returns owned snapshots/handles. Existing constructors/properties/defaults/aliases stay exact. |
| EDIT existing Rust/native source tests, facade compile contract and bounded process reference | Independent 16-MiB/134217728-bit limits, empty/one-byte/11-bit fill/equal-target/dormant state, provenance, pointer-proven original-buffer sharing and source immutability, invalid targets/seeds and separate-process Rust/native parity. Preserve all existing source assertions. |
| DEFER Qt source model/bridge and session/worker source metadata | No Qt change until paired acceptance is committed. Later adoption keeps editor persistence and lifecycle contracts, with separate current consumer tests. |

Level 1 expands to affected Rust/native/facade/source owners, scoped Clippy,
analysis-only build and exact additive inventories. No scientific algorithm,
upstream/shared codec, CLI, project schema, registration or packaging changes.
Level 2/3 official task/final gates remain pending; completed tasks stay closed.


[Paired Level 1 acceptance](validation.md#uiu-ds-01-source-file-admission-api-accepted)
passes six Rust/ten exact-archive native cases, scoped contracts and complete
additive inventories. Source review added with_fill before Qt adoption because
reconstructing the original file on each edit would lose existing shared-buffer
behavior. Pointer/ownership tests prove the existing FilePayload owner is reused.
The initial two-query build remains separate from final three-operation native
acceptance. Qt source/session/worker consumption is the next dependent unit.

## UIU-DS-01 source editor admission facade

Consume prerequisite 80c9fc47 for REQ-UIU-02/03/08/20/22/24/27 and UIU-DS-01.
EDIT source_input/model.rs and bridge/source_input.rs completely: SourceInput,
PrbsSeed and FileFillPolicy own admission, file counts and immutable fill edits;
the picker reads SourceInput.file_limits. Preserve draft/accepted state, unchanged
Options JSON/defaults, exact bytes/hash/labels, empty/unnamed files, dormant seeds
and all existing source assertions. Name independent initial continuous/fill seeds
and the source-options transport cap in the existing direction constants module.

EDIT only the source field/type import in the still-frozen session.rs and the
already-public worker request's metadata comparison. Public source snapshots and
file_info replace private PayloadSourceConfig serialization; seed identity remains
separate. Keep every other session byte/owner untouched and freeze its remaining
private sites under a focused review. This is input typing, not UIU-SESSION-01's
future direction lifecycle implementation. No CXX/QML declaration changes.

The source export implementation uses the same exact original bytes and existing
project encoding. Treat that persistence boundary as affected: run source and
Qt Save/Open/recovery regressions plus targeted Rust/native saved-file/fill
compatibility, reusing the exact accepted installed wheel. Validate actual source
selection, original-byte identity, zero/equal/partial fill, rejection atomicity
and default/dormant choices. Qt format/Clippy, dependency boundary, traceability
and current optimized build apply. No new schema/API/backend, registration or
cross-direction lifecycle change; full task/milestone/final gates stay pending.


[Focused consumer acceptance](validation.md#uiu-ds-01-source-editor-admission-facade-accepted)
passes six Qt Rust/one QML behavior plus two Rust/two exact-reused-native saved
project cases, scoped checks and a current optimized build. A new source test
independently pins the unchanged saved section and proves stage/accepted buffer
sharing. All original source assertions remain through public accessor/limit
adaptation. Two whole entries retire; the source-type-only session review keeps
all other bytes exact. The boundary retains 21 files/85 sites/three dependencies.
No CXX declaration, API/schema/backend or direction lifecycle changes occur.

## UIU-DS-01 raw source-catalog admission prerequisite

REQ-UIU-02/03/08/09/20/24/27 and UIU-DS-01 own this additive API prerequisite.
REUSE the existing complete ProfileCapture, MeasurementPlayback, response admission
and Designer recipe evaluator. Raw source IDs remain bytes; ProfileId remains
0..15 and successful passive admission does not establish run readiness.

| Owner / disposition | Rust contract and native Python mapping |
| --- | --- |
| EDIT ofdm/api/inputs/profile_capture.rs | `frame(id: u8) -> ApiResult<&SourceProfileFrame>` borrows the existing source DTO without copying or mutation; the public alias preserves its nominal type. Python `.frame(id) -> dict` owns a bounded copy of the same original key/epoch/source/roles/loadings. Original capture provenance remains separate from the DTO's decoded-source provenance. |
| EDIT ofdm/api/sequence.rs | `MeasurementSequence::from_capture(&ProfileCapture, raw_id, &TransmissionConfiguration, Vec<RxMer>, options)` reuses the exact existing measurement planner and resource/duplicate/grid/epoch/coverage checks. Native static `from_capture(capture, profile_id, transmission, captures, options)` matches. |
| EDIT ofdm/api/sequence.rs | `TransmissionConfiguration::check_response_capture(&ProfileCapture, raw_id, Option<&ChannelEstimate>, tolerance)` reuses existing physical response checks; native method uses the same name and optional response/tolerance conventions. |
| EDIT pnm-input/workbench/designer/admission.rs | `RxMerDesign::{flat_capture, imported_capture, check_capture}` use a catalog plus raw ID in place of Dpd; all other arguments and result/error semantics match existing methods. Native methods mirror these operations; recipes and every calculation remain Rust. |
| EDIT native adapters/root typing, existing source/sequence/Designer tests and facade compile contracts | Preserve every existing signature/assertion. Test boundary ID 255, absent IDs, original order/bytes/hash/epoch/roles/nulls, borrowed-buffer identity/owned native copies, existing-ID equivalence and all affected rejection/ownership contracts. |
| DEFER Qt input/session owners | No Qt consumer changes before paired API acceptance and its focused local commit. |

Level 1 expands through affected Rust/native APIs, existing measured/Designer
science tests, facade compatibility, analysis-only build, scoped Clippy and exact
additive operation inventories. No scientific algorithm, project format, lifecycle,
CXX/QML registration, upstream or packaging changes. Parent Level 2 and final
Level 3 acceptance remain pending. Completed planning/FEC tasks are not reopened.

## UIU-DS-01 profile input owner facade

Consume accepted prerequisite 05d30422 for REQ-UIU-02/03/08/09/20/22/24/27.
EDIT complete profile_input/document.rs and mod.rs to REUSE public CaptureLoader,
ProfileCapture.frame/schemes/metadata and TransmissionConfiguration admission.
Keep one immutable catalog and NamedCapture original, sharing their original
byte storage; snapshots share catalog frames. Preserve sorted UI IDs versus
original source order, compact tree labels, exact binary/JSON originals and all
valid saved profile sections. Public scheme records supply assignment labels;
Qt only formats rows, units and bounded display depth. A read-only serde view of
the already validated decoder JSON preserves original display text (including
MAC letter case); a named UTF-8 BOM transport marker is skipped only for that
view, never removed from original/intermediate provenance. Remove decoder dispatch,
assignment interpretation, geometry arithmetic and Qt original hashing.

EDIT already-public plot.rs to borrow the retained catalog rather than admitting
another original on each request. EDIT only profile collection/original-byte
access in bridge/profile_input.rs and workbench/project_file.rs; CXX declarations,
project schemas, registration and all other owners remain unchanged. Frame copies
are made only when publishing the existing independently owned session input.
Existing assertion intent is preserved with public accessors. Name shared profile
tree depths; reuse authoritative input/OCD/scheme bounds and existing SI display
units. No new scientific implementation or API operation is introduced.

The accepted public capture/workbench contract requires nonempty capture names,
while payload-file labels may be empty. Retain that distinction and existing
Rust/native name-rejection tests; do not invent a provenance label. The old Qt
passive decoder could stage an empty capture label that the public run/project
contract rejects. Record enforcement of this accepted contract at input admission;
no new unnamed capture capability or change to existing public behavior.

Level 1 runs complete small profile/plot tests, affected publication/project and
Qt/QML cases, format/Clippy/boundary, current optimized build and traceability.
Persistence risk expands to relevant Rust/native saved-project checks using the
exact accepted wheel without rebuilding unchanged backend/binding code. Parent
Level 2 and final Level 3 remain pending; no historical checkbox is changed.

## UIU-DS-01 original measurement-frame prerequisite

REQ-UIU-02/03/08/09/20/24/27 and UIU-DS-01 own this additive observation.
REUSE existing immutable ChannelEstimate, RxMer and source DTO validation.
EDIT api/inputs.rs to expose `ChannelEstimate::frame() -> &ChannelEstimateFrame`
and nominal data-only aliases ChannelEstimateFrame/RxMerFrame. The existing
RxMer.frame/from_frame signatures and every original observation remain exact.
The existing RxMerFrame validation method retains its scientific owner; channel
admission remains with the existing loader/importer. Adapters admit captures
through high-level operations. No engine/kernel module is exported.

EDIT native ChannelEstimate conversion and root typing with `.frame() -> dict`,
an owned bounded copy of the same key, epoch, source and I/Q values. The existing
RxMer native values/grid/provenance operations retain their accepted language
mapping; no extra native class/borrowed lifetime is introduced for the DTO aliases.
EDIT existing measurement preview tests and facade nominal contracts. Cover exact
raw I/Q/nulls, borrowed storage, owned copies, preserved key/epochs, original versus
decoded provenance, invalid DTO validation and original constructor signatures.
Rust acceptance precedes the matching binding. Existing decoder and observation
algorithms are unchanged. Defer Qt measured-input adoption to the next focused
consumer after paired acceptance is committed.

Level 1 expands through measurement/sequence/facade compatibility, scoped Clippy,
analysis-only build, fresh exact installed native wheel and additive inventories.
No backend behavior, project/CXX/QML registration, lifecycle, upstream or package
change; parent Level 2 and final Level 3 gates remain pending.

## UIU-DS-01 ordinary measurement admission facade

Consume prerequisite 1820cdff for REQ-UIU-02/03/08/09/20/22/24/27.
EDIT measurement_input/document.rs to REUSE public CaptureLoader.rxmer/channel_estimate,
ChannelEstimate.frame, RxMer.frame and CaptureProvenance. Remove direct PNM decoder
and scientific importer/type dependencies for ordinary inputs. Keep existing editor
Capture/Measurement shapes with nominal public frame DTO aliases, preserving all
ordinary original bytes, names, formats, source-kind strings, original and decoded
hashes, epochs/nulls/grids and persistence. Only independently owned legacy DTO
copies receive the existing original-file metadata projection; public source
frames/provenance are never modified. One temporary bounded original-byte copy
bridges the unchanged legacy editor storage; retaining API input handles belongs
to the remaining measured-model consolidation, without a global cache.

REUSE the legacy Designer detection/dispatch branch byte-for-byte. Its remaining
coarse input-format import stays frozen in this file and the actual Designer
science owner remains separately frozen. This unit does not claim complete
measurement/Designer or parent acceptance. Narrowly review this exact owner
change; retire only decoder/import/type sites that have actually moved. The
public provenance report supplies the detected-format display label; Qt performs
no new format guessing or scientific calculation. Reuse authoritative capture
limits; centralize the independent existing 64-MiB editor collection policy,
which remains distinct from the sequence API's 96-MiB original-byte budget.

Level 1 covers ordinary capture hashes/BOM/binary/nulls/source planes, existing
measurement/animation/Designer/project cases, affected QML, format/Clippy/current
optimized build, boundary and traceability. Persistence risk expands to targeted
Rust/native saved-input cases against the exact accepted unchanged wheel. No
backend/binding/API, CXX/QML registration, schema or lifecycle change; parent
Level 2 and final Level 3 remain pending.

## UIU-DS-01 profile performance observation prerequisite

REQ-UIU-02/03/08/19/20/24/27 and UIU-DS-01 own this focused typed observation.
The profile-performance viewer still invokes private per-carrier MER/Shannon
owners and copies raw FEC counter types. REUSE existing public CarrierMer,
FecReport, MetricScope, ChannelJobStatus and quantity types. ADD focused
ofdm/api/full_loop/performance.rs and EDIT private reports.rs projection helpers.
`FullLoopResult::profile_performance() -> ApiResult<FullLoopProfilePerformance>`
returns original profile/key/status, one original-coordinate point per source
carrier, scoped typed MER/power/counts, ideal Shannon capacity MER, observation-
window FEC/BER, whole-run unrecovered payloads and the existing error text.
No observed metric is fabricated for absent/zero-error samples; the existing
sample/power quantities preserve those distinctions. Diagnostic TX reference
comparisons never enter receiver processing. Ideal capacity remains distinct
from DOCSIS operating thresholds, measured RxMER and coded throughput.

FullLoopCarrierPerformance contains source_offset, typed frequency_hz, authoritative
role, existing CarrierMer and optional typed ideal_shannon_mer_db. Original private
scientific owners supply every calculation; no scientific implementation changes.
The existing classic PerformanceReport reuses the same private typed projection
helper, preserving its public schema/fields. EDIT native full-loop result conversion
and root typing with `.profile_performance() -> dict`; no native DTO class or
existing signature change. Native edits/retained lifetime cannot mutate a result.

EDIT existing full-loop observation/facade tests and ADD independent private report
projection tests: unit reference/0.1 error yields 20 dB; explicit FEC partition and
source-mismatch counts; actual full source grid and empty/pre-cancelled scope;
classic direct-engine and native report compatibility. Rust acceptance precedes
binding implementation. Defer the Qt viewer until paired acceptance is committed.
Level 1 expands through affected shared reporting/Rust/native/facade/Qt consumers,
Clippy, analysis-only, CLI compile, exact new wheel/inventories and traceability.
Parent Level 2 and final Level 3 remain pending; completed FEC/API tasks stay closed.

The bounded independent Rust reference is the public adapter example
`crates/pnm-input/examples/profile_performance.rs`, which loads the existing
self-contained saved project through Project and runs its first admitted request.
Native Python repeats that same public project operation. The adapter's existing
JSON dependency is reused; facade manifests and all dependency locks stay exact.
Receiver QAM-history retention remains separate work; this prerequisite exposes
only the full-source-grid performance observation needed by its existing viewer.

## UIU-DS-01 profile performance viewer adoption

REQ-UIU-02/03/08/19/20/24/27 and UIU-DS-01 retain ownership. REUSE the accepted
FullLoopResult.profile_performance operation and public SourceProfileFrame/FecReport.
EDIT downstream/instruments/receiver/performance.rs to retain only owned public
observations and format existing JSON/plots. Remove its private MER/Shannon/FEC
imports; retain the independent bounded QAM-history route unchanged. Original
profile/channel identities, source frequency order, gaps, PLC bands, series IDs,
window FEC/BER and whole-run unfinished counts preserve existing presentation.
An observation failure becomes an explicit per-profile failure, without stale data.

EDIT downstream/common/constants.rs for existing retention, vertical scale,
rounding, drawing-bucket and series-role policies. Reuse shared frequency units,
horizontal ticks and single-carrier frequency denominator. Equal-valued vertical
ticks/axis defaults remain separate policies where their purposes differ.
EDIT the actual closed-loop worker test to compare public observations with the
existing rendered scalars and per-carrier zoom values; preserve all original
assertions. Level 1 includes that worker, the independent four-profile sweep,
PLC-band vectors, affected performance/FEC QML navigation, Qt format/Clippy and
optimized build, source-boundary and traceability checks. No backend/binding,
serialization, registration or lifecycle change; reuse the accepted API evidence
without rebuilding a wheel. Retire only this complete frozen file after review.
Parent Level 2 and final Level 3 remain unchecked.

## UIU-DS-01 channel and receiver spectrum adoption

REQ-UIU-02/03/08/09/16/17/18/24/27 and UIU-DS-01 own this existing-API consumer unit.
REUSE FullLoopResult.channel_spectrum, calibrated_spectrum, physical_bins,
amplifier_bins and response, plus the accepted shared spectrum renderer and
TransmissionConfiguration.physical_layout. EDIT channel/mod.rs, receiver/views.rs,
receiver/diagnostics.rs, transmitter/channel_input.rs and spectrum/plc.rs; remove
the unused private PSD formatter from spectrum/mod.rs. No public API extension.
Keep the existing retained-sample budget/continuity, raw FFT power separate from
PSD, amplifier-only reference separate from CPA/H/noise, and configured H separate
from received H-hat. Missing estimates never fall back to configured H.

Qt may group supplied finite power values and normalize drawing coordinates, but
may not calculate PSD, gain, power integration, log power, response or impulse data.
The raw FFT renderer adapts only the presentation portion of this repository's
Apache-2.0 scenario/measurement_view.rs at a0ddab1334e73b190e76ee51cc89f316707e19d4;
the original owner/header/license remain, no external source or new dependency is
imported, and all scientific values come from physical_bins. Preserve original
JSON, min/max gaps, ticks, source-frequency readouts and unavailable counts. Name
independent renderer/selector policies centrally. Keep the remaining receiver
QAM/soft-history route frozen and preserve its exact code when reviewing partial
diagnostics changes.

Level 1 covers actual worker/tap power and amplifier-QAM parity, independent FFT
power/response comparisons, existing PLC clipping vectors, affected SA/response
QML interactions, Qt formatting/Clippy/optimized build and boundary/traceability.
The already accepted paired APIs need no new wheel or repeated unrelated suites.
Retire complete files only after no direct scientific owner remains; record the
remaining diagnostics roots explicitly. Parent Level 2 and final Level 3 remain.

## UIU-DS-01 measured-input prerequisite gaps

REQ-UIU-02/03/08/09/21/24/27 and UIU-DS-01 own two concrete remaining gaps before
measurement/Designer/session adoption. The current unity-channel chart in
measurement_input/model.rs computes response projections privately; existing
ChannelEstimate.preview requires a measured original, and ChannelPreview supplies
only configured gain. Do not manufacture a capture or provenance for unity.
ADD ProfileCapture.unity_response_preview(profile_id, viewport, metric), returning
a ConfiguredResponsePreview with original grid/identity, existing bounded view,
explicit synthetic-unity reference and projection. Reuse response_value and
measurement_view numerical owners for all four existing projections. This operation
claims no measured response or numerology compatibility beyond the source grid.

The Designer currently repeats -30..120 dB, 256-capture and 4,000,000-value limits
and invokes private recipe validation. Existing RxMerDesign owns evaluation but
does not expose its complete admitted limits. ADD RxMerDesign.limits returning
an immutable RxMerDesignLimits report of existing baseline, capture/value/edit,
variation, tilt/ripple/notch and echo-distance domains. REUSE scientific constants;
centralize only the existing native Designer aggregate-value guard in its common
constants owner. Keep it distinct from equal-valued playback budgets. Change no
domain value, evaluator, saved recipe or acceptance semantics.

Rust signatures:
`ProfileCapture::unity_response_preview(u8, MeasurementViewport, ChannelResponseMetric)
-> ApiResult<ConfiguredResponsePreview>` and
`RxMerDesign::limits() -> RxMerDesignLimits`. Native mirrors the first with
`unity_response_preview(profile_id, metric="gain_db", start=0, count=0) -> dict`
and the second with `RxMerDesign.limits() -> dict`. No native class is added.
Keep crate/facade ownership and original import compatibility. Independent unity
values, raw byte IDs, viewports/ownership, existing Designer bounds and baseline
vectors, complete exact inventories and direct Rust/native process parity precede
Qt adoption. Rust tests pass before binding implementation; commit the paired
prerequisite first. No other speculative API extension belongs to this unit.
Parent Level 2 and final Level 3 remain pending.

The complete native acceptance runner must generate all five current optional
downstream reference processes, including these views and the preceding profile
performance operation. Wire those existing examples without relaxing its strict
no-skipped-tests rule. A focused runner contract discovers optional reference
guards and verifies complete wiring to real Rust example sources. Level 1 runs
the affected new process/current native cases and runner tests; complete native
acceptance remains a milestone/final gate.

## UIU-DS-01 measured-input and configured-channel adoption

REQ-UIU-02/03/08/09/16/17/18/21/22/24/27 and UIU-DS-01 own the remaining
measured-input cohort. Reuse the accepted public catalog, sequence, response,
channel preview and Designer contracts; no further measured-input API is planned.
The lead alone edits these downstream owners; parallel session preparation is
confined to new unregistered shell files.

EDIT measurement_input/model.rs, document.rs, mod.rs and plc.rs, channel_model.rs,
downstream/workbench/session.rs and project_file.rs, and their existing bridge
consumers. Retain the immutable original ProfileCapture alongside its legacy
selected SourceProfileFrame snapshot. Keep passive publication of selected RxMER
frame DTOs/options independent of scientific run admission so incomplete/mismatched
idle saved projects remain loadable. This is application input storage, without
a second simulation interface. Original bytes and original/decoded hashes remain
distinct. Public MeasurementSequence and ChannelConfiguration operations perform
scientific admission and previews; response tolerance stays independent of RxMER.
Retain all existing selectors, errors where meaningful, JSON keys, gaps, chart
units/coordinates, PLC overlays, partial-project behavior and FEC worker inputs.

REUSE merlin::downstream::{ProfileCapture, SourceProfileFrame, MeasurementSequence,
MeasurementSequenceOptions, RxMer, ChannelEstimate, ChannelConfiguration} and the
accepted synthetic-unity preview. No Qt private measurement projection, residual
power conversion, physical channel planning, gain calculation or response fitting
remains in migrated owners. Existing screen transforms and presentation-only
axis/tick policies stay in Qt with named constants. Designer's independent
noncompounding operation and bounded archived receiver migration remain explicit
subsequent owners within the same parent task.

Validate complete affected Qt owner tests and original worker/project assertions,
affected QML measurement/channel/Designer interactions, scoped Clippy/formatting,
latest optimized UI build, boundary and canonical traceability. Expand Rust/native
project compatibility checks if persisted admission changes; do not rebuild a
wheel for unchanged accepted APIs. Compare frozen entries explicitly and retire
only complete migrated owners. Parent Level 2 and milestone Level 3 remain open
until all downstream owners and dependencies are resolved.

## UIU-DS-01 Designer and export owner adoption

REQ-UIU-02/03/08/09/21/22/24/27 and UIU-DS-01 migrate the remaining nineteen
Designer/transport/export sites. EDIT measurement_input/designer.rs and document.rs,
bridge/rxmer_designer.rs and rxmer_export.rs. REUSE the immutable public
RxMerDesign, its authoritative limits, source-catalog flat/imported admission,
from_bytes, with_recipe and materialize operations, and the facade's native export
owner. Retain Qt's existing versioned envelope transport and control restoration;
only the backend evaluates/validates recipes or constructs flat source masks.
Changing collection position/count serializes the existing typed envelope back
through public from_bytes admission, without a new scientific API or evaluator.

Retain original baseline bytes and generated label/envelope conventions, restore
common controls without compounding previous edits, and preserve separate original
and decoded identities. The existing transport strips only an initial UTF-8 BOM
before Designer-envelope admission; do not invent a different scientific capture
or alter ordinary-capture provenance. Pure transport detection uses the existing
named BOM marker. Native operation results supply all samples; common-axis screen
projection, PLC clipping and timeline geometry remain presentation.

Keep the legacy 64 MiB UI collection budget distinct from public scientific count/
value/recipe domains. Validate original Designer/worker/export assertions and
affected QML editors, exact-wheel native Designer/saved-project compatibility,
formatting/Clippy, optimized Qt build, boundary and traceability. Preserve existing
independent numerical values and cancellation/atomic-adoption checks. Retire whole
entries only after all direct private owners are removed. Bounded receiver
QAM/soft history and final dependency removal remain separate downstream owners;
the parent and final numeric acceptance gates stay open.

The existing Designer QML consumes the public limits report for capture counts,
value-edit count, tilt, ripple, notch and random-deviation stepping. Distinct
scientific limits stay distinct even when numerically equal. Missing reports
never activate guessed domains; Rust still validates typed values. Existing
screen-only empty extent, padding and tick policies receive independent names in
the common application constants owner without changing their values. The
unchanged visual layout retains its existing dark-theme guide screenshot.

## UIU-DS-01 bounded receiver archive prerequisite

REQ-UIU-02/07/09/16/17/18/19/20/24/27 and UIU-DS-01 require preserving independent
receiver QAM/soft history after removing private scientific dependencies. Existing
FullLoopResult is an immutable live-result owner, but cloning it for each of 32
archived runs retains the complete job, source/captures and stages. The current
Qt archive intentionally retains only 8192 payload cells, 1024 NCP cells, 1024
cells per protected role and 16200 original decoder LLRs. This demonstrated
consumer gap needs one detached public owner before the Qt history migration.

| Path/symbol | Existing contract | Decision and owner |
| --- | --- | --- |
| `apps/qt/src/downstream/instruments/receiver/qam_history.rs`, Snapshot/retain | Independent bounded histories, uniform original indices floor(k*N/min(N,cap)), original decoder-word context; no waveform/job retention | ADAPT through a detached FullLoopReceiverArchive; retain run identity/status and the 32-run application policy in Qt. |
| `crates/ofdm/src/api/full_loop/cells/selection.rs`, Selected | Full-population filtering and original coordinate/role identity before output pagination | REUSE for live and detached reports; no Qt filtering or scientific reconstruction. |
| `crates/ofdm/src/api/full_loop/cells/queries.rs`, qam/cell_decisions | Scientific MER over all selected retained cells; bounded physical decisions and original likelihoods | SHARE private implementations between the existing result and detached archive, preserving old exports/defaults/results. |
| `crates/ofdm/src/api/full_loop/cells/soft.rs`, soft views | Independent cell and coding-order decoder planes; existing Rust likelihood statistics/histograms/envelopes | SHARE the existing owners. Archive the first actual nonempty retained decoder event with its exact offset, total, context and finite prefix; no synthetic LLRs. |
| `crates/ofdm/src/simulation/channel/protected_constellation/`, new/streams | Existing validated empty shell plus actual immutable protected observations | REUSE the constructor and copy only selected public stream pairs; do not clone hidden pilot-sequence work state or whole report vectors. |

ADD `crates/ofdm/src/api/full_loop/receiver_archive/` and paired native binding/test
modules; EDIT focused cell query helpers/re-exports, common domain constants,
namespace exports, operation traceability and the existing user/API guides.
FullLoopResult::receiver_archive(policy) returns an immutable detached archive.
Policy admits independently reducible payload/NCP/protected/decoder caps and a
caller byte budget. Existing Qt caps are the defaults/maxima for this bounded
history operation, distinct from live scientific capture and stage/query limits.
An independent eight-MiB maximum archive byte policy bounds each copy; the future
session aggregate budget and 32-run Qt cap remain separate. Count actual selected
nested LLR/context payload bytes with checked arithmetic before allocating copies;
allocator overhead and engine buffers are not claimed as counted bytes.

Archive info reports actual retained counts/bytes, correction mode and decoder
availability/original event index. qam, cell_decisions and cell_soft_view reuse the
same typed queries and scientific helpers as live results. decoder_soft_view has
one explicitly selected archived event, preserving its physical coding context.
Archive clones share only detached immutable data; dropping all live requests,
results and original capture handles must not invalidate queries or retain those
owners. Missing streams/events remain unavailable; reduction never fabricates
complete observations or changes original coordinates/status/provenance.

Before Qt adoption: independently check 10003→8192 (last10001), empty/zero caps,
all six roles and original metadata, first nonempty/truncated decoder context,
byte/overflow/domain rejection before copies, all-selected MER beyond2048, square/
cross-QAM decisions and strict soft histogram endpoints/extrema. Prove live-query
behavior unchanged and detached lifetime using weak original-owner references.
Pair native ownership, dictionary isolation, rejection and fresh Rust-process
numeric parity from an exact freshly built/installed wheel. Run affected Rust API,
facade/legacy imports, native Clippy and operation/export tracing; no Qt source or
frozen entry changes in this prerequisite. Source-only tests do not establish Qt
or parent acceptance. Then migrate the three receiver owners and matcher-invisible
soft calculations as one consumer batch, remove dependencies and execute the
existing DS Level2/downstream Level3 gate before integrating prepared agents.

The implemented owner uses checked fixed/payload/context-byte preflight and shares
only the detached data through Arc. Protected reports start from the existing
validated empty constructor so hidden pilot-sequence state is not retained.
Independent history caps and byte/transport policies live in the downstream
common constants module. The unchanged 2048-cell QAM/soft and 128-decision limits
also receive separate shared names used by both live and archived queries.
Paired runtime evidence and exact 600/570 operation tracing are recorded under
[archive acceptance](validation.md#uiu-ds-01-bounded-receiver-archive-api-accepted).
All original completed API/FEC tasks and the twelve remaining frozen Qt sites stay
unchanged; consumer adoption and DS milestone acceptance remain separate.

## UIU-DS-01 receiver consumer and dependency exit

REQ-UIU-02/03/07/08/09/16/17/18/19/20/24/27 and UIU-DS-01 now adopt the independently
accepted archive prerequisite. EDIT receiver/qam_history.rs, selection.rs,
diagnostics.rs and soft_display.rs; ADAPT the channel plot, performance publication
and affected worker tests. REUSE the existing merlin facade/native-input feature;
remove the three direct scientific Cargo dependencies only after all calls migrate.
The boundary record retires exact complete owners/dependency records with linked
acceptance and retains its original inventory/history; never refresh a surviving
private site merely to pass the check.

History stores FullLoopReceiverArchive with its application profile/status, not
FullLoopResult clones. Archive failure makes that instrument unavailable without
dropping completed FEC/performance results. Public live/archive qam supplies all-
selected metrics and a bounded original-cell page; public decisions supply physical/
normalized slicing, labels and distances. Public cell/decoder soft views supply
histograms, endpoint counts, contiguous extrema and coding context. Missing decoder
data stays unavailable and never borrows physical-cell soft values. Latest-result
queries keep full scientific retention; independent histories keep their old caps.

Qt retains only legacy selector transport, run routing, labels, chart geometry and
read-only presentation bundles. Backend selection/metrics remain authoritative:
never replace an all-selected sample count with a display-page length. Reuse common
scatter/percentage geometry constants and name remaining direction-specific selector/
axis policies without tying equal-valued independent domains together. Cell filters
and scientific physical-bin/order validation pass through the public query owner;
the unchanged one-million-symbol legacy UI selector policy stays application-owned.

Preserve all existing test names and independent vectors. The accepted backend
archive/live-query tests own the original 10003→8192/last10001 and beyond-display
23.010299956639813/20-dB numerical oracles; keep the exact nearest-point/perfect/
empty checks there as needed. Existing soft-owner tests preserve strict endpoints,
zero/empty and32768→1024 extrema. Qt tests continue to exercise public runtime
ownership, missing-run isolation, unchanged metric/soft formatting and actual
worker/channel/slicer behavior. Independent worker noise/reference checks must use
the complete original selected population from public report/query data, rather
than silently limiting their oracle to the new display page. Retain their equations,
tolerances and truth-plane separation.

Level 1 checks cover changed Qt Rust/QML consumers and relevant backend oracles,
formatting/Clippy, optimized UI and native probes, zero-exception boundary and
traceability. Because this closes the downstream dependency frontier, complete
relevant Qt/feature compatibility and the existing downstream Level2/Level3 gate
must pass before checking the parent or integrating prepared session/shell work.
No schema, scientific algorithm, Qt registration or Python payload change is
planned by this consumer batch; reuse the exact accepted native artifact.

## UIU-SESSION-01 explicit ownership foundation

After [DS-01 milestone acceptance](validation.md#uiu-ds-01-receiver-public-api-and-dependency-exit),
review and adopt the isolated `65c91e73f9c09fce56b880b7c303ca72483cf2ee`
preparation ([agent handoff](agent-handoffs/session-shell.md)) into the existing
integration branch. This is the first bounded unit
of UIU-SESSION-01, tracing REQ-UIU-03/06/07 and the
[session architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle).
It adds no scientific API, backend algorithm, project schema or QML registration.
The existing public backend controls remain the only execution controls.

| Disposition | Exact owner | Contract / integration decision |
| --- | --- | --- |
| Add, reviewed agent preparation | `apps/qt/src/shell/mod.rs` | Separate typed accepted direction configurations/results and atomic identity/revision transitions. REUSE as application ownership; actual downstream store migration follows. |
| Add, reviewed agent preparation | `apps/qt/src/shell/identity.rs` | Checked process/session/generation/run/revision identities; no wrap/reuse or backend ID substitution. |
| Add, reviewed agent preparation | `apps/qt/src/shell/dispatch.rs` | One shared Busy lease including pause/cancelling; cancellation does not establish termination. |
| Add, reviewed agent preparation | `apps/qt/src/shell/retention.rs` | Shared checked application byte reservations preceding retained allocation/clone and following queued/old payload ownership. No guessed scientific memory limits. |
| Add, reviewed agent preparation | `apps/qt/src/shell/tests.rs` | Fixed-size application fixtures for isolation, callback identity, Busy/pause/cancel/destruction, byte accounting and actual-target Save/Open guards. No simulation placeholders establish engine acceptance. |
| Reuse, subsequent host unit | Existing downstream inputs/results, receiver worker and bridge owners | Move implicit stores to explicit downstream ownership; preserve actual engine/result/UI behavior. The host must supply conservative reachable-allocation accounting. |
| Reuse, subsequent host unit | Existing upstream public configuration/control/report owners | Separate actual upstream session and real public-control adapter; no Qt scientific configuration or execution clone. |
| Defer to their tasks | Shell tabs, native file actions, recovery and global registration | Prepared components and source-only tests do not accept UIU-SHELL-01, FILE/RECOVERY or either direction adapter. |

The sole worker lease must remain outside panic unwinding and reach a terminal
envelope on normal completion, partial cancellation, error or spawn failure.
Only known worker termination may acknowledge it. Abandonment cancels and keeps
Busy admission; a new coordinator/gate must not bypass an unfinished worker.
Callbacks carry the exact token and are rechecked at actual state adoption.
Tab selection is presentation only. Neither callback routing nor scientific
input/result lookup may use the selected direction as an implicit fallback.

Accepted/queued/candidate/old payloads retain their reservations until actual
release, including replacement failure and destroyed presentation owners. Host
admission must bound reachable payload bytes and container/shared-root counts
under the [approved contract](retention-contract-decision.md); reservation wrappers
alone do not prove aggregate memory acceptance. Configurations remain immutable snapshots;
executing mutable backend sessions stay worker-owned. Configuration changes are
atomic and rejected while that same owner is Busy in this primitive; the actual
adapter must preserve the accepted edit/cancellation contract when adopting it.

Level 1 validates all16 owner cases in the integration checkout with pinned
Rust/Clippy/format tools, exact source/notice/number review, boundary and document
traceability checks. No Cargo graph or compiled Qt consumer changes in this
foundation unit, so wheel/core/Qt executable rebuilds are not required. The latest
DS executable remains current. UIU-SESSION-01 stays unchecked until actual store/
worker migration, two real direction consumers, aggregate bounds and all Level 2
lifecycle/compatibility/Qt checks pass. The prepared source and its own test pass
never substitute for these remaining gates.

Integration review caught the prepared external test-module path at the existing
Qt boundary. Move that exact test body beside its source owner as `tests.rs` and
use ordinary `mod tests;`. Keep the guard unchanged; record the rejected first
boundary invocation and rerun the16 owner cases/Clippy/format/boundary after this
placement-only adaptation. No scientific or test expectations change.

## UIU-SESSION-01 explicit downstream context migration

Continue the accepted foundation by removing unbound downstream scientific data
lookup. REQ-UIU-03/06/07, UIU-SESSION-01 and the
[session architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle)
own this focused consumer unit. Preserve all existing scientific/public APIs,
editor acceptance, actual results, independent vectors and file schema behavior.

| Disposition | Exact owner / inspected contract | Decision |
| --- | --- | --- |
| Add / Edit | `apps/qt/src/downstream/workbench/store.rs` and `session.rs`: GUI-thread INPUTS/SAVED singletons | ADD an explicit clonable downstream context with accepted inputs/sections, latest result, bounded instrument histories and prepared editor transfer. REUSE existing DTOs unchanged. All reads/writes require that context. |
| Edit | `src/downstream/simulation/closed_loop.rs`, `instruments/transmitter/preview.rs` | Bind each lifecycle/preview model to an explicit context; queries and invalidation use their own owner. Preserve background frozen requests and cancellation behavior. |
| Edit | `src/downstream/instruments/receiver/{qam_history,performance,selection,diagnostics,views}.rs` | Replace global result/history routing with context parameters. Keep authoritative public queries and original all-selected/scientific planes. |
| Edit | `src/downstream/instruments/{channel/mod,transmitter/views,transmitter/messages,transmitter/channel_input}.rs` and channel input preview | Propagate the requesting adapter's context; no selected-tab or latest-other-context fallback. |
| Edit | `src/downstream/workbench/project_load.rs`, `project_file.rs`, corresponding tests | Prepared inputs/models remain independent until guarded GUI adoption. Transfer editors only through the target context; preserve Send background loading and avoid retained-owner cycles. |
| Edit | `apps/qt/src/bridge/{source_input,profile_input,measurement_input,rxmer_designer,closed_loop,transmitter_analyzer,receiver_instruments,workbench_file}.rs` | Capture a fixed downstream bootstrap context at QObject construction, then pass it explicitly to all data/model operations. No QML/public invokable signature or CXX-Qt registration changes. Channel instrument methods belong to measurement_input.rs; no separate channel bridge exists. |
| Reuse | Existing original-source path registry in `src/platform/input_file.rs` | Genuine application-wide no-clobber infrastructure, not scientific data routing. Future two-direction adoption must clear only its owner's paths while checking the union; leave its existing bounds/behavior intact until that identity wiring. |
| Subsequent unit | Shared dispatch/identity/retention foundation and actual upstream configuration/report/control owners | Bind coordinator identities, worker terminal envelopes and conservative aggregate accounting after explicit storage. The context-only unit does not accept these remaining contracts. |

Keep the bootstrap registry limited to constructing fixed downstream context
handles, never resolving selected-direction data. Isolated contexts permit two
independent populations in the same process. Every scientific input/result/query
call receives its actual handle. The context preserves existing finite histories
and thread-safe background transfers; it must not own a prepared model that
strongly retains the same context. Prepared models rebind on actual GUI adoption.
No default scientific operation, numerical constant or accepted input is changed.

Acceptance: preserve all existing assertions and add actual context-isolation,
missing-result/history and prepared-adoption tests. Exercise affected Qt Rust/
QML consumers, Clippy/format, final boundary, optimized build/native Open probes
and traceability. Core/native Python/CLI/package algorithms/contracts are unchanged,
so do not rebuild their wheel or repeat their completed milestone suites. The
current authoritative scalar/provenance values must remain exact. UIU-SESSION-01
stays unchecked until the later real callback/dispatch/resource and complete
session gates pass. This is a finite storage migration, not a new parent plan.

## UIU-SESSION-01 terminal worker transport

Continue REQ-UIU-03/06/07 and the existing session task with one shared application
transport for both direction adapters. The scientific operation/control remains
direction-specific. Its completion must be known before dispatch admission can
reopen; a cancel request, GUI destruction or panic is not completion evidence.

| Disposition | Exact owner / consumer | Decision |
| --- | --- | --- |
| Add | `apps/qt/src/shell/worker.rs` | Bounded typed callbacks carrying the genuine lease token; keep the sole lease outside caught scientific-operation unwinding. Terminal envelopes own known-completion acknowledgement, including dropped/disconnected GUI owners. |
| Edit | `apps/qt/src/shell/mod.rs` | Export the shared application transport beside existing dispatch ownership. No backend API, numerical policy or Qt registration. |
| Add | `apps/qt/src/shell/worker/tests.rs` | Real threads and dispatch leases cover success/error/panic, backpressure, dropped owners and deterministic failed OS-spawn injection. This validates the transport, not either engine's integration. |
| Adapt, subsequent consumer | Downstream `simulation/closed_loop/worker.rs` and new upstream controller | Reuse their existing public controls/results with this single transport; retain independent semantics. Host validates callbacks against actual accepted identities before adoption. |

The producer supplies the existing direction-specific channel capacity. The
transport adds no timeout, byte limit, scientific defaults or background polling.
It never joins from Qt. OS spawn failure returns the original lease to the host;
normal/error/panic completion delivers exactly one terminal envelope. Dropping a
known-terminal envelope may safely acknowledge termination, while dropping an
ordinary active lease retains Busy as before. Detached worker completion after
GUI destruction acknowledges only after the scientific closure has returned or
unwound. No duplicate worker/thread abstraction belongs in either adapter.

Acceptance uses all standalone session/transport tests, warnings-denied Clippy,
formatting, final boundary and traceability. Actual two-direction coordinator,
conservative aggregate retention and complete Level 2 session acceptance remain
required before checking UIU-SESSION-01. No scientific/public API change justifies
a wheel rebuild or repeated full workspace regression in this source-only unit.

The subsequent [retention accounting decision](retention-contract-decision.md)
records the demonstrated public metadata gap and the subsequently approved
payload-byte/container/root contract. The decision does not accept runtime
aggregate admission; actual public metadata and host integration remain required.

## UIU-SESSION-01 accepted edits during cancellation

Preserve the downstream adapter's existing accepted-edit behavior when adopting
the shared coordinator: a valid edit cancels old work and invalidates its result,
but the old worker keeps global Busy until terminal acknowledgement. Add explicit
`configure_downstream_cancelling` / `configure_upstream_cancelling` transitions to
`apps/qt/src/shell/mod.rs`; retain the existing reject-while-Busy operations.
Preflight closing, accounting ownership and both checked revisions before any
cancel request or state mutation. An edit in the idle other direction must not
cancel the active direction. Scientific admission and immutable candidate creation
remain with existing public owners; no scientific API or run default changes.

Extend `apps/qt/src/shell/tests.rs` with accepted-edit/stale-callback/terminal,
other-direction isolation and failure-before-cancellation cases. Reuse terminal
transport tests. This source-only prerequisite traces REQ-UIU-03/06/07 through
UIU-SESSION-01; real direction wiring and implementation of the subsequently
approved aggregate accounting contract remain required. Validate standalone tests/Clippy/format, final boundary
and canonical traceability without rebuilding unchanged scientific consumers.

## UIU-SESSION-01 actual upstream controller preparation

The isolated session agent prepares an actual public-API consumer for
REQ-UIU-03/06/07 and UIU-SESSION-01, reusable by later UIU-US-02/REQ-UIU-12.
Review the complete prepared commit before adopting; no main/Cargo/bridge/QML
registration or scientific implementation belongs in this preparation.

| Disposition / exact owner | Contract |
| --- | --- |
| Add `apps/qt/src/upstream/mod.rs`, `session.rs` | `PreparedSession::from_document(document, observation_policy, reservation)` calls existing public admission off-GUI. `UpstreamController::new(key, prepared)` binds an immutable host-issued identity. |
| Reuse shared shell transport | `start(lease, control, result_reservation, capacity)` runs only public `Simulation::run_with_control`. The same `Arc<UpstreamControl>` belongs to host admission and the controller; cancellation is idempotent. No separate thread/terminal abstraction. |
| Add controller lifecycle | Progress returns the real public snapshot with its token; pause/resume/cancel require that exact active token. `poll` consumes at most one event and returns the original typed terminal/sole lease. The canonical host must revalidate before UI adoption. |
| Preserve scientific ownership | Move the real Simulation out of a short-lock slot during execution; an unwind guard restores it on every operation exit. Charged `Arc<Retained<...>>` owners keep configuration and queued/old report reservations through actual release. |
| Add `apps/qt/src/upstream/tests.rs` | Existing tracked 50-kHz public configuration exercises actual complete/cancel/pause/error results, provenance, stale identity and charged alias/disposal behavior. Fixed accounting ticks test reservation lifetime only, never a backend byte bound. |
| Reuse agent-owned handoff | Exact origin, focused commands/artifacts and remaining host/registration/resource hooks. Lead alone updates canonical documentation/checklist. |

The controller trusts independently audited host reservations and introduces no
byte estimates/defaults. It must not establish aggregate acceptance using fixture
counter ticks. Run actual public-API tests and warnings-denied Clippy with existing
compiled libraries, plus format/boundary/traceability. No changed scientific/API/
binding/layout source requires a new wheel or full workspace regression. Real Qt
registration, canonical two-direction integration, audited aggregate admission
and complete Level 2 session acceptance remain mandatory before checking the task.


## UIU-SESSION-01 approved multidimensional retention admission

The user approved [payload bytes and container/root counts](retention-contract-decision.md)
on 2026-11-23. This unit implements the application accounting primitive for
[REQ-UIU-07](../../../requirements.md#req-uiu-07), under the existing
[session task](../../checklist.md#task-uiu-session-01) and
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle).
It does not accept aggregate backend or direction-host memory admission.

| Disposition / inspected owner | Contract / gap / decision |
| --- | --- |
| ADAPT `apps/qt/src/shell/retention.rs`, `RetentionBudget` | Existing checked bytes-only shared reservations gain independent payload-byte, ordered-entry and shared-root ceilings. Preflight all checked additions and limits under one lock before changing any counter. No guessed production defaults. |
| REUSE `Retained`, `RetentionReservation` | Reservations move with ownership and release all three charges only after data drops; shared aliases retain one charge until final drop. Independently cloned roots require separate reservations. |
| PRESERVE existing `new`, `reserve`, `used` | Bytes-only primitive compatibility; the constructor admits zero entries/roots rather than silently granting unlimited populations. These operations alone cannot account for real backend roots. |
| ADD `apps/qt/src/shell/retention_tests.rs`; EDIT `shell/mod.rs` | Export the application quantity model and test exact/exceeded limits, each overflow, transactional failures, concurrent admission and queued/old/final-drop ownership. Existing coordinator/transport/real upstream controller cases remain. |
| DEFER public scientific metadata and host adoption | Scientific owner crates must expose audited root footprints and pre-production bounds through public Rust/native Python APIs before Qt uses them. No private structures or serialized-size estimates. |

Level 1 runs all combined primary-checkout ownership/controller cases, production
and test Clippy with warnings denied, format, final API-boundary and requirement/
link/history checks. No registered Qt, scientific API, Python, serialization or
presentation code changes in this unit; reuse cached scientific artifacts and
the current Qt executable. Complete Qt/feature/compatibility Level 2 acceptance
still precedes checking UIU-SESSION-01. Historical validation receipts remain
unchanged; previous archive byte statistics keep their original meaning.


## UIU-SESSION-01 public retained-root metadata prerequisite

The demonstrated session consumer gap and [approved contract](retention-contract-decision.md)
require metadata over actual retained scientific roots. This additive prerequisite
belongs to [UIU-SESSION-01](../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../requirements.md#req-uiu-03) and
[REQ-UIU-07](../../../requirements.md#req-uiu-07), under the
[session architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle).
The existing API tasks and scientific acceptance remain complete; they are not
reopened. Root metadata is passive and changes no algorithms, limits or outcomes.

| Disposition / exact owner | Contract and missing behavior |
| --- | --- |
| ADD `crates/phy-api/src/retention.rs` and owner tests; EDIT `src/lib.rs` | Common `RetainedFootprint` has payload_bytes, ordered_entries, shared_roots (usize). All metadata includes the fixed root once, requested container capacity, nested/cached payloads and conservatively repeated Arc payloads/edges. Cross-crate helpers are implementation support, hidden from the facade. Checked addition/multiplication rejects overflow. No allocator/tree/refcount header estimate. |
| EDIT `crates/merlin/src/lib.rs`, common exports | Re-export only the genuinely common typed footprint. Keep independent PHY dependencies on merlin-api; no PHY/facade cycle. |
| ADAPT `crates/ofdma/src/api/session.rs`, `simulation/report.rs` and their existing scientific storage owners | `Simulation::retained_footprint(&self) -> ApiResult<RetainedFootprint>` includes idle options/configuration/document/provenance. `BatchReport::retained_footprint(&self) -> PhyResult<RetainedFootprint>` includes original/canonical provenance, configuration/options/plan, grant/decoded decisions and private observations, including skipped role gains and receiver/soft stores. Preserve caller spare capacities and repeated shared payloads. Exhaustive owner destructuring detects added fields requiring review. |
| ADAPT subsequent downstream roots | FullLoopRequest/SweepPlan/Result/ReceiverArchive and independently accepted inputs require their own audited implementations; archive historical length-based statistics remain unchanged. |
| EDIT existing native adapters/declarations and operation matrix after Rust checks | Mirror each implemented root operation in installed Python using one common immutable metadata type and the original structured error categories. No Python sizing calculation. No new wheel until related Rust/native metadata changes are ready together. |
| SUBSEQUENT pre-production bounds and Qt adoption | Passive current-root metadata cannot bound a future result. Prepared request/session output bounds and archive copy preflight remain separate mandatory admission gates before Qt production/copy. |

The implementation helper trait reports only dynamic children; the root walker
adds fixed size exactly once. Vec/String capacity is counted without allocating
or cloning. Container entries include owned key/value payloads separately from
excluded private nodes. Every Arc edge counts its reachable pointee and children,
conservatively repeating aliases rather than an unbounded deduplication table.
No serialized JSON sizing or fixed multipliers can replace this owner audit.

Rust tests use independent capacity/delta anchors, private cached/serialization-
skipped ownership, actual complete/partial runs and passive query compatibility.
Paired installed native tests compare metadata against separately executed Rust
references on the same target; these target-layout quantities are not portable
serialized project settings. Run affected Rust/native consumers, API export/
compatibility checks, Clippy, formatting, boundary and traceability before accepting
this focused API unit. Host admission and full session Level 2 stay open.


The first paired implementation unit covers upstream roots only:

| Rust operation | Native Python operation | Failure / lifecycle / acceptance |
| --- | --- | --- |
| `merlin::upstream::Simulation::retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint>` | `merlin.upstream.Simulation.retained_footprint(self) -> merlin.common.RetainedFootprint` | Pure idle inspection; native existing Busy guard and interpreter release; original ResourceLimit on checked overflow. Tests include spare capacity, caches, rejected configure and paused concurrent access. |
| `merlin::upstream::BatchReport::retained_footprint(&self) -> PhyResult<merlin_api::RetainedFootprint>` | `merlin.upstream.BatchReport.retained_footprint(self) -> merlin.common.RetainedFootprint` | Pure immutable complete/partial report inspection with native interpreter release; original ResourceLimit on overflow. Independent Rust/native references include both numerologies and estimate/soft enabled/disabled stores. |
| `merlin::common::RetainedFootprint` fields | Immutable native properties `payload_bytes`, `ordered_entries`, `shared_roots` | Target-layout quantities, no Python calculation, no constructor/mutation, owned metadata survives original owner release. |

Lead-owned paired files: ADD `bindings/python/src/retention.rs`,
`bindings/python/tests/test_retained_footprint.py` and
`crates/ofdma/examples/upstream_retained_footprint.rs`; EDIT native lib/upstream
session/report registration, common Python module/stub, upstream stub, facade
namespace test, API operation matrix and the focused
[user guide](../../../user/api/retained-resources.md). Reuse the existing native
Busy/error paths and one distribution; do not add bindings to either scientific
crate or expose walker implementation support through the facade.


### Downstream retained-root preparation

The subsequent downstream owner batch uses the same approved quantities and
helper contract. It adds passive `retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint>`
to FullLoopRequest, FullLoopSweepPlan, FullLoopResult and FullLoopReceiverArchive.
The concrete Qt consumers are prepared serial runs, accepted actual reports and
detached old QAM/soft histories; their current public logical/length counts do
not include private cached arrays, all capacities or nested ownership.

| Disposition / existing owner | Responsibility |
| --- | --- |
| EDIT `crates/ofdm/src/api/full_loop/{request.rs,sweep.rs,result.rs,receiver_archive/mod.rs}` | Four public passive root operations. Preserve original archive retained_bytes statistic and copy policy exactly. |
| EDIT existing API input/source/sequence/profile owners and scientific child owners as required | Owner-local complete dynamic-child visitors, exhaustive field/variant audits and shared leaf arithmetic; include MeasurementPlayback caches, original input/provenance, settings, protected pilot sequence and retained stage stores. No algorithms, default, cache shape or serialization changes. |
| ADD `crates/ofdm/tests/api_retained_footprint.rs`; ADD owner-private tests as needed | Independent capacity/cached/Arc anchors, actual complete/partial results, history detachment and passive original-query compatibility. Preparation tests require execution in the primary checkout. |
| EDIT lead-owned `bindings/python/src/full_loop/{request.rs,sweep.rs,result.rs,receiver_archive.rs}` and root type stub | Expose the same four existing native classes' passive methods returning `merlin.common.RetainedFootprint`, releasing the interpreter and preserving Rust errors. Genuine downstream imports retain their existing root class aliases. |
| ADD `bindings/python/tests/test_downstream_retained_footprint.py`, `crates/pnm-input/examples/downstream_retained_footprint.rs`; EDIT `crates/merlin/tests/namespaces.rs` | Exact original single/multiple-RxMER saved projects through the same public loader in independent Rust/native processes; observations enabled/disabled and complete/pre-cancelled roots, detached history lifetime, original reports/provenance and legacy type identity. Compile-check the Copy helper's rejection of heap-owning leaves. |
| SUBSEQUENT lead host unit | Independently accepted input roots and presentation copies still need separate host charges; the four root operations alone cannot establish aggregate acceptance. Future output and archive-copy bounds remain separate public-owner prerequisites. |

The isolated agent may prepare only the downstream Rust owner/test patch. The lead
retains all shared contract, facade, native, architecture/checklist and acceptance
ownership. No future-result or archive-copy pre-production bound is accepted by
passive metadata; those remain explicit integration gates.


The downstream audit identifies two fixed foreign leaves: std::time::Duration
and `fec_codec::statistics::FecCounters` (five private u64 fields, Copy). Add the
hidden shared support helper `fixed_children<Value: Copy>(&Value)` returning zero
dynamic children; fixed storage is already owned by the enclosing root/container.
The explicit Copy bound and owner source audit prevent silently accepting a later
heap-owning counter. Borrowed/static data remains charged at its actual owner;
this is not a blanket implementation for arbitrary foreign types. Keep codec
statistics and scientific behavior unchanged and add no dependency to the common
leaf. Reuse this helper only at the audited fixed-leaf fields.

All four Rust methods return `ApiResult<merlin::common::RetainedFootprint>`;
their Python signatures are `retained_footprint(self) ->
merlin.common.RetainedFootprint`. Checked arithmetic failure retains ResourceLimit.
Quantities describe the actual owner: cloning a Vec/String may shrink spare
capacity, so equal scientific inputs do not promise equal footprints after an
arbitrary clone. The independent reference deliberately matches the native
worker's existing request-clone dispatch. No clone implementation is changed.

The extra public-owner batch is passive metadata only. Relevant complete API,
measurement/playback, protected-constellation/archive and facade tests, owner
Clippy/formatting and installed native consumers cover its changed graph; full
unrelated scientific suites remain at the session/milestone gate. Rebuild the
native wheel once after both owner and wrapper sources are ready. Preserve prior
receipts and record precise new cases and inventory additions before acceptance.


#### Downstream root acceptance

The [primary validation receipt](validation.md#uiu-session-01-downstream-retained-root-metadata)
and [exact executed cases](session-downstream-metadata-operation-test-results.json)
accept all four current-root operations with 103 Rust cases/doctests and 34 installed
native cases. Original source statements, complete older operation rows and
historical archive statistics are preserved. This accepts passive metadata only;
[output construction accounting](retention-contract-decision.md#output-construction-boundary-requiring-clarification),
pre-production bounds and actual session admission remain open.

### Independent input and idle-preview owner closure

This is remaining UIU-SESSION-01 work under REQ-UIU-03/07, separate from
pre-production output bounds. The user has approved retained-owner aggregate
accounting with separate finite temporary-construction/numerical execution limits;
see the [construction boundary](retention-contract-decision.md#output-construction-boundary-requiring-clarification).
A read-only audit of actual Qt retention finds the following additional opaque
owners. Their private data cannot be charged solely through a future request or
last result: partial accepted inputs, drafts, independent previews and queued
Designer edits can remain alive before a full-loop request exists.

| Concrete retained consumer | Scientific owner / disposition |
| --- | --- |
| `apps/qt/src/downstream/workbench/session.rs`, partial Inputs; `inputs/source_input/model.rs`, draft/accepted SourceFile | ADD passive method on existing `SourceInput` in `crates/ofdm/src/api/admission.rs`; reuse its complete audited visitor. |
| `inputs/profile_input/{mod.rs,document.rs}`, original input/catalog/decoded presentation document | ADD passive methods on existing `NamedCapture`, `ProfileCapture` and `CaptureProvenance` in existing API input owners. The decoded presentation JSON remains separately charged application storage. |
| `inputs/measurement_input/{model.rs,document.rs}`, accepted response and captured originals | ADD passive method on existing `ChannelEstimate`; ordinary RxMER frame DTOs/original application byte Arcs remain public application bookkeeping. |
| `instruments/transmitter/preview.rs`, PreparedPreview/Job/Analyzer | ADD passive methods on existing `TransmitterPreviewRequest` and `TransmitterPreview`. Complete their owner-local engine request/result visitors in existing transmitter_preview.rs, waveform/spectrum.rs and plc/descriptors.rs, including full private PSD and retained original provenance/source. ConfiguredPhysicalLayout is public except nested CaptureProvenance, handled by that owner's method. |
| `inputs/measurement_input/designer.rs`, `bridge/{measurement_input.rs,rxmer_designer.rs}`, live/cloned/queued designs | ADD passive method on existing `pnm_input::RxMerDesign`; visit exact original input, typed record/baseline/recipe and cached materialized RxMer. Add owner visitors to existing DS RxmerRecipe and PNM Stored; no scientific recalculation. |

All eight methods use `retained_footprint(&self) -> ApiResult<RetainedFootprint>`
and the existing checked ResourceLimit category. Retain constructors first,
exhaustive owner field/variant audits and all original scientific statements,
schemas/defaults/copy policies. No additional public types or operations beyond
this demonstrated consumer set are authorized by this unit.

Lead-owned paired work adapts existing native SourceInput, ProfileCapture,
ChannelEstimate, TransmitterPreviewRequest/TransmitterPreview and RxMerDesign
wrappers, root stubs, facade tests and independent Rust/native references. Python
has no independently retained NamedCapture or CaptureProvenance wrapper: inputs
are bytes/name arguments and copied provenance dictionaries. Record that language
difference explicitly rather than introducing unused native classes. Shared
metadata remains `merlin.common.RetainedFootprint`; Python performs no sizing.

Add the direct `merlin-api` common-leaf dependency to `pnm-input` solely to reuse
the contract/walkers across crate boundaries; the lead owns manifests and all
locks. This introduces no facade/PHY cycle, scientific dependency or source pin
change. The DS preparation agent owns only its named API/input/TX/recipe child
owners and new `api_input_retained_footprint.rs`. A second isolated preparation
agent owns only the PNM Designer/Stored visitors and its new integration test.
Neither edits shared contracts, manifests, bindings, registration or canonical
documents. The [Designer handoff](agent-handoffs/designer-retention.md) records
its source-only scope. Preparation commits explicitly leave tests unexecuted; the lead adopts
and validates the combined owner graph before native exposure/acceptance.

Remaining visible application owners still need host bookkeeping: saved JSON,
project-load candidates, original application byte Arcs, Designer recipes/session
DTOs, independently copied result/instrument arrays, queued previews/exports and
history status wrappers. Context handles alias stores and must not recursively
expand the complete Context again. Standalone metadata does not charge any owner
automatically or accept the aggregate runtime gate.


The [primary metadata evidence](validation.md#uiu-session-01-independent-input-and-preview-metadata)
accepts these eight actual owners with 84 Rust/47 installed native cases and
additive 615/585 operation tracing. This completes the scientific current-input
metadata closure only; application charges and output pre-production bounds
remain separate acceptance work.

### Pre-production detached archive bounds

This next UIU-SESSION-01 prerequisite traces REQ-UIU-03/07 and the
[approved construction boundary](retention-contract-decision.md#output-construction-boundary-requiring-clarification).
The real QAM history consumer must reserve its detached target while its live
source and old archives remain charged. Preserve the existing receiver_archive
operation and historical info.retained_bytes statistic without behavioral changes.

| Disposition / owner | Selected public contract and acceptance |
| --- | --- |
| ADD `crates/ofdm/src/api/full_loop/receiver_archive/bounded.rs`; EDIT existing `receiver_archive/mod.rs` registration only | Add FullLoopResult.receiver_archive_retained_bound(&self, policy: &FullLoopReceiverArchivePolicy) -> ApiResult<RetainedFootprint>, and receiver_archive_bounded(&self, policy: FullLoopReceiverArchivePolicy, limit: RetainedFootprint) -> ApiResult<FullLoopReceiverArchive>. Both preserve original decoder-event/selector/error semantics. |
| REUSE legacy `receiver_archive/copy.rs` | Keep legacy operations and statistics unchanged. Shared selection/accounting helpers may become visible only to the new sibling, without body changes. The new path preflights every dimension before copying, then normalizes only detached storage to exact final capacities. Never mutate the source or retain live result ownership. |
| ADD owner/private and API regression cases | Exact bound equals actual metadata for omitted/complete/partial observations, payload/NCP LLRs, protected streams and decoder context; each dimension exact/+1 exhaustion; checked overflow; unchanged observations/status/coordinates, legacy statistic, aliases/lifetime and accepted source after rejection. Boxed final buffers enforce final capacities; temporary builders use separate finite count/byte limits. |
| LEAD subsequent native/facade/reference pairing | Mirror the two operations on the existing native FullLoopResult with the same typed common metadata, GIL release and structured errors. Independently compare actual Rust/native observations, preflight, bounded copy and old statistics using installed-wheel tests. |

The archive target has no ordered entries and one shared allocation edge. Count
its fixed root and actual retained payload once, including requested nested LLR,
role-vector and context-string storage. Recreate protected reports through their
existing constructor so the live private pilot cache remains absent, matching the
legacy archive. No scientific recomputation, new QAM filtering, view restriction,
standard-library growth assumptions or late-only oversized-product rejection.
The lead owns canonical documents, shared contracts, native bindings and primary
acceptance. The [isolated archive handoff](agent-handoffs/archive-retention.md)
records its owner-only source preparation and unexecuted acceptance tests. Future full-run result/report bounds and host admission remain
separate gates; this public copy operation cannot complete UIU-SESSION-01.


The [primary archive acceptance](validation.md#uiu-session-01-pre-production-detached-archive-bounds)
passes 79 Rust/28 installed native cases and 24 independent reference cases, with
617 Rust/588 native operation rows. Native common quantities gain an immutable
constructor matching the existing Rust struct literal; all earlier operations
remain unchanged. This accepts exact target preflight/copy, not aggregate host
admission or future full-run bounds.

### Pre-production upstream report bounds

The actual upstream controller currently receives an externally asserted result
reservation, then calls legacy run_with_control. UIU-SESSION-01 / REQ-UIU-03/07
requires backend proof before that consumer can accept aggregate retention.
An independent audit identifies all destinations in the existing BatchReport
ownership walk: configuration/options/plan, grants and decoded word buffers,
observation maps/traces/private gains, estimate/soft Arc records, source/canonical
configuration/provenance and failure text. Existing logical limits remain intact.

| Disposition / owner | Selected operation and enforcement |
| --- | --- |
| ADD focused private owner modules under `crates/ofdma/src/api` and `simulation`; EDIT existing `api/session.rs` registration | Simulation.batch_report_retained_bound(&self) -> ApiResult<RetainedFootprint> and Simulation.run_with_control_bounded(&mut self, control: &RunControl, limit: RetainedFootprint) -> ApiResult<BatchReport>. Freeze the current accepted options/configuration/observation policy; preflight checked quantities and all retained source exports before control claim or source progress. |
| REUSE existing engine, profile/schedule and current-root visitors | Preserve legacy run/run_with_control and scientific execution. Use exact final-capacity destinations for the opt-in retained product, including nested and shared caches. Temporary builders use existing finite execution limits; no Vec growth-factor assumption. All complete/cancelled/failed reports fit the pre-admitted bound. A final footprint assertion verifies an invariant, never replaces pre-production admission or discards completed science. |
| EDIT private owner-local configuration/observation/receiver/soft normalization helpers only where needed | Preserve source bytes/hashes, values, missing observations, ordering, identities and independent RX truth. Exhaustive fields/variants, checked arithmetic, explicit finite diagnostic policy. Introduce no new public scientific operation beyond the two demonstrated session methods. |
| ADD focused Rust tests and LEAD native pairing | Exercise typed/document/measured configurations, observations disabled/enabled, exact/each-dimension exhaustion, overflow, complete/pre-cancelled/runtime partial results, unchanged accepted sessions and reusable controls after preflight rejection. Compare science with legacy execution and independent Rust/native processes; elapsed time is not an equality oracle. |

The isolated upstream agent owns only `crates/ofdma` owner/test sources plus a
unique report-retention handoff. Lead owns shared contracts, all manifests/locks,
facade/native/Qt consumers, canonical documents, adoption and acceptance commits.
No shared-shell or archive edits are delegated. Host simultaneous input/old/new
charges and real Qt registration/wiring remain separate mandatory tasks.

For these bounded operations, native Python needs to construct immutable common
resource quantities, analogous to Rust's existing public struct literal. The lead
has added RetainedFootprint(payload_bytes, ordered_entries=0, shared_roots=0) on the
existing native class, preserving its three read-only fields and target-usize
nonnegative conversion. This is caller policy/metadata, not a reservation or a
scientific storage estimate. No extra Rust type or constructor is required.


### Pre-production downstream result bounds

The downstream worker must reserve its future public FullLoopResult alongside
accepted inputs, requests, old results and queued presentation/archive copies.
Current-root metadata cannot prove that future product. This is the remaining
UIU-SESSION-01 / REQ-UIU-03/07 downstream backend prerequisite, independent of the
archive and upstream report implementations.

ADD FullLoopRequest.result_retained_bound(&self) -> ApiResult<RetainedFootprint>
and FullLoopRequest.run_bounded(&self, control: &FullLoopControl,
limit: RetainedFootprint) -> ApiResult<FullLoopResult> in focused private full-loop
owner modules. Preserve legacy run and every scientific algorithm/limit. Preflight
checked complete product quantities and freeze final input/settings/policy ownership
before control.begin. Reuse existing engine orchestration, then enforce exact
final retained capacities without changing science, missing observations, source
provenance or completed partial outcomes. Temporary bounded engine/output builders
use separate execution limits; final products are admitted before production.
Do not assume Vec/String growth factors or reject only after a result exists.

The bound covers the complete audited result graph: job/settings/inputs, detailed
words, recovered file prefix, carrier metrics, receiver estimates, payload/NCP/
protected constellations including private pilot cache, retained stage maps/events/
contexts/roles and failure text. Bounds use accepted scientific geometry and
existing finite word/cell/observation/source policies, plus fixed values and
independent ordered-entry/shared-root quantities. Exhaustive field/variant audits
and owner-local private helpers preserve module ownership. No public getter or
new scientific API beyond these two demonstrated consumer operations is added.

The isolated downstream agent may edit only `crates/ofdm` owners/tests excluding
all receiver_archive files, plus a unique result-retention handoff. Shared contracts,
manifest/lock, facade/native/Qt, registration, canonical documents and acceptance
belong to the lead. Validate exact and insufficient dimensions, arithmetic
overflow, observed/unobserved complete/pre-cancelled/source-file results, rejection
before control/source advance, unchanged legacy queries/statistics and direct
Rust/native parity. A final invariant assertion is diagnostic, never admission or
an expected failure that discards completed science. Actual application charges
and complete Qt/session acceptance remain separate gates.


### Pre-production idle transmitter preview bounds

The demonstrated consumer is
`apps/qt/src/downstream/instruments/transmitter/preview.rs`: PreparedPreview/Job
can retain a queued or accepted TransmitterPreview independently of a full-loop
result. Its existing current-root footprint cannot admit the future worker target.
UIU-SESSION-01 / REQ-UIU-03/07 therefore also requires the following small pair.

| Disposition / owner | Contract and reuse |
| --- | --- |
| ADD `crates/ofdm/src/api/transmitter/bounded.rs`; EDIT transmitter.rs registration/control-only dispatch | TransmitterPreviewRequest.preview_retained_bound(&self) -> ApiResult<RetainedFootprint>, generate_bounded(&self, control: &TransmitterPreviewControl, limit: RetainedFootprint) -> ApiResult<TransmitterPreview>. Shared private engine dispatch preserves legacy generation/cancellation/progress/source behavior. |
| EDIT existing `simulation/transmitter_preview.rs` private snapshot helper only | Read the original frozen settings/layout, without source advance; no API or Qt dependency in the engine. Existing generator/scientific bodies remain unchanged. |
| REUSE result-retention private normalization and existing spectrum constants | Normalize cloned input/provenance/layout buffers before measuring the frozen target. Bound actual maximum sample population from the existing Welch-window budget/FFT geometry, plus full PSD density and fixed engine/root storage. Exact final Vec capacities and frozen snapshots enforce the admitted target before publication. |
| LEAD native/facade/tests/reference | Pair the two methods on the existing native request, use common immutable quantities, release the interpreter and preserve structured errors. Exact/insufficient dimensions, checked arithmetic, unchanged fresh control on rejection, cancellation, PRBS/file/FFT modes, legacy sample/PSD/provenance and direct Rust/native references are acceptance gates. |

The caller reserves source/request, old and queued preview targets independently.
Temporary waveform/PSD/output builders keep their existing finite geometry,
window/source and cancellation limits. No aggregate scratch/RSS claim, scientific
algorithm, changed profile support or altered legacy sample query is introduced.
Root owns only transmitter API/engine/PSD files; the downstream preparation agent
owns other full-loop/input normalization helpers. No concurrent file editing is
permitted. Final output bounds for full-loop, upstream and idle preview may share
one scoped native build after all owner sources and wrappers are ready.

### Next application consumer: proven upstream controller admission

Under UIU-SESSION-01 / REQ-UIU-03/06/07, the next focused application unit replaces
caller-asserted quantities in the prepared upstream controller with actual public
Simulation metadata and BatchReport preflight bounds. Prepare/configure through
the existing finite backend builders off-GUI, then reserve final accepted ownership
before adoption. Temporary builders use the approved separate execution policies;
remove the former preparation-peak aggregate claim. Report preparation must stay
off-GUI and bind its admitted target to the exact immutable controller/session.
Start validates that prepared target and uses run_with_control_bounded; the shared
transport, genuine global lease, partial results and final-alias charge survive.

The isolated controller preparation scope is only `apps/qt/src/upstream/session.rs`,
its existing `tests.rs` and one unique controller-admission handoff. The lead owns
all shared retention/coordinator modules, registration, manifests, canonical
documents, root adoption and executed acceptance. Private backend structures and
new public scientific operations are excluded. Include typed scientific and
application-wrapper quantities with checked arithmetic and no guessed multipliers;
list remaining host/control/channel/DTO owners explicitly instead of claiming
complete aggregate acceptance. Production registration and complete Level 2
acceptance remain later integration gates. The [primary controller evidence](validation.md#uiu-session-01-upstream-controller-target-admission)
now validates this scope against the accepted bounded backend; the separate
[downstream dispatch adoption](validation.md#uiu-session-01-downstream-qt-retained-target-adoption)
uses bounded targets without claiming shared aggregate host admission.

### Downstream Qt retained-target operation adoption

UIU-SESSION-01 / REQ-UIU-03/07 can now reuse the independently accepted
session-output and archive pairs in the three existing downstream consumers.
EDIT `simulation/closed_loop/worker.rs::run_one` to obtain the narrowed request's
result bound and call run_bounded on its existing worker. EDIT
`instruments/transmitter/preview.rs::Analyzer::generate` to preflight/generate the
preview inside its existing worker. EDIT `instruments/receiver/qam_history.rs::retain`
to preflight and use the bounded archive copy before replacing the detached view.
Reuse all scientific/control/notification/error handling and presentation data;
introduce no public API, numerical algorithm, QML or registration change.

This enforces backend target capacities in real Qt call paths. The supplied target
quantities are the backend bound itself; they do not establish a shared aggregate
host policy. Input/request, old/queued outputs, copied observations/history DTOs,
controls and transport still require separately admitted application charges.
Keep the parent unchecked. Existing real closed-loop/preview/history consumer
cases and changed-path Clippy/format/boundary checks must pass; build the latest
optimized executable. Native Python/Rust science evidence at 1841d92 is reused:
this Qt-only dispatch adoption changes none of those linked APIs or algorithms.
Complete session Level 2 and milestone/final Level 3 remain separate gates.

### Actual downstream host and terminal adoption

UIU-SESSION-01 / REQ-UIU-03/06/07 now prioritizes real consumer wiring. REUSE the
existing ShellSessions, checked identities, DispatchGate and terminal transport;
add no accounting abstraction or scientific API. The concrete application host
tracks direction-specific readiness and callback generations. Actual scientific
configurations/results remain in their existing downstream Context/Model and
upstream controller owners; metadata readiness is never scientific acceptance or
proof of aggregate resource closure.

| Owner and disposition | Required behavior and reason |
| --- | --- |
| EDIT `apps/qt/src/main.rs` | Register the existing application shell/upstream Rust modules in the native binary, with reusable application contracts exported at its root. Normal Cargo binary tests compile the actual components; no new library target, Cargo crate/dependency, scientific facade or QML type is introduced. |
| ADD `apps/qt/src/shell/application.rs`, `shell/constants.rs`; EDIT `shell/mod.rs` | One concrete clonable application host over existing ShellSessions, separate direction readiness, genuine global leases, canonical callback validation and existing retention budget. Fixed metadata charges use actual layouts. The host accepts explicit resource limits; normal desktop defaults are named application safety policies, not PHY limits or RSS claims. No aggregate acceptance until all existing scientific and application owners participate. |
| EDIT `downstream/workbench/session.rs`, `simulation/closed_loop.rs` and `closed_loop/lifecycle.rs`, bridge restore | Bind each real context to its canonical host; isolated preparation/tests receive independent hosts. Reconfiguration cancels only the old owned generation. Rebound prepared models obtain the target host's actual key. Start acquires the shared lease before invalidating accepted results or dispatching science; a rejected second Start preserves them. |
| EDIT `downstream/simulation/closed_loop/worker.rs`, adjacent worker tests | REUSE the existing scientific sweep/control/timing code through spawn_worker. Profile/Rejected/Finished remain data messages; only the common terminal releases the genuine lease. Every callback is checked against the canonical host immediately before adoption. Preserve actual complete/partial science, runtime narrowing, pause and bounded backpressure. |
| REUSE existing upstream controller | Test both actual engines sharing this same host/gate, including paused/cancelling Busy, stale generations, terminal release and destroyed owners. Its QObject/configuration/presentation wiring follows; no fake scientific result establishes acceptance. |

Root owns the host, Model/Context/bridge/native registration and canonical
acceptance. The session agent exclusively prepares the worker and its tests in
its existing worktree. The upstream UI agent owns recovered upstream QML/tests
only; shared build/module registration remains root-owned. Existing caches and
profiles stay intact; no repeated wheels or unrelated complete regressions.

Level 1 requires both real direction consumers, existing changed-path downstream
Rust cases, affected QML lifecycle cases, Clippy/format/boundary and optimized
native build/probes. Public scientific/native/persistence contracts do not
change, so reuse their exact accepted evidence. UIU-SESSION-01 remains unchecked
until actual upstream QObject wiring, complete retained-owner closure and full
relevant Level 2/milestone acceptance pass. Do not add further accounting helpers
without a demonstrated owner or failing acceptance case.

The first focused compile demonstrated that a separate library target in this
package also links CXX-Qt's generated registration archive, which needs the
binary's QObject implementations. Keep the existing binary target instead of
changing CXX build/link ownership; register the application components there.
The failed attempt remains in the local receipt. A new fixture also needed normal
field assignment because Model implements Drop; no production behavior or
existing assertion was changed to resolve that test-construction error.

Final adoption is transactional at the actual Receiver Apply/Restore consumers:
prepare the existing public request first, perform one host generation admission,
then commit options/verification and clear old views. Rejected metadata admission
preserves the old accepted request/key/report and reports failure to Apply.
The new regression uses a valid changed transmission candidate under explicit
synthetic counter pressure, then verifies later success after release. Existing
scientific compatibility assertions remain unchanged.

The new configuration/admission methods live in focused
`closed_loop/lifecycle.rs`; the existing runner retains execution/state
presentation. Public operations precede private helpers in each implementation.
This placement changes no operation, signature or scientific expression.


### UIU-SESSION-01 concrete direction adapter and request-owner closure

REQ-UIU-03/06/07 retain the existing acceptance gates. The demonstrated next
runtime gaps are the missing upstream QObject consumer and uncharged accepted
and worker downstream request/plan owners. Reuse the existing public APIs,
retention reservations, canonical host and common terminal transport. Add no
scientific API, algorithm or general accounting framework.

| Owner / disposition | Contract and focused acceptance |
| --- | --- |
| ADD `apps/qt/src/upstream/workbench/` | A GUI-thread model owns its accepted controller and last token/report. Configuration parsing/PreparedSession construction and PreparedRun preflight run in finite background preparation workers. Preserve accepted state on rejection; prevent detached callbacks from adopting after owner destruction or replacement. Start acquires the same ApplicationSessions lease; public pause/cancel/progress remain direction-specific. Revalidate canonical tokens immediately before GUI report adoption and finish genuine terminals, including stale/error paths. |
| ADD `apps/qt/src/bridge/upstream_session.rs`; EDIT `bridge/mod.rs`, `build.rs`, `upstream/mod.rs` | Thin registered UpstreamSession QObject delegates to the application model. Apply accepts the existing public configuration document, with a named finite byte policy. State exposes bounded scalar status/error and decimal-string identities; never full scientific arrays. Use the same fixed downstream construction host, not selected-tab routing. No QML layout change in this unit. |
| EDIT downstream `closed_loop.rs`, `closed_loop/lifecycle.rs`, `closed_loop/worker.rs`; ADD focused owner tests | Charge real accepted request/plan metadata through existing Retained/Arc reservations. Share immutable admitted owners with workers instead of uncharged payload clones. Keep candidates and old aliases charged through replacement/final drop. Test each aggregate dimension rejecting a real accepted candidate without changing old request/generation/report; preserve original science assertions. Per-profile working requests and results remain distinct owners requiring their actual admission; do not claim complete closure for this partial unit. |

Lead owns upstream model/bridge/native registration and canonical records. The
session agent owns only the listed downstream request/plan paths plus adjacent
focused tests and its unique handoff in the existing isolated worktree. No agent
may edit shared host/budget primitives, manifests, registration or canonical docs.
Source-only preparation does not pass acceptance. Lead adopts exact reviewed
paths, executes relevant real-engine/lifecycle and all affected registration
consumers, formatting, Clippy, boundary/traceability, fresh Qt build/native probes,
and saves independently testable focused units. Existing scientific/native API
and persisted contracts remain unchanged; full session Level 2 is still required.


The upstream preparation completion carries the canonical configuration key
captured when Apply starts; a later host generation rejects that stale candidate.
Stop during target preparation suppresses dispatch while restoring the same idle
controller when its finite preflight completes. Destruction drops the completion
receiver and never joins. Immutable controller ownership remains in a guarded slot
during target preflight so a preparation unwind/disconnection can restore it.
Normal reports validate against the host before UI adoption, and stale terminals
still consume their genuine lease. Session/generation/run values cross QML as
lossless decimal strings; absent BER remains null. Full retained-owner closure is
still required and must not be inferred from this registered adapter.


### UIU-SESSION-01 actual downstream result and presentation owners

The actual Worker now admits each backend result target before scientific
execution through result_retained_bound/run_bounded and existing Retained/Arc
lifetimes. PreparedResult borrows the exact runtime-narrowed request. Its checked
charge replaces the FullLoopResult fixed root with the actual ResultData layout
once and adds the final Arc root; backend entries/roots otherwise pass through.
The reservation survives queued Box, current Arc and old aliases until final drop.
Start preserves accepted science until a genuine new Profile replaces it; target
rejection never discards the previous report. Stored ResultData observations now
have [separate admission](#uiu-session-01-stored-observation-and-capture-admission);
independent instrument/transport copies remain separate open owners. See [primary acceptance](validation.md#uiu-session-01-downstream-backend-result-target-admission).
No completed scientific result may be discarded because a later presentation
copy cannot be admitted.
The isolated [source preparation](agent-handoffs/downstream-result-admission.md)
retains its historical source-only checks; primary runtime acceptance is separate.

The existing observation/capture projectors are application-owned copies, not new
scientific algorithms. Their concrete typed vectors/maps/JSON and intermediate
query copies now use explicit pre-copy admission, exact field/variant walks and
finite existing query policies. Preserve the same observed values, nulls, source
coordinates, messages, paging and instrument behavior. If a target bound cannot
be proved from the existing public contract, report the exact consumer/dependency
instead of inventing a multiplier, private backend read or public API.

Session agent owns only closed_loop.rs, closed_loop/worker.rs and adjacent tests,
focused result-retention modules, closed_loop/observations.rs, transmitter/capture.rs
and their focused tests plus a unique handoff in its existing isolated worktree.
Root retains all shared primitives, bridge/registration, canonical records and
acceptance. Input/editor, independent preview/history and infrastructure owners
remain separately enumerated; this scope must not claim their closure. Source-only
agent preparation is followed by lead consumer tests, Clippy/format/boundary,
traceability and fresh optimized/native checks. No repeated wheel or unrelated
complete scientific suite is needed without a linked scientific/API change.


The accepted request/plan unit uses two real allocations per admitted owner:
Arc<T> for immutable science and Arc<Retained<Arc<T>>> for charged application
ownership. The public footprint includes T once; add the actual retained-wrapper
layout and two shared roots. Rebind reserves full target coverage before cloning
only the inner handle. This is a concrete project-transfer owner, not a general
accounting abstraction. Its primary acceptance is linked from the official
checklist; all independent source-agent checks remain historically source-only.


### UIU-SESSION-01 existing observation-query copy envelope

The actual Observations::from_result consumer requires pre-copy bounds for public
stage_inventory and stage returns. The owner audit proves a reuse path, avoiding
new public operations. A stage copy is one same-typed source snapshot slot, its
unchanged context and a data subsequence; all eight data variants have fixed-size
values. Inventory contains fixed Info entries, each backed by a source snapshot.
After exact final-capacity normalization, either individual Rust return has payload
bytes no greater than the immutable source result's retained_footprint payload,
with zero ordered entries/shared roots. Empty inventory fits the source root.
Use size_of assertions for layout inequalities; no numeric multiplier or clone
capacity assumption. This is a per-copy envelope: simultaneous copies, paged
concatenation, Qt JSON/observations and Python transport/objects remain separate.

| Disposition / exact owner | Implementation and acceptance |
| --- | --- |
| EDIT `crates/ofdm/src/api/full_loop/observations/store.rs` and `observations.rs`; ADD focused `observations/query_storage.rs` | Reuse existing exact-capacity normalization for current query returns; reuse it in retained-result finalization rather than duplicate it. Preserve values, errors, original contexts/coordinates and signatures. Document the conservative source-root envelope on these existing operations only; it is not a general future-output bound. |
| ADD focused private query-storage tests; EDIT `crates/ofdm/tests/api_result_retained_bound.rs` | Exhaustive eight variants, string/grid-role capacity spares, null/zero and source immutability, actual observed/disabled results, layout dominance, all three quantities and owned lifetime. Keep all existing vectors/assertions. |
| REUSE native `bindings/python/src/full_loop/observations.rs` and existing native observation/bounded-output regressions | Existing native methods call the same Rust owner. Rebuild one exact native wheel for changed linked code, test installed imports/values/errors and direct Rust parity; no new signatures or operation-matrix rows. |

Root owns this scientific-owner storage/API documentation unit, native pairing,
all canonical records and acceptance. The session agent remains isolated to its
Qt result-owner preparation. No DSP/FEC math, default, schema or namespace change.
Run focused owner/API/native and affected Qt consumers; preserve caches. Commit
this independently validated prerequisite before any Qt consumer relies on its
new capacity guarantee. Complete session Level 2/milestone gates remain open.


### UIU-SESSION-01 per-profile execution request admission

The worker now closes its selected-profile/runtime-narrowed owner gap using the
accepted finite preparation boundary: build/narrow the temporary candidate through existing public owners,
drop the intermediate request, then admit the final candidate's actual public
footprint using request_retention::admit_request. Borrow that charged immutable
owner through result preflight, execution and projection. The accepted source
request/plan remain independently charged. No new public API, accounting framework
or scientific reconstruction is needed. Independent QA confirmed this consistency
with existing accepted input-candidate preparation; its review is source-only.

EDIT request_retention.rs for the concrete owned-candidate admission helper and
worker.rs for ownership transfer, retaining cfg(test)-only legacy borrowed fixture
entry points. EDIT existing exact-pressure fixtures only to include the real
working owner during execution; preserve their assertions and all prior science.
ADD focused child tests beside request_retention tests for exact current-root
charges, runtime narrowing, intermediate release and rejection in owned dimensions before control/source execution, exact zero-entry
admission, and existing plan/result entry-limit rejection. Run targeted DS/US shared lifecycle/project
consumers, QML run/instrument behavior, Clippy/format/boundary/traceability and
fresh optimized/native checks. Reuse the unchanged backend wheel/scientific
acceptance. REQ-UIU-03/06/07 and UIU-SESSION-01 own this partial closure; all other
retained owners and full task/milestone acceptance remain open.


### UIU-SESSION-01 stored observation and capture admission

REUSE the accepted existing query envelope and shared Retained budget. Adopt the
isolated [observation/capture preparation](agent-handoffs/downstream-observation-admission.md)
into the primary checkout. Inventory, accumulator, page and replacement each
reserve separate source-envelope storage across both immutable passes. An
allocation-free typed/JSON selection plan reserves the exact final display target
before its existing serializer/copy builders run; normalize final capacities.
No scientific algorithms or public APIs are added. REQ-UIU-03/06/07 and
UIU-SESSION-01 own this stored projection closure.

Root EDITS ResultData/worker to pass the canonical budget. Projection rejection
keeps the same completed backend, empty display containers and a borrowed static
diagnostic, borrowed by the existing Receiver reason and projected into the
observation_warnings UI response without
retaining a fallback Vec/String. Declare ResultData's backend/fixed-layout guard
last, after display buffers/guard. Existing pressure/lifetime fixtures account for
actual newly charged projection children and asynchronous query/target overlap;
keep their scientific assertions and final exact cleanup. ADD one actual worker
fallback/queue/old-alias test; run all new prepared helper/owner cases, affected
real DS/US/project/instrument compatibility, QML, Clippy/format/boundary/traceability
and fresh optimized/native checks. Reuse unchanged backend/native acceptance.
Independent instrument-query/transport, input/preview/history and infrastructure
owners and complete SESSION Level2/milestone acceptance remain separate open gates.


### UIU-SESSION-01 idle transmitter preview owner admission

Adopt the isolated [preview source preparation](agent-handoffs/idle-preview-admission.md)
without changing scientific generation or presentation values. REUSE public
TransmitterPreviewRequest metadata and preview_retained_bound/generate_bounded.
The concrete prepared owner contains fixed accepted timing, optional request and
only the public PhysicalLayoutView actually consumed by the analyzer; drop the
unused configured key/provenance wrapper during finite preparation. Count actual
carrier/span capacities, opaque request metadata and both real Arc wrappers.
Worker dispatch shares the admitted immutable root without deep input clones.
Pre-admit the backend product plus its Retained wrapper before generation; the
same charge follows Box queue, accepted/stale product and final disposal.

Root EDITS bridge/transmitter_analyzer.rs to attempt fallible prepared rebind before
cancelling/replacing the accepted model. Target failure must preserve active
accepted work. Rebind supports the sole idle prepared-project consumer; reserve
complete target coverage before aliasing the same scientific inner root. Retain
ordinary invalid-input behavior and the original three scientific test bodies.
REQ-UIU-03/06/07 and UIU-SESSION-01 own this focused prepared/product closure.
Run seven new owner cases, existing preview science/readiness/timing, affected
project/context/shared-direction lifecycle, QML, Clippy/format/boundary/traceability
and optimized/native checks. No backend API or wheel rebuild is needed. Remaining
outer control/channel/diagnostic/transport and independent input/history owners
and full SESSION Level2/milestone acceptance stay open.


### UIU-SESSION-01 detached receiver history admission

REQ-UIU-03/06/07 and UIU-SESSION-01 own selective adoption of the
[isolated archive preparation](agent-handoffs/receiver-archive-admission.md).
REUSE existing receiver_archive_retained_bound/receiver_archive_bounded, canonical
Context budget and Retained guard. EDIT qam_history.rs; ADD its focused retention
and test children. Replace the public archive handle once with the actual Snapshot
layout (including its last guard), preserve backend entry/root quantities and add
the actual outer Snapshot Arc. Reserve before copying and move the archive into
that wrapper without a second scientific clone. Registry/get/current/old aliases
share the charge through final disposal. Host rejection preserves the accepted
entry/status; ordinary backend failures keep their historical unavailable behavior.

Execute six new owner cases plus the original archive/query oracle, affected
receiver/instrument/shared lifecycle/project cases, QML, Clippy/format/boundary and
traceability, optimized build and native probes. Values, reference planes, public
signatures and dependencies remain unchanged, so reuse backend/native evidence.
This is snapshot-only admission: Entry diagnostics, registry Vec capacity,
performance records, other independent display/transport/input/project and outer
infrastructure owners remain separate. Complete SESSION Level2 and applicable
milestone/finalLevel3 acceptance are still required; keep the parent unchecked.


### UIU-SESSION-01 copied receiver performance admission

REQ-UIU-03/06/07 and UIU-SESSION-01 own selective adoption of the
[performance source preparation](agent-handoffs/receiver-performance-admission.md).
REUSE profile_performance and existing FEC builders as finite temporary preparation,
without assuming collect/clone capacities. Normalize final stored Point/text/JSON
capacities, measure actual Entry layout and JSON map population, then admit before
registry adoption. Retain the independently charged scientific source and old
entries throughout preparation/admission. A last Retained guard follows child
disposal; no new scientific/public operation or presentation Arc is introduced.

EDIT performance.rs; ADD its focused retention/test children. Make begin/retain/
reject host-fallible. Admit every initial entry before clearing either history;
original query errors retain their admitted failed-entry behavior. Root EDITS
closed_loop.rs: remove the premature clear, return the genuine acquired lease on
begin rejection before token/state mutation, surface display admission failures
while still adopting completed science. Setup errors keep their original text.
Add an actual Model rejection/no-dispatch case, plus six source owner cases and
all affected receiver/closed-loop/project/context/shared lifecycle tests. Check
QML, Clippy/format/boundary/traceability and optimized/native execution. Reuse
unchanged backend/native evidence; no wheel rebuild. Registry capacities, QAM
status/catalog, other input/editor/project/query/transport and infrastructure
owners and full SESSION Level2/milestone acceptance remain explicitly open.


### UIU-SESSION-01 atomic accepted inputs and prepared projects

The next concrete consumer is the existing Context Inputs/saved stores. Source,
profile, measurement, loss and power Apply currently publish independently after
mutating their editor; project adoption also publishes before later model rebind.
REQ-UIU-03/06/07 requires one immutable admitted accepted-state owner and an
infallible final adoption boundary. This extends existing owners; no scientific
API, namespace, schema, decoder, simulation behavior or Qt registration changes.

| Owner/file | Decision and concrete behavior |
| --- | --- |
| EDIT downstream/workbench/session.rs; ADD session/accepted.rs and focused tests | Replace the two mutable stores with one immutable Arc/Retained accepted Inputs+sections. Existing readers use charged shared snapshots or scoped borrows. Prepare the complete replacement and matching saved section before adoption; rejected candidates preserve all old aliases/data. Reuse public opaque root metadata and actual visible DTO/JSON capacities; include the actual wrapper/root once. |
| EDIT bridge/source_input.rs, profile_input.rs, measurement_input.rs; existing input model Clone derivations/acceptance helpers only as needed | Validate a finite editor candidate, prepare matching published inputs/section, admit the complete target, then commit accepted editor/state together. Keep invokable signatures and original bytes/hashes/values. Measurement dependency refresh publishes only when its accepted profile-presence dependency changes, through the same transaction. |
| ADD reviewed passive visitors beneath existing Source/Profile/Measurement/Designer owners and inputs/retention.rs; EDIT only owning registrations/forwarders | Count actual requested storage, public root children and independent copies. Reuse one concrete JSON/input-DTO visitor across these real consumers; no new budget/admission abstraction. Private application Designer Origin/Record ownership stays owner-local. Context/accounting graph ownership remains a separate lead-owned consumer. |
| EDIT workbench/project_load.rs, project_file.rs, bridge/workbench_file.rs and existing receiver/preview preparation hooks | Prepare/charge canonical target owners before queued project retention. Preserve independent preparation contexts to avoid Context/prepared-model cycles. Admit all target scientific/editor/input/document ownership before global generation, accepted sections, native path or original-path registry change. Final GUI transfer reuses pre-admitted target aliases; failed/stale/cancelled candidates preserve the old project. |
| EDIT existing closed_loop lifecycle and input/Designer callers | Borrow the admitted snapshot while preparing existing public requests; avoid uncharged deep snapshots escaping to workers. Existing finite construction stays separate from aggregate retained ownership. |

The project preparation worker receives the exact accepting host budget and
identity without retaining a target Context/prepared-store cycle. Existing public
operations still own validation and science. Same-budget prepared aliases need
no second scientific copy/reservation on GUI reload; identity/readiness adoption
must remain checked. Save/recovery serialization borrows accepted sections or
keeps an explicit copy charge through writing, preserving exact legacy JSON and
atomic destination rules. Do not silently drop guards when moving data.

Validation covers actual capacity/entry/root arithmetic, old snapshot/final-alias
lifetimes, rejected coupled Apply, read refresh passivity, fully queued cancelled/
stale/failed project ownership, no owner cycles, target admission rollback and
existing saved/legacy project behavior. Run all affected editor/project/receiver/
preview/shared-direction lifecycle cases, native project compatibility where
applicable, QML, Clippy/format/boundary/traceability and optimized/native checks.
Unchanged linked scientific APIs do not require rebuilding wheels. Independent
returned instrument/Qt transport, registry spare capacity and outer infrastructure
owners plus complete SESSION Level2/milestone acceptance remain open.


This implementation unit closes accepted Inputs/sections, their shared read
aliases, coupled editor Apply, borrowed Save serialization and queued project
input admission against the captured target budget/key. The general project
model preflight described above remains the next concrete owner work: prepared
editors, receiver/preview target readiness and independent metadata/path owners
must all be admitted before claiming complete project/SESSION acceptance.
The [source Apply handoff](agent-handoffs/runtime-apply-transactions.md) records
its unexecuted source-only checkpoint; primary tests establish runtime evidence.
Only the shared JSON/profile/RxMER visitor leaves are adopted from the frozen
passive metadata preparation in this unit. The additional model/Designer visitors
remain source-only until their actual retained consumers are wired. Existing
performance JSON metadata now reuses that same actual-capacity leaf.

The worker also retains the original charged Snapshot captured at Open start.
Canonical SessionKey alone misses an accepted update before receiver refresh;
accepted adoption compares original/current owner identity and swaps under the
same store lock. It preserves intervening edits without copied JSON fingerprints
or target Context cycles. [Primary execution and limitations](validation.md#uiu-session-01-atomic-accepted-input-and-saved-document-admission)
cover this source-audited gap and the focused unit; remaining all-target project
preflight stays separate.


### UIU-SESSION-01 prepared project target preflight

The concrete remaining consumer performs receiver/preview target reservations
inside GUI reload after the project inputs/path already changed. Reuse existing
public root metadata/admission and the accepted-owner transaction; preflight all
three scientific prepared owners against the target budget on the load worker.
Retain isolated candidate Contexts in stored models, preventing a target/prepared
cycle. No scientific/public API, schema or Qt registration change is needed.
REQ-UIU-03/06/07 and UIU-SESSION-01 own this focused continuation.

| Owning files | Concrete change |
| --- | --- |
| EDIT closed_loop/lifecycle.rs and transmitter/preview.rs; ADD focused tests | Existing scientific preparation takes an explicit target budget, propagating host rejection while retaining legacy incomplete scientific readiness. Check idle roots/budget ownership before adoption. Final attachment uses a precommitted genuine key and the same guard aliases, with no second reservation, science or generation advance. |
| EDIT shell/application.rs and focused host tests | Guard the existing downstream readiness commit with the expected key under its session lock. Retain existing metadata/cancel/lease behavior; wrong/stale keys and admission failures preserve the old host. |
| EDIT workbench/session/accepted.rs, session.rs, project_load.rs and focused project tests | Hold the original accepted-owner check through the last fallible target readiness commit, then swap the admitted input owner. Stamp the detached receiver's genuine key, publish transfer state/results/path only after all preflight succeeds. Successful ordinary Apply/New invalidates unconsumed older prepared models; failed edits retain them. |
| EDIT bridge/closed_loop.rs and existing preview attachment caller only as needed | Consume already-target-admitted models without generation/readiness changes or another root reservation. Preserve receiver-first reload order and downstream actions. |

Test all target dimensions, stale key/accepted-owner rejection, late readiness
pressure, queued/cancelled/drop and unchanged generation during attachment; retain
existing complete/incomplete project, old report/paths, public science and no-cycle
assertions. Run relevant model/project/shared lifecycle, QML/boundary/Clippy/format,
traceability and fresh optimized/native checks. Reuse unchanged scientific/native
inventories; do not rebuild a wheel. Editor/Designer graph and independently
retained transport/infrastructure closure plus complete SESSION acceptance remain
separate concrete owners, rather than being accepted by this preflight unit.

Selective preparation source: `f8a576e6bdb43712a4b69d677ae1858b79596121`,
with primary integration fixes for model encapsulation, Drop-safe fixtures and
test-only legacy transfer helpers. The actual primary
[validation record](validation.md#uiu-session-01-prepared-project-target-preflight)
owns acceptance of this focused unit; source-only checks do not establish runtime
acceptance. Ordinary legacy rebind remains test-only compatibility coverage;
production Open uses the pre-admitted target path.

### UIU-SESSION-01 retained editor candidates and prepared transfer

The next concrete owners are the existing source, profile and measurement models:
their original input/catalog/decoded frame/JSON owners persist independently of
accepted Inputs and public scientific requests. Reuse existing Retained and
RetentionReservation; add no accounting framework or scientific API. Measure each
actual finite clone after validation, admit before persistent replacement, and
keep old/model/input/document charges together until their actual final drops.
REQ-UIU-03/06/07 and the still-open UIU-SESSION-01 own this continuation.

| Owning paths | Change and acceptance |
| --- | --- |
| EDIT `apps/qt/src/shell/retention.rs` and its existing tests | Consume an exclusively owned Retained into payload/reservation parts without releasing its charge. This concrete queue-to-GUI transfer preserves data-before-guard drop ordering. |
| EDIT `apps/qt/src/downstream/inputs/retention.rs`; ADD `retention/model_children.rs` and focused tests | Reuse actual JSON/profile/RxMER leaves; add current response/recipe children. One helper covers the maximum of the two actual editor storage layouts (queued Retained versus GUI model plus optional reservation), with checked fixed-root replacement. No guessed multiplier or private allocator layout. |
| EDIT `apps/qt/src/downstream/inputs/source_input/model.rs`, `profile_input/mod.rs`, `measurement_input/model.rs`; ADD their `retention.rs`/tests | Adapt frozen passive visitors from `6d20fa7a3a6b05398b766529531ef3e7fb0b70a1`, register actual owner metadata and admit concrete candidates. Apply reserves the editor before the existing complete Inputs/document transaction, then transfers its guard after success. Failed admission/publication preserves both. |
| EDIT `apps/qt/src/downstream/inputs/measurement_input/designer.rs`; ADD `designer/retention.rs`/tests | Count optional Designer Origin children already retained in measurement captures. Session/worker/export admission remains a separate concrete owner; passive Session metadata alone does not accept it. |
| EDIT `apps/qt/src/bridge/source_input.rs`, `profile_input.rs`, `measurement_input.rs`, `rxmer_designer.rs` | Preserve all existing invokable signatures and UI behavior. Cover begin/stage/edit/cancel/reload/Apply and Designer completion candidate replacement. Store a reservation after each model field; guard queued-to-GUI transfer without re-admission. Profile gets a thin QObject owner rather than aliasing a mutable scientific-input model. |
| EDIT `apps/qt/src/downstream/workbench/project_load.rs` and its tests | Admit all three actual editor candidates against the target before queue/publication, retain isolated Contexts and verify exact target ownership. Failed/cancelled/stale loads release candidates without changing accepted state/path; GUI take transfers existing guards. |

Integration lead owns shared leaves, profile/measurement adapters, project wiring
and all acceptance records. The isolated session agent owns only source model/
bridge/test paths; its [source-only handoff](agent-handoffs/source-editor-admission.md)
records the frozen proposal before primary validation. No concurrent edits to manifests, registrations, architecture
or checklist. Test actual independent spare capacities, every positive aggregate
dimension, old/new overlap, rejected Apply after editor admission, target transfer
under pressure, cancellation and final disposal. Preserve existing original-byte,
schema, scientific and legacy sweep assertions. Run changed editor/project/model
Rust and relevant Qt/QML, Clippy/format/boundary/traceability and fresh executable
checks. Reuse unchanged native wheel/reference for affected project compatibility;
full SESSION Level 2/milestone gates and remaining retained owners stay pending.

Read-only QA found that legacy New committed empty Inputs before its now-fallible
editor reloads. New therefore uses the same complete PreparedProject preflight
and guarded adoption as Open; `apps/qt/src/bridge/workbench_file.rs` publishes its
cleared Save path only after success. Empty editor transfers remain admitted
through GUI reload even under full counter pressure. Global appearance stays
unchanged (`dark: null` in New metadata). The existing downstream shell displays
New rejection through its file-feedback dialog. Tests retain old snapshot aliases
and leave exactly the actual empty accepted-State charge, reproducing the former
partial-commit condition without guessing editor sizes.

The untouched measurement default interval previously allocated before any guard.
`Options.interval_symbols` now borrows named static default text through internal
Cow storage; parsed/editor-owned text uses its actual String capacity. The existing
unsigned-decimal JSON and numerical default are unchanged. Untouched defaults own
no heap children; fixed QObject/Context infrastructure remains separately scoped.
`apps/qt/src/downstream/common/constants.rs` owns that default text and the current
native schema version shared by New/Save/recovery. Existing independent schema
and numerical fixtures retain their historical values.

`apps/qt/qml/downstream/configuration/source_input/SourceInputDialog.qml` now
handles an admitted-Begin error before replacing controls. Focused rendering/
interaction tests in `apps/qt/tests/tst_workbench.qml` preserve existing control
values and exercise both Source Begin and New rejection. This changes failure
handling through existing banners/dialogs, with no layout or theme change.

### UIU-SESSION-01 receiver registry storage

REQ-UIU-03/06/07 and the open UIU-SESSION-01 also cover retained QAM registry
slots/status strings and performance spare slots. Reuse the existing immutable
Snapshot and live Entry guards; no new scientific API or accounting framework.
Adapt the frozen source-only visitors/tests at
`546db799a36364369814f0abf288a0c3a823b460`; primary integration owns runtime evidence.
Its [source-only handoff](agent-handoffs/history-registry-retention.md) preserves
the original proposal and unexecuted-runtime boundary.

| Owner/file disposition | Current contract, reuse and acceptance |
| --- | --- |
| EDIT `apps/qt/src/downstream/instruments/receiver/qam_history.rs`; ADD `qam_history/registry_retention.rs`, its tests and `qam_history/registry_tests.rs` | ADAPT the actual-capacity visitor into a concrete Registry containing entries followed by their guard. Prepare and admit a finite initial/replacement catalog before changing the current one. Copy status/Arc handles only; existing Snapshot guards survive selected aliases. Clear drops slots before releasing their guard. Test actual spare capacity, rejected initial and replacement admission, old aliases and final disposal. |
| EDIT `apps/qt/src/downstream/instruments/receiver/performance.rs`; ADD `performance/registry_retention.rs` and its tests | REUSE each existing live Entry charge. Normalize the finite completed Vec through boxed-slice storage before retaining it, so capacity equals live length and no spare slot remains uncharged. Clear replaces the Vec rather than retaining empty capacity. Existing replacement does not grow the buffer; insertion normalizes while locked before publication. Test passive spare metadata and every runtime mutation's zero-spare invariant. |
| EDIT `apps/qt/src/downstream/workbench/session.rs` | Change only the QAM Store payload to its concrete Registry. The fixed Store/Context/Vec-handle graph remains a separate pending infrastructure owner; this unit does not exclude it or claim SESSION closure. |
| EDIT existing `qam_history/retention_tests.rs`, `performance/retention_tests.rs`, and necessary closed-loop fixtures | Preserve scientific/null/identity assertions. Include actual newly admitted initial registry charges in fixture accounting; no size estimates or weakened limit checks. |

Performance Begin prepares/adopts both complete registries before clearing either
old history. All remaining operations after admission are infallible data moves.
Temporary construction is finite under MAX_PROFILES; it is not an aggregate heap
or preparation-peak guarantee. Existing scientific entry admission, archive bounds,
query semantics and completed result ownership remain unchanged. Run affected
receiver/lifecycle tests, Clippy/format/boundary, relevant QML and current executable
probes; no Python rebuild is needed for unchanged science/API/schema/registration.
Full SESSION and later task acceptance remain separately pending.

### UIU-SESSION-01 retained Designer session copies

The existing GUI Designer Session and the independent copies sent to generation
and export workers are concrete retained input owners under REQ-UIU-03/06/07 and
UIU-SESSION-01. Reuse existing Retained/admit_model and the owner-local Session
visitor, including actual hash/origin/request capacities, original profile/catalog
and design children. No new public scientific API or memory framework is needed.
The [source-only handoff](agent-handoffs/designer-session-admission.md) records
preparation separately from primary consumer validation.

| Path/symbol and disposition | Current contract and focused acceptance |
| --- | --- |
| EDIT `apps/qt/src/downstream/inputs/measurement_input/designer.rs`, `designer/retention.rs`, its tests; ADD `designer/admission_tests.rs` | ADAPT frozen source proposal `06582379172bf2b708723c4f8ee49bc062a2505e`: enable complete passive Session metadata and actual candidate/clone admission. Source-only checks do not establish runtime acceptance. Preserve existing scientific bodies and independent numerical assertions. |
| EDIT `apps/qt/src/bridge/measurement_input.rs` | Store the actual GUI Session as Retained<Session>; replace it only after candidate admission. Reuse one typed admitted-copy helper for both actual worker consumers, measuring the finite clone rather than predicting its capacities. |
| EDIT `apps/qt/src/bridge/rxmer_designer.rs`, `rxmer_export.rs`; ADD `rxmer_designer/retention_tests.rs` | Begin admits before replacing the previous session. Preview borrows the same owner. Generation/export workers retain independently admitted Session inputs through their actual completion/destruction. Test rejected replacement/copy in each positive dimension, unchanged previous controls/profile/original metadata, live worker plus replaced GUI overlap, cancellation and final disposal. |
| EDIT `apps/qt/src/downstream/common/constants.rs` | Name the existing recipe, playback-selection and export-scope document ceilings with their separate UTF-8 byte purposes. Recipe and playback currently share a value but remain independent contracts. Values, boundary comparisons and existing diagnostics remain unchanged. |

Request/control/worker-envelope data, generated capture queues, export Prepared
products and retained QML copies remain separate pending owners. This unit covers
Session input copies only and does not claim complete worker or SESSION closure.
Export preparation keeps its old ready snapshot until the admitted copy and worker
are successfully prepared; a new input-admission failure must not discard it.
Keep existing error/metadata/invokable signatures and numerical/export behavior.
Run relevant Designer/editor/export consumers, Clippy/format/boundary, affected
QML and fresh executable probes. Reuse unchanged science/native compatibility;
full task/milestone acceptance remains pending.

### UIU-SESSION-01 retained export products and ready-state rollback

The next concrete owner is the existing immutable export Prepared product: frames,
recipe/provenance JSON, both format summaries, range error and diagnostic. It
persists in a worker completion queue, Ready state and save worker, independently
of the already admitted Designer Session. REQ-UIU-03/06/07 and the still-open
UIU-SESSION-01 own this focused continuation.
The [source-only product handoff](agent-handoffs/export-product-retention.md)
at `a0b33c70325e1070321f9dd75831fe12e306b977` is selectively adopted; its tests
remain unexecuted until primary validation below.

| Path/symbol and disposition | Contract and acceptance |
| --- | --- |
| EDIT `apps/qt/src/bridge/rxmer_export.rs`; ADD `rxmer_export/retention.rs`, its tests | ADAPT the frozen source-only actual-capacity visitor when ready. Admit full Prepared storage through existing primitives before queueing. One explicit Arc<Retained<Prepared>> permits immutable save-worker/previous-Ready aliases and spawn-failure recovery; its additional shared root is charged. No scientific API or generic memory framework. |
| ADD `apps/qt/src/bridge/rxmer_export/lifecycle.rs`; EDIT existing invokables | Move existing preparation/poll/save lifecycle bodies behind thin QObject calls without changing scientific/export algorithms or return keys. Keep previous Ready charged while new work runs; restore it on input/target admission rejection, cancellation, disconnected worker or spawn failure. Save borrows a shared Prepared rather than consuming the only accepted owner before spawn. |
| ADD `apps/qt/src/bridge/rxmer_export/admission_tests.rs` | Exercise actual Ready identity/frames/metadata through all positive input/product limits, deterministic spawn failure, cancellation/queue disposal and final aliases. Close the direct prior-Ready rejection/spawn coverage gap recorded by the Session-input unit. Use the existing worker spawn-seam pattern for deterministic failures; no OS resource exhaustion. |
| EDIT `apps/qt/src/downstream/common/constants.rs` | Name the single terminal export queue slot as a transport policy, not a scientific count. Retain separate bounded execution for finite backend builders. |

Only Prepared products and their aliases are accepted here. Saved completion
Value/error envelopes, worker request/control/closure/channel bookkeeping and
retained QML copies remain distinct pending owners; do not claim complete worker
or SESSION closure. Existing file publication remains in the public Rust PNM API,
with no destination overwrite or source mutation. Relevant Rust/Qt/QML, all-target
Clippy/format/boundary/traceability and fresh executable checks precede a focused
commit; unchanged scientific/native APIs need no wheel rebuild.

### UIU-SESSION-01 retained Designer capture batches

Generated Designer captures retain original bytes, full decoded frames and private
Origin/design records independently of the GUI Session and measurement model.
REQ-UIU-03/06/07 and the open UIU-SESSION-01 require their completion queue to retain
an admitted batch guard until disposal or successful target-model admission.
The [source-only handoff](agent-handoffs/designer-capture-batch-admission.md)
records the selectively reviewed preparation; primary runtime acceptance is in
the [capture-batch evidence](validation.md#uiu-session-01-retained-designer-capture-batches).

| Path/symbol and disposition | Contract and focused acceptance |
| --- | --- |
| EDIT `apps/qt/src/downstream/inputs/measurement_input/model.rs`, `model/retention.rs`; ADD `model/capture_admission_tests.rs` | Reuse the existing complete capture child visitor for actual Vec batches, counting the containing Vec once and requested slots. Admit through existing primitives after finite backend construction and before queueing. Original Model metadata/scientific bodies stay unchanged. Selectively adopt source preparation only after exact-parent/content review. |
| EDIT `apps/qt/src/bridge/rxmer_designer.rs`; ADD `rxmer_designer/capture_batch_tests.rs` | Worker terminals carry Retained<Vec<Capture>>. Cancel/stale/error paths dispose the batch safely. Successful staging transfers its existing reservation alongside metadata until the independently admitted GUI Model replaces the old one; no gap or premature release while target admission may still fail. Preserve existing selection remapping, complete-batch adoption and scientific assertions. |
| EDIT `apps/qt/src/downstream/common/constants.rs` | Name the single completed-capture queue slot independently of the equal-valued export terminal policy. Progress remains its existing atomic count. |

Test actual generated original/frame/Origin capacities, every positive batch
dimension, queued cancellation/stale/disposal, simultaneous source/target admission
and rejected target preserving accepted editor/input state. No science/API/schema
or generic framework changes. Work request/hash/options/control/error envelopes,
fixed infrastructure and retained QML remain separate pending owners. Relevant
Rust/Qt/QML, Clippy/format/boundary/traceability and fresh executable validation
precede the focused commit; unchanged native science needs no wheel rebuild.


### UIU-SESSION-01 retained Designer worker inputs

REQ-UIU-03/06/07 and the open UIU-SESSION-01 also cover independent parsed
requests and persistent selection snapshots retained by generation/export workers.
The [source-only child handoff](agent-handoffs/designer-worker-input-children.md)
records the independently prepared extraction, separate from primary runtime.
Reuse Request and Options child traversals from their existing Session/Model
owners; preserve scientific validation, error wording, metadata and accepted state.

| Path/symbol and disposition | Contract and focused acceptance |
| --- | --- |
| EDIT `downstream/inputs/measurement_input/designer.rs`, `designer/retention.rs`, `model.rs`, `model/retention.rs`; ADD focused child tests | Extract and forward app-private Request/Options child visitors; existing Session/Model accounting reuses them without double-counting fixed storage. Query actual capacities without cloning, serializing or constructing captures. Original numerical assertions remain. |
| EDIT `bridge/rxmer_designer.rs`; ADD focused worker-input ownership module/tests | Admit concrete parsed request and GUI work selection snapshot before thread publication. Count actual hashes/profile text/options and preserve guard lifetime through stale/cancel/disconnect/target adoption. Independently retained worker input survives GUI disposal until actual thread completion. |
| EDIT `bridge/rxmer_export/lifecycle.rs`; ADD concrete input ownership/tests | Admit parsed Request and fixed Selection together before spawning. Input rejection and spawn failure preserve prior Ready. Workers retain input guards until actual operation completion, including output construction/admission overlap. |

No new memory framework or scientific/public API. Terminal error/Saved Value
payloads, remaining control/closure/channel/fixed infrastructure and persistent QML
remain distinct pending owners. Execute affected Designer/export/editor Rust/QML,
Clippy/format/boundary and fresh executable checks; unchanged scientific/native
APIs need no wheel rebuild. Complete SESSION Level 2/milestone gates stay open.


### UIU-SESSION-01 retained worker error completions

REQ-UIU-03/06/07 and the open UIU-SESSION-01 require the actual Designer generation
and export queues to retain admitted error messages and optional help diagnostics.
These are concrete existing consumers: generation AdmissionError owns a String
and optional Box<Diagnostic> with four private Strings; export queues an owned
String. Existing successful batch/Prepared guards remain unchanged.
The [source-only error handoff](agent-handoffs/worker-error-metadata.md) records
private metadata preparation separately from primary queue/runtime acceptance.

| Path/symbol and disposition | Contract and focused acceptance |
| --- | --- |
| EDIT `downstream/help.rs`; ADD owner-local `help/retention.rs` and tests | Passively count actual AdmissionError fixed value, message capacity, optional Diagnostic box and all private text capacities. Exhaustively audit fixed diagnostic fields; no JSON reconstruction or scientific operation. |
| ADD concrete `bridge/worker_failure.rs`; EDIT `bridge/mod.rs` and named common constant | Use existing Retained/admit_model for detailed errors. If this error itself cannot be admitted, drop it before publication and send a payload-free rejection variant with a named static message. This is an actual bounded failure transport, not a new accounting framework. |
| EDIT `bridge/rxmer_designer.rs`, `rxmer_export.rs`, export lifecycle and affected tests | Admit errors before single-slot queue publication. Keep guards through cancel/stale/disconnect disposal and borrowed response construction, preserving detailed wording/diagnostics whenever admitted. Rejected replacement preserves accepted editor/Ready data. |

Test actual private capacities/passivity, queued/final-drop error lifetimes,
resource-pressure fallback, preserved original diagnostic rendering and existing
scientific/cancellation behavior. Fixed transport/control infrastructure, Saved
completion Value and persistent QML remain separate pending owners. Execute affected
Rust/QML, Clippy/format/boundary/traceability and fresh executable checks. Reuse
unchanged scientific/native evidence; complete SESSION gates remain open.


### UIU-SESSION-01 retained save-completion data

REQ-UIU-03/06/07 and the open UIU-SESSION-01 also cover queued export save replies:
path, summary or publication-error JSON remains independent of guarded Prepared.
Admit the actual completed Value through existing JSON children and Retained/model
layout helpers before queue publication. Keep its guard through reply construction;
normal saved/cancelled/error keys and values remain unchanged.

| Path/symbol and disposition | Contract and focused acceptance |
| --- | --- |
| ADD `bridge/rxmer_export/saved.rs`; EDIT Outcome and lifecycle | Concrete Detailed(Retained<Value>) or payload-free rejected-status variant. Actual fixed Value/JSON capacity and ordered entries are admitted; no invented shared-root demand. Rejected metadata drops before publication, retaining only a Copy status and the already charged Prepared alias. |
| EDIT `downstream/common/constants.rs` | Distinct static messages identify whether publication succeeded, was cancelled or failed when detailed status cannot fit. A successful file write must never be reported as failed merely because later status admission failed. |
| EDIT export save handler; ADD focused saved-completion tests | Add a narrow native-picker seam, preserving the existing production picker and public Rust save operation. Test actual JSON file publication, queued status, cancellation, existing-file rejection/no overwrite, and payload/entry admission rejection after successful publication. Source/Ready owners and output bytes remain preserved. |

The fixed fallback deliberately omits the normal saved/path pair when no path can
be retained. The existing dialog renders its precise static message without a
missing-path assignment or presentation rewrite. It reports completed publication
in text; no false failure or automatic re-export is introduced. The frontend's
normal fields remain unchanged whenever admitted. Temporary JSON/reply construction
continues under separate finite execution limits; fixed control/transport/closure
infrastructure and persistent QML remain pending. Execute relevant Rust/QML,
Clippy/format/boundary/traceability and fresh executable checks; no new scientific
API, binding/schema/dependency or memory framework. SESSION full gates stay open.


### UIU-SESSION-01 canonical fixed downstream Stores

REQ-UIU-03/06/07 and the open UIU-SESSION-01 require each Context's five shared
Store allocations to participate in its actual application budget. The existing
Store owns accepted inputs, latest result, QAM Registry, performance slots and
queued PreparedModels. Context clones must share these existing guards; detached
preparation needs separate Stores against the same canonical application host.

| Path/symbol and disposition | Contract and focused acceptance |
| --- | --- |
| EDIT `downstream/workbench/store.rs`, `session.rs`; ADD owner tests | Existing Store becomes Arc<Retained<Mutex<T>>> and admits exact fixed storage/one root before allocation. Add fallible Context::for_application; isolated/default preserve their intended independent/bootstrap construction. Partial construction rolls back, clone/final-drop lifetimes stay exact. Dynamic child payloads retain their existing separate owner guards. |
| EDIT `downstream/workbench/project_load.rs`, project file, profile/receiver/transmitter constructors | Detached preparation receives ApplicationSessions, never target Context, and creates separate canonical-budget Stores. Explicit-context construction avoids initializing unused worker-thread bootstrap hosts. Keep actual preflight/readiness and callback identity checks unchanged. Take prepared models before attaching the target Context to prevent a Store-to-model-to-same-Store cycle. |
| EDIT queued editor storage and affected test fixtures | Box the three already charged Retained editor values so fixed Store layout and editor wrappers occupy distinct actual storage. Preserve original adapter take/adopt imports and saved JSON. Construct fixtures against their intended host rather than replacing only Context.application; update measured fixed-owner baselines without weakening original science or final-alias assertions. |

Test exact byte/root rejection (zero ordered entries for fixed Stores), partial
construction rollback, source/target identity, independent detached stores, stale/
cancelled adoption and actual final alias disposal. Broaden to all affected Qt
Rust/project/lifecycle consumers and QML/native compatibility because construction
and shared ownership change. Preserve saved-workbench formats, numerical vectors,
completed checklist history and stable caches. Application session/gate/accounting
allocations, worker control/closure/transport and retained presentation remain
separate pending owners; complete SESSION Level 2/milestone gates stay open.

The isolated [source handoff](agent-handoffs/fixed-context-stores.md) preserves
its preparation-only boundary; primary execution is in the
[validation record](validation.md#uiu-session-01-canonical-fixed-downstream-stores).


### UIU-SESSION-01 canonical fixed application host

REQ-UIU-03/06/07 and the existing open SESSION aggregate-admission gate require
actual Accounting, DispatchGate and ApplicationSessions shared pointees to remain
charged through their final aliases. Existing readiness metadata alone covers
only its two inline Retained fields. Reuse the existing counter/reservation and
concrete owners; no new memory-accounting framework or scientific API.

| Path/symbol and disposition | Contract and focused acceptance |
| --- | --- |
| EDIT `shell/retention.rs`; ADD application baseline tests | Application-only fallible construction admits exact Mutex<Accounting> bytes/root before Arc allocation. Its used baseline lasts until the final counter alias disappears, without a strong reservation back to itself. Generic new/with_limits preserve their zero-baseline contract. Reuse the existing checked aggregate addition internally across actual owner fixtures; no new arithmetic implementation. |
| EDIT `shell/dispatch.rs`; ADD retained gate tests | Concrete GateStorage owns state before its optional reservation, admitted by the application-only factory. Generic Default stays available for pure fixtures. Clones and real RunLease keep fixed bytes/root alive after host destruction; control data drops before the guard. Preserve every dispatch/cancellation transition. |
| EDIT `shell/application.rs`; ADD concrete host tests | Shared Sessions becomes Arc<Retained<Mutex<Sessions>>>. Its complement subtracts the two already charged inline Retained readiness layouts using checked arithmetic, so total idle fixed bytes count each pointee once. Admit before Arc allocation and preserve readiness candidate rollback, known-terminal shutdown and original identities. |
| EDIT affected ownership fixtures only | Measure exact host/counter baselines following actual Store, host, queued lease and budget aliases. Preserve existing science, pressure/rejection, generation and final-drop assertions; generic counter tests stay unchanged. |

Test exact and one-below byte/root limits, zero ordered-entry demand, constructor
rollback, readiness replacement rejection, paused/cancelling host destruction,
real terminal/error/panic/spawn-failure disposal and final budget-only ownership.
Broaden to both direction/lifecycle consumers and all affected Qt Rust, QML,
boundary/Clippy/format/traceability and fresh executable probes. Reuse unchanged
scientific/Python artifacts, not unexecuted pass claims. Controls/closure/channel,
other retained metadata and persistent presentation remain separate owners;
SESSION full Level 2 and applicable milestone/final gates stay open.

The isolated [fixed-host preparation](agent-handoffs/fixed-host-owners.md) records
the source-only boundary. Primary integration owns actual execution and acceptance.


### UIU-SESSION-01 scientific control allocation metadata prerequisite

REQ-UIU-03/06/07 already require complete admitted scientific owners. The frozen
remaining-owner audit identifies exactly three existing Qt consumers whose public
controls lack allocation metadata: downstream worker Dispatch.active/executing
FullLoopControl, transmitter preview Job/worker TransmitterPreviewControl, and
upstream session/workbench UpstreamControl containing upstream RunControl.
Scalar snapshots cannot establish private allocation layouts. This is a concrete
consumer prerequisite; completed planning/FEC/API milestones remain closed.

Lead-owned acceptance files are ADD `crates/merlin/tests/control_footprints.rs`
for typed legacy/facade imports and passive lifecycle/alias behavior, and ADD
`crates/cli/examples/control_footprints.rs` for a separate-process Rust JSON
reference. The CLI crate already owns serde_json; this avoids adding dependencies
to the facade or either independent PHY. ADD native parity tests under
`bindings/python/tests/` after Rust acceptance, then update the existing operation
matrix and public API guides. No manifest or global registration changes.

| Additive Rust operation | Native Python parity | Scientific owner / disposition |
| --- | --- | --- |
| FullLoopControl::retained_footprint(&self) -> ApiResult<RetainedFootprint> | merlin.downstream.FullLoopControl.retained_footprint(self) -> merlin.common.RetainedFootprint | EDIT `crates/ofdm/src/api/full_loop/control.rs`; ADD owner tests. Root + actual Shared + common stop atomic, two roots/zero ordered entries. |
| TransmitterPreviewControl::retained_footprint(&self) -> ApiResult<RetainedFootprint> | merlin.downstream.TransmitterPreviewControl.retained_footprint(self) -> merlin.common.RetainedFootprint | EDIT `crates/ofdm/src/api/transmitter/control.rs`; ADD owner tests. Its own root/Shared layouts + common stop atomic, two roots/zero entries. |
| Upstream RunControl::retained_footprint(&self) -> PhyResult<RetainedFootprint> | merlin.upstream.RunControl.retained_footprint(self) -> merlin.common.RetainedFootprint | EDIT `crates/ofdma/src/simulation/control.rs`; ADD owner tests. Root + actual State, one root; optional common cancellation adds its atomic and one root. |

REUSE the existing hidden public RetainedPayload support trait at the common
`crates/phy-api/src/control.rs` data owner for its private cancellation child.
Neither PHY can implement a foreign trait on that foreign type, and a private
visitor cannot cross crate boundaries. Add its owner-local implementation and
tests, without a fourth inherent common operation or extra getters. Use exact
compiled layouts, exhaustive field audits, checked sums and existing structured
ResourceLimit mapping (upstream field retained_footprint). Keep lock lifetimes
short and separate; metadata never waits/notifies, claims a job, advances clocks,
starts work or mutates controls. Clone counts must not change reported quantities.
Existing scientific algorithms, outcomes, namespace aliases and imports remain.

Rust operations and owner tests land before native exposure. Then EDIT the three
existing native class owners in `bindings/python/src/full_loop/control.rs`,
`transmitter.rs` and `upstream/simulation.rs`, and their existing root/upstream
stubs. Reuse existing common quantity conversion/error mapping and importable
namespaces. Lead owns facade regression/direct-process Rust reference, installed
native parity, operation matrix, public guides and acceptance evidence. No Qt
adapter, registration, Cargo dependency or scientific mode changes in this unit.

Acceptance checks independent private-layout formulas and literal root counts,
optional-token delta, clone/Weak lifetime and unchanged idle/pause-request/cancel/
terminal observations. Preserve actual run/Busy/partial-result cases; compare
science independently of elapsed time. Native tests must execute a fresh wheel
containing these Rust/binding changes and compare direct Rust reference results,
owned immutable metadata and common-token behavior. Validate legacy/facade imports
and affected Qt/CLI/native consumers; do not claim untouched complete suites ran.
Commit this independently validated paired API extension before Qt adoption.
Outer application control/wrapper/closure/queue admission and pending transport/
QML policy are separate work; complete SESSION acceptance remains open.

The [source preparation handoff](agent-handoffs/control-retention-api.md) records
the frozen Rust/native commits and their source-only validation. Primary runtime
evidence belongs to the [integrated validation record](validation.md#uiu-session-01-scientific-control-allocation-metadata).


### UIU-SESSION-01 concrete scientific control admission

The accepted three-operation metadata prerequisite enables the next finite
REQ-UIU-03/06/07 owner unit. Admit the existing control graphs and concrete
directional cancellation adapters through Retained, RetentionReservation and the
canonical host budget. No new public API or generic accounting framework.

| Path and disposition | Ownership and acceptance |
| --- | --- |
| EDIT `apps/qt/src/downstream/simulation/closed_loop/worker.rs`, `closed_loop.rs`; ADD adjacent control tests | Admit the full DownstreamControl shared pointee before global begin. Each profile uses one Arc<Retained<FullLoopControl>> shared by Dispatch.active and execution, admitted before publishing active state or running science. Preserve requested stop/pause between profiles, completed prefixes and known-terminal lease handling. |
| EDIT `apps/qt/src/downstream/instruments/transmitter/preview.rs`; REUSE existing local retention owner; ADD adjacent tests | Share one admitted Arc<Retained<TransmitterPreviewControl>> between Job and worker. Reject before clearing accepted status or creating the channel/spawn; preserve old results and guarded execution aliases through GUI disposal. |
| EDIT `apps/qt/src/upstream/session.rs`, `upstream/workbench/mod.rs`; ADD adjacent tests | Admit UpstreamControl before global begin. Its existing Arc is shared by gate/controller/execution; data precedes its reservation. Start validates the actual host counter and rejects uncharged/foreign controls. Preserve optional common-stop semantics and old reports. |
| EDIT exact existing constructor/pressure fixtures where required | Keep scientific assertions and pressure boundaries; supply the intended real host budget and include newly admitted control storage in independent fixture expectations. Do not change generic fixed-counter baselines or previously accepted owner arithmetic. |

Primary execution also identified two existing request-owner fixtures in
`closed_loop/request_retention/tests.rs`: real-run pressure must leave exact
adapter/profile control headroom, and the acknowledged-pause ownership sum must
include those live controls. Preserve candidate rejection and final-drop assertions;
the cancellation floor includes the still-tracked adapter and its upper bound
allows the actual profile control until execution releases it. Reconcile current
ownership comments in `upstream/mod.rs` and `upstream/session.rs` without changing
scientific behavior. These are focused primary supplements to frozen preparation.

For a retained DS/preview control, subtract the scientific inline handle already
included in its public footprint from the final Retained wrapper layout, then
add the backend footprint and one new outer shared root. The complete graph has
three roots and zero ordered entries. DS adapter admission covers its final
pointee layout including its guard and inline Dispatch mutex; active scientific
controls have their own charge. US adapter admission similarly subtracts its
inline backend handle, adds that backend footprint and one outer shared root.
Do not charge Accounting, gate, Sessions or Context Stores again.

Validate independent layout formulas, exact/below byte/root limits, candidate
rejection before lease/science, old accepted result survival, per-profile failure
with completed prefixes, stop/pause races, genuine terminal/spawn/stale disposal
and final GUI/gate/execution alias release. Expand targeted tests to all affected
direction/lifecycle consumers and relevant Qt/QML checks. Native APIs and their
exact accepted wheel may be reused if their source remains unchanged. Record
source-only preparation separately from primary execution; parent SESSION stays
unchecked. Timing, outer worker/transport storage, errors/paths/help and QML remain
separate owners; channel/QML policy answers are not prerequisites for this unit.

The [frozen source handoff](agent-handoffs/control-owner-admission.md) records
source-only preparation from `62f98d5`. Primary
[integrated validation](validation.md#uiu-session-01-concrete-scientific-control-admission)
passes all ten new control cases and the affected existing consumers. The
[evidence receipt](session-control-owner-admission-evidence.json) preserves the
initial two fixture failures and exact corrections, actual commands/source hashes,
final executable identity and reused API evidence. No scientific/public API or
QML implementation changes were needed. Parent SESSION remains unchecked.

### UIU-SESSION-01 concrete run-timing admission

The next finite REQ-UIU-03/06/07 unit admits the existing downstream wall-clock
owner. `RunTiming` contains inline `Mutex<State>` and one owned
`Vec<(u8,u64)>`; its phase label is borrowed static text. REUSE the canonical
budget, Retained guard, existing clock/worker APIs and existing
`MAX_PROFILE_TIMINGS` alias to the receiver profile limit. Do not introduce
another accounting abstraction, scientific API or timing algorithm.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| EDIT `apps/qt/src/downstream/simulation/run_timing.rs`: RunTiming, State, construction/end_profile | One Arc shares clock state between GUI and execution; the existing capped vector can grow capacity. | Root and actual vector capacity need admission before retention. | ADAPT constructor to reserve full final guarded pointee plus exact existing maximum tuple capacity and one root. A concrete prepared timing token consumes that reservation at the original clock-start location. Allocate exact capacity by boxed-slice conversion, clear length, then retain unchanged bounded push behavior. Keep state before the final nonoptional guard. |
| EDIT `apps/qt/src/downstream/simulation/closed_loop.rs`: Model::start | Current timing is constructed after global begin and old presentation clearing. | Timing rejection must preserve accepted timing, science, revision and dispatch availability. | ADAPT: reserve timing before global begin or clearing accepted state; construct the clock from that reservation at its original Start location to preserve epoch. Share the same Arc with existing worker; release candidates on Busy/spawn rejection. |
| EDIT existing `closed_loop/request_retention/tests.rs`, `result_retention/tests.rs`, `result_retention/projection_tests.rs`, `worker/tests.rs`, `worker/control_retention_tests.rs` | Actual runtime/pressure/queue fixtures use real controls and original scientific assertions. | New timing quantities must be included in the intended live-owner budget. | ADAPT only constructors and exact arithmetic/lifetime expectations. Prepare timing before intentional control pressure. No unrelated assertion removal. |
| ADD adjacent `run_timing/retention_tests.rs` and `closed_loop/timing_retention_tests.rs` | Existing deterministic clock tests remain. | Prove admission, capacity and transactional real Start behavior. | SEPARATE focused tests for this concrete owner; no generic framework. |

Use checked multiplication/addition for `sizeof(RunTiming)` plus
`MAX_PROFILE_TIMINGS * sizeof((u8,u64))`; zero ordered entries and one shared root.
The fixed mutex/state/vector handle/guard are already inside the pointee and
must not be counted twice. Aliases share one charge through final GUI/execution
drop, including after a terminal message or GUI disposal. A Weak observer does
not retain payload bytes. Do not change the static-label, pause-inclusive wall
time, progress age, idempotent finish or capped profile-history semantics.

Acceptance covers independent compiled layout, exact/below byte/root limits,
actual vector capacity before/after all admitted pushes, overflow checks, final
aliases, unchanged deterministic clock labels/freeze, and real Start pressure
preserving old result/timing/configuration/token without a new lease. Exercise
actual paused/cancelled/stale/terminal consumers and only affected Qt/QML checks;
reuse unchanged scientific/native API evidence explicitly. Lead owns primary
adoption, execution, canonical records and commit. Outer Worker/Job/controller,
transport and other metadata/QML remain separate. No pending channel/QML policy
answer is required to admit this concrete timing payload.


The [frozen timing handoff](agent-handoffs/run-timing-admission.md) records
preparation-only commit `4988f19` from exact `ddec888`. Primary
[integrated validation](validation.md#uiu-session-01-concrete-run-timing-admission)
executes all six new owner/Model cases and affected existing consumers. The
[receipt](session-run-timing-admission-evidence.json) preserves exact commands,
source/executable identities and unchanged-consumer reuse. Read-only QA corrected
a stale-cancellation test race before primary execution without extending actual
production lifetime. Complete SESSION acceptance remains open.

### UIU-SESSION-01 admitted worker failure payloads

This finite REQ-UIU-03/06/07 unit covers queued failure payloads. REUSE the
existing `bridge/worker_failure.rs` owner, actual AdmissionError/Diagnostic
metadata, canonical budget and allocation-free fallbacks. Its current Designer
callers and wording remain compatible. No new scientific API, error interpretation
or channel framework is needed. Standard-channel private storage is a separate
pending policy/owner; this unit does not claim complete transport acceptance.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| EDIT `apps/qt/src/bridge/worker_failure.rs`; ADD `apps/qt/src/shared/worker_failure.rs`; EDIT `shared/mod.rs` | WorkerFailure already admits original message/private Diagnostic metadata or uses a static fallback. | Other application workers need the same payload contract without depending on bridge ownership or Designer-specific fallback text. | SHARE the one implementation through a private application module; retain a bridge re-export and existing tests/imports. Add consumer-selected borrowed fallback text while preserving the original Designer default. Detailed data uses a boxed retained pointee so future envelope slots do not recount its inline payload. |
| REUSE `apps/qt/src/downstream/help/retention.rs`, `inputs/retention.rs`, existing Designer/export consumers and tests | Actual error graph and retained-wrapper arithmetic are already implemented. | Relocation/boxing must preserve exact pointee accounting and old behavior. | REUSE, with focused existing tests; do not duplicate the walker or add another accounting abstraction. |
| EDIT `apps/qt/src/downstream/simulation/closed_loop/worker.rs`, `closed_loop.rs` and affected adjacent tests | Rejected carries an owned String; Finished selects only static literals; generic terminal errors are application-owned. | Queued dynamic failures need admission and static reasons need no allocation. | ADAPT Rejected/terminal failure payloads to the existing owner before send; borrow Finished literals. GUI/performance consumers read the same text/help without cloning guards. Later retained GUI copies remain separately accountable. |
| EDIT `apps/qt/src/downstream/instruments/transmitter/preview.rs` and affected adjacent tests | Worker returns owned error text through a bounded channel. | That error can remain queued after GUI disposal. | ADAPT to the same admitted failure owner before publication; preserve accepted source/results and cancellation/stale behavior. |
| EDIT `apps/qt/src/upstream/workbench/preparation.rs`, `workbench/mod.rs` and affected adjacent tests | Configuration/target preparation queues owned String errors; run preparation retains its original controller. | Queued errors need canonical-budget admission without losing the controller on failure. | ADAPT both result variants; pass the existing host budget explicitly for run preparation, and keep disconnected/cancelled diagnostics static. |
| EDIT `apps/qt/src/upstream/session.rs`, `upstream/tests.rs` and adjacent failure tests | Terminal ApiError can retain opaque serde_json/codec internals. Current Qt consumers use category and rendered text. | Qt cannot soundly measure those private error allocations. | SEPARATE a small application-only category/text projection before terminal publication. Preserve the public ErrorCode even if text admission falls back; drop the opaque error before enqueue. Scientific/native error APIs remain unchanged. |
| ADD focused shared/worker and adjacent direction failure tests where needed | Existing real Busy/error/panic/disconnection and old-result cases remain. | Prove the new queued ownership boundaries. | Exact/below error capacity and actual wrapper checks, static fallback/category preservation, genuine queued/stale/GUI-dropped/final-disposal cases; preserve original assertions and source bytes. |

Each detailed box charges the actual retained AdmissionError pointee and all
owned message/Diagnostic capacities once, using existing checked arithmetic.
Its enclosing enum/terminal/channel slot and later GUI copies are distinct
concrete owners; do not claim that moving a guard also charges an independent
copy. Borrowed static fallbacks retain no dynamic text. Preserve current bounded
upstream text policy and original Designer message/diagnostic behavior. Shared
failure relocation may widen only private application visibility, with no CXX-Qt
or QML registration change.

Lead owns primary adoption, canonical records and executed acceptance. Expand
focused tests across all changed Designer/export/downstream/preview/upstream
consumers, actual error/Busy/cancellation/partial/stale/terminal paths, relevant Qt
Clippy/format/boundary checks and current executable. Reuse unchanged scientific/
native and standalone QML evidence explicitly. Fixed outer runtime, closure/
channel storage, GUI metadata, paths/help and persistent QML remain separate;
parent SESSION and its complete Level 2/3 gates stay open.

The [primary receipt](session-worker-failure-admission-evidence.json) and
[validation](validation.md#uiu-session-01-admitted-worker-failure-payloads) record
127 distinct affected Qt Rust passes and six fresh desktop probes. Source-only
preparation at `325017c` is separate; its [handoff](agent-handoffs/worker-failure-admission.md)
retains the source-only validation boundary. Primary execution found one new stale fixture
that expected success from finishing an intentionally invalidated token. The
existing contract returns `StaleCallback` while releasing the lease; the corrected
fixture asserts both facts and passes alone. No production change was needed,
and the unchanged fifty downstream passes were preserved without another run.

### UIU-SESSION-01 offline help catalog lifetime

This independent REQ-UIU-03/07 owner unit removes the parsed help catalog's
permanent heap cache. The embedded JSON and FAQ are static program data;
`downstream/help.rs::catalog` currently retains a parsed Vec and owned strings
through a global OnceLock outside the application budget. Catalog lookup needs
no persistent mutable state or scientific calculation.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| EDIT `apps/qt/src/downstream/help.rs`: catalog/entry | Parse bounded bundled JSON once, then permanently retain Catalog/Entry heap data. Existing help tests establish exact messages, keys, FAQ links and numeric context. | The global cache has no canonical host lifetime or admission. | ADAPT to bounded on-demand owned parsing/lookup; drop Catalog/Entry after each call. Reuse the existing source-byte/schema/entry limits and every rendering/diagnostic operation. Preserve output and unavailable-help behavior. No new budget wrapper, public API, dependency or GUI registration. |
| ADD `apps/qt/src/downstream/help/catalog_tests.rs`; REUSE existing help and worker-failure tests | Private help rendering and existing retained Diagnostic metadata are separate owners. | Prove owned lookup isolation, unchanged detached output and source admission limits. | SEPARATE targeted catalog-lifetime/input tests; retain all original assertions and existing Diagnostic walker. |

The private catalog parse is temporary execution work, bounded by the existing
embedded source-byte and entry limits and the parser's existing finite recursion
limit. It is not retained between queries. Returned Diagnostic/message/JSON data
remain distinct consumers; their existing guards or pending GUI/presentation
ownership still apply. Do not count borrowed static source bytes as dynamic heap,
claim a total-heap/RSS bound or alter scientific validation to avoid help errors.
Removing the small cache exchanges repeated bounded parsing for eliminating one
global retained owner; no throughput or memory-performance measurement is claimed.

This unit is independent of the worker-failure source worktree: no AdmissionError,
Diagnostic field, retention walker, bridge/shared module or error type changes.
Lead owns these exact files and canonical records; the source agent owns its
non-overlapping failure unit. Run focused help/failure-consumer tests, Qt Clippy/
format/boundary checks and build/probes. Full scientific/native/QML/packaging gates
remain unchanged and are not required for identical private help outputs.


The [help catalog receipt](session-help-catalog-lifetime-evidence.json) and
[integrated validation](validation.md#uiu-session-01-offline-help-catalog-lifetime)
record the actual focused checks, strengthened exact valid-source boundary and
unchanged-output/consumer evidence. Read-only QA found no other concrete issue.
That help receipt predates the separately recorded worker-failure acceptance above.
Complete SESSION acceptance remains open.

### UIU-SESSION-01 original source-path ownership

This finite REQ-UIU-03/06/07 unit replaces the process-thread-global
`platform/input_file.rs::INPUT_PATHS` with one charged registry per canonical
ApplicationSessions host. Protection is the union of both direction owners;
successful downstream Open clears only downstream ownership. Current import
callers remain downstream; this introduces no upstream importer or UI behavior.
Preserve the existing distinct-path cap as one named shell domain limit.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| ADD `apps/qt/src/shell/input_paths.rs`, adjacent tests; EDIT `shell/mod.rs`, `shell/constants.rs` | Global BTreeSet tracks canonical originals until project adoption. | Path capacity, entry/container counts, host isolation and direction ownership need explicit admission. | SEPARATE one concrete registry using existing Retained/budget only. Each canonical path carries direction flags and its final guard in the same owning record; ordering/lookup depend only on the immutable path. Use BTreeSet records with interior flags under the registry mutex, or an equally explicit destruction-safe representation. |
| EDIT `apps/qt/src/shell/application.rs`, application tests/retention tests; REUSE existing counter/gate/Sessions | Clones share three guarded canonical roots. | Registry must have host lifetime without a second budget or source-path/global cycle. | ADAPT constructor with one shared guarded registry root, rollback and documented fourth-root accounting. Registry never owns Context/ApplicationSessions; its reservations retain only the existing counter. Preserve core three-root assertions/history while verifying expanded host totals. |
| EDIT `apps/qt/src/platform/input_file.rs`; REUSE bounded file reader/canonicalization | read_local registers every successful bounded read before editor adoption; project reads stay untracked. | Import registration and overwrite queries must use an explicit host/direction. | ADAPT internal Rust signatures; admit actual canonical PathBuf capacity before insertion, avoid duplicate charges and preserve existing read/protection behavior. Rejected registry admission leaves all prior records intact. |
| EDIT `bridge/source_input.rs`, `profile_input.rs`, `measurement_input.rs` | Existing GUI imports already carry Context and its host. | Pass fixed downstream ownership explicitly. | REUSE context, limits and all scientific/staging operations; no QML/CXX signature or file-content changes. |
| EDIT `downstream/workbench/project_file.rs`, `project_load.rs`, `bridge/workbench_file.rs` and exact adjacent fixtures | Save protects imported originals; successful Open currently clears the global set. | Host-wide Save protection and direction-local Open cleanup must agree. | ADAPT private project write/recovery calls to query the explicit host union; clear downstream flags only after successful guarded project adoption. Rejected/stale/cancelled Open never clears paths. Preserve schemas, self-contained originals, accepted Save paths and atomic write behavior. |
| EDIT existing `session/fixed_tests.rs`, project-file/load and WorkbenchFile editor fixtures; ADD focused path tests | Existing precise host/Stores/project tests remain. | New fixed registry baseline and meaningful source-protection coverage. | ADAPT exact measured allowances only; retain original assertions. Test actual canonical/symlink protection, duplicate/cap boundaries, both direction flags, independent hosts, rejection/rollback and final aliases. |

Charge the registry's actual final retained mutex pointee and one shared root
once. Every distinct record charges its actual complete record layout (including
its guard), PathBuf capacity and one ordered entry, with no extra shared root.
Do not count fixed registry/entry fields twice or depend on private BTreeMap
key/value drop ordering. Path data precedes its guard; flags never affect ordering.
Borrowed canonical lookup avoids another retained path copy. Private tree-node,
allocator and shared-owner bookkeeping retain the accepted exclusions, bounded
separately by the existing entry/root policies. No numeric limit is inferred from
another domain solely because its value matches.

Lead owns architecture/checklist/evidence and integrated tests/commits. Source
preparation may use exact accepted `cd01664` in the existing isolated worktree;
its paths do not overlap queued-failure source. Adoption must verify each actual
base file before importing either unit. Public scientific/Python and saved JSON
contracts remain unchanged. Expand tests across fixed host/Stores, actual
imports/project Save/Open/recovery, both direction isolation and original-input
survival; keep source-only checks separate. This does not accept pending loader
path/GUI copies, channel/QML policy, remaining runtime owners or complete SESSION.

The [source-only handoff](agent-handoffs/source-path-admission.md) records frozen
`ff66f8e`. The [primary receipt](session-source-path-admission-evidence.json) and
[validation](validation.md#uiu-session-01-original-source-path-ownership) record
all 359 current Qt Rust cases and six fresh desktop probes passing. The complete
Qt run is justified by the changed canonical host baseline, with no duplicate
scientific workspace, wheel or standalone QML run. Complete SESSION stays open.

### UIU-SESSION-01 persistent direction-model metadata

This finite REQ-UIU-03/06/07 unit covers the already inventoried direction-model
error/Diagnostic/options owners. Read-only QA confirms that reusing the existing
boxed `WorkerFailure` is safe and simpler than a new GUI accounting abstraction.
The outer Model owns its inline enum/Box handle; the independently allocated
retained error pointee and reachable actual capacities are charged exactly once.
No public scientific/native API, QML registration, saved schema or algorithm changes.

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| EDIT `apps/qt/src/shared/worker_failure.rs` and focused adjacent tests | Existing admitted boxed error/Diagnostic payload, borrowed fallback and actual graph walker. | GUI persistence and host transfer need the same concrete owner. | REUSE with only concrete private default, ownership-check and checked actual-copy/rebind operations as needed. No new generic accounting framework. |
| EDIT `downstream/simulation/closed_loop.rs`, `closed_loop/lifecycle.rs` and adjacent tests | Persistent error String and optional Diagnostic survive callbacks/configuration. Several message replacements intentionally retain old help. | Independent GUI copies currently lose queued/backend coverage after disposal. | ADAPT to the existing failure owner. Move queued data when semantics permit; otherwise admit actual message/help copies while their sources remain covered. Preserve old Diagnostic for message-only changes when admission fits. Reset with borrowed empty text. |
| EDIT `downstream/simulation/closed_loop/request.rs`, focused options metadata/tests | Current Options contains selected-profile Vec and optional Echoes path Vec; remaining current leaves are fixed. | Accepted options are independent copies of request/plan data. | REUSE actual-capacity vector helpers and checked addition for children only. Keep original Option/Options fixed fields in the outer Model; add a final child reservation. Audit all current variants/fields explicitly, measure actual clones and preserve JSON. |
| EDIT `downstream/workbench/project_load.rs`, target/admission fixtures and prepared-target lifecycle hooks | Prepared receiver assigns cloned options before target scientific preflight. | Options/error guards must belong to the exact target, including incomplete projects. | ADAPT target-budget preparation before queue/adoption. Check budget identity; cross-host copies admit on the destination before Context/generation swaps. Rejection preserves original options, request/report, host key and Save state. |
| EDIT `downstream/instruments/transmitter/preview.rs` and adjacent tests | GUI error persists independently of queued failure; completion/cancellation notices are static literals copied into String. | The copied error needs its own coverage; notice allocation is unnecessary. | REUSE WorkerFailure for errors, borrow static notices and empty resets. Preserve status/message precedence, original source/result/revision and target/rebind rejection behavior. |
| EDIT `upstream/workbench/mod.rs`, `presentation.rs`, preparation/tests as required | Persistent GUI error copies use the existing bounded character policy. | Queued/terminal source guards do not cover a later independent GUI String. | REUSE admitted failure owner, moving eligible preparation failures and admitting borrowed-terminal copies before disposal. Preserve bounded text, category at the original terminal, controller/report and lifecycle semantics. |
| EDIT `bridge/closed_loop.rs` and focused bridge tests | Apply retains a separate admission Diagnostic outside Model. | Model's error guard cannot cover this independent owner. | REUSE the same concrete failure owner for the actual Apply error/help; admission_help returns unchanged Diagnostic/null JSON. Clear with borrowed empty text. No qinvokable/signature changes. |

The lead-owned bridge row is independently implemented and validated while the
isolated source agent prepares the non-overlapping core rows. Its [receipt](session-apply-help-owner-evidence.json)
and [validation](validation.md#uiu-session-01-apply-help-owner) record 18 affected
Qt Rust passes and six fresh desktop probes. Explicit static-empty bridge
Default/reset preserves original construction without depending on the pending
core owner's Default helper. Tests exercise the actual Rust owner/helper; they
do not establish native invokable Apply-pressure or complete SESSION acceptance.

For normal status updates, rejected detail admission drops the unadmitted copy
and uses the existing static fallback without inventing help; completed science
still publishes and previous accepted results remain. Message-only updates retain
prior help when admitted; fallback has no Diagnostic. A project/host transfer is
stricter: failed target coverage returns an error before replacing source owners
or Context, rather than silently accepting a fallback. Clear must release owned
capacity, not call String::clear while releasing its guard. Options guards follow
their covered inline data; fixed Model layouts remain a separate existing owner.

Source preparation may continue only after freezing the original-path unit, on
a separate preserved branch in the existing session worktree. Lead keeps primary
adoption, canonical records, executed validation and acceptance commits. Verify
each base file before importing; expand focused validation across all affected
direction, bridge and actual project consumers. Test exact/spare/below capacity,
source/target overlap, rejection and final drop, static resets, identical normal
status/help JSON and rejected Apply/Open/host transfers. Do not alter scientific
assertions to accommodate the accounting changes. Other runtime/transport,
loader paths, persistent QML and pending policy answers remain outside this unit;
complete SESSION and its Level 2/3 gates stay open.

The frozen [source handoff](agent-handoffs/direction-model-metadata.md) and
[primary receipt](session-direction-model-metadata-evidence.json) distinguish
source-only preparation from current execution. [C1 validation](validation.md#uiu-session-01-persistent-direction-model-metadata)
records 369 distinct Qt Rust passes, six fresh desktop probes and preserved
initial compile/fixture failures. Original science/budgets/assertions remain.
Both policy questions are now resolved; the [frozen closure list](session-closure.md)
limits remaining required work to C2–C6 without expanding this metadata unit.

### UIU-SESSION-01 approved presentation and channel boundaries

The user's 2026-11-24 [approved clarification](retention-contract-decision.md#approved-presentation-and-channel-boundaries)
resolves both pending policy questions. Persistent QML uses explicit logical
UTF-16 bytes, array/object entries and open/retained-view counts; Rust keeps actual
capacities. Standard channels charge message data and explicit channel/slot
counts, excluding private synchronization bookkeeping. Reuse the existing bounded
transport and admission primitives. The finite remaining owner inventory is
unchanged; implementation and consumer validation are still required.

REQ-UIU-07 and UIU-SESSION-01 retain the same IDs and acceptance owners. Earlier
scoped receipts' pending-policy wording is historical, not a current blocker.
No prior test is rerun or credited by this documentation clarification, and the
parent remains unchecked until complete Level 2/3 gates pass.


### UIU-SESSION-01 frozen acceptance closure

The user's 2026-11-24 close-session directive freezes the [six-condition closure
list](session-closure.md). It maps only existing REQ-UIU-03/06/07 blockers to exact
owners, minimum fixes and acceptance tests. Current functional dispatch does not
need another subsystem. Persistent tabs/actions, new file/recovery delivery and
the final numeric audit remain their existing tasks. No new owner category,
public API or generic framework is authorized without a demonstrated existing
requirement failure. Finish C1–C6, accept SESSION with current evidence and
immediately continue UIU-SHELL-01.


The frozen closure's [C2 concrete layouts](session-closure.md#c2-concrete-fixed-layout-implementation)
now have [primary evidence](session-fixed-runtime-owners-evidence.json) and
[validation](validation.md#uiu-session-01-fixed-runtime-owners), distinct from the
[source-only handoff](agent-handoffs/fixed-runtime-owners.md). C3–C6 remain required;
no new category or scientific API expands the frozen scope.


### UIU-SESSION-01 C3 concrete worker transport

[C3's existing REQ-UIU-07 gap](session-closure.md#c3-concrete-transport-integration)
uses the existing application counter for explicit channel/slot populations,
actual closure storage and separately admitted message frames. The
[source handoff](agent-handoffs/worker-transport.md) is source-only preparation;
[current primary validation](validation.md#uiu-session-01-concrete-worker-transport)
now closes C3. Both independent PHY APIs and native
Python contracts retain their existing three scientific footprint quantities.

| Exact inspected owner | Existing contract / gap | Selected change |
| --- | --- | --- |
| `apps/qt/src/shell/{retention,worker}.rs` | Existing atomic budget and genuine terminal/lease lifecycle; concrete dispatch/frame populations were missing. | REUSE checked reservations and std bounded queues; ADAPT finite prepared dispatch. `transport.rs` measures the actual closure and admits each actual frame independently, including received/held messages. |
| DS `closed_loop.rs` / `closed_loop/worker.rs` | Frozen public requests and one profile event per selected profile plus sweep status. | Preflight before begin/history/status changes; keep original timing epoch, errors and partial results. Charge actual private profile-ID capacity. |
| Preview and upstream session/preparation | Existing guarded products/controllers and stale/cancel semantics. | Preserve their child guards; charge concrete task/control/recovery storage, return original lease/controller after rejection and forward received delivery guards through adoption. |
| `bridge/rxmer_designer.rs`, `bridge/rxmer_export{,/lifecycle,/saved}.rs` | Existing immutable admitted inputs and actual batch/Ready/save products. | Box-move separately charged inline roots; admit complete Work/Working complements, actual atomic pointees and one terminal before work. Keep old Ready on rejection. |
| `downstream/workbench/project_load.rs` | Existing captured target, single-flight cancellation and prepared editors. | Pass the canonical budget explicitly; admit the actual loader Job/closure/controls/terminal. Product/path metadata remains C4. |

`shared/worker_controls.rs` centralizes the existing cancellation, completed-count
and loader-stage atomic pointees. These are actual application data, distinct
from excluded private std synchronization. Named desktop policies bound 64 live
channels and 256 configured slots, independently. Per-frame permits avoid a
capacity-plus-one estimate; finite DS/US event inventories reserve all delivery
frames before science, and every held terminal retains its own guard. Requested
permit-buffer storage is admitted before allocation and retained as an exact
Box slice. Data drops before guards on normal return, rejection, cancellation,
unwind and never-started disposal. No generic mutable ownership framework or
scientific implementation is added.

The lead owns bridge/loader/global wiring, five-dimension fixture compatibility,
source adoption and runtime checks. The isolated source worktree changes only
its eighteen handoff paths. Source formatting is distinct from primary runtime
evidence. Complete affected Qt tests, relevant Clippy, fresh native probes and
API/traceability checks are required before C3 closes; C4–C6 remain required.


C3's [executed receipt](session-worker-transport-evidence.json) closes this existing
condition. Old fixture sums preserve all five application quantities. Pressure is
applied after actual transport admission through the existing prepared-worker
operation or minimal preview/upstream starter seams; original scientific target,
control and error gates still execute. Exact held-frame/endpoint ledgers and
actual task cleanup replace assumptions that terminal receipt implies closure
exit. No numerical expectation, tolerance or scientific behavior changes. C4–C6
remain required; standalone source checks and normal native probes do not complete
SESSION or its native Apply/help-pressure interaction.

### UIU-SESSION-01 C4 project paths and metadata

The remaining [C4](session-closure.md) gap is existing REQ-UIU-06/07 ownership,
without a schema or scientific change. Reuse `RetentionBudget`, `Retained`, actual
PathBuf capacity and `downstream/inputs/retention.rs::json_children`. A small
application-private `shared/project_path.rs` operation admits a boxed path for
three actual consumers; the original-source registry retains its distinct record
layout and protection semantics.

| Inspected owner | Existing contract / gap | Minimum disposition |
| --- | --- | --- |
| `downstream/workbench/project_load.rs::PreparedProject` | Admitted editors/science, but independent path, JSON metadata and containing complement lack guards. | ADAPT with separate boxed path/metadata guards and a final fixed witness; do not revisit existing guarded graphs. Validate all target witnesses before the existing accepted-state transaction. Move admitted path and metadata out on success. Build New's existing metadata overrides before admission. |
| `bridge/workbench_file.rs` | Existing Open/New/Save actions and atomic public-project writer; path clones lose coverage. | ADAPT Open by moving the admitted path. Borrow an existing Save path; admit a normalized Save As/first-Save candidate before writing, preserving the previous path until success. Keep existing output/schema and native picker behavior. |
| `platform/startup.rs`, one `main.rs` callsite | Pure parsed CLI path is currently stored before loader capture without admission. | REUSE parser; convert to an admitted runtime request using the same main-thread Context budget before storing it. Transfer the original guard to the loader; create no second application host. |

The source agent owns only the loader/project owner, adjacent new C4 tests and a
[source handoff](agent-handoffs/project-metadata.md) in its isolated worktree.
The lead owns the shared path helper,
startup/main, file bridge and old-fixture compatibility, canonical records and
all integrated checks. Loader transport/errors already have C3 guards and must
not be charged a second time. Saved `Project::write`, public Rust/Python inputs,
original protection and atomic rename semantics remain unchanged.

Acceptance covers actual spare path/JSON capacities, byte/entry and foreign-host
rejection before mutation/write, queued/stale/cancelled disposal, same-host move
under pressure, failed Open preservation, Save path rejection before filesystem
changes and final release. Run affected project/bridge/startup tests, relevant
Clippy/format/API boundary and current native probes. Validate saved-input Rust/
installed-Python compatibility using unchanged cached artifacts; no fresh wheel
is necessary without a linked-code/package change. C5/C6 remain separate.


C4's [primary validation](validation.md#uiu-session-01-project-paths-and-metadata)
and [executed receipt](session-project-metadata-evidence.json) now close this
owner. The fixture-only raw prepare signature remains under `cfg(test)`; production
moves admitted paths through the private guarded builder. One previous fixture
ledger now explicitly observes prepared-inline disposal and returned-metadata
retention before final metadata disposal. Source-only checks are separate from
primary acceptance. C5/C6 remain required; no new requirement or task is created.

### UIU-SESSION-01 C5 existing persistent presentation owners

This implements only frozen C5 / REQ-UIU-07. Reuse the existing checked
RetentionBudget/reservations and the [prepared logical counting helper](agent-handoffs/presentation-footprint.md).
Scientific RetainedFootprint and its three quantities stay unchanged. Add the
four approved presentation quantities to the existing application counter:
logical UTF-16 bytes, array/object entries, retained views and open views.
Keep them separate from actual Rust capacities and standard-channel populations.
No new accounting framework or scientific/public API is required.

At C5 acceptance, named desktop admission policies were 64 MiB logical UTF-16,
1,000,000 logical entries, 128 retained views and 32 open views. The later
[SHELL adjustment](#uiu-shell-01-persistent-tabs-and-scoped-actions) accommodates
both actual direction trees. These are independent application
limits, not PHY or Qt allocation/RSS claims. The existing counter checks every
quantity atomically, and its actual layout remains covered by the original host
witness. Tests use independent explicit ceilings rather than repeating policy.

The lead owns a thin application-private presentation QObject and shared QML
component. Each existing consumer measures a plain candidate before persistent
assignment, reserves it while the old value remains held, then releases the old
witness after replacement. Coupled fields form one coherent candidate. Plot
bindings share the parent owner; independent selector/filter/row wrappers and
editable text need their existing presentation complements. Hidden/closed data
keeps its witness. Open slots are admitted before opening and released on close;
retained views/data release only on actual clear/disposal. Rejected replacement
keeps old data and uses bounded static feedback. Runtime completion/control
transitions still execute if optional details cannot be retained.

An isolated source unit owns only application counter/policy additions and adjacent
focused tests; it must not change bridges, global registration, Cargo, QML,
canonical documents or scientific crates. The lead owns primary adoption, old
fixture compatibility, actual consumer wiring and runtime acceptance. This
prerequisite alone cannot close C5. Current consumers remain the finite inventory
in session-closure.md; no new category or requirement expands it.

C5 source prerequisites are [counter](agent-handoffs/presentation-counter.md)
and [measurement viewport/tree](agent-handoffs/measurement-presentation.md).
Their isolated formatting/body checks do not establish runtime acceptance.
The lead integrates the actual native factory and the complete frozen consumer
inventory; primary runtime evidence, rather than source handoffs, closes C5.
The additional isolated source records are [logical helper](agent-handoffs/presentation-footprint.md),
[measurement dialogs](agent-handoffs/measurement-dialog-presentation.md),
[TX dialogs](agent-handoffs/transmitter-presentation.md),
[RX dialogs](agent-handoffs/receiver-presentation.md),
[channel dialogs](agent-handoffs/channel-presentation.md) and
[configuration dialogs](agent-handoffs/configuration-presentation.md).
The [primary validation](validation.md#uiu-session-01-persistent-presentation-admission)
owns executed results and failed-attempt history; C6 remains separate.

Editable text uses a conservative logical UTF-16 complement from each actual Qt
TextField.maximumLength before enabling/populating its dynamic text. A separate
instance of the same native view adapter keeps that finite complement until owner
disposal, independent of DTO clear/hide. This conservatively uses another retained
view slot; it creates no new counter or resource category. Validators/limits and
Rust diagnostics remain unchanged. Tests verify actual Qt programmatic/editing
length behavior, failed admission and independent text/data lifetime.

The combined C5 wiring keeps native draft synchronization separate from optional
presentation admission. A rejected native staging result cannot be repaired by
accepting cached options: indexed actions and previews wait for a real metadata
reload. Spectrum parents publish tap/state together; proxies read the admitted
bundle and automatic child queries wait for the related QML notifications.
Rejected timing/measurement selections restore their admitted context. Four
pre-existing workbench control booleans may update in place during detail
rejection because they add no bytes or entries; native lifecycle stays live.
Save reads current backend options transiently rather than persisting an older
optional display snapshot. Dialog guards and lifecycle connections are explicit
properties to avoid Qt's eager implicit contentItem construction.

For the existing C5 native integration/visual gate, add one bounded, opt-in
`--session-presentation-probe` mode to the current executable. It loads a fixed
in-repository QML probe through the existing module and actual native factory;
normal startup does not instantiate it or change resource policies. The probe
prepares the existing receiver editor, fills the approved logical-byte budget
while its old state stays held, invokes its actual Apply action, checks terminal
completion plus visible Help rejection, then clears pressure and checks recovery.
A bounded search derives filler capacity from the native limits and the existing
UTF-16 helper. Capture the real dark VM view to a fresh ignored evidence path.
This supplies the missing native consumer/registration evidence and user-guide
visual, not a scientific API, general scripting interface or new accounting
category. C6 still requires the complete existing session/milestone gates.

The [isolated native probe handoff](agent-handoffs/presentation-native-probe.md)
records source preparation only; primary registration and actual VM execution
provide acceptance. The launcher requires a fresh ignored screenshot path.


### UIU-SESSION-01 final closure

The [C6 integrated receipt](session-acceptance-evidence.json) and
[validation](validation.md#uiu-session-01-integrated-acceptance) close the frozen
C1–C6 list with unchanged scientific/compatibility contracts. Independent native
direction owners, serialized lifecycle and approved resource contracts now pass
the complete applicable acceptance gates. Source-only preparation and historical
frontier descriptions do not override this current result. Persistent visible
tabs and action routing remain UIU-SHELL-01; no additional SESSION infrastructure
is authorized by this closure.

### UIU-SHELL-01 persistent tabs and scoped actions

SESSION is accepted at `50339f01e2c0829350ac39d4be0fe893eee74da6`.
This implements the existing [REQ-UIU-05/06/08/25](../../../requirements.md#req-uiu-05)
and [UIU-SHELL-01](../../checklist.md#task-uiu-shell-01), without reopening session,
scientific API or FEC work. Reuse the reviewed persistent-tab source and existing
native direction controls. Both view instances remain alive when hidden; the
canonical application owns selection, direction keys, dirty flags and dispatch.
Qt never treats a display/result refresh token as session identity.

| Disposition | Owner/file | Responsibility and acceptance |
| --- | --- | --- |
| Edit | `apps/qt/src/shell/application.rs` | Expose the existing selected-direction cursor; selection must not configure, cancel or release another direction. |
| Add | `apps/qt/src/shell/presentation.rs` | Bounded scalar projection of existing canonical keys, dirty flags and active phase, plus exact selected-direction identity validation. No scientific readiness or results are inferred. Test unconfigured identities, selection during run/pause/cancel and stale action rejection. |
| Add | `apps/qt/src/bridge/direction_shell.rs` | Thin `DirectionShell` QObject returns that projection, accepts two direction tags and validates unchanged opaque identities. Retains existing host handles only; no scientific owner or parallel interface. |
| Reuse/adapt | `apps/qt/qml/shell/directions/PersistentDirectionTabs.qml`; `apps/qt/tests/shell/` | Adopt only the three prepared component/fixture/test paths from `93dd675`; preserve persistent Loader ownership, selected-source checks and keyboard/focus assertions. Admit dynamic host display via the accepted PresentationOwner. Existing source handoff remains historical. |
| Edit | `apps/qt/src/{shell,bridge}/mod.rs`; `apps/qt/build.rs`; `apps/qt/qml/Main.qml` | Root-only registration and composition. Poll both native adapters independently of tab visibility; route selected actions after canonical identity validation. Keep File/Settings/About/update infrastructure shared. |
| Edit | `apps/qt/qml/downstream/DownstreamWorkbench.qml` | Reuse working downstream editors/instruments; make internal title/actions optional for shared-shell composition, retaining standalone defaults and current tests. |
| Reuse later | Approved isolated `qml/upstream/` source | Source-only preparation proceeds independently. Startup/editing/scientific instruments retain their own US task acceptance; the shell must not fabricate their backend observations or readiness. |

The finite implementation order is canonical scalar identity/selection projection,
then actual persistent-tab composition/action routing, then complete applicable
SHELL acceptance and current native visual. Focused units run targeted Qt tests,
format/Clippy/boundary and traceability; native registration/layout expands to a
fresh optimized build and real launch/tab-switch checks. Full unchanged backend
and wheel evidence is reused after exact source verification. Native upstream
configuration/instruments, unified file/recovery adoption and final numeric audit
retain their existing delivery tasks and acceptance; no new requirement or
memory mechanism is introduced. The
[SHELL receipt](shell-acceptance-evidence.json) and
[primary validation](validation.md#uiu-shell-01-persistent-tabs-and-scoped-actions)
record acceptance of this implementation; upstream editing, project/recovery
and instrument acceptance remain separate.

The shell's concrete presentation owners are Add
`apps/qt/qml/shell/directions/DirectionShellHost.qml` (admitted scalar metadata,
independent upstream polling and exact-origin action routing) and Add
`DirectionRunControls.qml` in that directory (shared presentation-only controls
with bounded derived text). Their focused QML fixtures/tests live under
`apps/qt/tests/shell/`. The current downstream action row/header retain their
standalone defaults and become optional only in Main's shared composition.
Upstream Configure initially routes to the approved UCD drill-in; editing and
scientific observation bindings remain the explicit US task gates. File actions
remain available only for the direction whose native file adapter is implemented;
an upstream selection must never fall through into downstream Save/Open/New.
Unified project/recovery delivery retains UIU-FILE-01/UIU-RECOVERY-01.


Edit `shared/theme/Theme.qml` for one named session-poll cadence. Main uses one
Timer for both adapters, and the downstream standalone Timer is optional rather
than duplicated. Edit the existing opt-in `validation/SessionPresentationProbe.qml`
only to accept a caller-owned capture Item and find the shared footer notice;
its accepted Apply/Help checks remain unchanged. The shell capture includes tabs
and controls while both native direction owners remain independent.
The [isolated upstream source receipt](agent-handoffs/upstream-presentation-integration.md)
is reviewed preparation; actual Main creation and task acceptance remain primary.

The focused shell review identified three compatibility corrections within this
task: retry rejected footer text on the existing host poll cadence after pressure
clears; retain the existing ContextualStatus diagnostic/F1 and progress-age help;
and observe the preserved void downstream Start signature through its actual
polled busy state or changed result token with real run timing. Result tokens are
only run-refresh evidence, never session identities. Rejected Start keeps its
original native reason. Tests use a void downstream fixture, including rejection
and completion before polling, rather than a string-returning substitute.

The actual linked build found a pinned qt-build-utils 0.7.3 basename collision
between the two VerificationDialog files. Rename only the incoming upstream type
to UpstreamVerificationDialog and update its owning reference/registration.
Build registration rejects duplicate cache basenames before compilation; retain
all compiled dependencies and caches. The source-only adoption receipt preserves
the original names/hashes; final integration evidence records this narrow change.

Add `qml/validation/DirectionShellProbe.qml` as one fixed opt-in native acceptance
consumer; Main/build registration remain root-owned. The isolated source hash is
`74b3aaca68c9f3f2466a94219bcef95e731528a31aa52882e3087dc9cc6da082`.
It uses actual Main menu/control/tab routes and native keys, acknowledged paused
lease, exact preserved options, dark images and terminal cancellation. No general
loader or scientific API is introduced. The
[debug inventory](../../debugging.md#direction-shell-probe) documents invocation
and evidence requirements. Source syntax alone does not establish native success.

Actual two-canvas startup saturated the existing 128 retained-view slots:
the diagnostic counter showed 128 retained views, zero open views, 6,501,756
logical bytes and 8,144,194 Rust payload bytes. Workbench Open completed, but the
shared footer and a new receiver window could not obtain view admission.
The upstream tree alone adds 21 existing owners before any drill-in opens.
Change only `src/shell/constants.rs::MAX_SESSION_RETAINED_VIEWS` to 256 so both
persistent trees can keep their original guarded data. Existing byte, entry,
root, transport, open-view and scientific limits remain unchanged. This resolves
REQ-UIU-05/08 under the existing REQ-UIU-07 contract; it adds no accounting
category, mechanism or scientific API and does not reopen SESSION acceptance.
Keep exact-ceiling rejection tests and actual two-canvas/native-open acceptance.
Temporary private counter logging is removed before the accepted build.

The SHELL implementation adopts only the thirteen upstream startup/drill-in QML
files and startup test. ResourceGridDialog/View, its test and two fixtures stay
preserved at `aaddc5ba61698f7b346116484b5d6be4a24fe57a` in the isolated upstream
worktree for UIU-US-04. Their earlier primary fixture run remains recorded as
source preparation, not shipped grid functionality or task acceptance.

The final primary executable passes actual persistent-child, canonical-cursor,
idle/running/paused switching, shared Settings/About, Configure, foreign control
and File rejection, and genuine downstream terminal checks. Its pressure probe
also preserves accepted receiver Apply and Help recovery with both view trees
retained. Complete affected Qt Rust/QML, boundary, packaging-resource and
documentation gates are recorded in the receipt. Unchanged backend/native Python
milestone evidence is explicitly reused after all 852 relevant source files
match the accepted SESSION commit. No new wheel or installed-platform claim is
made. Continue UIU-US-01 with the existing approved layout and configuration
contract, without reopening SESSION or backend planning.
