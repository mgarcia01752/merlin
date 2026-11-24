# C2 fixed-runtime owner preparation

Base: `ff088c2bd30b2dac48f8fb0e8c1d0316133a9970`; isolated branch
`feature/qt-session-fixed-runtime`, preserving the frozen C1 branch.
UIU-SESSION-01 / REQ-UIU-03/06/07, following the lead's finite
[closure decision](../session-closure.md#c2-concrete-fixed-layout-implementation).
No new requirement, scientific operation or ownership framework is introduced.

Concrete reuse: existing RetentionBudget/Retained and target ownership checks.
DS Model, preview Analyzer and upstream workbench Model each reserve their actual
full fixed layout before final Box construction, with a last nonoptional
Retained<()> witness. Bytes include that inline witness; entries/shared roots are
zero. Existing separately guarded data children are not counted again. Inline
Worker/Job/Pending/controller/Active storage belongs to its containing layout.

Preview keeps its already charged product Box. ControllerSlot uses the existing
Arc<Retained<Mutex<Option<UpstreamController>>>> shape: exact pointee bytes and
one shared root before allocation/channel/spawn, preserving original-controller
return on failure and worker-alias/final-drop ownership.

Lead owns all bridges, PreparedModels Box fields/callers, historical fixtures,
canonical records, runtime execution and acceptance. Its take-before-attach
sequence avoids Context/prepared-store cycles. Factories are fallible; no full
uncharged model fallback, generic mutable Retained accessor or new expect is added.
All compilation/runtime/Clippy/build/wheel checks are UNEXECUTED in this worktree.

Interfaces prepared:

- DS Model::for_context(Context) and Analyzer::for_context(Context) return
  Result<Box<Self>, RetentionError>; US workbench Model::new(ApplicationSessions)
  has the same result shape. Production DS/preview Default construction is removed.
- DS prepare_options_for_budget/prepare_inputs_for_budget/configure and test-only
  rebind stage foreign fixed coverage before commit. check_prepared_target checks
  its actual witness; attach_prepared still changes Context only after the check.
- Preview configure_for_budget and rebind_prepared stage the fixed witness before
  other target work; check_prepared_target checks its real host. Same-host guards
  are reused under pressure. Successful poll keeps Some(product Box); viewport
  only adds as_deref, preserving scientific values and existing product charges.
- US Model's application is private/unchanged after construction; its _fixed_guard
  is RAII-only. Pending::run preserves its original error tuple and poison recovery;
  slot locks use Retained::value(), without new mutable accessors or control APIs.

Six prepared cases, all UNEXECUTED: DS exact/below fixed bytes and Box move/drop;
DS foreign preparation rollback plus same-host pressured attachment; Analyzer
exact/below fixed bytes; Analyzer foreign rollback, real preview completion and
separate product/alias final drop; US exact/below fixed bytes; genuine controller
slot byte/root rejection returning the identical inspected scientific owner,
actual target preparation, held alias and final closure-alias disposal.
Every fixed model claims zero entries/shared roots; only the concrete Arc slot
claims one shared root. Tests use actual sizeof/capacities/counter baselines.
The observed slot alias is explicitly not presented as a worker barrier.

Executed here: pinned rustfmt parse/format/check, whitespace and exact source/
parent/allowlist/old-body/NUL/SPDX audits, generated Python guide check. All Cargo,
compilation, runtime, Clippy, Qt/native builds, wheel and regression checks remain
UNEXECUTED; old bridge/queue/fixture consumers intentionally require lead wiring.
The lead must box PreparedModels.receiver/transmitter and update historical
constructor fixtures, retain minimal unavailable QObject adapters with retry,
then run affected C2/session cases and reconcile acceptance.

Exact paths, source hashes, six case names and source commands are in ignored
receipt directory:
`docs/development/local/unified-qt-workbench/fixed-runtime-owners-source-ff088c2/`.
Focused local save uses tools/git/git-save.sh --commit-msg, with Feature/details/
Tests lines at most 60 columns. C2 source preparation is not SESSION acceptance.

Existing upstream target_preparation_fallback_restores_controller_and_same_completed_report
must leave exactly size_of::<Retained<std::sync::Mutex<Option<UpstreamController>>>>()
bytes available before Start, then assert actual Pending::Run dispatch. The actual
slot owner field supports Arc::downgrade without a new fixture/API hook. Wait for
that Weak to disappear before final exact baseline checks: completion receive is
not proof that the worker closure has returned. Preserve its original backend
report-target/failure-detail boundary and old report/controller assertions.
