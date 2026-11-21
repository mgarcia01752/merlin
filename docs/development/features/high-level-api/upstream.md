# Converged public Rust and native Python APIs

The [selected exact namespace mapping](namespace-design.md) uses a minimal `merlin` facade, independent PHY owners, a neutral common contract leaf and one real Python package. [INT-API-MAP-01](../../checklist.md#int-api-map-01) records the design decision; public exports still await their runtime gates.

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-rust-python-and-cli-apis).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Scope and support

This is the active feature design for converging direction-specific operations through
the existing public Rust engine and native Python distribution. Existing downstream
Rust/Python APIs, imports, schemas, examples and saved-workbench contracts remain
implemented within their recorded supported scope. Initial upstream typed Rust
composition now passes local synthetic session acceptance. Final facade/common
contracts, measured operations and native wrappers remain at their delivery gates.

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
| [REQ-US-API-01](../../../requirements.md#req-us-api-01) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/ofdma/tests/api_parity.rs` (planned) |
| [REQ-US-API-02](../../../requirements.md#req-us-api-02) | [INT-PY-01](../../checklist.md#int-py-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/ofdma/tests/api_parity.rs` (planned) |
| [REQ-COM-API-03](../../../requirements.md#req-com-api-03) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/ofdma/tests/api_parity.rs` (planned) |
| [REQ-COM-API-04](../../../requirements.md#req-com-api-04) | [INT-PY-01](../../checklist.md#int-py-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/ofdma/tests/api_parity.rs` (planned) |
| [REQ-COM-API-05](../../../requirements.md#req-com-api-05) | [INT-PY-01](../../checklist.md#int-py-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/ofdma/tests/api_parity.rs` (planned) |
| [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-RUST-01](../../checklist.md#int-rust-01) | Module: `crates/ofdma/src/api/session.rs`; Test: `crates/ofdma/tests/api_parity.rs` (planned) |
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
