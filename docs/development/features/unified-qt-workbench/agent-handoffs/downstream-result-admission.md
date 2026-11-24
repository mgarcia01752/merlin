# Downstream backend-result admission preparation

Source-only UIU-SESSION-01 / REQ-UIU-03/06/07 preparation on clean
`c47abda29732f19a6459924a5cce7c365c9bddd8`, following the lead's
`design.md#uiu-session-01-actual-downstream-result-and-presentation-owners`.
The lead approved backend-result admission first because the concrete DTO query
and JSON-copy target contract below is still missing. No aggregate/task acceptance.

## Admitted backend and fixed application layout

Production Worker receives the canonical RetentionBudget explicitly from Model.
run_one_admitted narrows remaining runtime through the unchanged public operation.
PreparedResult borrows that exact immutable request, calls result_retained_bound,
then reserves all three aggregate dimensions before run_bounded claims its public
control or advances source/science. No production unbounded fallback, multiplier,
private backend read, late oversized rejection or scientific API was added.

Checked payload charge is:
`backend_bound.payload_bytes - sizeof(FullLoopResult) + sizeof(ResultData)`.
ResultData now contains JobView(Retained<FullLoopResult>), so its actual fixed layout
includes the complete Retained reservation and observation/capture container handles.
The backend fixed root is replaced once; backend entries pass through, and one
future Arc<ResultData> root is added. Dynamic presentation buffers are explicitly
outside this partial proof. Temporary backend/preflight builders retain existing
finite execution policies, without an aggregate scratch/preparation-peak claim.

The same reservation moves through ResultData, queued Box, current Arc and every
old/independent Arc alias until final drop. Backend errors drop only their new target.
Complete and partial reports have no post-execution admission failure path.
Existing dynamic projectors still run with their original warnings/data behavior.

User-visible lifetime change: Start retains the previous completed report until a
new genuine-token-validated Profile is adopted. Admission/spawn/backend failure
therefore preserves that old report's exact source/job data and ownership. Accepted
input invalidation still clears it. result_token remains its existing presentation
refresh revision, including Start/Rejected/Finished/Terminal increments; it is not
the report's genuine RunToken and is not a relabeling of old scientific output.
History/performance reset behavior and the other direction remain unchanged.

Existing run_one and ResultData::new calls are cfg(test)-only compatibility helpers.
run_one delegates the same admitted real path with an independent explicit test
budget. ResultData::new charges actual already-completed fixture metadata for the
existing workbench/session and qam_history query tests outside this write scope;
that test helper is explicitly not pre-execution evidence.

## Exact residual presentation dependency

No changes to observations.rs or transmitter/capture.rs. Existing public
FullLoopResult::stage_inventory allocates a collected Vec. stage calls
observations/store.rs::query, cloning ProbeContext.run_id/configuration_id Strings
and optional transmit_cell_roles Vec, then FullLoopStageData::window copies each
typed data Vec. The public query/policy contract bounds event/element work but does
not promise those new requested capacities before copying. The public context's
role variants are fixed Copy values; no nested role String is assumed.

Application snapshot paging extends those returned vectors; Observations::observe
clones response/LLR vectors and extends tap arrays. Capture::observe additionally
copies PLC frames/waves and constructs Snapshot.data through serde_json::to_value
and phase through format!, with owned map/key/array/String targets. No pre-copy
requested-capacity contract exists for these serializer/query outputs. Post-copy
measurement/normalization would come after the required admission; a length/growth
factor is not proof. A proved borrowed/preflighted query plus capacity-enforced
application DTO construction is a separate prerequisite decision, not a new public
operation implemented here. Completed science is never rejected for these copies.

Remaining independent owners also include per-profile working/narrowed requests,
input/editor/JSON, independent preview/history, extra Model/closure/control/budget
handles, channels, timing and error owners. Backend target admission alone closes
none of their dynamic graphs and does not establish full ResultData/aggregate closure.

## Files and checks

Edit only closed_loop.rs, closed_loop/worker.rs, worker/tests.rs and the explicitly
authorized fixture-only request_retention/tests.rs; add result_retention.rs,
result_retention/tests.rs and this handoff (seven files). Existing scientific
assertions remain; request fixtures now honestly include one real report target
and the old paused worker's target instead of omitting those new reservations.

Six new test functions prepare actual per-dimension pre-control/source rejection;
exact-limit complete/pre-cancelled science compared with the original public run;
Busy cleanup/old aliases; genuine queued Box-to-Arc/final-alias lifetime; old-report
preservation on a rejected second target in all dimensions; and wrapper overflow.
No synthetic scientific outcomes or test byte ticks stand in for backend quantities.

Executed: pinned direct rustfmt/check, source/diff/allowlist review, unchanged DTO
bodies and original test-name/assertion preservation checks, guide freshness.
**Compilation, Clippy, all new/existing runtime tests, Qt/native execution and
integration acceptance are UNEXECUTED.** No Cargo/build/cache/wheel operations.
Lead reexecutes affected Model/worker/request, project/instrument/QML compatibility,
Clippy/boundary and fresh Qt evidence before accepting this partial unit.
Ignored exact command/source receipt:
`docs/development/local/session-shell/downstream-result-admission/receipt.json`.
