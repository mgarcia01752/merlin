# Unified Qt Workbench completion report

All 21 authorized UIU tasks are accepted in the integrated checkout.
The [official checklist](../../checklist.md#unified-qt-workbench-delivery) owns
completion; each row links the executed acceptance evidence for its commit.

| Task | Acceptance commit | Executed validation |
| --- | --- | --- |
| [UIU-PLAN-01](../../checklist.md#task-uiu-plan-01) | `e033152f815f` | [Agree the UI planning baseline, full inventory, ownership/API gaps and migration design.](validation.md#uiu-plan-01-accepted-planning-baseline) |
| [UIU-BASE-01](../../checklist.md#task-uiu-base-01) | `c87006fae0a2` | [Establish a fresh downstream Qt baseline before moving source.](validation.md#uiu-base-01-fresh-qt-baseline-accepted) |
| [UIU-BOUND-01](../../checklist.md#task-uiu-bound-01) | `f8f2a55fea76` | [Enforce the public-API boundary before migration.](validation.md#uiu-bound-01-public-api-boundary-accepted) |
| [UIU-API-01](../../checklist.md#task-uiu-api-01) | `dbe4ffe9a279` | [Expose compatible input/editor/preview admission through the facade.](validation.md#uiu-api-01-public-admission-accepted) |
| [UIU-API-02](../../checklist.md#task-uiu-api-02) | `fca3aa5bf6e2` | [Complete downstream controlled jobs and retained public observations.](validation.md#uiu-api-02-public-full-loop-and-observations-accepted) |
| [UIU-API-03](../../checklist.md#task-uiu-api-03) | `59f27aa0de89` | [Add bounded idle upstream resource-grid observation.](validation.md#uiu-api-03-idle-upstream-resource-grid-accepted) |
| [UIU-API-04](../../checklist.md#task-uiu-api-04) | `bbaed98b2ddb` | [Close only actual upstream instrument API gaps.](validation.md#uiu-api-04-upstream-instrument-backend-accepted) |
| [UIU-API-05](../../checklist.md#task-uiu-api-05) | `ca588e2e390a` | [Define and implement direction-aware public project admission.](validation.md#uiu-api-05-project-backend-accepted) |
| [UIU-ORG-01](../../checklist.md#task-uiu-org-01) | `15b16a4aad7b` | [Move proven generic shell/theme/dialog infrastructure by inventory.](validation.md#uiu-org-01-generic-presentation-accepted) |
| [UIU-DS-01](../../checklist.md#task-uiu-ds-01) | `5959b33f49c3` | [Organize downstream owners and migrate legacy scientific calls.](validation.md#uiu-ds-01-receiver-public-api-and-dependency-exit) |
| [UIU-SESSION-01](../../checklist.md#task-uiu-session-01) | `50339f01e2c0` | [Make direction sessions explicit and serialize initial dispatch.](validation.md#uiu-session-01-integrated-acceptance) |
| [UIU-SHELL-01](../../checklist.md#task-uiu-shell-01) | `45695b67b112` | [Build persistent direction tabs and scoped shell actions.](validation.md#uiu-shell-01-persistent-tabs-and-scoped-actions) |
| [UIU-US-01](../../checklist.md#task-uiu-us-01) | `c458baba0416` | [Implement approved upstream startup layout and configuration drill-ins.](validation.md#uiu-us-01-integrated-configuration-acceptance) |
| [UIU-US-02](../../checklist.md#task-uiu-us-02) | `00d0ef4e558a` | [Connect upstream execution and direction lifecycle.](validation.md#uiu-us-02-integrated-execution-acceptance) |
| [UIU-US-03](../../checklist.md#task-uiu-us-03) | `b339873f8a1e` | [Integrate upstream native PNM inputs and viewers.](validation.md#uiu-us-03-integrated-native-input-acceptance) |
| [UIU-US-04](../../checklist.md#task-uiu-us-04) | `680a15506864` | [Render authoritative upstream allocation/pilot grid.](validation.md#uiu-us-04-integrated-native-resource-grid-acceptance) |
| [UIU-SHARED-01](../../checklist.md#task-uiu-shared-01) | `3bcc97097d7b` | [Extract proven shared QAM/spectrum/response/measurement presentation.](validation.md#uiu-shared-01-integrated-shared-instrument-acceptance) |
| [UIU-SHARED-02](../../checklist.md#task-uiu-shared-02) | `4b18caf84d27` | [Share compatible FEC/verification presentation.](validation.md#uiu-shared-02-integrated-decoder-and-verification-acceptance) |
| [UIU-FILE-01](../../checklist.md#task-uiu-file-01) | `81a0e78eefab` | [Wire per-direction project Save/Open and dirty guards.](validation.md#uiu-file-01-native-routing-and-dirty-transactions) |
| [UIU-RECOVERY-01](../../checklist.md#task-uiu-recovery-01) | `81e35c11b40c` | [Preserve both direction projects across application restart/update.](validation.md#uiu-recovery-01-dual-project-restart-acceptance) |
| [UIU-QA-01](../../checklist.md#task-uiu-qa-01) | `Final QA commit containing this report` | [Accept unified Qt functionality and reconcile final correspondence.](validation.md#uiu-qa-01-final-unified-acceptance) |

The final local commit SHA is reported in the handoff. Its exact report owner can
also be resolved with `git log -1 --format=%H -- docs/development/features/unified-qt-workbench/completion-report.md`.

Final [executed evidence](final-acceptance-evidence.json) records commands, named
native cases, hashes, failed attempts/resolutions and unexecuted platform gates.
The final gates include 922 workspace cases, 140 analysis cases, four example tests,
one doctest, 14 executable API examples, 212 tooling cases including 25 packaging
cases, 647 Qt cases plus the explicit actual-GUI artifact case, 441 QML records,
217 installed native cases and three exact nested-project parity cases.
Both fresh native direction runs, shared instruments, separate-process recovery,
legacy/current project checking, warnings-denied Clippy and public boundary checks
pass. The required-reference runner fix additionally passes all 15 testing-tool cases.

One simulation runs at a time; both projects, dirty states and Save paths remain
independent. The public APIs own science; Qt remains a native presentation adapter.
Windows/hosted/installed updater lifecycles and macOS are not accepted by these
Linux checkout results. No push, master merge, release or system installation occurred.
