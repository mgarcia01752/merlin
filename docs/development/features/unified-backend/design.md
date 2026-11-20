# Unified backend documentation consolidation

Task: **INT-DOC-01**, with final runtime-backed reconciliation tracked separately
as **INT-DOC-02** in the [official checklist](../../checklist.md#unified-backend-integration).
This task consolidates documentation; it does not migrate scientific implementations.

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
