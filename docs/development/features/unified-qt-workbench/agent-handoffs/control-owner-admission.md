# Concrete scientific control admission: source preparation

Exact base: `62f98d50554924eb121f4d1bee53673cb11c8094`.
Branch: `feature/qt-control-owner-admission`; frozen ff062c1/f873607 branches stay.
This finite UIU-SESSION-01 preparation follows the lead-owned
[control-admission design](../design.md#uiu-session-01-concrete-scientific-control-admission),
[REQ-UIU-03/06/07](../../../../requirements.md#req-uiu-03) and the existing
[session lifecycle](../../../../architecture.md#unified-qt-session-and-file-lifecycle).
Lead owns canonical records, actual compilation/runtime and acceptance.

## Actual owner and reuse map

| Owned path/symbol | Concrete change and reused contract |
| --- | --- |
| `closed_loop/worker.rs`: DownstreamControl, Dispatch.active, Worker::spawn | Admit the adapter's actual Arc pointee/inline Mutex/last guard before host begin. Each actual profile shares one `Arc<Retained<FullLoopControl>>` between dispatch and execution, using public metadata before publishing pause/progress/science. |
| `closed_loop.rs`: Model::start | Fallible adapter admission precedes canonical application.begin; rejection preserves old science and presentation revision. |
| `transmitter/preview.rs`, existing `preview/retention.rs` | Admit one `Arc<Retained<TransmitterPreviewControl>>` before status clearing/channel/spawn; Job and worker share it through cancellation/stale/disposal. Reuse local PreviewError and canonical budget. |
| `upstream/session.rs`: UpstreamControl, UpstreamController::start | Existing outer Arc carries backend/atomic before its final internal guard. Reuse wrapped_resources and existing budget.owns; Start rejects foreign guards with the original unstarted lease. The sole fallible constructor and private nonoptional guard make uncharged production adapters unrepresentable. |
| `upstream/workbench/mod.rs`: Model::dispatch | Admit the actual upstream adapter against the same canonical budget before global begin. |

FullLoop/TX scientific footprint already includes their inline handles. Replace
that handle once with `sizeof(Retained<Control>)`, then add one new outer Arc root:
three roots, zero entries. DS cancellation adapter separately charges its full
actual pointee layout plus one root. US replaces its inline RunControl once with
its entire guarded adapter layout and adds one outer root: two roots without the
optional common stop, three with it. Existing counter/gate/Sessions/Stores remain
charged by their accepted owners. Guards drop after all data; Arc aliases share
one reservation and never expose an independently retained raw scientific clone.
No API, scientific algorithm, generic accounting framework or host baseline change.

## Focused scope and validation boundary

Add adjacent worker/preview/upstream-session control-retention tests. Existing
fixture-only edits are limited to the exact approved result/session/worker/preview/
upstream/workbench test owners; preserve original science assertions. Exercise
independent actual layouts, exact/below positive dimensions, preserved candidates,
optional common-stop, aliases/final drop and real worker/stale/spawn disposal.

All Cargo, compilation, Clippy, runtime, Qt and native checks are UNEXECUTED here.
Only pinned source formatting and source/allowlist audits run in this worktree.
The lead executes integrated owners and all affected consumers after adoption.
Timing, transport, errors/paths/help and QML remain separately scoped owners;
this source preparation does not establish SESSION aggregate acceptance.

## Prepared cases and review receipt

Ten new cases, all UNEXECUTED: DS adapter exact/below byte/root and final alias;
DS scientific-control exact/below byte/root, dispatch/execution aliases and passive
pause/cancel; real profile-control rejection with source invariance and sole
terminal; static invalid-thread-label rejection returning the original lease
(not an actual OS failure). Preview exact/below byte/root and final alias;
control rejection preserving actual old source/result/token/notice; genuine
stale Job and final control disposal. US plain/optional common token exact/below
byte/root and final alias; actual foreign-budget Start rejection preserving the
unstarted lease and configuration; paused real worker/controller destruction.
Ordered-entry demand is zero for all controls; no positive entry fixture is invented.

The seven existing fixture files only adapt admitted constructors/access paths,
actual wrapper allowances and alias-lifetime accounting. Original product-pressure
coverage leaves independently measured control capacity and asserts a genuine Job;
separate new cases cover control rejection. Product receipt precedes worker return,
so bounded Weak/exact-usage waits preserve actual post-send control guard lifetime.
Existing scientific assertions, numerical fixtures, callbacks, partial-result and
terminal semantics remain in place. DS profile scope releases the executing control
before Profile publication; adapter/gate aliases remain charged until final drop.

Executed: pinned Rust 1.85.1 rustfmt --check on all sixteen Rust files; diff --check;
exact base/seventeen-path/empty-index/text-source/frozen-branch audit; existing test
name preservation, ten-case inventory and fixture existence. Read-only diff review
confirmed unchanged scientific execution helpers and only guarded access at pause,
cancel/progress; public bounded run/generate calls retain their original inputs.
No Cargo, compilation, runtime or Clippy command was executed. The helper's guide
check/save refresh must remain current; no native artifact is built in this unit.

Final receipt supplies commit/parent, exact paths/hashes and clean verification.
Lead integrates and runs all affected control/result/project/preview/upstream
consumers, then Qt/QML/fresh executable checks. The existing upstream module-level
partial-owner summary may be reconciled by the lead; it is outside this allowlist.
No remaining source dependency blocks this finite control owner unit. Outer worker,
Job/controller/transport/timing/error and presentation owners remain separate;
this does not imply complete aggregate or official SESSION acceptance.
