# Session/shell preparation

## UIU-SESSION-01 bounded owner unit

Prepared on `feature/qt-session-shell` from
`102f4908f45292c6715308e9693fdf26b89939c2`. The integration lead approved the
Rust-only interface before implementation. This is preparation under
[approved design](../design.md), not integrated acceptance.
[Architecture](../../../../architecture.md#unified-qt-session-and-file-lifecycle),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07) and
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01) remain canonical.
UIU-DS-01 remains a prerequisite; this unit does not change official status.

### Chosen boundary and owner map

| Disposition | Path | Responsibility |
| --- | --- | --- |
| Add | `apps/qt/src/shell/mod.rs` | Unregistered pure application module and typed direction sessions. |
| Add | `apps/qt/src/shell/identity.rs` | Opaque checked process session, configuration and run identities. |
| Add | `apps/qt/src/shell/retention.rs` | Shared checked byte reservations owned with retained payloads. |
| Add | `apps/qt/src/shell/dispatch.rs` | Shared one-run gate, nonblocking cancellation and terminal lease. |
| Add | `apps/qt/tests/shell/session.rs` | Standalone owner/lifecycle/retention tests. |
| Reuse later | Existing directional public controls | Lead supplies cancellation adapters; no scientific interface changes. |
| Defer | Existing input/result stores, bridge/startup/build registration | Lead alone migrates/wires after downstream acceptance. |

`ShellSessions<DC, DR, UC, UR>` holds separate immutable typed configurations and
results. New accepted configuration advances the checked generation. Application
metadata does not expose a mutable scientific payload. Tokens carry direction,
session, generation and run. Direction selection never changes execution.

`DispatchGate` is owned by the application and shared with a non-cloneable
`RunLease`. Running, paused and cancelling work occupies one slot. Shutdown and
Drop request cancellation once and never join a worker. Only actual worker
termination acknowledges/releases the lease; losing a lease keeps admission
Busy rather than admitting overlapping work. The worker must own its execution
inputs/control until termination even if the presentation owner disappears.

`RetentionBudget` has a caller-supplied application byte ceiling, no default and
no scientific inventory. Reserve before allocating/cloning retained data, then
attach the reservation to its exclusively owned payload. Reservations follow
accepted, queued and old-run payloads until drop. The caller must charge a
conservative upper bound for all reachable allocations and must not leak owned
payload aliases around this wrapper. This is ownership accounting, not an
allocator meter or a replacement for backend execution limits.

Save snapshots include dirty revision; completion cannot clear later changes.
Open guards target the detected direction and are rechecked at actual replacement,
including identity, generation and dirty revision/state. Dirty replacement needs
explicit discard permission. Neither helper reads/writes files, serializes projects
or changes schemas. UIU-FILE-01 retains actual chooser, prompt and write acceptance.

### Reuse decisions

| Inspected owner | Existing contract | Decision |
| --- | --- | --- |
| `src/downstream/workbench/session.rs` | Thread-local accepted inputs and saved editor sections | DEFER migration to lead; do not modify or reproduce science. |
| `src/downstream/simulation/closed_loop/worker.rs` | Public controlled worker, pause/cancel and retained actual reports | REUSE later through application cancellation adapter; do not implement another worker. |
| `skills/public-api/SKILL.md` saved-workbench contract | Shared Rust/Python project semantics | Preserve; no project schema or persisted setting changes here. |
| PNM-Viewer-Toolkit reference checkout | Referenced path is absent locally | No material copied; new GPL application ownership code only. |

### Validation and handoff

The final source comprises four new Rust owners plus one test file. No QML is
delivered; UIU-SHELL-01 presentation and executable acceptance stay pending.
Canonical architecture and official checklist were reviewed against this scope;
no owner/status update is appropriate before registration and actual migration.
Per-direction project paths/instrument selections and close-both-dirty UI remain
lead-owned migration work, beyond this unit's explicit selected-tab/dirty guards.

The interface is:

- `ShellSessions<DC, DR, UC, UR>::new(budget, gate, downstream, upstream)` creates
  two typed accepted configurations, initially clean, selecting downstream.
- `downstream()` / `upstream()` borrow typed immutable configuration/result owners.
  Configurations must not permit interior scientific mutation; executing mutable
  backend sessions remain worker-owned. `configure_*` advances generation and
  edit revision atomically and invalidates old results, only while that owner is idle.
- `begin(direction, Arc<dyn RunCancellation>)` returns the unique `RunLease`;
  `lease.token()` identifies every progress/result callback. `DispatchGate::active`
  reports the exact Busy owner. `acknowledge_pause` records an already accepted
  backend pause/resume, including continued slot ownership.
- `publish_*` accepts actual reserved partial reports. `finish_*(lease, result)`
  consumes terminal leases on success, no-result failure, stale/wrong-direction,
  closing and foreign-budget rejection. `result = None` preserves the last
  accepted result. `validate_callback` guards progress adoption immediately before
  the host applies its fields; it grants no permission across later events.
- `shutdown` and Drop cancel owned work once, without joining. A lease can also
  acknowledge actual termination after its coordinator has been destroyed.
  Dropping the sole lease unacknowledged deliberately strands Busy admission;
  there is no automatic reuse or unchecked token release. The host must retain
  the lease outside panic unwinding and consume it after known termination.
- `save_target` snapshots selection; `acknowledge_save` cannot clean a newer
  identity/generation/edit revision. `open_target(detected_direction)` guards the
  actual destination; `replace_*` rechecks it and `ReplacePermission` at adoption.
- `RetentionBudget::reserve(RetainedBytes)` reserves before payload allocation;
  `RetentionReservation::retain` attaches accounting. Payload drop precedes charge
  release. Cloned budgets share counters; foreign budgets are rejected.
  Replacements must have room for both accepted and candidate ownership at once.

### Executed checks

Used the existing pinned compiler read-only, with isolated ignored outputs:

```bash
/home/dev01/Projects/merlin/local/toolchains/rust-1.85.1-ylt882zx/bin/rustc \
  --edition=2021 --test -D warnings apps/qt/src/shell/mod.rs \
  -o docs/development/local/session-shell/session-tests
docs/development/local/session-shell/session-tests --test-threads=4
```

All 16 tests pass. A second ignored harness imports this owner as a nested
`mod shell`, confirming the tests do not assume crate-root registration; all 16
pass there too. The harness suppresses only unused exports/dead code for the
unregistered module, not compilation warnings in the directly compiled owner.
The first test run caught an inferred integer type in one queued fixture; its
explicit `u8` annotation corrects the independently expected byte-size anchor.
No production algorithm or budget was changed to satisfy that assertion.

The executed cases are:

- `process_identity_exhaustion_never_wraps`
- `duplicate_or_stale_terminal_does_not_release_newer_run`
- `distinct_typed_directions_keep_configuration_results_and_dirty_state`
- `running_and_paused_runs_reject_second_start_and_keep_scoped_controls`
- `cancel_while_paused_is_once_and_holds_slot_until_partial_terminal_report`
- `callbacks_require_direction_session_generation_and_run_identity`
- `failed_terminal_and_rejected_candidate_preserve_previous_accepted_report`
- `queue_old_run_replacement_and_owner_drop_keep_byte_charges`
- `byte_limits_are_inclusive_shared_and_checked_without_large_allocations`
- `dirty_guard_rechecks_actual_target_and_save_does_not_clean_newer_edits`
- `generation_revision_and_run_exhaustion_preserve_state_without_wrap`
- `destruction_requests_once_and_worker_lease_prevents_successor_overlap`
- `closing_or_wrong_direction_terminal_rejection_still_consumes_lease`
- `dropped_unacknowledged_lease_cancels_but_never_reopens_gate`
- `foreign_owner_destruction_never_cancels_another_sessions_worker`
- `retained_payload_is_destroyed_before_its_byte_reservation_is_released`

Focused formatting uses the pinned `rustfmt --edition 2021 --check` on the four
owners and test file. `python3 tools/python/update_integration_doc.py --check`
reports the generated guide current without changing it. Final whitespace,
allowlist, SPDX/public-banner and numeric-source reviews accompany the local save.
The only production numbers are zero initial counters/charges, checked unit
increments and the intrinsic two-session identity pair; there is no memory,
trial, runtime or scientific default. Test sizes are independent fixture anchors.

Disk before/after: 122 GiB total, 91 GiB used, 25 GiB available (79%); inodes
12%. Own ignored compiler outputs occupy about 13 MiB. No shared target/cache,
other checkout, source evidence or installed files were changed or removed.

### Not executed and deferred gates

No Cargo target, Clippy, Qt executable/QML suite, backend workspace, wheel, CLI,
packaging or native UI checks were run: this unregistered std-only owner has no
consumers in this branch and those builds are explicitly lead-owned. There are
no scientific operations, public backend API/schema/persisted-setting changes,
so this unit introduces no Rust/Python saved-workbench compatibility fixture.
Existing consumers, real direction workers and actual file operations must still
pass expanded Level 1/2 checks when integrated. Parent UIU-SESSION-01,
UIU-SHELL-01 and FILE/RECOVERY acceptance remain open; Level 3 remains lead-owned.

### Lead integration hooks

1. After UIU-DS-01 acceptance, register `mod shell` in application startup. Keep
   bridge declarations under the existing bridge owner and QML registration with
   the existing module; none are provided here.
2. Create one application-wide `DispatchGate` and explicit `RetentionBudget`.
   Wrap the actual separate accepted configuration/input/result payloads with
   conservative reservations. Retire implicit downstream stores under the lead's
   exclusive ownership. Do not create a fresh gate to bypass an unfinished lease.
3. Adapt each direction's existing public cancellation control to
   `RunCancellation`. Call `begin` before dispatch. Keep worker execution state,
   original inputs and lease in a terminal envelope. Spawn failure and every
   completion/cancel/error path consume the lease through `finish_*` or actual
   terminal acknowledgement. Catch panic outside the lease's unwinding scope.
4. Tag all callbacks with the lease token; reserve queued report ownership before
   making copies. Adopt reports through the matching `publish_*`/`finish_*` owner.
   Guard progress with `validate_callback` immediately before applying fields.
   Public backend controls own pause
   semantics; record only accepted pause state in the shared gate.
5. Bind selected-tab controls to that direction's exact key. Show another
   direction's Busy ownership; never redirect Stop/Pause. Selecting a tab alone
   changes no configuration, worker control, result or dirty state.
6. FILE-01 may use the Save/Open guards after public project direction admission,
   but must implement real chooser/dirty prompts/safe writes and retain existing
   Rust/Python schema compatibility. This helper does no serialization or disk I/O.
7. Before official acceptance, test both actual direction consumers, retained
   payload bounds/terminal delivery and destruction, then persistent QML tabs,
   existing menus/update/focus/theme behavior and latest native executable.

Focused local save SHA is reported with the handoff; no push/merge/release/install
is authorized or performed.

## Integration review correction

The lead moves the exact `apps/qt/tests/shell/session.rs` test body beside its
owner as `apps/qt/src/shell/tests.rs` and replaces the external path attribute
with ordinary `mod tests;`. The existing final Qt boundary rejects external
source-tree inclusion. Test assertions and production logic remain unchanged;
lead-owned final checks and acceptance are recorded in the canonical feature
validation. The prepared commit above remains historical branch evidence.

## UIU-SESSION-01 upstream controller preparation

Prepared after `93dd675fc34250ce961d14c640f9decd76f66f16` with the lead-approved
interface. The immediate owner is
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01), serving
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07); later reuse belongs to
[UIU-US-02](../../../checklist.md#task-uiu-us-02) and
[REQ-UIU-12](../../../../requirements.md#req-uiu-12).
[Canonical session ownership](../../../../architecture.md#unified-qt-session-and-file-lifecycle)
and the [approved design](../design.md) remain authoritative. This is preparation,
not official task, Qt registration or aggregate-memory acceptance. The earlier
`93dd675` tabs remain unintegrated.

### Owner/interface contract

Only three new application files and this handoff change:

| Add | Responsibility |
| --- | --- |
| `apps/qt/src/upstream/mod.rs` | Unregistered exports and acceptance boundary. |
| `apps/qt/src/upstream/session.rs` | Actual public session/document, exclusive worker ownership, charged actual reports and exact-token controls. |
| `apps/qt/src/upstream/tests.rs` | Ten real public-API fixture tests inside the source tree. |

Reuse is direct: `merlin::upstream::{ConfigurationDocument, Simulation,
RunControl, BatchReport}` retains validation/science/finite options/original
errors; public shell worker exports at primary `2ed5a1e` supply the sole
thread/terminal transport. The unchanged supported fixture is
`examples/cli/upstream/us-50khz.json`. No engine, scientific API, saved schema,
serialization, binding, generic shell or registration code changes.

- `PreparedSession::from_document(document, observations, reservation)` admits
  through public operations off-GUI and owns the actual session/source document.
  `UpstreamController::new(key, prepared)` requires a host-issued upstream key.
- `inspect` borrows the actual idle session immutably, returning Busy rather than
  waiting. Owned copies/instrument snapshots need separate host reservations.
  Changed science requires a new prepared owner and changed canonical identity.
- `start(lease, control, result_reservation, capacity)` checks the immutable key
  and calls only public `Simulation::run_with_control` via shared transport.
  A short-lock application guard restores the actual session on return/error/
  unwind. `StartError::into_parts` returns the original unstarted lease for host
  acknowledgement, including OS spawn errors.
- The same `Arc<UpstreamControl>` must reach shell `begin` and `start`; it wraps
  the actual public control with one cancellation request across gate/controller/
  destruction. `progress` returns the public scalar snapshot with its token.
  `pause/resume/cancel` require the exact active token; pause request is not ack.
- `poll` consumes at most one event. Actual reports, including real partial ones,
  carry their original token. Original typed `ApiError` remains in returned
  `WorkerTerminal<ApiError>`. API failure/panic preserves the old report. Host
  consumes or drops every known terminal; disconnection without terminal retains
  local Busy conservatively. Drop cancels without joining or releasing early.
- Configuration and result charges live inside shared retained payloads, surviving
  worker/queue/old-view aliases and destruction. Cloning underlying data requires
  another reservation. The controller owns no budget and cannot verify provenance;
  the host supplies independently audited, pre-reserved charges from the correct
  application budget, including preparation peak ownership.

Public preview/options/input-length/policy counts give finite populations but
not private observation/playback capacities or a deep reachable-byte bound.
Serialized size is not heap size. No estimate or numerical policy is invented;
aggregate proof stays pending. Test accounting ticks prove ownership only.

### Executed focused evidence

Exact command arrays, logs and source/artifact hashes are in the ignored receipt:

`/home/dev01/Projects/merlin-worktrees/qt-session-shell/docs/development/local/session-shell/upstream-validation/receipt.json`

Pinned `rustc` compiles the real public-API tests against cached
`libmerlin-3892f0def2e621d5.rlib` (`downstream,native-input,upstream`). The harness
uses the accepted branch shell plus an exact primary worker copy, omitting only
its own test-module declaration. Ten tests pass, zero fail; 16 earlier foundation
tests compile but are filtered, not rerun. Cases:

1. Actual completed recovery, original document bytes and real report coverage.
2. Acknowledged pause, global other-direction Busy and actual resume completion.
3. Paused cancellation/actual partial report plus old-report charge retention.
4. Invalid real-document rejection preserving the existing accepted owner.
5. Genuine public `control.busy` failure preserving old report/session.
6. Changed generation/replaced session rejection returning the unstarted lease.
7. Stale envelope carrying a real prior report rejected without adoption.
8. Queued rendezvous report/disconnection cleanup after actual completion.
9. Paused destruction/cancellation with eventual terminal and charge cleanup.
10. Application unwind restoring the actual accepted public session.

Direct `clippy-driver -D warnings -D clippy::all`, pinned `rustfmt --check`,
whitespace/allowlist/SPDX/banner/comment/numeric review and Python-guide currency
pass. Harness-only allowances cover unused exports/dead code for unregistered
modules; no Clippy lint is suppressed. The stale-envelope injection tests identity,
not a fake engine. Counter charges, callback capacity and scheduling deadline are
explicit harness anchors, not scientific defaults or deep-byte measurements.

| Source/artifact | SHA-256 |
| --- | --- |
| `upstream/mod.rs` | `2f0795590115d8366560441df0e07d0183fa3c9218a3c0ad8a5ea4057f650d0a` |
| `upstream/session.rs` | `8eb393fe83e35e8b2d4af84c77339b5104011e5d45988cbfc6dbbc03dfec77a4` |
| `upstream/tests.rs` | `258f45e9f74434ce957198618b12025b632e81d8713620c73c53bb27ed511366` |
| Ignored `receipt.json` | `daea87e5a93fd0d2c7f911f693ba962510a312ddbb772464a777b6c658799fe4` |
| Ignored `upstream-tests` | `d1c36b9c156918a5a3af39a04f6bf57da054b19b11260c7d14f8f0be9b5b0a64` |

The receipt also hashes the exact shell foundation, primary worker, fixture,
backend cache/fingerprint, harness, copied worker and logs. Disk before/after:
79% used, 25 GiB available, inodes 13%; this unit's ignored outputs occupy 14 MiB.
No shared target/cache, input evidence or other checkout was edited or cleaned.

### Remaining lead hooks/gates

1. Review/adopt only these files and this new handoff section, preserving the
   primary's earlier test-placement adaptation. Register/wire the actual owner
   under lead control; tabs, build, bridge, Cargo and QML remain separate.
2. Prepare off-GUI with audited host reservations/provenance. Recheck actual
   Open/configuration guards and advance canonical identity on replacement;
   never rebind changed science under an unchanged key.
3. Share the control Arc/genuine gate/lease and validated callback-count policy.
   Poll without Qt waits. Revalidate every returned token against canonical
   `ShellSessions` immediately before current UI/store adoption: an immutable
   retired controller only validates its old key. Record actual pause/resume ack
   in the gate; gate cancellation reaches the same once-only adapter. Consume
   terminal leases through canonical finish/drop; never free admission on request.
4. Recompile/rerun against primary sources, then validate actual upstream Qt
   lifecycle/downstream isolation/responsiveness and native targets. No Cargo,
   shared-target build, wheel/full workspace/Python/CLI, full Qt/native, packaging,
   installation or hosted checks were run. Cached-library tests are preparation,
   not primary-source rebuild evidence. Aggregate memory, FILE/RECOVERY,
   instruments, expanded Level 1/2 and final Level 3 remain pending. Architecture/
   checklist were reviewed without changing official ownership or status.

Focused save SHA/clean-tree verification are reported separately. No push,
merge, release or installation occurs. The helper's deterministic Python-guide
refresh must retain its existing hash.

## UIU-SESSION-01 upstream retained-root metadata preparation

Prepared from `d1a1aa788c3ce93e6c4e8618c3d349f562f91252` under the user's approved
payload-byte / separate ordered-entry / shared-root contract. The lead owns the
canonical design in primary
`design.md#uiu-session-01-public-retained-root-metadata-prerequisite`, linked to
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07). No official acceptance or
Qt admission follows from this source-only preparation.

### Operations and owner map

- `api::Simulation::retained_footprint(&self) -> ApiResult<RetainedFootprint>`
  measures current options/configuration/document/provenance and private caches.
- `simulation::BatchReport::retained_footprint(&self) -> PhyResult<RetainedFootprint>`
  includes original/canonical/provenance strings/bytes, configuration/options/
  plan, grants/decoder decisions and all private observation stores.
- The approved common walker adds the fixed root once. Owner-local trait methods
  visit dynamic children, actual Vec/String spare capacities and each shared
  pointee/edge, conservatively repeating aliases. Checked overflow maps to the
  existing ResourceLimit field `retained_footprint`. Static borrowed text owns no
  pointed storage; allocator/tree-node/refcount headers remain excluded.

All 69 reachable local types are covered. Sixty struct patterns are exhaustive;
inline enums are matched exhaustively. Foreign `DecodeResult` is destructured
locally to count both decision vectors without an orphan trait implementation.
The foreign `DocsisLdpcCode` is a fixed inline unit enum. Neither root owns a
`serde_json::Value`: JSON provenance is retained as String, never parsed for size.

Only owner metadata is added under `crates/ofdma/src/`:

| Concrete owners (relative to that directory) | Audited ownership |
| --- | --- |
| `api/session.rs`; `simulation/request.rs`, `report.rs`, `control.rs` | Two public roots and complete accepted request/result fields. |
| `configuration/mod.rs`, `profile.rs`, `grant.rs`, `waveform.rs`, `receiver.rs`, `guards.rs`, `document.rs`, `synthetic.rs`; `configuration/ucd/channel.rs`, `controls.rs`, `profiles.rs` | Typed/source documents, inventory/control vectors, original shared bytes and caller capacities. |
| `channel/stream.rs`; `signal_processing/pre_eq.rs`; `measurements/mod.rs`, `pre_eq.rs`, `rxmer.rs` | Channel taps, raw/normalized banks, effective-bank cache, original/cached RxMER powers and playback inputs. |
| `types/iq.rs`, `bits.rs`; `grant/plan.rs`; `receiver/framing.rs`; `resources/grid.rs` | Inline types, plans and owned decoded payload/information/mother decisions. |
| `instruments/observer.rs`, `receiver.rs`, `metrics.rs`; `instruments/soft/policy.rs`, `store.rs`, `views.rs` | Ordered trace payloads, serialization-skipped role gains, estimate/soft shared records and every metric buffer. |

The production diff is additions only: no original algorithm, container behavior,
limits, serialization or data field is removed/changed. Existing copyrights are
preserved; added reusable Rust metadata/tests remain Apache-2.0. No Qt, manifest,
lock, common/facade/Python or global documentation file changes here.

### Prepared tests and executed source checks

All twelve new owner tests are **unexecuted**, including compilation/Clippy.
Lead execution must combine this patch with the new common support in primary;
that support is deliberately absent from this standalone branch. No Cargo,
shared-target, wheel, full suite or native tests were run in this worktree.

Six integration cases in `crates/ofdma/tests/retained_footprint.rs` prepare checks
for fixed idle root/options spare capacity; source strings/vectors/original bytes;
measured provenance capacity; public report/decoder spare buffers; real
complete/pre-cancelled report and passive invariance; and actual estimate/soft
shared-store inclusion. Six private-owner cases cover normalized Pre-Eq buffers,
effective-bank cache, RxMER cached powers, skipped role gains/ordered entries,
repeated estimate Arc payloads, and repeated soft-cell/word metric buffers.
Private capacity/alias anchors do not establish scientific acceptance.

Executed here: pinned rustfmt parse/format check, whitespace review, exact-scope
review, ownership-closure review (69 implementations) and exhaustive struct-field
review (60 patterns), plus Python-guide currency. The guide refresh is unchanged.
Disk before/after remains 79% used, 25 GiB available; inodes 13%. No build outputs,
shared targets, caches or external inputs were changed/cleaned.

### Lead hooks and remaining prerequisite

Adopt only this owner patch, its integration test and this new handoff section;
preserve prior Qt preparations and primary handoff adaptations. Required common
support is read-only primary `crates/phy-api/src/retention.rs` (SHA-256
`52f53a15b46f483750e7661cb641a89409db82fdbbd2eef62c58deb19c00aed7`)
and `retention/support.rs` (SHA-256
`0ffc6f1f19cbbb38387bf82ea7d0507ae8cd243c11b20e2dba6bddd6fd0e7cd7`), including
the generic pair implementation. Lead owns actual compile/test/Clippy, public
exports/native parity, requirement/inventory checks and acceptance evidence.

Current-root metadata is passive and cannot reserve a future result or temporary
execution workspace before allocation. The future-output bound and producer/copy
preflight remain the next mandatory prerequisite; no estimate, substitute bound,
Qt admission or aggregate-memory acceptance is claimed here. No push/release/
installation occurs. Focused save SHA and clean-tree result are reported separately.

## UIU-SESSION-01 downstream retained-root preparation

Prepared from `bbac5fbacf3db97d451cf7ea5b49c228a764a464` on
`feature/qt-retention-downstream`. The preserved `feature/qt-session-shell`
branch remains at `665d758d1e89ca4542999a499fc934ad82057efa`. The lead owns
[the canonical design](../design.md#downstream-retained-root-preparation),
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07). This source-only preparation
establishes no official task, Qt or aggregate-memory acceptance.

Four immutable roots now provide
`retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint>`:
`FullLoopRequest`, `FullLoopSweepPlan`, `FullLoopResult` and
`FullLoopReceiverArchive`. The common helper counts the fixed root once;
owner-local visitors cover requested Vec/String capacity and every initialized
child, conservatively repeating Arc pointees and reporting each shared root.
Ordered entries are separate from payload bytes. Allocator, tree-node and Arc
control-block overhead are excluded. Checked overflow maps to ResourceLimit
with `retained_footprint` context. Success neither allocates nor advances science.

The complete owner graph has 83 exhaustive struct patterns, 28 exhaustive enum
matches and seven tuple-wrapper implementations across 55 downstream source
files. API inputs/provenance/catalogs and source selections reach original and
decoded bytes, source strings, assignments/schemes/summaries, all private
MeasurementPlayback frames/powers/average caches, settings/playback/noise/layout/H,
actual job/FEC/received-H/cell-LLR/recovered-file results, the serialization-skipped
protected pilot sequence, ordered stage stores/contexts/roles/data and detached
archive observations. Borrowed static text is non-owning. Duration and the audited
five-u64 Copy FecCounters use the agreed `retention_support::fixed_children`
helper; no foreign trait implementation or new dependency is introduced.

Production changes are additions only. Original algorithms, fields, containers,
defaults, limits and serialization are unchanged. Archive `info().retained_bytes`
and `receiver_archive/copy.rs` retain their existing length-based statistic and
copy policy. Each clone is measured as its actual owner: cloning Vec/String may
shrink spare capacity, while shared Arc graphs retain the same payload.

Ten new tests are prepared, **UNEXECUTED**, including compilation and Clippy:

1. Private playback frame/power/average spare buffers and three Arc roots.
2. Skipped protected pilot cache and empty stream spare buffer, passive JSON.
3. Original/decoded repeated byte Arc and provenance/decoder string spares.
4. Ordered stage entry, spare event slots, context IDs/roles and LLR buffers.
5. Payload/NCP pair buffers and both nested LLR capacities, passive JSON.
6. Repeated original-file Arc ownership without payload cloning.
7. Idle request and selected sweep spare capacity without execution.
8. Real complete result, actual decoder stage and per-owner passive stability.
9. Real pre-cancelled partial result with original inputs and no fabricated work.
10. Real finite-file recovery and detached QAM/decoder archive after original
    request/result destruction, preserving the legacy retained-bytes statistic.

The six private cases are storage accounting anchors, not scientific acceptance.
The four integration cases in `crates/ofdm/tests/api_retained_footprint.rs` reuse
the established supported small full-loop fixture. Decoder retention preserves
actual phase tags; it does not assume decode completes during observation.

Executed here: pinned rustfmt parse/format checks, whitespace/allowlist review,
exhaustive owner-field and source-closure review, read-only independent review,
and Python-guide currency. All owner tests, Cargo/Clippy, complete affected
consumer/native parity, inventories and Qt checks remain for primary adoption.
No Cargo/shared-target/wheel/full-suite build was run. The existing architecture
and checklist remain unchanged; the lead owns their reconciliation. Disk remains
80% used with 25 GiB available and 13% inodes; no targets/caches were cleaned.

Receipt and exact source hashes are in the ignored local directory
`docs/development/local/session-shell/downstream-retention/` (`receipt.json`,
`source-sha256.txt`, `owner-fields.json`, `review.txt`). Lead integration must
combine this owner patch with primary `merlin_api::RetainedFootprint` and
`retention_support`, including `fixed_children`, then compile/run the ten cases
and complete the affected API/native and feature gates. No input-root public
operation is added. Future-output reservation and archive pre-copy capacity
bounds remain mandatory prerequisites; passive current-root metadata cannot
supply either bound or establish aggregate admission. No push/release/install.
