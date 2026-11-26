# Downstream worker shared-terminal preparation

Source-only UIU-SESSION-01 / REQ-UIU-03/06/07 preparation from
`c33e4b273c3680e6499fb2b1be579b8cf303d79d`. The integration lead owns Model,
Context, canonical sessions, registration, shared infrastructure and acceptance.
No official task or aggregate-memory acceptance is claimed.

## Interface

`DownstreamControl::new() -> Self` owns the existing Dispatch mutex. Share the
same Arc with canonical ShellSessions admission and Worker. It implements
RunCancellation through idempotent cooperative cancellation; no join or lease
release occurs on request. Its `acknowledged_pause() -> Option<bool>` returns
Some(true) only for the actual public Paused state, Some(false) for Running,
and None for inactive/terminal controls or either cancellation flag. Worker
exposes the same acknowledgement plus existing cancel/pause/progress methods.

`Worker::spawn(request, plan, timing, Arc<DownstreamControl>, RunLease)` returns
`Result<Worker, WorkerSpawnError>`. `receiver` now contains
`WorkerEvent<WorkerMessage, String>`; `token()` preserves the original lease's
exact direction/session/generation/run. All new helpers are pub(super).

Profile, Rejected and Finished(reason) remain ordinary DATA. Finished never
releases admission. The shared transport supplies the sole real terminal after
return/unwind and owns the lease outside panic unwinding. Worker destruction
requests cancellation; receiver destruction releases bounded-send backpressure.
Only actual closure termination can acknowledge the lease. OS spawn failure
returns the original unstarted lease through WorkerSpawnError::into_parts for host acknowledgement.

The full scientific dispatch loop, runtime narrowing and public bounded run,
profile order, partial/error text, timing lock order and existing completion
backpressure are preserved. `run_one` is byte-identical to the base. No scientific
API, policy limit, memory abstraction, guessed charge or numerical algorithm changes.

## Checks and integration

Changed files only:

- `apps/qt/src/downstream/simulation/closed_loop/worker.rs`
- New `apps/qt/src/downstream/simulation/closed_loop/worker/tests.rs`
- This unique handoff.

Four prepared test functions are **UNEXECUTED**: original actual paused/cancelled
partial-report assertions plus canonical Busy/token/terminal checks; actual complete
Profile/Finished delivery retaining Busy; separate application error/panic terminal
disposal fixtures; and disconnected Worker cancellation retaining Busy until the
closure really returns. The latter two execute no scientific substitute. Genuine
ShellSessions leases use zero-sized unit configuration fixtures, with no claimed
scientific or aggregate accounting. Existing shared transport spawn-failure cases
remain the lead's tests; no production fault-injection hook was introduced.

Executed: pinned direct rustfmt/check, diff/allowlist/source review, byte-identical
run_one and normalized-loop comparison, and Python guide freshness. Compilation,
Clippy, all runtime tests and Qt execution are **UNEXECUTED**, per delegated scope.
No Cargo/build/target/cache/wheel operations were performed. The old Model signature
and unwrapped receiver matches intentionally require lead integration first.

Lead hooks: acquire the genuine canonical downstream lease with the same control
Arc; pass it into spawn and acknowledge recovered unstarted failures. Validate each
data token against canonical state before adoption. Keep Worker through Finished,
consume/dispose every real terminal (including stale/error/panic outcomes), and
finish timing/error presentation there as appropriate. Acknowledge pause only from
the actual helper result; cancellation retains the gate. Preserve Context/session
selection and existing scientific/display ownership separately. Reexecute affected
real worker, Model, shell and cross-direction integration cases before acceptance.

Ignored source/check receipt:
`docs/development/local/session-shell/downstream-worker-resume/receipt.json`.
