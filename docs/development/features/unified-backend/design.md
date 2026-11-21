# Unified backend documentation consolidation

Task: **INT-DOC-01**, with final runtime-backed reconciliation tracked separately
as **INT-DOC-02** in the [official checklist](../../checklist.md#unified-backend-integration).
This task consolidates documentation; it does not migrate scientific implementations.

The subsequent [downstream backfill](../downstream-requirements/design.md) adds
detailed existing requirements, the original numbered API clauses and historical
task/evidence navigation. DS-REQ-01 is a prerequisite of INT-BASE-01;
DS-GAP-01..03 additionally gate INT-REG-01. These are separate from source
migration acceptance and do not change historical downstream checkbox states.

## Architecture and file ownership

Extend the [canonical architecture](../../../architecture.md#unified-backend-integration)
with current/proposed dependencies, distinct downstream/upstream pipelines,
reference planes and concrete planned module/test ownership. Preserve all existing
downstream architecture sections and task IDs. Detailed original upstream stages
are retained as explicitly historical technical references, not a second current
architecture. Do not concatenate the source architecture into the canonical file.

Create [docs/requirements.md](../../../requirements.md) as the authoritative
behavior/acceptance baseline. Incorporate the existing API contract without losing
its identifiers, retain its old path as a compatibility index, and map all 59
source requirements to distinct upstream or genuinely common IDs. Keep delivery
status solely in the existing official checklist; UI detail stays subordinate.

| Path | Disposition | Responsibility |
| --- | --- | --- |
| `docs/architecture.md` | Extend | Canonical current and proposed unified ownership, stages and dependencies. |
| `docs/requirements.md` | Add | Shared/downstream/upstream requirements with owners and acceptance. |
| `docs/development/checklist.md` | Extend | Dependency-ordered integration gates; no inherited source pass status. |
| `docs/development/features/high-level-api/requirements.md` | Adapt after preservation | Compatibility index into the canonical requirements baseline. |
| `docs/development/features/unified-backend/source-review.md` | Add | Per-document Merge/Adapt/Retain/Exclude decisions, preserved source evidence and limitations. |
| `docs/development/features/unified-backend/source-inventory.json` | Add | Exact revisions, document/test/vector fingerprints and requirement/task mappings. |
| `docs/development/features/unified-backend/references/` | Add selected references | Historical scientific contracts, validation records, field coverage and deferred designs only. |
| `docs/development/features/unified-backend/validation.md` | Add | Observed documentation checks and outstanding runtime acceptance. |
| `docs/development/README.md`, `docs/development/features/README.md`, `docs/README.md`, `AGENTS.md`, relevant `skills/` entrypoints | Extend | Discover canonical requirements and final ownership rules. |
| `NOTICE` | Extend | Source scope, preserved notices and adaptation record. |

## Requirements and acceptance

- **DOC-R1:** Every source requirement and official source task MUST have a local
  destination or explicit deferred/excluded disposition. Acceptance: complete
  revision-pinned inventory, unique mappings, no missing requirement/task IDs.
- **DOC-R2:** Existing downstream requirement/task identifiers and prior accepted
  statuses MUST remain stable. Acceptance: source comparison, original API contract
  preservation and compatibility fragments; no original technical inputs deleted.
- **DOC-R3:** Requirements and integration tasks MUST link architecture, planned
  implementation/test ownership, dependencies and acceptance. Acceptance: links,
  fragments, coverage and dependency-cycle checks pass.
- **DOC-R4:** Source passes, source design completion and MERLIN acceptance MUST
  remain distinct. Acceptance: all new scientific integration gates remain open;
  final documentation/skills acceptance waits for real workspace tests and paths.
- **DOC-R5:** Detailed source stages, independent vectors and normative provenance
  MUST remain recoverable. Acceptance: retained references, exact pinned source
  links/hashes and numerical fixture inventory; no PDF/private corpus imports.

These documentation requirements derive from the ownership above and are delivered
by INT-DOC-01. INT-DOC-02 verifies their final implementation correspondence after
runtime migration. The user's ten final documentation completion conditions remain
a final gate, not a claim that this planning consolidation implements OFDMA.

## Validation order

Inspect both baselines and all upstream design/validation records; establish the
canonical architecture; consolidate requirements; derive the integration checklist;
preserve references/attribution; validate coverage, dependencies and navigation;
reconcile status and supply the prepared commit command. No builds, installation,
source checkout writes, commits or publication are implied.

## INT-DOC-01 traceability repair

The 2026-11-20 pre-migration review found `INT-PNM-01` had an empty requirement
field in both the checklist and inventory. Link REQ-US-PNM-01/02/03 to capture
admission/decoding as well as the existing INT-PNM-02 association/playback delivery.
Their behavior, primary delivery and acceptance remain unchanged. Audit every
integration task, adding missing inverse links inside existing requirement and
architecture owners. Preserve all stable identifiers and completed history.

`tools/validation/integration_traceability.py` checks nonempty requirement lists,
checklist/inventory agreement, actual requirement/architecture destinations and
both return paths. `tools/validation/test_integration_traceability.py` exercises
the empty-list defect, wrong fragments, missing return paths, inventory drift and
accidental borrowing from a following task, and audits the real checkout. Existing
validation-tool unittest discovery includes the guard; no CI entrypoint changes.
This repairs DOC-R3 under INT-DOC-01 and does not accept migrated functionality.

## INT-BASE-01 baseline execution

Run the existing full MERLIN runner and native Python/CLI baseline before source
imports. Record exact MERLIN/source revisions, manifests/locks, compiler identity,
commands, deterministic results and limitations in a focused baseline record.
Preserve cache/build profiles and existing tests. An unavailable tool or failed
attempt cannot satisfy the gate. The first attempt found stale local Rust links
and an incomplete user toolchain; verified cached 1.85.1 components may be reused
under ignored MERLIN-local tooling without changing global installations or pins.

Executed results are in the [baseline](baseline.md). The
[rights/reuse decisions](reuse-review.md) and hash-pinned
[scientific file inventory](scientific-source-review.json) satisfy INT-LICENSE-01
and INT-REUSE-01 before the first scientific import. Source validation remains
separate from actual MERLIN acceptance.

## INT-FEC-01: First additive backend implementation

Add upstream `mod.rs`, `definitions.rs`, `codec.rs`, `error.rs`, `tests.rs` and
the three independent test support files under `crates/fec-codec/src/upstream_ldpc/`.
Reuse the existing private sparse plan through an additional byte-buffer method;
keep the existing GF2 encoder and downstream codec exports unchanged. Add a
dense-oracle byte/GF2 equivalence test. Source matrix, fixture and golden values
must remain unchanged; only ownership/import paths and typed plan-error mapping
change. Preserve source holders/SPDX and record provenance in NOTICE/inventory.

Test-only RNG dependencies reuse target-pinned versions already in the workspace;
only dependency edges may change, not third-party versions. Accept only after all
three families' source/oracle/shortening/correction/failure cases and existing
downstream FEC tests pass in MERLIN, along with relevant shared-code regressions,
formatting and Clippy. Full upstream transport and Python remain later gates;
this step introduces no early Python codec facade or alternate CLI.

## DOC-ORG-01: Functional feature-documentation ownership

The user approved the [frozen proposal](feature-documentation-plan.md) and
[170-file baseline](feature-documentation-inventory.json) after commit
`ca2de34fa70e8e2b40c2622a2123c35d4471a070`. Keep those proposal-time records
unchanged; their awaiting-review wording is historical. The
[current mapping](source-inventory.json) and
[structural validation](validation.md#doc-org-01-functional-documentation-acceptance)
record execution. The [official task](../../checklist.md#doc-org-01) owns completion.

Requirements are [DOC-R3](../../../requirements.md#ds-doc-r3) and
[REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01); the
[canonical documentation owner](../../../architecture.md#requirements-and-historical-evidence-ownership)
remains authoritative. DS-REQ-01 and INT-DOC-01 precede this task; INT-DOC-02 also
depends on it without accepting unfinished backend work.

Add nine active functional records under `fec/`, `signal-processing/`, `ofdma/`,
`pnm-measurement-inputs/` and `high-level-api/`. Preserve all original paths,
151 Keep files, 12 historical Archive files, source requirements/fixtures and all
historical checkbox lines/aliases. Adapt only the seven approved original records.
Update canonical links and the grouped index; correct only the current
REQ-COM-SYS-05 test owner. No scientific/API/UI implementation or dependency changes.

Documentation validation is owned by `tools/validation/feature_documentation.py`
and its focused cases in `tools/validation/test_feature_documentation.py`.
Inputs are current tracked/new documentation and the frozen approved inventory;
outputs are diagnostics/counts only. Bound scanning to repository files and skip
private/build/cached corpora; never write evidence, source fixtures or task status
from the validator. Tests cover path/fragments, preservation, planned/current
locators, aliases and dependency rejection. The completion report records actual
checks and omissions; no simulation/Qt tests are needed for documentation changes.
