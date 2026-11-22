# Unified API compatibility matrix

Subordinate to [API architecture](../../../architecture.md#unified-rust-python-and-cli-apis),
[requirements](../../../requirements.md#req-com-api-09) and the
[official checklist](../../checklist.md#int-rust-01). This is the single operation
matrix required by master §7. The [full row data](operation-matrix.json) now records
481 explicit public Rust operations and 465 installed native Python entries with
all seventeen cells, expanded macro methods, adapter exports and deliberate language differences. Shared contract cells
resolve through named definitions in that same data file. The session table below
is a readable subset of this one matrix. [Current native operation acceptance](../unified-qt-workbench/full-loop-request-evidence.json)
checks every installed constructor, method, property and constant, exact signatures,
105 directional/common class exports and 59 preserved legacy root exports. Named
owner regressions establish behavioral scope; metadata alone is not scientific
execution or exhaustive getter-branch coverage.

The additive [UIU-API-01 admission/preview acceptance](../unified-qt-workbench/validation.md#uiu-api-01-public-admission-accepted)
appends 99 Rust and 75 native rows, retaining every original row and historical
run record, including [historical native acceptance](python-operation-evidence.json). Its [current exact execution cases](../unified-qt-workbench/api-01-operation-test-results.json)
and [candidate/wheel evidence](../unified-qt-workbench/transmitter-preview-evidence.json)
cover the new input/editor/preview owners. That acceptance checked its complete
matrix; named owner cases establish scope without claiming exhaustive
getter/field/helper branch coverage.

The [UIU-API-02 controlled-request unit](../unified-qt-workbench/validation.md#uiu-api-02-controlled-request-unit)
adds 18 Rust/17 native rows with [current executed cases](../unified-qt-workbench/api-02-request-operation-test-results.json)
and a fresh 82-case installed wheel. This accepts the scoped request/control/report
operations; retained queries and complete catalog/sweep parity keep the parent task
open. All earlier operation rows and historical evidence remain intact.

The [operation run records](operation-test-results.json) add a targeted measured-plan
roundtrip and retain exact named historical MERLIN component cases. The
[namespace run records](namespace-test-results.json) retain actual compatibility,
engine/lifecycle and analysis-only execution. Historical source results are not
substituted for these MERLIN runs.

<a id="namespace-proposal-for-review"></a>
## Selected namespace implementation

The [selected namespace-to-Cargo mapping](namespace-design.md) supersedes the
initial crate-alias/common-ownership alternatives. The user requires a literal
`merlin::{downstream,upstream,common}` root. The implemented minimal Cargo/library
`merlin` facade re-exports existing `docsis_ofdm_fec::api`, implemented
`docsis_ofdma::api` and the neutral `merlin_api` contracts. This is justified
by independent PHY usability and exact backward compatibility; neither PHY depends
on the facade or the other PHY. Common contracts live in their own small leaf,
never in a downstream-specific owner that upstream must import.

Python uses one existing `merlin-ofdm` distribution and real `merlin` package,
one private `merlin._native` extension and import-only directional/common modules.
Root `merlin.Simulation` remains the same downstream native type. Real native
namespaces/classes, type files and installed-wheel execution now pass INT-PY-01.
[INT-API-MAP-01](../../checklist.md#int-api-map-01) documents the exact mapping;
INT-RUST-01/INT-PY-01 record implementation acceptance.
The complete exports, dependency graph, compatibility behavior, implemented common
signatures and named acceptance tests are in that selected design. This matrix
continues to own the existing exact session signatures and parity rows below.

## Existing inventory and ownership

These are verified existing owners, not hypothetical API names:

| Surface | Exact owner / current exported operations |
| --- | --- |
| Rust exports | [api/mod.rs](../../../../crates/ofdm/src/api/mod.rs), [Simulation](../../../../crates/ofdm/src/api/simulation.rs), [Merlin/options/control](../../../../crates/ofdm/src/api/controller.rs), [profile operations](../../../../crates/ofdm/src/api/controller/profiles.rs). |
| Rust domain objects | [configuration.rs](../../../../crates/ofdm/src/api/configuration.rs), [inputs.rs](../../../../crates/ofdm/src/api/inputs.rs), [units.rs](../../../../crates/ofdm/src/api/units.rs), [reports.rs](../../../../crates/ofdm/src/api/reports.rs), [error.rs](../../../../crates/ofdm/src/api/error.rs). Existing constructors/getters/builders are preserved; enumerate every export into the final matrix before convergence acceptance. |
| Native capture/workbench/export | [pnm-input/lib.rs](../../../../crates/pnm-input/src/lib.rs), [workbench.rs](../../../../crates/pnm-input/src/workbench.rs), [rxmer_export](../../../../crates/pnm-input/src/rxmer_export/mod.rs). Rust file admission, discovery and provenance remain authoritative. |
| Python registrations | [lib.rs](../../../../bindings/python/src/lib.rs), [controller.rs](../../../../bindings/python/src/controller.rs), [inputs.rs](../../../../bindings/python/src/inputs.rs), [quantities.rs](../../../../bindings/python/src/quantities.rs), [options.rs](../../../../bindings/python/src/options.rs), [receiver.rs](../../../../bindings/python/src/receiver.rs), [reports.rs](../../../../bindings/python/src/reports.rs), [workbench.rs](../../../../bindings/python/src/workbench.rs), [rxmer_export.rs](../../../../bindings/python/src/rxmer_export.rs), and [upstream registration](../../../../bindings/python/src/upstream/mod.rs). One private binary serves the real package modules. |
| Implemented upstream foundation | [fec-codec upstream_ldpc](../../../../crates/fec-codec/src/upstream_ldpc/mod.rs), three code families, shortening, independent source vectors, shared sparse encoder mechanism. [Executed acceptance](../unified-backend/fec-validation.md), committed in `343b07d9209dc10f3c21d80ae2222a60432f684a`. The independent upstream PHY/session and measured adapter now pass local Rust acceptance; native Python now passes its separate installed-wheel acceptance. More recent full regression evidence is linked from the official checklist. |

## Downstream session operation rows

All Rust paths in the following table are methods of
`docsis_ofdm_fec::api::Simulation`, with referenced domain types from the same
namespace. Signatures are exact current source declarations with whitespace
normalized. Python paths are current native `merlin.Simulation`; Python `Merlin`
is its alias. `S`, `C`, `P` and `G` below are explicit shared contract cells, not
unspecified acceptance:

- **S**: owned native session, no geometry initially; supplying OCD copies it;
  no work runs on construction. No physical units. Existing downstream behavior.
- **C**: input values are copied/owned into Rust; getters borrow in Rust and own
  returned Python objects. Units remain in typed quantities (Hz, dB, ms, ns,
  iterations, seeds or counts); IDs are controller-scoped. Set/load/prepare is
  idle and atomic. `ApiResult` maps to structured `ApiError` with `ErrorCode`;
  native conversion errors remain Python errors, mutable access may be Busy.
- **P**: metadata/IDs/reports follow accepted profile/capture provenance and
  independent report ownership. Preparation returns typed `PreparedExperiment`
  only in Rust; no Python preparation method is currently exported.
- **G**: bounded explicit execution returns owned `BatchReport` with accepted
  run inputs, actual decoder/scientific status and partial coverage. Cancellation
  preserves completed work; native run releases the interpreter lock and guards
  Busy. Timing is separate from deterministic science. No shared pause/resume
  Python method is assumed from the common lifecycle diagram.

Each existing row links [REQ-DS-API-01](../../../requirements.md#req-ds-api-01),
[REQ-COM-API-09](../../../requirements.md#req-com-api-09),
[INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01)
and [INT-REG-01](../../checklist.md#int-reg-01). Status is **implemented, inspected;
historical regression record available, no new execution in this preflight**.
The [previous MERLIN run](../unified-backend/fec-validation.md#executed-checks)
records 25 actual native Python cases and full Rust regression; it does not accept
future aliases, submodules or upstream operations. Exact current cases are T1–T5:

| Key | Exact test file and case | Evidence scope |
| --- | --- | --- |
| T1 | `crates/pnm-input/tests/high_level_api.rs::simulation_loading_is_idle_atomic_and_preserves_scoped_measurements`; `bindings/python/tests/test_api.py::SimulationTests.test_idle_load_reload_and_legacy_alias` | Idle loading and retained measurements, existing imports. |
| T2 | `crates/pnm-input/tests/high_level_api.rs::high_level_api_matches_direct_engine_and_owns_old_reports`; `bindings/python/tests/test_parity.py::RustPythonParity.test_every_static_selection_policy_matches_rust` | Direct-engine result parity; existing native comparison. |
| T3 | `crates/pnm-input/tests/high_level_api.rs::cpa_controls_validate_atomically_and_preserve_idle_settings`; `bindings/python/tests/test_receiver.py::ReceiverModeTests.test_cpa_setters_getters_and_atomic_validation` | CPA/launch controls; existing native control validation. |
| T4 | `crates/pnm-input/tests/high_level_api.rs::cancellation_preserves_all_requested_entries_without_execution`; `bindings/python/tests/test_api.py::NativeApiTests.test_gil_release_busy_guard_and_cross_thread_cancellation` | Cooperative control, ownership and Busy. |
| T5 | `crates/pnm-input/tests/high_level_api.rs::profile_catalog_discovery_matches_core_json_and_binary_without_selection`; `bindings/python/tests/test_api.py::LoadedProfilesTests.test_inventory_selection_cancellation_and_single_run_parity` | Profile catalog/discovery and profile runs. |

| Rust exact signature | Python current signature / result | Contract / tests |
| --- | --- | --- |
| `pub fn new() -> Self` | `Simulation(ocd=None) → Simulation` | S / T1 |
| `pub fn with_ocd(ocd: Ocd) -> Self` | `Simulation(ocd=None) → Simulation (supply validated OCD)` | S / T1 |
| `pub fn load_profiles(&mut self, profiles: Vec<Dpd>) -> ApiResult<()>` | `load_profiles(self, path, protocol=None, overrides=None) → None; Rust typed input, Python Rust-file adapter` | C/P / T1 |
| `pub fn load_profiles_with_options( &mut self, protocol: ProtocolScope, profiles: Vec<Dpd>, overrides: OcdOverrides, ) -> ApiResult<()>` | `load_profiles(self, path, protocol=None, overrides=None) → None` | C/P / T1 |
| `pub fn profile_ids(&self) -> Vec<ProfileId>` | `profile_ids (read-only property) → list[ProfileId]` | C/P / T5 |
| `pub fn profile(&self) -> Option<&Dpd>` | `profile (read-only property) → DPD or None` | C/P / T5 |
| `pub fn ocd(&self) -> ApiResult<&Ocd>` | `ocd (read-only property) → OCD` | C / T1 |
| `pub fn options(&self) -> ApiResult<&EvaluationOptions>` | `options (read-only property) → EvaluationOptions` | C / T3 |
| `pub fn profile_by_id(&self, id: ProfileId) -> ApiResult<&Dpd>` | `profile_by_id(self, profile_id) → DPD` | C/P / T5 |
| `pub fn select_profile(&mut self, id: ProfileId) -> ApiResult<()>` | `select_profile(self, profile_id) → None` | C/P / T5 |
| `pub fn set_profile(&mut self, profile: Dpd) -> ApiResult<()>` | `set_profile(self, dpd) → None` | C/P / T1 |
| `pub fn add_rxmer(&mut self, frame: RxMer) -> ApiResult<RxMerId>` | `add_rxmer(self, frame) → RxMerId` | C/P / T2 |
| `pub fn add_channel_estimate(&mut self, frame: ChannelEstimate) -> ApiResult<ChannelEstimateId>` | `add_channel_estimate(self, frame) → ChannelEstimateId` | C/P / T2 |
| `pub fn rxmer(&self, id: RxMerId) -> ApiResult<&RxMer>` | `get_rxmer(self, capture_id) → RxMer` | C/P / T2 |
| `pub fn channel_estimate(&self, id: ChannelEstimateId) -> ApiResult<&ChannelEstimate>` | `get_channel_estimate(self, capture_id) → ChannelEstimate` | C/P / T2 |
| `pub fn set_options(&mut self, options: EvaluationOptions) -> ApiResult<()>` | `set_options(self, options) → None` | C / T2 |
| `pub fn set_cpa_attenuation_db(&mut self, value_db: f64) -> ApiResult<()>` | `set_cpa_attenuation_db(self, value_db) → None` | C; dB in 0..60 / T3 |
| `pub fn get_cpa_attenuation_db(&self) -> ApiResult<f64>` | `get_cpa_attenuation_db(self) → float` | C; dB / T3 |
| `pub fn set_launch_amplifier_gain_db(&mut self, value_db: f64) -> ApiResult<()>` | `set_launch_amplifier_gain_db(self, value_db) → None` | C; dB / T3 |
| `pub fn get_launch_amplifier_gain_db(&self) -> ApiResult<f64>` | `get_launch_amplifier_gain_db(self) → float` | C; dB / T3 |
| `pub fn prepare(&self) -> ApiResult<Vec<PreparedExperiment>>` | `No Python export; retain current direction-specific distinction` | C/P / T2 |
| `pub fn run(&mut self) -> ApiResult<BatchReport>` | `run(self) → BatchReport` | G / T2 |
| `pub fn run_with_control(&mut self, control: &RunControl) -> ApiResult<BatchReport>` | `run_with_control(self, control) → BatchReport` | G / T4 |
| `pub fn run_all_profiles(&mut self) -> ApiResult<BatchReport>` | `run_all_profiles(self, control=None) → BatchReport` | G / T5 |
| `pub fn run_all_profiles_with_control( &mut self, control: &RunControl, ) -> ApiResult<BatchReport>` | `run_all_profiles(self, control=None) → BatchReport` | G / T4/T5 |

## Full operation rows and delivery boundaries

Search [operation-matrix.json](operation-matrix.json) by its stable `operation`
path, for example `merlin::upstream::Simulation::run`. Each row contains the
seventeen master cells: direction, operation, Rust namespace/declaration,
Python namespace/declaration, ownership, units, errors, lifecycle, result/schema,
compatibility, requirement, task, exact tests, executed evidence and status.
Source files and compiler signatures are recorded in the Rust declaration cell.
Named contract references resolve in the data's `contracts` object.

| Row group | Rust scope | Python scope |
| --- | --- | --- |
| Downstream: 234 operations | Existing API, expanded quantities, file/workbench/export adapters and exact old/new nominal identity. | Existing root signatures are verified from the actual 0.4.0 extension; directional aliases and a fresh wheel still need INT-PY-01. |
| Upstream: 126 operations | Independent real session/models, measured admission, raw observations and the two hidden cross-crate Serde helpers. | Planned native declarations come from the reviewed source adapter; new shared cancellation/error mappings are explicit. No new upstream import is currently claimed. |
| Common: 4 operations | Clone/atomic query and one-way control contracts, with directional rich lifecycle retained. | Common re-exports/query and installed imports remain planned. The Rust Cancellation trait has no Python class. |

Rust borrowed views, generic deserializers, validated-owner helpers and native
file conveniences are recorded with explicit language-only or adapter mappings.
A missing direct Python helper is never labeled as a callable native method.
Constructor/returned-view conversions reuse Rust validation; no adapter calculates
an alternative scientific result. Existing root/class signatures remain unchanged.

Validation uses `tools/validation/api_operation_matrix.py` with fresh compiler
exports from `rust_api_inventory.py` and the named executed records. It rejects
missing/empty cells, unresolved contracts/requirements/tasks, stale owners/signatures,
missing exact cases, unexecuted Rust cases and uncovered compiler exports. Native
export-completeness and independent numerical parity remain INT-PY-01/INT-TOOLS-01
acceptance work. The official checklist alone owns task completion.
