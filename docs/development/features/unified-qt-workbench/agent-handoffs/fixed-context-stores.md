# Fixed Context Stores: source preparation

Base: `dbc7e95c1a532bf9ae67666a6998b646509cd177`.
Branch: `feature/qt-fixed-context-stores`; prior `cca164c` preserved.
Owner: UIU-SESSION-01, REQ-UIU-03/06/07. Canonical decision:
[fixed downstream Stores](../design.md#uiu-session-01-canonical-fixed-downstream-stores).
This preparation establishes no runtime or SESSION acceptance.

## Exact interface and scope

- `store.rs`: `Store::new(value, &RetentionBudget) -> Result<Self, RetentionError>`
  reserves `sizeof(Retained<Mutex<T>>)` and one shared root before allocating
  `Arc<Retained<Mutex<T>>>`. It consumes zero ordered entries. Dynamic `T`
  children require their separate existing admissions; the inline Store handle
  belongs to its containing owner. Clone shares the original guard. Data drops
  before the reservation at final alias disposal. Default Store is removed;
  borrow/poison recovery behavior stays unchanged. Test-only downgrade now returns
  `Weak<Retained<Mutex<T>>>`.
- `session.rs`: `Context::for_application(ApplicationSessions) ->
  Result<Self, RetentionError>` constructs five independent empty Stores on that
  exact host counter. It creates no host or identity and changes no readiness.
  A later rejection drops every earlier Store. `isolated` constructs the existing
  desktop host and delegates to this factory with an explicit expect; default
  keeps its existing thread-local sharing behavior. Field map and payload defaults
  are unchanged.
- New focused tests: `store/retention_tests.rs`, `session/fixed_tests.rs`.
- This handoff is the fifth and final changed path.

## Source checks and prepared cases

Executed only pinned rustfmt/format check, diff whitespace review, exact-path,
original-inline-test preservation and raw-NUL source scans. No runtime command
was invoked. Compilation, Cargo, Clippy and **all five new runtime cases are
UNEXECUTED**, delegated to primary integration:

1. Exact Store bytes/root admission with zero entries, independent byte/root
   rejection, original read/write behavior and clone/final-drop charge lifetime.
2. Payload Drop observes the charge still live; the final alias releases the
   allocation and reservation, while Weak does not keep the charge alive.
3. Exact five-Store layout/root admission preserves both direction snapshots and
   readiness and produces all original empty defaults.
4. Pressure leaves each possible insufficient root count; every partial prefix
   rolls back. A byte ceiling one below the actual full layout also rolls back.
5. Independent Contexts share canonical host identities but have different Store
   Weak pointers; clones share all five pointers and guards through final drop.

Fixtures derive bytes from actual layouts and host baselines, with no assumed
allocation size or invented ordered-entry demand. No existing test/assertion or
scientific/JSON body was edited.

## Lead integration gates

Lead owns project-load ApplicationSessions capture, explicit-context model/decode
constructors and boxing the three already charged queued editors. Boxing must land
with this Store change to avoid overlapping inline editor/layout charges. The
actual `PreparedModels` layout is measured automatically after that integration.
Lead also updates test fixtures that replace only `context.application` and existing
PreparedProject callers; these are deliberately unchanged here.

Keep independent candidate Stores until each model is taken, then attach target
Context, avoiding the target Store/model cycle. Application sessions/gate/counter,
control/closure/transport and presentation owners remain separate pending work.
Preserve all affected lifecycle/project/scientific assertions and execute the
relevant Qt Rust, Clippy, QML and desktop checks in primary before acceptance.

Source receipt: the save report supplies the exact commit, parent, five paths and
SHA-256 hashes; repository contents are the reproducible source artifact. No build
or runtime artifacts were created.
