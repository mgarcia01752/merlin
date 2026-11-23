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
