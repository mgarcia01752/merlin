# Receiver registry storage preparation

This source-only unit belongs to
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07). It closes no official
acceptance gate. Base: `1921ba95dd2a93caa40540535f9a5189af0d490c`;
branch: `feature/qt-history-registry-retention`.

The existing [archive](../design.md#uiu-session-01-detached-receiver-history-admission)
and [performance](../design.md#uiu-session-01-copied-receiver-performance-admission)
contracts leave concrete registry storage as a separate consumer. Their scientific
and live-entry admission bodies remain unchanged in this unit.

## Exact files and contracts

Only these new source files and this handoff are added:

- `apps/qt/src/downstream/instruments/receiver/qam_history/registry_retention.rs`
- `apps/qt/src/downstream/instruments/receiver/qam_history/registry_retention/tests.rs`
- `apps/qt/src/downstream/instruments/receiver/performance/registry_retention.rs`
- `apps/qt/src/downstream/instruments/receiver/performance/registry_retention/tests.rs`

Each unregistered module exports to its parent:

```rust
pub(super) fn resources(
    entries: &Vec<super::Entry>,
) -> Result<crate::shell::RetainedResources, crate::shell::RetentionError>;
```

The QAM visitor counts `capacity * size_of::<Entry>()` for every live and spare
slot, plus each live `status.capacity()`. Exhaustive Entry destructuring audits
`id`, `status` and `snapshot`. The optional Arc handle is inline in its slot;
the Snapshot's existing guard already covers its full archive, wrapper and outer
shared root. The visitor does not clone an Arc or count that pointee again.

The performance visitor counts only
`(capacity - len) * size_of::<Entry>()`. Existing `PreparedEntry::admit` already
charges the full live Entry layout and all status/points/error/FEC-JSON children.
The exhaustive audit includes `id`, `channel`, `status`, `fec`, `ber`, `pending`,
`points`, `error`, `fec_details` and `_retained`; future fields require review of
both that admission and this boundary. No live fixed bytes or children are counted
again. No scientific data, JSON values, storage capacities or guards are modified.

Both visitors return zero ordered entries and shared roots. Their arithmetic is
checked. Fixed Vec handles and the surrounding Store/Context/accounting graph are
separate infrastructure consumers, not exclusions from the aggregate contract.

## Lead integration hooks

Register `mod registry_retention;` in the two existing owning parent files, then
forward their borrowed registry Vecs to `registry_retention::resources`. These
modules deliberately do not register themselves or change callers. The lead owns
actual infrastructure reservations and transactional registry growth/replacement;
this passive query alone does not provide pre-production admission. Preserve old
and candidate owner overlap and existing Snapshot/live Entry guards through all
aliases and final data disposal. Never add these quantities to a second full
Vec/live-entry charge without subtracting the already-covered portion.

## Source-prepared tests and evidence

Two focused cases are included, both **UNEXECUTED**:

1. QAM: a real supported short public run produces an admitted detached Snapshot.
   Actual spare slots and spare status capacity are counted exactly; repeated
   queries preserve status, IDs, scientific metadata, Arc identity/strong count
   and existing budget usage. Status growth uses actual measured capacity, clear
   retains spare slot storage, and an old Snapshot alias keeps its guard until
   final drop. The scientific run's preparation is only a fixture, not an aggregate
   source/target admission claim.
2. Performance: an actual admitted failed Entry preserves the full original
   diagnostic. Only spare slots are reported; moving the Entry out makes one
   additional slot spare while its existing live-entry charge survives registry
   disposal and releases only on the Entry's final drop.

Pinned Rust 1.85.1 rustfmt and source/allowlist/diff checks are the only local
validation. Compilation, Clippy, Rust/Qt/QML/native runtime checks and official
SESSION acceptance are unexecuted here and remain lead-owned. No scientific,
public API, serialization, registration, Context or shared accounting changes are
included. Existing source-editor and earlier preparation branches are preserved.

Focused save command (executed only after complete scope review):

```bash
cd /home/dev01/Projects/merlin-worktrees/qt-session-shell &&
tools/git/git-save.sh --commit-msg "Feature: Measure receiver registry storage
- Count QAM statuses and performance spare slots
- Preserve existing charged scientific owners
- Tests: source checks; runtime unexecuted"
```
