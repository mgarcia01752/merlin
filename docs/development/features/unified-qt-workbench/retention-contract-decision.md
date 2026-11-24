# Session retention accounting decision

Status: approved clarification, 2026-11-23; implementation acceptance is pending.
Owner: [UIU-SESSION-01](../../checklist.md#task-uiu-session-01),
[REQ-UIU-07](../../../requirements.md#req-uiu-07),
[session architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle).

## Demonstrated consumer gap

The original [ownership design](design.md#uiu-session-01-explicit-ownership-foundation)
required accounting for every reachable retained allocation before admission.
The approved contract below clarifies the quantities being bounded. Existing
public counts do not expose cached playback arrays, unused
vector capacities or private retained observation stores. Counting serialized JSON
also misses skipped scientific ownership such as upstream role-gain arrays.

Actual roots include downstream requests/sweeps/results/archives and upstream
configured sessions/results, plus independently retained accepted input owners.
Reducing them to restricted public views would remove existing arbitrary
viewport/history queries and retained source provenance. Preserve those behaviors.

## Approved explicit contract

The user selected **Bound data bytes and container counts**. This is the current
retention contract for REQ-UIU-07; it does not establish runtime acceptance.

Bound three independent aggregate quantities:

- Retained payload bytes: fixed owned values, requested Vec/String/Box/slice
  storage including spare capacity, nested cached data and reachable shared
  payloads. Conservatively repeat a shared payload when roots cannot deduplicate.
- Retained ordered-container entries: aggregate tree entries whose private
  node layout is owned by the Rust standard library.
- Shared allocation roots: aggregate Arc/reference-counted owner edges, whose
  private reference-count headers are owned by the Rust standard library.

Do not describe the byte field as total heap allocation or process RSS. Explicitly
exclude allocator, reference-count and tree-node bookkeeping from that field.
Keep the excluded container/root populations bounded independently. No private
standard-library layout constants, unsafe allocator instrumentation, scientific
algorithm changes or altered historical retention statistics are introduced.

An additive typed public footprint operation belongs to each scientific root's
existing crate. Prepared request/session output bounds and archive preflight must
cover complete, partial and cancelled retained products before production. Rust
and installed native Python must agree; Qt only consumes the public metadata.
An unavailable bound rejects admission explicitly rather than guessing.

Reservations must cover accepted, candidate, worker, queued and replaced owners
until actual final drop. Presentation copies remain separately charged. Failed
admission preserves accepted configuration/results; cancellation does not release
either execution admission or retained ownership prematurely.

## Unselected alternative: retain the stronger byte claim

Keep the current total reachable-allocation byte wording. Session acceptance then
remains pending a safe, portable way to account for private standard-library
container/refcount allocation layouts as well as all payloads. Logical counts or
a guessed multiplier cannot satisfy that stronger claim. Do not silently change
scientific containers, add unsafe/foreign allocator instrumentation or mark the
task accepted merely to close this gap.

## Reconciliation and acceptance

Reconcile architecture, REQ-UIU-07, session design, RetainedBytes/budget contracts
and checklist acceptance additively, preserving IDs and completed states. The scientific and compatibility
acceptance gates remain unchanged. Test each aggregate dimension at/beyond its
limit and arithmetic overflow, including queued/old aliases, rejected replacement,
cancel/pause, panic and destroyed owners. Complete actual two-direction integration
and Level 2 acceptance before checking UIU-SESSION-01.

Public scientific root metadata, pre-production bounds and actual direction-host
admission remain implementation gates. Neither the decision nor reservation
primitive tests establish aggregate resource acceptance.
