# Converged public Rust and native Python APIs

The [selected exact namespace mapping](namespace-design.md) uses a minimal `merlin` facade, independent PHY owners, a neutral common contract leaf and one real Python package. [INT-API-MAP-01](../../checklist.md#int-api-map-01) records the design decision; Rust and native Python exports now pass namespace and complete operation coverage acceptance; final workspace regressions remain gated.

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-rust-python-and-cli-apis).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Scope and support

This is the active feature design for converging direction-specific operations through
the existing public Rust engine and native Python distribution. Existing downstream
Rust/Python APIs, imports, schemas, examples and saved-workbench contracts remain
implemented within their recorded supported scope. Initial upstream typed Rust
composition now passes local synthetic session acceptance. The real facade/common
contracts now pass targeted Rust checks; complete Rust operation coverage is accepted; native
wrappers and complete native operation traceability now pass local INT-PY-01 acceptance. Measured Rust
admission now uses the adapter-owned MeasuredExperiment and core-owned typed
MeasuredConfiguration handoff; no core-to-capture dependency is introduced.

The [current backend master review](../unified-backend/master-prompt-review.md)
adds explicit direction namespaces, package-import acceptance and a fourteen-field
implementation evidence contract. The single [API compatibility matrix](compatibility-matrix.md)
owns operation mapping and the namespace proposal; it is initially incomplete for
the future upstream API. [REQ-COM-API-07](../../../requirements.md#req-com-api-07),
[REQ-COM-API-08](../../../requirements.md#req-com-api-08) and
[REQ-COM-API-09](../../../requirements.md#req-com-api-09) refine the existing
contracts. [INT-PLAN-01](../../checklist.md#int-plan-01) accepts planning only;
namespace approval precedes compatibility-sensitive implementation. Final delivery
stays with INT-RUST-01 and INT-PY-01 below.

## Dependency and operation ownership

Core PHY modules own typed validation, scientific state and results. The implemented
upstream `crates/ofdma/src/api/` composes those owners; the native input adapter owns
bounded file admission. The existing Python extension converts native values and
calls the same Rust operations. CLI and future Qt are consumers of those APIs.
No adapter computes a second result or introduces a second upstream distribution.

Typed Rust configuration/session foundation accompanies upstream stages before
loop acceptance. The actual source→TX→channel→RX→independent original comparison
loop precedes final Rust/Python convergence and any upstream Python facade. Do not
publish an early FEC-only Python product as a substitute for that upstream API.

## Request, lifecycle and result contracts

Inputs are owned immutable accepted configuration/provenance and explicit bounded
request/detail budgets. Shared capabilities land in Rust first, with matching Python
units, defaults, validation and unsupported/domain-valid distinctions. Capture/profile
inventories come from Rust discovery; examples promising all profiles cannot hardcode
a catalog. Inspect/configure/preview is idle, atomic and separate from running.

One accepted generation/run identity follows each report and retained tap. Busy,
invalid/unsupported/version errors and callback failures have structured matching
meanings. Cancellation/pause checks occur at defined safe boundaries and preserve
completed codec/scientific outcomes; genuine decoder failures remain scientific
results rather than fabricated transport success. Polling or taps cannot rerun work.

Returned Python objects own their data and do not alias mutable Rust worker state.
Actual native long-running work releases the interpreter lock; the job still owns
scientific synchronization and finite retention. Callbacks are outside scientific
locks, and failure/cancellation does not discard accepted partial results. These
upstream behaviors require the explicit native cases below, not wrapper sketches.

Existing saved-workbench/schema/embedded-payload compatibility applies in both
languages. Preserve byte/grid/provenance meaning, accepted defaults and error policy;
future format evolution requires explicit versioning/migration, not product renaming
of original source bytes or silent reinterpretation of old recipes.

## Acceptance, references and limitations

Run actual independently executed Rust and native-Python operations against the same
fixtures. Compare configuration/errors/owned reports, capture discovery, invalid/
unsupported admission, Busy, cancellation/pause and actual partial/failure decisions.
Keep deterministic science separate from elapsed time. Native import/type files or
existing test paths alone do not establish a successful native run.

The existing 93 original API clauses and retained named requirements remain in the
canonical baseline. This record links them rather than creating a parallel contract.
PHY-specific interpretation stays with the pinned directional feature owners;
API lifecycle/ownership policy is engineering behavior, not a new normative PHY claim.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-API-01](../../../requirements.md#req-us-api-01) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/merlin/tests/namespaces.rs` |
| [REQ-US-API-02](../../../requirements.md#req-us-api-02) | [INT-PY-01](../../checklist.md#int-py-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `bindings/python/tests/test_upstream_experiment.py` |
| [REQ-COM-API-03](../../../requirements.md#req-com-api-03) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/merlin/tests/namespaces.rs` |
| [REQ-COM-API-04](../../../requirements.md#req-com-api-04) | [INT-PY-01](../../checklist.md#int-py-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `bindings/python/tests/test_upstream_experiment.py` |
| [REQ-COM-API-05](../../../requirements.md#req-com-api-05) | [INT-PY-01](../../checklist.md#int-py-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `bindings/python/tests/test_upstream_experiment.py` |
| [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/merlin/tests/namespaces.rs` |
| [REQ-DS-API-01](../../../requirements.md#req-ds-api-01) | [INT-REG-01](../../checklist.md#int-reg-01) | Module: [crates/ofdm/src/api](../../../../crates/ofdm/src/api); Module: [bindings/python/src](../../../../bindings/python/src); Test: [bindings/python/tests/test_api.py](../../../../bindings/python/tests/test_api.py); Test: [bindings/python/tests/test_parity.py](../../../../bindings/python/tests/test_parity.py) |
| [REQ-COM-IDENTITY-01](../../../requirements.md#req-com-identity-01) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: [crates/ofdm/src/api](../../../../crates/ofdm/src/api); Module: [bindings/python/src](../../../../bindings/python/src); Test: [bindings/python/tests/test_api.py](../../../../bindings/python/tests/test_api.py) |

Related delivery gates: [INT-US-LOOP-01](../../checklist.md#int-us-loop-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01), [INT-CLI-01](../../checklist.md#int-cli-01).

## Supporting records and validation evidence

- [high-level-api/design.md](design.md).
- [high-level-api/requirements.md](requirements.md).
- [unified-backend/references/remaining-delivery-validation.md#aggregate-api-and-scientific-acceptance](../unified-backend/references/remaining-delivery-validation.md#aggregate-api-and-scientific-acceptance).
- [Public API / saved-workbench compatibility](../../../../skills/public-api/SKILL.md) and [paired native validation procedure](../../../../skills/public-api/references/paired-api.md).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.

## INT-RUST-01 complete operation coverage

Namespace checkpoint `c6e3065` provides the real crates and tests. Complete the
existing compatibility matrix with compiler-discovered explicit public methods,
including macro-generated quantities and public re-exports. The matrix's full
seventeen-cell rows live in `operation-matrix.json`; compatibility-matrix.md is
its navigable explanation and selected session view, not a second status owner.
Each operation is keyed by its stable public Rust path. Shared contract cells
resolve through named definitions, avoiding repeated prose across simple getters.

Reuse the current compiler export inventory and native 0.4.0 downstream metadata.
Current native metadata establishes existing signatures only; it does not accept
new Python namespaces or execution. Preserve deliberate language differences:
Rust borrowed views, generic Serde helpers, low-level typed builders and native
file conveniences need explicit mappings or justified language-only entries,
not artificial Python methods. New upstream native signatures follow the reviewed
source adapter; complete Rust coverage must distinguish those planned bindings
from actual exports. INT-PY-01 replaces plans with installed native observations.

Acceptance: every documented public Rust operation and hidden cross-crate Serde
helper has one complete row; every source path/signature and named Rust test
resolves; evidence has a real executed candidate and declared scope. Re-export
preservation is proved by all-type identity and retained downstream regressions,
not a claim that a new test re-exercises every getter branch. Full Python export
coverage and independent numerical parity remain their own delivery gate.

### Rust API acceptance

The complete Rust operation matrix passes fresh compiler-export and named executed
case checks: 364 rows, with deliberate language-only differences made explicit.
The namespace/common unit has 8 tests; downstream high-level/lifecycle compatibility
has 26/6 passing cases, upstream engine coverage has 83, and analysis-only has 139.
The additional measured-plan roundtrip brings current association API coverage to
9 cases. All 43 validation-tool tests pass, including negative contract/signature/
execution fixtures. The 5 release/version tooling cases, Clippy, formatting and
required direction/common-only builds pass. No science or accepted FEC behavior is
changed by the matrix/owner reconciliation.

Full native export discovery, independent Python numerical/lifecycle parity and
installed-wheel acceptance remain INT-PY-01. Final complete-workspace regressions
remain INT-REG-01; existing historical downstream acceptance remains intact.

Exact API-coverage evidence: [candidate](operation-candidate.json),
[requirement record](operation-evidence.json) and [named run records](operation-test-results.json).
Initial owner/test locators in the preserved source-requirement inventory remain
planning provenance; the functional owners and exact records above are current.

## INT-PY-01 native namespace implementation unit

Prerequisite Rust acceptance: `854caec`. Adapt the six reviewed source native
modules into `bindings/python/src/upstream/`; retain their typed conversion,
Busy/interpreter-lock/cancellation/report ownership behavior. Local `api` imports
combine facade upstream types and pnm-input capture admission inside the adapter
only. Neither engine gains an adapter/Python dependency. Existing downstream
native code changes only its API import alias plus the additive common stop query.

Select one `_native` extension and real import-only package modules under
`bindings/python/python/merlin/`. Preserve every root downstream class and its
`__module__ = "merlin"`; directional modules re-export the exact native objects.
Add `merlin/merlin.py` as a legacy import shim for the previous generated native
module path, retaining one binary. Keep distribution name/version and PyO3 pins.
Register upstream classes privately with public `merlin.upstream` class identities.
The upstream control constructor optionally accepts the common native token.

| Existing/source owner | Decision / concrete destination |
| --- | --- |
| `bindings/python/src/{lib,controller,...}.rs` | REUSE downstream objects, conversion and engine; edit entry point/import only and add read-only cancellation query. |
| `morgana/bindings/python/src/{capture,configuration,document,measurements,reports,simulation}.rs` | ADAPT into `bindings/python/src/upstream/`; local sibling imports and facade/input owners; no copied algorithms. |
| Source native error conversion | ADAPT to the existing root ApiError and shared ErrorCode; preserve upstream field/offset/legacy code as typed optional metadata. Existing downstream messages/code behavior stays the same. |
| Source `configure_measured_json` | ADAPT: pnm-input admits/prepares the plan, then calls the accepted typed core handoff. No core capture parser is added. |
| Source four native test files / six upstream Rust examples | ADAPT into local upstream tests and pnm-input Rust examples, with exact public imports/fixtures/schema names. Preserve assertions and independently executed Rust comparisons. |
| `tools/python/{build_api_reference,smoke_wheel}.py` | EDIT private-extension detection and installed package/typing checks, retaining legacy example/FEC and notice gates. |

New root/directional/common `.pyi` files and py.typed describe actual objects and
signatures. New exception metadata defaults to None, with upstream errors retaining
original field/offset and legacy diagnostic strings; common machine categories
come directly from Rust, never message parsing. Verify actual installed-wheel
imports/discovery/typing/one-binary/legacy identity outside the checkout, all old
native cases, source-derived upstream parity/lifecycle cases, shared cancellation
while paused, notices and every exported operation before accepting INT-PY-01.
Only local isolated build/test environments are used; no system installation.

### Installed namespace unit validation

The [candidate](python-candidate.json), [46 executed native cases](python-test-results.json)
and [requirement slices](python-evidence.json) identify this implementation unit.
All 25 original downstream cases, 17 adapted upstream cases and four namespace
cases pass in an installed Linux x86_64 CPython 3.10 wheel, outside the checkout,
with fresh separate Rust references enabled (29.254 s). This includes actual
SIGINT, interpreter-lock release, Busy, pause/resume/cancel, retained results,
raw capture provenance and both upstream numerologies. Other interpreters and
Windows runtime remain unverified.

The existing wheel smoke passes both downstream examples: 82 total codewords,
zero corrected/uncorrectable and zero observed BER in each (5.15 s including a
fresh installation). The paired unified upstream examples discover profiles
through Rust and recover all 1,976 compared source bits with zero errors.
Nineteen Python tool cases, native all-target Clippy, affected Rust all-target
Clippy and both formatting checks pass.

The complete suite initially exposed comparison of process-relative controller
scope counters. Compare their equality/distinctness relationships instead,
retaining exact slots, policies and scientific data; no engine behavior changed.
The first mixed wheel also omitted notices: pinned maturin 1.8.3 resolves wheel
include globs from python-source ([source](https://github.com/PyO3/maturin/blob/v1.8.3/src/module_writer.rs#L1306-L1328)).
Staging now follows that base, preserving the old archive paths. The builder
checks every notice's exact archived bytes before issuing checksums; independent
negative cases reject omissions, changes, missing namespace files and two binaries.

INT-PY-01 stays open until the operation matrix covers actual native exports and
signatures with executed-case references. This accepted unit is not final backend
acceptance and does not imply hosted/package publication.

### Native operation coverage acceptance

The subsequent [operation candidate](python-operation-candidate.json),
[executed cases](python-operation-test-results.json) and
[requirement evidence](python-operation-evidence.json) close INT-PY-01 locally.
The one authoritative matrix now has 364 Rust rows and 373 native rows, each with
seventeen explicit cells. Native constructors, methods, properties, constants and
language-specific field/Serde projections retain actual signatures, Rust type and
conversion owners. No Rust function is invented for a field or enum projection.
All 88 directional/common class exports match installed metadata; every original
root class, alias, constructor/default and member signature is preserved (59 exports).

Two targeted native cases close gaps in the retained source tests: normalized-bank
generation and initialize semantics; owned accepted configurations; preview/source
counts; original spectrum ticks and centered Hz. These pass in addition to the
46 installed namespace-unit cases. Forty-six validation-tool and twenty Python-tool
cases pass, including rejection of omitted exports, stale defaults/descriptors,
wrong aliases/owners, unexecuted evidence and changed legacy signatures. The native
binary remains the tested namespace-unit revision `a09605a`.

The exact compiler/native/executed-case audit command is retained in the evidence.
Metadata establishes availability and declared signatures; scientific acceptance
comes from the named Rust/native owner cases and retained independent vectors.
No exhaustive internal branch coverage, external interoperability, Windows runtime
or other interpreter execution is claimed. Final workspace regressions, generated
directional references and overall backend acceptance remain later checklist gates.

## INT-CLI-01 existing executable integration

Preserve all four existing commands, version text, output schemas and exit handling.
Add `ofdm-fec upstream <run|preview|measured|ucd|rxmer|pre-eq> --input FILE --output FILE`
through the current parser/dispatcher. Run and preview accept the generic upstream
configuration; measured accepts the complete captured association plan; the three
capture queries export Rust-decoded documents without simulation. Reject downstream
options on upstream commands instead of silently ignoring them.

Reuse `json_io`'s 16 MiB bounded input/output and create-new transactional exports.
Raw capture adapters retain original bytes and exact grids/metadata. Generic and
measured admission remain idle; only run/measured explicitly execute. Preserve
incomplete/failed/cancelled reports before returning a nonzero exit status, without
equating terminal completion with full source recovery. Preview never clocks source.
The existing signal handler requests both its original downstream AtomicBool and
the common stop token, bridged into upstream control; no polling thread or second
signal handler is added. No CLI pause/progress API is advertised.

| Inspected owner | Decision / destination |
| --- | --- |
| `crates/cli/src/{main,arguments}.rs` and `commands/mod.rs` | EDIT additive directional parsing/dispatch and shared signal request; preserve downstream syntax and handlers. |
| `crates/cli/src/json_io.rs` | REUSE bounded original bytes and pre-serialized create-new output; no second I/O implementation. |
| `crates/merlin/src/lib.rs`, `pnm-input::upstream` | REUSE accepted public session and capture/association APIs; add only adapter dependencies to the CLI manifest. |
| Source `crates/cli/src/{args,command,json_io}.rs` at pinned revision | Retain as inspected reference; do not copy the standalone executable, fixture-RNG/LDPC experiment or duplicate file adapter. |

Added `crates/cli/src/commands/upstream.rs` and
`crates/cli/tests/upstream.rs`. Test both numerologies, idle preview, measured
provenance, exact raw captures, invalid schemas/options/limits, occupied destinations,
failed report retention, actual Ctrl-C where supported and direct Rust science
parity. Run every existing downstream CLI regression before acceptance. Update
CLI/user/example guides and fourteen-field evidence; final overall backend gates
remain separate. Requirements: REQ-COM-IDENTITY-01 and REQ-DS-API-01 through
[INT-CLI-01](../../checklist.md#int-cli-01), using the accepted upstream API owner.

### CLI acceptance

The [candidate](cli-candidate.json), [32 named executed cases](cli-test-results.json)
and [requirement evidence](cli-evidence.json) close INT-CLI-01. All 23 original
downstream CLI cases pass with the nine new upstream cases. Direct Rust/process
parity covers both numerologies and generic/measured admission. Five raw capture
variants retain the exact Rust documents/originals; truncated data rejects. Idle
preview, actual Linux SIGINT, occupied paths and 16 MiB input/output bounds pass.
The output-limit case preserves a large original whitespace prefix rather than
discarding provenance to fit the report.

A destructive zero-H channel completes receiver observations with source-bit
errors, as the accepted engine already specifies. The CLI retains that Completed
report and decoder outcomes while returning status 1 for failed source recovery.
The initial new test's stronger Failed-state expectation was corrected; no
scientific implementation changed. CLI all-target Clippy passes (13.63 s), as does
workspace formatting. Cargo's recorded per-binary test times remain distinct from
scientific determinism and full-command performance. No Windows signal lifecycle,
Qt upstream UI, hosted execution or package publication is claimed.
