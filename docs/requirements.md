# MERLIN requirements baseline

This is MERLIN's authoritative behavior and acceptance baseline. The
[architecture](architecture.md) owns design/module placement and the
[official developer checklist](development/checklist.md) owns completion status.
Feature contracts and historical source records refine these obligations; they
are not separate competing baselines. Requirements apply to their declared scope.
Current downstream support is retained; upstream and common extraction requirements
below are **pending integration acceptance**, regardless of standalone source passes.
Individual algorithm deliveries may gain passing MERLIN evidence before the full
upstream chain; their parent transport/regression/final acceptance gates stay open.

## Contents

- [Baseline rules](#baseline-rules)
- [Downstream obligations](#downstream-obligations)
- [Common consolidation requirements](#common-consolidation-requirements)
- [Backend master prompt additions](#backend-master-prompt-additions)
- [Upstream and inherited common contracts](#upstream-and-inherited-common-contracts)
- [Detailed downstream contract register](#detailed-downstream-contract-register)
- [Existing downstream API contract](#high-level-api-acceptance-contract)
- [Individual API handoff requirements](#individual-api-handoff-requirements)
- [Preservation and delivery acceptance routing](#preservation-and-delivery-acceptance-routing)

## Baseline rules

MUST/SHALL state MERLIN product obligations, not an invented CableLabs normative
force. Preserve pinned endpoint/mode and original normative references. Keep
recognized/domain-valid/executable states, measured/derived/simulated results,
source-only passes and integrated acceptance distinct. Every requirement links an
architecture owner, concrete current/planned implementation/test owner and delivery
task. Planned paths are code-formatted and must not be mistaken for existing files.

The 59 pinned upstream requirement IDs map one-to-one into `REQ-US-*` (upstream)
or `REQ-COM-*` (genuinely shared contracts). Original IDs remain aliases in the
[source inventory](development/features/unified-backend/source-inventory.json).
Shared requirements do not prove a common algorithm implementation is compatible.
Existing downstream IDs, including the numbered API ranges and explicit
`REQ-PY-DOC-*`, `REQ-API-PARITY-*`, `REQ-REPO-ARCHIVE-001`, `API-WB-*` and
`API-RECEIVER-*` contracts below, remain unchanged. Existing checklist IDs stay
stable delivery/refinement IDs rather than being renumbered into upstream IDs.

Final acceptance requires relevant tests inside unified MERLIN. An earlier
source test, documentation checkbox, matching hash or successful compile is not
that acceptance. Source code/fixtures/docs must remain preserved until relevant
requirements/design/evidence survive and replacements pass. Deferred UI/wider
modes below are explicit future scope, not immediate backend work.

## Downstream obligations

The existing official checklist's complete downstream task bodies and nested
acceptance remain binding refinements of these baseline requirements. Their
original IDs/statuses are preserved in the inventory. This table consolidates
coverage without falsely creating new historical runtime evidence.

| ID | Required behavior | Architecture / implementation owner | Acceptance / delivery |
| --- | --- | --- | --- |
| <a id="req-ds-fec-01"></a>REQ-DS-FEC-01 | MERLIN MUST retain current BCH/LDPC, shortening, NCP and PLC code definitions, outcomes and public codec imports. | [FEC ownership](architecture.md#unified-shared-dsp-and-fec); `crates/fec-codec/src/` and downstream transmission owners. | Existing `crates/fec-codec/tests/{normative,codec,shortening_codec,ncp,plc_decoding}.rs` plus downstream control-channel integration; INT-REG-01. Integration references: [INT-FEC-01](development/checklist.md#int-fec-01), [INT-REG-01](development/checklist.md#int-reg-01). Feature records: [fec/design.md](development/features/fec/design.md). |
| <a id="req-ds-phy-01"></a>REQ-DS-PHY-01 | Downstream QAM/LLR/bit and cell interleaving, randomization, pilots, packing, PLC/NCP and waveform processing MUST preserve exact supported behavior and grids. | [Downstream path](architecture.md#unified-downstream-processing); `crates/ofdm/src/{transmission,signal_processing}/`. | `crates/ofdm/tests/{payload_transport,waveform_integration,qam_mapping,receiver_configuration}.rs` and all relevant existing parameter suites; INT-REG-01. Integration references: [INT-BASE-01](development/checklist.md#int-base-01), [INT-REG-01](development/checklist.md#int-reg-01). Feature records: [signal-processing/design.md](development/features/signal-processing/design.md). |
| <a id="req-ds-run-01"></a>REQ-DS-RUN-01 | Existing source/receiver/channel jobs MUST retain truth separation, duration/startup/drain, bounded work, deterministic seeds and partial/cancelled outcomes. | [Lifecycle](architecture.md#lifecycle-validation-and-future-interfaces); `crates/ofdm/src/simulation/`. | Existing `crates/ofdm/tests/{channel_job,duration_timing,prbs_source,file_source_channel}.rs`; INT-REG-01. Integration references: [INT-REG-01](development/checklist.md#int-reg-01). |
| <a id="req-ds-input-01"></a>REQ-DS-INPUT-01 | Capture bytes/hashes/grids/nulls/profile identities, association, measurement policies and saved workbench compatibility MUST remain intact. | [PNM ownership](architecture.md#pnm-measurement-input-direction); `crates/pnm-input/src/`, downstream `scenario/` and `api/`. | Existing `crates/pnm-input/tests/{high_level_api,workbench,designer_workbench}.rs`; INT-REG-01. Integration references: [INT-REG-01](development/checklist.md#int-reg-01). |
| <a id="req-ds-api-01"></a>REQ-DS-API-01 | Existing Rust imports, Python exports, CLI syntax, report/schema defaults and saved recipes MUST remain compatible; shared capabilities MUST land in Rust first. | [Unified adapters](architecture.md#unified-rust-python-and-cli-apis); `crates/ofdm/src/api/`, `bindings/python`, `crates/cli`. | Existing native `bindings/python/tests/{test_api,test_parity,test_workbench,test_designer_workbench}.py`, API examples and CLI cases; INT-RUST-01, INT-PY-01, INT-CLI-01, INT-REG-01. Integration references: [INT-RUST-01](development/checklist.md#int-rust-01), [INT-PY-01](development/checklist.md#int-py-01), [INT-CLI-01](development/checklist.md#int-cli-01), [INT-REG-01](development/checklist.md#int-reg-01). Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md). |
| <a id="req-ds-ui-01"></a>REQ-DS-UI-01 | Backend integration MUST preserve existing Qt downstream presentation/readiness/persistence; it MUST NOT add upstream UI as a side effect. | [Qt boundary](architecture.md#ui-merlin-simulation-workbench); `apps/qt/`. | Relevant existing Qt tests/build only if integration changes their dependencies/contracts; INT-REG-01. Detailed UI status remains subordinate in `docs/development/ui-checklist.md`. Integration references: [INT-REG-01](development/checklist.md#int-reg-01). |
| <a id="req-ds-delivery-01"></a>REQ-DS-DELIVERY-01 | Existing VERSION/source pins, licenses, package targets, Master/Release security/native gates and generated Python installation guide policy MUST remain authoritative. | [Delivery](architecture.md#unified-normative-evidence-and-exclusions); current `tools/`, `.github/workflows/` and setup owners. | Existing tooling, version, both-lock audit, release/package/native gates when applicable; INT-LICENSE-01, INT-TOOLS-01, INT-ACCEPT-01. Integration references: [INT-LICENSE-01](development/checklist.md#int-license-01), [INT-TOOLS-01](development/checklist.md#int-tools-01), [INT-ACCEPT-01](development/checklist.md#int-accept-01). |

## Common consolidation requirements

| ID | Required behavior | Architecture / owner | Acceptance / task |
| --- | --- | --- | --- |
| <a id="req-com-reuse-01"></a>REQ-COM-REUSE-01 | Shared algorithms MUST be selected by exact semantic/equivalence and performance evidence; direction-specific normative state MUST remain separate. | [Shared DSP/FEC](architecture.md#unified-shared-dsp-and-fec); existing core owners and shared `crates/dsp` FFT/PRBS15 kernels. | Function/path reuse matrix, independent vectors, retained golden values, allocation/runtime measurements; INT-REUSE-01, INT-DSP-01, INT-FEC-01. Integration references: [INT-REUSE-01](development/checklist.md#int-reuse-01), [INT-FEC-01](development/checklist.md#int-fec-01), [INT-DSP-01](development/checklist.md#int-dsp-01). Feature records: [fec/design.md](development/features/fec/design.md), [signal-processing/design.md](development/features/signal-processing/design.md).  Shared DSP implementation: [transform](../crates/dsp/src/transform.rs), [PRBS15](../crates/dsp/src/prbs15.rs), [independent/equivalence tests](../crates/dsp/tests/equivalence.rs); [executed validation](development/features/signal-processing/validation.md). Directional channel/QAM/measurement owners remain separate. |
| <a id="req-com-identity-01"></a>REQ-COM-IDENTITY-01 | The integrated product MUST remain MERLIN with one CLI/Python family; former product crates/controllers/imports MUST NOT be introduced. Required source attribution and raw source metadata MUST remain accurate. | [Crates](architecture.md#unified-crate-ownership); local manifests, APIs, packaging metadata and NOTICE. | Source/import/export/package inventory plus downstream compatibility; INT-RUST-01, INT-PY-01, INT-CLI-01, INT-TOOLS-01. Integration references: [INT-CLI-01](development/checklist.md#int-cli-01). Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md). |
| <a id="req-com-doc-01"></a>REQ-COM-DOC-01 | One architecture, requirements baseline and official checklist MUST cover both directions with exact source/document/task mappings and final implementation evidence. | [Final documentation gate](architecture.md#unified-acceptance-and-final-documentation-gate); canonical docs and skills. | Complete source inventory, unique IDs, dependency/coverage/link checks and final module/API/runtime correspondence; INT-DOC-01, INT-DOC-02. Integration references: [INT-DOC-01](development/checklist.md#int-doc-01), [INT-DOC-02](development/checklist.md#int-doc-02), [INT-ACCEPT-01](development/checklist.md#int-accept-01). Documentation delivery: [DOC-ORG-01](development/checklist.md#doc-org-01). |

## Backend master prompt additions

The [current full master prompt](development/backend-integration-master-guide.md)
and [37-clause review](development/features/unified-backend/master-prompt-review.md)
add the following obligations without changing earlier requirement clauses or
accepted task states. These are planned delivery contracts, not current API exports.
The single [API compatibility matrix](development/features/high-level-api/compatibility-matrix.md)
records current inspected operations and remaining coverage explicitly.

| ID | Required behavior | Architecture / implementation owner | Acceptance / delivery |
| --- | --- | --- | --- |
| <a id="req-com-api-07"></a>REQ-COM-API-07 | Rust SHALL expose explicit downstream and upstream public namespaces with distinct directional configuration and compatible common lifecycle/error/report conventions. Preserve `docsis-ofdm-fec`, `docsis_ofdm_fec::api::{Merlin,Simulation}` and all existing downstream imports/behavior through compatible aliases or re-exports. Present the final namespace-to-crate mapping for approval before compatibility-sensitive export changes; conceptual names are not implemented symbols. | [API architecture](architecture.md#unified-rust-python-and-cli-apis); existing `crates/ofdm/src/api/` and `crates/ofdma/src/api/`; approved facade/common ownership is implemented, with complete operation coverage still gated. | [INT-PLAN-01](development/checklist.md#int-plan-01) records the proposal; [INT-RUST-01](development/checklist.md#int-rust-01) delivers it after upstream loop acceptance. Preserve `crates/ofdm/tests/public_paths.rs`; add direction import/alias/typed configuration tests at the approved paths and execute Rust-only and analysis-only regressions.  Selected architecture: [namespace design](development/features/high-level-api/namespace-design.md); [INT-API-MAP-01](development/checklist.md#int-api-map-01) records the exact mapping without accepting its implementation. |
| <a id="req-com-api-08"></a>REQ-COM-API-08 | One native Python distribution SHALL support standard `merlin.downstream` and `merlin.upstream` imports mirroring Rust direction separation while preserving root `merlin.Simulation`, `merlin.Merlin`, existing classes, signatures and saved-workbench semantics. Select real packages or native submodules using verified direct import, initialization, discovery, typing, wheel and installed-distribution behavior on declared Python 3.10+ environments. | [API architecture](architecture.md#unified-rust-python-and-cli-apis); existing `bindings/python/{pyproject.toml,src/}`; planned upstream native modules. All science remains in Rust. | [INT-PLAN-01](development/checklist.md#int-plan-01) records alternatives; [INT-PY-01](development/checklist.md#int-py-01) requires native import/discovery/alias/ownership/concurrency and deterministic parity cases plus installed wheel checks. Preserve `bindings/python/tests/{test_api,test_parity,test_workbench}.py`; new direction import tests remain planned.  Selected architecture: [namespace design](development/features/high-level-api/namespace-design.md); [INT-API-MAP-01](development/checklist.md#int-api-map-01) records the exact mapping without accepting its implementation. |
| <a id="req-com-api-09"></a>REQ-COM-API-09 | Maintain one authoritative operation-level API compatibility matrix with direction, operation, actual Rust namespace and exact signature, Python import and exact signature, input ownership, units, structured errors, lifecycle, report/schema, existing/new/alias compatibility, stable requirement/task, exact tests, executed evidence and explicit status. Never label hypothetical symbols or source-only passes as implemented/accepted MERLIN APIs. | [API architecture](architecture.md#unified-rust-python-and-cli-apis); [compatibility matrix](development/features/high-level-api/compatibility-matrix.md); existing export owners and planned upstream API. | [INT-PLAN-01](development/checklist.md#int-plan-01) establishes inspected inventory/seed rows and gaps. [INT-RUST-01](development/checklist.md#int-rust-01), [INT-PY-01](development/checklist.md#int-py-01) and [INT-DOC-02](development/checklist.md#int-doc-02) require every exported operation to have exact verified implementation/native test/evidence coverage.  Selected architecture: [namespace design](development/features/high-level-api/namespace-design.md); [INT-API-MAP-01](development/checklist.md#int-api-map-01) records the exact mapping without accepting its implementation.  Rust operation acceptance: [364-row data](development/features/high-level-api/operation-matrix.json) and [exact evidence](development/features/high-level-api/operation-evidence.json); native coverage remains separate. |
| <a id="req-com-trace-01"></a>REQ-COM-TRACE-01 | Every applicable backend requirement SHALL trace architecture → requirement → checklist → implementation → test → validation. Record stable ID, architecture owner, feature design, checklist task, crate, exact source file, symbol/stage, test file, exact case, validation command, observed result, commit SHA, status and limitations. Distinguish planned, implemented-unvalidated, integrated-validated, failed/blocked, deferred and historical-source evidence; paths alone never establish passes. Preserve existing evidence and identify its precision/coverage gaps. | [Traceability ownership](architecture.md#requirements-and-historical-evidence-ownership); existing source/backfill indexes and feature validation records; [record contract](development/features/unified-backend/master-prompt-review.md#implementation-evidence-contract). | [INT-PLAN-01](development/checklist.md#int-plan-01) maps every master clause and records gaps. Each implementation task fills its fourteen fields before acceptance; [INT-DOC-02](development/checklist.md#int-doc-02) and [INT-ACCEPT-01](development/checklist.md#int-accept-01) require complete backend coverage with executed unified-workspace evidence. Historical working-candidate SHA limitations must remain explicit. |
| <a id="req-com-trace-02"></a>REQ-COM-TRACE-02 | Extend existing validation tooling to reject missing owners, invalid/stale paths, missing public symbols/exact tests/executed evidence, broken bidirectional requirement/task/architecture links, duplicate IDs/invalid aliases, unmatched Rust/Python exports and completion/status inconsistent with actual implementation coverage. Preserve historical IDs/states; supporting indexes MUST NOT become a competing checklist. | [Validation ownership](architecture.md#unified-normative-evidence-and-exclusions); `tools/validation/{integration_traceability,feature_documentation}.py`, existing native API inventory tooling. | [INT-PLAN-01](development/checklist.md#int-plan-01) adds prompt/requirement/task coverage checks and an explicit remaining automation register. [INT-TOOLS-01](development/checklist.md#int-tools-01) implements all remaining listed checks with negative fixtures and native export/evidence validation; [INT-ACCEPT-01](development/checklist.md#int-accept-01) rejects unresolved mandatory coverage gaps. |

## Upstream and inherited common contracts

Each contract below retains the pinned source's observable behavior and acceptance
except the explicitly recorded local-policy adaptations. Source aliases and exact
source text remain fingerprinted/pinned; common contracts also bind existing
MERLIN behavior. Each new upstream consumer requires its indicated integration acceptance; current
completed/remaining states are owned by the official checklist.

### Shared source contracts

#### REQ-COM-SYS-03

Scientific algorithms, validation and orchestration SHALL execute in safe Rust independently of presentation or language runtimes. Adapters MUST NOT duplicate scientific calculations.

Acceptance: Core-only build/tests require neither Python nor Qt; adapters contain no DSP/FEC/association duplicates, native DSP/codec FFI or unsafe simulation code.

Architecture: [canonical owner](architecture.md#unified-crate-ownership).
Implementation: `crates/ofdma/src/lib.rs and core manifests`. Acceptance owner: `Rust-only dependency/build inspection and no-unsafe guards`.
Delivery: [**INT-REUSE-01**](development/checklist.md#int-reuse-01); source alias `REQ-SYS-03`.

Feature records: [signal-processing/design.md](development/features/signal-processing/design.md).

#### REQ-COM-SYS-05

Scientific claims SHALL identify pinned evidence and the applicable endpoint without confusing internal closed-loop evidence with external interoperability. Each stage SHALL have source-checked expected inputs/outputs recorded before its implementation, independently of production tables/algorithms.

Acceptance: Source/matrix/label checks record revision, section/page and source-review gaps; reports/docs label internal experiments accurately and retain optional-capability distinctions. Build these cases stage by stage; external interoperability fixtures and a complete upfront catalog are not prerequisites.

Architecture: [canonical owner](architecture.md#unified-normative-evidence-and-exclusions).
Implementation: `crates/fec-codec/src/upstream_ldpc/definitions.rs`. Acceptance owner: [source audit](../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs).
Delivery: [**INT-FEC-01**](development/checklist.md#int-fec-01); source alias `REQ-SYS-05`.
Integration references: [INT-BASE-01](development/checklist.md#int-base-01).

Feature records: [fec/design.md](development/features/fec/design.md).

#### REQ-COM-OBS-01

Shared Rust QAM and SA instruments SHALL inspect bounded real stage observations at named TX/channel/RX reference planes.

Acceptance: Query results identify run, configuration, tap, grid/sample units and retention coverage; absent history remains unavailable; instrument enable/disable/query does not alter waveform, RNG or decoder behavior.

Architecture: [canonical owner](architecture.md#unified-channel-and-measurement-reference-planes).
Implementation: `crates/ofdma/src/instruments/metrics.rs`. Acceptance owner: `crates/ofdma/tests/matched_metrics.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-OBS-01`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-COM-OBS-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-COM-OBS-02

Scientific results SHALL distinguish measured observations, analytical/model inputs, receiver estimates and simulated FEC/source outcomes.

Acceptance: Reports preserve convergence/parity/known-zero/source-error distinctions, failed words, actual iterations, partial coverage and provenance; zero observed error uses an explicit status rather than fabricated finite RxMER or JSON infinity.

Architecture: [canonical owner](architecture.md#unified-channel-and-measurement-reference-planes).
Implementation: `crates/ofdma/src/instruments/metrics.rs`. Acceptance owner: `crates/ofdma/tests/matched_metrics.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-OBS-02`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-COM-OBS-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-COM-API-03

Both APIs SHALL return owned results and structured domain errors using native language conventions.

Acceptance: Error categories/details match without parsing messages; Python uses exceptions/None and checked integer/buffer conversions; outputs remain valid after later calls, input release and session disposal; zero-copy claims require lifetime proof.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/merlin/tests/namespaces.rs`.
Delivery: [**INT-RUST-01**](development/checklist.md#int-rust-01); source alias `REQ-API-03`.

Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md).

Rust namespace slice: [exact implementation/test record](development/features/high-level-api/namespace-evidence.json), keyed by `REQ-COM-API-03`; complete Rust operation coverage is accepted under INT-RUST-01; native coverage remains INT-PY-01.

#### REQ-COM-API-04

Native Python execution SHALL release the interpreter lock during long Rust work and serialize mutable session access with explicit Busy behavior.

Acceptance: Actual native concurrency tests demonstrate independent cancellation/progress access, deterministic Busy for competing calls and session restoration after normal/error/cancel exits; no Python callback executes inside core locks.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `bindings/python/tests/test_upstream_experiment.py (planned native acceptance)`.
Delivery: [**INT-PY-01**](development/checklist.md#int-py-01); source alias `REQ-API-04`.

Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md).

#### REQ-COM-API-05

Rust and Python contracts, examples and Python type declarations SHALL describe the same implemented capability set.

Acceptance: Independently executed bounded native runs match resolved settings, seeds, provenance, status, FEC outcomes and partial coverage with declared numerical tolerances; actual extension exports match `.pyi`/typing metadata and paired examples run.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `bindings/python/tests/test_upstream_experiment.py (planned native acceptance)`.
Delivery: [**INT-PY-01**](development/checklist.md#int-py-01); source alias `REQ-API-05`.

Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md).

#### REQ-COM-API-06

Exported/configuration documents SHALL use explicit independent versions and declared compatibility rules.

Acceptance: Unknown versions/typed fields reject or follow an explicit migration; no FEC report is reinterpreted as configuration/workbench; any later shared saved experiment runs through both APIs without dropped settings or manual conversion.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/merlin/tests/namespaces.rs`.
Delivery: [**INT-RUST-01**](development/checklist.md#int-rust-01); source alias `REQ-API-06`.

Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md).

Rust namespace slice: [exact implementation/test record](development/features/high-level-api/namespace-evidence.json), keyed by `REQ-COM-API-06`; complete Rust operation coverage is accepted under INT-RUST-01; native coverage remains INT-PY-01.

#### REQ-COM-LIFE-01

Runs SHALL freeze accepted settings with distinct run/configuration identities and deterministic independent source/noise state.

Acceptance: Repeated runs with equal seeds/settings reproduce declared deterministic outputs; observers/chunking do not affect science; stale results or cross-session IDs are rejected.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-LIFE-01`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-COM-LIFE-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-COM-LIFE-02

Cancellation/failure SHALL preserve completed science and identify incomplete or not-started work under bounded safe work quanta.

Acceptance: Stage-boundary and between-word cancellation tests retain actual completed results/counts, pending tails/ranges and partial status; no forced thread termination or invented completion is used.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-LIFE-02`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-COM-LIFE-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-COM-LIFE-03

Pause/resume, when exposed, SHALL preserve all scientific stream state and define paused-time and stop-from-paused semantics. Bounded cancellation and partial-result acceptance SHALL precede pause/resume delivery.

Acceptance: Tests retain PRBS/mask/permutation/Pre-EQ/RNG/history state and pending output across pause; controls remain unavailable in an API until its safe-boundary/lifecycle contract is implemented. First-loop readiness MUST NOT depend on pause/resume; later exposure requires state/time/stop-from-paused tests.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-LIFE-03`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-COM-LIFE-03`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-COM-NFR-01

Inputs, expanded plans, per-call work, cumulative runtime/memory, output queues and retained observations SHALL be bounded before allocation/execution.

Acceptance: Existing architecture budgets and independently checked overflow/exhaustion cases reject without state mutation; final cumulative job budgets are documented before execution support.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-NFR-01`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-COM-NFR-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-COM-NFR-02

Reusable code/tooling SHALL preserve Apache-2.0 ownership and all applicable third-party notices, licenses and provenance. Reviewed reference-code adaptations SHALL remain locally owned and MUST NOT introduce an external source-checkout or former-product runtime dependency.

Acceptance: Dependency/reuse/package review retains actual copied notices and modifications; GPL presentation code does not enter the independent scientific core; private PDFs/captures/build outputs remain excluded. Each adaptation retains source/revision/notices/modifications and passes independent upstream checks.

Architecture: [canonical owner](architecture.md#unified-crate-ownership).
Implementation: `NOTICE and local dependency manifests`. Acceptance owner: `source/dependency/license and package notice inspection`.
Delivery: [**INT-LICENSE-01**](development/checklist.md#int-license-01); source alias `REQ-NFR-02`.
Local adaptation: Replace prohibition on depending on MERLIN with prohibition on external checkout/former product dependencies; retain local crate dependencies and existing Apache/MIT/GPL boundaries.

#### REQ-COM-NFR-03

Development setup SHALL require explicit intent and provide read-only check/dry-run behavior on declared targets.

Acceptance: Mode/host/pin/path/bounds/failure tests pass; inspection creates no caches or installs; apply obtains only missing declared prerequisites with pinned local Rust/locked sources; real fresh provisioning is verified separately.

Architecture: [canonical owner](architecture.md#unified-normative-evidence-and-exclusions).
Implementation: `scripts/install-development.sh (retain current owner)`. Acceptance owner: `existing setup tooling checks plus explicitly authorized target provisioning`.
Delivery: [**INT-TOOLS-01**](development/checklist.md#int-tools-01); source alias `REQ-NFR-03`.

#### REQ-COM-NFR-04

Validation and delivery records SHALL state actual evidence and preserve reproducible local tooling.

Acceptance: Required local gate passes with downloaded source/toolchain caches retained and stable Cargo build outputs retained after testing; hosted/native/package/interoperability claims require actual corresponding runs; prepared save commands do not imply commit/push/release execution.

Architecture: [canonical owner](architecture.md#unified-acceptance-and-final-documentation-gate).
Implementation: `scripts/test-local.sh and existing build/release helpers`. Acceptance owner: `complete existing validation gates, native Python/CLI/PNM and final inventories`.
Delivery: [**INT-ACCEPT-01**](development/checklist.md#int-accept-01); source alias `REQ-NFR-04`.
Local adaptation: Retain stable Cargo outputs and caches under MERLIN policy; do not inherit mandatory source cleanup.

### Upstream-specific contracts

#### REQ-US-SYS-01

The toolkit SHALL run a bounded one-CM/one-channel upstream reference experiment from source bits through the actual TX, channel and RX, then compare recovered meaningful bits independently.

Acceptance: A completed and drained unity run recovers all generated meaningful bits with full contiguous coverage and zero mismatches; source counts, recovered counts and decoder flags are checked separately.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-SYS-01`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-SYS-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-SYS-02

Normal receiver inputs SHALL contain received samples and legitimate accepted receiver configuration. They SHALL NOT contain source truth or injected channel/noise truth.

Acceptance: Receiver admission tests exclude original bits, PRBS seed/ledger, injected H/C, RxMER targets and noise seeds/power; diagnostic truth is consumed only by separate comparisons.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-SYS-02`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-SYS-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-SYS-04

The toolkit SHALL distinguish representable settings, domain validity and executable support.

Acceptance: Unsupported optional modes remain inspectable with reasons; execution rejects them before consuming input, random state or source bits; recognized QAM/code identifiers alone never imply a complete coding chain.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-SYS-04`.

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-SYS-04`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CFG-01

US Configuration SHALL retain a discoverable profile/preset inventory and explicitly select a one-CM/one-channel configuration shared by both endpoints.

Acceptance: Rust inventory exposes every retained entry without hardcoded adapter catalogs; invalid selection preserves the prior accepted plan; inventory does not automatically create multi-preset trials.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-01`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CFG-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CFG-02

Configuration admission SHALL produce an immutable CMTS plan and separate accepted TX/RX views through shared validation.

Acceptance: Both views agree on permitted geometry/grant/pilot/PHY-seed settings while normal RX excludes source, C/update values and channel/noise truth; failure preserves accepted settings and readiness.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-02`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CFG-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CFG-03

Generic OFDMA JSON SHALL have an explicit format/schema version, named presets, selection and separated normative configuration versus simulation settings.

Acceptance: Schema/parser fixtures cover required/optional/null/default/unknown-field behavior, unsupported versions, bounded extension metadata and round trips; vendor IDs are optional provenance, never normative preset meanings.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-03`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CFG-03`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CFG-04

Configuration coverage SHALL account for every applicable field/option in the pinned MULPI OFDMA scope.

Acceptance: The field-level matrix records source path/page, endpoint, force/applicability, encoding/units/enums, JSON pointer, Rust owner, support state and independent fixtures; every inapplicable or unresolved row has an explicit disposition.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-04`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CFG-04`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CFG-05

Typed and document-based configurations SHALL use equivalent domain rules and bounded admission. The first reference loop SHALL use typed synthetic configuration; JSON and verified capture loaders SHALL follow its acceptance.

Acceptance: Invalid ranges, reserved values, inconsistent pilot/loading/geometry, associations and aggregate budgets reject atomically; valid synthetic configurations do not require an unverified capture decoder. Full applicable MULPI/JSON delivery remains required after the typed reference milestone.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-05`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CFG-05`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CFG-06

Example presets SHALL demonstrate generic configurations without claiming universal recommended modulation.

Acceptance: Reviewed 25/50 kHz, mixed-loading and zero-valued-minislot examples identify source versus engineering choices; ranging/probe examples explicitly state execution support; conditional/optional settings are not silently discarded.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-06`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CFG-06`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-PRBS-01

The source SHALL emit continuous deterministic PRBS15 using the architecture's recurrence, seed range and bit order, independently of PHY scrambling.

Acceptance: Literal initial vectors and the 32767-bit period pass for independently checked seeds; zero/out-of-range seeds reject; grants, words and chunk boundaries do not reseed the source.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-01`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-PRBS-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-PRBS-02

Source consumption SHALL track accepted meaningful payload with checked absolute positions and bounded chunks.

Acceptance: Fill/shortening/parity/pad/pilots consume no source bits; backpressure and rejected work do not advance the generator; chunked generation matches contiguous generation and overflow rejects before mutation.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-02`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-PRBS-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-PRBS-03

Verification SHALL compare actual recovered meaningful decisions using an independently owned expected stream and validated source association.

Acceptance: Wrong but valid codewords produce source errors even if parity passes; failures do not suppress available decisions; gaps/duplicates/reordering stop contiguous comparison explicitly without phase search or repair.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-03`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-PRBS-03`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-PRBS-04

Verification results SHALL distinguish generated, compared and unverified bits, coverage, mismatch counts and completion.

Acceptance: Zero compared bits yield unavailable BER; zero observed errors with partial/truncated/discontinuous coverage cannot claim complete recovery; retained results survive later operations.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-04`.

Feature records: [ofdma/configuration.md](development/features/ofdma/configuration.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-PRBS-04`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-01

Grant framing SHALL derive coded capacity and the upstream codeword sequence from accepted resources/loading while separating meaningful bytes, information-fill ones, shortening zeros, parity and zero grant pad.

Acceptance: Independent threshold/borrowing/boundary cases conserve bits and source consumption; RX derives the same layout from legitimate configuration; rejected plans preserve state.

Architecture: [canonical owner](architecture.md#us-tx-01-grant-framing-and-fec-selection).
Implementation: `crates/ofdma/src/grant/plan.rs`. Acceptance owner: `crates/ofdma/tests/grant_plan.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-01`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-02

Encoding SHALL use the selected source-verified upstream long/medium/short LDPC family with systematic order and exact shortening semantics. All three code families SHALL pass source audit and codec acceptance before dependent waveform implementation.

Acceptance: Independently checked matrix entries/parity and full/shortened words pass; only appended shortening-zero coordinates are omitted; no downstream BCH or assumed puncturing is added.

Architecture: [canonical owner](architecture.md#us-tx-02-ldpc-encoding-and-shortening).
Implementation: `crates/fec-codec/src/upstream_ldpc/codec.rs`. Acceptance owner: [source audit](../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs) and [codec cases](../crates/fec-codec/src/upstream_ldpc/tests.rs); [MERLIN results](development/features/unified-backend/fec-validation.md).
Delivery: [**INT-FEC-01**](development/checklist.md#int-fec-01); source alias `REQ-TX-02`.

Feature records: [fec/design.md](development/features/fec/design.md), [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

#### REQ-US-TX-03

PHY randomization SHALL use the defined 23-bit grant seed, first-bit/tap convention and per-grant reset over exactly the transmitted bit positions.

Acceptance: Literal mask vectors, chunk continuity and reset tests cover parity and final grant pad; omitted shortening and known pilots add no clocks; invalid/reordered input preserves state.

Architecture: [canonical owner](architecture.md#us-tx-03-grant-seeded-phy-randomization).
Implementation: `crates/ofdma/src/bit_processing/randomizer.rs`. Acceptance owner: `crates/ofdma/tests/randomization.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-03`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-03`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-04

QAM mapping SHALL form MSB-first cell words in accepted visit order using exact upstream labels, normalization and role-dependent loading.

Acceptance: Independent label/energy vectors cover the declared BPSK/QPSK through 4096-QAM scope including non-square orders; inactive resources consume no payload; unknown/unimplemented orders reject.

Architecture: [canonical owner](architecture.md#us-tx-04-cell-words-and-upstream-qam).
Implementation: `crates/ofdma/src/signal_processing/qam.rs`. Acceptance owner: `crates/ofdma/tests/qam_mapping.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-04`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-04`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-05

OFDMA framing SHALL apply the accepted pilot pattern, known signs, role/loading plan, block partitioning and joint interleaving/address order.

Acceptance: Independent masks, pilot vectors and permutation/bijection cases cover supported patterns, edge/body layout, exclusions, zero resources and boost exceptions; bit/cell totals are conserved and scaling occurs once.

Architecture: [canonical owner](architecture.md#us-tx-05-pilots-resources-and-joint-interleaving).
Implementation: `crates/ofdma/src/transmitter/framing.rs`. Acceptance owner: `crates/ofdma/tests/upstream_interleaving.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-05`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-05`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-06

Pre-EQ SHALL apply one accepted complex corrective bank after role scaling and before IFFT, preserving the distinction between C and injected H.

Acceptance: Independent normalization/phase/default/reset/replacement/adjustment cases pass; excluded carriers remain zero; stale bases/nonfinite/invalid banks reject; bank transitions occur only at accepted symbol boundaries.

Architecture: [canonical owner](architecture.md#us-tx-06-cm-pre-eq).
Implementation: `crates/ofdma/src/signal_processing/pre_eq.rs`. Acceptance owner: `crates/ofdma/tests/pre_eq.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-06`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md), [ofdma/pre-equalization.md](development/features/ofdma/pre-equalization.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-06`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-07

IFFT SHALL transform complete accepted 2K/50 kHz or 4K/25 kHz physical grids at 102.4 Msamples/s using the defined centered unitary convention.

Acceptance: Independent direct-sum, tone, impulse and energy checks establish sign/bin order/scaling; useful sample count and timing are exact; inactive bins and invalid geometry are checked.

Architecture: [canonical owner](architecture.md#us-tx-07-centered-unitary-ifft).
Implementation: `crates/ofdma/src/signal_processing/transform.rs`. Acceptance owner: `crates/ofdma/tests/transforms.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-07`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-07`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-TX-08

Sample assembly SHALL apply the specified CP/RP, cyclic extension, taper and overlap while retaining exact useful/prefix origins, burst edges, tails and explicit guards.

Acceptance: Independent numbered-sample/window tests cover zero/nonzero RP and adjacent symbols; complete output is invariant across chunks/backpressure; invalid geometry rejects and cancellation retains honest partial status.

Architecture: [canonical owner](architecture.md#us-tx-08-prefix-window-and-sample-assembly).
Implementation: `crates/ofdma/src/transmitter/assembly.rs`. Acceptance owner: `crates/ofdma/tests/tx_framing.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-08`.

Feature records: [ofdma/transmitter.md](development/features/ofdma/transmitter.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-TX-08`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CH-01

Channel admission SHALL validate finite indexed sample blocks, clock, identities, continuity and budgets before changing state.

Acceptance: Stale/duplicate/overlap/unexplained-gap/nonfinite/overflow inputs reject atomically; empty blocks are no-ops, not finish; unsupported composition rejects before TX consumption.

Architecture: [canonical owner](architecture.md#us-ch-01-indexed-sample-admission).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-01`.

Feature records: [ofdma/channel.md](development/features/ofdma/channel.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CH-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CH-02

Initial unity transport SHALL preserve sample bit patterns and metadata exactly without impairments or random draws.

Acceptance: Bitwise tests include signed zero and arbitrary chunking with unchanged indices/rate/burst/tail/guard/validity; empty/disabled impairment settings use the same bypass.

Architecture: [canonical owner](architecture.md#us-ch-02-exact-unity-transport).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-02`.

Feature records: [ofdma/channel.md](development/features/ofdma/channel.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CH-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CH-03

The response extension SHALL apply the declared causal synthetic complex FIR without implicit normalization, circular wrap or per-symbol resets.

Acceptance: Literal complex convolution/impulse/delay cases establish phase, original origin, state continuity and exact terminal drain; zero/deep-null responses remain valid destructive experiments.

Architecture: [canonical owner](architecture.md#us-ch-03-physical-channel-response).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-03`.

Feature records: [ofdma/channel.md](development/features/ofdma/channel.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CH-03`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CH-04

Residual synthesis SHALL offer explicit Off, sample-AWGN and restricted RxMER-derived modes using separate deterministic noise state and declared reference power.

Acceptance: Independent power/statistical/draw-order tests cover seeds, missing-carrier policy and pilot/role scaling; initial RxMER synthesis rejects nonunity C/H or incompatible timing across the complete schedule.

Architecture: [canonical owner](architecture.md#us-ch-04-awgn-and-restricted-rxmer-residuals).
Implementation: `crates/ofdma/src/channel/residual.rs`. Acceptance owner: `crates/ofdma/tests/rxmer_channel.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-04`.

Feature records: [ofdma/channel.md](development/features/ofdma/channel.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CH-04`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CH-05

The timing extension SHALL implement only the designed integer delay, constant phase and CFO with emitted-sample-clock continuity.

Acceptance: Independent impulse/tone/rotation cases establish signs/origin, chunk/pause continuity and exact delay drain; zero mode is bitwise bypass; fractional clock and stochastic phase noise remain unavailable.

Architecture: [canonical owner](architecture.md#us-ch-05-integer-timing-phase-and-cfo).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-05`.

Feature records: [ofdma/channel.md](development/features/ofdma/channel.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CH-05`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-CH-06

Channel output SHALL retain indexed samples and bounded passive observations while separating diagnostics from legitimate RX inputs.

Acceptance: Observer selection/capacity does not change samples or randomness; output validity/partial/drain metadata is retained; injected H/noise settings never become hidden RX corrections.

Architecture: [canonical owner](architecture.md#us-ch-06-output-and-passive-observations).
Implementation: `crates/ofdma/src/instruments/observer.rs`. Acceptance owner: `crates/ofdma/tests/matched_metrics.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-06`.

Feature records: [ofdma/channel.md](development/features/ofdma/channel.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-CH-06`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-01

Burst framing SHALL derive timing from accepted RX configuration and preserve actual indexed blocks and explicit missing intervals.

Acceptance: Literal origin/stride/window cases pass; unexplained gaps/reordering reject; declared gaps retain coverage loss rather than becoming available zero samples; no hidden timing search is performed.

Architecture: [canonical owner](architecture.md#us-rx-01-configured-burst-framing-and-timing).
Implementation: `crates/ofdma/src/receiver/framing.rs`. Acceptance owner: `crates/ofdma/tests/rx_framing.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-01`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-01`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-02

Useful-symbol extraction SHALL select exactly the scheduled N-sample windows without silently filling incomplete windows.

Acceptance: Numbered-sample/CP/overlap/truncation cases distinguish available zeros from missing samples and retain the correct symbol identities and unavailable outcomes.

Architecture: [canonical owner](architecture.md#us-rx-02-useful-symbol-extraction).
Implementation: `crates/ofdma/src/receiver/framing.rs`. Acceptance owner: `crates/ofdma/tests/rx_framing.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-02`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-02`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-03

FFT SHALL produce raw physical bins using the inverse of the accepted centered unitary TX convention.

Acceptance: Independent complex-sum, signed-tone, impulse and energy cases establish bin/sign/scaling correctness and preserve availability metadata without injecting channel truth.

Architecture: [canonical owner](architecture.md#us-rx-03-centered-unitary-fft).
Implementation: `crates/ofdma/src/signal_processing/transform.rs`. Acceptance owner: `crates/ofdma/tests/transforms.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-03`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-03`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-04

Channel estimation/equalization SHALL estimate effective C×H from received known pilots using the declared FrameFlatLs or FramePilotLinear policy and explicit zero-forcing validity.

Acceptance: Independent complex-anchor/interpolation/notch cases verify known pilot gain/sign treatment, estimate coverage and unavailable outputs; no hidden supplied-H, fast within-frame tracking or noise-estimation capability is claimed.

Architecture: [canonical owner](architecture.md#us-rx-04-received-pilot-estimation-and-equalization).
Implementation: `crates/ofdma/src/signal_processing/estimation.rs`. Acceptance owner: `crates/ofdma/tests/rx_estimation.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-04`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-04`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-05

Resource extraction SHALL independently invert the accepted resource/interleaving order and preserve unavailable scheduled cells and their bit positions.

Acceptance: Independent mixed-loading/permutation/boost cases conserve order/counts; missing cells are not compacted and known amplitude is removed once.

Architecture: [canonical owner](architecture.md#us-rx-05-resource-extraction-and-inverse-interleaving).
Implementation: `crates/ofdma/src/receiver/mod.rs`. Acceptance owner: `crates/ofdma/tests/waveform_loop.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-05`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-05`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-06

Soft demapping SHALL produce MSB-first finite MaxLog natural-log P(0)/P(1) metrics from shared upstream labels/scales and a valid ReferenceVariance.

Acceptance: Independent label/distance/sign tests cover positive-favors-zero convention, configured saturation, ties and zero-confidence erasures; nonpositive/nonfinite variance rejects.

Architecture: [canonical owner](architecture.md#us-rx-06-soft-qam-demapping).
Implementation: `crates/ofdma/src/signal_processing/demapping.rs`. Acceptance owner: `crates/ofdma/tests/qam_mapping.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-06`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-06`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-07

Soft derandomization SHALL independently regenerate the grant mask and reverse LLR signs before LDPC without changing magnitude or position.

Acceptance: Mask-one metrics reverse sign; erasures stay zero; every transmitted position including pad advances the mask, while omitted shortening/pilots do not; word ordering and grant resets remain exact.

Architecture: [canonical owner](architecture.md#us-rx-07-soft-phy-derandomization).
Implementation: `crates/ofdma/src/bit_processing/randomizer.rs`. Acceptance owner: `crates/ofdma/tests/randomization.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-07`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-07`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-RX-08

LDPC integration SHALL restore shortening exactly once through the existing codec contract and retain real decoder outcomes.

Acceptance: Retained-length inputs, independent shortening cases and 1..100 iteration bounds pass; convergence, syndrome, shortened-zero consistency, failed decisions and actual iterations remain distinct; controls act between indivisible codec calls.

Architecture: [canonical owner](architecture.md#us-rx-08-shortening-restoration-and-ldpc-decoding).
Implementation: `crates/fec-codec/src/upstream_ldpc/codec.rs`. Acceptance owner: [source audit](../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs) and [codec cases](../crates/fec-codec/src/upstream_ldpc/tests.rs); [MERLIN results](development/features/unified-backend/fec-validation.md).
Delivery: [**INT-FEC-01**](development/checklist.md#int-fec-01); source alias `REQ-RX-08`.

Feature records: [fec/design.md](development/features/fec/design.md), [ofdma/receiver.md](development/features/ofdma/receiver.md).

#### REQ-US-RX-09

Information deframing SHALL remove only receiver-known residual fill and retain actual complete/partial/failed meaningful decisions and coverage.

Acceptance: Independent byte/fill/short-word cases preserve positions and continuity; failures are not dropped; source association/comparison happens outside normal RX.

Architecture: [canonical owner](architecture.md#us-rx-09-information-deframing).
Implementation: `crates/ofdma/src/receiver/framing.rs`. Acceptance owner: `crates/ofdma/tests/rx_framing.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-09`.

Feature records: [ofdma/receiver.md](development/features/ofdma/receiver.md).

MERLIN execution: [exact implementation/test record](development/features/ofdma/implementation-evidence.json), keyed by `REQ-US-RX-09`; [83-case acceptance](development/features/ofdma/design.md#session-and-core-acceptance).

#### REQ-US-PNM-01

Imported observations SHALL preserve original bytes, hashes, timestamps, exact grids, nulls, identities, units and reference/provenance.

Acceptance: Original input remains recoverable after decode/selection/replay; malformed, incompatible and oversized inputs reject without replacing accepted state; upstream formats need pinned evidence and lawful fixtures.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-01`.
Integration references: [INT-PNM-01](development/checklist.md#int-pnm-01).

Feature records: [pnm-measurement-inputs/upstream.md](development/features/pnm-measurement-inputs/upstream.md).

MERLIN decoder slice: [exact implementation/test record](development/features/pnm-measurement-inputs/implementation-evidence.json), keyed by `REQ-US-PNM-01`; whole-plan acceptance is recorded under INT-PNM-02 in that same evidence inventory.

#### REQ-US-PNM-02

RxMER SHALL support explicit single selection, power-domain averaging and bounded scheduled hold/interpolation policies over multiple captures.

Acceptance: Independent conversions/weights/time-boundary/null cases match resolved power and coverage; original capture times stay unchanged; synthetic replay time is separate and importing more files does not create trials implicitly.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-02`.
Integration references: [INT-PNM-01](development/checklist.md#int-pnm-01).

Feature records: [ofdma/pre-equalization.md](development/features/ofdma/pre-equalization.md), [pnm-measurement-inputs/upstream.md](development/features/pnm-measurement-inputs/upstream.md).

MERLIN decoder slice: [exact implementation/test record](development/features/pnm-measurement-inputs/implementation-evidence.json), keyed by `REQ-US-PNM-02`; whole-plan acceptance is recorded under INT-PNM-02 in that same evidence inventory.

#### REQ-US-PNM-03

US-Pre-EQ SHALL support a single fixed bank, multiple snapshots and multiple explicitly based initialization/adjustment updates.

Acceptance: Independent complex/base/reset tests reject stale, missing or ambiguous dependencies; immutable schedules apply complete accepted banks only at defined symbol boundaries and preserve probe/source associations.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-03`.
Integration references: [INT-PNM-01](development/checklist.md#int-pnm-01).

Feature records: [ofdma/pre-equalization.md](development/features/ofdma/pre-equalization.md), [pnm-measurement-inputs/upstream.md](development/features/pnm-measurement-inputs/upstream.md).

MERLIN decoder slice: [exact implementation/test record](development/features/pnm-measurement-inputs/implementation-evidence.json), keyed by `REQ-US-PNM-03`; whole-plan acceptance is recorded under INT-PNM-02 in that same evidence inventory.

#### REQ-US-PNM-04

Combined input preflight SHALL validate the entire selected playback/update plan against channel/receiver support before execution.

Acceptance: A later incompatible Pre-EQ event fails preparation even if the initial bank is compatible; RxMER targets, corrective C and injected H remain separate; no unsupported composition is silently enabled.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-04`.

Feature records: [ofdma/pre-equalization.md](development/features/ofdma/pre-equalization.md), [pnm-measurement-inputs/upstream.md](development/features/pnm-measurement-inputs/upstream.md).

MERLIN execution: [exact implementation/test record](development/features/pnm-measurement-inputs/implementation-evidence.json), keyed by `REQ-US-PNM-04`; [association acceptance](development/features/pnm-measurement-inputs/upstream.md#measured-association-acceptance).

#### REQ-US-OBS-03

Future workbenches SHALL present PRBS, CM-US-TX, CH-SIM and CMTS-US-RX in path order with separate US Configuration and PNM Files inputs.

Acceptance: Stage navigation opens the corresponding defined stage; shared QAM/SA actions route to actual selected taps; source-only acceptance never marks the entire experiment ready.

Architecture: [canonical owner](architecture.md#unified-normative-evidence-and-exclusions).
Implementation: `apps/qt (upstream UI owner deferred; no files authorized)`. Acceptance owner: `future upstream UI interaction/accepted-state cases (deferred)`.
Delivery: [**INT-DEFER-01**](development/checklist.md#int-defer-01); source alias `REQ-OBS-03`.

#### REQ-US-OBS-04

Editors SHALL distinguish draft and accepted settings, keep loading/apply idle and preserve accepted state on cancel/invalid apply.

Acceptance: Accepted readiness uses text as well as color; preview consumes no active source/random state; displayed results distinguish complete, partial and discontinuous coverage.

Architecture: [canonical owner](architecture.md#unified-normative-evidence-and-exclusions).
Implementation: `apps/qt (upstream UI owner deferred; no files authorized)`. Acceptance owner: `future upstream UI interaction/accepted-state cases (deferred)`.
Delivery: [**INT-DEFER-01**](development/checklist.md#int-defer-01); source alias `REQ-OBS-04`.

#### REQ-US-API-01

The high-level Rust API SHALL expose typed idle session, configuration/inventory/selection, input admission, preview and explicit bounded execution while preserving existing FEC entrypoints. This Rust session SHALL be developed alongside the backend and used for reference unity-loop acceptance.

Acceptance: Direct Rust consumers exercise valid/invalid operations without CLI/Qt/Python; existing FEC imports, calls and schema-1 report semantics remain compatible.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/merlin/tests/namespaces.rs`.
Delivery: [**INT-RUST-01**](development/checklist.md#int-rust-01); source alias `REQ-API-01`.

Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md).

Rust namespace slice: [exact implementation/test record](development/features/high-level-api/namespace-evidence.json), keyed by `REQ-US-API-01`; complete Rust operation coverage is accepted under INT-RUST-01; native coverage remains INT-PY-01.

#### REQ-US-API-02

The optional native Python API SHALL wrap working public Rust operations with equivalent domain validation and results. Native Python implementation SHALL begin after the Rust unity loop passes acceptance.

Acceptance: Real native calls cover configuration, single/multi inputs and execution; no Python DSP/association/decoder implementation or subprocess sidecar is used; Rust-only dependency checks exclude binding runtimes. No early FEC-only Python wrapper is part of the selected delivery order.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `bindings/python/tests/test_upstream_experiment.py (planned native acceptance)`.
Delivery: [**INT-PY-01**](development/checklist.md#int-py-01); source alias `REQ-API-02`.

Feature records: [high-level-api/upstream.md](development/features/high-level-api/upstream.md).

## Detailed downstream contract register

Existing stage/task identifiers below are retained as requirement identities; they
are **not new functionality or renamed historical tasks**. Unnamed obligations
and the two different `UI-DOC-02` contracts receive distinct stable `REQ-DS-*`
identifiers; both original `UI-DOC-02` labels remain untouched as aliases.
The paired channel-composition and receiver-configuration labels remain aliases
of one contract each. Task text, nested acceptance, normative references and
limitations at each linked task/design are binding refinements; this register
does not discard them in favor of its short summaries.

“Historical checked” means the original task mark is preserved. Records identify
past execution within their stated platform, version and scope; tests identify
maintained acceptance locations, **not a new run**. “Gap” preserves that mark
while withholding traceability closure until the linked targeted task closes.
An open task or umbrella remains pending even when a constituent test passed.
Later refinements listed in the lifecycle column govern current controls, paths
and defaults; an earlier milestone is not a demand to restore obsolete behavior.
The 2026-11-08 checklist review and initial validation paragraphs are historical
snapshots; later task-specific records take precedence. Native/hosted execution
and installed lifecycle checks remain separate from local implementation checks.

The [review method and gaps](development/features/downstream-requirements/design.md)
and [machine-readable task/evidence index](development/features/downstream-requirements/traceability.json)
retain the baseline identity, source hashes, aliases and exact task occurrences.
Detailed UI refinements keep their status owner in the subordinate UI checklist.

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-dev-guide-01"></a>**DEV-GUIDE-01**: Incorporate reviewed foundation clarifications additively. | [DEV-GUIDE-01](development/checklist.md#task-dev-guide-01). Historical checked. | [architecture](architecture.md#engineering-guidance-ownership); [README.md](../skills/README.md), [SKILL.md](../skills/development-workflow/SKILL.md) | Record: [engineering-guidance/validation](development/features/engineering-guidance/validation.md). |


### Upstream skill migration

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-skill-us-01"></a>**SKILL-US-01**: Migrate pinned upstream guidance into existing MERLIN skills. | [SKILL-US-01](development/checklist.md#task-skill-us-01). Historical checked. | [architecture](architecture.md#upstream-guidance-migration); [SKILL.md](../skills/docsis-signal-processing/SKILL.md), [SKILL.md](../skills/scientific-testing/SKILL.md), [SKILL.md](../skills/public-api/SKILL.md) | Record: [upstream-skills/validation](development/features/upstream-skills/validation.md). |


### Architecture and module rules

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-arch-01"></a>**ARCH-01**: Maintain one canonical architecture and this checklist. | [ARCH-01](development/checklist.md#task-arch-01). Historical checked. | [architecture](architecture.md#requirements-and-historical-evidence-ownership); [architecture.md](architecture.md), [requirements.md](requirements.md), [checklist.md](development/checklist.md) | Record: [downstream-requirements/validation](development/features/downstream-requirements/validation.md). |
| <a id="ds-arch-02"></a>**ARCH-02**: Separate codec, DSP, transmission, simulation and adapter ownership. | [ARCH-02](development/checklist.md#task-arch-02). Historical checked; current evidence: [DS-GAP-01](development/checklist.md#ds-gap-01). | [architecture](architecture.md#modules-and-stage-boundaries); [lib.rs](../crates/ofdm/src/lib.rs), [lib.rs](../crates/fec-codec/src/lib.rs), [main.rs](../crates/cli/src/main.rs) | Tests: [public_paths.rs](../crates/ofdm/tests/public_paths.rs), [concatenated.rs](../crates/fec-codec/tests/concatenated.rs). Record: [source-organization/architecture](development/features/source-organization/architecture.md). Current scoped results: [baseline](development/features/unified-backend/baseline.md). |
| <a id="ds-arch-03"></a>**ARCH-03**: Record stage test-point IDs, metadata and bounded observation design. | [ARCH-03](development/checklist.md#task-arch-03). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [model.rs](../crates/ofdm/src/simulation/test_points/model.rs), [observer.rs](../crates/ofdm/src/simulation/test_points/observer.rs) | Tests: [test_point_observer.rs](../crates/ofdm/tests/test_point_observer.rs), [pilot_observation.rs](../crates/ofdm/tests/pilot_observation.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-arch-04"></a>**ARCH-04**: Implement the common typed test-point model and opt-in observer. | [ARCH-04](development/checklist.md#task-arch-04). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [model.rs](../crates/ofdm/src/simulation/test_points/model.rs), [observer.rs](../crates/ofdm/src/simulation/test_points/observer.rs) | Tests: [test_point_observer.rs](../crates/ofdm/tests/test_point_observer.rs), [pilot_observation.rs](../crates/ofdm/tests/pilot_observation.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-arch-05"></a>**ARCH-05**: Implement bounded stage/range selection and export through an adapter. | [ARCH-05](development/checklist.md#task-arch-05). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [export.rs](../crates/ofdm/src/simulation/test_points/export.rs), [channel.rs](../crates/cli/src/commands/channel.rs) | Tests: [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-arch-06"></a>**ARCH-06**: Prove probes do not change RNG draws, state, outputs or decisions. | [ARCH-06](development/checklist.md#task-arch-06). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [observation.rs](../crates/ofdm/src/simulation/source/observation.rs), [observer.rs](../crates/ofdm/src/simulation/test_points/observer.rs) | Tests: [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs), [payload_observation.rs](../crates/ofdm/tests/payload_observation.rs). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-arch-07"></a>**ARCH-07**: Expose parity, twist and demultiplexing as separately callable stages, including reverse boundaries, while preserving the existing bit-processing API. | [ARCH-07](development/checklist.md#task-arch-07). Historical checked. | [architecture](architecture.md#modules-and-stage-boundaries); [mod.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/mod.rs) | Tests: [bit_interleaving.rs](../crates/ofdm/tests/bit_interleaving.rs), [bit_processing_boundaries.rs](../crates/ofdm/tests/bit_processing_boundaries.rs). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |


### Payload source

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-src-01"></a>**SRC-01**: Deterministic seeded payload source, separate from codecs. | [SRC-01](development/checklist.md#task-src-01). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [seeded.rs](../crates/ofdm/src/simulation/source/seeded.rs) | Tests: [prbs_source.rs](../crates/ofdm/tests/prbs_source.rs), [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs). Record: [prbs-source/validation](development/features/prbs-source/validation.md). |
| <a id="ds-src-02"></a>**SRC-02**: PRBS source with explicit polynomial, seed, ordering and continuity. | [SRC-02](development/checklist.md#task-src-02). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [prbs.rs](../crates/ofdm/src/simulation/source/prbs.rs) | Tests: [prbs_source.rs](../crates/ofdm/tests/prbs_source.rs), [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs). Record: [prbs-source/validation](development/features/prbs-source/validation.md#src-02--src-03-completed). |
| <a id="ds-src-03"></a>**SRC-03**: Independent PRBS recurrence vectors, full-period tests and recovered-stream comparison through actual waveform samples at both spacings. PRBS design records seed/bit order, bounds and the distinction between mathematical and external interoperability evidence. | [SRC-03](development/checklist.md#task-src-03). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [prbs.rs](../crates/ofdm/src/simulation/source/prbs.rs) | Tests: [prbs_source.rs](../crates/ofdm/tests/prbs_source.rs), [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs). Record: [prbs-source/validation](development/features/prbs-source/validation.md#src-02--src-03-completed). |
| <a id="ds-src-04"></a>**SRC-04**: Add a bounded exact-byte text/binary source beside PRBS15, with an explicit bit order and cursor; preserve PRBS/noise behavior. | [SRC-04](development/checklist.md#task-src-04). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [generator.rs](../crates/ofdm/src/simulation/source/file/generator.rs), [verification.rs](../crates/ofdm/src/simulation/source/file/verification.rs) | Tests: [file_source.rs](../crates/ofdm/tests/file_source.rs), [file_source_channel.rs](../crates/ofdm/tests/file_source_channel.rs). Record: [file-payload-source/implementation](development/features/file-payload-source/implementation.md#src-0406-exact-byte-payload-source). |
| <a id="ds-src-05"></a>**SRC-05**: Define send-once EOF, final-codeword padding and meaningful-length accounting; support explicit total-bit targets with opt-in PRBS15 append-to-fill. Independently compare file/fill bits, exclude padding and preserve original bytes. | [SRC-05](development/checklist.md#task-src-05). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [generator.rs](../crates/ofdm/src/simulation/source/file/generator.rs), [verification.rs](../crates/ofdm/src/simulation/source/file/verification.rs) | Tests: [file_source.rs](../crates/ofdm/tests/file_source.rs), [file_source_channel.rs](../crates/ofdm/tests/file_source_channel.rs). Record: [file-payload-source/implementation](development/features/file-payload-source/implementation.md). |
| <a id="ds-src-06"></a>**SRC-06**: Expose source selection through shared Rust/CLI configuration; test text/binary byte preservation, size checks, optional PRBS fill, empty/oversized inputs, EOF, truncation and cancellation; keep noise/source state independent. See the file-source design, task design, implementation and tests and source/workspace plan. | [SRC-06](development/checklist.md#task-src-06). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [generator.rs](../crates/ofdm/src/simulation/source/file/generator.rs), [verification.rs](../crates/ofdm/src/simulation/source/file/verification.rs) | Tests: [file_source.rs](../crates/ofdm/tests/file_source.rs), [file_source_channel.rs](../crates/ofdm/tests/file_source_channel.rs). Record: [file-payload-source/implementation](development/features/file-payload-source/implementation.md). |


### Forward error correction

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-fec-01"></a>**FEC-01**: Rust BCH14232/14400 and short16200 rate8/9 LDPC encode/decode. | [FEC-01](development/checklist.md#task-fec-01). Historical checked; current evidence: [DS-GAP-02](development/checklist.md#ds-gap-02). | [architecture](architecture.md#purpose-and-scope); [bch.rs](../crates/fec-codec/src/bch.rs), [ldpc.rs](../crates/fec-codec/src/ldpc.rs), [concatenated.rs](../crates/fec-codec/src/concatenated.rs) | Tests: [codec.rs](../crates/fec-codec/tests/codec.rs), [normative.rs](../crates/fec-codec/tests/normative.rs), [concatenated.rs](../crates/fec-codec/tests/concatenated.rs). Record: [rust-only-signal-processing/architecture](development/features/rust-only-signal-processing/architecture.md). Current scoped results: [baseline](development/features/unified-backend/baseline.md). |
| <a id="ds-fec-02"></a>**FEC-02**: Concatenated codec results, payload truth comparison and actual LDPC iterations, with correction/failure and zero-error tests. | [FEC-02](development/checklist.md#task-fec-02). Historical checked; current evidence: [DS-GAP-02](development/checklist.md#ds-gap-02). | [architecture](architecture.md#purpose-and-scope); [bch.rs](../crates/fec-codec/src/bch.rs), [ldpc.rs](../crates/fec-codec/src/ldpc.rs), [concatenated.rs](../crates/fec-codec/src/concatenated.rs) | Tests: [codec.rs](../crates/fec-codec/tests/codec.rs), [normative.rs](../crates/fec-codec/tests/normative.rs), [concatenated.rs](../crates/fec-codec/tests/concatenated.rs). Record: [rust-only-signal-processing/architecture](development/features/rust-only-signal-processing/architecture.md). Current scoped results: [baseline](development/features/unified-backend/baseline.md). |
| <a id="ds-fec-03"></a>**FEC-03**: Implement PHY-verified downstream shortened codeword lengths and effective rates using the short16200-bit rate8/9 mother code; retain all BCH/LDPC parity and verify both TX/RX shortening boundaries independently. | [FEC-03](development/checklist.md#task-fec-03). Historical checked. | [architecture](architecture.md#downstream-fec-shortening-fec-03); [codec.rs](../crates/fec-codec/src/shortening/codec.rs), [parameters.rs](../crates/fec-codec/src/shortening/parameters.rs) | Tests: [shortening_geometry.rs](../crates/fec-codec/tests/shortening_geometry.rs), [shortening_correction.rs](../crates/fec-codec/tests/shortening_correction.rs), [shortening_codec.rs](../crates/fec-codec/tests/shortening_codec.rs). Record: [fec-shortening/validation](development/features/fec-shortening/validation.md#fec-03-validation). |


### Codeword bit processing

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-bits-01"></a>**BITS-01**: Uniform16/256-QAM parity/twist/demux and soft inverse. | [BITS-01](development/checklist.md#task-bits-01). Historical checked. | [architecture](architecture.md#modules-and-stage-boundaries); [mod.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/mod.rs) | Tests: [bit_interleaving.rs](../crates/ofdm/tests/bit_interleaving.rs), [bit_processing_boundaries.rs](../crates/ofdm/tests/bit_processing_boundaries.rs). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-bits-02"></a>**BITS-02**: Non-shortened mixed24-column parity/twist ordering, bypassed demultiplexing and independently transcribed full-matrix permutation tests. | [BITS-02](development/checklist.md#task-bits-02). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [mixed.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/mixed.rs), [planner.rs](../crates/ofdm/src/transmission/packing/planner.rs) | Tests: [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs), [bit_interleaving.rs](../crates/ofdm/tests/bit_interleaving.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |


### QAM mapping and soft demapping

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qam-01"></a>**QAM-01**: Embedded label-ordered mapping and max-log LLRs through16384-QAM. | [QAM-01](development/checklist.md#task-qam-01). Historical checked; current evidence: [DS-GAP-02](development/checklist.md#ds-gap-02). | [architecture](architecture.md#qam-coding-and-diagnostic-decisions-qam-0203); [qam.rs](../crates/ofdm/src/signal_processing/modulation/qam.rs), [soft_bits.rs](../crates/ofdm/src/signal_processing/modulation/soft_bits.rs) | Tests: [qam_mapping.rs](../crates/ofdm/tests/qam_mapping.rs), [qam_reference_tables.rs](../crates/ofdm/tests/qam_reference_tables.rs). Record: [qam-mapping/architecture](development/features/qam-mapping/architecture.md). Current scoped results: [baseline](development/features/unified-backend/baseline.md). |
| <a id="ds-qam-02"></a>**QAM-02**: Standalone hard slicer with deterministic ties, nearest label, reference I/Q and error/distance output; keep LDPC soft inputs unchanged. | [QAM-02](development/checklist.md#task-qam-02). Historical checked. | [architecture](architecture.md#qam-coding-and-diagnostic-decisions-qam-0203); [mod.rs](../crates/ofdm/src/signal_processing/modulation/slicing/mod.rs) | Tests: [qam_slicing.rs](../crates/ofdm/tests/qam_slicing.rs). Record: [qam-completion/validation](development/features/qam-completion/validation.md#qam-0203-validation). |
| <a id="ds-qam-03"></a>**QAM-03**: Complete coding/placement support beyond uniform16/256-QAM, including verified per-order padding, shortening and non-square rules. | [QAM-03](development/checklist.md#task-qam-03). Historical checked. | [architecture](architecture.md#qam-coding-and-diagnostic-decisions-qam-0203); [parameters.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/parameters.rs), [variable.rs](../crates/ofdm/src/signal_processing/receiver/payload/variable.rs) | Tests: [qam_coding_orders.rs](../crates/ofdm/tests/qam_coding_orders.rs), [shortened_transport.rs](../crates/ofdm/tests/shortened_transport.rs), [uniform_profile_orders.rs](../crates/ofdm/tests/uniform_profile_orders.rs). Record: [qam-completion/validation](development/features/qam-completion/validation.md). |
| <a id="ds-qam-04"></a>**QAM-04**: Validate full/shortened/mixed QAM within MERLIN's closed loop. | [QAM-04](development/checklist.md#task-qam-04). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [job.rs](../crates/ofdm/src/simulation/channel/job.rs), [reference.rs](../crates/ofdm/src/simulation/payload/reference.rs) | Tests: [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs), [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs). Record: [closed-loop-validation/validation](development/features/closed-loop-validation/validation.md). |


### Data and NCP randomization

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-rand-01"></a>**RAND-01**: Stateful data/NCP cell randomization and soft inverse, including pilot skipping, zero-loaded clocking and128-symbol reset tests. | [RAND-01](development/checklist.md#task-rand-01). Historical checked; current evidence: [DS-GAP-02](development/checklist.md#ds-gap-02). | [architecture](architecture.md#transmission-and-receiver-flow); [cell.rs](../crates/ofdm/src/signal_processing/modulation/randomization/cell.rs) | Tests: [cell_randomizer.rs](../crates/ofdm/tests/cell_randomizer.rs). Record: [cell-randomizer/architecture](development/features/cell-randomizer/architecture.md). Current scoped results: [baseline](development/features/unified-backend/baseline.md). |


### Codeword packing

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-pack-01"></a>**PACK-01**: Bounded256-QAM queue, continuation, filler and NCP reservations. | [PACK-01](development/checklist.md#task-pack-01). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [packer.rs](../crates/ofdm/src/transmission/packing/packer.rs), [planner.rs](../crates/ofdm/src/transmission/packing/planner.rs) | Tests: [symbol_packing.rs](../crates/ofdm/tests/symbol_packing.rs), [packed_noiseless.rs](../crates/ofdm/tests/packed_noiseless.rs), [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs). Record: [symbol-packing/architecture](development/features/symbol-packing/architecture.md), [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |
| <a id="ds-pack-02"></a>**PACK-02**: Bounded coded-bit packing against inverse-frequency profiles, variable-width mapping, zero loading and last-cell padding without TX IDs in RX. | [PACK-02](development/checklist.md#task-pack-02). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [packer.rs](../crates/ofdm/src/transmission/packing/packer.rs), [planner.rs](../crates/ofdm/src/transmission/packing/planner.rs) | Tests: [symbol_packing.rs](../crates/ofdm/tests/symbol_packing.rs), [packed_noiseless.rs](../crates/ofdm/tests/packed_noiseless.rs), [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs). Record: [symbol-packing/architecture](development/features/symbol-packing/architecture.md), [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |
| <a id="ds-pack-03"></a>**PACK-03**: Gate simulation-only packer state in analysis builds; validate without warning suppression. Design. | [PACK-03](development/checklist.md#task-pack-03). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [mod.rs](../crates/ofdm/src/transmission/packing/mod.rs), [check-before-push.sh](../scripts/check-before-push.sh) | Tests: [symbol_packing.rs](../crates/ofdm/tests/symbol_packing.rs). Record: [symbol-packing/design](development/features/symbol-packing/design.md#pack-03-analysis-only-packing-build-hygiene). |


### Next Codeword Pointer signaling

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ncp-01"></a>**NCP-01**: Real NCP messages, CRC-24-D, separate control LDPC and QAM grouping. | [NCP-01](development/checklist.md#task-ncp-01). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [codec.rs](../crates/fec-codec/src/ncp/codec.rs), [field.rs](../crates/ofdm/src/transmission/ncp/field.rs) | Tests: [ncp.rs](../crates/fec-codec/tests/ncp.rs), [ncp_field.rs](../crates/ofdm/tests/ncp_field.rs), [ncp_packed_noiseless.rs](../crates/ofdm/tests/ncp_packed_noiseless.rs). Record: [ncp-codec/architecture](development/features/ncp-codec/architecture.md). |
| <a id="ds-ncp-02"></a>**NCP-02**: CRC-accepted receiver pointer discovery without transmitter IDs. | [NCP-02](development/checklist.md#task-ncp-02). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [codec.rs](../crates/fec-codec/src/ncp/codec.rs), [field.rs](../crates/ofdm/src/transmission/ncp/field.rs) | Tests: [ncp.rs](../crates/fec-codec/tests/ncp.rs), [ncp_field.rs](../crates/ofdm/tests/ncp_field.rs), [ncp_packed_noiseless.rs](../crates/ofdm/tests/ncp_packed_noiseless.rs). Record: [ncp-codec/architecture](development/features/ncp-codec/architecture.md). |
| <a id="ds-ncp-03"></a>**NCP-03**: Opt-in bounded data/NCP generation switching: received DPD bank, C-selected data profiles0..15, exact128-symbol U announcement and N-selected positive uniform NCP changes. Preserve codeword/randomizer/interleaver state; verify real PLC DPD recovery and crossing-word payload recovery at both spacings. Same-parity generation replacement requires reacquisition; OPT tagging and dynamic shortened words are excluded. The duration CLI remains fixed-profile. Guide: dynamic profiles. | [NCP-03](development/checklist.md#task-ncp-03). Historical checked. | [architecture](architecture.md#completion-group-dynamic-control-layout-acquisition-and-delivery); [activation.rs](../crates/ofdm/src/transmission/ncp/activation.rs), [generations.rs](../crates/ofdm/src/transmission/plc/receiver_configuration/generations.rs) | Tests: [dynamic_profiles.rs](../crates/ofdm/tests/dynamic_profiles.rs). Record: [completion-group/validation](development/features/completion-group/validation.md). |


### Fixed channel and cell transport

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-cfg-01"></a>**CFG-01**: Fixed25/50kHz numerology, up-to192MHz setup, prefix/roll-off validation and integer one-second timing calculations. | [CFG-01](development/checklist.md#task-cfg-01). Historical checked; current evidence: [DS-GAP-02](development/checklist.md#ds-gap-02). | [architecture](architecture.md#waveform-direction-and-configuration); [settings.rs](../crates/ofdm/src/transmission/settings.rs), [timing.rs](../crates/ofdm/src/transmission/timing.rs) | Tests: [transmission.rs](../crates/ofdm/tests/transmission.rs), [duration_timing.rs](../crates/ofdm/tests/duration_timing.rs). Record: [single-channel-transmission/architecture](development/features/single-channel-transmission/architecture.md). Current scoped results: [baseline](development/features/unified-backend/baseline.md). |
| <a id="ds-layout-01"></a>**LAYOUT-01**: Explicit physical PLC/continuous/scattered/excluded roles. | [LAYOUT-01](development/checklist.md#task-layout-01). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [physical.rs](../crates/ofdm/src/transmission/layout/physical.rs), [pilot_reference.rs](../crates/ofdm/src/transmission/layout/pilot_reference.rs) | Tests: [carrier_layout.rs](../crates/ofdm/tests/carrier_layout.rs). Record: [carrier-layout/architecture](development/features/carrier-layout/architecture.md). |
| <a id="ds-layout-02"></a>**LAYOUT-02**: Reduced-domain association and128-symbol inverse pilot reference. | [LAYOUT-02](development/checklist.md#task-layout-02). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [physical.rs](../crates/ofdm/src/transmission/layout/physical.rs), [pilot_reference.rs](../crates/ofdm/src/transmission/layout/pilot_reference.rs) | Tests: [carrier_layout.rs](../crates/ofdm/tests/carrier_layout.rs). Record: [carrier-layout/architecture](development/features/carrier-layout/architecture.md). |
| <a id="ds-layout-03"></a>**LAYOUT-03**: Generate additional continuous pilots using PHY I20 density, merged spectrum, PLC/poor-bin avoidance, edge guards, isolated-region coverage and reproducible bounded disturbance. Validate both spacings and invalid inputs. Guide: pilot placement. | [LAYOUT-03](development/checklist.md#task-layout-03). Historical checked. | [architecture](architecture.md#completion-group-dynamic-control-layout-acquisition-and-delivery); [continuous.rs](../crates/ofdm/src/transmission/layout/continuous.rs) | Tests: [continuous_pilot_placement.rs](../crates/ofdm/tests/continuous_pilot_placement.rs). Record: [completion-group/validation](development/features/completion-group/validation.md). |
| <a id="ds-int-01"></a>**INT-01**: Frequency permutation and inverse, full/partial columns and both NI limits. | [INT-01](development/checklist.md#task-int-01). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [frequency.rs](../crates/ofdm/src/signal_processing/interleaving/frequency.rs), [time.rs](../crates/ofdm/src/signal_processing/interleaving/time.rs) | Tests: [cell_interleaving.rs](../crates/ofdm/tests/cell_interleaving.rs), [interleaved_noiseless.rs](../crates/ofdm/tests/interleaved_noiseless.rs). Record: [cell-interleaving/architecture](development/features/cell-interleaving/architecture.md). |
| <a id="ds-int-02"></a>**INT-02**: Stateful time interleaving and inverse at all supported depths, including dummy padding, M-1 pair delay, startup/drain and rejected-call atomicity. | [INT-02](development/checklist.md#task-int-02). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [frequency.rs](../crates/ofdm/src/signal_processing/interleaving/frequency.rs), [time.rs](../crates/ofdm/src/signal_processing/interleaving/time.rs) | Tests: [cell_interleaving.rs](../crates/ofdm/tests/cell_interleaving.rs), [interleaved_noiseless.rs](../crates/ofdm/tests/interleaved_noiseless.rs). Record: [cell-interleaving/architecture](development/features/cell-interleaving/architecture.md). |
| <a id="ds-int-03"></a>**INT-03**: Physical pilot-reference alignment over the complete128-symbol cycle. | [INT-03](development/checklist.md#task-int-03). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [frequency.rs](../crates/ofdm/src/signal_processing/interleaving/frequency.rs), [time.rs](../crates/ofdm/src/signal_processing/interleaving/time.rs) | Tests: [cell_interleaving.rs](../crates/ofdm/tests/cell_interleaving.rs), [interleaved_noiseless.rs](../crates/ofdm/tests/interleaved_noiseless.rs). Record: [cell-interleaving/architecture](development/features/cell-interleaving/architecture.md). |
| <a id="ds-cell-01"></a>**CELL-01**: Integration tests recovering real NCP and payloads at both spacings, maximum time depth and the randomizer reset, with zero payload bit errors. | [CELL-01](development/checklist.md#task-cell-01). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [frequency.rs](../crates/ofdm/src/signal_processing/interleaving/frequency.rs), [time.rs](../crates/ofdm/src/signal_processing/interleaving/time.rs) | Tests: [cell_interleaving.rs](../crates/ofdm/tests/cell_interleaving.rs), [interleaved_noiseless.rs](../crates/ofdm/tests/interleaved_noiseless.rs). Record: [cell-interleaving/architecture](development/features/cell-interleaving/architecture.md). |
| <a id="ds-cell-02"></a>**CELL-02**: Promote test-only data/NCP/filler assembly and receive orchestration into reusable production modules with selected test points. | [CELL-02](development/checklist.md#task-cell-02). Historical checked. | [architecture](architecture.md#production-fixed-profile-payload-transport); [transmitter.rs](../crates/ofdm/src/simulation/payload/transmitter.rs), [collector.rs](../crates/ofdm/src/signal_processing/receiver/payload/collector.rs) | Tests: [payload_transport.rs](../crates/ofdm/tests/payload_transport.rs), [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs). Record: [payload-transport/validation](development/features/payload-transport/validation.md#cell-02--wave-rx-06-validation). |


### Pilot and PLC signal stages

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-plc-01"></a>**PLC-01**: Confirm local PHY3.1 I20 section7.5.15 pilot requirements and section7.5.13 PLC requirements; record endpoint, sequence/bit order, scaling, phase and exact section/page evidence. Check PHY4.0 delegation where applicable. | [PLC-01](development/checklist.md#task-plc-01). Historical checked. | [architecture](architecture.md#stage-references); [plc-evidence.md](reference/plc-evidence.md), [sequence.rs](../crates/ofdm/src/signal_processing/pilots/sequence.rs) | Tests: [pilot_sequence.rs](../crates/ofdm/tests/pilot_sequence.rs). Record: [reference/plc-evidence](reference/plc-evidence.md), [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-plc-02"></a>**PLC-02**: Define typed pilot sequence/value and PLC stage results, with shared test-point metadata and bounded optional observation. | [PLC-02](development/checklist.md#task-plc-02). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [mod.rs](../crates/ofdm/src/signal_processing/pilots/mod.rs), [observation.rs](../crates/ofdm/src/simulation/pilots/observation.rs) | Tests: [pilot_sequence.rs](../crates/ofdm/tests/pilot_sequence.rs), [pilot_power.rs](../crates/ofdm/tests/pilot_power.rs), [pilot_signals.rs](../crates/ofdm/tests/pilot_signals.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-plc-03"></a>**PLC-03**: Implement shared pilot sequence logic in its own module and verify independent sequence/phase vectors. | [PLC-03](development/checklist.md#task-plc-03). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [mod.rs](../crates/ofdm/src/signal_processing/pilots/mod.rs), [observation.rs](../crates/ofdm/src/simulation/pilots/observation.rs) | Tests: [pilot_sequence.rs](../crates/ofdm/tests/pilot_sequence.rs), [pilot_power.rs](../crates/ofdm/tests/pilot_power.rs), [pilot_signals.rs](../crates/ofdm/tests/pilot_signals.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-plc-04"></a>**PLC-04**: Implement continuous-pilot signal generation separately from additional-pilot placement; expose signs, I/Q, boost and locations. | [PLC-04](development/checklist.md#task-plc-04). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [mod.rs](../crates/ofdm/src/signal_processing/pilots/mod.rs), [observation.rs](../crates/ofdm/src/simulation/pilots/observation.rs) | Tests: [pilot_sequence.rs](../crates/ofdm/tests/pilot_sequence.rs), [pilot_power.rs](../crates/ofdm/tests/pilot_power.rs), [pilot_signals.rs](../crates/ofdm/tests/pilot_signals.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-plc-05"></a>**PLC-05**: Implement scattered-pilot signal generation separately, using the existing physical pattern and correct PLC-aligned phase after interleaving. | [PLC-05](development/checklist.md#task-plc-05). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [mod.rs](../crates/ofdm/src/signal_processing/pilots/mod.rs), [observation.rs](../crates/ofdm/src/simulation/pilots/observation.rs) | Tests: [pilot_sequence.rs](../crates/ofdm/tests/pilot_sequence.rs), [pilot_power.rs](../crates/ofdm/tests/pilot_power.rs), [pilot_signals.rs](../crates/ofdm/tests/pilot_signals.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-plc-06"></a>**PLC-06**: Implement shared pilot power/boost conversion with explicit reference energy; test signs, levels and collisions with protected roles. | [PLC-06](development/checklist.md#task-plc-06). Historical checked. | [architecture](architecture.md#test-points-and-data-access); [mod.rs](../crates/ofdm/src/signal_processing/pilots/mod.rs), [observation.rs](../crates/ofdm/src/simulation/pilots/observation.rs) | Tests: [pilot_sequence.rs](../crates/ofdm/tests/pilot_sequence.rs), [pilot_power.rs](../crates/ofdm/tests/pilot_power.rs), [pilot_signals.rs](../crates/ofdm/tests/pilot_signals.rs). Record: [pilot-plc-signals/architecture](development/features/pilot-plc-signals/architecture.md). |
| <a id="ds-plc-07a"></a>**PLC-07A**: Implement validated timestamp/MC message blocks, exact frame capacity and consecutive MSB-first36-byte/288-bit FEC inputs. | [PLC-07A](development/checklist.md#task-plc-07a). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [information.rs](../crates/ofdm/src/transmission/plc/information.rs), [message_channel.rs](../crates/ofdm/src/transmission/plc/message_channel.rs) | Tests: [plc_information.rs](../crates/ofdm/tests/plc_information.rs), [plc_message_channel.rs](../crates/ofdm/tests/plc_message_channel.rs). Record: [plc-message-blocks/architecture](development/features/plc-message-blocks/architecture.md). |
| <a id="ds-plc-07"></a>**PLC-07**: Complete fixed-setting PLC acquisition messages: OCD, Profile A and NCP DPD MAC packets, packet CRCs, segmentation and repetition scheduling. | [PLC-07](development/checklist.md#task-plc-07). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [preamble.rs](../crates/ofdm/src/transmission/plc/preamble.rs), [mapping.rs](../crates/ofdm/src/transmission/plc/mapping.rs), [mac_packets.rs](../crates/ofdm/src/transmission/plc/mac_packets.rs), [assembly.rs](../crates/ofdm/src/transmission/frame/assembly.rs), [observation.rs](../crates/ofdm/src/simulation/plc/observation.rs) | Tests: [plc_preamble.rs](../crates/ofdm/tests/plc_preamble.rs), [plc_frame_signals.rs](../crates/ofdm/tests/plc_frame_signals.rs), [plc_acquisition_roundtrip.rs](../crates/ofdm/tests/plc_acquisition_roundtrip.rs). Record: [plc-completion/validation](development/features/plc-completion/validation.md). |
| <a id="ds-plc-08"></a>**PLC-08**: Implement verified PLC coding as a separate stage; reuse a codec only where the PHY parameters match, otherwise add a focused Rust primitive. | [PLC-08](development/checklist.md#task-plc-08). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [encoding.rs](../crates/fec-codec/src/plc/encoding.rs), [decoding.rs](../crates/fec-codec/src/plc/decoding.rs), [coding.rs](../crates/ofdm/src/transmission/plc/coding.rs) | Tests: [plc_encoding.rs](../crates/fec-codec/tests/plc_encoding.rs), [plc_decoding.rs](../crates/fec-codec/tests/plc_decoding.rs), [plc_coding.rs](../crates/ofdm/tests/plc_coding.rs). Record: [plc-ldpc/architecture](development/features/plc-ldpc/architecture.md). |
| <a id="ds-plc-09"></a>**PLC-09**: Implement PLC block interleaving in its own module with independent permutation vectors and a coded-to-interleaved test point. | [PLC-09](development/checklist.md#task-plc-09). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [block_interleaving.rs](../crates/ofdm/src/transmission/plc/block_interleaving.rs) | Tests: [plc_block_interleaving.rs](../crates/ofdm/tests/plc_block_interleaving.rs), [plc_interleaved_codec.rs](../crates/ofdm/tests/plc_interleaved_codec.rs). Record: [plc-block-interleaving/architecture](development/features/plc-block-interleaving/architecture.md). |
| <a id="ds-plc-10"></a>**PLC-10**: Implement PLC randomization with independent state and its own phase/reset tests; do not reuse data/NCP randomizer state. | [PLC-10](development/checklist.md#task-plc-10). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [randomization.rs](../crates/ofdm/src/transmission/plc/randomization.rs) | Tests: [plc_randomization.rs](../crates/ofdm/tests/plc_randomization.rs), [plc_randomized_codec.rs](../crates/ofdm/tests/plc_randomized_codec.rs). Record: [plc-randomization/validation](development/features/plc-randomization/validation.md). |
| <a id="ds-plc-11"></a>**PLC-11**: Implement the PLC preamble in its own module with phase/sign tests. | [PLC-11](development/checklist.md#task-plc-11). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [preamble.rs](../crates/ofdm/src/transmission/plc/preamble.rs), [mapping.rs](../crates/ofdm/src/transmission/plc/mapping.rs), [mac_packets.rs](../crates/ofdm/src/transmission/plc/mac_packets.rs), [assembly.rs](../crates/ofdm/src/transmission/frame/assembly.rs), [observation.rs](../crates/ofdm/src/simulation/plc/observation.rs) | Tests: [plc_preamble.rs](../crates/ofdm/tests/plc_preamble.rs), [plc_frame_signals.rs](../crates/ofdm/tests/plc_frame_signals.rs), [plc_acquisition_roundtrip.rs](../crates/ofdm/tests/plc_acquisition_roundtrip.rs). Record: [plc-completion/validation](development/features/plc-completion/validation.md). |
| <a id="ds-plc-12"></a>**PLC-12**: Implement PLC QAM mapping separately with label/scale vectors. | [PLC-12](development/checklist.md#task-plc-12). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [preamble.rs](../crates/ofdm/src/transmission/plc/preamble.rs), [mapping.rs](../crates/ofdm/src/transmission/plc/mapping.rs), [mac_packets.rs](../crates/ofdm/src/transmission/plc/mac_packets.rs), [assembly.rs](../crates/ofdm/src/transmission/frame/assembly.rs), [observation.rs](../crates/ofdm/src/simulation/plc/observation.rs) | Tests: [plc_preamble.rs](../crates/ofdm/tests/plc_preamble.rs), [plc_frame_signals.rs](../crates/ofdm/tests/plc_frame_signals.rs), [plc_acquisition_roundtrip.rs](../crates/ofdm/tests/plc_acquisition_roundtrip.rs). Record: [plc-completion/validation](development/features/plc-completion/validation.md). |
| <a id="ds-plc-13"></a>**PLC-13**: Implement physical-bin assembly that combines interleaved cells, pilot values and PLC without moving protected bins or replacing placeholders early. | [PLC-13](development/checklist.md#task-plc-13). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [preamble.rs](../crates/ofdm/src/transmission/plc/preamble.rs), [mapping.rs](../crates/ofdm/src/transmission/plc/mapping.rs), [mac_packets.rs](../crates/ofdm/src/transmission/plc/mac_packets.rs), [assembly.rs](../crates/ofdm/src/transmission/frame/assembly.rs), [observation.rs](../crates/ofdm/src/simulation/plc/observation.rs) | Tests: [plc_preamble.rs](../crates/ofdm/tests/plc_preamble.rs), [plc_frame_signals.rs](../crates/ofdm/tests/plc_frame_signals.rs), [plc_acquisition_roundtrip.rs](../crates/ofdm/tests/plc_acquisition_roundtrip.rs). Record: [plc-completion/validation](development/features/plc-completion/validation.md). |
| <a id="ds-plc-14"></a>**PLC-14**: Add selectable outputs for pilot values, PLC bytes/coded bits/IQ and complete physical frequency bins; test with observation on and off. | [PLC-14](development/checklist.md#task-plc-14). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [preamble.rs](../crates/ofdm/src/transmission/plc/preamble.rs), [mapping.rs](../crates/ofdm/src/transmission/plc/mapping.rs), [mac_packets.rs](../crates/ofdm/src/transmission/plc/mac_packets.rs), [assembly.rs](../crates/ofdm/src/transmission/frame/assembly.rs), [observation.rs](../crates/ofdm/src/simulation/plc/observation.rs) | Tests: [plc_preamble.rs](../crates/ofdm/tests/plc_preamble.rs), [plc_frame_signals.rs](../crates/ofdm/tests/plc_frame_signals.rs), [plc_acquisition_roundtrip.rs](../crates/ofdm/tests/plc_acquisition_roundtrip.rs). Record: [plc-completion/validation](development/features/plc-completion/validation.md). |
| <a id="ds-plc-15"></a>**PLC-15**: Add module tests, a bounded example, integration tests and a user guide with equations/diagram. Mark this stage complete only after these pass. | [PLC-15](development/checklist.md#task-plc-15). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [preamble.rs](../crates/ofdm/src/transmission/plc/preamble.rs), [mapping.rs](../crates/ofdm/src/transmission/plc/mapping.rs), [mac_packets.rs](../crates/ofdm/src/transmission/plc/mac_packets.rs), [assembly.rs](../crates/ofdm/src/transmission/frame/assembly.rs), [observation.rs](../crates/ofdm/src/simulation/plc/observation.rs) | Tests: [plc_preamble.rs](../crates/ofdm/tests/plc_preamble.rs), [plc_frame_signals.rs](../crates/ofdm/tests/plc_frame_signals.rs), [plc_acquisition_roundtrip.rs](../crates/ofdm/tests/plc_acquisition_roundtrip.rs). Record: [plc-completion/validation](development/features/plc-completion/validation.md). |


### Receiver configuration from PLC

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-rx-config-01"></a>**RX-CONFIG-01**: Add receiver-owned state that consumes CRC-validated, reassembled OCD/ProfileA/NCP DPD descriptors and applies a consistent channel configuration. Preserve channel/profile/change identities; validate required descriptors before data/NCP recovery. Derive layout, interleaving and modulation from received signaling rather than copying transmitter settings. Keep supplied PLC bootstrap knowledge explicit. Test missing/corrupt/inconsistent messages, configuration readiness and unchanged valid state after rejection. | [RX-CONFIG-01](development/checklist.md#task-rx-config-01). Historical checked. | [architecture](architecture.md#production-fixed-profile-payload-transport); [mod.rs](../crates/ofdm/src/transmission/plc/receiver_configuration/mod.rs) | Tests: [receiver_configuration.rs](../crates/ofdm/tests/receiver_configuration.rs), [plc_receiver_acquisition.rs](../crates/ofdm/tests/plc_receiver_acquisition.rs). Record: [receiver-configuration/validation](development/features/receiver-configuration/validation.md#rx-config-01-validation). |


### Waveform transmitter: IFFT

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-tx-01"></a>**WAVE-TX-01**: Convert the complete physical frequency grid to time-domain I/Q in a separate IFFT module. Specify bin ordering, amplitude normalization, sample clock and sample indices. Verify independent impulse/tone and energy vectors at both FFT sizes, alongside the matched WAVE-RX-01 transform. | [WAVE-TX-01](development/checklist.md#task-wave-tx-01). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform transmitter: cyclic-prefix insertion

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-tx-02"></a>**WAVE-TX-02**: Insert the selected cyclic prefix in a focused TX operation. Users choose a PHY-permitted length before the run; validate and retain that configuration for the run. Copy the useful symbol's trailing samples to its front without changing the useful samples. Test every permitted prefix, invalid settings, output lengths and symbol boundaries at both spacings. | [WAVE-TX-02](development/checklist.md#task-wave-tx-02). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform transmitter: windowing and overlap-add

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-tx-03"></a>**WAVE-TX-03**: Keep TX windowing and stateful overlap-add in separate modules. Apply the selected roll-off and cyclic extension using pinned PHY rules. Verify taper samples, energy, continuity, symbol cadence, startup/drain tails and state across block boundaries. Roll-off must not add to symbol cadence. | [WAVE-TX-03](development/checklist.md#task-wave-tx-03). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform channel: reference connection

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-ch-01"></a>**WAVE-CH-01**: Connect TX time samples to RX time samples through a reference/noiseless channel module. Document input/output reference planes, sample clock, timing and gain. Verify unity transfer and block continuity; impairments remain separate MODEL work. | [WAVE-CH-01](development/checklist.md#task-wave-ch-01). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform receiver: FFT

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-rx-01"></a>**WAVE-RX-01**: Convert a useful received time-domain symbol to physical frequency-bin I/Q in a separate FFT module. Match the documented TX ordering and normalization. Test independent vectors and IFFT/FFT recovery at both sizes; a round trip alone is insufficient validation. | [WAVE-RX-01](development/checklist.md#task-wave-rx-01). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform receiver: cyclic-prefix removal

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-rx-02"></a>**WAVE-RX-02**: Remove the configured prefix from a correctly framed RX symbol, preserving exactly the useful FFT input. Check input length, boundaries and every permitted prefix at both spacings. Keep prefix removal separate from timing acquisition and from TX prefix insertion. | [WAVE-RX-02](development/checklist.md#task-wave-rx-02). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform receiver: timing and symbol framing

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-rx-03"></a>**WAVE-RX-03**: Recover symbol windows from the received stream using explicit known-reference timing first. Compose framing, prefix removal and FFT recovery without borrowing transmitter buffers or payload truth. Verify offsets, arbitrary input block splits, partial symbols and windowed boundaries. Known timing does not establish blind synchronization. | [WAVE-RX-03](development/checklist.md#task-wave-rx-03). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform receiver: signal extraction and equalization

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-rx-04"></a>**WAVE-RX-04**: Extract pilots and PLC from recovered bins and apply known-reference equalization first. Preserve frequency/role associations and amplitude units; feed actual received PLC cells into the existing decoder and receiver-configuration owner. Keep bootstrap settings explicit. | [WAVE-RX-04](development/checklist.md#task-wave-rx-04). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform receiver: nonideal acquisition and payload recovery

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-rx-05"></a>**WAVE-RX-05**: Complete bounded nonideal acquisition verification for the minimum single-channel engine, retaining explicit bootstrap/search assumptions. | [WAVE-RX-05](development/checklist.md#task-wave-rx-05). Historical checked. | [architecture](architecture.md#completion-group-dynamic-control-layout-acquisition-and-delivery); [search.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/search.rs) | Tests: [receiver_search.rs](../crates/ofdm/tests/receiver_search.rs), [nonideal_acquisition.rs](../crates/ofdm/tests/nonideal_acquisition.rs). Record: [receiver-acquisition/validation](development/features/receiver-acquisition/validation.md#wave-rx-05-validation). |
| <a id="ds-wave-rx-06"></a>**WAVE-RX-06**: Connect physical data extraction, inverse cell interleaving, source-phase inverse randomization, NCP discovery and data decoding in production. Use received configuration; preserve the separate control/payload codec paths. The original fixed uniform256-QAM ProfileA and positive uniform QPSK/16-QAM/64-QAM NCP now have live readiness checks, source/channel identities, explicit startup filler and M-1 drain, terminal rejection diagnostics and ten bounded test points. Tests include actual PLC acquisition then real sample-domain payload recovery at both spacings, all three NCP orders in cell transport, reset-crossing words, CRC/profile/continuation errors and observer invariance. Dynamic switching, mixed/zero DPD loading and blind acquisition remain outside this fixed subset. | [WAVE-RX-06](development/checklist.md#task-wave-rx-06). Historical checked. | [architecture](architecture.md#production-fixed-profile-payload-transport); [transmitter.rs](../crates/ofdm/src/simulation/payload/transmitter.rs), [collector.rs](../crates/ofdm/src/signal_processing/receiver/payload/collector.rs) | Tests: [payload_transport.rs](../crates/ofdm/tests/payload_transport.rs), [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs). Record: [payload-transport/validation](development/features/payload-transport/validation.md#cell-02--wave-rx-06-validation). |


### Waveform validation: observations

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-qa-01"></a>**WAVE-QA-01**: Add direction-tagged test points at each TX, channel and RX boundary, with explicit sample/bin units, finite block/range limits and direct library access. Prove observation does not change outputs or stage state. | [WAVE-QA-01](development/checklist.md#task-wave-qa-01). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Waveform validation: noiseless integration

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wave-qa-02"></a>**WAVE-QA-02**: Connect TX waveform generation, the noiseless channel and matched known-reference RX using real pilot/PLC signals. Verify recovered frequency bins numerically and actual decoded PLC information with zero bit errors at both spacings. Include prefix/window/overlap boundaries and retain decoder diagnostics. Document which payload/runner stages remain unfinished. | [WAVE-QA-02](development/checklist.md#task-wave-qa-02). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [mod.rs](../crates/ofdm/src/signal_processing/waveform/mod.rs), [receiver.rs](../crates/ofdm/src/simulation/waveform/receiver.rs), [observation.rs](../crates/ofdm/src/simulation/waveform/observation.rs) | Tests: [waveform_transforms.rs](../crates/ofdm/tests/waveform_transforms.rs), [waveform_prefix.rs](../crates/ofdm/tests/waveform_prefix.rs), [waveform_window.rs](../crates/ofdm/tests/waveform_window.rs), [waveform_integration.rs](../crates/ofdm/tests/waveform_integration.rs), [waveform_observation.rs](../crates/ofdm/tests/waveform_observation.rs). Record: [waveform/validation](development/features/waveform/validation.md). |


### Configurable-duration reference run

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-run-01"></a>**RUN-01**: Separate transmitter, receiver and runner modules; avoid moving numerical algorithms into a large orchestration function. | [RUN-01](development/checklist.md#task-run-01). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md#run-01--run-02-validation), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-02"></a>**RUN-02**: Stream a user-configured duration of one fixed channel, initially defaulting to one simulated second. Use checked64-bit sample accounting and bounded buffers for longer runs; never reset stage state at second boundaries. Keep acquisition, observation, queue completion, cell drain and final tail separate. `duration_timing`/`sample_timing` now provide64-bit plans; the legacy one-second helper delegates to them without changing its result contract. | [RUN-02](development/checklist.md#task-run-02). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md#run-01--run-02-validation), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-03"></a>**RUN-03**: Preserve partial codewords, interleaver history, PLC phase and sequence state across buffer boundaries; no per-codeword state resets. | [RUN-03](development/checklist.md#task-run-03). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-04"></a>**RUN-04**: Use source-symbol/sample identities for comparison and report complete, incomplete and excluded warm-up/drain payloads separately. Whole-word source-window classification retains crossing/outside counts, late contained recoveries, received incomplete starts and generated-unrecovered words. Window BER/rate use only contained comparisons; final rate is null for partial jobs. | [RUN-04](development/checklist.md#task-run-04). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-05"></a>**RUN-05**: Zero noiseless payload errors with actual NCP discovery; retain LDPC iterations, BCH outcomes, codeword errors and the number of compared bits. | [RUN-05](development/checklist.md#task-run-05). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-06"></a>**RUN-06**: Bound memory/runtime/probe output, support cancellation and retain completed simulation results, including an explicit partial status. | [RUN-06](development/checklist.md#task-run-06). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-07"></a>**RUN-07**: Expose the channel runner through one public Rust API and a thin CLI command with shared typed JSON results; preserve the earlier coding mode. | [RUN-07](development/checklist.md#task-run-07). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |
| <a id="ds-run-08"></a>**RUN-08**: Provide a reproducible PRBS-based example, math, assumptions and CLI/user documentation for the duration-based path and its initial one-second default. | [RUN-08](development/checklist.md#task-run-08). Historical checked. | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |


### Impairments and measurement-driven operation

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-input-01"></a>**INPUT-01**: Import current `pnm-decode` JSON for RxMER, downstream channel estimates and applied modulation profiles into the shared typed scenario model. Keep binary decoding external and direct Rust API inputs available; preserve source grids, nulls, timestamps and provenance. The CLI currently requires this repository's scenario JSON; `import-pnm` converts current decoder range-scheme documents using explicit timestamp tolerance and SHA-256 of the exact JSON inputs. | [INPUT-01](development/checklist.md#task-input-01). Historical checked. | [architecture](architecture.md#pnm-measurement-input-direction); [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs), [measurements.rs](../crates/ofdm/src/scenario/pnm_json/measurements.rs) | Tests: [pnm_json_import.rs](../crates/ofdm/tests/pnm_json_import.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [impairment-group/validation](development/features/impairment-group/validation.md), [completion-group/validation](development/features/completion-group/validation.md). |
| <a id="ds-req-ds-width-01"></a>**REQ-DS-WIDTH-01**: - [x] Validate synthetic24/90MHz inputs at25kHz with four captured-profile QAM orders, deterministic regeneration and eight noiseless waveform/FEC runs. See width design; measured selection remains ProfileA only and external/native validation stays pending. | [unnumbered task](development/checklist.md#task-req-ds-width-01). Historical checked. | [architecture](architecture.md#waveform-direction-and-configuration); [settings.rs](../crates/ofdm/src/transmission/settings.rs), [timing.rs](../crates/ofdm/src/transmission/timing.rs) | Tests: [channel_widths.rs](../crates/ofdm/tests/channel_widths.rs). Record: [channel-widths/design](development/features/channel-widths/design.md). |
| <a id="ds-input-02"></a>**INPUT-02**: Explicit decoder contract revision1 pins pnm-decode0.3.6; request schema2 requires the producer declaration. Preserve legacy schema1, exact-byte hashes/nulls/grids and shared Rust validation. Test actual exports, additive display fields and rejection of unsupported producer/revision drift. Guide: import contract. | [INPUT-02](development/checklist.md#task-input-02). Historical checked. | [architecture](architecture.md#pnm-measurement-input-direction); [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs), [measurements.rs](../crates/ofdm/src/scenario/pnm_json/measurements.rs) | Tests: [pnm_json_import.rs](../crates/ofdm/tests/pnm_json_import.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [impairment-group/validation](development/features/impairment-group/validation.md), [completion-group/validation](development/features/completion-group/validation.md). |
| <a id="ds-model-01"></a>**MODEL-01**: Controlled cell AWGN and RxMER-derived residual noise in the original coding experiment, distinct from measured observations and screening. | [MODEL-01](development/checklist.md#task-model-01). Historical checked. | [architecture](architecture.md#pnm-measurement-input-direction); [impairments.rs](../crates/ofdm/src/signal_processing/channel/impairments.rs), [residual.rs](../crates/ofdm/src/signal_processing/noise/residual.rs), [awgn.rs](../crates/ofdm/src/signal_processing/noise/awgn.rs) | Tests: [waveform_impairments.rs](../crates/ofdm/tests/waveform_impairments.rs), [lte_ingress.rs](../crates/ofdm/tests/lte_ingress.rs). Record: [impairment-group/validation](development/features/impairment-group/validation.md). |
| <a id="ds-model-02"></a>**MODEL-02**: Scenario identity/grid/time association and protected-role profiles. | [MODEL-02](development/checklist.md#task-model-02). Historical checked. | [architecture](architecture.md#temporal-measurements-and-applied-profiles); [association.rs](../crates/ofdm/src/scenario/association.rs), [evaluation.rs](../crates/ofdm/src/profile/evaluation.rs) | Tests: [documents.rs](../crates/ofdm/tests/documents.rs), [profile_lab.rs](../crates/ofdm/tests/profile_lab.rs). Record: [impairment-group/validation](development/features/impairment-group/validation.md). |
| <a id="ds-model-03"></a>**MODEL-03**: Analytical temporal screening and gross useful-symbol capacity. | [MODEL-03](development/checklist.md#task-model-03). Historical checked. | [architecture](architecture.md#temporal-measurements-and-applied-profiles); [association.rs](../crates/ofdm/src/scenario/association.rs), [evaluation.rs](../crates/ofdm/src/profile/evaluation.rs) | Tests: [documents.rs](../crates/ofdm/tests/documents.rs), [profile_lab.rs](../crates/ofdm/tests/profile_lab.rs). Record: [impairment-group/validation](development/features/impairment-group/validation.md). |
| <a id="ds-model-04"></a>**MODEL-04**: Verified waveform channel/impairment models with test points and explicit reference planes; validate against the noiseless baseline first. | [MODEL-04](development/checklist.md#task-model-04). Historical checked. | [architecture](architecture.md#pnm-measurement-input-direction); [impairments.rs](../crates/ofdm/src/signal_processing/channel/impairments.rs), [residual.rs](../crates/ofdm/src/signal_processing/noise/residual.rs), [awgn.rs](../crates/ofdm/src/signal_processing/noise/awgn.rs) | Tests: [waveform_impairments.rs](../crates/ofdm/tests/waveform_impairments.rs), [lte_ingress.rs](../crates/ofdm/tests/lte_ingress.rs). Record: [impairment-group/validation](development/features/impairment-group/validation.md). |


### Paired constellation before and after equalization

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-const-01"></a>**CONST-01**: Define typed selection and payload-only coordinate attribution through interleaving/NCP; default to8192 total QAM pairs for a supported profile. | [CONST-01](development/checklist.md#task-const-01). Historical checked. | [architecture](architecture.md#paired-prepost-equalization-constellation); [collector.rs](../crates/ofdm/src/simulation/channel/constellation/collector.rs) | Tests: [paired_constellation.rs](../crates/ofdm/tests/paired_constellation.rs). Record: [paired-constellation/implementation](development/features/paired-constellation/implementation.md). |
| <a id="ds-const-02"></a>**CONST-02**: Collect matched raw FFT/equalized I/Q with finite storage, exact carrier/time metadata, shared scaling and honest partial/exhausted status. | [CONST-02](development/checklist.md#task-const-02). Historical checked. | [architecture](architecture.md#paired-prepost-equalization-constellation); [collector.rs](../crates/ofdm/src/simulation/channel/constellation/collector.rs) | Tests: [paired_constellation.rs](../crates/ofdm/tests/paired_constellation.rs). Record: [paired-constellation/implementation](development/features/paired-constellation/implementation.md). |
| <a id="ds-const-03"></a>**CONST-03**: Expose one Rust result through the public API and opt-in channel CLI JSON; preserve existing input contracts and no-clobber/output limits. | [CONST-03](development/checklist.md#task-const-03). Historical checked. | [architecture](architecture.md#paired-prepost-equalization-constellation); [collector.rs](../crates/ofdm/src/simulation/channel/constellation/collector.rs) | Tests: [paired_constellation.rs](../crates/ofdm/tests/paired_constellation.rs). Record: [paired-constellation/implementation](development/features/paired-constellation/implementation.md). |
| <a id="ds-const-04"></a>**CONST-04**: Verify H/inverse reference planes, payload eligibility including failed words, startup/drain handling and unchanged simulation/RNG outcomes. | [CONST-04](development/checklist.md#task-const-04). Historical checked. | [architecture](architecture.md#paired-prepost-equalization-constellation); [collector.rs](../crates/ofdm/src/simulation/channel/constellation/collector.rs) | Tests: [paired_constellation.rs](../crates/ofdm/tests/paired_constellation.rs). Record: [paired-constellation/implementation](development/features/paired-constellation/implementation.md). |
| <a id="ds-const-05"></a>**CONST-05**: Provide a reproducible8192-pair demo, user instructions and actual validation evidence; update design/status after implementation. | [CONST-05](development/checklist.md#task-const-05). Historical checked. | [architecture](architecture.md#paired-prepost-equalization-constellation); [collector.rs](../crates/ofdm/src/simulation/channel/constellation/collector.rs) | Tests: [paired_constellation.rs](../crates/ofdm/tests/paired_constellation.rs). Record: [paired-constellation/implementation](development/features/paired-constellation/implementation.md). |


### Remaining receiver and measurement work

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-model-05"></a>**MODEL-05**: Applied downstream channel-estimate model and receiver estimation/ equalization with no double counting of post-equalization RxMER noise. | [MODEL-05](development/checklist.md#task-model-05). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs) | Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |
| <a id="ds-ch-est-rx-07"></a>**CH-EST-RX-07**: Define independent RX inputs, noise policy and bounded sample-filter contracts; audit all simulator-assisted receiver inputs. | [CH-EST-RX-07](development/checklist.md#task-ch-est-rx-07). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md). |
| <a id="ds-ch-est-rx-08"></a>**CH-EST-RX-08**: Add continuous synthetic echo/delay filtering with persistent sample history, explicit tail accounting and bounded efficient processing. | [CH-EST-RX-08](development/checklist.md#task-ch-est-rx-08). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md). |
| <a id="ds-ch-est-rx-09"></a>**CH-EST-RX-09**: Reconstruct a bounded causal filter from CM coefficients with explicit band/gap/delay policies and measured approximation errors. | [CH-EST-RX-09](development/checklist.md#task-ch-est-rx-09). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md). |
| <a id="ds-ch-est-rx-10"></a>**CH-EST-RX-10**: Validate/refine pilot estimation, timing, received-only reliability and deep-null policies against actual channel memory. | [CH-EST-RX-10](development/checklist.md#task-ch-est-rx-10). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md). |
| <a id="ds-ch-est-rx-11"></a>**CH-EST-RX-11**: Integrate shared Rust/CLI/WB settings and bounded applied-H versus estimated-H diagnostics without exposing truth to receiver decisions. MCE defaults to Off, with enablement/settings in Receiver Configuration; preserve saved modes and validate Apply/Cancel, persistence and next-run use. Hide the direct supplied-H diagram branch for MCE; retain channel paths. Label response Display choices using the retained MCE or legacy pilot result. Verify supplied H impairs both MCE states identically; Off uses reference correction and On uses received-pilot estimates without supplied H in RX. | [CH-EST-RX-11](development/checklist.md#task-ch-est-rx-11); [CH-EST-RX-11](development/ui-checklist.md#task-ch-est-rx-11). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md). |
| <a id="ds-ch-est-rx-12"></a>**CH-EST-RX-12**: Verify analytic echoes, real waveform/FEC recovery, input isolation, resource limits and unchanged legacy modes; document scope. | [CH-EST-RX-12](development/checklist.md#task-ch-est-rx-12). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md). |
| <a id="ds-model-06"></a>**MODEL-06**: Measurement-driven channel runs using the applied modulation profile, missing-carrier policy, temporal frames and actual FEC outcomes. | [MODEL-06](development/checklist.md#task-model-06). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [mixed.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/mixed.rs), [planner.rs](../crates/ofdm/src/transmission/packing/planner.rs) | Tests: [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs), [bit_interleaving.rs](../crates/ofdm/tests/bit_interleaving.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |
| <a id="ds-model-07"></a>**MODEL-07**: Validate any temporal/burst model independently; RxMER alone does not supply time correlation, impulse noise or modem implementation behavior. | [MODEL-07](development/checklist.md#task-model-07). Historical checked. | [architecture](architecture.md#transmission-and-receiver-flow); [mixed.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/mixed.rs), [planner.rs](../crates/ofdm/src/transmission/packing/planner.rs) | Tests: [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs), [bit_interleaving.rs](../crates/ofdm/tests/bit_interleaving.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |


### Project-local tooling

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qa-01"></a>**QA-01**: Project-local Cargo wrapper, version source and Git/release helpers. | [QA-01](development/checklist.md#task-qa-01). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh), [bump-version.sh](../tools/bump-version.sh) | Tests: [test_push.py](../tools/git/tests/test_push.py), [test_release.py](../tools/git/tests/test_release.py), [test_versions.py](../tools/release/test_versions.py). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |


### Local validation and simulation regression

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qa-02"></a>**QA-02**: Formatting, warnings-denied Clippy, workspace and analysis-only tests, release CLI build and smoke validation in the existing pre-push gate. | [QA-02](development/checklist.md#task-qa-02). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [check-before-push.sh](../scripts/check-before-push.sh), [test-local.sh](../scripts/test-local.sh) | Tests: [test_local.py](../tools/testing/test_local.py), [test_nextest.py](../tools/testing/test_nextest.py). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md), [closed-loop-validation/validation](development/features/closed-loop-validation/validation.md). |
| <a id="ds-qa-03"></a>**QA-03**: Deterministic numerical tests with pinned source evidence and independent fixed vectors where available. | [QA-03](development/checklist.md#task-qa-03). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [check-before-push.sh](../scripts/check-before-push.sh), [test-local.sh](../scripts/test-local.sh) | Tests: [test_local.py](../tools/testing/test_local.py), [test_nextest.py](../tools/testing/test_nextest.py). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md), [closed-loop-validation/validation](development/features/closed-loop-validation/validation.md). |


### Rust logging and diagnostics

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qa-08"></a>**QA-08**: Add standard Rust facade and CLI-owned stderr logging, explicit levels/error ownership and tests for filters, clean data output and simulation invariance; document the current instrumentation scope. | [QA-08](development/checklist.md#task-qa-08). Historical checked. | [architecture](architecture.md#diagnostic-logging); [runner.rs](../crates/ofdm/src/simulation/runner.rs), [main.rs](../crates/cli/src/main.rs) | Tests: [logging.rs](../crates/fec-codec/tests/logging.rs). Record: [rust-logging/architecture](development/features/rust-logging/architecture.md). |


### CLI and test-point validation

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qa-04"></a>**QA-04**: Extend the gate as new channel commands/test-point exports are added. Release-mode channel/JSONL smoke validation and actual CLI no-clobber/schema tests now accompany the existing coding CLI checks. | [QA-04](development/checklist.md#task-qa-04). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [main.rs](../crates/cli/src/main.rs), [check-before-push.sh](../scripts/check-before-push.sh) | Tests: [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [run-infrastructure/validation](development/features/run-infrastructure/validation.md). |


### Closed-loop end-to-end validation

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qa-05"></a>**QA-05**: Validate MERLIN TX/channel/RX against the original source. | [QA-05](development/checklist.md#task-qa-05). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [job.rs](../crates/ofdm/src/simulation/channel/job.rs), [reference.rs](../crates/ofdm/src/simulation/payload/reference.rs) | Tests: [payload_waveform.rs](../crates/ofdm/tests/payload_waveform.rs), [mixed_profile_transport.rs](../crates/ofdm/tests/mixed_profile_transport.rs). Record: [closed-loop-validation/validation](development/features/closed-loop-validation/validation.md). |


### Ubuntu builds and deferred cross-platform runtime

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-qa-06"></a>**QA-06**: Validate Ubuntu 22.04, 24.04 and 26.04 x64 in CI. Windows desktop validation is configured under UI-MSI-01; native evidence remains pending. macOS is deferred; Linux checks do not establish support. | [QA-06](development/checklist.md#task-qa-06). Pending / partial. | [architecture](architecture.md#completion-group-dynamic-control-layout-acquisition-and-delivery); [package.py](../tools/packaging/package.py), [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py). Record: [completion-group/validation](development/features/completion-group/validation.md). |


### Native packaging and release automation

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-install-examples-01"></a>**INSTALL-EXAMPLES-01**: Include the curated runnable WB inventory, PNM originals, hashes and notices in desktop installers; verify installed Open location, offline restore and relocation on each supported OS. Design. | [INSTALL-EXAMPLES-01](development/checklist.md#task-install-examples-01); [INSTALL-EXAMPLES-01](development/ui-checklist.md#task-install-examples-01). Pending / partial. | [architecture](architecture.md#curated-workbench-examples-and-open-location); [workbench](../examples/workbench), [example_directory.rs](../apps/qt/src/platform/example_directory.rs) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [workbench-examples/design](development/features/workbench-examples/design.md#install-examples-01-application-installer-contract-pending). |
| <a id="ds-ch-est-playback-01"></a>**CH-EST-PLAYBACK-01**: Define and validate varying channel-estimate collections before enabling the reserved multi-RxMER/multi-channel scenario. | [CH-EST-PLAYBACK-01](development/checklist.md#task-ch-est-playback-01); [CH-EST-PLAYBACK-01](development/ui-checklist.md#task-ch-est-playback-01). Pending / partial. | [architecture](architecture.md#purpose-and-scope); [lib.rs](../crates/ofdm/src/lib.rs) | Record: [visual-workspace/plan](development/features/visual-workspace/plan.md). |
| <a id="ds-qa-07"></a>**QA-07**: Native packaging and release automation after core functionality. | [QA-07](development/checklist.md#task-qa-07). Pending / partial. | [architecture](architecture.md#completion-group-dynamic-control-layout-acquisition-and-delivery); [package.py](../tools/packaging/package.py), [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py). Record: [completion-group/validation](development/features/completion-group/validation.md). |


### Optional Python adapter

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-api-01"></a>**API-01**: Deliver optional Python access to the existing Rust simulation engine. The 2026-11-14 handoff supersedes the original `MerlinSimulation` sketch; implementation is tracked by API-RUST-01 and API-PY-01 below. Current design, historical proposal. | [API-01](development/checklist.md#task-api-01). Historical checked; current refinement: [API-RUST-01](requirements.md#ds-api-rust-01), [API-PY-01](requirements.md#ds-api-py-01). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). |


### Qt simulation workbench

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-rf-02"></a>**UI-RF-02**: Expand the channel workbench to available window space with balanced edge margins, equal growing gaps and matching measurement branches. Design. | [UI-RF-02](development/checklist.md#task-ui-rf-02); [UI-RF-02](development/ui-checklist.md#task-ui-rf-02). Historical checked. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [power.rs](../crates/ofdm/src/signal_processing/channel/power.rs), [ChannelAmplifierDialog.qml](../apps/qt/qml/workbenches/channel/ChannelAmplifierDialog.qml) | Tests: [channel_power.rs](../crates/ofdm/tests/channel_power.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rf-power-budget/design](development/features/rf-power-budget/design.md#ui-rf-02-responsive-channel-workbench-spacing). |
| <a id="ds-ui-rf-01"></a>**UI-RF-01**: Implement amplifier/attenuator dialogs, scalar Rust conditioning, shared TP SA/QAM, optional electrical calibration/independent receiver noise and versioned WB persistence. Scoped tests and native VM checks pass. Design. | [UI-RF-01](development/checklist.md#task-ui-rf-01); [UI-RF-01](development/ui-checklist.md#task-ui-rf-01). Historical checked. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [power.rs](../crates/ofdm/src/signal_processing/channel/power.rs), [ChannelAmplifierDialog.qml](../apps/qt/qml/workbenches/channel/ChannelAmplifierDialog.qml) | Tests: [channel_power.rs](../crates/ofdm/tests/channel_power.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rf-power-budget/design](development/features/rf-power-budget/design.md#ui-rf-01-completed-validation). |
| <a id="ds-rf-power-01"></a>**RF-POWER-01**: Complete RF-port loading/capability, per-subband electrical limits and physical receiver overload/AGC modeling. Scalar conditioning, optional reference/noise, calibrated TP Channel Power and WB persistence are implemented by UI-RF-01;60 dBmV is not a universal normative CMTS ceiling. Design. | [RF-POWER-01](development/checklist.md#task-rf-power-01); [RF-POWER-01](development/ui-checklist.md#task-rf-power-01). Pending / partial. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [power.rs](../crates/ofdm/src/signal_processing/channel/power.rs), [ChannelAmplifierDialog.qml](../apps/qt/qml/workbenches/channel/ChannelAmplifierDialog.qml) | Tests: [channel_power.rs](../crates/ofdm/tests/channel_power.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rf-power-budget/design](development/features/rf-power-budget/design.md#rf-power-01-absolute-downstream-power-budget). |
| <a id="ds-ui-org-02"></a>**UI-ORG-02**: Group all QObject bridges, input models, workbench state, simulation lifecycle and platform integration; decouple saved-input restoration from Qt bridges. Design. | [UI-ORG-02](development/checklist.md#task-ui-org-02); [UI-ORG-02](development/ui-checklist.md#task-ui-org-02). Historical checked. | [architecture](architecture.md#reuse-of-the-pnm-viewer-qt-framework); [main.rs](../apps/qt/src/main.rs), [Main.qml](../apps/qt/qml/Main.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-source-organization/design](development/features/qt-source-organization/design.md#ui-org-02-completed-validation). |
| <a id="ds-ui-org-01"></a>**UI-ORG-01**: Group QML windows/components and Rust instrument providers by responsibility; update registration, imports, resource references and coding-style guidance. Design. | [UI-ORG-01](development/checklist.md#task-ui-org-01); [UI-ORG-01](development/ui-checklist.md#task-ui-org-01). Historical checked. | [architecture](architecture.md#reuse-of-the-pnm-viewer-qt-framework); [main.rs](../apps/qt/src/main.rs), [Main.qml](../apps/qt/qml/Main.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-source-organization/design](development/features/qt-source-organization/design.md#ui-org-01-organize-qt-presentation-sources). |
| <a id="ds-ui-instr-01"></a>**UI-INSTR-01**: Share one channel TP QAM presentation with tap-specific data providers. Instrument-folder organization is handled by UI-ORG-01. Plan. | [UI-INSTR-01](development/checklist.md#task-ui-instr-01); [UI-INSTR-01](development/ui-checklist.md#task-ui-instr-01). Historical checked. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [ChannelWorkbenchDialog.qml](../apps/qt/qml/workbenches/channel/ChannelWorkbenchDialog.qml), [composition.rs](../crates/ofdm/src/signal_processing/channel/composition.rs) | Tests: [channel_composition.rs](../crates/ofdm/tests/channel_composition.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [channel-workbench/design](development/features/channel-workbench/design.md). |
| <a id="ds-wave-ch-tilt-01"></a>**WAVE-CH-TILT-01**: Implement manual tilt and full additional cable loss with Rust-owned composition, bounded preview, explicit RxMER/correction policy and versioned workbench persistence. Design. | [WAVE-CH-TILT-01](development/checklist.md#task-wave-ch-tilt-01). Historical checked. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [additional_loss.rs](../crates/ofdm/src/signal_processing/channel/additional_loss.rs) | Tests: [additional_loss.rs](../crates/ofdm/tests/additional_loss.rs). Record: [channel-tilt/design](development/features/channel-tilt/design.md#wave-ch-tilt-01-additional-channel-tilt-and-cable-loss). |
| <a id="ds-ui-ch-02"></a><a id="ds-wave-ch-compose-01"></a>**UI-CH-02**: Replace duplicate measurement controls with graphic loss/echo cards and an accepted combined-response preview. Compose diagonal loss with stateful echoes through one shared Rust plan; validate analytic response, actual samples, payload recovery and native UI. Design. Aliases: WAVE-CH-COMPOSE-01. | [UI-CH-02 / WAVE-CH-COMPOSE-01](development/checklist.md#task-ui-ch-02). Historical checked. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [ChannelWorkbenchDialog.qml](../apps/qt/qml/workbenches/channel/ChannelWorkbenchDialog.qml), [composition.rs](../crates/ofdm/src/signal_processing/channel/composition.rs) | Tests: [channel_composition.rs](../crates/ofdm/tests/channel_composition.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [channel-workbench/design](development/features/channel-workbench/design.md#ui-ch-02--wave-ch-compose-01-validation). |
| <a id="ds-ui-ch-01"></a>**UI-CH-01**: Implement Channel Input/Model/Output staging, direct measurement input frames and Spectrum/QAM chooser at each signal boundary. Design. | [UI-CH-01](development/checklist.md#task-ui-ch-01); [UI-CH-01](development/ui-checklist.md#task-ui-ch-01). Historical checked. | [architecture](architecture.md#channel-composition-preview-ui-ch-02--wave-ch-compose-01); [ChannelWorkbenchDialog.qml](../apps/qt/qml/workbenches/channel/ChannelWorkbenchDialog.qml), [composition.rs](../crates/ofdm/src/signal_processing/channel/composition.rs) | Tests: [channel_composition.rs](../crates/ofdm/tests/channel_composition.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [channel-workbench/design](development/features/channel-workbench/design.md#ui-ch-01-basic-channel-simulator-staging). |
| <a id="ds-ui-doc-01"></a>**UI-DOC-01**: Refresh native user-guide captures, remove obsolete overview copies and reconcile stale instructions. Audit record. | [UI-DOC-01](development/checklist.md#task-ui-doc-01); [UI-DOC-01](development/ui-checklist.md#task-ui-doc-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [README.md](README.md), [images](images) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [user-guide-captures/design](development/features/user-guide-captures/design.md#ui-doc-01-current-native-user-guide-captures), [user-guide-captures/png-audit](development/features/user-guide-captures/png-audit.md). |
| <a id="ds-ui-sa-07"></a>**UI-SA-07**: Place metadata inside the SA plot, run actions in the footer and cable echo context in a labeled grid. Design. | [UI-SA-07](development/checklist.md#task-ui-sa-07); [UI-SA-07](development/ui-checklist.md#task-ui-sa-07). Historical checked; current refinement: [UI-SA-DETAILS-01](requirements.md#ds-ui-sa-details-01). | [architecture](architecture.md#shared-spectrum-analyzer-sa-and-transmitter-readiness); [transmitter_preview.rs](../crates/ofdm/src/simulation/transmitter_preview.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [transmitter_preview.rs](../crates/ofdm/tests/transmitter_preview.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-spectrum/validation](development/features/transmitter-spectrum/validation.md). |
| <a id="ds-ui-sa-06"></a>**UI-SA-06**: Use radio buttons for SA frequency axes and remove redundant caption/raw Tap text. Design. | [UI-SA-06](development/checklist.md#task-ui-sa-06); [UI-SA-06](development/ui-checklist.md#task-ui-sa-06). Historical checked. | [architecture](architecture.md#shared-spectrum-analyzer-sa-and-transmitter-readiness); [transmitter_preview.rs](../crates/ofdm/src/simulation/transmitter_preview.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [transmitter_preview.rs](../crates/ofdm/tests/transmitter_preview.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-spectrum/validation](development/features/transmitter-spectrum/validation.md). |
| <a id="ds-req-ds-doc-terms-01"></a>**REQ-DS-DOC-TERMS-01**: Add terminology Markdown and About menu online access. Design. | [UI-DOC-02](development/checklist.md#task-req-ds-doc-terms-01); [UI-DOC-02](development/ui-checklist.md#task-req-ds-doc-terms-01). Historical checked. | [architecture](architecture.md#terminology-cell-and-symbol); [terminology.md](user/terminology.md), [AboutDialog.qml](../apps/qt/qml/about/AboutDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [terminology/design](development/features/terminology/design.md#ui-doc-02-online-terminology-guide). |
| <a id="ds-ui-tx-06"></a>**UI-TX-06**: Identify Source and Source Bits as originating in PRBS-TX. Design. | [UI-TX-06](development/checklist.md#task-ui-tx-06); [UI-TX-06](development/ui-checklist.md#task-ui-tx-06). Historical checked. | [architecture](architecture.md#graphical-transmitter-test-points); [transmitter](../apps/qt/src/instruments/transmitter), [TransmitterWorkbenchDialog.qml](../apps/qt/qml/workbenches/transmitter/TransmitterWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-workbench/design](development/features/transmitter-workbench/design.md#ui-tx-06-source-stage-origin-label), [transmitter-graphs/design](development/features/transmitter-graphs/design.md). |
| <a id="ds-ui-tx-05"></a>**UI-TX-05**: Color Data/NCP QAM roles with a legend and retain actual TX role provenance through bounded observation/display selection. Design. | [UI-TX-05](development/checklist.md#task-ui-tx-05); [UI-TX-05](development/ui-checklist.md#task-ui-tx-05). Historical checked. | [architecture](architecture.md#graphical-transmitter-test-points); [transmitter](../apps/qt/src/instruments/transmitter), [TransmitterWorkbenchDialog.qml](../apps/qt/qml/workbenches/transmitter/TransmitterWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-workbench/design](development/features/transmitter-workbench/design.md#ui-tx-05-qam-symbol-role-colors-and-legend), [transmitter-graphs/design](development/features/transmitter-graphs/design.md). |
| <a id="ds-ui-tx-04"></a>**UI-TX-04**: Replace message TLV buttons with a folding tree grid and registry-backed decoded fields, preserving exact packet bytes. Design. | [UI-TX-04](development/checklist.md#task-ui-tx-04); [UI-TX-04](development/ui-checklist.md#task-ui-tx-04). Historical checked. | [architecture](architecture.md#graphical-transmitter-test-points); [transmitter](../apps/qt/src/instruments/transmitter), [TransmitterWorkbenchDialog.qml](../apps/qt/qml/workbenches/transmitter/TransmitterWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-workbench/design](development/features/transmitter-workbench/design.md#ui-tx-04-folding-descriptor-tree-grid), [transmitter-graphs/design](development/features/transmitter-graphs/design.md). |
| <a id="ds-ui-tx-03"></a>**UI-TX-03**: Implement the staged Transmitter workbench with test-point instruments and generated/emitted OCD/DPD message inspection. Design and validation. | [UI-TX-03](development/checklist.md#task-ui-tx-03); [UI-TX-03](development/ui-checklist.md#task-ui-tx-03). Historical checked; current refinement: [UI-TX-GRAPH-01](requirements.md#ds-ui-tx-graph-01). | [architecture](architecture.md#graphical-transmitter-test-points); [transmitter](../apps/qt/src/instruments/transmitter), [TransmitterWorkbenchDialog.qml](../apps/qt/qml/workbenches/transmitter/TransmitterWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-workbench/design](development/features/transmitter-workbench/design.md#ui-tx-03-transmitter-workbench-and-message-inspection), [transmitter-graphs/design](development/features/transmitter-graphs/design.md). |
| <a id="ds-ui-examples-01"></a>**UI-EXAMPLES-01**: Organize matching single/multi-RxMER bundles, each with a portable WB and original captures. Design. | [UI-EXAMPLES-01](development/checklist.md#task-ui-examples-01); [UI-EXAMPLES-01](development/ui-checklist.md#task-ui-examples-01). Historical checked. | [architecture](architecture.md#curated-workbench-examples-and-open-location); [workbench](../examples/workbench), [example_directory.rs](../apps/qt/src/platform/example_directory.rs) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [workbench-examples/design](development/features/workbench-examples/design.md#ui-examples-01-curated-runnable-bundles). |
| <a id="ds-ui-examples-02"></a>**UI-EXAMPLES-02**: Add channel 193 `multi-rxmer-2`: 132 original RxMER files, companion DPD, flat response and bounded 256-file support. Design. | [UI-EXAMPLES-02](development/checklist.md#task-ui-examples-02); [UI-EXAMPLES-02](development/ui-checklist.md#task-ui-examples-02). Historical checked. | [architecture](architecture.md#curated-workbench-examples-and-open-location); [workbench](../examples/workbench), [example_directory.rs](../apps/qt/src/platform/example_directory.rs) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [workbench-examples/design](development/features/workbench-examples/design.md#ui-examples-02-channel-193-measured-rxmer-collection). |
| <a id="ds-ui-open-01"></a>**UI-OPEN-01**: Default native Open to installed/source example resources, independently of the launch directory. Design. | [UI-OPEN-01](development/checklist.md#task-ui-open-01); [UI-OPEN-01](development/ui-checklist.md#task-ui-open-01). Historical checked. | [architecture](architecture.md#curated-workbench-examples-and-open-location); [workbench](../examples/workbench), [example_directory.rs](../apps/qt/src/platform/example_directory.rs) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [workbench-examples/design](development/features/workbench-examples/design.md#ui-open-01-relocatable-examples-directory). |
| <a id="ds-ui-01"></a>**UI-01**: Build the planned Qt system workspace after core APIs exist: clickable source/TX/channel/RX blocks with focused pop-up dialogs. Include persistent lower-right Start, Stop and Pause/Resume controls; define Rust pause lifecycle and runtime-budget behavior before implementing pause. Include the separate OFDM transmission settings panel from the updated diagram guide: cyclic prefix, roll-off/windowing, time-interleaver depth and pilot settings; validate through Rust and apply changes to the next run. Linked detail tasks retain their individual acceptance; this umbrella remains open. | [UI-01](development/checklist.md#task-ui-01); [UI detail](development/ui-checklist.md#task-ui-01); [UI detail](development/ui-checklist.md#task-ui-01-detail-2); [UI detail](development/ui-checklist.md#task-ui-01-detail-3); [UI detail](development/ui-checklist.md#task-ui-01-detail-4); [UI detail](development/ui-checklist.md#task-ui-01-detail-5); [UI detail](development/ui-checklist.md#task-ui-01-detail-6); [UI detail](development/ui-checklist.md#task-ui-01-detail-7); [UI detail](development/ui-checklist.md#task-ui-01-detail-8); [UI detail](development/ui-checklist.md#task-ui-01-detail-9); [UI detail](development/ui-checklist.md#task-ui-01-detail-10); [UI detail](development/ui-checklist.md#task-ui-01-detail-11); [UI detail](development/ui-checklist.md#task-ui-01-detail-12); [UI detail](development/ui-checklist.md#task-ui-01-detail-13); [UI detail](development/ui-checklist.md#task-ui-01-detail-14); [UI detail](development/ui-checklist.md#task-ui-01-detail-15); [UI detail](development/ui-checklist.md#task-ui-01-detail-16); [UI detail](development/ui-checklist.md#task-ui-01-detail-17); [UI detail](development/ui-checklist.md#task-ui-01-detail-18); [UI detail](development/ui-checklist.md#task-ui-01-detail-19); [UI detail](development/ui-checklist.md#task-ui-01-detail-20); [UI detail](development/ui-checklist.md#task-ui-01-detail-21). Pending / partial. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |
| <a id="ds-ui-02"></a>**UI-02**: Add a role-based file panel and OS file picker; validate inputs before applying them to a new run, with clickable/editable PRBS15/text/binary source selection, size feedback and an explicit append-PRBS15 fill option. Implemented: PRBS-TX source editor with continuous PRBS15 or exact-byte File selection, explicit seeds and optional finite total-bit fill. Options and readiness are defined in the architecture; source acceptance/readiness is implemented; run integration remains pending. Linked detail tasks retain their individual acceptance; this umbrella remains open. | [UI-02](development/checklist.md#task-ui-02); [UI detail](development/ui-checklist.md#task-ui-02); [UI detail](development/ui-checklist.md#task-ui-02-detail-2); [UI detail](development/ui-checklist.md#task-ui-02-detail-3); [UI detail](development/ui-checklist.md#task-ui-02-detail-4); [UI detail](development/ui-checklist.md#task-ui-02-detail-5); [UI detail](development/ui-checklist.md#task-ui-02-detail-6); [UI detail](development/ui-checklist.md#task-ui-02-detail-7); [UI detail](development/ui-checklist.md#task-ui-02-detail-8); [UI detail](development/ui-checklist.md#task-ui-02-detail-9); [UI detail](development/ui-checklist.md#task-ui-02-detail-10). Pending / partial. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |
| <a id="ds-ui-03"></a>**UI-03**: Present separate LDPC, BER, payload pre/post-CE, continuous/scattered pilot and PLC-data/preamble analyzers with shared bounded Rust results. Linked detail tasks retain their individual acceptance; this umbrella remains open. | [UI-03](development/checklist.md#task-ui-03); [UI detail](development/ui-checklist.md#task-ui-03); [UI detail](development/ui-checklist.md#task-ui-03-detail-2); [UI detail](development/ui-checklist.md#task-ui-03-detail-3); [UI detail](development/ui-checklist.md#task-ui-03-detail-4); [UI detail](development/ui-checklist.md#task-ui-03-detail-5); [UI detail](development/ui-checklist.md#task-ui-03-detail-6); [UI detail](development/ui-checklist.md#task-ui-03-detail-7). Pending / partial. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |
| <a id="ds-ui-04"></a>**UI-04**: Validate responsive jobs, cancellation, observation invariance and dark/light readability; add user guides and actual workspace screenshots. See the starting diagram and interaction plan. All simulation computation stays in Rust. The shell alone does not complete UI-01 or the UI-04 validation milestone. Linked detail tasks retain their individual acceptance; this umbrella remains open. | [UI-04](development/checklist.md#task-ui-04); [UI detail](development/ui-checklist.md#task-ui-04); [UI detail](development/ui-checklist.md#task-ui-04-detail-2); [UI detail](development/ui-checklist.md#task-ui-04-detail-3); [UI detail](development/ui-checklist.md#task-ui-04-detail-4); [UI detail](development/ui-checklist.md#task-ui-04-detail-5); [UI detail](development/ui-checklist.md#task-ui-04-detail-6); [UI detail](development/ui-checklist.md#task-ui-04-detail-7); [UI detail](development/ui-checklist.md#task-ui-04-detail-8); [UI detail](development/ui-checklist.md#task-ui-04-detail-9). Pending / partial. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |


### Future DOCSIS scope

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-later-01"></a>**LATER-01**: OFDMA architecture and implementation after downstream scope. | [LATER-01](development/checklist.md#task-later-01). Pending / partial. | [architecture](architecture.md#purpose-and-scope); [lib.rs](../crates/ofdm/src/lib.rs) | Record: [visual-workspace/plan](development/features/visual-workspace/plan.md). |
| <a id="ds-later-02"></a>**LATER-02**: Channel bonding/multi-channel simulation or FDX-specific behavior only under a separate explicit design and validation scope. | [LATER-02](development/checklist.md#task-later-02). Pending / partial. | [architecture](architecture.md#purpose-and-scope); [lib.rs](../crates/ofdm/src/lib.rs) | Record: [visual-workspace/plan](development/features/visual-workspace/plan.md). |


### Pilot/PLC supplied-H reference evidence

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-req-ds-supplied-h-01"></a>**REQ-DS-SUPPLIED-H-01**: - [x] Provide a noiseless measured-H demonstration with separate continuous/scattered-pilot, PLC-data and preamble plots using existing probes; verify modulation geometry and Y=HZ in Rust. This does not itself complete CH-EST-RX-01..06. The later shared collector/estimator is now implemented and validated separately in the linked receiver implementation record. | [unnumbered task](development/checklist.md#task-req-ds-supplied-h-01). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs) | Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md). |


### Repository introduction

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-doc-brand-05"></a>**DOC-BRAND-05**: Refresh the native README overview/analyzers, add RxMER Designer, center gallery titles and remove obsolete image assets. Design. | [DOC-BRAND-05](development/checklist.md#task-doc-brand-05). Historical checked. | [architecture](architecture.md#updated-workspace-diagram-guide); [README.md](../README.md), [README.md](README.md) | Record: [readme-workbench-diagram/design](development/features/readme-workbench-diagram/design.md#doc-brand-05-refresh-the-native-screenshot-gallery). |
| <a id="ds-doc-brand-04"></a>**DOC-BRAND-04**: Link current SA, QAM, response/echo and RxMER screenshots from a compact README analyzer gallery. Design. | [DOC-BRAND-04](development/checklist.md#task-doc-brand-04). Historical checked. | [architecture](architecture.md#updated-workspace-diagram-guide); [README.md](../README.md), [README.md](README.md) | Record: [readme-workbench-diagram/design](development/features/readme-workbench-diagram/design.md#doc-brand-04-analyzer-gallery). |
| <a id="ds-doc-brand-03"></a>**DOC-BRAND-03**: Add a documentation index and UI, Rust API and planned Python guide entry points. Design. | [DOC-BRAND-03](development/checklist.md#task-doc-brand-03). Historical checked. | [architecture](architecture.md#updated-workspace-diagram-guide); [README.md](../README.md), [README.md](README.md) | Record: [readme-workbench-diagram/design](development/features/readme-workbench-diagram/design.md#doc-brand-03-documentation-entry-points). |
| <a id="ds-doc-brand-02"></a>**DOC-BRAND-02**: Shorten README, describe the CMTS downstream PHY simulation directly and link existing flow/status documentation. Design. | [DOC-BRAND-02](development/checklist.md#task-doc-brand-02). Historical checked. | [architecture](architecture.md#updated-workspace-diagram-guide); [README.md](../README.md), [README.md](README.md) | Record: [readme-workbench-diagram/design](development/features/readme-workbench-diagram/design.md#doc-brand-02-concise-repository-introduction). |


### Documentation-only push tooling

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-doc-push-01"></a>**DOC-PUSH-01**: Automatically skip the local build/test gate only for documentation-only outgoing branch commits; retain full checks for mixed/code changes, initial pushes and releases. Inspect merge parents and rename paths. | [DOC-PUSH-01](development/checklist.md#task-doc-push-01). Historical checked. | [architecture](architecture.md#local-documentation-only-push-validation); [docs_only.py](../tools/git/docs_only.py), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_docs_only.py](../tools/git/tests/test_docs_only.py), [test_push.py](../tools/git/tests/test_push.py). Record: [docs-only-push/design](development/features/docs-only-push/design.md#doc-push-01-documentation-only-branch-pushes). |


### Security workflows

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-sec-01"></a>**SEC-01**: Prepare Rust CodeQL and RustSec dependency-audit workflows, weekly/manual triggers, release gates, report retention and README badges. | [SEC-01](development/checklist.md#task-sec-01). Historical checked. | [architecture](architecture.md#security-validation); [security-audit.sh](../scripts/security-audit.sh), [ci.yml](../.github/workflows/ci.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py). Record: [security-checks/design](development/features/security-checks/design.md#sec-01-source-and-dependency-checks), [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md). |
| <a id="ds-req-ds-sec-hosted-01"></a>**REQ-DS-SEC-HOSTED-01**: - [ ] Verify CodeQL analysis/upload and dependency checks in GitHub Actions after pushing; resolve findings without treating successful scans as proof that the engine has no vulnerabilities. | [unnumbered task](development/checklist.md#task-req-ds-sec-hosted-01). Pending / partial. | [architecture](architecture.md#security-validation); [security-audit.sh](../scripts/security-audit.sh), [ci.yml](../.github/workflows/ci.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py). Record: [security-checks/design](development/features/security-checks/design.md), [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md). |


### Development build and test cache (proposed)

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-build-01"></a>**BUILD-01**: Define content fingerprints and an explainable task planner. | [BUILD-01](development/checklist.md#task-build-01). Pending / partial. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [test-local.sh](../scripts/test-local.sh) | Tests: [test_local.py](../tools/testing/test_local.py). Record: [build-cache/design](development/features/build-cache/design.md#build-01-fingerprints-and-read-only-task-planning). |
| <a id="ds-build-02"></a>**BUILD-02**: Reuse verified build artifacts and separate successful check records. | [BUILD-02](development/checklist.md#task-build-02). Pending / partial. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [test-local.sh](../scripts/test-local.sh) | Tests: [test_local.py](../tools/testing/test_local.py). Record: [build-cache/design](development/features/build-cache/design.md#build-02-build-artifacts-and-success-records). |
| <a id="ds-build-03"></a>**BUILD-03**: Map affected crate/subdirectory tests and conservative consumers. | [BUILD-03](development/checklist.md#task-build-03). Pending / partial. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [test-local.sh](../scripts/test-local.sh) | Tests: [test_local.py](../tools/testing/test_local.py). Record: [build-cache/design](development/features/build-cache/design.md#build-03-explicit-affected-test-mapping). |
| <a id="ds-build-04"></a>**BUILD-04**: Validate invalidation, concurrency and actual warm-run savings. | [BUILD-04](development/checklist.md#task-build-04). Pending / partial. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [test-local.sh](../scripts/test-local.sh) | Tests: [test_local.py](../tools/testing/test_local.py). Record: [build-cache/design](development/features/build-cache/design.md#build-04-verification-measurement-and-rollout). |


### Development setup

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-dev-01"></a>**DEV-01**: Add explicit Ubuntu x64 development setup with safe check/dry-run, pinned local Rust/Qt and required package declarations. Host syntax/control-flow checks pass; clean-machine provisioning remains unverified. | [DEV-01](development/checklist.md#task-dev-01). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [install.sh](../install.sh), [install-development.sh](../scripts/install-development.sh) | Tests: [test_setup.py](../tools/development/tests/test_setup.py). Record: [development-setup/design](development/features/development-setup/design.md#dev-01-explicit-ubuntu-development-installer). |
| <a id="ds-dev-02"></a>**DEV-02**: Fetch the PNM decoder by exact Git/version pins into ignored Cargo storage; preserve notices without vendoring viewer source. | [DEV-02](development/checklist.md#task-dev-02). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [install.sh](../install.sh), [install-development.sh](../scripts/install-development.sh) | Tests: [test_setup.py](../tools/development/tests/test_setup.py). Record: [development-setup/design](development/features/development-setup/design.md#dev-02-pinned-decoder-source-fetch). |
| <a id="ds-dev-03"></a>**DEV-03**: Validate complete provisioning on pristine Ubuntu 22.04/24.04/26.04. | [DEV-03](development/checklist.md#task-dev-03). Pending / partial. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [install.sh](../install.sh), [install-development.sh](../scripts/install-development.sh) | Tests: [test_setup.py](../tools/development/tests/test_setup.py). Record: [development-setup/design](development/features/development-setup/design.md). |


### Parallel local validation

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-dev-05"></a>**DEV-05**: Schedule independent Rust tests across local CPUs with a pinned runner, complete test-phase coverage and measured full-suite results. Design. | [DEV-05](development/checklist.md#task-dev-05). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [test-local.sh](../scripts/test-local.sh), [testing](../tools/testing) | Tests: [test_nextest.py](../tools/testing/test_nextest.py). Record: [parallel-local-tests/design](development/features/parallel-local-tests/design.md#dev-05-parallel-local-validation). |


### Workbench file filtering

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-wb-open-01"></a>**UI-WB-OPEN-01**: Filter main Open / Ctrl+O to workbench JSON in the OS-native chooser, preserving folders and input-block selection behavior. Design. | [UI-WB-OPEN-01](development/checklist.md#task-ui-wb-open-01); [UI-WB-OPEN-01](development/ui-checklist.md#task-ui-wb-open-01). Historical checked. | [architecture](architecture.md#command-line-workbench-startup-and-diagnostics); [project_load.rs](../apps/qt/src/workbench/project_load.rs), [WorkbenchLoadingDialog.qml](../apps/qt/qml/shell/WorkbenchLoadingDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [responsive-workbench-open/design](development/features/responsive-workbench-open/design.md). |


### DPD visual inspection

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-dpd-view-01"></a>**UI-DPD-VIEW-01**: Add a read-only profile plot from DPD using the shared PNM Viewer-derived chart; hide filenames and document viewing/zoom behavior. Design. | [UI-DPD-VIEW-01](development/checklist.md#task-ui-dpd-view-01); [UI-DPD-VIEW-01](development/ui-checklist.md#task-ui-dpd-view-01). Historical checked; recovered evidence: [DS-GAP-03](development/checklist.md#ds-gap-03). | [architecture](architecture.md#downstream-profile-input-dialog); [plot.rs](../apps/qt/src/inputs/profile_input/plot.rs), [ProfilePlotDialog.qml](../apps/qt/qml/inputs/profile_input/ProfilePlotDialog.qml), [ProfileChart.qml](../apps/qt/qml/inputs/profile_input/ProfileChart.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [dedicated DPD viewer validation](development/features/pnm-measurement-inputs/profile-view.md). |


### Ubuntu 22.04 desktop delivery

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-deb-01"></a>**UI-DEB-01**: Build the amd64 Qt DEB with private runtime, examples, notices and local package validation. Design. | [UI-DEB-01](development/checklist.md#task-ui-deb-01); [UI-DEB-01](development/ui-checklist.md#task-ui-deb-01). Historical checked. | [architecture](architecture.md#ubuntu-2204-desktop-delivery-and-release-notification); [desktop.py](../tools/packaging/desktop.py), [mod.rs](../apps/qt/src/platform/updates/mod.rs) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [tst_updates.qml](../apps/qt/tests/tst_updates.qml). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md#ui-deb-01-ubuntu-2204-amd64-deb). |
| <a id="ds-ui-update-01"></a>**UI-UPDATE-01**: Add background stable-release notification, About retry and global startup preference. Design. | [UI-UPDATE-01](development/checklist.md#task-ui-update-01); [UI-UPDATE-01](development/ui-checklist.md#task-ui-update-01). Historical checked. | [architecture](architecture.md#ubuntu-2204-desktop-delivery-and-release-notification); [desktop.py](../tools/packaging/desktop.py), [mod.rs](../apps/qt/src/platform/updates/mod.rs) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [tst_updates.qml](../apps/qt/tests/tst_updates.qml). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md#ui-update-01-stable-release-notifications). |
| <a id="ds-ui-deb-02"></a>**UI-DEB-02**: Retain successful hosted Ubuntu 22.04 build and disposable installed/reinstall/remove results; verify this VM's interactive installed UI. | [UI-DEB-02](development/checklist.md#task-ui-deb-02); [UI-DEB-02](development/ui-checklist.md#task-ui-deb-02). Pending / partial. | [architecture](architecture.md#ubuntu-2204-desktop-delivery-and-release-notification); [desktop.py](../tools/packaging/desktop.py), [mod.rs](../apps/qt/src/platform/updates/mod.rs) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [tst_updates.qml](../apps/qt/tests/tst_updates.qml). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md). |
| <a id="ds-ui-deb-03"></a>**UI-DEB-03**: Fix first-run scratch storage and SDK bootstrap failures; verify packaging tests in a fresh tracked checkout. Hosted rerun pending. | [UI-DEB-03](development/checklist.md#task-ui-deb-03); [UI-DEB-03](development/ui-checklist.md#task-ui-deb-03). Historical checked. | [architecture](architecture.md#ubuntu-2204-desktop-delivery-and-release-notification); [desktop.py](../tools/packaging/desktop.py), [mod.rs](../apps/qt/src/platform/updates/mod.rs) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [tst_updates.qml](../apps/qt/tests/tst_updates.qml). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md#ui-deb-03-fresh-runner-bootstrap-recovery). |
| <a id="ds-dev-04"></a>**DEV-04**: Add explicit local `--skip-test` with retained non-simulation checks and Git guards. Design. | [DEV-04](development/checklist.md#task-dev-04). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [git-push.sh](../tools/git/git-push.sh) | Tests: [test_validation_options.py](../tools/git/tests/test_validation_options.py). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md). |
| <a id="ds-security-deps-02"></a>**SECURITY-DEPS-02**: Patch rand/cxx advisories and audit both graphs without suppression. Design. | [SECURITY-DEPS-02](development/checklist.md#task-security-deps-02); [SECURITY-DEPS-02](development/ui-checklist.md#task-security-deps-02). Historical checked. | [architecture](architecture.md#security-validation); [security-audit.sh](../scripts/security-audit.sh), [ci.yml](../.github/workflows/ci.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py). Record: [security-checks/design](development/features/security-checks/design.md), [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md#security-deps-02-release-dependency-audit). |
| <a id="ds-ui-dpd-view-02"></a>**UI-DPD-VIEW-02**: Adapt the full PNM Viewer modulation view with profile overlays, dB/bit loading, carrier filters/lanes and gross Profile BW. Design. | [UI-DPD-VIEW-02](development/checklist.md#task-ui-dpd-view-02); [UI-DPD-VIEW-02](development/ui-checklist.md#task-ui-dpd-view-02). Historical checked; recovered evidence: [DS-GAP-03](development/checklist.md#ds-gap-03). | [architecture](architecture.md#downstream-profile-input-dialog); [plot.rs](../apps/qt/src/inputs/profile_input/plot.rs), [ProfilePlotDialog.qml](../apps/qt/qml/inputs/profile_input/ProfilePlotDialog.qml), [ProfileChart.qml](../apps/qt/qml/inputs/profile_input/ProfileChart.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [dedicated DPD viewer validation](development/features/pnm-measurement-inputs/profile-view.md). |


### Receiver profile performance

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-rx-perf-01"></a>**RX-PERF-01**: Run all accepted DPD profiles as independent sequential experiments with common settings/seeds, bounded result retention, lifecycle controls and WB-persisted run mode. Preserve profile-specific failures/results. | [RX-PERF-01](development/checklist.md#task-rx-perf-01). Historical checked. | [architecture](architecture.md#receiver-profile-performance); [performance.rs](../crates/ofdm/src/simulation/channel/performance.rs), [receiver](../apps/qt/src/instruments/receiver) | Tests: [receiver_profiles.rs](../crates/ofdm/tests/receiver_profiles.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [profile-performance/design](development/features/profile-performance/design.md#rx-perf-01-independent-profile-sweep). |
| <a id="ds-fec-counts-01"></a>**FEC-COUNTS-01**: Add actual TotalCW, CCW, UFEC and clean-word accounting, including LDPC-only correction evidence and observation-window semantics. | [FEC-COUNTS-01](development/checklist.md#task-fec-counts-01). Historical checked. | [architecture](architecture.md#receiver-profile-performance); [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs), [performance.rs](../crates/ofdm/src/simulation/channel/performance.rs) | Tests: [receiver_profiles.rs](../crates/ofdm/tests/receiver_profiles.rs), [statistics.rs](../crates/fec-codec/tests/statistics.rs). Record: [profile-performance/design](development/features/profile-performance/design.md#fec-counts-01-payload-codeword-accounting). |
| <a id="ds-ui-rx-perf-01"></a>**UI-RX-PERF-01**: Add Receiver Profile Performance: measured MER and ideal profile minimum MER overlay, profile selection and matching FEC counters. | [UI-RX-PERF-01](development/checklist.md#task-ui-rx-perf-01); [UI-RX-PERF-01](development/ui-checklist.md#task-ui-rx-perf-01). Historical checked. | [architecture](architecture.md#receiver-profile-performance); [performance.rs](../crates/ofdm/src/simulation/channel/performance.rs), [receiver](../apps/qt/src/instruments/receiver) | Tests: [receiver_profiles.rs](../crates/ofdm/tests/receiver_profiles.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [profile-performance/design](development/features/profile-performance/design.md#ui-rx-perf-01-receiver-profile-performance-instrument). |
| <a id="ds-ui-rx-perf-02"></a>**UI-RX-PERF-02**: Shade the selected result’s first-to-last PLC region behind both curves, clipping to the viewport. All 81 adapter and 74 QML checks pass; native dark/light and zoom verified with real run results. | [UI-RX-PERF-02](development/checklist.md#task-ui-rx-perf-02). Historical checked. | [architecture](architecture.md#receiver-profile-performance); [performance.rs](../crates/ofdm/src/simulation/channel/performance.rs), [receiver](../apps/qt/src/instruments/receiver) | Tests: [receiver_profiles.rs](../crates/ofdm/tests/receiver_profiles.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [profile-performance/design](development/features/profile-performance/design.md#ui-rx-perf-02-plc-region-on-performance-plots). |


### Automatic Ubuntu desktop updates

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-update-02"></a>**UI-UPDATE-02**: Right-aligned Update chip and automatic Ubuntu download/verify/polkit install/restart with WB recovery. Local adapter/QML, package identity and native recovery checks pass. Design. | [UI-UPDATE-02](development/checklist.md#task-ui-update-02); [UI-UPDATE-02](development/ui-checklist.md#task-ui-update-02). Historical checked. | [architecture](architecture.md#ubuntu-2204-desktop-delivery-and-release-notification); [install.rs](../apps/qt/src/platform/updates/install.rs), [UpdateNotification.qml](../apps/qt/qml/shell/UpdateNotification.qml) | Tests: [tst_updates.qml](../apps/qt/tests/tst_updates.qml). Record: [automatic-updates/design](development/features/automatic-updates/design.md#ui-update-02-automatic-ubuntu-desktop-update). |
| <a id="ds-ui-update-qa-01"></a>**UI-UPDATE-QA-01**: Validate a real installed upgrade, authentication cancellation, APT failure and automatic restart on disposable Ubuntu 22.04. | [UI-UPDATE-QA-01](development/checklist.md#task-ui-update-qa-01); [UI-UPDATE-QA-01](development/ui-checklist.md#task-ui-update-qa-01). Pending / partial. | [architecture](architecture.md#ubuntu-2204-desktop-delivery-and-release-notification); [install.rs](../apps/qt/src/platform/updates/install.rs), [UpdateNotification.qml](../apps/qt/qml/shell/UpdateNotification.qml) | Tests: [tst_updates.qml](../apps/qt/tests/tst_updates.qml). Record: [automatic-updates/design](development/features/automatic-updates/design.md). |


### Main workbench configuration and shared CI

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-wb-config-01"></a>**UI-WB-CONFIG-01**: Main Configure WB with Run/Profiles tabs, explicit New/Open activation, subset persistence/migration and Receiver-only controls. Adapter/QML checks and native visual/Save As/Open verification pass. | [UI-WB-CONFIG-01](development/checklist.md#task-ui-wb-config-01); [UI-WB-CONFIG-01](development/ui-checklist.md#task-ui-wb-config-01). Historical checked. | [architecture](architecture.md#main-wb-configuration-and-explicit-profile-selection-ui-wb-config-01); [project_file.rs](../apps/qt/src/workbench/project_file.rs), [WorkbenchConfigurationDialog.qml](../apps/qt/qml/workbenches/workbench/WorkbenchConfigurationDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [workbench-configuration/design](development/features/workbench-configuration/design.md#ui-wb-config-01-main-workbench-run-configuration). |
| <a id="ds-ui-wb-config-02"></a>**UI-WB-CONFIG-02**: Explain Noise Seed on label/value hover and editor focus, with accessible help, terminology and native guide screenshot. | [UI-WB-CONFIG-02](development/checklist.md#task-ui-wb-config-02). Historical checked. | [architecture](architecture.md#main-wb-configuration-and-explicit-profile-selection-ui-wb-config-01); [project_file.rs](../apps/qt/src/workbench/project_file.rs), [WorkbenchConfigurationDialog.qml](../apps/qt/qml/workbenches/workbench/WorkbenchConfigurationDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [workbench-configuration/design](development/features/workbench-configuration/design.md#ui-wb-config-02-explain-noise-seed). |
| <a id="ds-ci-consolidate-01"></a>**CI-CONSOLIDATE-01**: Implement two entry points sharing all validation, parallel native tests and release-only packages; static validation passes. | [CI-CONSOLIDATE-01](development/checklist.md#task-ci-consolidate-01); [CI-CONSOLIDATE-01](development/ui-checklist.md#task-ci-consolidate-01). Historical checked. | [architecture](architecture.md#shared-master-and-release-validation-ci-consolidate-01); [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [workbench-configuration/design](development/features/workbench-configuration/design.md#ci-consolidate-01-two-workflow-entry-points). |
| <a id="ds-ci-desktop-apt-01"></a>**CI-DESKTOP-APT-01**: Implement strict refresh, bounded retry and CI mirror preference; fake-command retry tests and shell validation pass. | [CI-DESKTOP-APT-01](development/checklist.md#task-ci-desktop-apt-01); [CI-DESKTOP-APT-01](development/ui-checklist.md#task-ci-desktop-apt-01). Historical checked. | [architecture](architecture.md#shared-master-and-release-validation-ci-consolidate-01); [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [workbench-configuration/design](development/features/workbench-configuration/design.md#ci-desktop-apt-01). |
| <a id="ds-ci-desktop-qa-01"></a>**CI-DESKTOP-QA-01**: Verify shared hosted validation, successful APT setup, release packaging/installed lifecycle and publication on a new authorized tag. | [CI-DESKTOP-QA-01](development/checklist.md#task-ci-desktop-qa-01); [CI-DESKTOP-QA-01](development/ui-checklist.md#task-ci-desktop-qa-01). Pending / partial. | [architecture](architecture.md#shared-master-and-release-validation-ci-consolidate-01); [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [workbench-configuration/design](development/features/workbench-configuration/design.md). |
| <a id="ds-ui-dialog-actions-01"></a>**UI-DIALOG-ACTIONS-01**: Share lower-right Apply then Cancel actions across all nine draft editors; simplify labels and preserve cancellation. Design. | [UI-DIALOG-ACTIONS-01](development/checklist.md#task-ui-dialog-actions-01); [UI-DIALOG-ACTIONS-01](development/ui-checklist.md#task-ui-dialog-actions-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [components](../apps/qt/qml/components), [theme](../apps/qt/qml/theme) | Tests: [tst_buttons.qml](../apps/qt/tests/tst_buttons.qml). Record: [popup-close-controls/design](development/features/popup-close-controls/design.md). |
| <a id="ds-ui-fec-02"></a>**UI-FEC-02**: Implement the approved FEC overview, actual iteration bars, per-profile selection and separate control/codeword tabs. Adapter/QML checks and native simulation/visual checks pass; compiled Qt executable is ready. Design. | [UI-FEC-02](development/checklist.md#task-ui-fec-02); [UI-FEC-02](development/ui-checklist.md#task-ui-fec-02). Historical checked. | [architecture](architecture.md#fec-monitor-presentation-ui-fec-02); [receiver](../apps/qt/src/instruments/receiver), [fec](../apps/qt/qml/instruments/fec) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [fec-monitor/design](development/features/fec-monitor/design.md#ui-fec-02-readable-fec-monitor). |
| <a id="ds-ui-run-time-01"></a>**UI-RUN-TIME-01**: Show live/final wall elapsed time, per-profile timing, actual phase and recovery progress. Clock/lifecycle, adapter and QML checks pass; native bounded Profile3 timing/CPU sample and screenshots verified. Design. | [UI-RUN-TIME-01](development/checklist.md#task-ui-run-time-01); [UI-RUN-TIME-01](development/ui-checklist.md#task-ui-run-time-01). Historical checked. | [architecture](architecture.md#run-wall-timing-ui-run-time-01); [run_timing.rs](../apps/qt/src/simulation/run_timing.rs) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [run-elapsed-time/design](development/features/run-elapsed-time/design.md#ui-run-time-01-visible-run-and-profile-elapsed-time). |
| <a id="ds-ui-terms-02"></a>**UI-TERMS-02**: Define interleaver depth/drain in the About-linked guide and require technical UI definitions through the UI-design skill. Design. | [UI-TERMS-02](development/checklist.md#task-ui-terms-02); [UI-TERMS-02](development/ui-checklist.md#task-ui-terms-02). Historical checked. | [architecture](architecture.md#terminology-cell-and-symbol); [terminology.md](user/terminology.md), [AboutDialog.qml](../apps/qt/qml/about/AboutDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [terminology/design](development/features/terminology/design.md). |
| <a id="ds-qam-perf-01"></a>**QAM-PERF-01**: Replace high-order exhaustive soft demapping with verified axis lookups and exact cross-QAM search; preserve labels and numerical results. Core/analysis and Qt checks pass; kernel benchmark and native Profile3 run measured. Compiled Qt executable is ready. Design. | [QAM-PERF-01](development/checklist.md#task-qam-perf-01). Historical checked. | [architecture](architecture.md#indexed-qam-soft-demapping-qam-perf-01); [index.rs](../crates/ofdm/src/signal_processing/modulation/soft_bits/index.rs) | Tests: [qam_soft_index.rs](../crates/ofdm/tests/qam_soft_index.rs). Record: [qam-demapper-performance/design](development/features/qam-demapper-performance/design.md#qam-perf-01-exact-indexed-max-log-soft-demapping). |
| <a id="ds-ui-wb-controls-01"></a>**UI-WB-CONTROLS-01**: Rounded main WB actions and idle Configure entry chooser: Create New/Open Existing continue only after activation; native cancellation/failure preserves state. Design. | [UI-WB-CONTROLS-01](development/checklist.md#task-ui-wb-controls-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [components](../apps/qt/qml/components), [theme](../apps/qt/qml/theme) | Tests: [tst_buttons.qml](../apps/qt/tests/tst_buttons.qml). Record: [popup-close-controls/design](development/features/popup-close-controls/design.md). |


### Shared action buttons

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-buttons-01"></a>**UI-BUTTONS-01**: Apply the approved shared rounded action style across all frames, including standard dialog and Apply/Cancel buttons. Preserve roles, keyboard behavior, themes and specialized diagram controls; require the component through the UI skill. Design. | [UI-BUTTONS-01](development/checklist.md#task-ui-buttons-01); [UI-BUTTONS-01](development/ui-checklist.md#task-ui-buttons-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [components](../apps/qt/qml/components), [theme](../apps/qt/qml/theme) | Tests: [tst_buttons.qml](../apps/qt/tests/tst_buttons.qml). Record: [popup-close-controls/design](development/features/popup-close-controls/design.md). |
| <a id="ds-ui-qam-dots-01"></a>**UI-QAM-DOTS-01**: Circular received-I/Q markers and channel-output display-limit note; preserve TX role shapes and actual sample coordinates. Design. | [UI-QAM-DOTS-01](development/checklist.md#task-ui-qam-dots-01). Historical checked. | [architecture](architecture.md#received-qam-markers-ui-qam-dots-01); [qam](../apps/qt/qml/instruments/qam) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qam-markers/design](development/features/qam-markers/design.md#ui-qam-dots-01-received-iq-marker-shape). |
| <a id="ds-ui-help-01"></a>**UI-HELP-01**: Stable playback diagnostic, computed timing advice, hover/focus help and bundled offline FAQ. Unknown messages remain intact. Design. | [UI-HELP-01](development/checklist.md#task-ui-help-01); [UI-HELP-01](development/ui-checklist.md#task-ui-help-01). Historical checked. | [architecture](architecture.md#contextual-help-and-spectrum-details); [help.rs](../apps/qt/src/help.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [channel_playback.rs](../crates/ofdm/tests/channel_playback.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [contextual-help/design](development/features/contextual-help/design.md#ui-help-01-contextual-diagnostics-and-offline-faq). |
| <a id="ds-ui-sa-details-01"></a>**UI-SA-DETAILS-01**: Move acquisition metadata below spectrum axes with a Details checkbox; preserve chart data, warnings and power. Design. | [UI-SA-DETAILS-01](development/checklist.md#task-ui-sa-details-01); [UI-SA-DETAILS-01](development/ui-checklist.md#task-ui-sa-details-01). Historical checked. | [architecture](architecture.md#contextual-help-and-spectrum-details); [help.rs](../apps/qt/src/help.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [channel_playback.rs](../crates/ofdm/tests/channel_playback.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [contextual-help/design](development/features/contextual-help/design.md#ui-sa-details-01-spectrum-acquisition-readout). |
| <a id="ds-release-resume-01"></a>**RELEASE-RESUME-01**: Explicit version-only interrupted-bump recovery with unchanged index, exact candidate validation and no second bump. Design. | [RELEASE-RESUME-01](development/checklist.md#task-release-resume-01). Historical checked. | [architecture](architecture.md#interrupted-release-candidate-recovery); [push-release.py](../tools/git/push-release.py) | Tests: [test_release.py](../tools/git/tests/test_release.py). Record: [release-resume/design](development/features/release-resume/design.md#release-resume-01-explicit-interrupted-bump-recovery). |
| <a id="ds-release-resume-02"></a>**RELEASE-RESUME-02**: Print the exact recovery command for recognized pending candidates; retain original errors and show inspection for other work. Design. | [RELEASE-RESUME-02](development/checklist.md#task-release-resume-02). Historical checked. | [architecture](architecture.md#interrupted-release-candidate-recovery); [push-release.py](../tools/git/push-release.py) | Tests: [test_release.py](../tools/git/tests/test_release.py). Record: [release-resume/design](development/features/release-resume/design.md#release-resume-02-actionable-failure-output). |
| <a id="ds-dev-test-quick-01"></a>**DEV-TEST-QUICK-01**: Explicit local subset with reviewed sweep omissions, representative recovery cases, full default and unchanged hosted coverage. Verified 471 tests in 132.02 seconds; 21 named sweep cases omitted. Design. | [DEV-TEST-QUICK-01](development/checklist.md#task-dev-test-quick-01). Historical checked. | [architecture](architecture.md#explicit-quick-local-validation); [quick-tests.json](../tools/testing/quick-tests.json), [test-local.sh](../scripts/test-local.sh) | Tests: [test_quick_tests.py](../tools/testing/test_quick_tests.py). Record: [quick-local-tests/design](development/features/quick-local-tests/design.md#dev-test-quick-01-explicit-local-test-subset). |
| <a id="ds-release-asset-01"></a>**RELEASE-ASSET-01**: Shared canonical names, unchanged package IDs, legacy desktop updater compatibility and generic local APT instructions. Verified 14 packaging, 45 adapter and 46 QML tests; native Qt build passes. Design. | [RELEASE-ASSET-01](development/checklist.md#task-release-asset-01). Historical checked. | [architecture](architecture.md#release-asset-identity); [packaging](../tools/packaging) | Tests: [test_asset_names.py](../tools/packaging/test_asset_names.py). Record: [release-asset-names/design](development/features/release-asset-names/design.md#release-asset-01-consistent-distributable-filenames). |
| <a id="ds-ui-about-01"></a>**UI-ABOUT-01**: Simplify About, move manual updates into it, add report action and remove dropdown ellipses. Verified 45 adapter and 47 QML tests, native build and actual dark/light captures; manual update routing passes. Design. | [UI-ABOUT-01](development/checklist.md#task-ui-about-01); [UI-ABOUT-01](development/ui-checklist.md#task-ui-about-01). Historical checked. | [architecture](architecture.md#focused-about-and-dropdown-labels-ui-about-01); [AboutDialog.qml](../apps/qt/qml/about/AboutDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [about-menu/design](development/features/about-menu/design.md#ui-about-01-focused-about-and-menu-labels). |


### Responsive Open and RxMER selection

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-wb-open-02"></a>**UI-WB-OPEN-02**: Background native Open/recovery, prepared-model adoption, loading stage/elapsed/Cancel, no repeated GUI imports; worker failure, cancellation and native responsiveness verified. Design. | [UI-WB-OPEN-02](development/checklist.md#task-ui-wb-open-02); [UI-WB-OPEN-02](development/ui-checklist.md#task-ui-wb-open-02). Historical checked. | [architecture](architecture.md#command-line-workbench-startup-and-diagnostics); [project_load.rs](../apps/qt/src/workbench/project_load.rs), [WorkbenchLoadingDialog.qml](../apps/qt/qml/shell/WorkbenchLoadingDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [responsive-workbench-open/design](development/features/responsive-workbench-open/design.md#ui-wb-open-02--responsive-workbench-loading). |
| <a id="ds-ui-rxmer-select-01"></a>**UI-RXMER-SELECT-01**: Retain loaded originals and persist selected capture identities; Single Shot/Replay controls and CWB-derived capacity. Verify selection, legacy restore, strict boundaries and native layout. Design. | [UI-RXMER-SELECT-01](development/checklist.md#task-ui-rxmer-select-01); [UI-RXMER-SELECT-01](development/ui-checklist.md#task-ui-rxmer-select-01). Historical checked. | [architecture](architecture.md#rxmer-capture-actions-ui-rxmer-04); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-selection/design](development/features/rxmer-selection/design.md#ui-rxmer-select-01--select-captures-and-show-playback-capacity). |
| <a id="ds-ui-rxmer-anim-01"></a>**UI-RXMER-ANIM-01**: PNM Viewer-style selected-capture animation, Play/Pause/step/slider/speed, bounded original viewports and lifecycle cleanup. Design. | [UI-RXMER-ANIM-01](development/checklist.md#task-ui-rxmer-anim-01); [UI-RXMER-ANIM-01](development/ui-checklist.md#task-ui-rxmer-anim-01). Historical checked. | [architecture](architecture.md#rxmer-capture-actions-ui-rxmer-04); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-selection/design](development/features/rxmer-selection/design.md#ui-rxmer-anim-01--animated-capture-preview). |
| <a id="ds-ui-tx-graph-01"></a>**UI-TX-GRAPH-01**: Graphical transmitter test-point cards and full stage graphs, optional exact Details, role legends and shared SA navigation. Verified bounded real graphs, missing history and native dark/light layouts; 54 Rust adapter tests and 53 QML checks pass. Design. | [UI-TX-GRAPH-01](development/checklist.md#task-ui-tx-graph-01); [UI-TX-GRAPH-01](development/ui-checklist.md#task-ui-tx-graph-01). Historical checked. | [architecture](architecture.md#graphical-transmitter-test-points); [transmitter](../apps/qt/src/instruments/transmitter), [TransmitterWorkbenchDialog.qml](../apps/qt/qml/workbenches/transmitter/TransmitterWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-workbench/design](development/features/transmitter-workbench/design.md), [transmitter-graphs/design](development/features/transmitter-graphs/design.md#ui-tx-graph-01--graphical-transmitter-test-points). |
| <a id="ds-ui-wb-open-03"></a>**UI-WB-OPEN-03**: Show an animated loading activity bar and distinguish Time since Open from file progress and simulation time; retain current phase and cancellation. QML behavior, native rendering and build verified. Design. | [UI-WB-OPEN-03](development/checklist.md#task-ui-wb-open-03); [UI-WB-OPEN-03](development/ui-checklist.md#task-ui-wb-open-03). Historical checked. | [architecture](architecture.md#command-line-workbench-startup-and-diagnostics); [project_load.rs](../apps/qt/src/workbench/project_load.rs), [WorkbenchLoadingDialog.qml](../apps/qt/qml/shell/WorkbenchLoadingDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [responsive-workbench-open/design](development/features/responsive-workbench-open/design.md#ui-wb-open-03--clear-loading-activity). |
| <a id="ds-ui-ch-icon-01"></a>**UI-CH-ICON-01**: Shared graphical Spectrum Analyzer / QAM Analyzer choices for every channel input, output and amplifier TapPoint. Use matching graph cards with titles above and no TX Output caption. Verify equal icons, keyboard routing and native rendering; 53 QML checks pass. Design. | [UI-CH-ICON-01](development/checklist.md#task-ui-ch-icon-01); [UI-CH-ICON-01](development/ui-checklist.md#task-ui-ch-icon-01). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-ui-fec-table-01"></a>**UI-FEC-TABLE-01**: Compact Channel / Metric / Value grid for whole-run control statistics, with aligned values and bounded width. Design. | [UI-FEC-TABLE-01](development/checklist.md#task-ui-fec-table-01); [UI-FEC-TABLE-01](development/ui-checklist.md#task-ui-fec-table-01). Historical checked. | [architecture](architecture.md#fec-monitor-presentation-ui-fec-02); [receiver](../apps/qt/src/instruments/receiver), [fec](../apps/qt/qml/instruments/fec) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [fec-monitor/design](development/features/fec-monitor/design.md#ui-fec-table-01--compact-control-statistics). |
| <a id="ds-ui-wb-open-04"></a>**UI-WB-OPEN-04**: Hide loading timing except --debug-ui; support startup workbench paths and redirected stdin through bounded background validation. Maintain debug-control documentation and lifecycle logging. Normal/debug rendering, startup, Save destinations, errors and cancellation verified; 58 Rust tests, 53 QML checks and native Ubuntu launches pass. Design. | [UI-WB-OPEN-04](development/checklist.md#task-ui-wb-open-04); [UI-WB-OPEN-04](development/ui-checklist.md#task-ui-wb-open-04). Historical checked. | [architecture](architecture.md#command-line-workbench-startup-and-diagnostics); [project_load.rs](../apps/qt/src/workbench/project_load.rs), [WorkbenchLoadingDialog.qml](../apps/qt/qml/shell/WorkbenchLoadingDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [responsive-workbench-open/design](development/features/responsive-workbench-open/design.md#ui-wb-open-04--quiet-loading-and-command-line-workbenches). |
| <a id="ds-ui-sa-timing-02"></a>**UI-SA-TIMING-02**: Expand the compact timing plot into a resizable full two-symbol and overlap view. Distinguish overlap from usable guard and label CP end / FFT start. Use actual core window weights, stable time axes, explicit result/current-configuration source and accepted-setting refresh. Verified both FFTs, all CP/RP pairs, keyboard/resize and native dark/light UI; 61 Rust tests, 54 QML checks, Clippy and native build pass. Design. | [UI-SA-TIMING-02](development/checklist.md#task-ui-sa-timing-02); [UI-SA-TIMING-02](development/ui-checklist.md#task-ui-sa-timing-02). Historical checked. | [architecture](architecture.md#sa-symbol-timing-and-cable-panel-ui-sa-04); [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [symbol-timing-popup/design](development/features/symbol-timing-popup/design.md#ui-sa-timing-02--full-symbol-timing-popup). |
| <a id="ds-ui-qam-mer-01"></a>**UI-QAM-MER-01**: Reuse PNM Viewer's MER in a typed core calculator, show separate raw/equalized estimates over every selected retained cell, and reduce channel/receiver QAM marker size. Numeric/reference tests, 62 adapter tests, 54 QML checks, core/Qt Clippy and the native build pass. Actual Ubuntu dark/light views verify both reference planes. Design. | [UI-QAM-MER-01](development/checklist.md#task-ui-qam-mer-01); [UI-QAM-MER-01](development/ui-checklist.md#task-ui-qam-mer-01). Historical checked. | [architecture](architecture.md#nearest-point-mer-estimator-ui-qam-mer-01); [mer.rs](../crates/ofdm/src/signal_processing/modulation/mer.rs), [receiver](../apps/qt/src/instruments/receiver) | Tests: [constellation_mer.rs](../crates/ofdm/tests/constellation_mer.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qam-mer/design](development/features/qam-mer/design.md#ui-qam-mer-01--calculated-constellation-mer). |
| <a id="ds-ui-input-restore-01"></a>**UI-INPUT-RESTORE-01**: Replace demo actions with Restore Workbench in DPD, Channel Estimate and RxMER; restore accepted data/options without Apply. Remove production demo invokables; verify drafts, empty states and WB reload. Qt Clippy, 62 Rust tests, 55 QML checks and native build pass; actual Ubuntu dark/light views verify restoration from an opened WB. Design. | [UI-INPUT-RESTORE-01](development/checklist.md#task-ui-input-restore-01); [UI-INPUT-RESTORE-01](development/ui-checklist.md#task-ui-input-restore-01). Historical checked. | [architecture](architecture.md#restore-workbench-inputs-ui-input-restore-01); [workbench](../apps/qt/src/workbench), [shell](../apps/qt/qml/shell) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [restore-workbench-inputs/design](development/features/restore-workbench-inputs/design.md#ui-input-restore-01--restore-accepted-workbench-inputs). |
| <a id="ds-ui-qam-runs-02"></a>**UI-QAM-RUNS-02**: Add captured Run Profile selection and bounded QAM history, keeping wire IDs distinct. Show matched TX-reference MER alongside nearest-point estimates; verify channel193 impairment, transport invariance, archive identity/bounds, missing states, filters and native dark/light views. 14 scoped core cases, 65 adapter tests and 55 QML checks pass, with core/Qt Clippy, no-default-feature MER tests and the latest native build. Design. | [UI-QAM-RUNS-02](development/checklist.md#task-ui-qam-runs-02); [UI-QAM-RUNS-02](development/ui-checklist.md#task-ui-qam-runs-02). Historical checked. | [architecture](architecture.md#run-specific-qam-and-reference-mer-ui-qam-runs-02); [receiver](../apps/qt/src/instruments/receiver), [model.rs](../crates/ofdm/src/simulation/channel/constellation/model.rs) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qam-run-results/design](development/features/qam-run-results/design.md#ui-qam-runs-02--run-identity-and-transmitted-reference-mer). |


### RxMER Designer

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-rxmer-design-01"></a>**RXMER-DESIGN-01**: Bounded Rust recipe, immutable designed envelopes, profile-derived flat sequences, per-effect capture scopes, before/after preview, cancellable batch staging, WB roundtrip and continuous first-to-last PLC region in main RxMER and Designer charts. Full local Rust, 73 adapter and 63 QML checks pass, with core/Qt Clippy, the latest build and native dark/light inspection. Three native profiles compare 4,710,792 bits with zero errors. Design · Guide. | [RXMER-DESIGN-01](development/checklist.md#task-rxmer-design-01). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-01-validation), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-01-rxmer-designer). |
| <a id="ds-rxmer-design-02"></a>**RXMER-DESIGN-02**: Connect Designer wheel/drag/keyboard zoom and Full Span, add Zoom to PLC and first/last frequency context in both RxMER views, and strengthen the true-width PLC shading. Qt Clippy, 74 adapter tests, 64 QML checks and the latest native build pass; native zoom, capture changes and Home reset verified. Design. | [RXMER-DESIGN-02](development/checklist.md#task-rxmer-design-02). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-02-zoom-and-plc-visibility), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-02-zoom-and-visible-plc-context). |
| <a id="ds-rxmer-design-03"></a>**RXMER-DESIGN-03**: Add 1 dB tilt up/down buttons while retaining the editable field and fractional values. Verify keyboard steps, invalid drafts and existing limits; 65 QML checks and the native Qt build pass. Design. | [RXMER-DESIGN-03](development/checklist.md#task-rxmer-design-03). Historical checked; current refinement: [RXMER-DESIGN-08](requirements.md#ds-rxmer-design-08). | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-03-tilt-steps), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-03-one-db-tilt-controls). |
| <a id="ds-rxmer-design-04"></a>**RXMER-DESIGN-04**: Order tilt buttons low to high: decrease, then increase. Existing Designer QML checks and native build pass. Design. | [RXMER-DESIGN-04](development/checklist.md#task-rxmer-design-04). Historical checked; current refinement: [RXMER-DESIGN-08](requirements.md#ds-rxmer-design-08). | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-04-button-order), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-04-low-to-high-button-order). |
| <a id="ds-rxmer-design-05"></a>**RXMER-DESIGN-05**: Add an explicit Flip button to reverse signed tilt while keeping magnitude, scope and zoom. Verify both directions, keyboard activation, fractional values, zero and invalid drafts; 66 QML checks and the rebuilt native Qt application pass. Design. | [RXMER-DESIGN-05](development/checklist.md#task-rxmer-design-05). Historical checked; current refinement: [RXMER-DESIGN-07](requirements.md#ds-rxmer-design-07). | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-05-flip-action), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-05-reverse-tilt-direction). |
| <a id="ds-rxmer-design-06"></a>**RXMER-DESIGN-06**: Disable the tilt value, step/Flip actions and capture scope while RxMER tilt is unchecked; preserve drafts and keep other effects independent. All 67 QML checks and the native build pass. Design. | [RXMER-DESIGN-06](development/checklist.md#task-rxmer-design-06). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-06-inactive-tilt-controls), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-06-disable-inactive-tilt-controls). |
| <a id="ds-rxmer-design-07"></a>**RXMER-DESIGN-07**: Replace the tilt label and Flip action with a Low → High / Low ← High direction button before the signed dB field. Keep reversal, step controls and disabled guards; all 67 QML checks and the native build pass. Native direction changes verified. Design. | [RXMER-DESIGN-07](development/checklist.md#task-rxmer-design-07). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-07-direction-button), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-07-direction-button-replaces-tilt-label). |
| <a id="ds-rxmer-design-08"></a>**RXMER-DESIGN-08**: Stack small increase/decrease triangles beside the tilt field using the shared action style. Preserve input/shortcuts, bounds and disabled guards. All 80 adapter and 72 QML checks pass, with native dark/light verification and refreshed guide screenshots. Design. | [RXMER-DESIGN-08](development/checklist.md#task-rxmer-design-08). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-08-compact-stacked-tilt-arrows). |
| <a id="ds-rxmer-design-09"></a>**RXMER-DESIGN-09**: Add shared stacked arrows for ripple depth (1 dB) and crest spacing (1 MHz), preserving fractional editing, keyboard, bounds and zoom. All 80 adapter and 74 QML checks pass; native dark/light verified and guide screenshots refreshed. Design. | [RXMER-DESIGN-09](development/checklist.md#task-rxmer-design-09). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-09-ripple-depth-and-spacing-steppers). |
| <a id="ds-rxmer-design-10"></a>**RXMER-DESIGN-10**: Add stacked arrows to notch center, width, depth and edge, guarding capture-band and edge/width limits. Preserve editable values, keyboard and capture scope; same validation as above. Design. | [RXMER-DESIGN-10](development/checklist.md#task-rxmer-design-10). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-10-notch-field-steppers). |
| <a id="ds-rxmer-design-11"></a>**RXMER-DESIGN-11**: Save PNM 1.0 binary or full-precision JSON with crate-owned encoding, independent Designer/Rust guards, explicit binary range conversion, metadata/provenance and bounded single/collection saves. Native format popup/pickers, Python adapters and no-overwrite publication are covered by 547 Rust, 82 Qt adapter and 76 QML checks; native dark/light and installed viewer decoding pass. Windows runtime remains unverified. Plan. | [RXMER-DESIGN-11](development/checklist.md#task-rxmer-design-11). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [mod.rs](../crates/pnm-input/src/rxmer_export/mod.rs) | Tests: [rxmer_export.rs](../crates/pnm-input/tests/rxmer_export.rs), [test_rxmer_export.py](../bindings/python/tests/test_rxmer_export.py). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-11-binary-and-json-save). |


### Desktop build defaults

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-build-release-01"></a>**UI-BUILD-RELEASE-01**: Default local build/run to optimized release, preserve explicit debug actions and opt-in diagnostics. Shell/routing checks and native release startup pass. Multi-RxMER UI completes three profiles in 00:00:07; keep checks in the development profile. Windows is unverified. Design. | [UI-BUILD-RELEASE-01](development/checklist.md#task-ui-build-release-01). Historical checked. | [architecture](architecture.md#ui-iteration-and-vm-handoff); [qt-desktop.sh](../scripts/qt-desktop.sh) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py). Record: [parallel-local-tests/design](development/features/parallel-local-tests/design.md). |


### Windows desktop UI delivery

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-msi-01"></a>**UI-MSI-01**: Implement Windows x64 desktop-only MSI staging and CI, conditional native adapter/pickers, embedded icon, WiX artwork, Start menu and unchecked optional desktop shortcut. Share public examples/notices; preserve Ubuntu/security gates and exclude Windows CLI/macOS artifacts. Design · Local validation. | [UI-MSI-01](development/checklist.md#task-ui-msi-01). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md). |
| <a id="ds-ui-msi-02"></a>**UI-MSI-02**: Run native Windows build/MSI/lifecycle jobs and retain installer-byte reports/logs. v0.2.7 passes adapter/QML/build/MSI/default install/icon/runtime/repair/shortcut/uninstall and complete publication. Hosted evidence. | [UI-MSI-02](development/checklist.md#task-ui-msi-02). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-02-successful-native-release-validation). |
| <a id="ds-ui-msi-09"></a>**UI-MSI-09**: Interactively verify icons, wizard, native pickers and normal security prompts on Windows 11; verify upgrade/downgrade with user data preservation. CI repair does not establish these interactive outcomes. Scope. | [UI-MSI-09](development/checklist.md#task-ui-msi-09). Pending / partial. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-09-remaining-interactive-and-upgrade-review). |
| <a id="ds-ui-msi-03"></a>**UI-MSI-03**: Normalize bundled FAQ line endings and verify all catalog sections under LF/CRLF after the first hosted Windows adapter failure. All six focused FAQ tests pass; native MSI/lifecycle passes in v0.2.7. Design. | [UI-MSI-03](development/checklist.md#task-ui-msi-03). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-03-first-hosted-windows-faq-failure). |
| <a id="ds-ui-msi-04"></a>**UI-MSI-04**: Retain Windows QML text/JUnit/startup logs and exact exit status after run 43370737351 passes adapter tests but fails silently in QML. Verify file loggers with one local QML test; native cause remains pending. Design. | [UI-MSI-04](development/checklist.md#task-ui-msi-04). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-04-windows-qml-failure-needs-startuptest-logs). |
| <a id="ds-ui-msi-05"></a>**UI-MSI-05**: Fit About actions to text/padding with equal widths and responsive stacking; save QML screenshots relative to prepared output folders. Both focused layout/render checks pass, including larger fonts and themes. Hosted native MSI/lifecycle passes in v0.2.7; UI-MSI-09 remains open. Design. | [UI-MSI-05](development/checklist.md#task-ui-msi-05). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-05-confirmed-qml-layoutpath-repair). |
| <a id="ds-ui-msi-06"></a>**UI-MSI-06**: Preserve reviewed license resource bytes under Windows autocrlf checkout and report failing file/expected/actual hashes. Verify collection and rejection of altered licenses with isolated Git checkouts. Hosted native MSI/lifecycle passes in v0.2.7; UI-MSI-09 remains open. Design. | [UI-MSI-06](development/checklist.md#task-ui-msi-06). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-06-windows-notice-checksum-failure). |
| <a id="ds-ui-msi-07"></a>**UI-MSI-07**: Use a filename-extension-bearing MSI icon identifier consistently for Installed apps and shortcuts. Preserve installed checks and record missing registry fields clearly; five focused packaging tests pass. Native lifecycle acceptance passes in v0.2.7 under UI-MSI-02. Design. | [UI-MSI-07](development/checklist.md#task-ui-msi-07). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-07-installed-icon-registration). |
| <a id="ds-ui-msi-08"></a>**UI-MSI-08**: Verify the registered MSI ProductIcon through Windows Installer metadata and compare cached icon bytes with the branded source. Installed icon/hash and complete lifecycle checks pass in v0.2.7. Design. | [UI-MSI-08](development/checklist.md#task-ui-msi-08). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md#ui-msi-08-use-msi-product-metadata-for-icon-verification). |


### Example organization

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-examples-01"></a>**EXAMPLES-01**: Move standalone JSON requests/payload to `examples/cli/`, add file purposes/use cases/commands and an example-folder index, update consumers and exclude CLI requests from desktop payloads. Keep measurement data and workbenches separate; preserve all moved bytes. Design. | [EXAMPLES-01](development/checklist.md#task-examples-01). Historical checked. | [architecture](architecture.md#examples-01-separate-cli-example-requests); [README.md](../examples/README.md), [packaging](../tools/packaging) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py). Record: [example-layout/validation](development/features/example-layout/validation.md). |


### Command timing

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-tool-time-01"></a>**TOOL-TIME-01**: Report total elapsed hours/minutes/seconds and original exit status for local unit tests and branch/release push commands. Include compilation and delegated patch releases; retain existing validation guards. Verify success, failure, elapsed waits and release dry-runs with isolated tests. Design. | [TOOL-TIME-01](development/checklist.md#task-tool-time-01). Historical checked. | [architecture](architecture.md#lifecycle-validation-and-future-interfaces); [testing](../tools/testing) | Tests: [test_local.py](../tools/testing/test_local.py). Record: [command-timing/design](development/features/command-timing/design.md#tool-time-01-test-and-push-elapsed-summaries). |


### Windows control readability and spectrum PLC annotation

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-win-theme-01"></a>**UI-WIN-THEME-01**: Select Fusion through Qt's startup API; retain readable analyzer dropdown/spin text in both themes. Packaged smoke clears inherited style hints and verifies selection. Native Windows review pending. Design. | [UI-WIN-THEME-01](development/checklist.md#task-ui-win-theme-01). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows_desktop.py](../tools/packaging/windows_desktop.py), [application_style.rs](../apps/qt/src/platform/application_style.rs) | Tests: [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [windows-desktop/validation](development/features/windows-desktop/validation.md). |
| <a id="ds-sa-plc-01"></a>**SA-PLC-01**: Shade the first–last PLC carrier region from the frozen waveform layout across shared TX/channel/RX PSD views, clipped to zoom. Design. | [SA-PLC-01](development/checklist.md#task-sa-plc-01). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [receiver](../apps/qt/src/instruments/receiver), [response](../apps/qt/qml/instruments/response) | Tests: [tst_response_view.qml](../apps/qt/tests/tst_response_view.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md). |


### Windows in-app MSI updates

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-win-update-01"></a>**UI-WIN-UPDATE-01**: Replace the release-page action with verified MSI download, normal UAC installation and unelevated WB recovery/restart. Design. | [UI-WIN-UPDATE-01](development/checklist.md#task-ui-win-update-01). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows.rs](../apps/qt/src/platform/updates/windows.rs), [windows_handoff.ps1](../apps/qt/src/platform/updates/windows_handoff.ps1) | Tests: [tst_updates.qml](../apps/qt/tests/tst_updates.qml), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [windows-desktop/design](development/features/windows-desktop/design.md#ui-win-update-01-install-a-verified-msi-from-update). |
| <a id="ds-ui-win-update-02"></a>**UI-WIN-UPDATE-02**: Verify the native Windows upgrade handoff, UAC approval/cancel, MSI failure/rollback, 3010 handling, shortcut/location preservation and unelevated WB recovery. Local Linux checks do not establish these Windows installer lifecycle results. | [UI-WIN-UPDATE-02](development/checklist.md#task-ui-win-update-02). Pending / partial. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows.rs](../apps/qt/src/platform/updates/windows.rs), [windows_handoff.ps1](../apps/qt/src/platform/updates/windows_handoff.ps1) | Tests: [tst_updates.qml](../apps/qt/tests/tst_updates.qml), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [windows-desktop/design](development/features/windows-desktop/design.md#ui-win-update-02-native-upgrade-lifecycle-verification). |


### Documentation PNG audit

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-req-ds-doc-png-01"></a>**REQ-DS-DOC-PNG-01**: Review every tracked PNG and existing light variant; refresh outdated native captures and reconcile obsolete images/references. Design. | [UI-DOC-02](development/checklist.md#task-req-ds-doc-png-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [README.md](README.md), [images](images) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [user-guide-captures/design](development/features/user-guide-captures/design.md#ui-doc-02-audit-all-tracked-pngs-and-refresh-stale-ui-captures), [user-guide-captures/png-audit](development/features/user-guide-captures/png-audit.md). |


### High-level simulation API

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-api-rust-01"></a>**API-RUST-01**: Implement and validate the canonical high-level Rust API, binary/JSON adapters, independent selections and owned actual simulation reports. | [API-RUST-01](development/checklist.md#task-api-rust-01). Historical checked; current refinement: [API-SIMULATION-01](requirements.md#ds-api-simulation-01), [API-PROFILES-01](requirements.md#ds-api-profiles-01), [API-OCD-01](requirements.md#ds-api-ocd-01). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). |
| <a id="ds-api-py-01"></a>**API-PY-01**: Bind the validated Rust API with PyO3/maturin and test native wheels and parity on Ubuntu 22.04 x86_64 / CPython 3.10. Design, validation. Other targets unverified. | [API-PY-01](development/checklist.md#task-api-py-01). Historical checked; current refinement: [API-SIMULATION-01](requirements.md#ds-api-simulation-01), [API-PROFILES-01](requirements.md#ds-api-profiles-01), [API-OCD-01](requirements.md#ds-api-ocd-01). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). |
| <a id="ds-api-examples-01"></a>**API-EXAMPLES-01**: Centralize runnable Rust and Python API examples under `examples/api`, register Rust Cargo targets, and link setup/run instructions from the examples index. Both workflows run successfully. Examples, design. | [API-EXAMPLES-01](development/checklist.md#task-api-examples-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-01-follow-up), [high-level-api/design](development/features/high-level-api/design.md#api-examples-01-central-rust-and-python-examples). |
| <a id="ds-api-integration-01"></a>**API-INTEGRATION-01**: Document downstream Python dependency setup, version/tag and wheel pins, optional extras and current availability; validate exact-version installation in a fresh local environment. Integration guide. | [API-INTEGRATION-01](development/checklist.md#task-api-integration-01). Historical checked. | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). |
| <a id="ds-api-integration-02"></a>**API-INTEGRATION-02**: Require a separate Python installation-guide updater in save/push/release flows, include refreshes in release commits and exact recovery, and block stale/invalid instructions. Disposable Git workflow and generator tests pass. Ignore generated/downloaded archives. Requirements, workflow. | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02). Historical checked. | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-integration-02-delivery-tooling-follow-up). |
| <a id="ds-api-examples-02"></a>**API-EXAMPLES-02**: Explain both high-level examples with filename headers, numbered walkthrough remarks and named configuration constants with units. Rust checking, Python execution and equivalence checks pass. Design. | [API-EXAMPLES-02](development/checklist.md#task-api-examples-02). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md), [high-level-api/design](development/features/high-level-api/design.md#api-examples-02-developer-walkthrough-comments). |
| <a id="ds-api-examples-03"></a>**API-EXAMPLES-03**: Use the channel-193 multi-rxmer-2 binaries in a Python collection walkthrough. Load all 132 RxMER captures, average in Rust and run with explicit unity H. Document desktop-WB differences. Native run completed with 83 codewords, 0 corrected/uncorrectable and BER 0. Design. | [API-EXAMPLES-03](development/checklist.md#task-api-examples-03). Historical checked; current refinement: [API-EXAMPLES-08](requirements.md#ds-api-examples-08), [API-EXAMPLES-10](requirements.md#ds-api-examples-10). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md), [high-level-api/design](development/features/high-level-api/design.md#api-examples-03-channel-193-python-collection-example). |
| <a id="ds-api-examples-04"></a>**API-EXAMPLES-04**: Restore the single-capture run_single_capture.py and keep the collection workflow in run_multi_rxmer_mce.py. Both native examples, exact restoration, import-only checks and documentation links pass. Design. | [API-EXAMPLES-04](development/checklist.md#task-api-examples-04). Historical checked; current refinement: [API-EXAMPLES-08](requirements.md#ds-api-examples-08), [API-EXAMPLES-10](requirements.md#ds-api-examples-10). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md), [high-level-api/design](development/features/high-level-api/design.md#api-examples-04-keep-collection-walkthrough-separate). |
| <a id="ds-api-examples-05"></a>**API-EXAMPLES-05**: Run the collection independently for captured Profiles 0, 3 and 4. Label actual FEC/BER and host time per profile; document explicit fixture selection and 1 ms observation. All three native runs complete; syntax/import and documentation checks pass. Design. | [API-EXAMPLES-05](development/checklist.md#task-api-examples-05). Historical checked; current refinement: [API-INPUT-02](requirements.md#ds-api-input-02), [API-EXAMPLES-10](requirements.md#ds-api-examples-10). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md), [high-level-api/design](development/features/high-level-api/design.md#api-examples-05-independent-collection-runs-per-captured-profile). |
| <a id="ds-api-input-02"></a>**API-INPUT-02**: Discover sorted validated profile IDs in the public Rust API and native binary/JSON loader, mirror file/byte conveniences in Python and use discovery in the all-profile collection example. Require Rust-first shared API evolution; 16 Rust API tests, 7 Python API tests and all four selection-policy parity modes pass. Design. | [API-INPUT-02](development/checklist.md#task-api-input-02). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#profile-catalog-discovery-api-input-02). |
| <a id="ds-api-input-03"></a>**API-INPUT-03**: Retain the source profile catalog in loaded DPDs; expose Rust profile_ids() and Python profile_ids without file access or decoding. Keep static discovery; 17 Rust API tests, 8 Python API tests, source-removal/ownership checks and four policy parity modes pass. Design. | [API-INPUT-03](development/checklist.md#task-api-input-03). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#loaded-dpd-catalog-member-api-input-03). |
| <a id="ds-api-example-ber-01"></a>**API-EXAMPLE-BER-01**: Display Python example BER in scientific notation using a named precision constant; keep numeric Rust/Python API values intact. Both fixture-backed examples pass, retaining unavailable values explicitly. Design. | [API-EXAMPLE-BER-01](development/checklist.md#task-api-example-ber-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-example-ber-01-scientific-ber-presentation), [high-level-api/design](development/features/high-level-api/design.md#api-example-ber-01-readable-python-ber-output). |
| <a id="ds-api-receiver-01"></a>**API-RECEIVER-01**: Expose existing receiver correction in Rust options and mirror it in typed Python options/reports. Match the curated collection example to Qt; all three profiles' FEC agree. Validate direct-engine parity, all four RxMER policies across three receiver modes and independent ownership. Design, validation and measured delta. | [API-RECEIVER-01](development/checklist.md#task-api-receiver-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-receiver-01-receiver-settings-and-ui-parity). |
| <a id="ds-api-receiver-02"></a>**API-RECEIVER-02**: Expand the typed MCE name to MerlinChannelEstimated / MERLIN_CHANNEL_ESTIMATED. Preserve saved JSON compatibility and accept the expanded input spelling; verify native Python options, all receiver/selection parity modes, MCE tests and Rust forwarding. Design. | [API-RECEIVER-02](development/checklist.md#task-api-receiver-02). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-receiver-02-expanded-mce-names). |
| <a id="ds-api-examples-06"></a>**API-EXAMPLES-06**: Select MERLIN_CHANNEL_ESTIMATED in run_multi_rxmer_mce.py; document the saved WB's different receiver mode. Run all three profiles with the native MCE API and preserve the pilot-mode UI comparison test. Design. | [API-EXAMPLES-06](development/checklist.md#task-api-examples-06). Historical checked; current refinement: [API-EXAMPLES-08](requirements.md#ds-api-examples-08), [API-EXAMPLES-10](requirements.md#ds-api-examples-10). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-06-default-collection-mce-run), [high-level-api/design](development/features/high-level-api/design.md#api-examples-06-use-mce-in-the-collection-walkthrough). |
| <a id="ds-api-examples-07"></a>**API-EXAMPLES-07**: Add run_multi_rxmer_cm_chan_est.py for the bundled saved WB's pilot receiver. Share the MCE walkthrough through an explicit mode argument; validate actual profile FEC against native UI anchors and shared WB settings. Design. | [API-EXAMPLES-07](development/checklist.md#task-api-examples-07). Historical checked; current refinement: [API-EXAMPLES-08](requirements.md#ds-api-examples-08), [API-EXAMPLES-10](requirements.md#ds-api-examples-10). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-07-saved-ui-companion), [high-level-api/design](development/features/high-level-api/design.md#api-examples-07-separate-saved-ui-comparison-entry-point). |
| <a id="ds-api-wb-01"></a>**API-WB-01**: Add bounded saved-WB file/byte loading in Rust and Python for accepted PRBS15/static single/average configurations. Preserve embedded originals, saved receiver/channel settings and profile/capture selections; preflight without running and return owned per-profile batches on explicit run. Verify native Rust/Python/UI FEC parity, integrity, limits and cancellation. Design, user contract. | [API-WB-01](development/checklist.md#task-api-wb-01). Historical checked; current refinement: [API-WB-02](requirements.md#ds-api-wb-02), [API-WB-03](requirements.md#ds-api-wb-03). | [architecture](architecture.md#saved-workbench-api-input); [workbench.rs](../crates/pnm-input/src/workbench.rs), [designer.rs](../crates/pnm-input/src/workbench/designer.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs), [designer_workbench.rs](../crates/pnm-input/tests/designer_workbench.rs), [test_workbench.py](../bindings/python/tests/test_workbench.py), [test_designer_workbench.py](../bindings/python/tests/test_designer_workbench.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-wb-01-native-saved-workbench-import), [high-level-api/validation](development/features/high-level-api/validation.md#api-wb-01-native-saved-wb-loading). |
| <a id="ds-api-wb-02"></a>**API-WB-02**: Set CmChannelEstimated in all runnable bundled WBs and the saved-UI companion; retain the separate MCE walkthrough. Verify unchanged capture/settings payloads, package defaults and Rust/Python/native UI FEC. Design. | [API-WB-02](development/checklist.md#task-api-wb-02). Historical checked. | [architecture](architecture.md#saved-workbench-api-input); [workbench.rs](../crates/pnm-input/src/workbench.rs), [designer.rs](../crates/pnm-input/src/workbench/designer.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs), [designer_workbench.rs](../crates/pnm-input/tests/designer_workbench.rs), [test_workbench.py](../bindings/python/tests/test_workbench.py), [test_designer_workbench.py](../bindings/python/tests/test_designer_workbench.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-wb-02-align-bundled-workbenches-with-the-default-receiver), [high-level-api/validation](development/features/high-level-api/validation.md#api-wb-02-supplied-channel-bundled-defaults). |
| <a id="ds-api-wb-03"></a>**API-WB-03**: Load saved RxMER Designer flat/imported recipes directly through both Rust and Python APIs; preserve provenance and verify bounded engine parity. Design. | [API-WB-03](development/checklist.md#task-api-wb-03). Historical checked. | [architecture](architecture.md#saved-workbench-api-input); [workbench.rs](../crates/pnm-input/src/workbench.rs), [designer.rs](../crates/pnm-input/src/workbench/designer.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs), [designer_workbench.rs](../crates/pnm-input/tests/designer_workbench.rs), [test_workbench.py](../bindings/python/tests/test_workbench.py), [test_designer_workbench.py](../bindings/python/tests/test_designer_workbench.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-wb-03-validation-ubuntu-x64-cpython-310), [high-level-api/validation](development/features/high-level-api/validation.md). |
| <a id="ds-ch-est-rx-13"></a>**CH-EST-RX-13**: Unify receiver configuration in the agreed three-mode dropdown, hide MCE averaging outside MCE and document channel/noise sources, defaults, unity behavior, pilot history, API names and saved WB semantics. Design, user guide. | [CH-EST-RX-13](development/checklist.md#task-ch-est-rx-13); [CH-EST-RX-13](development/ui-checklist.md#task-ch-est-rx-13). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [filter.rs](../crates/ofdm/src/signal_processing/channel/filter.rs), [mod.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/mod.rs) | Tests: [mce.rs](../crates/ofdm/tests/mce.rs), [channel_filter.rs](../crates/ofdm/tests/channel_filter.rs). Record: [independent-channel-estimation/implementation](development/features/independent-channel-estimation/implementation.md#ch-est-rx-13-receiver-mode-wording-and-guide). |
| <a id="ds-api-examples-08"></a>**API-EXAMPLES-08**: Rename Python examples by capture workflow and receiver mode; update imports, tests and current documentation. All four native examples passed in 14.72s; three receiver tests passed in 9.58s. Design. | [API-EXAMPLES-08](development/checklist.md#task-api-examples-08). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-08-descriptive-python-filenames), [high-level-api/design](development/features/high-level-api/design.md#api-examples-08-descriptive-python-entry-point-names). |
| <a id="ds-api-examples-09"></a>**API-EXAMPLES-09**: Rename Rust examples and Cargo targets by workflow; update current guide links and commands. Formatting, example Clippy and all three release example executions pass (13.96s total). Design. | [API-EXAMPLES-09](development/checklist.md#task-api-examples-09). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-09-matching-rust-filenames-and-cargo-targets), [high-level-api/design](development/features/high-level-api/design.md#api-examples-09-matching-descriptive-rust-entry-points). |
| <a id="ds-api-examples-10"></a>**API-EXAMPLES-10**: Provide matching Rust MCE and supplied-CM-channel collection examples with a shared documented workflow, named settings and captured-profile discovery. Format/Clippy and exact native Rust/Python FEC/status/BER parity pass for all three profiles in both modes (20.60s). Design. | [API-EXAMPLES-10](development/checklist.md#task-api-examples-10). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-10-matching-rust-collection-examples), [high-level-api/design](development/features/high-level-api/design.md#api-examples-10-equivalent-rust-and-python-collection-workflows). |
| <a id="ds-api-examples-11"></a>**API-EXAMPLES-11**: Add matching designed-RxMER Rust/Python examples, preserving custom binary/sidecar provenance and discovering all compatible profiles. Formatting/Clippy and exact FEC/status/displayed-BER parity pass in 46.59s; inputs are unchanged and Python import starts no simulation. Design. | [API-EXAMPLES-11](development/checklist.md#task-api-examples-11). Historical checked; current refinement: [API-EXAMPLES-12](requirements.md#ds-api-examples-12). | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-11-custom-designed-rxmer), [high-level-api/design](development/features/high-level-api/design.md#api-examples-11-custom-designed-rxmer-capture). |
| <a id="ds-api-examples-12"></a>**API-EXAMPLES-12**: Move designed binary/sidecar into the shared measurements collection with a README and updated examples/indexes. Both native examples pass from the new paths (42.99s); result fields and input hashes remain unchanged. Formatting, Clippy and links pass. Design. | [API-EXAMPLES-12](development/checklist.md#task-api-examples-12). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-examples-12-shared-designed-capture-location), [high-level-api/design](development/features/high-level-api/design.md#api-examples-12-place-designed-input-with-shared-capture-examples). |
| <a id="ds-ui-doc-03"></a>**UI-DOC-03**: Refresh the general Receiver Configuration PNG and add a native three-choice dropdown capture. Review current MCE/Designer PNGs, correct visible-state captions and link designed-input API examples. Design, audit. | [UI-DOC-03](development/checklist.md#task-ui-doc-03). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [README.md](README.md), [images](images) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [user-guide-captures/design](development/features/user-guide-captures/design.md#ui-doc-03-validation), [user-guide-captures/png-audit](development/features/user-guide-captures/png-audit.md#ui-doc-03-receiver-mode-follow-up). |
| <a id="ds-api-doc-01"></a>**API-DOC-01**: Build searchable Rust/Python high-level API HTML from public source documentation and the actual native extension. Discover and verify every exported Python class/member; provide a shared local landing page, strict warning gates, pinned tooling and build instructions. Clean hidden interpreter parameters, document units/defaults and fix literal Rust bracket links. Offline build, browser review and targeted native checks pass. Design, build guide. | [API-DOC-01](development/checklist.md#task-api-doc-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [python](../tools/python) | Tests: [test_api_reference.py](../tools/python/test_api_reference.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-doc-01-generated-high-level-rust-and-python-references). |
| <a id="ds-api-wheel-01"></a>**API-WHEEL-01**: Configure release-only native Ubuntu/Windows CPython 3.10 wheel builds, clean installed-example/FEC gates and required checksummed release assets. Preserve security gates and link a separate Python release guide from installation/integration docs. Local Linux wheel smoke passes; hosted Windows execution remains unverified until the native job runs. Design, release guide. | [API-WHEEL-01](development/checklist.md#task-api-wheel-01). Historical checked. | [architecture](architecture.md#api-wheel-01-native-python-release-validation); [build_wheel.py](../tools/python/build_wheel.py), [release.yml](../.github/workflows/release.yml) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-wheel-01-release-only-native-wheel-validation), [high-level-api/design](development/features/high-level-api/design.md#api-wheel-01-release-only-native-wheel-build-and-installed-example-gate). |
| <a id="ds-fec-sparse-01"></a>**FEC-SPARSE-01**: Replace NCP/PLC dense encoder setup with private sparse substitution; preserve the payload staircase path and all decoder behavior. Baseline before integration, synthetic bit-exact/syndrome tests, independent NCP/PLC basis oracles, 38 codec tests and focused API recovery pass. Report isolated setup/encode timings separately from compilation and simulation. Design and validation. | [FEC-SPARSE-01](development/checklist.md#task-fec-sparse-01). Historical checked. | [architecture](architecture.md#sparse-control-channel-encoding-fec-sparse-01); [sparse_encoder.rs](../crates/fec-codec/src/sparse_encoder.rs), [codec.rs](../crates/fec-codec/src/ncp/codec.rs), [encoding.rs](../crates/fec-codec/src/plc/encoding.rs) | Tests: [tests.rs](../crates/fec-codec/src/sparse_encoder/tests.rs), [ncp.rs](../crates/fec-codec/tests/ncp.rs), [plc_encoding.rs](../crates/fec-codec/tests/plc_encoding.rs). Record: [sparse-encoder/design](development/features/sparse-encoder/design.md#fec-sparse-01-sparse-systematic-encoder-setup). |
| <a id="ds-api-wheel-02"></a>**API-WHEEL-02**: Select native PowerShell for Windows wheel compilation and Bash for Ubuntu; fold shared smoke arguments portably. Local YAML and tooling checks pass; hosted patch verification remains pending. Failure and fix. | [API-WHEEL-02](development/checklist.md#task-api-wheel-02). Historical checked. | [architecture](architecture.md#api-wheel-01-native-python-release-validation); [build_wheel.py](../tools/python/build_wheel.py), [release.yml](../.github/workflows/release.yml) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-wheel-02-windows-linker-path-failure-and-patch), [high-level-api/design](development/features/high-level-api/design.md#api-wheel-02-native-windows-shell-and-linker). |
| <a id="ds-rxmer-design-12"></a>**RXMER-DESIGN-12**: Keep notch arrows clickable with native larger fonts using measured-width layout reflow. All 82 adapter/77 QML checks and native mouse stepping pass; refresh Designer images. Hosted Windows patch is pending. Design. | [RXMER-DESIGN-12](development/checklist.md#task-rxmer-design-12). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-12-native-font-clipping-regression), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-12-font-aware-notch-field-layout). |
| <a id="ds-rxmer-design-13"></a>**RXMER-DESIGN-13**: Flat-baseline seeded random variation before frequency effects, imported-source guards, reproducible previews and saved recipes. Design. | [RXMER-DESIGN-13](development/checklist.md#task-rxmer-design-13). Historical checked. | [architecture](architecture.md#rxmer-design-01-designed-rxmer-and-profile-plc-overlays); [rxmer_designer.rs](../crates/ofdm/src/signal_processing/rxmer_designer.rs), [rxmer_designer.rs](../apps/qt/src/bridge/rxmer_designer.rs) | Tests: [rxmer_designer.rs](../crates/ofdm/tests/rxmer_designer.rs), [tst_rxmer_designer.qml](../apps/qt/tests/tst_rxmer_designer.qml). Record: [rxmer-designer/validation](development/features/rxmer-designer/validation.md#rxmer-design-13-seeded-starting-variation), [rxmer-designer/design](development/features/rxmer-designer/design.md#rxmer-design-13--flat-baseline-random-variation). |
| <a id="ds-ci-msvc-01"></a>**CI-MSVC-01**: Replace the Node 20 MSVC action in both Windows jobs with a shared native PowerShell initializer, retaining VS 2022/x64 and verifying linker selection. Workflow wiring checked locally; hosted execution of the replacement remains pending. See Windows setup. | [CI-MSVC-01](development/checklist.md#task-ci-msvc-01). Historical checked. | [architecture](architecture.md#shared-master-and-release-validation-ci-consolidate-01); [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [workbench-configuration/design](development/features/workbench-configuration/design.md). |
| <a id="ds-api-doc-02"></a>**API-DOC-02**: Name guides Basic Rust API / Basic Python API and separate Rust/Python example syntax in coding-style and public-api guidance. Preserve API signatures and package names. Documentation tooling and syntax checks pass. | [API-DOC-02](development/checklist.md#task-api-doc-02). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [python](../tools/python) | Tests: [test_api_reference.py](../tools/python/test_api_reference.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-doc-02-basic-python-api-terminology-and-language-separation). |
| <a id="ds-ubuntu-compat-01"></a>**UBUNTU-COMPAT-01**: Validate original 22.04 desktop/wheel assets on Ubuntu 24.04 and 26.04 before release. Matrix and Ubuntu 22.04+ documentation implemented; local workflow/tooling checks pass, hosted execution pending. | [UBUNTU-COMPAT-01](development/checklist.md#task-ubuntu-compat-01). Pending / partial. | [architecture](architecture.md#ubuntu-2204-released-package-compatibility); [ci.yml](../.github/workflows/ci.yml), [desktop.py](../tools/packaging/desktop.py) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md), [high-level-api/design](development/features/high-level-api/design.md). |
| <a id="ds-api-ocd-01"></a>**API-OCD-01**: Derive captured bandwidth only when one supported width fits. Preserve explicit settings and ambiguity errors; simplify both languages' examples. Boundary, binary/JSON and explicit/derived checks pass. | [API-OCD-01](development/checklist.md#task-api-ocd-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-ocd-01-and-api-profiles-01-local-ubuntu-validation). |
| <a id="ds-api-profiles-01"></a>**API-PROFILES-01**: Retain compatible profiles in the controller, expose IDs/lookup/selection and run_all_profiles with bounded preflight/cancellation. Keep legacy construction/setters. Rust/Python FEC parity and native examples pass locally; other native platforms remain unverified for this change. | [API-PROFILES-01](development/checklist.md#task-api-profiles-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-ocd-01-and-api-profiles-01-local-ubuntu-validation). |
| <a id="ds-api-protocol-01"></a>**API-PROTOCOL-01**: Share the DOCSIS 3.1 loading default between Rust and Python; preserve explicit protocol selection and update both examples. Fixture default/explicit parity, Rust example compilation and Python API tests pass locally; other platforms remain unverified for this change. | [API-PROTOCOL-01](development/checklist.md#task-api-protocol-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-protocol-01--default-interpretation-validation). |
| <a id="ds-api-simulation-01"></a>**API-SIMULATION-01**: Add idle Simulation sessions in Rust and Python with atomic profile loading, retained IDs and legacy compatibility. Update both language examples and generated references. Shared example checks report elapsed time; CI retains Rust logs and native wheel timing evidence. Local fixture, cancellation, parity and documentation checks pass; hosted Windows/newer Ubuntu execution remains unverified for this change. | [API-SIMULATION-01](development/checklist.md#task-api-simulation-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-simulation-01--session-and-timed-example-validation). |
| <a id="ds-api-cmce-name-01"></a>**API-CMCE-NAME-01**: Use CmChannelEstimated / CM_CHANNEL_ESTIMATED in Rust/Python, preserving compatibility names and saved response settings. Local compatibility, workbench, engine parity and wheel checks pass. | [API-CMCE-NAME-01](development/checklist.md#task-api-cmce-name-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). |
| <a id="ds-api-cmce-example-01"></a>**API-CMCE-EXAMPLE-01**: Add paired Rust/Python single-capture examples explicitly applying a captured CM response to the channel and receiver. Fixture FEC parity, native isolated-wheel checks, example Clippy and timing tests pass locally. Include both in timed build/release example checks; hosted Windows/Ubuntu release execution remains unverified for this change. | [API-CMCE-EXAMPLE-01](development/checklist.md#task-api-cmce-example-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [rust](../examples/api/rust), [python](../examples/api/python) | Tests: [test_api_examples.py](../tools/testing/test_api_examples.py), [test_receiver.py](../bindings/python/tests/test_receiver.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-cmce-example-01--explicit-channel-estimate-walkthroughs), [high-level-api/design](development/features/high-level-api/design.md#api-cmce-example-01--explicit-channel-and-receiver-response). |
| <a id="ds-api-cpa-01"></a>**API-CPA-01**: Name Common Path Attenuator (CPA) in the core model, UI and terminology. Add Rust/Python launch gain and CPA setters/getters; preserve saved JSON, validate finite bounds and reject changes atomically. Targeted Rust/Python and full Qt checks pass; platform scope remains Linux. | [API-CPA-01](development/checklist.md#task-api-cpa-01). Historical checked. | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md#api-cpa-01--shared-gain-and-cpa-controls). |
| <a id="ds-api-install-01"></a>**API-INSTALL-01**: Separate Python-only source/release installer with local prerequisite checks and release checksum verification; see installer guide. | [API-INSTALL-01](development/checklist.md#task-api-install-01). Historical checked. | [architecture](architecture.md#python-only-installation); [install-local.sh](../scripts/python-api/install-local.sh), [python](../tools/python) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-install-01--standalone-python-api-installer). |
| <a id="ds-req-ds-py-install-host-01"></a>**REQ-DS-PY-INSTALL-HOST-01**: - [ ] Validate Python installer end to end on native Windows and clean Ubuntu hosts. | [unnumbered task](development/checklist.md#task-req-ds-py-install-host-01). Pending / partial. | [architecture](architecture.md#python-only-installation); [install-local.sh](../scripts/python-api/install-local.sh), [python](../tools/python) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/design](development/features/high-level-api/design.md). |
| <a id="ds-api-install-02"></a>**API-INSTALL-02**: Accept newer host Python, match downloaded wheel ABI and configure shared CI/CD source-installer checks with version-only smoke. | [API-INSTALL-02](development/checklist.md#task-api-install-02). Historical checked. | [architecture](architecture.md#python-only-installation); [install-local.sh](../scripts/python-api/install-local.sh), [python](../tools/python) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-install-02-host-python-and-installer-ci). |
| <a id="ds-req-ds-py-install-ci-01"></a>**REQ-DS-PY-INSTALL-CI-01**: - [ ] Observe hosted installer results across the configured platform matrix. | [unnumbered task](development/checklist.md#task-req-ds-py-install-ci-01). Pending / partial. | [architecture](architecture.md#python-only-installation); [install-local.sh](../scripts/python-api/install-local.sh), [python](../tools/python) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/design](development/features/high-level-api/design.md). |
| <a id="ds-pnm-ref-01"></a>**PNM-REF-01**: Review V05/V02 RxMER/FEC guidance; add pinned text, PDF download links and discoverable reference skill. | [PNM-REF-01](development/checklist.md#task-pnm-ref-01). Historical checked. | [architecture](architecture.md#stage-references); [SKILL.md](../skills/docsis-pnm-rxmer-fec/SKILL.md) | Record: [pnm-reference-skill/design](development/features/pnm-reference-skill/design.md#pnm-ref-01-pnm-rxmer-and-fec-reference-skill). |
| <a id="ds-req-ds-field-fec-01"></a>**REQ-DS-FIELD-FEC-01**: - [ ] Implement profile-relative carrier counts and verify field-capture FEC behavior; reference-skill completion does not establish simulation validity. | [unnumbered task](development/checklist.md#task-req-ds-field-fec-01). Pending / partial. | [architecture](architecture.md#stage-references); [SKILL.md](../skills/docsis-pnm-rxmer-fec/SKILL.md) | Record: [pnm-reference-skill/design](development/features/pnm-reference-skill/design.md). |


### Reference experiments

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-wb-cl-01"></a>**WB-CL-01**: Add synthetic 192 MHz / 8 MHz notch and flat-baseline workbenches, PLC at 333 MHz, with explicit assumptions and actual decoder validation. Design. | [WB-CL-01](development/checklist.md#task-wb-cl-01). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [cl-192mhz-8mhz-notch](../examples/workbench/cl-192mhz-8mhz-notch), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs). Record: [cl-notch-workbench/design](development/features/cl-notch-workbench/design.md#wb-cl-01-cablelabs-style-192-mhz-notch-workbench), [cl-notch-workbench/comparison-sim-1](development/features/cl-notch-workbench/comparison-sim-1.md). |
| <a id="ds-ui-tooltip-01"></a>**UI-TOOLTIP-01**: Centralize one-second tooltip timing across application views and shared controls. Design. | [UI-TOOLTIP-01](development/checklist.md#task-ui-tooltip-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [components](../apps/qt/qml/components) | Tests: [tst_tooltips.qml](../apps/qt/tests/tst_tooltips.qml). Record: [tooltip-delay/design](development/features/tooltip-delay/design.md#ui-tooltip-01-one-shared-tooltip-delay). |
| <a id="ds-wb-cl-02"></a>**WB-CL-02**: Add a separately saved notch experiment with a frequency-aligned, interpolated CM channel response, retained source provenance and decoder run. Design. | [WB-CL-02](development/checklist.md#task-wb-cl-02). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [cl-192mhz-8mhz-notch](../examples/workbench/cl-192mhz-8mhz-notch), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs). Record: [cl-notch-workbench/design](development/features/cl-notch-workbench/design.md#wb-cl-02-validation), [cl-notch-workbench/comparison-sim-1](development/features/cl-notch-workbench/comparison-sim-1.md). |
| <a id="ds-qam-mode-01"></a>**QAM-MODE-01**: Verify the MCE-to-QAM sample path and identify the recorded equalizer for current and archived runs. Design. | [QAM-MODE-01](development/checklist.md#task-qam-mode-01). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-wb-cl-03"></a>**WB-CL-03**: Run the preserved user Designer WB, an aligned 8 MHz notch and a unity notch-depth sweep using actual FEC; record timing and honest CableLabs comparison limits. Results. | [WB-CL-03](development/checklist.md#task-wb-cl-03). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [cl-192mhz-8mhz-notch](../examples/workbench/cl-192mhz-8mhz-notch), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs). Record: [cl-notch-workbench/design](development/features/cl-notch-workbench/design.md#wb-cl-03-user-design-versus-cl-style-notch-sweep), [cl-notch-workbench/comparison-sim-1](development/features/cl-notch-workbench/comparison-sim-1.md#wb-cl-03-saved-designer-comparison). |
| <a id="ds-wb-cl-04"></a>**WB-CL-04**: Include the approved CL bundle in CLI packaging inventory assertions while retaining integrity/license checks. All 23 packaging tests passed. Design. | [WB-CL-04](development/checklist.md#task-wb-cl-04). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [cl-192mhz-8mhz-notch](../examples/workbench/cl-192mhz-8mhz-notch), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs). Record: [cl-notch-workbench/design](development/features/cl-notch-workbench/design.md#wb-cl-04-release-packaging-inventory-regression), [cl-notch-workbench/comparison-sim-1](development/features/cl-notch-workbench/comparison-sim-1.md). |
| <a id="ds-api-install-03"></a>**API-INSTALL-03**: Fetch locked native dependencies before offline wheel license collection; test Linux/Windows command ordering and fetch failures. A local cold-cache wheel build and installed version smoke passed. Hosted installer validation remains tracked above. | [API-INSTALL-03](development/checklist.md#task-api-install-03). Historical checked. | [architecture](architecture.md#python-only-installation); [install-local.sh](../scripts/python-api/install-local.sh), [python](../tools/python) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-install-03-cold-cache-wheel-dependency-preparation). |
| <a id="ds-wb-cl-05"></a>**WB-CL-05**: Preserve capture/workbench bytes across Git line-ending settings and test a Windows-style checkout without weakening provenance. All 24 packaging tests pass locally; hosted Windows validation is pending. | [WB-CL-05](development/checklist.md#task-wb-cl-05). Historical checked. | [architecture](architecture.md#minimum-acceptance-merlin-closed-loop); [cl-192mhz-8mhz-notch](../examples/workbench/cl-192mhz-8mhz-notch), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs). Record: [cl-notch-workbench/design](development/features/cl-notch-workbench/design.md#wb-cl-05-preserve-fixture-bytes-on-windows-checkout), [cl-notch-workbench/comparison-sim-1](development/features/cl-notch-workbench/comparison-sim-1.md). |
| <a id="ds-ubuntu-compat-02"></a>**UBUNTU-COMPAT-02**: Derive external package requirements from direct ELF dependencies; leave transitive system dependencies to the target OS. Packaging regressions and a real local Qt runtime deployment pass. Hosted Ubuntu 26.04 lifecycle validation remains pending. | [UBUNTU-COMPAT-02](development/checklist.md#task-ubuntu-compat-02). Historical checked. | [architecture](architecture.md#ubuntu-2204-released-package-compatibility); [ci.yml](../.github/workflows/ci.yml), [desktop.py](../tools/packaging/desktop.py) | Tests: [test_desktop.py](../tools/packaging/test_desktop.py). Record: [ubuntu-desktop/design](development/features/ubuntu-desktop/design.md#ubuntu-compat-02-direct-elf-package-dependencies), [high-level-api/design](development/features/high-level-api/design.md). |
| <a id="ds-api-install-04"></a>**API-INSTALL-04**: Pin the child Windows wheel build to the active MSVC compiler/linker instead of Git Bash link; retain native installer CI. All 16 wheel-tool tests pass; native Windows confirmation is pending. | [API-INSTALL-04](development/checklist.md#task-api-install-04). Historical checked. | [architecture](architecture.md#python-only-installation); [install-local.sh](../scripts/python-api/install-local.sh), [python](../tools/python) | Tests: [test_wheel_tools.py](../tools/python/test_wheel_tools.py). Record: [high-level-api/design](development/features/high-level-api/design.md#api-install-04-native-msvc-linker-from-git-bash). |
| <a id="ds-win-update-03"></a>**WIN-UPDATE-03**: Implement native MSI handoff paths and explicit, user-reviewed clipboard/issue reporting for update failures. Local Rust/QML checks pass; see the Windows desktop design. | [WIN-UPDATE-03](development/checklist.md#task-win-update-03). Historical checked. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows.rs](../apps/qt/src/platform/updates/windows.rs), [windows_handoff.ps1](../apps/qt/src/platform/updates/windows_handoff.ps1) | Tests: [tst_updates.qml](../apps/qt/tests/tst_updates.qml), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [windows-desktop/design](development/features/windows-desktop/design.md#win-update-03-native-paths-and-user-reviewed-error-reports). |
| <a id="ds-req-ds-win-update-verify-01"></a>**REQ-DS-WIN-UPDATE-VERIFY-01**: Run the Windows PowerShell path contract and verify installed update/report behavior with interactive UAC. | [WIN-UPDATE-03 native verification](development/checklist.md#task-req-ds-win-update-verify-01). Pending / partial. | [architecture](architecture.md#ui-msi-01-windows-desktop-delivery); [windows.rs](../apps/qt/src/platform/updates/windows.rs), [windows_handoff.ps1](../apps/qt/src/platform/updates/windows_handoff.ps1) | Tests: [tst_updates.qml](../apps/qt/tests/tst_updates.qml), [test_windows_desktop.py](../tools/packaging/test_windows_desktop.py). Record: [windows-desktop/design](development/features/windows-desktop/design.md). |
| <a id="ds-fec-mon-ber"></a>**FEC-MON-BER**: Display FEC Monitor BER in scientific notation while preserving numeric results; see design. | [FEC-MON-BER](development/checklist.md#task-fec-mon-ber). Historical checked. | [architecture](architecture.md#fec-monitor-presentation-ui-fec-02); [receiver](../apps/qt/src/instruments/receiver), [fec](../apps/qt/qml/instruments/fec) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [fec-monitor/design](development/features/fec-monitor/design.md#fec-mon-ber-scientific-notation). |
| <a id="ds-rx-demux"></a>**RX-DEMUX**: Label the receiver separation stage DEMUX, retaining PLC / NCP / Data and the QAM action; see design. | [RX-DEMUX](development/checklist.md#task-rx-demux). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-rx-llr-02"></a>**RX-LLR-02**: Provide symbol decision regions, confidence histograms and decoder-index views with explicit retained-data scope. See design. | [RX-LLR-02](development/checklist.md#task-rx-llr-02). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md#rx-llr-02-readable-symbol-and-confidence-diagnostics), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-rx-resp-plc"></a>**RX-RESP-PLC**: Shade first–last PLC in receiver frequency responses; show VF/distance only for echo (design). | [RX-RESP-PLC](development/checklist.md#task-rx-resp-plc). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [receiver](../apps/qt/src/instruments/receiver), [response](../apps/qt/qml/instruments/response) | Tests: [tst_response_view.qml](../apps/qt/tests/tst_response_view.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md#rx-resp-plc--plc-region-and-contextual-velocity-factor). |


### UI-01: Workbench and section dialogs

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-copy-01"></a>**UI-COPY-01**: Use user-focused text and concise essential notes; remove programming-language/framework commentary from normal UI. Design. | [UI-COPY-01](development/ui-checklist.md#task-ui-copy-01). Historical checked. | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [ui-copy/design](development/features/ui-copy/design.md#ui-copy-01-user-focused-wording-and-essential-notes). |
| <a id="ds-ui-rxmer-04"></a>**UI-RXMER-04**: Remove RxMER demo/notch buttons and use Open file(s). Design. | [UI-RXMER-04](development/ui-checklist.md#task-ui-rxmer-04). Historical checked. | [architecture](architecture.md#shared-channel-estimaterxmer-viewer); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-workbench/validation](development/features/rxmer-workbench/validation.md). |
| <a id="ds-ui-shell-11"></a>**UI-SHELL-11**: Remove the left icon/WB gutter and use equal side margins. Design. | [UI-SHELL-11](development/ui-checklist.md#task-ui-shell-11). Historical checked. | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-verify-02"></a>**UI-VERIFY-02**: Restyle PRBS-RX with comparison cards, payload recovery and retained word-error charts. Design. | [UI-VERIFY-02](development/ui-checklist.md#task-ui-verify-02). Historical checked. | [architecture](architecture.md#qt-closed-loop-execution-and-workbench-files); [VerificationDialog.qml](../apps/qt/qml/workbenches/verification/VerificationDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [verification-panel/design](development/features/verification-panel/design.md#ui-verify-02-recovered-data-summary-and-graphs). |
| <a id="ds-ui-shell-10"></a>**UI-SHELL-10**: Center diagram connections, refine the two-line branded header and show PNM type footnotes inside input blocks. Design. | [UI-SHELL-10](development/ui-checklist.md#task-ui-shell-10). Historical checked. | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md#ui-shell-10-centered-connections-and-in-card-pnm-types), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-shell-09"></a>**UI-SHELL-09**: Group OCD/DPD under Transmitter Configuration and match the revised diagram/header styling. Design. | [UI-SHELL-09](development/ui-checklist.md#task-ui-shell-09). Historical checked. | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md#ui-shell-09-group-transmitter-configuration), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-zoom-01"></a>**UI-ZOOM-01**: Replace all zoom buttons with shared mouse wheel/range navigation and retain Full Span. Design. | [UI-ZOOM-01](development/ui-checklist.md#task-ui-zoom-01). Historical checked. | [architecture](architecture.md#chart-zoom-interaction-ui-zoom-01); [components](../apps/qt/qml/components) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [mouse-zoom/design](development/features/mouse-zoom/design.md#ui-zoom-01-mouse-driven-chart-navigation). |
| <a id="ds-ui-shell-07"></a>**UI-SHELL-07**: Place RxMER above Channel Simulator and split DS Channel Estimate into channel/receiver inputs; remove the top comparison connector. Design. | [UI-SHELL-07](development/ui-checklist.md#task-ui-shell-07). Historical checked. | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md#ui-shell-07-measurement-input-placement), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-shell-02"></a>**UI-SHELL-02**: Add left-aligned File &#124; Settings &#124; About actions; move Light/Dark mode into Settings → Global; remove the instruction banner. Settings/About have separate QML files; explicit WB saves include the theme. | [UI-SHELL-02](development/ui-checklist.md#task-ui-shell-02). Historical checked; current refinement: [UI-WB-CONTROLS-01](requirements.md#ds-ui-wb-controls-01), [UI-ABOUT-01](requirements.md#ds-ui-about-01). | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md#ui-shell-02-application-actions-and-global-appearance), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-wb-json-01"></a>**UI-WB-JSON-01**: Implement File Open/Save for JSON containing all PNM inputs and accepted settings of every block; define bounds, schema and input encoding. | [UI-WB-JSON-01](development/ui-checklist.md#task-ui-wb-json-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |
| <a id="ds-ui-shell-03"></a>**UI-SHELL-03**: Use a standard application menu bar above WB branding, replacing the content-area navigation buttons. | [UI-SHELL-03](development/ui-checklist.md#task-ui-shell-03). Historical checked; current refinement: [UI-WB-CONTROLS-01](requirements.md#ds-ui-wb-controls-01), [UI-ABOUT-01](requirements.md#ds-ui-about-01). | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md#ui-shell-03-standard-top-application-menu), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-shell-06"></a>**UI-SHELL-06**: Add a shared circular X to all popup titles, retaining footer buttons, draft cancellation and nested-popup behavior. Design. | [UI-SHELL-06](development/ui-checklist.md#task-ui-shell-06). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [components](../apps/qt/qml/components), [theme](../apps/qt/qml/theme) | Tests: [tst_buttons.qml](../apps/qt/tests/tst_buttons.qml). Record: [popup-close-controls/design](development/features/popup-close-controls/design.md#ui-shell-06-popup-close-controls). |
| <a id="ds-ui-shell-05"></a>**UI-SHELL-05**: Add About icon/version/project and offline license links; standardize title capitalization and matched rounded light/dark icons. | [UI-SHELL-05](development/ui-checklist.md#task-ui-shell-05). Historical checked. | [architecture](architecture.md#focused-about-and-dropdown-labels-ui-about-01); [AboutDialog.qml](../apps/qt/qml/about/AboutDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [about-menu/design](development/features/about-menu/design.md). |
| <a id="ds-ui-rx-config-01"></a><a id="ds-ui-verify-01"></a>**UI-RX-CONFIG-01**: Implement Receiver / PRBS-RX configuration and actual recovered-data status. Aliases: UI-VERIFY-01. | [UI-RX-CONFIG-01 / UI-VERIFY-01](development/ui-checklist.md#task-ui-rx-config-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |
| <a id="ds-ui-run-01"></a>**UI-RUN-01**: Add persistent lower-right Start, Stop and Pause/Resume icons with tooltips, accessible names and state-dependent availability. | [UI-RUN-01](development/ui-checklist.md#task-ui-run-01). Historical checked. | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). |


### UI-03: Observations and analyzers

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-tx-01"></a>**UI-TX-01**: Derive green Transmitter readiness from accepted OCD/DPD/source through the supported Rust request; invalidate/cancel on applied input changes. | [UI-TX-01](development/ui-checklist.md#task-ui-tx-01). Historical checked. | [architecture](architecture.md#shared-spectrum-analyzer-sa-and-transmitter-readiness); [transmitter_preview.rs](../crates/ofdm/src/simulation/transmitter_preview.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [transmitter_preview.rs](../crates/ofdm/tests/transmitter_preview.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-spectrum/validation](development/features/transmitter-spectrum/validation.md). |
| <a id="ds-ui-tx-02"></a>**UI-TX-02**: Reuse a generic SA dialog for actual TX-output samples, with centered baseband default, optional capture mapping, normalized Hann PSD, bounded min/max viewports, zoom, progress and cooperative cancellation. Design: Transmitter spectrum. | [UI-TX-02](development/ui-checklist.md#task-ui-tx-02). Historical checked. | [architecture](architecture.md#shared-spectrum-analyzer-sa-and-transmitter-readiness); [transmitter_preview.rs](../crates/ofdm/src/simulation/transmitter_preview.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [transmitter_preview.rs](../crates/ofdm/tests/transmitter_preview.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-spectrum/validation](development/features/transmitter-spectrum/validation.md). |
| <a id="ds-ui-sa-03"></a>**UI-SA-03**: Clear the SA lower text panel and reserve blank space. | [UI-SA-03](development/ui-checklist.md#task-ui-sa-03). Historical checked; current refinement: [UI-SA-04](requirements.md#ds-ui-sa-04), [UI-SA-07](requirements.md#ds-ui-sa-07), [UI-SA-DETAILS-01](requirements.md#ds-ui-sa-details-01). | [architecture](architecture.md#shared-spectrum-analyzer-sa-and-transmitter-readiness); [transmitter_preview.rs](../crates/ofdm/src/simulation/transmitter_preview.rs), [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [transmitter_preview.rs](../crates/ofdm/tests/transmitter_preview.rs), [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [transmitter-spectrum/validation](development/features/transmitter-spectrum/validation.md). |
| <a id="ds-ui-sa-04"></a>**UI-SA-04**: Fit symbol/window timing and nominal cable echo budgets below the PSD, using frozen result settings and Rust calculations. | [UI-SA-04](development/ui-checklist.md#task-ui-sa-04). Historical checked. | [architecture](architecture.md#sa-symbol-timing-and-cable-panel-ui-sa-04); [spectrum](../apps/qt/qml/instruments/spectrum) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [symbol-timing-popup/design](development/features/symbol-timing-popup/design.md). |


### Receiver workbench navigation and instruments

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-rx-wb-01"></a>**UI-RX-WB-01**: Replace the Receiver preview with its own workbench: processing stages, clickable test-point markers and instrument navigation; soft demapping and optional hard slicing remain separate branches. | [UI-RX-WB-01](development/ui-checklist.md#task-ui-rx-wb-01). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-ui-rx-wb-02"></a>**UI-RX-WB-02**: Align equal-width stage/instrument chips and annotation rows; use rounded edges, subtle chip pins and visible keyboard/hover feedback. | [UI-RX-WB-02](development/ui-checklist.md#task-ui-rx-wb-02). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md#ui-rx-wb-02-presentation-follow-up). |
| <a id="ds-ui-qam-01"></a>**UI-QAM-01**: Add Data Profile / NCP / PLC and Raw / Equalized / Paired controls, separate PLC data/preamble and unavailable received-profile state. | [UI-QAM-01 selectors](development/ui-checklist.md#task-ui-qam-01); [UI-QAM-01 data](development/ui-checklist.md#task-ui-qam-01-detail-2); [UI-QAM-01](development/ui-checklist.md#task-ui-qam-01-detail-3). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-ui-qam-02"></a>**UI-QAM-02**: Add bounded matched received NCP I/Q classification/collection before enabling its constellation view; LLR/control events are not I/Q. | [UI-QAM-02](development/ui-checklist.md#task-ui-qam-02). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-ui-sa-05"></a>**UI-SA-05**: Add independent SA Test Point routing with empty-state admission and a distinct FFT-bin-power label; disable receiver generation. | [UI-SA-05 selector](development/ui-checklist.md#task-ui-sa-05); [UI-SA-05 data](development/ui-checklist.md#task-ui-sa-05-detail-2). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-ui-ch-est-rx-01"></a>**UI-CH-EST-RX-01**: Display supplied H and actual H-hat snapshots with response curves and band-limited impulse/echo diagnostics. | [UI-CH-EST-RX-01](development/ui-checklist.md#task-ui-ch-est-rx-01). Historical checked. | [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [receiver](../apps/qt/src/instruments/receiver), [response](../apps/qt/qml/instruments/response) | Tests: [tst_response_view.qml](../apps/qt/tests/tst_response_view.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md#ui-ch-est-rx-01-response-and-echo). |
| <a id="ds-ui-slicer-01"></a>**UI-SLICER-01**: Show selected physical decisions/residuals and actual NCP/payload cell LLRs plus the retained LDPC-input window. | [UI-SLICER-01](development/ui-checklist.md#task-ui-slicer-01). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md#ui-slicer-01--ui-fec-01), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |
| <a id="ds-ui-fec-01"></a>**UI-FEC-01**: Add dedicated FEC statistics, iteration distribution and bounded word outcomes, with a prominent Receiver header shortcut. | [UI-FEC-01](development/ui-checklist.md#task-ui-fec-01). Historical checked. | [architecture](architecture.md#receiver-workbench); [receiver](../apps/qt/src/instruments/receiver), [ReceiverWorkbenchDialog.qml](../apps/qt/qml/workbenches/receiver/ReceiverWorkbenchDialog.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml), [tst_soft_diagnostics.qml](../apps/qt/tests/tst_soft_diagnostics.qml). Record: [receiver-instruments/design](development/features/receiver-instruments/design.md#ui-slicer-01--ui-fec-01), [receiver-workbench/validation](development/features/receiver-workbench/validation.md). |


### Proposed RxMER editor and shell follow-up

| Stable requirement / retained obligation | Checklist task / lifecycle | Architecture and current implementation owner | Acceptance test or validation record |
| --- | --- | --- | --- |
| <a id="ds-ui-shell-04"></a>**UI-SHELL-04**: Use the requested main WB heading and conventional File labels. | [UI-SHELL-04](development/ui-checklist.md#task-ui-shell-04). Historical checked. | [architecture](architecture.md#ofdm-editor-registry-and-application-navigation); [Main.qml](../apps/qt/qml/Main.qml), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [development/qt](development/qt.md), [visual-workspace/design](development/features/visual-workspace/design.md), [visual-workspace/validation](development/features/visual-workspace/validation.md). |
| <a id="ds-ui-rxmer-01"></a>**UI-RXMER-01**: Stage single/multiple RxMER inputs with exact DPD association. | [UI-RXMER-01](development/ui-checklist.md#task-ui-rxmer-01). Historical checked. | [architecture](architecture.md#shared-channel-estimaterxmer-viewer); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-workbench/validation](development/features/rxmer-workbench/validation.md). |
| <a id="ds-ui-rxmer-02"></a>**UI-RXMER-02**: Add static/average/hold/interpolated-power configuration and bounded previews with strict required-carrier validity. | [UI-RXMER-02](development/ui-checklist.md#task-ui-rxmer-02). Historical checked. | [architecture](architecture.md#shared-channel-estimaterxmer-viewer); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-workbench/validation](development/features/rxmer-workbench/validation.md). |
| <a id="ds-ui-rxmer-03"></a>**UI-RXMER-03**: Expose symbol intervals/order/end policy and previews. | [UI-RXMER-03 editor](development/ui-checklist.md#task-ui-rxmer-03); [UI-RXMER-03 dispatch](development/ui-checklist.md#task-ui-rxmer-03-detail-2). Historical checked. | [architecture](architecture.md#shared-channel-estimaterxmer-viewer); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-workbench/validation](development/features/rxmer-workbench/validation.md). |
| <a id="ds-ui-channel-01"></a>**UI-CHANNEL-01**: Require valid RxMER and response configurations for green Channel Simulator; provide explicit Flat/unity H=1+0i default on the DPD grid. | [UI-CHANNEL-01](development/ui-checklist.md#task-ui-channel-01). Historical checked. | [architecture](architecture.md#shared-channel-estimaterxmer-viewer); [src](../apps/qt/src), [qml](../apps/qt/qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [rxmer-workbench/validation](development/features/rxmer-workbench/validation.md). |

### Feature-scoped receiver contracts

These original `CH-EST-RX-01..06` identifiers are retained from the accepted
receiver design. Delivery remains the historical MODEL-05 task; the original
design and implementation record preserve the full numerical/stream contract.

<a id="ds-ch-est-rx-01"></a>
- **CH-EST-RX-01**: Receiver-owned pilot references and per-pilot response estimates; known timing/phase, noiseless independent amplitude/phase anchors first.
  Owner: [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [MODEL-05](development/checklist.md#task-model-05). Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md).

<a id="ds-ch-est-rx-02"></a>
- **CH-EST-RX-02**: Tested data/PLC-band interpolation and estimated equalizer mode; quantify coverage/startup and response error without supplied H as RX input.
  Owner: [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [MODEL-05](development/checklist.md#task-model-05). Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md).

<a id="ds-ch-est-rx-03"></a>
- **CH-EST-RX-03**: Bounded separate continuous/scattered-pilot raw/corrected analyzers; include boost, physical locations, source phase and in-fit/held-out meaning.
  Owner: [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [MODEL-05](development/checklist.md#task-model-05). Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md).

<a id="ds-ch-est-rx-04"></a>
- **CH-EST-RX-04**: Bounded separate PLC16-QAM and BPSK-preamble paired analyzers, including acquisition and failed frames without payload-readiness gating.
  Owner: [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [MODEL-05](development/checklist.md#task-model-05). Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md).

<a id="ds-ch-est-rx-05"></a>
- **CH-EST-RX-05**: Noiseless and noisy comparisons with reference-H reception: estimation error, payload BER/LDPC iterations, PLC/control outcomes and collector invariance. Cover both spacings, pilot phase changes and explicit coverage gaps.
  Owner: [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [MODEL-05](development/checklist.md#task-model-05). Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md).

<a id="ds-ch-est-rx-06"></a>
- **CH-EST-RX-06**: Shared public Rust API/CLI export, user guides, actual pre/post plots per stream, reproduction commands and full validation gate.
  Owner: [architecture](architecture.md#receiver-channel-estimation-ch-est-rx-and-pilotplc-analyzers); [pilot.rs](../crates/ofdm/src/signal_processing/receiver/channel_estimation/pilot.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [MODEL-05](development/checklist.md#task-model-05). Tests: [channel_estimation_rx.rs](../crates/ofdm/tests/channel_estimation_rx.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs). Record: [pilot-channel-estimation/implementation](development/features/pilot-channel-estimation/implementation.md).

### Existing engineering contract identities

These already named requirements remain defined by their linked accepted design
paragraphs; they govern documentation/review/discovery, not accepted upstream PHY.

| Preserved ID | Canonical refinement / architecture and owner | Checklist delivery / acceptance |
| --- | --- | --- |
| <a id="ds-guide-r1"></a>**GUIDE-R1** | [original contract](development/features/engineering-guidance/design.md#guide-r1-preserve-local-contracts); [architecture](architecture.md#engineering-guidance-ownership); [README.md](../skills/README.md), [SKILL.md](../skills/development-workflow/SKILL.md) | [DEV-GUIDE-01](development/checklist.md#task-dev-guide-01). Record: [engineering-guidance/validation](development/features/engineering-guidance/validation.md). |
| <a id="ds-guide-r2"></a>**GUIDE-R2** | [original contract](development/features/engineering-guidance/design.md#guide-r2-traceable-planning-and-reuse); [architecture](architecture.md#engineering-guidance-ownership); [README.md](../skills/README.md), [SKILL.md](../skills/development-workflow/SKILL.md) | [DEV-GUIDE-01](development/checklist.md#task-dev-guide-01). Record: [engineering-guidance/validation](development/features/engineering-guidance/validation.md). |
| <a id="ds-guide-r3"></a>**GUIDE-R3** | [original contract](development/features/engineering-guidance/design.md#guide-r3-review-and-evidence); [architecture](architecture.md#engineering-guidance-ownership); [README.md](../skills/README.md), [SKILL.md](../skills/development-workflow/SKILL.md) | [DEV-GUIDE-01](development/checklist.md#task-dev-guide-01). Record: [engineering-guidance/validation](development/features/engineering-guidance/validation.md). |
| <a id="ds-guide-r4"></a>**GUIDE-R4** | [original contract](development/features/engineering-guidance/design.md#guide-r4-self-contained-discovery); [architecture](architecture.md#engineering-guidance-ownership); [README.md](../skills/README.md), [SKILL.md](../skills/development-workflow/SKILL.md) | [DEV-GUIDE-01](development/checklist.md#task-dev-guide-01). Record: [engineering-guidance/validation](development/features/engineering-guidance/validation.md). |
| <a id="ds-guide-r5"></a>**GUIDE-R5** | [original contract](development/features/engineering-guidance/design.md#guide-r5-candidate-provenance-and-truthful-validation); [architecture](architecture.md#engineering-guidance-ownership); [README.md](../skills/README.md), [SKILL.md](../skills/development-workflow/SKILL.md) | [DEV-GUIDE-01](development/checklist.md#task-dev-guide-01). Record: [engineering-guidance/validation](development/features/engineering-guidance/validation.md). |
| <a id="ds-skill-us-r1"></a>**SKILL-US-R1** | [original contract](development/features/upstream-skills/design.md#skill-us-r1-pinned-and-scoped-migration); [architecture](architecture.md#upstream-guidance-migration); [SKILL.md](../skills/docsis-signal-processing/SKILL.md), [SKILL.md](../skills/scientific-testing/SKILL.md), [SKILL.md](../skills/public-api/SKILL.md) | [SKILL-US-01](development/checklist.md#task-skill-us-01). Record: [upstream-skills/validation](development/features/upstream-skills/validation.md). |
| <a id="ds-skill-us-r2"></a>**SKILL-US-R2** | [original contract](development/features/upstream-skills/design.md#skill-us-r2-direction-correct-scientific-guidance); [architecture](architecture.md#upstream-guidance-migration); [SKILL.md](../skills/docsis-signal-processing/SKILL.md), [SKILL.md](../skills/scientific-testing/SKILL.md), [SKILL.md](../skills/public-api/SKILL.md) | [SKILL-US-01](development/checklist.md#task-skill-us-01). Record: [upstream-skills/validation](development/features/upstream-skills/validation.md). |
| <a id="ds-skill-us-r3"></a>**SKILL-US-R3** | [original contract](development/features/upstream-skills/design.md#skill-us-r3-paired-apis-and-compatibility); [architecture](architecture.md#upstream-guidance-migration); [SKILL.md](../skills/docsis-signal-processing/SKILL.md), [SKILL.md](../skills/scientific-testing/SKILL.md), [SKILL.md](../skills/public-api/SKILL.md) | [SKILL-US-01](development/checklist.md#task-skill-us-01). Record: [upstream-skills/validation](development/features/upstream-skills/validation.md). |
| <a id="ds-skill-us-r4"></a>**SKILL-US-R4** | [original contract](development/features/upstream-skills/design.md#skill-us-r4-local-resources-and-attribution); [architecture](architecture.md#upstream-guidance-migration); [SKILL.md](../skills/docsis-signal-processing/SKILL.md), [SKILL.md](../skills/scientific-testing/SKILL.md), [SKILL.md](../skills/public-api/SKILL.md) | [SKILL-US-01](development/checklist.md#task-skill-us-01). Record: [upstream-skills/validation](development/features/upstream-skills/validation.md). |

## High-level API acceptance contract

Source: user-supplied `docs/development/MERLIN-API-Codex-Handoff.zip`, prepared
2026-11-14. Its requirements govern API-RUST-01 and API-PY-01. The original archive
and extracted reference documents are preserved, not represented as implemented
code. This record maps the contract to the current implementation and evidence.

| Requirements | Implementation boundary | Evidence/status |
| --- | --- | --- |
| 001–007, 010–015 | `docsis_ofdm_fec::api`, optional native input adapter | Validated inputs, owned results and Qt-independent build checks |
| 020–027, 080 | `pnm-input` and immutable named bytes | All three binary/JSON fixture pairs passed fixture-backed imports |
| 030–038, 081, 090–092 | `Ocd`, `Dpd`, existing captured-layout validation | Derive uniquely determined profile bandwidth; ambiguous captures require override; defaults recorded |
| 040–048, 082 | Independent controller collections, opaque scoped IDs | Explicit/unity/nearest-time policies implemented |
| 100–113, 114–116 | `prepare`, existing `MeasurementPlayback` power operations | Static selections, exact grids, explicit epochs, null preservation |
| 050–059, 083–085 | Sequential existing `run_channel` jobs | Direct engine parity, runtime continuation and cancellation tests passed |
| 060–069 | Owned batch, window FEC/profile summaries, whole-run diagnostics | No analytical substitute counters; detailed scopes in reports |
| 070–074, 086–088 | Separate optional Python adapter | Native wheel, seven Python tests and all four policy parity checks passed |

Current interface refinements against the proposed signature inventory:

- Input unit constructors use `new` and explicit `value` getters. Whole-second
  CaptureTime and millisecond ObservationDuration also have descriptive aliases.
- `prepare()` is public for no-execution inspection and engine parity. It returns
  immutable snapshots, not a second configurable engine.
- The optional `CaptureLoader` handles named binary/JSON bytes; `read_capture`
  adds bounded file I/O. These are the recommended file conveniences. `FromFile` adds one-argument
  measurement loaders; `ProfileFromFile` adds the explicit two-argument profile
  loader, avoiding a dummy selection argument for measurements.
- Existing validated engine grid/layout models are reused at manual OCD
  construction; no replacement scientific model or descriptor serialization.
- JSON output is an owned string; file export is separate and create-new only.
- Full engine diagnostics retain their existing typed Rust representation.
  Serialized diagnostics omit private compatibility settings for static derived
  observations; public sources retain original provenance and true weights.

Executed evidence and verification boundaries: [validation record](development/features/high-level-api/validation.md).

No Octave, simultaneous channels, continuous playback, MAC services, publishing
or system installation is part of this milestone.

### Mandatory downstream installation-document maintenance

These additional repository requirements govern ongoing delivery:

<a id="req-py-doc-001"></a>
- **REQ-PY-DOC-001:** Every `tools/git/git-push.sh` branch or release invocation
  MUST refresh or validate `docs/user/python/integration.md`, including
  documentation-only pushes and explicit test-omission modes.
<a id="req-py-doc-002"></a>
- **REQ-PY-DOC-002:** Generation MUST live in a separate script,
  `tools/python/update_integration_doc.py`, invoked by Git helpers. The guide
  MUST derive version/tag/installation pins from validated VERSION and Python
  metadata plus a tracked template; generation logic MUST NOT live in shell.
<a id="req-py-doc-003"></a>
- **REQ-PY-DOC-003:** Version-bump releases MUST refresh the guide before
  candidate validation and include it in the same validated release commit.
  Ordinary saves MUST include any refresh. A branch push that creates an
  uncommitted guide change MUST stop for review/commit rather than silently
  creating a commit or bypassing the clean-tree guard.
<a id="req-py-doc-004"></a>
- **REQ-PY-DOC-004:** Generation MUST be deterministic and leave unchanged
  content untouched. A dry run and existing-tag retry MUST check without
  modifying the guide or immutable tagged commit. Generator errors and stale
  checks MUST block pushing.
<a id="req-py-doc-005"></a>
- **REQ-PY-DOC-005:** Interrupted release recovery MUST recognize only the exact
  version metadata and generated guide reconstructed from committed inputs.
  Modified guides, staged changes and unrelated edits MUST remain rejected.
<a id="req-py-doc-006"></a>
- **REQ-PY-DOC-006:** Generation MUST NOT invent wheel availability, checksums,
  published tags or platform verification. Release artifact hashes and supported
  targets require their own evidence; refreshing docs does not publish wheels.
<a id="req-repo-archive-001"></a>
- **REQ-REPO-ARCHIVE-001:** Generated/downloaded archives, including ZIPs,
  compressed tar files and Python wheels, MUST be ignored by Git. A required
  source fixture needs a narrowly scoped, explicitly reviewed exception.

### Shared Rust/Python API evolution

<a id="req-api-parity-001"></a>
- **REQ-API-PARITY-001:** Shared capabilities requested through Python MUST be
  implemented and validated in the public Rust API first, then exposed by the
  optional adapter with matching domain semantics and structured errors. Python
  must not become the only owner of discovery, validation or scientific logic.
<a id="req-api-parity-002"></a>
- **REQ-API-PARITY-002:** Profile discovery MUST read the actual capture catalog
  through Rust decoding, return validated captured identities and reject invalid
  catalogs. Examples that run all profiles MUST use discovery rather than a
  hardcoded list or probing possible identifiers. Discovery MUST NOT simulate.
<a id="req-api-parity-003"></a>
- **REQ-API-PARITY-003:** New discovery operations MUST cover binary/decoded JSON
  agreement, ordering, empty/duplicate/invalid IDs and independent owned results
  across Rust and Python. File convenience wrappers remain adapter responsibilities.

<a id="req-api-parity-004"></a>
- **REQ-API-PARITY-004:** An already-loaded DPD MUST expose its retained profile
  catalog as a Rust member accessor and Python property, without file access,
  redecoding or simulation. Keep static discovery available. Distinguish the
  complete source catalog from the selected profile ID; preserve immutable
  ownership, created-profile singleton catalogs and edited-source catalogs.

### API-RECEIVER-01: receiver correction parity

- Expose the existing receiver correction policy through validated Rust options
  before mirroring it in Python; retain the supplied-channel default.
- Keep applied channel H and receiver correction independent. Loading/preparing
  or selecting a mode must not run simulation. Reports preserve chosen settings.
- Match the saved-UI collection companion's correction, seeds, iterations,
  duration, average policy and OCD timing to its curated WB, with a fixture
  drift check. Keep the separate MCE walkthrough explicitly labeled.
- Compare actual observation-window FEC and independent bit comparison per profile;
  matching BER alone is insufficient. Keep all static selection modes, owned
  results and cancellation intact. Do not force agreement by changing counters.

### API-RECEIVER-02: explicit MCE naming

Use `ReceiverMode::MerlinChannelEstimated` in Rust and
`ReceiverMode.MERLIN_CHANNEL_ESTIMATED` in Python. Preserve existing WB/CLI/report
JSON `merlin_estimated` and accept `merlin_channel_estimated` on input. Renaming
must not change correction algorithms, decoder reliability or receiver defaults.

### API-WB-01: saved workbench input

- Provide native Rust and Python WB file/byte loaders, independently of Qt/CLI.
  Read schema 1/2 accepted embedded captures and settings; verify lengths/hashes,
  exact grids, explicit selections and bounded work before returning.
- Preserve legacy/current profile selection precedence, static checked RxMER
  single/average behavior, saved OCD, PRBS15 seed, receiver mode/MCE settings,
  waveform channel filter, H/unity, association tolerance, loss/power and limits.
- Keep original WB bytes and capture provenance; loading/preparing does not run.
  Only run starts the existing engine. Return independent batches per profile,
  retain controller scopes and preserve cancellation/partial/not-started results.
- Reject unsupported sources/timeline modes, unknown fields and incomplete WBs
  explicitly. Do not approximate saved behavior. General timeline playback and
  file-source import are not implemented by this static PRBS15 loader.
- Add a Python WB-path example without duplicate scientific constants, and a
  compiling Rust example. Compare actual reports with the Rust engine and native
  UI, test integrity/selection/limits/cancellation, and document platform evidence.

### API-WB-02: bundled receiver defaults

All runnable WB examples must explicitly save CmChannelEstimated, matching the
new-configuration UI/API default. Preserve captures and all other settings.
The saved-UI Python companion and WB loader must reproduce that saved choice;
the separate MCE walkthrough remains MCE. Guard this policy in the packaged
bundle inventory test and maintain current FEC references and user guides.

### Generated API reference maintenance

Maintain searchable generated references for the public Rust high-level API,
PNM input/workbench adapter and native Python API. Source-owned documentation
must describe public parameters, units, defaults, ownership, errors, cancellation
and partial results; Python pages must not expose hidden interpreter arguments.
The development builder uses strict rustdoc and pinned Sphinx against the actual
installed native extension. Discover all exported classes, require public-member
coverage and reject installed-version mismatch. Rebuild/reinstall the native
wheel when its documentation changes even without a version bump. Keep generated
HTML ignored and local; publication remains a separate authorized action.
See the [build guide](reference/api-reference.md).

### Native wheel release gate

Release tags must build and install CPython 3.10 x64 wheels on native Ubuntu
22.04 and Windows 2022/MSVC runners and execute the actual single-capture Python
API example in an isolated fresh environment. Require exact package/ABI/platform
identity, complete notices and real fixture FEC/BER outcomes. Both checksummed
wheel artifacts are mandatory before publication; retain all existing application
and security gates. Keep release filenames, compatibility and installation/hash
verification in a separate [Python release guide](user/python/releases.md),
linked from Python installation and generated downstream integration documents.
Configuration is not evidence of native Windows execution or published assets.

API documentation and example changes cover both Rust and Python unless
explicitly scoped otherwise. Default profile-capture loading uses DOCSIS 3.1;
explicit protocol selection remains supported and file versions do not infer it.

Simulation is the preferred create/load/configure/run entry point in both APIs.
Keep legacy constructors compatible. Loading is idle and atomic; preserve scoped
measurement identity and owned results. Shared build/release example checks must
report elapsed wall time, including failures, with Python example-only time
separated from total wheel validation and Rust build-inclusive time labeled.

#### API-CPA-01 — Channel conditioning controls

Both APIs expose setters/getters for launch-amplifier gain and Common Path
Attenuator (CPA) loss. Rust validates finite 0..60 dB values and rejects invalid
updates without changing accepted settings. Setters never run simulation.
Explicit gain clears a target level; the gain getter returns effective gain.
Preserve legacy saved attenuation keys and define CPA in the shared terminology.


## Downstream refinement traceability

The original numbered API ranges and named refinements above retain their exact
contract text. Their acceptance owner remains the existing high-level API feature
and tests; the following groups provide direct architecture/test/delivery routing.
Current historical passes are not new runs in this documentation task.

| Preserved IDs | Architecture / current owner | Acceptance / integration revalidation |
| --- | --- | --- |
| Original 001–116 ranges as enumerated in the API contract | [Basic API](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); `crates/ofdm/src/api/`, `crates/pnm-input`, `bindings/python`. | `crates/pnm-input/tests/high_level_api.rs`, `bindings/python/tests/test_api.py` and `test_parity.py`; INT-RUST-01, INT-PY-01, INT-REG-01. Unlisted numeric IDs are not invented. |
| REQ-PY-DOC-001 through REQ-PY-DOC-006; REQ-REPO-ARCHIVE-001 | [Delivery contract](architecture.md#unified-normative-evidence-and-exclusions); `tools/python/update_integration_doc.py`, existing save/push/version helpers and `.gitignore`. | Existing helper/guide-generation and archive inventories; INT-TOOLS-01. |
| REQ-API-PARITY-001 through REQ-API-PARITY-004 | [API ownership](architecture.md#unified-rust-python-and-cli-apis); shared Rust inventory and optional Python conversions. | `crates/pnm-input/tests/high_level_api.rs`, native parity and all-profile examples; INT-RUST-01, INT-PY-01. |
| API-RECEIVER-01/02, API-WB-01/02, API-CPA-01 and API-OCD/PROFILES/PROTOCOL/SIMULATION refinements | [Existing API architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); current Rust API/input and native binding owners. | Existing receiver/workbench/designer/API native tests and corresponding Rust fixtures; INT-REG-01. |
| Generated API reference and native wheel release contract | [Native delivery](architecture.md#api-wheel-01-native-python-release-validation); current API doc/wheel/release tools. | Actual native import/export docs coverage and release-wheel examples on declared targets when applicable; INT-TOOLS-01, INT-DOC-02. |


## Individual API handoff requirements

The following 93 clauses preserve the **exact original requirement text and IDs**
from the user-supplied API handoff dated 2026-11-14. Missing numeric IDs are not
invented. The ignored original remains intact; its SHA-256 is
`6a3f34b38c1bf41af116b745396bf0ac6a7ef8724559dc70223028bede7c38f9`.
The handoff described a proposed milestone. Accepted current interfaces and
spelling refinements above, plus the later contract register, govern present
behavior without rewriting these original clauses. REQ-005/090/091 describe the
initial selected-profile workflow; API-PROFILES-01 and API-SIMULATION-01 add idle
sessions and sequential retained-profile runs. REQ-074 excludes Octave from the
milestone; it does not claim an implemented Octave adapter. REQ-087 has local
isolation evidence; it does not establish all native platforms or provisioning.
Each clause below links its maintained owner, acceptance and historical delivery.
Integration revalidation remains INT-RUST-01, INT-PY-01 and INT-REG-01.

<a id="req-001"></a>
- REQ-001: The API MUST reuse MERLIN's existing Rust simulation, DSP, modulation and BCH/LDPC implementations.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-002"></a>
- REQ-002: The API MUST NOT introduce a parallel simulation engine or duplicate scientific calculations in Python or application adapters.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-003"></a>
- REQ-003: The Rust library and standalone CLI MUST build and operate without Qt or Python. The Python extension MUST build and operate without Qt and MUST NOT require a CLI subprocess.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-004"></a>
- REQ-004: Qt MUST remain an optional presentation adapter. The core and facade MUST NOT depend on Qt types, libraries or build tools.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-005"></a>
- REQ-005: The initial workflow MUST support one selected modulation profile, one or more RxMER captures, optional independently collected channel estimates, bounded simulation and FEC/profile performance reports.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01), [API-PROFILES-01](development/checklist.md#task-api-profiles-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-006"></a>
- REQ-006: The initial workflow MUST NOT imply full CMTS MAC behavior, service-group simulation, multiple simultaneous OFDM channels or external modem interoperability.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-007"></a>
- REQ-007: Existing low-level Rust APIs and CLI functionality MUST remain available. The facade MUST NOT require a broad engine refactor as a prerequisite.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-010"></a>
- REQ-010: The public model MUST provide a controller (`Merlin`), profile (`Dpd`), channel configuration (`Ocd`), RxMER, channel estimate, evaluation options and owned reports.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-011"></a>
- REQ-011: Public scientific parameters, identifiers, units and results MUST use named domain types. Rust primitives MUST NOT appear as unnamed scientific fields or arguments. Primitive conversion MAY occur through explicit checked constructors and language input/output boundaries.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-012"></a>
- REQ-012: Reusable existing domain types MUST be used where their semantics are suitable. New validated wrappers MUST be introduced where stronger constraints are required; the facade MUST NOT create duplicate computational models.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-013"></a>
- REQ-013: The API MUST use checked constructors and structured recoverable errors. Invalid caller input MUST NOT trigger process termination or caller-input panics.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-014"></a>
- REQ-014: Rust consumers MUST be able to provide named bytes or typed inputs without filesystem access. Python MUST offer file-loading conveniences and MUST convert inputs through Rust validation.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-015"></a>
- REQ-015: Public operations MUST preserve input ownership rules and MUST NOT mutate original captures. Explicit profile edits MUST preserve the original captured profile and identify the edited result as a hypothesis.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-020"></a>
- REQ-020: File-loading conveniences MUST accept supported binary PNM captures and decoded JSON for profiles, RxMER and channel estimates.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-021"></a>
- REQ-021: Loading MUST detect the format from content and validate the expected capture kind. Filename extensions alone MUST NOT determine decoding.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-022"></a>
- REQ-022: Binary loading MUST reuse the available Rust decoder/converter. The architecture MUST identify and verify support for each capture type before claiming that type is implemented.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-023"></a>
- REQ-023: Unsupported formats, decoder contracts and capture types MUST produce explicit errors. The API MUST NOT silently treat unsupported binary input as decoded JSON or invoke an unannounced external converter.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-024"></a>
- REQ-024: Decoding and conversion MUST finish during loading, before simulation starts. Loading MUST NOT execute the transmit/channel/receive experiment or produce FEC simulation statistics.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-025"></a>
- REQ-025: Provenance MUST distinguish original binary/JSON bytes from decoded intermediates, including hashes, source identity, original timestamps and any declared decoder version.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-026"></a>
- REQ-026: Import MUST preserve exact frequency grids, missing readings, carrier roles, assignments and profile IDs. It MUST NOT resample, rewrite timestamps or replace missing measurements with zero implicitly.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-027"></a>
- REQ-027: File size, decoded expansion and retained capture storage MUST be bounded and validated.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-090"></a>
- REQ-090: The modulation-profile input MUST have a dedicated import/create/edit interface. A capture containing multiple profile IDs MUST require explicit selection of the profile to evaluate.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01), [API-PROFILES-01](development/checklist.md#task-api-profiles-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-091"></a>
- REQ-091: Each run MUST use one explicitly assigned DPD. Replacing the selected profile MUST NOT alter previously returned reports or original capture assignments.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01), [API-PROFILES-01](development/checklist.md#task-api-profiles-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-092"></a>
- REQ-092: The selected profile MUST be validated against the resolved OCD and the effective RxMER/channel conditions before simulation. Unsupported coding paths MUST NOT be accepted merely because a constellation name is recognized.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-030"></a>
- REQ-030: Users MUST be able to import a CM modulation profile as a DPD configuration or create a profile against a configured OCD.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-031"></a>
- REQ-031: The API MUST populate OCD settings from the imported profile wherever they are established unambiguously. It MUST NOT require users to repeat those settings.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01), [API-OCD-01](development/checklist.md#task-api-ocd-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-032"></a>
- REQ-032: Optional overrides MUST remain available. Each setting MUST resolve in this order: explicit override, unambiguous capture-derived value, documented valid application default.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-033"></a>
- REQ-033: Missing timing settings MUST default to CP 512 samples, roll-off 128 samples and interleaver depth 16. These defaults MUST be validated against numerology and layout and MUST be reported as assumptions rather than measured modem settings.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-034"></a>
- REQ-034: Bandwidth MUST be derived only when the capture establishes occupied bandwidth unambiguously. The API MUST NOT silently assume 192 MHz or equate data-carrier span with occupied bandwidth. An unresolved required value MUST produce a missing-setting error.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01), [API-OCD-01](development/checklist.md#task-api-ocd-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-035"></a>
- REQ-035: Resolved settings MUST retain whether they were explicit, derived or defaulted. Incompatible overrides MUST be rejected.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-036"></a>
- REQ-036: Profile creation and editing MUST preserve protected PLC, pilot and excluded roles. Imported unknown roles and zero loading MUST remain distinguishable.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-037"></a>
- REQ-037: Captured profile ID and any internal bootstrap/wire-profile mapping MUST remain distinct in provenance and results.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-038"></a>
- REQ-038: OCD/DPD configuration objects MUST NOT imply complete protocol-message serialization. Actual descriptor serialization is a separate capability.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-040"></a>
- REQ-040: RxMER and channel estimates MUST be added individually through separate operations. A required `MeasurementPair` MUST NOT be the input collection model.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-041"></a>
- REQ-041: Capture counts MUST be independent. A user MUST be able to load multiple RxMER captures and zero, one or multiple channel estimates collected at different times.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-042"></a>
- REQ-042: Addition MUST return a stable typed identifier scoped to the controller. Selection of an invalid or foreign identifier MUST fail validation.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-043"></a>
- REQ-043: Adding an input MUST validate its structure and storage limits without performing timestamp pairing. Failed additions MUST leave previously accepted configuration unchanged.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-044"></a>
- REQ-044: Evaluation MUST support explicit unity response, one explicitly selected channel estimate reused across experiments, and nearest-time selection within an explicit tolerance.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-045"></a>
- REQ-045: Explicit selection MUST permit different collection times while recording the reuse assumption and original epochs. Device/channel association, grid and coverage checks MUST still apply.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-046"></a>
- REQ-046: Nearest-time selection MUST reject missing and equally near ambiguous matches. It MUST NOT silently fall back to unity or another selection policy.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-047"></a>
- REQ-047: When no channel estimate is selected, a unity-response policy MUST be explicitly chosen. Omitted selection MUST NOT silently imply a measured response or unity assumption.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-048"></a>
- REQ-048: Association MUST occur when preparing the run using the complete accepted configuration. The report MUST identify the inputs and selection policy used in each experiment.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-100"></a>
- REQ-100: RxMER MUST have a dedicated collection and selection interface. Users MUST be able to choose an individual loaded capture by `RxMerId`, average all loaded compatible RxMER captures, or interpolate between loaded captures at an explicitly selected capture time.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-101"></a>
- REQ-101: The RxMER selection policy MUST also retain `Each` for independent experiments over all loaded captures. `Each` MUST be the default to preserve the simple batch workflow.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-102"></a>
- REQ-102: `Individual(id)` MUST use only the selected original capture. `AverageAll` MUST produce one derived per-carrier RxMER frame and one experiment. `InterpolateAt(time)` MUST produce one time-selected frame and one experiment. Neither derived mode MUST modify the source captures.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-103"></a>
- REQ-103: Averaging MUST combine normalized residual powers per subcarrier, not arithmetic dB values. For valid readings m_i in dB, the effective value MUST be -10 log10(mean(10^(-m_i/10))). Every loaded capture MUST have equal weight; the report MUST identify this as a capture average, not a duration-weighted average.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-104"></a>
- REQ-104: Averaging MUST require compatible identity and exact grids without implicit resampling. A derived carrier MUST remain missing if any contributing capture has no reading for that carrier. Coverage and contributing source IDs MUST be reported, and existing simulation coverage validation MUST still apply.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-105"></a>
- REQ-105: Interpolation MUST operate in capture time, using the immediately bracketing compatible captures and linear interpolation of residual power before conversion back to dB. It MUST NOT interpolate across subcarrier positions or extrapolate outside the available time range.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-106"></a>
- REQ-106: An exact-time interpolation request MUST select the unique capture at that time. Ambiguous duplicate epochs MUST produce an error. Non-exact interpolation MUST require valid readings at both endpoints for each derived carrier; otherwise that carrier MUST remain missing.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-107"></a>
- REQ-107: Derived RxMER provenance MUST include all contributing capture IDs, original epochs, operation, target time where applicable, weights and missing-value policy. A derived observation MUST NOT be represented as an original measured capture.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-108"></a>
- REQ-108: For nearest-time channel selection, `Individual` and `Each` MUST use the original RxMER epoch; `InterpolateAt` MUST use its target epoch. `AverageAll` MUST require explicit channel selection or explicit unity response, because a multi-capture average has no unique capture epoch.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-109"></a>
- REQ-109: Averaging and interpolation MUST be performed in Rust using existing shared operations where suitable. The facade MUST NOT duplicate the calculation in Python. Interpolation here MUST represent a static effective observation; it MUST NOT imply continuous stateful playback.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-110"></a>
- REQ-110: Channel estimates MUST have a dedicated import and collection interface, independent of RxMER loading, counts and capture times.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-111"></a>
- REQ-111: Each added channel estimate MUST retain its own typed identifier, complex response values, grid, original capture time and provenance. It MUST NOT be duplicated or rewritten merely to match multiple RxMER captures.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-112"></a>
- REQ-112: Channel selection MUST be applied to the effective RxMER experiment after RxMER selection or derivation. Unselected estimates MUST remain available for later runs without being applied implicitly.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-113"></a>
- REQ-113: Channel estimates MUST NOT be averaged or interpolated implicitly when the RxMER policy requests averaging or interpolation. Such channel-response processing is outside this initial requirement unless separately specified.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-050"></a>
- REQ-050: The controller MUST support configuration, profile assignment, independent capture addition, option selection and an explicit `run()` operation.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-051"></a>
- REQ-051: `run()` MUST validate and freeze a configuration snapshot before execution. Later changes MUST apply only to a subsequent run and MUST NOT modify previous reports.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-052"></a>
- REQ-052: The batch MUST execute the experiments defined by the RxMER selection policy. `Each` MUST produce one experiment per loaded RxMER in insertion order; `Individual`, `AverageAll` and `InterpolateAt` MUST each produce one experiment. Each experiment MUST begin with fresh simulation state and the recorded configured seeds.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-053"></a>
- REQ-053: Independent experiments MUST NOT be presented as continuous temporal playback. Stateful playback MUST be a separately named future operation.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-054"></a>
- REQ-054: The simulation MUST execute the existing supported source → TX → channel → RX → independent source-comparison chain. PRBS15 MUST be the initial default source.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-055"></a>
- REQ-055: Observation duration MUST be distinct from host runtime and acquisition/completion/drain intervals. Runtime, decoder iterations and retained details MUST have finite validated limits.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-056"></a>
- REQ-056: The initial API MUST support synchronous execution and cooperative cancellation. Cancellation MUST preserve completed scientific results and stop dispatch of remaining batch experiments.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-057"></a>
- REQ-057: Preflight errors MUST reject the batch before simulation. After execution starts, a failed or runtime-limited experiment MUST retain its partial report and MUST NOT prevent subsequent experiments, unless cancellation is requested.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-058"></a>
- REQ-058: Batch results MUST identify completed, partial and not-started experiments. A complete job status MUST NOT imply successful FEC recovery.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-059"></a>
- REQ-059: Identical inputs, resolved settings and seeds MUST produce equivalent scientific results within documented numerical tolerances. Host elapsed time MUST NOT be treated as a reproducible scientific result.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-060"></a>
- REQ-060: `run()` MUST return an owned batch report with one identifiable entry per requested effective RxMER experiment, including any not-started entry. Derived-input experiments MUST identify their operation and contributing captures.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-061"></a>
- REQ-061: Each executed experiment MUST expose status, input provenance, resolved settings, assumptions, execution coverage and structured diagnostics.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-062"></a>
- REQ-062: FEC reporting MUST include total, clean, corrected and uncorrectable codewords and independently detected source mismatches accepted by the decoder.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-063"></a>
- REQ-063: Uncorrectable counts MUST count the union of failed decoder stages. Corrections MUST reflect actual decoder changes and MUST NOT be inferred from iteration counts.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-064"></a>
- REQ-064: Profile reporting MUST include compared bits, bit errors, post-FEC BER, payload information-bit rate and available per-carrier simulated MER. LDPC/BCH failures and iteration statistics MUST be accessible.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-065"></a>
- REQ-065: Default FEC and profile summaries MUST share the observation-window scope. Whole-run totals and metrics with a different scope MUST be separately labeled.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-066"></a>
- REQ-066: Missing denominators, unavailable windows and unavailable measurements MUST remain optional/null. They MUST NOT become fabricated zero BER, zero rate or arbitrary finite MER.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-067"></a>
- REQ-067: Payload information-bit rate MUST NOT be labeled application goodput. Analytical screening MUST NOT populate simulated FEC outcomes.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-068"></a>
- REQ-068: Reports MUST describe simulated performance under recorded noise and receiver assumptions. They MUST NOT claim that RxMER uniquely reconstructs original interference or predicts a specific modem.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-069"></a>
- REQ-069: Structured JSON export MUST preserve schema version, status, metric scope, provenance and nullability. Export helpers MUST NOT overwrite an existing destination implicitly.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [reports.rs](../crates/ofdm/src/api/reports.rs), [accounting.rs](../crates/ofdm/src/simulation/channel/accounting.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-070"></a>
- REQ-070: Python MUST mirror the Rust workflow through native bindings, with owned result snapshots and structured exception mapping for pre-run errors.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-071"></a>
- REQ-071: The Python adapter MUST release the interpreter lock during Rust simulation and MUST preserve cancellation and partial-result semantics.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-072"></a>
- REQ-072: PyO3 bindings MUST remain in a separate optional adapter. Maturin MUST build wheels for declared supported targets. Installing a compatible precompiled wheel MUST NOT require Rust or Qt.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-073"></a>
- REQ-073: Python file conveniences MAY accept paths while the core remains path-independent. Unit-bearing Python properties MUST have documented conversion semantics.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-074"></a>
- REQ-074: A future generated GNU Octave adapter MUST consume the same deliberately small contract and Rust calculations. Octave implementation and arbitrary Rust-to-language translation MUST NOT be included in the initial Rust/Python milestone.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-080"></a>
- REQ-080: Tests MUST cover binary and JSON ingestion for each supported capture type, wrong-kind inputs, unsupported decoder contracts, null preservation and original/intermediate provenance.
  Owner: [architecture](architecture.md#pnm-measurement-input-direction); [lib.rs](../crates/pnm-input/src/lib.rs), [contract.rs](../crates/ofdm/src/scenario/pnm_json/contract.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [pnm_json_contract.rs](../crates/ofdm/tests/pnm_json_contract.rs). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-081"></a>
- REQ-081: Tests MUST cover imported and created profiles, derived OCD settings, defaults, valid overrides, incompatible overrides and unresolved bandwidth.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [configuration.rs](../crates/ofdm/src/api/configuration.rs), [inputs.rs](../crates/ofdm/src/api/inputs.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01), [API-OCD-01](development/checklist.md#task-api-ocd-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-082"></a>
- REQ-082: Tests MUST cover unequal capture counts, independent timestamps, explicit estimate reuse, unity, nearest-time success, missing/tied matches and foreign identifiers.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [association.rs](../crates/ofdm/src/scenario/association.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-083"></a>
- REQ-083: Tests MUST establish that loading does not execute simulation, failed additions are atomic, and repeated runs preserve prior result ownership.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-084"></a>
- REQ-084: Tests MUST compare facade results with the existing engine for supported ideal and measurement-driven cases, including counters, BER, rates and metric scopes.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-085"></a>
- REQ-085: Tests MUST cover cancellation, runtime limits, failed experiments, continuation, not-started entries, zero compared bits and preserved completed science.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [simulation.rs](../crates/ofdm/src/api/simulation.rs), [controller.rs](../crates/ofdm/src/api/controller.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-086"></a>
- REQ-086: Rust/Python parity tests MUST use identical resolved settings and seeds. Counter/status equality MUST be exact; floating-point tolerance MUST be documented.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-087"></a>
- REQ-087: Qt independence MUST be verified by building and running non-Qt targets in an environment without Qt. Wheel installation tests MUST use environments without a Rust compiler.
  Owner: [architecture](architecture.md#optional-native-python-api); [lib.rs](../bindings/python/src/lib.rs), [mod.rs](../crates/ofdm/src/api/mod.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-088"></a>
- REQ-088: Requirement acceptance MUST distinguish inspected source, implemented behavior and executed test evidence. Examples or file presence MUST NOT be reported as runtime verification.
  Owner: [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-114"></a>
- REQ-114: Tests MUST cover individual RxMER selection, default `Each` behavior, power-domain averaging of all compatible captures, missing contributors, incompatible grids and preservation of originals. Averaging equal 20 dB and 30 dB readings MUST yield approximately 22.596373 dB, not 25 dB.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-115"></a>
- REQ-115: Tests MUST cover power-domain interpolation at an interior time, exact endpoints, unequal time intervals, missing endpoint readings, duplicate-time ambiguity and out-of-range rejection. Midpoint interpolation between 20 dB and 30 dB MUST yield approximately 22.596373 dB.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).

<a id="req-116"></a>
- REQ-116: Tests MUST verify one report for each derived/individual experiment, original source provenance, interpolation weights, explicit-channel reuse and rejection of nearest-time channel selection for `AverageAll`. Python/Rust parity MUST cover every RxMER selection mode.
  Owner: [architecture](architecture.md#temporal-measurements-and-applied-profiles); [controller.rs](../crates/ofdm/src/api/controller.rs), [measurement_playback.rs](../crates/ofdm/src/simulation/channel/measurement_playback.rs). Delivery: [API-RUST-01](development/checklist.md#task-api-rust-01), [API-PY-01](development/checklist.md#task-api-py-01). Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [measurement_playback.rs](../crates/ofdm/tests/measurement_playback.rs), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md).



## Preservation and delivery acceptance routing

These direct links refine the original broad/named requirements without replacing
their wording. Historical local passes retain the limits in their linked records.
Integration gates remain pending; the three downstream evidence-gap tasks also
gate final regression acceptance. A test link alone is not a new execution.

| Existing requirement | Architecture / current owner | Maintained acceptance / historical record | Delivery |
| --- | --- | --- | --- |
| [REQ-DS-FEC-01](requirements.md#req-ds-fec-01) | [architecture](architecture.md#purpose-and-scope); [bch.rs](../crates/fec-codec/src/bch.rs), [ldpc.rs](../crates/fec-codec/src/ldpc.rs), [concatenated.rs](../crates/fec-codec/src/concatenated.rs) | Tests: [codec.rs](../crates/fec-codec/tests/codec.rs), [normative.rs](../crates/fec-codec/tests/normative.rs), [concatenated.rs](../crates/fec-codec/tests/concatenated.rs). Record: [rust-only-signal-processing/architecture](development/features/rust-only-signal-processing/architecture.md). | [INT-REG-01](development/checklist.md#int-reg-01) |
| [REQ-DS-PHY-01](requirements.md#req-ds-phy-01) | [architecture](architecture.md#qam-coding-and-diagnostic-decisions-qam-0203); [parameters.rs](../crates/ofdm/src/signal_processing/modulation/interleaving/parameters.rs), [variable.rs](../crates/ofdm/src/signal_processing/receiver/payload/variable.rs) | Tests: [qam_coding_orders.rs](../crates/ofdm/tests/qam_coding_orders.rs), [shortened_transport.rs](../crates/ofdm/tests/shortened_transport.rs), [uniform_profile_orders.rs](../crates/ofdm/tests/uniform_profile_orders.rs). Record: [qam-completion/validation](development/features/qam-completion/validation.md). | [INT-REG-01](development/checklist.md#int-reg-01) |
| [REQ-DS-RUN-01](requirements.md#req-ds-run-01) | [architecture](architecture.md#configurable-duration-channel-stepping); [runner.rs](../crates/ofdm/src/simulation/channel/runner.rs), [job.rs](../crates/ofdm/src/simulation/channel/job.rs) | Tests: [channel_runner.rs](../crates/ofdm/tests/channel_runner.rs), [channel_job.rs](../crates/ofdm/tests/channel_job.rs), [run_infrastructure.rs](../crates/ofdm/tests/run_infrastructure.rs). Record: [channel-runner/validation](development/features/channel-runner/validation.md), [run-infrastructure/validation](development/features/run-infrastructure/validation.md). | [INT-REG-01](development/checklist.md#int-reg-01) |
| [REQ-DS-INPUT-01](requirements.md#req-ds-input-01) | [architecture](architecture.md#saved-workbench-api-input); [workbench.rs](../crates/pnm-input/src/workbench.rs), [designer.rs](../crates/pnm-input/src/workbench/designer.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [workbench.rs](../crates/pnm-input/tests/workbench.rs), [designer_workbench.rs](../crates/pnm-input/tests/designer_workbench.rs), [test_workbench.py](../bindings/python/tests/test_workbench.py), [test_designer_workbench.py](../bindings/python/tests/test_designer_workbench.py). Record: [high-level-api/design](development/features/high-level-api/design.md), [high-level-api/validation](development/features/high-level-api/validation.md). | [INT-REG-01](development/checklist.md#int-reg-01) |
| [REQ-DS-API-01](requirements.md#req-ds-api-01) | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [INT-RUST-01](development/checklist.md#int-rust-01) |
| [REQ-DS-UI-01](requirements.md#req-ds-ui-01) | [architecture](architecture.md#ui-merlin-simulation-workbench); [session.rs](../apps/qt/src/workbench/session.rs), [Workbench.qml](../apps/qt/qml/shell/Workbench.qml) | Tests: [tst_workbench.qml](../apps/qt/tests/tst_workbench.qml). Record: [qt-closed-loop/validation](development/features/qt-closed-loop/validation.md). | [INT-REG-01](development/checklist.md#int-reg-01) |
| [REQ-DS-DELIVERY-01](requirements.md#req-ds-delivery-01) | [architecture](architecture.md#completion-group-dynamic-control-layout-acquisition-and-delivery); [package.py](../tools/packaging/package.py), [ci.yml](../.github/workflows/ci.yml), [release.yml](../.github/workflows/release.yml) | Tests: [test_packaging.py](../tools/packaging/test_packaging.py). Record: [completion-group/validation](development/features/completion-group/validation.md). | [INT-TOOLS-01](development/checklist.md#int-tools-01) |
| [REQ-PY-DOC-001](requirements.md#req-py-doc-001) | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-PY-DOC-002](requirements.md#req-py-doc-002) | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-PY-DOC-003](requirements.md#req-py-doc-003) | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-PY-DOC-004](requirements.md#req-py-doc-004) | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-PY-DOC-005](requirements.md#req-py-doc-005) | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-PY-DOC-006](requirements.md#req-py-doc-006) | [architecture](architecture.md#python-only-installation); [update_integration_doc.py](../tools/python/update_integration_doc.py), [git-save.sh](../tools/git/git-save.sh), [git-push.sh](../tools/git/git-push.sh) | Tests: [test_integration_doc.py](../tools/python/test_integration_doc.py), [test_release.py](../tools/git/tests/test_release.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-REPO-ARCHIVE-001](requirements.md#req-repo-archive-001) | [architecture](architecture.md#python-only-installation); [.gitignore](../.gitignore) | Read-only archive ignore-rule check: [backfill evidence](development/features/downstream-requirements/validation.md). | [API-INTEGRATION-02](development/checklist.md#task-api-integration-02) |
| [REQ-API-PARITY-001](requirements.md#req-api-parity-001) | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INPUT-02](development/checklist.md#task-api-input-02) |
| [REQ-API-PARITY-002](requirements.md#req-api-parity-002) | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INPUT-02](development/checklist.md#task-api-input-02) |
| [REQ-API-PARITY-003](requirements.md#req-api-parity-003) | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INPUT-02](development/checklist.md#task-api-input-02) |
| [REQ-API-PARITY-004](requirements.md#req-api-parity-004) | [architecture](architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters); [controller.rs](../crates/ofdm/src/api/controller.rs), [lib.rs](../crates/pnm-input/src/lib.rs), [lib.rs](../bindings/python/src/lib.rs) | Tests: [high_level_api.rs](../crates/pnm-input/tests/high_level_api.rs), [test_api.py](../bindings/python/tests/test_api.py), [test_parity.py](../bindings/python/tests/test_parity.py). Record: [high-level-api/validation](development/features/high-level-api/validation.md). | [API-INPUT-03](development/checklist.md#task-api-input-03) |

| [REQ-COM-REUSE-01](requirements.md#req-com-reuse-01) | [architecture](architecture.md#unified-shared-dsp-and-fec); [lib.rs](../crates/fec-codec/src/lib.rs) | Review exact algorithms, independent vectors and measured performance; extraction acceptance remains pending. [backfill evidence](development/features/downstream-requirements/validation.md); [integration evidence boundary](development/features/unified-backend/validation.md) | [INT-REUSE-01](development/checklist.md#int-reuse-01), [INT-DSP-01](development/checklist.md#int-dsp-01), [INT-FEC-01](development/checklist.md#int-fec-01) |
| [REQ-COM-IDENTITY-01](requirements.md#req-com-identity-01) | [architecture](architecture.md#unified-rust-python-and-cli-apis); [mod.rs](../crates/ofdm/src/api/mod.rs) | Preserve current imports, native Python/CLI contracts, distribution inventories and attribution; source-only identities are provenance. [backfill evidence](development/features/downstream-requirements/validation.md); [integration evidence boundary](development/features/unified-backend/validation.md) | [INT-RUST-01](development/checklist.md#int-rust-01), [INT-PY-01](development/checklist.md#int-py-01), [INT-TOOLS-01](development/checklist.md#int-tools-01) |
| [REQ-COM-DOC-01](requirements.md#req-com-doc-01) | [architecture](architecture.md#requirements-and-historical-evidence-ownership); [requirements.md](requirements.md) | Backfill structural evidence is distinct from final runtime/module/document correspondence; downstream gaps have scoped evidence; final documentation remains pending. [backfill evidence](development/features/downstream-requirements/validation.md); [integration evidence boundary](development/features/unified-backend/validation.md) | [DS-REQ-01](development/checklist.md#ds-req-01), [INT-DOC-02](development/checklist.md#int-doc-02) |

### Existing documentation-consolidation requirements

DOC-R1..R5 below retain the original feature contract identities (spelled
`DOC-R1` through `DOC-R5` in that record). They close documentation planning
only; the final integration/documentation gates remain open.

| Existing ID | Design / architecture / implementation | Checklist and evidence |
| --- | --- | --- |
| <a id="ds-doc-r1"></a>**DOC-R1** | [original clause](development/features/unified-backend/design.md#requirements-and-acceptance); [architecture](architecture.md#unified-backend-integration); [canonical documents](development/features/unified-backend/design.md#architecture-and-file-ownership) | [INT-DOC-01](development/checklist.md#int-doc-01); [documentation evidence](development/features/unified-backend/validation.md) |
| <a id="ds-doc-r2"></a>**DOC-R2** | [original clause](development/features/unified-backend/design.md#requirements-and-acceptance); [architecture](architecture.md#unified-backend-integration); [canonical documents](development/features/unified-backend/design.md#architecture-and-file-ownership) | [INT-DOC-01](development/checklist.md#int-doc-01); [documentation evidence](development/features/unified-backend/validation.md) |
| <a id="ds-doc-r3"></a>**DOC-R3** | [original clause](development/features/unified-backend/design.md#requirements-and-acceptance); [architecture](architecture.md#unified-backend-integration); [canonical documents](development/features/unified-backend/design.md#architecture-and-file-ownership) | [INT-DOC-01](development/checklist.md#int-doc-01); [documentation evidence](development/features/unified-backend/validation.md)  Documentation delivery: [DOC-ORG-01](development/checklist.md#doc-org-01). |
| <a id="ds-doc-r4"></a>**DOC-R4** | [original clause](development/features/unified-backend/design.md#requirements-and-acceptance); [architecture](architecture.md#unified-backend-integration); [canonical documents](development/features/unified-backend/design.md#architecture-and-file-ownership) | [INT-DOC-01](development/checklist.md#int-doc-01); [documentation evidence](development/features/unified-backend/validation.md) |
| <a id="ds-doc-r5"></a>**DOC-R5** | [original clause](development/features/unified-backend/design.md#requirements-and-acceptance); [architecture](architecture.md#unified-backend-integration); [canonical documents](development/features/unified-backend/design.md#architecture-and-file-ownership) | [INT-DOC-01](development/checklist.md#int-doc-01); [documentation evidence](development/features/unified-backend/validation.md) |
