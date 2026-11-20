---
name: external-change-review
description: Review MERLIN changes from another session, developer, generator or imported patch against the actual checkout while preserving index state and unrelated work.
---

# Review work outside the current baseline

Use this when adopting external work or encountering unfamiliar changes after a
handoff. Chat summaries locate evidence; they do not prove authorship, correctness
or current test status. For review-only requests, report findings without edits.

## Establish the candidate

Read local instructions and affected architecture/requirements/checklists. Record
repository, branch, HEAD and the review boundary. Inspect all work states:

```bash
git status --short --branch
git diff --name-status
git diff --cached --name-status
git ls-files --others --exclude-standard
git log -5 --oneline
```

Read the complete affected staged and unstaged diffs and untracked files explicitly;
`git diff` omits untracked contents. Inspect surrounding behavior, renames/deletions,
modes and symlink targets. Bound large/binary inspection and avoid printing secrets
or ignored personal inputs. Git cannot identify the author of an uncommitted edit;
report authorship as unknown unless separate evidence establishes it.

For committed work, choose and record a verified last-reviewed base, inspect its
diff/log to HEAD and constituent commits where sequencing matters. A merge base
can define a branch comparison but is not automatically the last reviewed revision.
If the boundary is uncertain, state the chosen scope and coverage limitation.

Preserve the index and unrelated work. Do not reset, stash, stage, overwrite or
delete unfamiliar changes as preparation for review. Refresh the affected review
if concurrent edits change its candidate. Ask only when overlapping work makes
the intended correction or safe ownership unclear.

## Review semantics with the right owner

Trace changed entrypoints, callers, data/state ownership and failure propagation.
Apply only relevant skills from the [catalog](../README.md):

- [development-workflow](../development-workflow/SKILL.md): requirements, reuse,
  exact owners and acceptance.
- [coding-style](../coding-style/SKILL.md): complete changed functions, bounded
  behavior, provenance and required notices.
- [scientific-testing](../scientific-testing/SKILL.md) and applicable pinned PHY
  skills: independent expectations and honest support/evidence boundaries.
- [public-api](../public-api/SKILL.md): Rust/Python, schema, old producers, recipes,
  cancellation, ownership and accepted-state compatibility.
- [ui-design](../ui-design/SKILL.md): actual interaction, themes and domain ownership.
- [git-workflow](../git-workflow/SKILL.md) and
  [toolkit-release](../toolkit-release/SKILL.md): tooling/gates when that area changes.

Check for hidden prerequisites, relaxed validation, duplicate algorithms and
unsupported claims, rather than only matching wording or style. Generic starter
policies do not replace local requirements. Archive snapshots remain historical;
do not execute them or apply their rules as current policy.

## Corrections, validation and handoff

Report concrete impact with file/section and evidence; separate defects from
optional preferences. Make authorized focused fixes and reconcile affected design,
requirements, status and guides. A review does not authorize commits, publication,
history rewriting, installation or edits to another checkout.

Inspect unfamiliar validation scripts before running them. Choose the repository's
scoped checks; record commands, actual results, environment, omissions and unresolved
findings. Earlier tests do not establish that a changed candidate still passes.
Keep source, local test, native installed and hosted evidence distinct.

Before handoff, compare HEAD, index and worktree with the recorded baseline. Explain
intended edits and concurrent changes; do not claim whole-repository correctness
from a scoped review. For example, staged code plus unstaged documentation requires
two diff reviews, while a newly referenced untracked helper requires its own content
and dependency inspection. A failing or unavailable integration check stays open.
