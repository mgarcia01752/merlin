# Unified documentation validation and completion boundary

The earlier sections preserve their named historical checkpoints. Current progress
is owned by the [official checklist](../../checklist.md); the latest tooling
acceptance section below records the additive runtime-evidence audit.

Current checkpoint: the sections below preserve the original INT-DOC-01 review.
The [pre-import baseline](baseline.md), [rights/reuse decisions](reuse-review.md)
and [standalone upstream LDPC results](fec-validation.md) now accept INT-BASE-01,
INT-LICENSE-01, INT-REUSE-01 and INT-FEC-01. DS-GAP-01/02 have scoped current
evidence; DS-GAP-03 has recovered historical DPD evidence. Final runtime and
documentation acceptance remain open.

The follow-up traceability audit repaired INT-PNM-01 with REQ-US-PNM-01/02/03
and added focused return links at existing requirement/architecture owners.
All 21 integration tasks now have nonempty matching references in both the
official checklist and inventory. `tools/validation/integration_traceability.py`
checks task → requirement → task and task → architecture → task links; its eight
cases include empty references, wrong targets, missing returns and boundary leakage.
Full validation-tool discovery passes nine tests. Existing IDs and historical
checkbox lines remain intact except newly evidenced open-to-complete gates.
All 515 canonical requirements and two compatibility aliases remain navigable;
all 59 source clauses, 93 original API clauses, 46 document dispositions and 52
source evidence identities are preserved. The current dependency graph has 27
known acyclic nodes. Maintained local links resolve after this implementation.

Scope: INT-DOC-01 at MERLIN baseline
`fa26de619b5d28a442c42e46ad1ed19262a9d08a`, source
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
This is documentation consolidation, not scientific implementation migration.

## Validation status

Passed on 2026-11-20 for this documentation checkpoint:

| Check | Observed result |
| --- | --- |
| Full source documentation inventory | 46 source documents, including all 30 feature design/validation/coverage records, have explicit Merge/Adapt/Retain/Exclude destinations and source hashes. |
| Source requirement and task coverage | All 59 source requirements map uniquely; all 103 source checklist IDs have local destinations without pass-status transfer. |
| Integration traceability/dependencies | 21 uniquely identified integration tasks link requirements, architecture/owner/test sections and acceptance; prerequisites exist and the graph is acyclic. |
| Scientific stage preservation | All 23 TX/channel/RX stages have canonical owner/test sections and linked representation boundaries; 26 complete stage/PRBS/Pre-Eq/RxMER source sections are retained as historical technical detail. |
| Independent evidence preservation | 52 source test/vector/fixture identities and hashes preserved; 11 selected validation/coverage/deferred-design records retained locally. No production-generated replacement oracle introduced. |
| Existing downstream state | All 276 original identified checklist statuses preserved. Original architecture/checklist lines remain intact; 143 untouched existing feature/reference records match baseline bytes. |
| Existing downstream requirements | Complete prior API acceptance text and numbered ranges preserved in the canonical baseline; all 11 explicit legacy REQ identifiers retained, with compatibility headings at the old path. |
| Repository internal links | 2,591 local links/fragments checked across maintained docs/skills/root entrypoints; zero broken targets. Private/local/generated evidence and extracted specification corpora are excluded from the maintained-link scan. |
| Pinned source references | 126 source file/fragment targets verified against the exact local Git revision, independently of remote reachability. |
| Skill metadata/discovery | All 12 canonical skills pass the skill-creator metadata validator; relative discovery links remain valid. |
| Whitespace/generated guide | `git diff --check`, new-document whitespace/final-newline checks and `python3 -B tools/python/update_integration_doc.py --check` pass; integration guide is current. |
| Scope and preservation | Source checkout remains clean at the pinned revision; target HEAD/index unchanged. No scientific source, manifest, dependency, UI implementation or tooling changed. |

The checks used read-only Git inspection, one-off Python source/hash/coverage/
dependency/link/anchor comparisons and the installed skill metadata validator.
The single existing broken analyzer-gallery fragment was repaired in the
workbench guide. External publication/network availability, original PDFs,
scientific/native execution and hosted CI were not exercised.

## Semantic reconciliation

The canonical architecture is a deliberately extended MERLIN design, not the
concatenated source architecture. It preserves existing downstream owners and
defines proposed shared FEC/DSP, upstream PHY, measurement and API ownership with
the current/planned distinction explicit. The historical stage appendix keeps
scientific equations, norms, bounds and limitations without transferring old
source implementation claims.

The requirement map distinguishes 15 common source contracts from 44 upstream
contracts. Common mechanisms still need equivalence/consumer validation. The
source-specific prohibition on depending on a MERLIN checkout is adapted to
external checkout/runtime dependence; source Cargo cleanup is replaced by MERLIN's
cache/build-artifact preservation. Other source behavior/acceptance remains
recoverable through the pinned originals and retained scientific references.

Typed Rust foundation and the initial public Rust run path accompany the upstream
stages before loop acceptance; final API convergence and Python/CLI follow that
real loop. Measurement inspection, full-plan association and actual playback
remain separate gates. The receiver never receives injected source/H/C/noise truth.
Downstream BCH/PLC/NCP, schema/recipe, CLI/Python and delivery policies remain intact.

INT-DOC-01 accepts this design/requirements/checklist consolidation only. All
20 other new gates remain open, including deferred future scope and final
documentation acceptance INT-DOC-02. No runtime gate is checked because its source
equivalent passed. Rust/Qt/Python simulation builds/tests were not rerun for this
documentation-only change, and no new runtime behavior is claimed accepted.

## Final completion criteria

| User criterion | Current checkpoint | Final gate |
| --- | --- | --- |
| One authoritative architecture | `docs/architecture.md`; source detail explicitly historical. | INT-DOC-02 checks final implementations. |
| One requirements baseline | `docs/requirements.md`; old API path is a compatibility index. | INT-DOC-02. |
| One developer checklist | Existing official checklist extended; UI detail subordinate. | INT-DOC-02. |
| Every applicable upstream requirement represented | All 59 source requirements mapped; feature coverage and deferred scope retained. | INT-REG-01 / INT-DOC-02 verify execution or explicit deferral. |
| Every integration task has acceptance | Named requirements, owner/test sections, dependencies and acceptance. | Task-local tests plus INT-ACCEPT-01. |
| Resolving internal references | Structural checks at this checkpoint. | Recheck after every move; INT-DOC-02. |
| Skills name final modules | Current and proposed paths distinguished. | **Pending:** final actual paths and callable API surface. |
| No obsolete user-facing product branding | No source product UI/package/import identity introduced; historical/legal provenance retained. | **Pending:** integrated export/package/user-guide review. |
| Documentation describes integrated implementation accurately | Existing downstream retained; upstream clearly planned/source-only. | **Pending:** final runtime/docs correspondence. |
| Accepted functionality has integrated evidence | No new upstream runtime accepted here. | **Pending:** full unified workspace tests and INT-ACCEPT-01. |

Therefore final documentation migration is not complete at this design checkpoint.
No original source technical document/vector is deleted; no package installation,
source checkout modification, build-cache cleanup, commit, push or publication
is authorized or performed by these documentation changes.

## DOC-ORG-01: Functional documentation acceptance

Executed on 2026-11-20, branch `merge-merlin-morgana`, working candidate based on
`ca2de34fa70e8e2b40c2622a2123c35d4471a070`. The user approved the frozen
[plan](feature-documentation-plan.md) and [170-file inventory](feature-documentation-inventory.json).
Those proposal-time records remain unchanged; their earlier awaiting-review wording
is historical. The [current mapping](source-inventory.json) records the executed
zero-move organization. Earlier dated checkpoints above retain their original scope.

The [official DOC-ORG-01 task](../../checklist.md#doc-org-01),
[canonical documentation owner](../../../architecture.md#requirements-and-historical-evidence-ownership),
[DOC-R3](../../../requirements.md#ds-doc-r3) and
[REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01) own this acceptance.
It accepts documentation structure/traceability only; INT-DOC-02 and all unfinished
backend/Qt tasks retain their existing states.

### Executed file mapping

| Disposition | Actual result |
| --- | --- |
| Keep | 151 original feature files remain byte-identical at the same paths. |
| Archive in place | 12 original historical source references remain byte-identical at the same paths. |
| Adapt | Seven approved original feature records: index; high-level API design; PNM README; integration design, source review, source inventory and validation. Original anchors/source findings remain. |
| Add | Nine functional feature records listed below. |
| Move / merge / delete | Zero / zero / zero. All 170 original feature paths remain present. |
| Frozen planning records | Two records already committed with the approval baseline remain byte-identical. |
| Final count | 181 feature files: the 170 originals, two frozen planning records and nine additions. |

New active owners:

- [FEC mechanisms/code families](../fec/design.md).
- [DSP conventions/reuse](../signal-processing/design.md).
- [OFDMA configuration/grants/source](../ofdma/configuration.md).
- [OFDMA transmitter](../ofdma/transmitter.md).
- [OFDMA receiver](../ofdma/receiver.md).
- [OFDMA channel](../ofdma/channel.md).
- [Pre-Eq correction banks](../ofdma/pre-equalization.md).
- [Upstream PNM](../pnm-measurement-inputs/upstream.md).
- [Converged Rust/Python APIs](../high-level-api/upstream.md).

The existing feature index now groups all records by engineering responsibility.
Existing downstream directories retain their owners. Integration records keep the
migration/evidence history; source-only stage contracts are linked from active
functional owners instead of duplicated. Canonical architecture, requirements and
checklist gain direct forward/return links; navigation does not depend solely on
the index. Source inventory metadata retains original clauses/hashes/locators and
adds the current mapping. REQ-COM-SYS-05 now names the verified
`crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs`; old locators in frozen
proposal/snapshot text remain historical, not current test owners.

### Commands and measured structural results

`python3 -B tools/validation/feature_documentation.py --migration` passes.
Its normal mode also passes through existing tool-test discovery. Strict migration
mode compares this candidate against the approved snapshot; normal mode supports
ongoing documentation checks without freezing future legitimate development.

| Check | Actual result |
| --- | --- |
| Local Markdown links/files/fragments | **7,632** links checked across maintained tracked/new Markdown, including relative paths, explicit/duplicate-heading fragments and images; zero broken targets. Fenced examples and external URL availability are excluded. |
| Original feature preservation | All 170 accounted for; all 163 Keep/Archive hashes match, seven Adapt diffs reviewed; no original feature anchor lost. Both frozen planning files match the approval commit. |
| Requirement identity and clauses | 515 unique canonical IDs and two existing compatibility aliases retained; all 59 inherited behavior/acceptance clauses and 93 original numbered API clauses preserved. |
| Existing downstream traceability | 127 current module/test/record routes resolve; all 430 original downstream requirement/task occurrences retain their forward/return links. |
| Functional traceability | Nine records; 57 requirement→feature mappings covering 50 distinct current IDs; 22 declared task→feature mappings covering 14 distinct existing tasks. Each record has architecture/requirement/task return links and existing or explicitly planned module/test owners. |
| Integration requirements | All 21 original integration tasks retain valid nonempty requirement lists and architecture/requirement return links, including INT-PNM-01. |
| Task history | All 521 original official/UI checkbox lines remain byte-identical. Only new DOC-ORG-01 is accepted; existing completed/pending integration states do not change. |
| Legacy aliases | Both completed UI-DOC-02 labels, exact canonical mappings and unique task-req-ds-doc-terms-01 / task-req-ds-doc-png-01 anchors are preserved. New task IDs remain unique. |
| Dependencies | 28 known nodes, matching current checklist/inventory prerequisites and no cycles; DOC-ORG-01 now also gates INT-DOC-02. |
| Machine records | All six original feature JSON records parse; original source/task/document/evidence snapshots and field values are unchanged except the approved current FEC locator and INT-DOC-02 dependency. Current mapping agrees with all original/new paths. |
| Navigation and skills | All 181 feature records reachable; no unintended orphan. Twelve canonical skill metadata records and 24 relative discovery links validate; local skill references resolve. |
| Tool regression cases | `python3 -B -m unittest discover -s tools/validation -p 'test_*.py'`: 25 passed, including 16 new positive/negative documentation contracts and the existing nine cases. |
| Guide and whitespace | `python3 -B tools/python/update_integration_doc.py --check` and `git diff --check` pass. |
| Implementation scope | Only approved documentation plus the two documentation-validation Python files changed. Rust, Cargo manifests/locks, native binding/Qt/QML implementations, scientific regression fixtures and public API behavior are unchanged. |

The new cases exercise missing links/fragments, literal fenced examples, duplicate
anchors, legacy alias navigation, explicit planned/current owners, the exact stale
FEC locator, historical checkbox preservation, unknown/cyclic dependencies and
missing feature return links and exact prior index navigation. The first working audit found one missing Pre-Eq
return link at the PNM handoff; it was corrected before acceptance. Final review
restored seven direct user/API/status/index routes and added a preservation case. Test-file
existence is not presented as scientific execution evidence.

### Boundaries and remaining work

No Rust simulation, native Python engine, Qt build, private specification PDF,
modem capture, installed lifecycle, external URL or hosted-CI validation was rerun
for this documentation-only change. Existing scientific evidence retains its
original checkout/platform and limitations. No source checkmark closes an upstream
runtime gate. The public paired API record explicitly includes existing upstream
Rust/Python requirement IDs as well as shared contracts; no IDs or behavior changed.

DOC-ORG-01 is complete after these structural checks. INT-DSP-01 remains the next
backend implementation gate; downstream history and all other task states are
preserved. Final runtime/module/API correspondence, final documentation acceptance
and unified backend acceptance remain open at INT-REG-01 / INT-DOC-02 /
INT-ACCEPT-01. No commit, push, merge, publication or installation was performed.

## INT-TOOLS-01: Implementation and delivery tooling

The [candidate](tooling-candidate.json), [executed cases](tooling-test-results.json)
and [requirement evidence](tooling-evidence.json) retain exact local validation inputs
and scope. The official checklist alone owns task completion.

The [implementation design](design.md#int-tools-01-executed-implementation-traceability)
extends the existing traceability audit with fourteen-field requirement records,
exact source symbols/cases, successful named executions and real candidate Git
identities. Its explicit acceptance mode also joins fresh compiler/native exports
and nextest discovery to exact Cargo targets. Eight previously completed
planning/FEC/DSP gates retain their verified original formats and acceptance states.

Reproduce installed native parity using a newly built ABI-matching wheel and a new
ignored output directory (no system installation or output replacement):

```bash
python3 -B tools/testing/native_api.py --wheel-dir local/python-wheels \
  --output docs/development/local/unified-backend/native-check
```

This runner creates eight independent upstream Rust reference results, existing
downstream selection/workbench/Designer references, a private native environment,
all installed tests and actual native export metadata. Development Python only
orchestrates the checks. The regular wheel smoke also runs both original downstream
examples plus the bounded upstream example and verifies actual directional imports.
The standard Rust example runner now owns twelve workflows.

Historical fingerprint verification requires the relevant Git objects. Preflight
therefore fetches full history; preserve reviewed acceptance checkpoints when
integrating branches. Source archives remain usable for simulation but do not
provide this historical Git-identity audit by themselves. The two CI entrypoints,
all security checks and declared native targets remain unchanged. Local checks do
not establish hosted execution, Windows runtime or native installer lifecycles.

Local acceptance passes: 190 development-tool cases, all 48 installed native cases
with fresh Rust references, 364 Rust/373 native operation rows and 608 compiled
case identities (discovery, not execution of the full suite). The fresh compiler
inventory resolves an omitted rustdoc source link through the exact explicit
public import, preserving distinct same-named PHY errors; its independent negative
case rejects name-only or comment-only guesses. Historical source/candidate
identities and every newly accepted task's requirement chain pass the full audit.

All twelve registered example workflows match actual Cargo targets. The actual
local CLI archive has 932 members, including upstream examples/guides/notices and
no private caches/build trees. It uses the already tested debug binary and is
package-inventory evidence. The wheel passes both old 82-codeword downstream
smokes and the complete 1,976-bit upstream fixture. The three locks retain exactly
119/121/202 third-party name/version/source/checksum tuples from the pre-migration
checkpoint; version metadata and the generated Python guide agree. No dependency
pin or native delivery target changed. Final whole-workspace regressions remain
INT-REG-01; no release, system installation or publication occurred.

## INT-REG-01: Complete unified workspace regression

The [regression source manifest](regression-candidate.json),
[six downstream requirement chains](regression-evidence.json) and
[exact executed cases and security results](regression-test-results.json) accept
the final corrected scientific source. `scripts/test-local.sh all` passes all
605 workspace cases, four preparation cases, 139 analysis-only cases and twelve
API workflows. The seven workspace and one analysis rustdoc-test phases run
successfully and contain zero doctest cases. The only three omitted tests are
explicitly ignored performance benchmarks, named in the run record.

The initial full run at `95e2ac5` passed but exposed an unused private cancellation
bridge in a build without `high-level-api`. Its caller already requires that
feature; the helper now has the same cfg guard. The full gate was rerun on that
corrected source (9m49s including relinking); default-feature Clippy, workspace
Clippy, binding Clippy and formatting all pass. No public signature, function
body, input contract or downstream scientific result changed.

The unchanged downstream Qt adapter passes warnings-denied Clippy, its
87 Rust cases and 89 software/offscreen QML cases, and the current executable
is built. The existing pinned Qt 6.8.3 SDK was used read-only; all outputs remain
in MERLIN. This is source-build compatibility, without upstream UI implementation
or installed GUI/native updater lifecycle acceptance.

The fresh 48-case installed CPython 3.10 suite and independently executed Rust
references from INT-TOOLS-01 remain applicable: native adapter/scientific bodies
are unchanged. Full workspace CLI/PNM/session cases also pass in this new run.
All three locks pass the unsuppressed security audit with zero vulnerabilities;
`humantime 2.4.0` has maintenance advisory `RUSTSEC-2026-0542`, retained explicitly
in the run record. Dependency pins and original downstream acceptance remain intact.
Local Linux results do not establish Windows/hosted/native-installation or external
modem/CMTS interoperability. Final documentation and backend review remain separate.
