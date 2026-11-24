# Worker failure payload admission preparation

Base: `3b798aac2f13651f23e50625f6cc1ea8ee8de0d1`.
Owner: UIU-SESSION-01, REQ-UIU-03/06/07. The lead's canonical
[design](../design.md#uiu-session-01-admitted-worker-failure-payloads)
records this finite unit before implementation; architecture, requirements,
checklist and acceptance remain lead-owned.

Reuse: relocate the existing GPL Qt `bridge/worker_failure.rs` implementation
at this base into `shared/worker_failure.rs`; the bridge re-exports that single
owner. Existing `AdmissionError::retained_resources`, `admit_model`, `Retained`
and the canonical application budget supply checked admission. Detailed errors
use `Box<Retained<AdmissionError>>`; the measured pointee is separate from future
fixed envelope slots. Original Designer fallback and static messages remain.

Consumers: downstream Rejected/common terminal, idle preview completion,
upstream preparation completions and upstream common terminal. Upstream's
application projection preserves the public ErrorCode and existing bounded
message policy, drops opaque ApiError before publication, and preserves category
under a borrowed fallback. Scientific execution/control/timing bodies are not
changed. GUI String copies, fixed envelope/channel/closure storage and whole
SESSION aggregate acceptance remain separate pending concrete owners.

Validation: implementation and focused source tests are preparation only.
Cargo compilation, runtime tests, Clippy, Qt builds and desktop probes are
UNEXECUTED here; the lead owns selective integration and actual validation.

## Concrete contract and integration

- `WorkerFailure::admit` keeps the original Designer fallback and diagnostic.
  `admit_with_fallback` reuses the actual error visitor and existing checked
  `admit_model` wrapper layout. Reservation precedes Box allocation; rejected
  detail drops before borrowed fallback publication. Retained data drops before
  its reservation, including a queued, stale or disconnected completion.
- Downstream Rejected and the common terminal error type use that owner;
  Finished borrows its original static literals. Preview and both upstream
  preparation variants admit actual bounded error text before send. Run
  preparation receives the existing host budget explicitly and restores its
  original controller on failure/cancellation as before.
- `RunFailure` is an application type exported beside ControllerEvent, with
  `category()` and `message()` accessors. It keeps the original ErrorCode even
  when admission falls back, reuses the unchanged `workbench::error_text` body,
  and explicitly drops ApiError before enqueue. No opaque scientific error graph
  crosses the application terminal channel.
- Adopt the exact receipt paths; no additional bridge/registration hook is needed.
  The existing bridge import remains a shim. Execute affected Designer/export,
  downstream/preview and upstream preparation/controller suites in primary.

Ten added source cases cover: exact String capacity/one-byte rejection; genuine
Diagnostic queue and disconnected send; borrowed static/selected fallback;
real downstream control rejection and stale data disposal; received/disconnected
common failed terminal; real preview target rejection preserving its prior
result/source/revision and error lifetime after GUI drop; opaque I/O error
projection with the existing bounded text; category retention under zero-byte
pressure; stale invalid configuration preserving the same old upstream report;
and target-preparation fallback restoring the same controller/report. Existing
scientific, paused/cancelled/partial, Busy, panic, target-pressure and Designer
assertions remain, adapted only to the new payload types. The terminal-only
fixture executes application transport, not substituted scientific behavior.

Pinned rustfmt/check, `git diff --check`, generated-guide check, exact
22-path/index/base/NUL/SPDX audit and 13 unchanged existing function/impl body
comparisons passed. Independent QA reported no concrete production/test defect
from source review. Compilation, Cargo tests, Clippy, builds and all runtime
cases remain **UNEXECUTED** in this worktree. This is source preparation, not
SESSION or aggregate retained-owner acceptance.

Source receipt and SHA-256 manifest (ignored local evidence):
`docs/development/local/unified-qt-workbench/worker-failure-admission-source-3b798aa/`.
Later independent GUI error/diagnostic String copies, fixed envelope/terminal/
channel/closure storage remain pending concrete owners; this unit charges the
separate detailed pointee rather than claiming those containing layouts.
