# MERLIN requirements baseline

This is MERLIN's authoritative behavior and acceptance baseline. The
[architecture](architecture.md) owns design/module placement and the
[official developer checklist](development/checklist.md) owns completion status.
Feature contracts and historical source records refine these obligations; they
are not separate competing baselines. Requirements apply to their declared scope.
Current downstream support is retained; upstream and common extraction requirements
below are **pending integration acceptance**, regardless of standalone source passes.

## Contents

- [Baseline rules](#baseline-rules)
- [Downstream obligations](#downstream-obligations)
- [Common consolidation requirements](#common-consolidation-requirements)
- [Upstream and inherited common contracts](#upstream-and-inherited-common-contracts)
- [Existing downstream API contract](#high-level-api-acceptance-contract)

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
| <a id="req-ds-fec-01"></a>REQ-DS-FEC-01 | MERLIN MUST retain current BCH/LDPC, shortening, NCP and PLC code definitions, outcomes and public codec imports. | [FEC ownership](architecture.md#unified-shared-dsp-and-fec); `crates/fec-codec/src/` and downstream transmission owners. | Existing `crates/fec-codec/tests/{normative,codec,shortening_codec,ncp,plc_decoding}.rs` plus downstream control-channel integration; INT-REG-01. |
| <a id="req-ds-phy-01"></a>REQ-DS-PHY-01 | Downstream QAM/LLR/bit and cell interleaving, randomization, pilots, packing, PLC/NCP and waveform processing MUST preserve exact supported behavior and grids. | [Downstream path](architecture.md#unified-downstream-processing); `crates/ofdm/src/{transmission,signal_processing}/`. | `crates/ofdm/tests/{payload_transport,waveform_integration,qam_mapping,receiver_configuration}.rs` and all relevant existing parameter suites; INT-REG-01. |
| <a id="req-ds-run-01"></a>REQ-DS-RUN-01 | Existing source/receiver/channel jobs MUST retain truth separation, duration/startup/drain, bounded work, deterministic seeds and partial/cancelled outcomes. | [Lifecycle](architecture.md#lifecycle-validation-and-future-interfaces); `crates/ofdm/src/simulation/`. | Existing `crates/ofdm/tests/{channel_job,duration_timing,prbs_source,file_source_channel}.rs`; INT-REG-01. |
| <a id="req-ds-input-01"></a>REQ-DS-INPUT-01 | Capture bytes/hashes/grids/nulls/profile identities, association, measurement policies and saved workbench compatibility MUST remain intact. | [PNM ownership](architecture.md#pnm-measurement-input-direction); `crates/pnm-input/src/`, downstream `scenario/` and `api/`. | Existing `crates/pnm-input/tests/{high_level_api,workbench,designer_workbench}.rs`; INT-REG-01. |
| <a id="req-ds-api-01"></a>REQ-DS-API-01 | Existing Rust imports, Python exports, CLI syntax, report/schema defaults and saved recipes MUST remain compatible; shared capabilities MUST land in Rust first. | [Unified adapters](architecture.md#unified-rust-python-and-cli-apis); `crates/ofdm/src/api/`, `bindings/python`, `crates/cli`. | Existing native `bindings/python/tests/{test_api,test_parity,test_workbench,test_designer_workbench}.py`, API examples and CLI cases; INT-RUST-01, INT-PY-01, INT-CLI-01, INT-REG-01. |
| <a id="req-ds-ui-01"></a>REQ-DS-UI-01 | Backend integration MUST preserve existing Qt downstream presentation/readiness/persistence; it MUST NOT add upstream UI as a side effect. | [Qt boundary](architecture.md#ui-merlin-simulation-workbench); `apps/qt/`. | Relevant existing Qt tests/build only if integration changes their dependencies/contracts; INT-REG-01. Detailed UI status remains subordinate in `docs/development/ui-checklist.md`. |
| <a id="req-ds-delivery-01"></a>REQ-DS-DELIVERY-01 | Existing VERSION/source pins, licenses, package targets, Master/Release security/native gates and generated Python installation guide policy MUST remain authoritative. | [Delivery](architecture.md#unified-normative-evidence-and-exclusions); current `tools/`, `.github/workflows/` and setup owners. | Existing tooling, version, both-lock audit, release/package/native gates when applicable; INT-LICENSE-01, INT-TOOLS-01, INT-ACCEPT-01. |

## Common consolidation requirements

| ID | Required behavior | Architecture / owner | Acceptance / task |
| --- | --- | --- | --- |
| <a id="req-com-reuse-01"></a>REQ-COM-REUSE-01 | Shared algorithms MUST be selected by exact semantic/equivalence and performance evidence; direction-specific normative state MUST remain separate. | [Shared DSP/FEC](architecture.md#unified-shared-dsp-and-fec); existing core owners; conditional `crates/dsp`. | Function/path reuse matrix, independent vectors, retained golden values, allocation/runtime measurements; INT-REUSE-01, INT-DSP-01, INT-FEC-01. |
| <a id="req-com-identity-01"></a>REQ-COM-IDENTITY-01 | The integrated product MUST remain MERLIN with one CLI/Python family; former product crates/controllers/imports MUST NOT be introduced. Required source attribution and raw source metadata MUST remain accurate. | [Crates](architecture.md#unified-crate-ownership); local manifests, APIs, packaging metadata and NOTICE. | Source/import/export/package inventory plus downstream compatibility; INT-RUST-01, INT-PY-01, INT-CLI-01, INT-TOOLS-01. |
| <a id="req-com-doc-01"></a>REQ-COM-DOC-01 | One architecture, requirements baseline and official checklist MUST cover both directions with exact source/document/task mappings and final implementation evidence. | [Final documentation gate](architecture.md#unified-acceptance-and-final-documentation-gate); canonical docs and skills. | Complete source inventory, unique IDs, dependency/coverage/link checks and final module/API/runtime correspondence; INT-DOC-01, INT-DOC-02. |

## Upstream and inherited common contracts

Each contract below retains the pinned source's observable behavior and acceptance
except the explicitly recorded local-policy adaptations. Source aliases and exact
source text remain fingerprinted/pinned; common contracts also bind existing
MERLIN behavior. All new upstream consumers await the indicated integration task.

### Shared source contracts

#### REQ-COM-SYS-03

Scientific algorithms, validation and orchestration SHALL execute in safe Rust independently of presentation or language runtimes. Adapters MUST NOT duplicate scientific calculations.

Acceptance: Core-only build/tests require neither Python nor Qt; adapters contain no DSP/FEC/association duplicates, native DSP/codec FFI or unsafe simulation code.

Architecture: [canonical owner](architecture.md#unified-crate-ownership).
Implementation: `crates/ofdma/src/lib.rs and core manifests`. Acceptance owner: `Rust-only dependency/build inspection and no-unsafe guards`.
Delivery: [**INT-REUSE-01**](development/checklist.md#int-reuse-01); source alias `REQ-SYS-03`.

#### REQ-COM-SYS-05

Scientific claims SHALL identify pinned evidence and the applicable endpoint without confusing internal closed-loop evidence with external interoperability. Each stage SHALL have source-checked expected inputs/outputs recorded before its implementation, independently of production tables/algorithms.

Acceptance: Source/matrix/label checks record revision, section/page and source-review gaps; reports/docs label internal experiments accurately and retain optional-capability distinctions. Build these cases stage by stage; external interoperability fixtures and a complete upfront catalog are not prerequisites.

Architecture: [canonical owner](architecture.md#unified-normative-evidence-and-exclusions).
Implementation: `crates/fec-codec/src/upstream_ldpc/definitions.rs`. Acceptance owner: `crates/fec-codec/tests/upstream_source_audit.rs`.
Delivery: [**INT-FEC-01**](development/checklist.md#int-fec-01); source alias `REQ-SYS-05`.

#### REQ-COM-OBS-01

Shared Rust QAM and SA instruments SHALL inspect bounded real stage observations at named TX/channel/RX reference planes.

Acceptance: Query results identify run, configuration, tap, grid/sample units and retention coverage; absent history remains unavailable; instrument enable/disable/query does not alter waveform, RNG or decoder behavior.

Architecture: [canonical owner](architecture.md#unified-channel-and-measurement-reference-planes).
Implementation: `crates/ofdma/src/instruments/metrics.rs`. Acceptance owner: `crates/ofdma/tests/matched_metrics.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-OBS-01`.

#### REQ-COM-OBS-02

Scientific results SHALL distinguish measured observations, analytical/model inputs, receiver estimates and simulated FEC/source outcomes.

Acceptance: Reports preserve convergence/parity/known-zero/source-error distinctions, failed words, actual iterations, partial coverage and provenance; zero observed error uses an explicit status rather than fabricated finite RxMER or JSON infinity.

Architecture: [canonical owner](architecture.md#unified-channel-and-measurement-reference-planes).
Implementation: `crates/ofdma/src/instruments/metrics.rs`. Acceptance owner: `crates/ofdma/tests/matched_metrics.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-OBS-02`.

#### REQ-COM-API-03

Both APIs SHALL return owned results and structured domain errors using native language conventions.

Acceptance: Error categories/details match without parsing messages; Python uses exceptions/None and checked integer/buffer conversions; outputs remain valid after later calls, input release and session disposal; zero-copy claims require lifetime proof.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/ofdma/tests/api_parity.rs`.
Delivery: [**INT-RUST-01**](development/checklist.md#int-rust-01); source alias `REQ-API-03`.

#### REQ-COM-API-04

Native Python execution SHALL release the interpreter lock during long Rust work and serialize mutable session access with explicit Busy behavior.

Acceptance: Actual native concurrency tests demonstrate independent cancellation/progress access, deterministic Busy for competing calls and session restoration after normal/error/cancel exits; no Python callback executes inside core locks.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/ofdma/tests/api_parity.rs`.
Delivery: [**INT-PY-01**](development/checklist.md#int-py-01); source alias `REQ-API-04`.

#### REQ-COM-API-05

Rust and Python contracts, examples and Python type declarations SHALL describe the same implemented capability set.

Acceptance: Independently executed bounded native runs match resolved settings, seeds, provenance, status, FEC outcomes and partial coverage with declared numerical tolerances; actual extension exports match `.pyi`/typing metadata and paired examples run.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/ofdma/tests/api_parity.rs`.
Delivery: [**INT-PY-01**](development/checklist.md#int-py-01); source alias `REQ-API-05`.

#### REQ-COM-API-06

Exported/configuration documents SHALL use explicit independent versions and declared compatibility rules.

Acceptance: Unknown versions/typed fields reject or follow an explicit migration; no FEC report is reinterpreted as configuration/workbench; any later shared saved experiment runs through both APIs without dropped settings or manual conversion.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/ofdma/tests/api_parity.rs`.
Delivery: [**INT-RUST-01**](development/checklist.md#int-rust-01); source alias `REQ-API-06`.

#### REQ-COM-LIFE-01

Runs SHALL freeze accepted settings with distinct run/configuration identities and deterministic independent source/noise state.

Acceptance: Repeated runs with equal seeds/settings reproduce declared deterministic outputs; observers/chunking do not affect science; stale results or cross-session IDs are rejected.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-LIFE-01`.

#### REQ-COM-LIFE-02

Cancellation/failure SHALL preserve completed science and identify incomplete or not-started work under bounded safe work quanta.

Acceptance: Stage-boundary and between-word cancellation tests retain actual completed results/counts, pending tails/ranges and partial status; no forced thread termination or invented completion is used.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-LIFE-02`.

#### REQ-COM-LIFE-03

Pause/resume, when exposed, SHALL preserve all scientific stream state and define paused-time and stop-from-paused semantics. Bounded cancellation and partial-result acceptance SHALL precede pause/resume delivery.

Acceptance: Tests retain PRBS/mask/permutation/Pre-EQ/RNG/history state and pending output across pause; controls remain unavailable in an API until its safe-boundary/lifecycle contract is implemented. First-loop readiness MUST NOT depend on pause/resume; later exposure requires state/time/stop-from-paused tests.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-LIFE-03`.

#### REQ-COM-NFR-01

Inputs, expanded plans, per-call work, cumulative runtime/memory, output queues and retained observations SHALL be bounded before allocation/execution.

Acceptance: Existing architecture budgets and independently checked overflow/exhaustion cases reject without state mutation; final cumulative job budgets are documented before execution support.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-NFR-01`.

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

#### REQ-US-SYS-02

Normal receiver inputs SHALL contain received samples and legitimate accepted receiver configuration. They SHALL NOT contain source truth or injected channel/noise truth.

Acceptance: Receiver admission tests exclude original bits, PRBS seed/ledger, injected H/C, RxMER targets and noise seeds/power; diagnostic truth is consumed only by separate comparisons.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-SYS-02`.

#### REQ-US-SYS-04

The toolkit SHALL distinguish representable settings, domain validity and executable support.

Acceptance: Unsupported optional modes remain inspectable with reasons; execution rejects them before consuming input, random state or source bits; recognized QAM/code identifiers alone never imply a complete coding chain.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/simulation/run.rs`. Acceptance owner: `crates/ofdma/tests/closed_loop.rs`.
Delivery: [**INT-US-LOOP-01**](development/checklist.md#int-us-loop-01); source alias `REQ-SYS-04`.

#### REQ-US-CFG-01

US Configuration SHALL retain a discoverable profile/preset inventory and explicitly select a one-CM/one-channel configuration shared by both endpoints.

Acceptance: Rust inventory exposes every retained entry without hardcoded adapter catalogs; invalid selection preserves the prior accepted plan; inventory does not automatically create multi-preset trials.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-01`.

#### REQ-US-CFG-02

Configuration admission SHALL produce an immutable CMTS plan and separate accepted TX/RX views through shared validation.

Acceptance: Both views agree on permitted geometry/grant/pilot/PHY-seed settings while normal RX excludes source, C/update values and channel/noise truth; failure preserves accepted settings and readiness.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-02`.

#### REQ-US-CFG-03

Generic OFDMA JSON SHALL have an explicit format/schema version, named presets, selection and separated normative configuration versus simulation settings.

Acceptance: Schema/parser fixtures cover required/optional/null/default/unknown-field behavior, unsupported versions, bounded extension metadata and round trips; vendor IDs are optional provenance, never normative preset meanings.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-03`.

#### REQ-US-CFG-04

Configuration coverage SHALL account for every applicable field/option in the pinned MULPI OFDMA scope.

Acceptance: The field-level matrix records source path/page, endpoint, force/applicability, encoding/units/enums, JSON pointer, Rust owner, support state and independent fixtures; every inapplicable or unresolved row has an explicit disposition.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-04`.

#### REQ-US-CFG-05

Typed and document-based configurations SHALL use equivalent domain rules and bounded admission. The first reference loop SHALL use typed synthetic configuration; JSON and verified capture loaders SHALL follow its acceptance.

Acceptance: Invalid ranges, reserved values, inconsistent pilot/loading/geometry, associations and aggregate budgets reject atomically; valid synthetic configurations do not require an unverified capture decoder. Full applicable MULPI/JSON delivery remains required after the typed reference milestone.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-05`.

#### REQ-US-CFG-06

Example presets SHALL demonstrate generic configurations without claiming universal recommended modulation.

Acceptance: Reviewed 25/50 kHz, mixed-loading and zero-valued-minislot examples identify source versus engineering choices; ranging/probe examples explicitly state execution support; conditional/optional settings are not silently discarded.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/configuration/ucd/mod.rs`. Acceptance owner: `crates/ofdma/tests/configuration_json.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-CFG-06`.

#### REQ-US-PRBS-01

The source SHALL emit continuous deterministic PRBS15 using the architecture's recurrence, seed range and bit order, independently of PHY scrambling.

Acceptance: Literal initial vectors and the 32767-bit period pass for independently checked seeds; zero/out-of-range seeds reject; grants, words and chunk boundaries do not reseed the source.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-01`.

#### REQ-US-PRBS-02

Source consumption SHALL track accepted meaningful payload with checked absolute positions and bounded chunks.

Acceptance: Fill/shortening/parity/pad/pilots consume no source bits; backpressure and rejected work do not advance the generator; chunked generation matches contiguous generation and overflow rejects before mutation.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-02`.

#### REQ-US-PRBS-03

Verification SHALL compare actual recovered meaningful decisions using an independently owned expected stream and validated source association.

Acceptance: Wrong but valid codewords produce source errors even if parity passes; failures do not suppress available decisions; gaps/duplicates/reordering stop contiguous comparison explicitly without phase search or repair.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-03`.

#### REQ-US-PRBS-04

Verification results SHALL distinguish generated, compared and unverified bits, coverage, mismatch counts and completion.

Acceptance: Zero compared bits yield unavailable BER; zero observed errors with partial/truncated/discontinuous coverage cannot claim complete recovery; retained results survive later operations.

Architecture: [canonical owner](architecture.md#unified-upstream-configuration-and-source).
Implementation: `crates/ofdma/src/source/prbs15.rs`. Acceptance owner: `crates/ofdma/tests/prbs.rs`.
Delivery: [**INT-US-CONFIG-01**](development/checklist.md#int-us-config-01); source alias `REQ-PRBS-04`.

#### REQ-US-TX-01

Grant framing SHALL derive coded capacity and the upstream codeword sequence from accepted resources/loading while separating meaningful bytes, information-fill ones, shortening zeros, parity and zero grant pad.

Acceptance: Independent threshold/borrowing/boundary cases conserve bits and source consumption; RX derives the same layout from legitimate configuration; rejected plans preserve state.

Architecture: [canonical owner](architecture.md#us-tx-01-grant-framing-and-fec-selection).
Implementation: `crates/ofdma/src/grant/plan.rs`. Acceptance owner: `crates/ofdma/tests/grant_plan.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-01`.

#### REQ-US-TX-02

Encoding SHALL use the selected source-verified upstream long/medium/short LDPC family with systematic order and exact shortening semantics. All three code families SHALL pass source audit and codec acceptance before dependent waveform implementation.

Acceptance: Independently checked matrix entries/parity and full/shortened words pass; only appended shortening-zero coordinates are omitted; no downstream BCH or assumed puncturing is added.

Architecture: [canonical owner](architecture.md#us-tx-02-ldpc-encoding-and-shortening).
Implementation: `crates/fec-codec/src/upstream_ldpc/codec.rs`. Acceptance owner: `crates/fec-codec/tests/upstream_source_audit.rs`.
Delivery: [**INT-FEC-01**](development/checklist.md#int-fec-01); source alias `REQ-TX-02`.

#### REQ-US-TX-03

PHY randomization SHALL use the defined 23-bit grant seed, first-bit/tap convention and per-grant reset over exactly the transmitted bit positions.

Acceptance: Literal mask vectors, chunk continuity and reset tests cover parity and final grant pad; omitted shortening and known pilots add no clocks; invalid/reordered input preserves state.

Architecture: [canonical owner](architecture.md#us-tx-03-grant-seeded-phy-randomization).
Implementation: `crates/ofdma/src/bit_processing/randomizer.rs`. Acceptance owner: `crates/ofdma/tests/randomization.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-03`.

#### REQ-US-TX-04

QAM mapping SHALL form MSB-first cell words in accepted visit order using exact upstream labels, normalization and role-dependent loading.

Acceptance: Independent label/energy vectors cover the declared BPSK/QPSK through 4096-QAM scope including non-square orders; inactive resources consume no payload; unknown/unimplemented orders reject.

Architecture: [canonical owner](architecture.md#us-tx-04-cell-words-and-upstream-qam).
Implementation: `crates/ofdma/src/signal_processing/qam.rs`. Acceptance owner: `crates/ofdma/tests/qam_mapping.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-04`.

#### REQ-US-TX-05

OFDMA framing SHALL apply the accepted pilot pattern, known signs, role/loading plan, block partitioning and joint interleaving/address order.

Acceptance: Independent masks, pilot vectors and permutation/bijection cases cover supported patterns, edge/body layout, exclusions, zero resources and boost exceptions; bit/cell totals are conserved and scaling occurs once.

Architecture: [canonical owner](architecture.md#us-tx-05-pilots-resources-and-joint-interleaving).
Implementation: `crates/ofdma/src/transmitter/framing.rs`. Acceptance owner: `crates/ofdma/tests/resource_interleaving.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-05`.

#### REQ-US-TX-06

Pre-EQ SHALL apply one accepted complex corrective bank after role scaling and before IFFT, preserving the distinction between C and injected H.

Acceptance: Independent normalization/phase/default/reset/replacement/adjustment cases pass; excluded carriers remain zero; stale bases/nonfinite/invalid banks reject; bank transitions occur only at accepted symbol boundaries.

Architecture: [canonical owner](architecture.md#us-tx-06-cm-pre-eq).
Implementation: `crates/ofdma/src/signal_processing/pre_eq.rs`. Acceptance owner: `crates/ofdma/tests/pre_eq.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-06`.

#### REQ-US-TX-07

IFFT SHALL transform complete accepted 2K/50 kHz or 4K/25 kHz physical grids at 102.4 Msamples/s using the defined centered unitary convention.

Acceptance: Independent direct-sum, tone, impulse and energy checks establish sign/bin order/scaling; useful sample count and timing are exact; inactive bins and invalid geometry are checked.

Architecture: [canonical owner](architecture.md#us-tx-07-centered-unitary-ifft).
Implementation: `crates/ofdma/src/signal_processing/transform.rs`. Acceptance owner: `crates/ofdma/tests/waveform_transform.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-07`.

#### REQ-US-TX-08

Sample assembly SHALL apply the specified CP/RP, cyclic extension, taper and overlap while retaining exact useful/prefix origins, burst edges, tails and explicit guards.

Acceptance: Independent numbered-sample/window tests cover zero/nonzero RP and adjacent symbols; complete output is invariant across chunks/backpressure; invalid geometry rejects and cancellation retains honest partial status.

Architecture: [canonical owner](architecture.md#us-tx-08-prefix-window-and-sample-assembly).
Implementation: `crates/ofdma/src/transmitter/assembly.rs`. Acceptance owner: `crates/ofdma/tests/tx_framing.rs`.
Delivery: [**INT-US-TX-01**](development/checklist.md#int-us-tx-01); source alias `REQ-TX-08`.

#### REQ-US-CH-01

Channel admission SHALL validate finite indexed sample blocks, clock, identities, continuity and budgets before changing state.

Acceptance: Stale/duplicate/overlap/unexplained-gap/nonfinite/overflow inputs reject atomically; empty blocks are no-ops, not finish; unsupported composition rejects before TX consumption.

Architecture: [canonical owner](architecture.md#us-ch-01-indexed-sample-admission).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-01`.

#### REQ-US-CH-02

Initial unity transport SHALL preserve sample bit patterns and metadata exactly without impairments or random draws.

Acceptance: Bitwise tests include signed zero and arbitrary chunking with unchanged indices/rate/burst/tail/guard/validity; empty/disabled impairment settings use the same bypass.

Architecture: [canonical owner](architecture.md#us-ch-02-exact-unity-transport).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-02`.

#### REQ-US-CH-03

The response extension SHALL apply the declared causal synthetic complex FIR without implicit normalization, circular wrap or per-symbol resets.

Acceptance: Literal complex convolution/impulse/delay cases establish phase, original origin, state continuity and exact terminal drain; zero/deep-null responses remain valid destructive experiments.

Architecture: [canonical owner](architecture.md#us-ch-03-physical-channel-response).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-03`.

#### REQ-US-CH-04

Residual synthesis SHALL offer explicit Off, sample-AWGN and restricted RxMER-derived modes using separate deterministic noise state and declared reference power.

Acceptance: Independent power/statistical/draw-order tests cover seeds, missing-carrier policy and pilot/role scaling; initial RxMER synthesis rejects nonunity C/H or incompatible timing across the complete schedule.

Architecture: [canonical owner](architecture.md#us-ch-04-awgn-and-restricted-rxmer-residuals).
Implementation: `crates/ofdma/src/channel/residual.rs`. Acceptance owner: `crates/ofdma/tests/rxmer_channel.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-04`.

#### REQ-US-CH-05

The timing extension SHALL implement only the designed integer delay, constant phase and CFO with emitted-sample-clock continuity.

Acceptance: Independent impulse/tone/rotation cases establish signs/origin, chunk/pause continuity and exact delay drain; zero mode is bitwise bypass; fractional clock and stochastic phase noise remain unavailable.

Architecture: [canonical owner](architecture.md#us-ch-05-integer-timing-phase-and-cfo).
Implementation: `crates/ofdma/src/channel/stream.rs`. Acceptance owner: `crates/ofdma/tests/channel_models.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-05`.

#### REQ-US-CH-06

Channel output SHALL retain indexed samples and bounded passive observations while separating diagnostics from legitimate RX inputs.

Acceptance: Observer selection/capacity does not change samples or randomness; output validity/partial/drain metadata is retained; injected H/noise settings never become hidden RX corrections.

Architecture: [canonical owner](architecture.md#us-ch-06-output-and-passive-observations).
Implementation: `crates/ofdma/src/instruments/observer.rs`. Acceptance owner: `crates/ofdma/tests/matched_metrics.rs`.
Delivery: [**INT-US-CH-01**](development/checklist.md#int-us-ch-01); source alias `REQ-CH-06`.

#### REQ-US-RX-01

Burst framing SHALL derive timing from accepted RX configuration and preserve actual indexed blocks and explicit missing intervals.

Acceptance: Literal origin/stride/window cases pass; unexplained gaps/reordering reject; declared gaps retain coverage loss rather than becoming available zero samples; no hidden timing search is performed.

Architecture: [canonical owner](architecture.md#us-rx-01-configured-burst-framing-and-timing).
Implementation: `crates/ofdma/src/receiver/framing.rs`. Acceptance owner: `crates/ofdma/tests/rx_framing.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-01`.

#### REQ-US-RX-02

Useful-symbol extraction SHALL select exactly the scheduled N-sample windows without silently filling incomplete windows.

Acceptance: Numbered-sample/CP/overlap/truncation cases distinguish available zeros from missing samples and retain the correct symbol identities and unavailable outcomes.

Architecture: [canonical owner](architecture.md#us-rx-02-useful-symbol-extraction).
Implementation: `crates/ofdma/src/receiver/framing.rs`. Acceptance owner: `crates/ofdma/tests/rx_framing.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-02`.

#### REQ-US-RX-03

FFT SHALL produce raw physical bins using the inverse of the accepted centered unitary TX convention.

Acceptance: Independent complex-sum, signed-tone, impulse and energy cases establish bin/sign/scaling correctness and preserve availability metadata without injecting channel truth.

Architecture: [canonical owner](architecture.md#us-rx-03-centered-unitary-fft).
Implementation: `crates/ofdma/src/signal_processing/transform.rs`. Acceptance owner: `crates/ofdma/tests/waveform_transform.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-03`.

#### REQ-US-RX-04

Channel estimation/equalization SHALL estimate effective C×H from received known pilots using the declared FrameFlatLs or FramePilotLinear policy and explicit zero-forcing validity.

Acceptance: Independent complex-anchor/interpolation/notch cases verify known pilot gain/sign treatment, estimate coverage and unavailable outputs; no hidden supplied-H, fast within-frame tracking or noise-estimation capability is claimed.

Architecture: [canonical owner](architecture.md#us-rx-04-received-pilot-estimation-and-equalization).
Implementation: `crates/ofdma/src/signal_processing/estimation.rs`. Acceptance owner: `crates/ofdma/tests/rx_estimation.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-04`.

#### REQ-US-RX-05

Resource extraction SHALL independently invert the accepted resource/interleaving order and preserve unavailable scheduled cells and their bit positions.

Acceptance: Independent mixed-loading/permutation/boost cases conserve order/counts; missing cells are not compacted and known amplitude is removed once.

Architecture: [canonical owner](architecture.md#us-rx-05-resource-extraction-and-inverse-interleaving).
Implementation: `crates/ofdma/src/receiver/mod.rs`. Acceptance owner: `crates/ofdma/tests/rx_resources.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-05`.

#### REQ-US-RX-06

Soft demapping SHALL produce MSB-first finite MaxLog natural-log P(0)/P(1) metrics from shared upstream labels/scales and a valid ReferenceVariance.

Acceptance: Independent label/distance/sign tests cover positive-favors-zero convention, configured saturation, ties and zero-confidence erasures; nonpositive/nonfinite variance rejects.

Architecture: [canonical owner](architecture.md#us-rx-06-soft-qam-demapping).
Implementation: `crates/ofdma/src/signal_processing/demapping.rs`. Acceptance owner: `crates/ofdma/tests/soft_metrics.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-06`.

#### REQ-US-RX-07

Soft derandomization SHALL independently regenerate the grant mask and reverse LLR signs before LDPC without changing magnitude or position.

Acceptance: Mask-one metrics reverse sign; erasures stay zero; every transmitted position including pad advances the mask, while omitted shortening/pilots do not; word ordering and grant resets remain exact.

Architecture: [canonical owner](architecture.md#us-rx-07-soft-phy-derandomization).
Implementation: `crates/ofdma/src/bit_processing/randomizer.rs`. Acceptance owner: `crates/ofdma/tests/randomization.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-07`.

#### REQ-US-RX-08

LDPC integration SHALL restore shortening exactly once through the existing codec contract and retain real decoder outcomes.

Acceptance: Retained-length inputs, independent shortening cases and 1..100 iteration bounds pass; convergence, syndrome, shortened-zero consistency, failed decisions and actual iterations remain distinct; controls act between indivisible codec calls.

Architecture: [canonical owner](architecture.md#us-rx-08-shortening-restoration-and-ldpc-decoding).
Implementation: `crates/fec-codec/src/upstream_ldpc/codec.rs`. Acceptance owner: `crates/fec-codec/tests/upstream_source_audit.rs`.
Delivery: [**INT-FEC-01**](development/checklist.md#int-fec-01); source alias `REQ-RX-08`.

#### REQ-US-RX-09

Information deframing SHALL remove only receiver-known residual fill and retain actual complete/partial/failed meaningful decisions and coverage.

Acceptance: Independent byte/fill/short-word cases preserve positions and continuity; failures are not dropped; source association/comparison happens outside normal RX.

Architecture: [canonical owner](architecture.md#us-rx-09-information-deframing).
Implementation: `crates/ofdma/src/receiver/framing.rs`. Acceptance owner: `crates/ofdma/tests/rx_framing.rs`.
Delivery: [**INT-US-RX-01**](development/checklist.md#int-us-rx-01); source alias `REQ-RX-09`.

#### REQ-US-PNM-01

Imported observations SHALL preserve original bytes, hashes, timestamps, exact grids, nulls, identities, units and reference/provenance.

Acceptance: Original input remains recoverable after decode/selection/replay; malformed, incompatible and oversized inputs reject without replacing accepted state; upstream formats need pinned evidence and lawful fixtures.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-01`.

#### REQ-US-PNM-02

RxMER SHALL support explicit single selection, power-domain averaging and bounded scheduled hold/interpolation policies over multiple captures.

Acceptance: Independent conversions/weights/time-boundary/null cases match resolved power and coverage; original capture times stay unchanged; synthetic replay time is separate and importing more files does not create trials implicitly.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-02`.

#### REQ-US-PNM-03

US-Pre-EQ SHALL support a single fixed bank, multiple snapshots and multiple explicitly based initialization/adjustment updates.

Acceptance: Independent complex/base/reset tests reject stale, missing or ambiguous dependencies; immutable schedules apply complete accepted banks only at defined symbol boundaries and preserve probe/source associations.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-03`.

#### REQ-US-PNM-04

Combined input preflight SHALL validate the entire selected playback/update plan against channel/receiver support before execution.

Acceptance: A later incompatible Pre-EQ event fails preparation even if the initial bank is compatible; RxMER targets, corrective C and injected H remain separate; no unsupported composition is silently enabled.

Architecture: [canonical owner](architecture.md#unified-pnm-and-capture-admission).
Implementation: `crates/pnm-input/src/upstream/association.rs`. Acceptance owner: `crates/pnm-input/tests/upstream_association.rs`.
Delivery: [**INT-PNM-02**](development/checklist.md#int-pnm-02); source alias `REQ-PNM-04`.

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
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/ofdma/tests/api_parity.rs`.
Delivery: [**INT-RUST-01**](development/checklist.md#int-rust-01); source alias `REQ-API-01`.

#### REQ-US-API-02

The optional native Python API SHALL wrap working public Rust operations with equivalent domain validation and results. Native Python implementation SHALL begin after the Rust unity loop passes acceptance.

Acceptance: Real native calls cover configuration, single/multi inputs and execution; no Python DSP/association/decoder implementation or subprocess sidecar is used; Rust-only dependency checks exclude binding runtimes. No early FEC-only Python wrapper is part of the selected delivery order.

Architecture: [canonical owner](architecture.md#unified-rust-python-and-cli-apis).
Implementation: `crates/ofdma/src/api/session.rs`. Acceptance owner: `crates/ofdma/tests/api_parity.rs`.
Delivery: [**INT-PY-01**](development/checklist.md#int-py-01); source alias `REQ-API-02`.

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
