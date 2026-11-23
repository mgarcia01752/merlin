# Parallel Qt implementation ownership

The user authorized isolated parallel preparation on 2026-11-23. This execution
record refines the [accepted design](design.md), [architecture](../../../architecture.md#unified-qt-workbench)
and [official checklist](../../checklist.md#unified-qt-workbench-delivery).
It changes no requirement, dependency or acceptance state. Apply the existing
[three-level strategy](testing-strategy.md); prepared branch tests are not
integrated task acceptance.

## Checkout and write ownership

| Owner | Branch and worktree | Exclusive write scope |
| --- | --- | --- |
| Integration lead | `feature/unified-qt-workbench`, `/home/dev01/Projects/merlin` | Downstream migration, backend APIs, existing Qt Rust/QML, registration, manifests/locks, startup, canonical architecture/requirements/checklist and acceptance evidence. |
| Upstream presentation | `feature/upstream-qt-ui`, `/home/dev01/Projects/merlin-worktrees/upstream-qt-ui` | New `apps/qt/qml/upstream/`, `apps/qt/tests/upstream/` and `agent-handoffs/upstream-ui.md` only. |
| Session/shell preparation | `feature/qt-session-shell`, `/home/dev01/Projects/merlin-worktrees/qt-session-shell` | New `apps/qt/src/shell/`, `apps/qt/qml/shell/directions/`, `apps/qt/tests/shell/` and `agent-handoffs/session-shell.md` only. |
| Dependency/evidence/disk audit | Read-only primary checkout | No file/index writes, builds, commits or deletion; return exact findings to the lead. |

Both isolated worktrees start at `102f4908f45292c6715308e9693fdf26b89939c2`.
The primary checkout's pending measured-input prerequisite was preserved with
hashes, binary patch and untracked copies in ignored local handoff evidence,
then accepted and committed as `f68b6d30781f8b509ccd5a42bd72f5fee2a78e2d`.
No reset, stash, discarded work or source duplication establishes a baseline.

Initially run at most three agents including the lead: upstream presentation and
a short read-only audit run alongside downstream work. After the audit finishes,
reuse its concurrency slot for session/shell preparation. Do not subdelegate or
start monitoring agents that repeat repository analysis. One coding agent is the
sole writer/committer in each isolated worktree. The lead alone commits accepted
integration in the primary checkout. Any reassignment requires an explicit
handoff before another writer starts.

Agents must not edit shared components, CXX-Qt bridge declarations/registration,
Cargo manifests/locks, global QML registration, application startup, existing
shell/downstream files or canonical documentation. Missing integration hooks are
reported to the lead. Agent-local handoff records carry focused design and test
evidence; canonical documentation remains single-owner. The Git helper stages
all nonignored work, so each agent reviews staged/unstaged/untracked contents
and its allowlist before saving, then verifies SHA and a clean worktree. No push,
master merge, publication or installation is authorized.

## Component boundaries and acceptance

Upstream preparation reuses existing theme, blocks, connections and dialogs
read-only. Its injected host supplies bounded presentation state from the future
direction adapter; signals request drill-ins/actions. The [approved layout](upstream-startup-layout.md)
keeps CM TX → channel → CMTS RX distinct from CMTS control, measured RxMER,
transmit correction C and recovered verification. Read-only drill-ins must label
unavailable data honestly. Editable controls and a resource-grid renderer may use
only verified existing public contracts; no inferred settings, pilot algorithm,
scientific validation or fabricated observations belong in QML.

Session/shell preparation provides application ownership/dispatch components,
independent direction state and persistent-tab presentation. It must not change
downstream input/result singletons concurrently with UIU-DS-01. Agree the proposed
interface with the lead first. One active simulation includes paused runs;
direction/session/generation/run identities reject stale callbacks. Switching tabs
does not stop a run. Save targets the active direction; Open guards the detected
target direction. These are application policies, not a second scientific API.

Each coding handoff must state its exact interface, files, requirement/task links,
actual focused tests, omissions, integration hooks and local commit SHA. Require
real QML rendering/interaction tests for presentation; session tests exercise
ownership, paused Busy, stale completion, cancellation/destruction and isolation.
Do not claim unregistered components are running in the installed application.
No new public backend API is permitted without a demonstrated consumer gap,
lead-owned design/traceability and independent paired acceptance.

Integration remains: **UIU-DS-01 → UIU-SESSION-01 → UIU-SHELL-01 → UIU-US-01**,
then direction execution, PNM and grid tasks with their accepted API dependencies.
Shared instruments require two real consumers; files/recovery and final QA retain
their existing gates. The lead reviews complete branch diffs, licenses, numeric
policies and tests, integrates eligible focused commits, adds registration/wiring,
then executes Level 2 acceptance before changing any official checkbox. Milestone
Level 3 remains lead-owned. Neither preparation nor source-only passes advance
an integration task past its dependencies.

## Build, storage and usage budget

Initial measurements: filesystem 122 GiB total, 91 GiB used, 26 GiB available
(79%); inodes 12%. Active root `target` is 22 GiB, Qt target 3.7 GiB, Qt Cargo
cache 545 MiB, backend cache 386 MiB, local integration tools/evidence 3.4 GiB,
and user cache 2.5 GiB. Default user Cargo registry/Git cache paths are absent.
These rounded measurements describe this checkout, not portable requirements.
No cleanup was needed before the small source-only worktrees were created.

Only the lead runs Cargo/wheel/milestone builds using the established caches.
Agents use existing Qt tools read-only and isolated small test outputs; do not
create full per-agent Cargo targets or concurrently write shared build targets.
Do not invalidate caches to obtain a clean-looking build. Record disk/inodes before
major compilation/test suites and after significant builds. Consolidate existing
measurements instead of repeating broad scans.

Below 75% utilization use normal operation. At 75–85% inspect growth/candidates;
at 85–90% clean verified disposable outputs before large builds; above 90% suspend
new disk-intensive builds. Less than 10 GB available is a risk regardless of the
percentage. Monitor inodes too. Remove only identified completed-task temporary
outputs or inactive redundant build/cache artifacts after checking tracked/dirty
source, scientific inputs, required evidence and active use. Preserve active
worktrees/Git metadata; no broad target/cache deletion, `cargo clean`, unrelated
system cleanup or privilege escalation. Report uncertain candidates.

Use narrow handoffs, short audit passes and change-focused tests to limit duplicated
context and computation. Parallel work may reduce elapsed time but can increase
total token usage; no Pro-usage saving or completion date is guaranteed. Report
remaining owner batches and validation/integration effort, not an unsupported
percentage estimate.
