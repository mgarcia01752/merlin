# Concrete RunTiming admission: source preparation

Exact base `ddec8882532dec2ca2dac786c43e0eabe4aa297a`; branch
`feature/qt-run-timing-admission`. Earlier frozen branches remain preserved.
This finite UIU-SESSION-01 prerequisite follows the lead-owned
[run-timing design](../design.md#uiu-session-01-concrete-run-timing-admission),
[REQ-UIU-03/06/07](../../../../requirements.md#req-uiu-03) and existing
[session lifecycle](../../../../architecture.md#unified-qt-session-and-file-lifecycle).
Lead owns canonical records, actual execution, integration and acceptance.

## Owner map and semantic reuse

EDIT `downstream/simulation/run_timing.rs`: reuse existing State, MAX_PROFILE_TIMINGS,
clock labels/formatting/progress/freeze and Mutex behavior. Add concrete PreparedTiming
with one private RetentionReservation; it starts no clock. RunTiming::prepare admits
`sizeof(RunTiming) + MAX_PROFILE_TIMINGS * sizeof((u8,u64))`, checked, with zero ordered
entries and one shared root. The final RunTiming owns State before a mandatory
Retained<()> guard. Exact boxed-slice-to-Vec normalization followed by clear leaves
all existing capped pushes within admitted capacity, without guessed growth.

EDIT `closed_loop.rs`: prepare timing before application.begin, performance.begin
or accepted status clearing; consume the token at the original clock-construction
location. Keep Arc<RunTiming> and production Worker unchanged. Old timing survives
candidate rejection; GUI and actual execution share one charge through final drop.
No independent charge for Mutex/State/Vec handle, static phase text, existing
counter/gate/Stores/controls or inline outer Worker storage is added.

ADD the two adjacent timing-retention test children. EDIT only the five authorized
request/result/projection/worker fixture files for constructors and actual timing
allowances/lifetimes. Preserve all original numerical/scientific/clock assertions.
All preparation tokens are single-use; every fixture uses its actual relevant budget.

## Validation boundary and remaining owners

All compilation, Cargo, Clippy, runtime, Qt and native checks are UNEXECUTED here.
Only pinned rustfmt and source/allowlist/body checks run in this isolated worktree.
No pending channel policy is needed for this one fixed-capacity timing payload.
Timing snapshot JSON, outer Model/Worker/transport and other presentation owners
remain separate concrete consumers; this preparation does not establish full
aggregate or official SESSION acceptance. Lead runs affected real lifecycle,
pressure, timing, Qt/QML and executable validation after adopting the source.


## Source receipt and prepared cases

Six new cases, all UNEXECUTED: exact/below positive byte/root admission and token
rollback; multiplication/addition overflow; exact initial/final vector capacity,
unchanged stored IDs and no growth beyond the existing cap; delayed deterministic
epoch/freeze with final aliases; old/candidate overlap and rejected replacement;
real Model Start pressure preserving old result/timing/request/plan/token/generation/
performance history; genuine paused stale cancellation and final disposal. The
first four owner tests combine these checks; two Model tests execute the real path.
After cancellation the worker may already have returned: a Weak upgrade is checked
only if successful, while that observed alias is held, then dropped before terminal
polling. Direct owner tests separately prove deterministic alias retention.

Existing second-Start result fixtures include both previous and candidate timing
charges and explicitly assert actual worker dispatch before their original target
rejection. Queue fixtures hold a real timing alias for exact live assertions, then
drop it at final cleanup. Profile-control pressure admits timing before exhausting
roots. No original clock/scientific assertion or test case is removed.

Executed: pinned Rust 1.85.1 rustfmt --check on nine Rust files; diff --check; exact
ten-path/base/empty-index/text-source/test-name audit; unchanged clock algorithm
bodies and byte-identical production Worker; all constructors require admission.
The generated integration guide --check reports current. Compilation, runtime,
Cargo, Clippy, Qt/native and full regressions remain UNEXECUTED. Final local receipt
reports commit/parent, source hashes and clean tree. No runtime artifacts produced.
