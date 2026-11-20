---
name: development-workflow
description: Plan MERLIN changes through architecture, requirements and dependency-ordered tasks; audit semantic reuse and reconcile acceptance evidence. Use for feature planning or changes to existing owners.
---

# MERLIN development workflow

Start from the user's goal and constraints, the active branch/worktree, and the
current [architecture](../../docs/architecture.md),
[requirements baseline](../../docs/requirements.md),
[official checklist](../../docs/development/checklist.md) and relevant feature
records. A new feature in MERLIN follows the **existing-codebase** path: inspect
current owners before choosing a design. A genuinely new standalone component
still needs an explicit boundary and dependency justification.

Planning produces reviewable design, requirements and tasks. It does not authorize
source migration, installation, commits or publication. Continue only within the
user's requested scope. Preserve prior accepted constraints when the task evolves.

## Architecture before implementation

Use **Architecture → Requirements → Developer checklist → Implementation →
Validation**, then reconcile the records. Maintain a proportional
`docs/development/features/<feature>/design.md`, following
[coding-style](../coding-style/SKILL.md#per-task-design-records).
Small changes may keep all these sections in one design document.

Name owners, inputs/outputs, units, missing/unsupported values, lifecycle,
failure/cancellation behavior, resource bounds and consumers where applicable.
For each affected stage, list concrete repository-relative implementation,
test and build/registration files with Add (planned), Edit or Reuse dispositions.
Verify existing paths; format proposed paths as code rather than broken links.
An unresolved placement is an open decision, not implemented support.

Link overview stage names to their canonical sections. Give diagrams a linked
companion index when their labels cannot navigate reliably. Keep IDs and anchors
stable and describe placeholders explicitly. When materially revising an older
stage, clarify its owner/file map without duplicating its design or rewriting
unrelated historical records.

## Reuse decisions

Trace entrypoint → adapter → owner → dependency, plus error/result propagation.
Inspect callers, tests, registration, units, normalization, bit/sample ordering,
state, concurrency, persistence and public consumers. Matching names or numeric
dimensions alone do not prove equivalent behavior.

Record each substantive candidate using this table in the feature design:

| Path and symbol | Current contract and evidence | Requirement gap | Decision / destination |
| --- | --- | --- | --- |
| Exact inspected source path and operation | Semantics, tests and limits | Required behavior still missing | REUSE, SHARE, ADAPT, SEPARATE or DEFER; exact owner path |

**REUSE** uses an existing owner directly; **SHARE** consolidates proven equivalent
operations; **ADAPT** shares a kernel while preserving direction/format contracts;
**SEPARATE** retains incompatible semantics; **DEFER** leaves an unproven or
unjustified consolidation pending, with its reason and next evidence needed.
Keep normative direction-specific tables/state machines distinct unless evidence
supports sharing. Extract for real consumers, not directory symmetry. Missing
in-tree callers do not establish that a public interface is unused.

Preserve imports through re-exports when required. Record migration impact and
independent numerical/behavior checks before replacing working implementations.
Actual material imports also follow the
[reused-material audit](../coding-style/SKILL.md#reused-material-audit).

## Linked requirements and tasks

For new or materially revised scope:

1. Assign stable requirement IDs with observable MUST/MUST NOT or SHALL/SHALL NOT
   acceptance and direct architecture-section links. Keep project acceptance
   separate from specification normative force and optional modem capabilities.
2. Give each active task a stable component ID, direct requirement links,
   explicit prerequisite IDs, acceptance evidence and applicable architecture/file
   owner links. If there is no design impact, say why. Order dependencies before
   consumers and reject unknown IDs or dependency cycles.
3. Requirements identify their delivery tasks in return. Every in-scope requirement
   has a task or an explicit deferred/excluded decision and future acceptance gate.
   Include cancellation, limits, compatibility and documentation where affected.
4. Preserve existing IDs and completion history. The
   [official checklist](../../docs/development/checklist.md) owns overall status;
   the [UI checklist](../../docs/development/ui-checklist.md) owns its detailed Qt
   tasks. Declare the owner for a scoped feature checklist and link it rather than
   maintaining competing status copies. Keep parent milestones open until all
   mandatory parts pass. Reopen invalidated acceptance with its reason.

For example, a cancellation change first names the accepted-result owner and
candidate/commit boundary in architecture. Requirement `CANCEL-R1` then requires
preserving accepted results after failure/cancellation. A task such as
`JOB-CANCEL-01` links that requirement and owner, names its actual prerequisites,
and accepts only when late-completion and partial-result cases pass through the
affected public consumers. Replace these illustrative IDs/links with actual
records; a diagram or compilation alone cannot satisfy that acceptance.

## Validate and reconcile

Apply [scientific-testing](../scientific-testing/SKILL.md) for signal processing
and input contracts, [public-api](../public-api/SKILL.md) for APIs/persistence and
[ui-design](../ui-design/SKILL.md) for Qt work. Preserve AGENTS' scoped build/test
commands, stable build profiles and caches. Documentation-only work needs relevant
metadata, links/fragments and resource checks, not a simulation rebuild.

Check one complete requirement → task → owner/test chain, cross-cutting and deferred
scope, duplicate IDs, missing dependencies and fragment targets. Preserve old
assertions/parameter coverage when moving tests. Record actual checks and limits,
then reconcile architecture, requirements, checklist and user guides. Partial or
unverified runtime acceptance stays open.

Read [evidence and selective adoption](references/evidence.md) when recording
facts, relying on changeable guidance, reviewing setup or updating shared practices.
Use [external-change-review](../external-change-review/SKILL.md) when the candidate
includes unfamiliar work from another session, developer or tool.
