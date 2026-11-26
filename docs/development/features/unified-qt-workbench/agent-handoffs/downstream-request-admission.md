# Downstream request/plan admission preparation

Source-only UIU-SESSION-01 / REQ-UIU-03/06/07 unit based on
`987345ad9eeccc95c89daad9fe0cfc123f94d454`, following the lead's
`design.md#uiu-session-01-concrete-direction-adapter-and-request-owner-closure`.
The lead owns canonical records, shared infrastructure, integration and acceptance.

## Actual retained owners

Model and Worker now share `Arc<Retained<Arc<FullLoopRequest>>>` and the matching
FullLoopSweepPlan owner. Public retained_footprint metadata supplies every backend
payload/capacity/cache/entry/root quantity. Each reservation adds exactly
`sizeof(Retained<Arc<T>>)` payload bytes and two shared roots: the inner scientific
allocation and outer retained wrapper allocation. The scientific fixed value stays
in its own allocation and is counted once. Private refcount bookkeeping is excluded
by the approved contract; no serialized sizing or guessed factor is used.

Configure prepares through the original public API, then reserves both actual
candidate owners before host generation/readiness admission. Any owner or host
rejection drops only candidate coverage; old request, plan, options, generation,
verification and report survive. Successful accepted edits retain old worker aliases
until their real closure ends. Scientific preparation builders retain existing finite
backend limits; this unit makes no preparation-peak/scratch-byte claim.

Start clones only admitted outer Arcs into Worker. Full scientific profile execution,
public runtime narrowing/bounded calls, order, pauses, cancellation, partial results,
error text, timing lock order and shared terminal/backpressure contracts remain.
The existing per-profile public request copies are explicitly separate owners.

Cross-host rebind pre-admits complete conservative target coverage before creating
any target wrapper or inner-Arc alias. It shares the exact immutable scientific
allocations and never clones a scientific payload. Original outer aliases keep
source charges; all new outer aliases keep target charges until final drop. A
partially admitted target pair rolls back on rejection. Same-host rebind preserves
outer owners, identified through the canonical unique downstream session identity.
The demonstrated rebind consumer is independently prepared project adoption.

## Exact file map and prepared checks

- Edit `apps/qt/src/downstream/simulation/closed_loop.rs`: admitted owner fields and immutable worker aliases.
- Edit `closed_loop/lifecycle.rs`: candidate admission and pre-reserved cross-host rebind.
- Edit `closed_loop/worker.rs`: consume shared admitted owners; preserve scientific loop.
- Add `closed_loop/request_retention.rs` and `request_retention/tests.rs`: checked concrete wrapper coverage and five focused cases.
- Adapt existing `closed_loop/tests.rs`, `session_tests.rs`, and `worker/tests.rs`: owner typing and explicit test ceilings only; retain original scientific assertions.
- Add this unique handoff.

Five new functions prepare actual exact metadata/cross-budget final-alias lifetime;
all three valid-candidate dimension rejections preserving a real completed report;
old-worker/new-candidate overlap; rejected/successful/same-host rebind; and checked
wrapper layout/overflow. A zero-entry request is accepted under an exact zero-entry
policy; the actual plan catalog supplies positive ordered-entry rejection. Tests do
not fabricate scientific state or assert clone-capacity equality.

Executed: pinned Rust 1.85.1 direct rustfmt/check, diff/allowlist/source review,
byte-identical run_one and normalized worker-loop comparison, unchanged existing
assertion counts, and Python guide freshness. **Compilation, Clippy, all new and
existing runtime tests, Qt execution and integration acceptance are UNEXECUTED.**
No Cargo/build/target/cache/wheel operation was performed.

## Remaining concrete consumers

This closes only accepted request/plan graphs and their retained wrapper lifetimes.
Per-profile plan.request results, fallback/runtime-narrowed working request copies,
result targets/queued reports, ResultData observation copies, performance/QAM history,
input/editor/saved JSON/preview DTOs, Model/worker/closure Arc handles, budget counters,
controls, timing, channels and errors still require their own concrete admission.
Backend bounded operations alone do not reserve those application owners. There is
no aggregate/session acceptance claim and no new scientific API or general framework.

Lead adoption must compile and run real request/worker/Model/cross-direction/project
consumers, Clippy/boundary checks and relevant native Qt registration/build evidence.
Existing root source assertions remain preserved; do not infer pass from this review.
Ignored exact commands/source hashes are in
`docs/development/local/session-shell/downstream-request-admission/receipt.json`.
