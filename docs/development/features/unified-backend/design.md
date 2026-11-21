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

## INT-TOOLS-01: Executed implementation traceability

Extend the existing traceability tool with focused implementation-evidence checks,
using the official checklist as the sole status owner. Add an evidence catalog to
the existing source inventory: record files, successful named-run records and
explicit pre-integration historical exceptions. Do not restage or reopen accepted
planning/FEC/history to force a new evidence format.

Each newly accepted runtime task must cover its declared requirements with all
fourteen evidence fields, exact architecture/design/task links, source symbols,
qualified acceptance cases, successful execution and a valid candidate fingerprint.
Verify historical candidate contents against their actual descendant Git revision
when later accepted work changes a file; do not compare old checkpoints blindly
to current bytes. Reject unknown or duplicate identities, unexecuted/failed cases,
missing owner symbols, mismatched fingerprints and premature completion. Current
compiler Rust exports and installed native metadata remain separate actual inputs
to the complete API matrix. Compile-only discovery is not execution evidence.

Reuse `integration_traceability.py`, `feature_documentation.py`, compiler
`rust_api_inventory.py`, installed `native_api_inventory.py` and
`api_operation_matrix.py`. Added `tools/validation/implementation_evidence.py`
and focused independent negative fixtures in
`tools/validation/test_implementation_evidence.py`; integrate the new checks into
the existing audit rather than introducing another completion tracker. Compiled
nextest case discovery may resolve qualified crate-root unit cases alongside
syntax-aware Python discovery; successful named records still establish execution.

Audit every local manifest/lock/version/pin/NOTICE and the actual package/example
inventories. Preserve existing dependency pins, the two workflow entrypoints,
native platform scope, stable profiles and useful build caches. Run the tool,
packaging, release, installer and example checks appropriate to any affected owner.
Record exact commands, candidate, outcomes and local/hosted limits. This task
does not accept final regressions, generated documentation or the whole backend;
INT-REG-01, INT-DOC-02 and INT-ACCEPT-01 remain dependency-ordered gates.

The implemented `tools/testing/native_api.py` creates a fresh private environment,
generates eight upstream and existing downstream/Designer references through
separate Rust processes, runs every installed native case outside the source
package and records actual exports. It never calculates science in Python or
installs system packages. `tools/testing/api_examples.py` now registers twelve
directional workflows; wheel smoke retains both old downstream fixtures and adds
the upstream fixture/direct module imports. Package tests retain new examples and
guides. The preflight workflow fetches full Git history to verify historical
candidate objects; its two-entrypoint/native-target/security design is unchanged.

Default traceability checks recorded chains and current declarations. Explicit
`--acceptance` additionally requires compiler exports, installed native exports,
nextest discovery and matching Cargo metadata; omitting an input fails. Neither
mode recreates an old test run. The code/test owners are
`tools/validation/{implementation_evidence,test_implementation_evidence}.py`,
`tools/testing/{native_api,test_native_api}.py` and the existing adapter/tool suites.

## INT-REG-01: Unified backend regression

Use accepted implementation checkpoint `95e2ac5` and preserve its scientific
sources, source vectors and dependency pins. Run the full existing
`scripts/test-local.sh all` gate with four isolated nextest processes, retained
doctests, all twelve API workflows, analysis-only coverage and the serialized
pause case. Run workspace formatting and all-target Clippy with warnings denied;
retain native adapter Clippy and the already fresh installed-wheel/direct-Rust
48-case run. Do not substitute source-only results, compile-only discovery or
quick subsets for this acceptance.

Validate the unchanged downstream Qt adapter through its existing check script
because shared scientific/API dependencies changed. Reuse the available pinned
Qt 6.8.3 SDK read-only; all outputs/caches remain in MERLIN. This is compatibility
validation, with no upstream Qt implementation, SDK/system installation or changes
to another repository. Keep runtime phases sequential to avoid unrelated load in
timing-sensitive cases and preserve useful build caches/profiles.

Audit all three locks without advisory suppressions and retain actual warnings
separately from vulnerabilities. Record commands, exact source candidate, named
cases, elapsed times, failures/corrections and host/platform limits. Existing
downstream history stays completed; new integration evidence belongs to INT-REG-01.
The architecture/requirement/checklist links and final documentation/whole-backend
acceptance remain separate subsequent gates. No publication, merge or release.

## INT-DOC-02: Final runtime and documentation correspondence

Reconcile current summaries and canonical skills with the accepted Rust/native
Python/CLI/PNM owners and INT-REG-01 evidence. Preserve historical sections, stable
IDs, source clauses, document dispositions and the three authoritative baselines.
No scientific implementation, public behavior, saved recipe or dependency changes.

Edit `tools/python/build_api_reference.py` and its focused negative tests to
generate pages for every installed root/directional/common import. Resolve aliases
to their actual native canonical modules; same-named upstream/downstream types
remain distinct. Require class anchors, canonical members and search output.
Build strict Sphinx 8.1.3 pages from the accepted installed wheel, and warnings-denied
rustdoc for the facade, independent PHYs, common contracts and PNM owner. Record
the actual native hash and namespace inventory. Generated HTML stays ignored.

Update only current overview/status wording, API-reference instructions, paired
examples and skill owner paths. Link the final ten-condition review and exact
requirement/acceptance records. Existing link/ID/dependency/source-preservation
validators and real export/compiled-case acceptance remain required; a successful
reference render does not substitute for the scientific regression gate.

## INT-ACCEPT-01: Final supported backend review

Review accepted INT-REG-01 checkpoint `559c49e` and INT-DOC-02 checkpoint
`b3ffc80`. Verify the full 832-file scientific/native/Qt regression manifest still
matches current bytes; documentation/reference tooling is the only later change.
Require every mandatory integration dependency accepted, all requirement chains
valid and the explicit INT-DEFER-01 exclusion retained. Reconcile numerical
vectors, independently executed Rust/native outcomes, reference-plane limits,
hot-path/allocation evidence, provenance, notices and unchanged third-party pins.

Run current metadata/link/source/history acceptance against fresh compiler/native
exports and Cargo test discovery. Retain full-suite and installed-wheel evidence
with their own immutable candidates, rather than rerunning unchanged science for
a documentation-only review. Repeat relevant tool/version/guide checks. Final
records supplement the official checklist and retain eight verified historical
planning/FEC/DSP exceptions. No source cleanup, Qt expansion, release, merge or
publication is part of backend acceptance.
