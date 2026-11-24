# SESSION acceptance closure

This is the frozen closure list for [UIU-SESSION-01](../../checklist.md#task-uiu-session-01),
following the user's 2026-11-24 close-session directive. It refines existing
acceptance under [REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../requirements.md#req-uiu-07), with the
[canonical architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle)
and [approved retention contract](retention-contract-decision.md).
These C1–C6 labels are local closure references, not new requirements or official
tasks. Historical completed tasks and evidence remain unchanged.

## Recovered state and scope

Audit base: `a6d984e91b8cbd65e49e1b9ff822759bbc0f703e` on
`feature/unified-qt-workbench`. The primary index was empty; reviewed direction
metadata source and four fixture corrections were preserved uncommitted during
validation. The two isolated source worktrees were clean: direction metadata
`ff088c2` and upstream QML `7a0cd0c`. Nothing was reset, discarded or pushed.
The final frozen scientific dependency inventory has zero dependencies and zero
exception files; its original record remains preserved. Official progress was
10/21 accepted tasks; SESSION was unchecked. Disk had 24 GiB free and 13% inode
use. Build caches and validation records are preserved.

Read-only QA confirmed exactly six outstanding closure conditions at the audit base. No production
regression was demonstrated. Functional direction ownership, global dispatch,
paused/cancelling exclusion, stale callback rejection, accepted/partial-result
preservation and destruction are already implemented; C6 verifies their integrated
candidate. No further scheduler or scientific API is needed.

## Fixed closure list

All six rows were **required blockers** at the audit base until their evidence closes. C1–C5 close
specific existing owners; C6 closes the official acceptance gate. An independent
owner's existing scientific/editor guard must never be counted again as another
fixed layout.

| ID / requirement | Expected behavior and current gap | Exact affected owners / files | Minimum change and closure validation |
| --- | --- | --- | --- |
| **C1** / REQ-UIU-03/06/07 | Persistent direction errors/options retain their own coverage; strict target rejection preserves source state. Closed: reviewed source is integrated and focused current validation passes; see C1 evidence below. | `shared/worker_failure.rs`; `downstream/simulation/closed_loop.rs`, `closed_loop/{metadata,lifecycle}.rs`; `downstream/instruments/transmitter/preview.rs`, `preview/retention.rs`; `downstream/workbench/project_load.rs`; `upstream/workbench/{mod,presentation}.rs`. All source paths are under `apps/qt/src/`. Bridge Apply help already has its separate accepted receipt. | Finish the existing WorkerFailure/options-witness change and its eight prepared cases. Correct only obsolete fixture field/accessor construction; preserve original assertions. Execute affected direction/project/bridge coverage, Clippy/format/boundary, current executable probes and traceability. |
| **C2** / REQ-UIU-07 | Existing fixed runtime storage remains outside admitted scientific children. Queued inline layouts overlap separately charged values. | `downstream/simulation/closed_loop.rs::Model/Worker`; `downstream/instruments/transmitter/preview.rs::Analyzer/Job` and `preview/retention.rs`; `downstream/workbench/project_load.rs::PreparedModels`; `upstream/session.rs::UpstreamController/Active`; `upstream/workbench/{mod,preparation}.rs::Model/ControllerSlot/Pending`; owning `bridge/{closed_loop,transmitter_analyzer,upstream_session}.rs` adapters. | Admit the concrete fixed complements with existing reservations. Separate queued receiver/preview storage only where necessary to avoid double counting; preserve existing product guards. Fallible construction must reject before canonical host mutation and leave a minimal unavailable adapter without panic or an uncharged full model. Verify exact/below limits, constructor rollback, original controller restoration, same-host transfer and final GUI/worker aliases. No generic mutable owner framework. |
| **C3** / REQ-UIU-07 | Existing retained worker closures/control metadata and channel/slot populations lack complete concrete coverage. Message scientific/error products already have their own guards. | `shell/worker.rs`; `downstream/simulation/closed_loop/worker.rs`; `downstream/instruments/transmitter/preview.rs::Job`; `upstream/workbench/preparation.rs`; `bridge/rxmer_designer.rs`; `bridge/rxmer_export/lifecycle.rs`; loader transport in `downstream/workbench/project_load.rs`. | Extend existing checked reservations for concrete closure payloads and explicit channel/slot populations. Keep standard bounded transport and exclude private synchronization bookkeeping. Verify rejection before dispatch, original lease/controller rollback, queued/stale/disconnected delivery and final sender/worker cleanup; retain original lifecycle assertions. |
| **C4** / REQ-UIU-06/07 | Original-source registry is accepted; independent retained loader/Save paths, metadata and errors remain uncovered. | `downstream/workbench/project_load.rs::{PreparedProject,Loader,Job}`; `bridge/workbench_file.rs::WorkbenchFileRust.path`; existing retained startup/Save path copies in these owners. | Admit actual PathBuf capacities and retained metadata/error data before queue/adoption, using existing visitors. Preserve separately guarded scientific/editor owners and saved JSON. Verify failed/stale/cancelled Open and rejected Save-path adoption leave old inputs, path and metadata unchanged; final disposal releases charges. Coordinate loader transport with C3 rather than charging it twice. |
| **C5** / REQ-UIU-07 | Existing persistent QML properties and derived arrays lack the approved aggregate logical presentation limits. | `apps/qt/qml/downstream/DownstreamWorkbench.qml`; existing measurement/Designer/export dialogs; transmitter stage/QAM/PLC views; receiver QAM/response/slicer/performance/FEC views; channel response/power/loss/taps and three spectrum instances; FAQ/help presentation. These are the already inventoried presentation consumers, not a new category. | Enforce logical UTF-16 bytes, array/object entries and retained/open-view populations on existing properties, reusing current DTO/count limits. Account for replacement overlap and retained hidden/closed views until clearing/disposal. Test exact/beyond bounds, rejected replacement, derived copies, hidden-view retention and final clear/destruction. Synchronous bounded responses alone are not persistent owners. Do not introduce another accounting framework or change scientific limits. |
| **C6** / REQ-UIU-03/06/07 | Functional behavior exists, but current integrated SESSION acceptance is incomplete. Native invokable Apply/help under detail pressure remains an explicit interaction gap. | Existing `shell`, both direction session/workbench and worker tests, project tests, `bridge/closed_loop` tests, Qt/QML/native smoke runners and traceability validators. | Execute complete relevant Level 2 coverage and the applicable both-direction lifecycle milestone gate under the existing testing strategy. Demonstrate independent configuration, one active run including pause/cancel, serialized dispatch, terminal/error/panic/destruction, stale rejection, accepted/partial results, final owner release and downstream compatibility. Exercise actual native Apply/help under pressure. Check public API boundary and architecture/requirements/checklist links. Reuse unchanged artifact evidence only where permitted and identified; never label an unexecuted suite passed. |

C1 precedes C2. C2 prepares fixed envelopes for C3/C4; C3 and C4 share loader
transport ownership and must agree on one charge. C5 can proceed independently
of private Rust layout changes through the approved logical policy. C6 follows
all implementation rows. Only after C6 passes may the lead check SESSION,
commit acceptance and immediately continue with UIU-SHELL-01.

## Deferred work and change control

**Future-task responsibility:** persistent tabs, active-direction menu/action
routing and visible shell integration are UIU-SHELL-01; approved upstream startup
and drill-in presentation belong to their existing upstream tasks. New unified
file/recovery delivery stays with UIU-FILE-01/UIU-RECOVERY-01. The full line-by-line
numeric/shared-constant audit stays with UIU-QA-01. Preserve current downstream
Open/Save behavior during SESSION; its regressions would be SESSION blockers.

**Optional improvements explicitly deferred:** allocator/RSS instrumentation,
transport replacement solely to inspect private bookkeeping, generalized mutable
ownership abstractions, unrelated refactoring, future concurrent execution and
new scientific metadata/API operations without an actual consumer gap.

No new row, retained category, API or reservation mechanism may extend this
list without an exact existing requirement, demonstrated failure/defect and
identified consumer. Record the evidence and smallest correction in this same
list; do not silently broaden scope or create a requirement to justify it.

## Parallel ownership and acceptance

The lead owns primary adoption, bridge/CXX/shared registration, Cargo/global QML
registration, canonical documentation/checklist and acceptance commits. The
session source agent uses the existing isolated worktree and only an explicit
non-overlapping allowlist; QA remains read-only. The upstream source worktree's
approved components stay preserved for dependency-ordered adoption. Keep at
most three active agents including the lead. No two agents commit in one worktree.
No duplicate complete regression runs are delegated.

Record C1–C6 evidence here as each closes. Until then, SESSION remains unchecked;
commit counts and historical test totals are not task acceptance.

## C2 concrete fixed-layout implementation

Reuse the existing RetentionBudget and a final nonoptional `Retained<()>` witness
inside each DS Model, preview Analyzer and upstream workbench Model. Their app-private
factories reserve exact `size_of::<Self>()` bytes, zero entries/roots, before the
final Box allocation. All fixed inline Worker/Job/Pending/controller/Active fields
are included once in those model layouts. No separate controller Box or public
scientific metadata operation is needed. Mutable data remains directly owned;
there is no generic mutable Retained accessor.

The prepared Store and lead-owned bridges retain `Option<Box<Model/Analyzer>>`
handles. Take-before-attach still prevents owner cycles. A failed CXX default
construction retains a minimal unavailable adapter and permits retry; it must not
panic or construct a full uncharged replacement. Before foreign target/rebind
mutation, reserve the new fixed witness; same-host attachment reuses its original
guard. Keep preview products in their already charged queued Box so their fixed
storage does not overlap the whole Analyzer charge.

The existing upstream controller slot moves inline controller storage outside the
GUI model during preparation. Charge its actual
`Arc<Retained<Mutex<Option<UpstreamController>>>>` pointee plus one root before
allocation/channel/spawn. Admission failure returns the original controller through
the existing error tuple. Final worker aliases retain that guard. This is real
additional slot storage, distinct from the GUI model's empty inline option.

Source agent ownership is limited to `closed_loop.rs`, `closed_loop/lifecycle.rs`,
`transmitter/preview.rs`, `upstream/workbench/{mod,preparation}.rs`, three adjacent
new `fixed_retention_tests.rs` files and one unique handoff. The lead owns all
bridge files, prepared queue types/callers, old fixture integration, canonical
records, build/test execution and acceptance. No shared accounting, scientific,
Cargo, global registration or QML edits are part of this source unit.


## Closure evidence

**C1 closed:** [current primary receipt](session-direction-model-metadata-evidence.json)
and [validation](validation.md#uiu-session-01-persistent-direction-model-metadata)
record actual metadata integration, corrected fixture failures and passing affected
checks. Five required conditions remain: C2, C3, C4, C5 and C6. No row was added.
Official progress remains 10/21; SESSION is unchecked until C6 passes.
