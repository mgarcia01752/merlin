# Unified Qt risk-based testing strategy

This is the validation scheduling policy for `feature/unified-qt-workbench`.
It refines the existing [validation plan](validation.md),
[architecture](../../../architecture.md#unified-qt-acceptance),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-26](../../../requirements.md#req-uiu-26) and
[official UIU checklist](../../checklist.md#unified-qt-workbench-delivery).
It changes when checks run, without removing tests, independent vectors,
compatibility obligations or task/final acceptance criteria. Historical passes
remain evidence for their recorded candidates; completed tasks stay completed.

## Select the level and affected consumers

Review staged, unstaged and untracked changes before selecting checks. Follow
the changed owner through Rust APIs, native bindings, CLI, Qt, project inputs and
both direction sessions. Record the level, affected consumers and selection
reason in the unit's existing validation record before claiming acceptance.
The escalation rules below apply at every level; a small diff is not proof of
low scientific or compatibility risk.

| Level | Trigger | Required scope |
| --- | --- | --- |
| **1 — Focused commit** | Each focused implementation/refactor commit. | Tests covering changed behavior, independent numerical/boundary anchors where affected, applicable formatting, relevant crate/target Clippy with warnings denied, and applicable public-API/dependency-boundary checks. Run affected Qt Rust/QML cases for presentation/adapters. Validate changed documentation/traceability. Broaden through the risk table. |
| **2 — Checklist acceptance** | Before marking any official UIU task complete. | Complete tests for its relevant feature/crate, affected Qt/QML integration suites, downstream/legacy and affected Rust/Python compatibility checks, plus requirement/architecture/checklist traceability. Satisfy every existing task-specific acceptance criterion. Level 1 subunit passes alone cannot complete the parent. |
| **3 — Integration milestone/final acceptance** | Major integration milestones and UIU-QA-01. | Full root workspace including doctests/examples/analysis-only coverage, complete Qt Rust/QML checks, installed native Python parity, CLI regressions, project persistence/recovery and packaging regressions as applicable to the integrated candidate. Retain all existing final, hosted and release gates and explicit platform evidence boundaries. |

Major milestones include completed downstream facade migration before shell/
upstream convergence, both directions using the unified session lifecycle, and
integrated project/recovery delivery. Apply the relevant milestone checks before
claiming that milestone, including when it coincides with a task's completion.
UIU-QA-01 remains the final complete integration gate, including zero scientific
boundary exceptions and the line-by-line numeric/shared-constant audit.

## Mandatory risk expansion

Expand to **all affected consumers**, regardless of the selected level. Scope
those consumers' tests to the actual risk at Level 1; run their complete relevant
feature/crate coverage at Level 2 and full applicable regressions at Level 3.
Do not postpone evidence required to validate a changed contract until the end.

| Change | Consumers and checks to include |
| --- | --- |
| Shared scientific algorithm, numerical units/order/normalization, FEC, domain limits or work bounds | Independent owner vectors and failure/partial cases, both PHYs where the owner is actually shared, and affected Rust API, native Python, CLI and Qt paths. Direction-specific consumers remain distinct. Preserve original expected values, tolerances and truth/reference-plane separation. |
| Public Rust/Python signature, export, defaults, result/error contract, behavior or native binding | Rust legacy/facade import and contract tests; matching native operation/conversion/ownership/parity cases from an artifact containing the changed Rust/binding code; actual export/operation-matrix checks. Include affected Qt/CLI consumers and feature configurations. Commit an independently validated API extension before Qt adoption. |
| CXX-Qt QObject, QML module/type/resource registration or build dependency | Affected Qt Rust tests and QML import/creation/navigation, registration/resource/dependency checks, optimized executable build and native launch/Open probes. Expand to all registrations affected by a common module/build change. Core/Python suites apply only if their contracts also change. |
| Project schema, serialization/defaults, embedded payloads, Designer recipes or recovery | Shared Rust admission/round-trip/error tests, installed Python load/run parity, affected CLI/project handling and Qt Save/Open/recovery tests. Preserve legacy versions, original bytes/provenance and atomic failure/cancellation. Follow the saved-workbench contract in the public-API skill. |
| Cross-direction lifecycle, run dispatch, cancellation, pause, stale callbacks or aggregate retention | Both direction sessions and affected adapters, Busy/paused-slot ownership, stale/late completion, partial-result preservation, destruction and resource bounds. Include native Python/CLI lifecycle paths when their shared owners are affected. |

Documentation/comment-only changes run relevant link/traceability/format checks;
scientific, Clippy, Qt and Python runtime checks are not applicable when no
corresponding source/contract changes. Presentation-only Qt changes do not
require another wheel or unrelated core suite. Backend-only changes do not
automatically require complete Qt/QML runs, but affected consumers still need
their risk-specific checks.

## Reuse existing runners and compiled artifacts

- Keep Cargo target directories, Cargo homes, the pinned toolchain/Qt SDK and
  stable development/test/release profiles. Do not clean targets, change feature
  sets/profiles without need, or touch generated resources to force rebuilds.
- Use existing targeted Cargo commands and Qt/QML filters at Level 1. For Qt
  Cargo tests, select the actual binary and declared module paths; multiple
  libtest filters belong after `--`. Verify actual case discovery and results.
- Use `scripts/test-local.sh all` for full root Rust validation. Its existing
  bounded nextest scheduling, serialized timing case, doctests, examples and
  analysis-only phases remain intact. Full execution remains the runner default.
- Use `scripts/qt-desktop.sh check` for complete Qt validation and its existing
  build/check environment for affected work. Keep the latest Qt executable built
  before UI handoff. A source-only change need not repeat unchanged desktop
  startup probes unless its risk affects them; Level 2/3 gates still apply.
- Run complete Qt crate acceptance without overlapping compiler or native GUI
  validation jobs. Existing lifecycle watchdogs are independent of scientific
  execution limits; CPU contention can obscure their failures. Preserve a failed
  attempt, diagnose the exact stage/dimension, and rerun the affected case/group
  before complete acceptance. Do not raise watchdogs or weaken owner assertions
  merely to obtain a pass.
- Build/install a fresh Python wheel when changed linked Rust/binding code or
  package/import/schema contracts require native verification. Verify the exact
  tested artifact and import origin. Reuse its installed environment for further
  applicable cases while that artifact remains the same. Do not rebuild it for
  an unrelated Qt-only/doc commit, or use an older binary to validate new code.
- Reuse completed check evidence within the same acceptance candidate when its
  affected inputs, code/dependencies/features/toolchain and tested artifact are
  unchanged; cite the exact earlier run. Re-run invalidated checks after a fix.
  Compiled-artifact reuse is not a test-result cache or permission to claim an
  earlier suite ran against changed inputs.

Existing explicit `--test-quick`/`--skip-test` meanings and push/release/CI gates
are unchanged. A selected subset is not full task/milestone acceptance. Do not
alter skip lists, suppressions, test expectations or gates to satisfy a level.

## Evidence for every focused commit and acceptance gate

Use the existing unit record rather than a second competing test inventory:

| Field | Required record |
| --- | --- |
| Level and risk | Level 1/2/3, active checklist ID, changed owners, affected consumers and expansion reason. |
| Executed | Exact commands/case filters, actual discovered pass/fail/skip counts, exit results, candidate/source identities and relevant artifact hashes. Keep logs/generated artifacts ignored. |
| Not required | Unexecuted checks outside this change's affected scope and the concrete reason, such as unchanged native Rust/binding artifact for a Qt graph-formatting change. |
| Deferred gate | Applicable complete suites deliberately deferred from a focused commit to task or milestone acceptance, naming the task/milestone. These are pending, not passed or permanently waived. |
| Reused evidence | Original command/candidate/artifact and why its relevant inputs remain unchanged. Label historical evidence separately. |

A failed required check blocks that unit's acceptance until resolved; retain
failure history and successful rerun scope. Never report an unexecuted suite as
passed. Review the complete Git change set, make the authorized focused local
commit with `tools/git/git-save.sh --commit-msg`, verify its SHA and continue in
the existing dependency order. No architecture restart or new approval loop is
introduced. The official checklist alone owns overall task completion.
