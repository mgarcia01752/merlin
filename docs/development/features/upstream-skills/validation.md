# Upstream skill migration validation

Candidate: `merge-merlin-morgana`, MERLIN baseline
`c9a143465aa3302da824b404f3ae7679fd7e9eda`, reviewed 2026-11-20.
Source: `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`, identified by the
[source inventory](source-inventory.json).
[Requirements and file map](design.md) define SKILL-US-01 acceptance.

## Structural and preservation checks

Passed on 2026-11-20 using read-only Git/file inspection and one-off Python checks:

| Check | Observed result |
| --- | --- |
| Source commit and working state | Exact pinned commit exists; source remained clean at that revision. |
| Source-file SHA-256 and pinned blob comparison | All 51 recorded files matched both the pinned Git blobs and current source files. |
| Complete source skill map | All 20 source skill entrypoints have one recorded local disposition/destination; 3 adapted scientific/API owners and 17 retained local owners. |
| Skill creator `scripts/quick_validate.py` | All 12 MERLIN canonical skills passed metadata validation. |
| Added local Markdown links and heading fragments | All 46 links resolved across 15 affected Markdown files, excluding the previously saved prompt. |
| Disposition destinations | All 20 local file/section targets resolved. |
| Existing relative skill discovery links | All 24 links resolve to canonical local folders; no duplicate skill links added. |
| Pinned PHY navigation/metadata comparison | Both editions' metadata, topic map and section index match the source; no corpus/path replacement needed. |
| Existing checklist IDs/statuses | All 275 pre-existing identified task statuses preserved; SKILL-US-01 is the only added task. |
| Prior work | Saved prompt hash unchanged; all pre-existing development-index lines preserved. |
| Git scope | Target HEAD/index unchanged; active branch is merge-merlin-morgana; only documentation/guidance/NOTICE/provenance paths changed. |
| Attribution and branding | Applicable source PDE notice qualification retained exactly; new canonical references contain no source product names, workstation paths or unavailable source commands. |
| `git diff --check` and changed-Markdown whitespace | Passed. The saved prompt is preserved byte-for-byte rather than normalized. |
| User-requested refactor handoff rule | Added to the local Git skill; message line limits checked and Python integration guide check passed as current. No commit was executed. |

The Python checks used the standard library for revision/hash, manifest coverage,
link/fragment, symlink, candidate-scope and preservation comparisons. Skill metadata
was checked with the installed skill-creator validator, not a source-repository
validation runner. These checks do not execute any illustrative science/API tests.

## Semantic review

- SKILL-US-R1: reviewed all source skill entrypoints and their local ownership;
  redundant generic skills retain MERLIN's current PDE-derived procedures, paths
  and tools rather than importing a broken sibling/template dependency chain.
- SKILL-US-R2: a matching-dimensions LDPC candidate cannot inherit downstream
  matrices/BCH; the upstream references require independent family/source tests.
  Received pilots observe C×H; measured Pre-Eq does not reveal H or receive noise.
  Missing source/PDF evidence remains a gap, not an invented verification claim.
- SKILL-US-R3: a proposed Python preview or JSON round trip is not accepted as a
  working paired API. Rust-first validation, ownership, interpreter-lock release,
  cancellation and actual native parity are explicit future acceptance checks;
  existing saved-workbench requirements remain intact.
- SKILL-US-R4: source edition metadata matches, while source-specific private
  corpus paths, validation scripts, cleanup rules and old product API names are
  deliberately not imported. Applicable attribution is preserved separately in
  NOTICE and this feature's source inventory.

Only the obsolete DSP skill description/adapter-deferred sentence was replaced;
all other existing guidance changes are additions. The architecture/checklist
review confirms that upstream runtime placement and implementation remain open.
SKILL-US-01 may close as a guidance task after these checks; it does not close a
backend integration phase or accept the source backend as verified MERLIN runtime.

## Limits

Only skills, guidance indexes, design/status records and applicable NOTICE content
change. No scientific code, fixtures, Cargo dependencies, Rust/Python API exports,
saved-workbench schemas, Qt UI, setup tools or Git/release helpers are migrated.
No runtime tests/builds, source-backend acceptance reruns, native platform checks,
hosted CI or external interoperability are claimed. Existing downstream support
and all unrelated checklist statuses remain unchanged.

Morgana is inspected at the specified local revision without fetch or writes.
Source PDFs, corpora, source-private captures and rendered pages are not imported.
Existing MERLIN reference files and caches remain in place. The prior saved master
prompt remains intact. No commit, push, publication or source repository edit is
part of this task.
