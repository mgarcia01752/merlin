# Public API gates for Qt

Status: [UIU-API-01 public admission is accepted](validation.md#uiu-api-01-public-admission-accepted).
Controlled-run, retained-observation, upstream and persistence extensions below
remain separate implementation/acceptance tasks.
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
shared direction-tagged project envelope, if exposed as `merlin::common::Project`,
is a facade re-export of the adapter-owned type, never an implementation in the
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
| [Input models](../../../../apps/qt/src/inputs), native capture parsing and [session](../../../../apps/qt/src/workbench/session.rs) | Accepted typed DS inputs, original/sequence/Designer/profile/channel/idle-TX operations and native CaptureLoader are available; current Qt consumers still use legacy owners | UIU-API-01 accepted: existing typed inputs and bounded public operations under `crates/ofdm/src/api/`, plus curated native-input facade re-exports. Partial editor drafts remain separate from runnable accepted requests. | Same invalid-input/error/units semantics in Rust and native Python; no source advance on preview, original bytes/hash/grid preserved, failed Apply atomic; existing DS editor fixtures and saved-workbench cases. |
| Original capture view, `scenario::measurement_view` | Existing bounded projection/grouping owner reused through the high-level API | UIU-API-01: implemented `RxMer::preview(MeasurementViewport)` and `ChannelEstimate::preview(MeasurementViewport, ChannelResponseMetric)`; owned `MeasurementPreview` carries view/grid/provenance/projection. | [Rust/Python parity](../../../user/api/input-admission.md#original-measurement-previews), [unit evidence](validation.md#uiu-api-01-original-measurement-preview-unit); original H/RxMER only. Sequence, Designer, channel and idle TX units have their own accepted evidence below. |
| Ordered measured collection, `MeasurementPlayback` | Existing power-domain modes and physical response admission reused through the high-level API | UIU-API-01: implemented `MeasurementSequence::new`, `info`, `capture`, `capacity`, `at`, `preview` and `TransmissionConfiguration::check_response`; typed options retain exact order, epoch tolerance and resource limits. | [Rust/Python parity](../../../user/api/input-admission.md#measured-sequence-admission-and-preview), [unit evidence](validation.md#uiu-api-01-measured-sequence-unit); actual contributor weights/provenance, power/clock literals, rejection and owned concurrent queries. Legacy project timeline support remains UIU-API-05. |
| Native saved Designer evaluator and `RxmerRecipe` | Typed immutable design operations reuse original baseline materialization; ordinary/derived source kind is publicly observable | UIU-API-01: `inputs::RxMerDesign` flat/imported/from-bytes/replacement/materialization with bounded positions, exact identity and curated recipe configurations; no scientific facade implementation. | [Rust/Python parity](../../../user/api/input-admission.md#immutable-rxmer-designer-inputs), [unit evidence](validation.md#uiu-api-01-designer-admission-unit); saved bytes/masks, independent tilt endpoints, noncompounding replacements, version/nesting/random/resource rejection and actual WB FEC regression. |
| Original compact profile document and quantitative inspector | Read-only complete byte-ID catalog preserves encoded schemes; scientific summaries/grouping reuse original assignment/Shannon/useful-rate owners | UIU-API-01: implemented `ProfileCapture`, `ProfileViewRequest`, bounded schemes/inspect, native loader and Dpd summary/view; legacy ProfileId behavior remains unchanged. | [Rust/Python contract](../../../user/api/profile-inspection.md), [unit evidence](validation.md#uiu-api-01-profile-inspection-unit); code/role/coordinate/rate/Shannon literals, binary/JSON equivalence, bounds and immutable source provenance. Raw source-ID full-loop compatibility remains API-02/DS-01. |
| Configured transfer, existing channel composition/loss/power/filter owners | Immutable settings and original-grid effective response reuse existing physical/epoch/coverage and composition admission | UIU-API-01: implemented `ChannelConfiguration::new`, `from_json`, `power_reference`, `preview` and `preview_capture`; no stream/source state, original H stays separate. | [Rust/Python contract](../../../user/api/channel-preview.md), [unit evidence](validation.md#uiu-api-01-channel-preview-unit); literal gains/electrical scale/echo taps, excluded grids, raw IDs, bounds and invalid-composition/identity rejection. |
| [Closed-loop worker](../../../../apps/qt/src/simulation/closed_loop.rs), `run_channel_controlled`, test-point observer and `Options` | DS public `Simulation::prepare_full_loop` now admits immutable single-profile requests and direction-specific acknowledged pause/cancel/progress with owned actual partial reports; retained stages and complete source-catalog/sweep parity remain open | UIU-API-02 request/control unit implemented and [validated](validation.md#uiu-api-02-controlled-request-unit). Reuse existing job/measurement/pause owners; old RunControl/nominal types stay compatible. Retained query and catalog/sweep units remain mandatory before task acceptance. | Idle/running/paused/cancelled/failed/completed transitions, safe boundaries, bounded time/memory, completed FEC trials retained, exact old configured outputs, native Python pause/cancel/progress/result parity. |
| [Receiver views](../../../../apps/qt/src/instruments/receiver/views.rs), [selection](../../../../apps/qt/src/instruments/receiver/selection.rs), [diagnostics](../../../../apps/qt/src/instruments/receiver/diagnostics.rs), [spectrum](../../../../apps/qt/src/instruments/spectrum/mod.rs) | Qt directly computes FFT/PSD, MER, response preparation and diagnostic selection | UIU-API-02: typed observations and bounded query/viewport inputs over an immutable run result; numerical preparation stays in DS science modules; API owns unit/reference-plane metadata. Qt maps normalized screen coordinates and labels. | Independent existing numerical vectors plus API equivalence to old supported observations, tap identity/paired lengths, PSD normalization, missing/invalid/partial states and finite output; Python exposes equivalent scientific data. |
| [TX providers](../../../../apps/qt/src/instruments/transmitter), PLC/NCP messages, source/waveform timing, physical layout | High-level DS reports do not expose all TX previews, decoded messages or physical stage views used by existing UI | UIU-API-01 accepted: typed idle `TransmitterPreviewRequest`, one-shot control, owned actual TX samples/PSD and transmission symbol timing/window. UIU-API-02 still owns retained full-loop stage/message/physical observations; no blanket engine re-export. | No hidden run or reseeding, explicit stage/source status, supported DS PLC/NCP/profile controls unchanged, sample/timing/grid metadata and native Python parity. |
| [US session](../../../../crates/ofdma/src/api/session.rs), `configure`, `preview`, `run_with_control`, measured admission | Existing accepted typed configuration, controlled batch report, selected QAM/spectrum taps and progress can be reused | UIU-API-04: audit each planned instrument against actual reports. Add only missing TX/channel/RX/response/soft/FEC observations as small upstream typed result/query APIs, backed by existing owners. No duplicate Qt simulation interface. | Synthetic/measured support states, config generations, limits and partial completion; actual US reference planes, selected tap bounds, no fabricated metrics; Rust/native Python report/error parity. |
| [ResourcePlan](../../../../crates/ofdma/src/resources/grid.rs), [pilot patterns](../../../../crates/ofdma/src/resources/pilot_patterns.rs) | `Simulation::preview()` returns counts/plan summaries, not authoritative per-cell physical allocation | UIU-API-03: proposed `Simulation::resource_grid(&self, request: ResourceGridRequest) -> ApiResult<ResourceGridSnapshot>`. Request bounds symbol/physical-carrier windows and maximum returned cells/spans. Snapshot carries configuration generation, original coordinates, actual roles/pattern/grant/IUC identity and counts. Reuse scientific ResourcePlan, never reconstruct pilots in QML. | All accepted numerologies/patterns, independent literal pilot coordinates, guard/exclusion/assignment edges, invalid/oversized/out-of-window requests, no source/run/config mutation; native Python installed-wheel operation and same snapshot fields; Qt viewport geometry separately. |
| [Native workbench loader](../../../../crates/pnm-input/src/workbench), [Qt project schema](../../../../apps/qt/src/workbench/project_file.rs), updater recovery | Existing schema 1/2 DS input API works; Qt alone owns some serialized validation and file lifecycle; no US project envelope | UIU-API-05: typed direction-tagged project envelope in native input adapter, DS legacy importer and US configuration/measured provenance project. Public byte admission/serialization/load/run through facade; Qt retains chooser, dirty guards, safe write/replacement and private recovery transport only. | Exact legacy DS semantics, UI→Rust/Python load and execute, Rust/Python→UI round trip, malformed/wrong-direction/oversized/hash mismatch/unsupported version, no mutation on failure, future fields/version policy; installed wheel tests. |

Source inspection also has a compatibility boundary: existing Qt retains raw u8
profile IDs, while the accepted simulation ProfileId stays 0..15. The profile
capture inspector preserves the complete source catalog without changing that
quantity. UIU-API-02/DS-01 must preserve or explicitly test the existing full-loop
behavior for applicable raw source IDs before replacing Qt's legacy route.

The resource-grid signature is a design proposal. Before its implementation,
define exact exported model fields, numeric budgets, ordering, missing values,
overflow behavior and native Python names in that focused API task. Apply the
same pre-code review to every other extension: specify signatures and parity,
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
