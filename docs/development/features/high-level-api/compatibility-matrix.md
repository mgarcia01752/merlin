# Unified API compatibility matrix

Subordinate to [API architecture](../../../architecture.md#unified-rust-python-and-cli-apis),
[requirements](../../../requirements.md#req-com-api-09) and the
[official checklist](../../checklist.md#int-rust-01). This is the single operation
matrix required by master §7. It starts with inspected downstream session rows;
full export coverage, upstream signatures and executed direction/native acceptance
remain open under INT-RUST-01, INT-PY-01 and INT-DOC-02. This seed is not final API
acceptance. Source inspection took place at `402106f0b85605e38dbc1aa755b3799e6939b024`.

## Namespace proposal for review

The current Rust Cargo package is `docsis-ofdm-fec`, Rust library
`docsis_ofdm_fec`; the `high-level-api` feature enables `api`.
[Its exports](../../../../crates/ofdm/src/api/mod.rs) include `Simulation`, `Merlin`,
configuration/input/report/unit types, `ApiError`, `ApiResult`, `ErrorCode`,
`ReceiverMode`, `SubcarrierSpacing` and `ProtocolScope`.
Neither `crates/ofdma` nor a Rust `merlin` facade exists at the review baseline.

| Logical interface | Proposed exact Rust export / owner | Decision state |
| --- | --- | --- |
| Existing downstream | `docsis_ofdm_fec::api::*`, Cargo `docsis-ofdm-fec`, `crates/ofdm/src/api/` | Existing; retain all imports and exact type identity. |
| Explicit downstream | `docsis_ofdm_fec::downstream::*`, additive `pub use api as downstream` in `crates/ofdm/src/lib.rs` under the same feature gate | Proposed alias; no second engine and no package rename. Approval required before edit. |
| Explicit upstream | `docsis_ofdma::upstream::*`, proposed Cargo `docsis-ofdma` in `crates/ofdma/Cargo.toml`; alias of its typed `api` module | Proposed; typed stage API first, final convergence after the actual closed loop. |
| Common contracts | Compatible directional lifecycle/error/report conventions; retain existing downstream `RunControl`, `RunLimits`, errors and reports | No common Rust module exists. Nominal shared types/leaf extraction require semantic evidence and a separately reviewed exact placement; never make upstream depend on downstream. |
| Optional single Rust root | If review requires literal `merlin::{downstream,upstream,common}`, evaluate a thin `crates/merlin` facade with a separately named Cargo package/library, re-exporting validated owners | Alternative only; no umbrella prerequisite or placeholder crate. Final Cargo/library names are unresolved, not approved. |

The recommended initial topology uses direction crates and compatible contracts.
The prompt's `merlin` tree is logical, not a claim that this Cargo crate exists.
The two direction paths above are concrete proposals. Common nominal ownership
and whether a literal shared root is necessary remain explicit design decisions.
Present the complete final mapping for approval under master §4.4/§19 before
compatibility-sensitive exports, including the initial upstream API. No approval
is inferred from completing INT-PLAN-01 or from earlier migration approval.
This review does not change the current branch or start INT-DSP-01.

The current Python distribution is `merlin-ofdm`, Cargo package `merlin-python`,
native library/import `merlin`, with one PyO3 extension. The module registers
`Simulation` and aliases `Merlin` to it; both are downstream. No `.pyi` or
`py.typed` file is currently tracked under `bindings/python`. Existing Sphinx
reference generation inspects actual native classes; it is not a type-stub generator.

| Proposed Python tree | Native dispatch | Import acceptance |
| --- | --- | --- |
| `merlin.Simulation`, `merlin.Merlin`, current domain/report classes | Existing downstream Rust, unchanged signatures and type identity | Preserve root imports and legacy aliases. |
| `merlin.downstream.Simulation` | Alias to the same downstream native class | Proposed; direct imports, identity, discovery and installed wheel checks required. |
| `merlin.upstream.Simulation` | Planned upstream Rust session, distinct typed UCD/IUC configuration | Proposed; no upstream class exists yet. |
| Common native objects | Re-export only genuinely compatible types | Proposed; avoid falsely equating directional report/configuration types. |

Evaluate a real Python package with thin `downstream.py` / `upstream.py` modules
against PyO3 submodule registration. A real-package design would require reviewed
maturin `module-name`/`python-source` changes and native extension placement; a
registered design must prove standard imports, `sys.modules`, discovery and wheel
installation. Select the layout after those checks; attaching attributes alone
is insufficient. Keep one distribution/extension, Python >=3.10, root imports,
structured native errors and saved-workbench behavior. No package edits are made
by this proposal.

## Existing inventory and ownership

These are verified existing owners, not hypothetical API names:

| Surface | Exact owner / current exported operations |
| --- | --- |
| Rust exports | [api/mod.rs](../../../../crates/ofdm/src/api/mod.rs), [Simulation](../../../../crates/ofdm/src/api/simulation.rs), [Merlin/options/control](../../../../crates/ofdm/src/api/controller.rs), [profile operations](../../../../crates/ofdm/src/api/controller/profiles.rs). |
| Rust domain objects | [configuration.rs](../../../../crates/ofdm/src/api/configuration.rs), [inputs.rs](../../../../crates/ofdm/src/api/inputs.rs), [units.rs](../../../../crates/ofdm/src/api/units.rs), [reports.rs](../../../../crates/ofdm/src/api/reports.rs), [error.rs](../../../../crates/ofdm/src/api/error.rs). Existing constructors/getters/builders are preserved; enumerate every export into the final matrix before convergence acceptance. |
| Native capture/workbench/export | [pnm-input/lib.rs](../../../../crates/pnm-input/src/lib.rs), [workbench.rs](../../../../crates/pnm-input/src/workbench.rs), [rxmer_export](../../../../crates/pnm-input/src/rxmer_export/mod.rs). Rust file admission, discovery and provenance remain authoritative. |
| Python registrations | [lib.rs](../../../../bindings/python/src/lib.rs), [controller.rs](../../../../bindings/python/src/controller.rs), [inputs.rs](../../../../bindings/python/src/inputs.rs), [quantities.rs](../../../../bindings/python/src/quantities.rs), [options.rs](../../../../bindings/python/src/options.rs), [receiver.rs](../../../../bindings/python/src/receiver.rs), [reports.rs](../../../../bindings/python/src/reports.rs), [workbench.rs](../../../../bindings/python/src/workbench.rs), [rxmer_export.rs](../../../../bindings/python/src/rxmer_export.rs). No new namespaces are registered. |
| Implemented upstream foundation | [fec-codec upstream_ldpc](../../../../crates/fec-codec/src/upstream_ldpc/mod.rs), three code families, shortening, independent source vectors, shared sparse encoder mechanism. [Executed acceptance](../unified-backend/fec-validation.md), committed in `343b07d9209dc10f3c21d80ae2222a60432f684a`. No upstream PHY/session/Python facade is implemented. |

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

## Pending operation coverage and delivery

| Direction / operation | Rust / Python namespace and signature | Ownership, units, errors, lifecycle and reports | Requirements / task / evidence |
| --- | --- | --- | --- |
| Downstream remaining domain factories, getters, reports, capture/workbench loading/export and diagnostics | Current modules in the inventory above; add exact operation rows before acceptance, preserving all existing signatures | Existing typed defaults/units/schemas, owned native values, create-new exports and idle loading remain binding; no blanket parity claim for unpaired helpers | REQ-DS-API-01, REQ-COM-API-09; INT-RUST-01/INT-PY-01/INT-DOC-02; matrix coverage gap, existing backfill/history retained. |
| Explicit downstream alias/import | Proposed `docsis_ofdm_fec::downstream`; `merlin.downstream`; no new callable signature selected | Same types/engine; root imports unchanged | REQ-COM-API-07/08; INT-RUST-01/INT-PY-01; planned, no executed evidence. |
| Upstream configure/validate/prepare | Planned `docsis_ofdma::upstream` and `merlin.upstream`; exact type/method signatures await typed implementation and review | Owned immutable UCD/IUC/grant/source views, explicit grid/units/bounds, received-only RX; load/prepare idle and atomic | REQ-US-API-01/02, REQ-COM-API-03/06/07/08/09; INT-US-CONFIG-01 then INT-RUST-01/INT-PY-01; planned. |
| Upstream run/report/control | Same planned namespaces; no fabricated `run`/`report`/pause method | Freeze accepted generation; Busy/unsupported/bounds/decoder outcomes and partial reports; pause only after preservation proof | REQ-COM-LIFE-01/02/03, REQ-US-API-01/02; INT-US-LOOP-01 then INT-RUST-01/INT-PY-01; planned. |
| Upstream capture/discovery/association | Planned upstream Rust and one native package; signatures await verified parsers and accepted plan | Exact bytes/hashes/grid/identity/time/update/reference planes; explicit invalid/unsupported/association status | REQ-US-PNM-01/02/03/04, REQ-COM-API-05/09; INT-PNM-01/02 then INT-RUST-01/INT-PY-01; planned. |

Every pending or additional exported operation must gain all seventeen §7 cells,
exact native and Rust cases, command/result/candidate evidence and limitations
before the corresponding implementation task closes. Use native inventory tooling
(`tools/python/build_api_reference.py`) alongside Rust public exports; it cannot
infer parity merely from names. Expand this matrix in place, not a second matrix.
Do not add a Python preparation method or fabricate shared controls for visual
symmetry. Preserve standard native errors and actual supported behavior.
