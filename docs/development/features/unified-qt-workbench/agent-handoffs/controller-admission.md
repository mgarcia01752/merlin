# Upstream controller admission preparation

Source-only UIU-SESSION-01 / REQ-UIU-03/06/07 preparation. The lead owns the
primary design section
`design.md#next-application-consumer-proven-upstream-controller-admission`,
canonical reconciliation, registration, application ownership and acceptance.
The public bounded API dependency is committed at
`1841d9281012c1ba602af1291313f90b44ffe3b9`; it is intentionally absent from this
isolated preparation base. No compilation or runtime acceptance is claimed.

## Interface and proven owner scope

`PreparedSession::from_document(document, observations, &RetentionBudget)` now
returns `Result<PreparedSession, AdmissionError>`. It prepares/configures through
existing public upstream operations off-GUI, reads actual
`Simulation::retained_footprint`, and reserves its final accepted owner before
publishing the shared wrapper. Failed science or budget admission drops only the
candidate and preserves existing controllers/reports. The consumed input document
is a separately charged host input; temporary backend builders retain their
existing finite execution limits. There is no aggregate preparation-peak claim.

`UpstreamController::prepare_run(&self) -> Result<PreparedRun, AdmissionError>`
is an idle-only, potentially expensive off-GUI operation. The future host moves
the idle controller to a preparation worker and returns both owners afterward.
It calls `batch_report_retained_bound`, reserves all three target dimensions
through the controller's stored budget, and binds that target to its exact
SessionKey and immutable SharedSession Arc. Prepare this before a global lease.

`start` consumes PreparedRun instead of a caller-supplied reservation. It rejects
active/stale/foreign owners before dispatch, returning the genuine unstarted
lease. The worker calls `run_with_control_bounded` with the admitted backend
limit. No post-run oversized-result rejection is introduced. Shared transport,
pause/cancel, partial reports, panic restoration, receiver destruction and final
alias charge lifetime retain the existing contracts.

Each reservation includes public scientific root metadata plus concrete fixed
wrapper storage. Checked arithmetic computes:

`sizeof(wrapper) + (scientific.payload_bytes - sizeof(scientific root))`.

Session wrapper: `Retained<Mutex<SessionStorage>>`, whose sole scientific child is
`Option<Simulation>`. Report wrapper: `Retained<BatchReport>`. The scientific
fixed root is replaced once; ordered entries pass through and one outer Arc
allocation root is added. No serialization sizing, private backend walk or guessed
factor is used. The fixed budget-handle bytes within Retained are included, but
its shared accounting pointee/edge closure remains host-owned below.

## Explicit remaining host owners

This is a backend-root plus concrete-wrapper partial proof. Before aggregate
acceptance the lead must close these owners, using checked typed quantities and
actual alias lifetime; none is silently treated as numerical scratch:

| Owner | Remaining charge or ownership hook |
| --- | --- |
| PreparedSession / UpstreamController | Their outer fixed values (`sizeof` of each type), including inline handles. PreparedSession's additional budget clone moves into Controller; its counter Arc edge/pointee needs host admission/deduplication. |
| PreparedRun | Its fixed value beyond the RetentionReservation already reserved in the future report wrapper; its SharedSession owner alias must stay tied to the source charge. Its reservation holds another budget-counter Arc edge until moved into the report. |
| RunningSession | Additional fixed `sizeof(RunningSession)` while the original Mutex/Option slot remains allocated but vacant; its source Arc alias survives through restoration. |
| Budget counter | Private Accounting pointee and all budget Arc edges held by session/report reservations and the extra host/controller handles. This unit neither walks nor estimates that private shared owner. |
| Control / coordinator / lease | Outer UpstreamControl Arc, backend RunControl shared state, control aliases in host/gate/active worker, canonical direction/session state and genuine dispatch lease. Pass the same control Arc to begin/start and revalidate callbacks against canonical ShellSessions. |
| Transport | Worker task/closure, bounded channel/queue and fixed WorkerEvent/WorkerTerminal envelopes, including error owners. Callback count alone is not a payload-byte proof. |
| Input / presentation / errors | Incoming ConfigurationDocument/raw bytes, preparation-result queue, independently cloned reports/configurations, instrument/preview/JSON DTOs, paths/dirty state, AdmissionError/backend errors and OS Start failures. Each separately retained copy/source/candidate requires its own proven charge. |

Known aliases of the same Retained payload share its reservation through final
drop. The newly owned Arc handles listed above require their own host accounting
or explicit proven deduplication; this unit does not establish that closure.
Scientific builders/scratch keep finite backend input/work/runtime policies,
separate from retained-root admission and without an aggregate scratch/RSS claim.

## Prepared checks and adoption

Only `apps/qt/src/upstream/{session.rs,tests.rs}` and this handoff change.
The existing ten real-fixture lifecycle cases now use actual three-dimensional
session/report charges. They cover completion/provenance, paused Busy, cancellation
with old aliases, rejected configuration preserving an old report, genuine public
control Busy, stale generations/sessions, stale queued reports, disconnected queue,
nonblocking destruction and unwind restoration. Three added cases cover every
report dimension plus configuration rejection, foreign owner/key preparation, and
checked fixed-root replacement/overflow. All thirteen cases are **UNEXECUTED**.

Executed: pinned Rust 1.85.1 direct rustfmt/check, diff/allowlist/source review and
Python guide freshness check. **Compilation, Clippy, all runtime tests, Qt/native
execution and combined host/session acceptance are UNEXECUTED.** No Cargo, target,
cache, wheel or build mutation was performed. Prior result branch/history remains
preserved at `913b4e734771bac04fe1cab7795ef3bdf785c5c8`.

Lead integration: adopt these three files onto the accepted bounded backend,
re-export `AdmissionError` and `PreparedRun` from upstream/mod.rs, update that
module's historical caller-asserted-reservation comment, then compile/reexecute the
combined real controller/shell checks against the fresh Qt facade cache. Wire
actual off-GUI preparation, canonical budget/key/guard/control ownership and the
remaining owner table before production registration or official acceptance.

Ignored receipt directory:
`docs/development/local/session-shell/controller-admission/`.
`receipt.json` records source hashes, exact commands, base/dependency commits and
all thirteen unexecuted cases; `source-sha256.txt` records all three delivered files.
