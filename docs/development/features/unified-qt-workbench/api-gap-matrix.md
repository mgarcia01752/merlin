# Public API gates for Qt

Status: the public backend gates are accepted: [API-01 admission](validation.md#uiu-api-01-public-admission-accepted),
[API-02 controlled runs/observations](validation.md#uiu-api-02-public-full-loop-and-observations-accepted),
[API-03 resource grids](validation.md#uiu-api-03-idle-upstream-resource-grid-accepted),
[API-04 upstream instruments](validation.md#uiu-api-04-upstream-instrument-backend-accepted)
and [API-05 projects](validation.md#uiu-api-05-project-backend-accepted).
Qt consumer adoption and final runtime acceptance remain their official tasks;
partial-unit statements below retain their historical stage where identified.
The [canonical API boundary](../../../architecture.md#unified-qt-public-api-boundary)
governs both direction adapters. Qt uses `merlin::downstream`,
`merlin::upstream` and genuinely compatible contracts under `merlin::common`.
Preserve `docsis_ofdm_fec::api::*`, both independent PHY crates, all accepted
facade exports and Python `merlin.Simulation`. One Python distribution retains
real importable `merlin.downstream` and `merlin.upstream` packages.

## Dependency mapping

```text
QML → src/bridge/* → direction Qt adapters → merlin facade
  facade downstream → docsis-ofdm-fec::api → existing DS scientific owners
  facade upstream   → docsis-ofdma::api    → existing US scientific owners
  facade common     → merlin-api          → no PHY/application dependencies
  facade native-input (implemented optional feature) → pnm-input
      pnm-input → both independent PHY APIs + pinned native PNM decoder
PHY crates → common API/DSP/FEC leaves
```

The selected facade remains re-exports/feature wiring only. Native file/project
implementation stays in Apache-2.0 `pnm-input`, with small new typed operations in
the respective PHY API where needed. An optional `native-input` facade feature
exposes admitted capture/project entry points under direction namespaces. The
shared direction-tagged project envelope `merlin::common::Project`
is an implemented facade re-export of the adapter-owned type, never an implementation in the
`merlin-api` leaf. This avoids a leaf→PHY cycle. The optional adapter already
depends on both PHYs; enabling it is not a claim of direction-only decoding.
Base facade direction features and independent PHY usability remain test gates.

Qt's direct `docsis-ofdm-fec`, `pnm` and `pnm-input` scientific admissions are
migration debt, not final architecture exceptions. Final scientific data access
uses the facade. CXX-Qt/CXX, Qt native style/pickers, serde transport, logging,
update HTTP, package handoff and presentation utilities remain narrowly scoped
infrastructure dependencies. Their code cannot decode or simulate a PHY as an
infrastructure workaround.

## Operation and parity matrix

| Current Qt call/data and inspected scientific owner | Existing accepted API / exact gap | Proposed public operation and owner | Rust/Python and acceptance gate |
| --- | --- | --- | --- |
| [Input models](../../../../apps/qt/src/downstream/inputs), native capture parsing and [session](../../../../apps/qt/src/downstream/workbench/session.rs) | Accepted typed DS inputs, original/sequence/Designer/profile/channel/idle-TX operations and native CaptureLoader are available; current Qt consumers still use legacy owners | UIU-API-01 accepted: existing typed inputs and bounded public operations under `crates/ofdm/src/api/`, plus curated native-input facade re-exports. Partial editor drafts remain separate from runnable accepted requests. | Same invalid-input/error/units semantics in Rust and native Python; no source advance on preview, original bytes/hash/grid preserved, failed Apply atomic; existing DS editor fixtures and saved-workbench cases. |
| Whole-run FEC recovery chart, existing Qt subtraction over actual FecCounters | Decoder outcome counters already exist; the source-verified recovery projection was still Qt-owned | UIU-DS-01 additive prerequisite: `FullLoopResult::fec_recovery() -> FullLoopFecRecovery` delegates constant-size accounting to simulation/channel/results; generated, recovered, rejected, mismatched and unrecovered counts. | Paired Rust/native complete, cancelled, empty and immutable-result checks plus the independent five-word classification oracle; [exact design](design.md#uiu-ds-01-source-verified-fec-recovery-prerequisite). Commit this prerequisite before changing the frozen Qt FEC consumer. |
| Finite source editor, native chooser and accepted worker snapshot | SourceInput owns exact bytes/fill admission; public limits, finite counts/policy and shared-buffer replacement are missing | UIU-DS-01 prerequisite: SourceInput.file_limits/file_info/with_fill reuse existing FilePayload bounds/accounting/immutable fill replacement, with typed SourceFileLimits/Info. | [Exact owner/design](design.md#uiu-ds-01-source-file-admission-metadata-prerequisite), independent limits/partial-byte/empty/source-ownership cases and installed native/direct-process parity before Qt adoption. |
| Original capture view, `scenario::measurement_view` | Existing bounded projection/grouping owner reused through the high-level API | UIU-API-01: implemented `RxMer::preview(MeasurementViewport)` and `ChannelEstimate::preview(MeasurementViewport, ChannelResponseMetric)`; owned `MeasurementPreview` carries view/grid/provenance/projection. | [Rust/Python parity](../../../user/api/input-admission.md#original-measurement-previews), [unit evidence](validation.md#uiu-api-01-original-measurement-preview-unit); original H/RxMER only. Sequence, Designer, channel and idle TX units have their own accepted evidence below. |
| Ordered measured collection, `MeasurementPlayback` | Existing power-domain modes and physical response admission reused through the high-level API | UIU-API-01: implemented `MeasurementSequence::new`, `info`, `capture`, `capacity`, `at`, `preview` and `TransmissionConfiguration::check_response`; typed options retain exact order, epoch tolerance and resource limits. | [Rust/Python parity](../../../user/api/input-admission.md#measured-sequence-admission-and-preview), [unit evidence](validation.md#uiu-api-01-measured-sequence-unit); actual contributor weights/provenance, power/clock literals, rejection and owned concurrent queries. Legacy project timeline support remains UIU-API-05. |
| Native saved Designer evaluator and `RxmerRecipe` | Typed immutable design operations reuse original baseline materialization; ordinary/derived source kind is publicly observable | UIU-API-01: `inputs::RxMerDesign` flat/imported/from-bytes/replacement/materialization with bounded positions, exact identity and curated recipe configurations; no scientific facade implementation. | [Rust/Python parity](../../../user/api/input-admission.md#immutable-rxmer-designer-inputs), [unit evidence](validation.md#uiu-api-01-designer-admission-unit); saved bytes/masks, independent tilt endpoints, noncompounding replacements, version/nesting/random/resource rejection and actual WB FEC regression. |
| Original compact profile document and quantitative inspector | Read-only complete byte-ID catalog preserves encoded schemes; scientific summaries/grouping reuse original assignment/Shannon/useful-rate owners | UIU-API-01: implemented `ProfileCapture`, `ProfileViewRequest`, bounded schemes/inspect, native loader and Dpd summary/view; legacy ProfileId behavior remains unchanged. | [Rust/Python contract](../../../user/api/profile-inspection.md), [unit evidence](validation.md#uiu-api-01-profile-inspection-unit); code/role/coordinate/rate/Shannon literals, binary/JSON equivalence, bounds and immutable source provenance. Complete raw/checked-ID admission and sweep compatibility now have API-02 acceptance below; Qt adoption remains DS-01. |
| Configured transfer, existing channel composition/loss/power/filter owners | Immutable settings and original-grid effective response reuse existing physical/epoch/coverage and composition admission | UIU-API-01: implemented `ChannelConfiguration::new`, `from_json`, `power_reference`, `preview` and `preview_capture`; no stream/source state, original H stays separate. | [Rust/Python contract](../../../user/api/channel-preview.md), [unit evidence](validation.md#uiu-api-01-channel-preview-unit); literal gains/electrical scale/echo taps, excluded grids, raw IDs, bounds and invalid-composition/identity rejection. |
| [Closed-loop worker](../../../../apps/qt/src/downstream/simulation/closed_loop.rs), `run_channel_controlled`, test-point observer and `Options` | DS public `Simulation::prepare_full_loop` admits immutable single-profile requests and acknowledged pause/cancel/progress with owned actual partial reports; opt-in typed stage, passive numerical and retained-message queries reuse existing owners. Calibrated channel spectra and scalar amplifier-boundary observations now have focused acceptance; complete catalog/sweep parity now has backend acceptance below | UIU-API-02 request/control unit implemented and [validated](validation.md#uiu-api-02-controlled-request-unit). Reuse existing job/measurement/pause owners; old RunControl/nominal types stay compatible. [Raw retained-stage ownership/query](validation.md#uiu-api-02-retained-stage-unit), passive numerical and retained-message queries have focused evidence. Calibrated channel spectra and scalar amplifier-boundary queries pass focused acceptance below; catalog/sweep parity and final backend acceptance pass below. | Idle/running/paused/cancelled/failed/completed transitions, safe boundaries, bounded time/memory, completed FEC trials retained, exact old configured outputs, native Python pause/cancel/progress/result parity. |
| [Receiver views](../../../../apps/qt/src/downstream/instruments/receiver/views.rs), [selection](../../../../apps/qt/src/downstream/instruments/receiver/selection.rs), [diagnostics](../../../../apps/qt/src/downstream/instruments/receiver/diagnostics.rs), [spectrum](../../../../apps/qt/src/downstream/instruments/spectrum/mod.rs) | Legacy Qt calls still calculate scientific views; public passive PSD/response/LLR, selected-cell QAM/MER/decisions and reference/soft queries reuse existing owners | UIU-API-02: typed immutable observations and bounded queries; [cell contract](../../../user/api/full-loop-cells.md) keeps all-selected measurements independent of display caps; [slicer views](../../../user/api/slicer-views.md) supply original references, finite regions and actual likelihood summaries. Calibrated channel-spectrum and scalar amplifier-boundary queries have focused acceptance below; API-02 backend acceptance passes below; Qt adoption remains DS-01. Qt owns screen coordinates/labels. | Independent numerical vectors plus API equivalence to supported observations, tap identity/paired lengths, PSD/MER normalization, reference geometry, likelihood boundaries, missing/invalid/partial states and finite output; native Python has matching data. |
| [TX providers](../../../../apps/qt/src/downstream/instruments/transmitter), PLC/NCP messages, source/waveform timing, physical layout | Typed idle TX/stage contracts and passive `FullLoopResult::plc_messages` cover original message data and bounded decoded descriptors | UIU-API-01 accepted: typed idle `TransmitterPreviewRequest`, one-shot control, owned actual TX samples/PSD and transmission symbol timing/window. UIU-API-02 adds actual retained stages and CRC-validated TX message queries with original bytes, typed TLVs and actual sample coverage; no blanket engine re-export. | No hidden run or reseeding, explicit stage/source status, supported DS PLC/NCP/profile controls unchanged, sample/timing/grid metadata and native Python parity. |
| [US session](../../../../crates/ofdma/src/api/session.rs), `configure`, `preview`, `run_with_control`, measured admission | Existing accepted typed configuration, controlled batch report, selected QAM/spectrum taps and progress can be reused | UIU-API-04 [audit and exact report-view design](design.md#uiu-api-04-upstream-instrument-gaps-and-report-views): typed verification/FEC summaries and bounded original-word paging have [focused acceptance](validation.md#uiu-api-04-bounded-recovery-view-unit). [Configured deterministic C/H/composite and periodic inverse](validation.md#uiu-api-04-configured-response-and-periodic-impulse-unit) now pass focused acceptance through idle/frozen public queries. [Actual received-pilot retention](validation.md#uiu-api-04-actual-received-pilot-estimate-unit) now passes scoped Rust/native acceptance with opt-in aggregate limits and passive original-coordinate queries. [Actual soft metrics](validation.md#uiu-api-04-actual-receiver-soft-observation-unit) now pass scoped Rust/native acceptance with independent limits and actual input/flag preservation. [Compatible QAM references, exact stage pairs and received-response display](validation.md#uiu-api-04-qam-pairing-and-received-response-display-unit) now pass scoped acceptance. The [final backend gate](validation.md#uiu-api-04-upstream-instrument-backend-accepted) completes API-04 with full Rust and installed native acceptance. No duplicate Qt simulation interface. | Synthetic/measured support states, config generations, limits and partial completion; actual US reference planes, selected tap bounds, no fabricated metrics; Rust/native Python report/error parity. Recovery views retain original outcomes/offsets/flags; configured response preserves replay generations, units, zero/unsupported states and periodic aliasing. Qt adoption and rendering remain separate task gates. |

### UIU-API-02 physical-bin and transmitted-role unit

`TransmissionConfiguration::physical_layout` and `physical_layout_capture` reuse
captured-layout admission and phase-specific physical roles. They return configured
full-FFT geometry, original source identity/grid, PLC bounds and bounded role spans.
`FullLoopResult::physical_bins` returns actual retained TX bins or raw RX FFT bins,
original context and instantaneous normalized complex-bin power; it performs no
FFT/PSD conversion. `FullLoopResult::transmit_cells` preserves actual typed cells
and mapped packing roles, with first-aligned-source metadata transport only where
retained provenance proves it. Missing/later roles remain unknown. All queries
have matching native Python operations and 8192-element caps. See the
[contract](../../../user/api/physical-views.md) and
[focused acceptance](validation.md#uiu-api-02-physical-bin-and-transmitted-role-unit).

API-02 backend acceptance passes below. Qt adoption remains UIU-DS-01. The existing normalized
contiguous-sample spectrum operation and raw-stage copy limits are unchanged.

### UIU-API-02 complete catalog and bounded sweep unit

`ProfileCapture::simulation_profile` admits original selected loading through the
unchanged checked run-ID domain while retaining complete original provenance.
`FullLoopSweepPlan` preserves all raw source IDs, current edited requests or
sorted explicit captured selections, validates every profile before admission
and supplies one independently controlled public request at a time. It shares
base/catalog ownership and keeps the existing 32-profile/262144-assignment policy.
The [exact owner/signature design](design.md#uiu-api-02-complete-catalog-and-bounded-sweep-unit)
and [user contract](../../../user/api/full-loop-sweep.md) preserve raw versus
checked IDs, budgets, original loading and cancellation/result ownership.
[Focused acceptance](validation.md#uiu-api-02-complete-catalog-and-bounded-sweep-unit)
and [final API-02 backend acceptance](validation.md#uiu-api-02-public-full-loop-and-observations-accepted)
pass full Rust and installed-native validation. Qt adoption remains UIU-DS-01.

### UIU-API-02 calibrated channel and amplifier unit

The existing channel-power/Welch/layout owners now back typed calibrated time-stage
spectra, fixed before/after-amplifier and actual receiver-input spectra, and
amplifier-only physical-bin observations. Idle TX PSD uses the same explicit
calibration without applying channel gain or noise. Original contexts, derived
ratios/reference planes, physical roles, zero/missing values and zoom-independent
full-band power are retained. Original normalized methods remain unchanged.
The native adapter mirrors these operations and passes all 107 installed cases,
including actual numerical, ownership and separate Rust-process parity. See the
[channel spectrum contract](../../../user/api/channel-spectrum.md) and
[owner design](design.md#uiu-api-02-calibrated-channel-and-amplifier-unit) and
[focused acceptance](validation.md#uiu-api-02-calibrated-channel-and-amplifier-unit).
Complete source-catalog/sweep parity and final API-02 backend acceptance pass
below. Qt adoption has not begun.

### UIU-API-02 ordered frame-stage spectrum unit

`FullLoopResult::spectrum_sequence(FullLoopSpectrumQuery)` adds the separately
labelled diagnostic sequence for actual IFFT, prefix, window and RX useful frames.
The existing waveform Welch owner supplies all PSD math; original symbol/sample
intervals, true physical contiguity, stop reasons and analyzed segment coverage
remain explicit. Missing history is never filled. The original `spectrum` query
and retention defaults are unchanged. Native Python mirrors the typed Rust query,
releases the GIL and keeps its exact numeric default visible to introspection.
See the [sampling contract](../../../user/api/full-loop-observations.md#ordered-frame-stage-spectrum)
and [focused acceptance](validation.md#uiu-api-02-ordered-frame-stage-spectrum-unit).

The numeric audit names independent equal-valued settings separately and derives
FFT/analysis dimensions from the existing owners. It preserves original physical
view bounds while giving role-span/identifier/frame limits distinct purposes.
Complete catalog/sweep and final API-02 backend acceptance pass below; Qt
migration remains a separate task.

### UIU-API-02 numerical query unit

`FullLoopResult::spectrum(FullLoopSpectrumQuery)` uses the existing waveform
Welch owner on actual retained time samples. It joins only adjacent samples
with matching original geometry, phase, reference and absolute origin, stops at
gaps, bounds work to 32 overlapping windows and returns the existing 2048-bucket
PSD contract with actual sample accounting. Frequency-domain stages reject;
they must never be transformed a second time or labelled as PSD.

`FullLoopResult::response(FullLoopResponseQuery)` distinguishes configured
channel transfer from an actual retained receiver estimate. The existing
composition, layout and receiver diagnostic owners supply all numerical
values. Excluded carriers remain gaps; absent or incomplete receiver estimates
reject instead of substituting supplied H. Full original 4K/8K response and
impulse arrays are bounded backend results; Qt owns only screen projection.

`FullLoopResult::llr_statistics(snapshot)` summarizes the actual retained
payload LDPC-input values through the existing likelihood owner. It retains
coding order, phase, original offsets, sign convention and retained-versus-total
counts, without changing or synthesizing decoder inputs. Python methods
return owned dictionaries with matching validation, releasing the GIL during
numerical preparation. These operations implement the existing UIU-API-02 and
REQ-UIU-02/07/09/16/17/18/19/20/24 contracts. Selected-cell/message units have
focused acceptance; complete
source-catalog/sweep parity and final API-02 backend acceptance also pass below.

| Existing owner | Current contract | Planned extension | Acceptance |
| --- | --- | --- | --- |
| [ResourcePlan](../../../../crates/ofdma/src/resources/grid.rs), [pilot patterns](../../../../crates/ofdma/src/resources/pilot_patterns.rs) | `Simulation::preview()` retains its existing plan summary; the new idle query provides authoritative physical allocation | UIU-API-03 implemented `Simulation::resource_grid(&self, request: ResourceGridRequest) -> ApiResult<ResourceGridSnapshot>` and bounded `to_json`. [Exact fields/budgets](design.md#uiu-api-03-idle-upstream-resource-grid-observation) retain configuration generation, original coordinates/timing, roles/pattern/grant/IUC identity and full-plan capacity independently of zoom. ResourcePlan remains the scientific authority. | [Backend acceptance](validation.md#uiu-api-03-idle-upstream-resource-grid-accepted): 88 upstream, 16 facade and 114 installed native cases; literal edge/body pilots, zero/exclusion/unallocated positions, all admitted patterns/frame sizes/numerologies, limits, noisy-run invariance, owned snapshots, Busy/cancel and separate-process parity. Qt viewer remains UIU-US-04. |
| [Native workbench loader](../../../../crates/pnm-input/src/workbench), [Qt project schema](../../../../apps/qt/src/downstream/workbench/project_file.rs), updater recovery | Existing schema 1/2 DS input API works; additive [saved full-loop preparation](validation.md#uiu-api-05-saved-downstream-full-loop-unit) now preserves file/fill, temporal playback and independent response tolerance. Qt still owns file lifecycle; [schema-3 Project admission](validation.md#uiu-api-05-direction-project-unit) now has scoped Rust/native acceptance. The [parent backend gate passes](validation.md#uiu-api-05-project-backend-accepted); Qt file/recovery adoption remains separate | UIU-API-05: typed direction-tagged project envelope in native input adapter, DS legacy importer and US configuration/measured provenance project. Public byte admission/serialization/load/run through facade; Qt retains chooser, dirty guards, safe write/replacement and private recovery transport only. | Exact legacy DS semantics, UI→Rust/Python load and execute, Rust/Python→UI round trip, malformed/wrong-direction/oversized/hash mismatch/unsupported version, no mutation on failure, future fields/version policy; installed wheel tests. |

Source inspection also has a compatibility boundary: existing Qt retains raw u8
profile IDs, while the accepted simulation ProfileId stays 0..15. The profile
capture inspector preserves the complete source catalog without changing that
quantity. API-02 now explicitly tests the legacy rejection outside 0..15, mixed
source-catalog admission and original selected loading. DS-01 must preserve this
accepted behavior when replacing Qt's legacy route.

The resource-grid operation now has implemented fields, numeric budgets,
ordering, missing-value/overflow behavior and native names with focused acceptance.
Apply the same pre-code review to every other extension: specify signatures and parity,
reuse existing symbols where sufficient, then add independent tests and commit
the API unit before its Qt consumer. This plan does not advertise new operations
as callable or require exposing private structures wholesale.

## Architectural enforcement

UIU-BOUND-01 introduces an automatic check of actual Cargo package identities
and Qt source imports/calls, including renamed/path dependencies, fully qualified
paths and `extern crate`. During migration only an exact, reviewed list of
pre-existing call sites may remain; each has an owner and sunset task. New direct
scientific imports/calls fail the check. File moves explicitly update the list;
they cannot expand permitted operations. Negative tests must prove alias and
direct-call detection. Final acceptance requires no scientific migration entries.

The check covers Qt production and test sources. Scientific unit tests belong in
their backend owner; Qt tests consume public APIs or pure UI fixtures. A source
guard alone cannot prove that arbitrary numerical code is presentation-only:
manual data-owner review and API/Qt parity assertions remain required. Do not
ban legitimate transitive scientific dependencies reached through the facade.

## Common contracts and lifecycle

Reuse common cancellation/error codes only where their meanings match. DS and US
configuration, results and run-control capabilities remain distinct. Adapters may
share UI status formatting and a dispatch slot without defining a second science
configuration language. Every callback/result carries direction, session identity,
accepted configuration generation and run identity. Stale callbacks cannot replace
current results; closing an instrument does not invalidate the backend run, and
destroying its owning session cancels/joins or safely drains the bounded worker.

## UIU-DS-01 source-catalog OCD compatibility follow-up

The [source-profile compatibility prerequisite](design.md#uiu-ds-01-source-catalog-ocd-compatibility-api-prerequisite)
closes the actual editor gap discovered during Qt adoption: original byte-sized
source IDs need geometry validation without conversion to a checked run ID.
`TransmissionConfiguration::check_profile_capture` and its installed native Python
method reuse the existing selected-source lookup/grid validator. No engine,
resource, operating-mode or schema change is introduced.

[Scoped paired acceptance](validation.md#uiu-ds-01-source-profile-compatibility-api-accepted)
preserves every prior operation/export and completed planning/FEC/API gate.
Qt consumers and their frozen debt remain UIU-DS-01 work; this backend extension
is committed before any source-catalog admission replacement.

## UIU-DS-01 idle replay-capacity follow-up

The [idle-capacity prerequisite](design.md#uiu-ds-01-idle-replay-capacity-api-prerequisite)
exposes the existing query through immutable `MeasurementSequenceOptions` before
profile/capture admission. The admitted sequence delegates to the same backend
calculation; no dummy inputs or duplicated Qt clock logic are required.
[Scoped paired acceptance](validation.md#uiu-ds-01-idle-replay-capacity-api-accepted)
preserves the original API duration domain and all prior operation rows.
Qt selection status and its narrower CWB admission remain UIU-DS-01 consumer work.

## UIU-DS-01 minimum observation timing follow-up

The [minimum-window prerequisite](design.md#uiu-ds-01-minimum-observation-timing-api-prerequisite)
adds `TransmissionConfiguration::minimum_observation_milliseconds_after` and
the matching installed native method. Both delegate to the backend integer-clock
owner, using shared SI conversion constants. [Scoped paired acceptance](validation.md#uiu-ds-01-minimum-observation-timing-api-accepted)
covers exact/fractional rounding, both clocks and maximum indices. The theoretical
threshold can exceed admission limits and excludes acquisition/drain/window tails.
Qt timing-help presentation remains unchanged until DS-01 adoption.

## UIU-DS-01 relative channel power follow-up

The [relative-power prerequisite](design.md#uiu-ds-01-relative-channel-power-api-prerequisite)
adds `ChannelSpectrum.normalized_power_db` and the matching serialized native key
through the existing scientific owner. [Scoped paired acceptance](validation.md#uiu-ds-01-relative-channel-power-api-accepted)
covers literal flat-density/zero anchors, full-band zoom/calibration independence
and fresh Rust/native process parity. Existing methods, constructors, original
fields and operation rows remain intact. Qt's relative-power formatter still
awaits adoption, with no local logarithmic calculation permitted in its replacement.

## UIU-DS-01 unnamed file-source compatibility follow-up

The [unnamed-source prerequisite](design.md#uiu-ds-01-unnamed-file-source-compatibility-api-prerequisite)
preserves the reproduced legacy empty file-label case with an explicit passive
public SourceInput operation and native equivalent. Existing named constructors
and profile/measurement/H provenance checks stay strict. Saved full-loop source
admission shares the same operation and exact integrity/resource checks.
[Paired acceptance](validation.md#uiu-ds-01-unnamed-file-source-api-accepted) precedes
restoring/adapting the preserved idle TX consumer. No schema or scientific mode changes.

## UIU-DS-01 idle transmitter preview consumer

[Scoped preview adoption](validation.md#uiu-ds-01-idle-transmitter-preview-facade-accepted)
now uses the accepted public source/capture/preview/control, timing/window,
configured-layout and calibrated-spectrum operations. The [exact record](downstream-preview-evidence.json)
retains original source/timing/spectrum behavior and the explicit unnamed-file
compatibility prerequisite. Qt projects backend values into the existing fields;
no private TX/window/FFT/calibration operation remains in these two retired files.
Other TX inspectors and channel/receiver providers remain separate DS-01 work.

## Raw source-catalog admission follow-up

The [exact prerequisite contract](design.md#uiu-ds-01-raw-source-catalog-admission-prerequisite)
and [paired acceptance](validation.md#uiu-ds-01-raw-source-catalog-api-accepted)
cover ProfileCapture.frame and source-catalog variants of measured sequence,
response and Designer operations. Existing Rust/native APIs, source-ID/run-ID
separation, original provenance and calculation owners are retained. Qt input
adoption remains UIU-DS-01; no completed API or FEC task is reopened.

The [original measurement-frame follow-up](design.md#uiu-ds-01-original-measurement-frame-prerequisite)
exposes only immutable original channel data and nominal source DTOs, with
[paired acceptance](validation.md#uiu-ds-01-original-measurement-frame-api-accepted).
It preserves separate original/intermediate provenance before measured-input
adapter adoption; no scientific engine/kernel export or completed-task restart.
