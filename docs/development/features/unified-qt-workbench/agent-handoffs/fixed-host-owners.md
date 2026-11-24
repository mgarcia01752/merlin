# Fixed host allocation owners: source preparation

Base: `99991d6df99d613927de6686e059ba6c35b791e0`.
Branch: `feature/qt-fixed-host-owners`; the prior fixed-Store branch is preserved.
This is a source-only UIU-SESSION-01 prerequisite under
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), following the
[session lifecycle](../../../../architecture.md#unified-qt-session-and-file-lifecycle)
and [approved retention contract](../retention-contract-decision.md).
Lead owns canonical design, final host wiring and runtime acceptance. No official
SESSION acceptance follows from this preparation.

## Concrete design and file map

- EDIT `apps/qt/src/shell/retention.rs`: add application-only
  `RetentionBudget::for_application(maximum) -> Result<Self, RetentionError>`.
  Validate exact `sizeof(Mutex<Accounting>)`, zero ordered entries and one shared
  root before Arc allocation; initialize `used` to that baseline. No reservation
  is stored inside its own counter, avoiding a strong-reference cycle. The
  baseline lasts until final Accounting destruction. Generic `new`/`with_limits`
  keep their original zero baseline and existing tests unchanged.
- EDIT `apps/qt/src/shell/dispatch.rs`: concrete private GateStorage owns the
  existing `Mutex<Option<Occupied>>` followed by an optional reservation. Add
  `DispatchGate::for_application(&RetentionBudget) -> Result<Self, RetentionError>`
  reserving `sizeof(GateStorage)`, zero entries and one root before Arc allocation.
  Default remains an unguarded generic fixture gate. Application integration must
  use the admitted factory. Gate clones and RunLease share the same last guard;
  data/control destruction precedes reservation release. Preserve every existing
  cancellation, pause, stale-token, acknowledgement and lock-order transition.
- ADD focused `retention/application_tests.rs` and
  `dispatch/retention_tests.rs`; preserve all existing tests/assertions.
- ADD only this handoff as the fifth path. No framework or scientific/API change.

The counter retains no reservation to itself. A gate reservation points to its
counter, while the counter holds no gate/host edge. A lease therefore keeps gate
and counter ownership alive after host destruction without forming a cycle.
Private refcount headers/allocator bookkeeping stay outside the approved payload
quantity. Actual concrete mutex/wrapper layouts are counted, not estimated.

## Prepared checks and acceptance boundary

Executed pinned rustfmt/format and diff-whitespace checks, exact five-path/parent
and raw-NUL scans, and source comparisons proving generic constructors/accounting,
all existing dispatch transitions and original inline tests unchanged. All five
original gate locks now address only the nested state. Default callsite review
found production ApplicationSessions still needs the lead factory replacement;
all other existing Default callsites are fixtures. The generated guide is current.

Compilation, Cargo, Clippy and **all six new runtime cases are UNEXECUTED**:

1. Counter exact layout/root admission and one-below independent rejections.
2. Baseline survival through payload/budget/reservation aliases, then final Weak
   destruction proving no self-reference cycle.
3. Both generic constructors keep their original zero baseline.
4. Gate exact layout/root admission, shared clone/final-drop lifetime, and the
   original unguarded Default shape.
5. Rejected gate byte/root admission preserves an existing application baseline.
6. A genuine gate-issued fixture lease remains Busy through acknowledged pause
   and cancellation after ordinary gate handles drop. Both normal acknowledgement
   and final disposal of an unacknowledged fixture destroy control while its guard
   remains charged; no scientific worker or reusable abandoned slot is claimed.

## Lead integration hooks

ApplicationSessions must use the two application-only constructors. Its sessions
Arc still needs its own retained wrapper: subtract the two existing inline
readiness Retained layouts from that wrapper's fixed charge using checked
arithmetic, preserving readiness candidate reservations. Final host total has
three shared roots and zero entries before any Context Stores/children.

Host drop must still cancel through ShellSessions::shutdown while the genuine
worker lease retains the gate. Normal terminal disposal, failure, panic and
unstarted spawn failure retain their existing acknowledgement responsibilities.
Final host/transport tests and application fixtures remain lead-owned. Control
pointees, closure/transport/presentation owners remain separate pending consumers.
No build/runtime artifacts are produced. Final save receipt supplies the exact
five paths, commit/parent and source SHA-256 hashes.

## Application-owner fixture supplement

Source parent: `c3c5f7fda4e3b1c11cccff25a1264f88577927ad`, same branch.
This follow-up changes only this handoff and the new
`apps/qt/src/shell/application/retention_tests.rs`. Lead owns all production host
integration and registers `#[cfg(test)] mod retention_tests;` in application.rs.
The tests require the agreed production private constructor
`ApplicationSessions::from_budget(RetentionBudget) -> Result<Self, ApplicationError>`
and sessions `Arc<Retained<Mutex<Sessions>>>`; neither is edited here.

Seven additional runtime test functions are prepared, all **UNEXECUTED**, including
compilation/Clippy. Pinned source formatting, diff-whitespace, exact two-path/parent
and raw-NUL reviews are the only performed checks. No runtime/build artifacts.

1. Independently measure the counter and gate via their constructors, then add the
   full Sessions wrapper layout once. Exact admission requires three roots/zero
   entries; below-byte and every insufficient root limit reject. This oracle does
   not repeat the production readiness-field subtraction.
2. Retain the real counter observer through from_budget rejection at gate, each
   readiness guard and final sessions byte prefixes, plus partial root prefixes.
   Every failed constructor restores precisely its initial Accounting baseline.
3. Exact idle capacity rejects all three readiness replacement operations while
   preserving both snapshots, admission state and the full original charge.
4. Host clones share the same sessions Arc; final sessions Weak destruction leaves
   only the Accounting baseline when a counter observer remains.
5. Genuine spawn_worker success, typed error and caught panic keep the gate charged
   after downstream/upstream host destruction from acknowledged paused or already
   cancelling phases. The terminal retains Busy until known-completion disposal.
6. Both directions and phases also cover receiver disconnect before release of the
   actual worker operation. Known terminal disposal acknowledges only after return;
   the final explicit gate alias remains charged until it and the worker alias drop.
7. An invalid static thread label returns the original unstarted genuine lease in
   both directions. This proves label rejection, not actual OS spawn failure.

The matrix prepares 16 real worker spawns and two pre-spawn label rejections.
Control counters, destruction signals and worker synchronization channels are
explicit fixture instrumentation, not complete control/channel/closure memory
acceptance. The lifecycle pause acknowledgement is application metadata only;
these tests establish no backend pause or scientific run acceptance. A named
five-second test-only watchdog bounds waits/final disconnect disposal. Generic
constructors and all prior source/tests stay unchanged. Lead executes actual
combined host/transport/Qt validation and owns final adoption and acceptance.
