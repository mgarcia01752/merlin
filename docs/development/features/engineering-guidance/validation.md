# Engineering guidance validation

Scope: [DEV-GUIDE-01 design and requirements](design.md), MERLIN branch
`merge-merlin-morgana`, baseline `096d5a476194d4f2e016863d723b225257bb6037`.
Source: the [fingerprinted PDE working-tree candidate](source-inventory.json),
reviewed 2026-11-20. This record covers documentation and discovery only.

## Checks

Passed on 2026-11-20:

| Check | Observed result |
| --- | --- |
| `git diff --check`, plus whitespace/final-newline checks on new Markdown | Passed. |
| One-off Python comparison of each tracked candidate against HEAD using line sequence comparison | All original lines in all 9 edited files preserved; no deletions or replacements. |
| Original checklist ID/status comparison against HEAD | All 274 existing identified task statuses preserved. DEV-GUIDE-01 is the only added task. |
| One-off added-link validation, checking destination files and Markdown heading fragments | All 72 added local file/fragment links resolved across 15 affected Markdown files. Existing unrelated links were outside this check. |
| Relative discovery-link checks against canonical skill folders | All 24 links resolved correctly, including the 4 new links. |
| Skill creator `scripts/quick_validate.py` against each canonical `skills/*` folder | All 12 skills passed metadata validation. This is structural validation, not runtime behavior evidence. |
| Source inventory SHA-256 and Git state comparison | All 29 selected PDE file hashes, source HEAD and status fingerprint matched the reviewed candidate. |
| Candidate scope and Git-state checks | Documentation/skill Markdown, source-identity JSON and relative discovery links only; index empty, baseline HEAD/master unchanged, active branch correct. |

The one-off checks ran with `python3` from the MERLIN root, using read-only Git,
filesystem, link and hash inspection. No validator, installer or test suite from
PDE was executed. No new executable tooling was added to MERLIN.

## Semantic review

- GUIDE-R1: checked additions against original instructions and retained existing
  package targets, direct-update behavior, Git helpers and all domain/API boundaries.
- GUIDE-R2: followed GUIDE-R2 → DEV-GUIDE-01 → the architecture/file plan → these
  checks, including the reverse link. The dependency chain is the four ordered
  design steps; no runtime implementation prerequisite is hidden. Planning and
  deferred scope remain distinct from accepted runtime support.
- GUIDE-R3: reviewed the staged-code/unstaged-docs/untracked-helper example against
  the external-review procedure; each state receives content review with no index
  reset or inferred authorship. The evidence reference preserves unknown/offline
  source status and distinguishes local, native and hosted claims.
- GUIDE-R4: checked the new skills' local resource links and canonical catalog;
  no required PDE helper/template path or absolute discovery link was introduced.
- GUIDE-R5: retained the dirty-candidate label and explicit lack of remote freshness
  verification; selected contents/state were rechecked before handoff.

These are documentation/semantic reviews, not executions of the illustrative
cancellation, parser, package, UI or installer scenarios described by the skills.
The official checklist may mark this guidance task complete; all unrelated runtime
tasks retain their prior status.

## Evidence limits

No Rust or Qt implementation, dependency, schema, helper, package or workflow
changed. Simulation tests, Qt builds, installation, native platform behavior,
hosted CI and hardware interoperability were not exercised by this task.
The PDE checkout was read locally; latest remote/publication status was not
checked. No commit, push or merge is part of this adoption.
