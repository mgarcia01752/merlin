# Session retention accounting decision

Status: approved clarifications, 2026-11-23 and 2026-11-24;
complete implementation acceptance is pending.
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

<a id="output-construction-boundary-requiring-clarification"></a>
## Approved output construction boundary

The approved three quantities above remain unchanged. A separate source audit
finds that existing final-product length ceilings do not establish requested
capacity ceilings for Vec/String construction. Do not assume a standard-library
growth multiplier or reject only after producing an oversized result.

The concrete proposed archive path preserves the existing `receiver_archive`
operation and its historical statistic. An additive public preflight operation
would compute all three target quantities; a paired bounded copy operation would
check them before copying. Normalize copied storage through boxed slices/strings
to give final retained buffers exact logical capacities. Retain original source,
old history and target reservations concurrently until their actual final drops.
Independent Rust/native tests must verify unchanged observations and statistics.
No bound/copy operation is implemented or accepted by this proposal.

| Owner | Remaining source-level gap |
| --- | --- |
| `crates/ofdm/src/api/full_loop/receiver_archive/copy.rs` | Selected pair/LLR/context copies have finite lengths; collect/clone capacities are not part of the legacy byte statistic. Final archive normalization can establish a target bound, but boxed collection itself uses a temporary builder. |
| `crates/ofdm/src/simulation/channel/job.rs`, `api/full_loop/observations/store.rs` | Future results retain word/constellation/event buffers, cloned settings/inputs and failure text; complete typed accounting and capacity enforcement are still needed. |
| `crates/ofdma/src/simulation/run.rs`, `instruments/{observer.rs,receiver.rs,soft/store.rs}`, `configuration/document.rs` | Future reports retain grant/decision/observation buffers, cloned snapshots, canonical JSON and diagnostics; all capacities and private caches need owner enforcement. |

The user selected **Retained owners; separate execution limits** on 2026-11-23.
This clarifies the earlier preparation-peak wording without accepting runtime
admission. The selected boundary is:

1. **Approved: retained-root accounting with separate execution policies:** the aggregate
   byte/entry/root budget covers all accepted, candidate, worker, queued and old
   retained roots and simultaneous copy source/target owners. Temporary backend
   construction and numerical scratch remain subject to separate finite input,
   element/count, worker and runtime policies; no aggregate scratch-byte/RSS
   guarantee is claimed. Final retained capacities still require enforcement.
2. **Unselected: one aggregate budget includes construction peaks:** additionally prove and
   reserve every temporary builder's requested-capacity peak before allocation.
   Existing length checks and boxed final-storage normalization do not establish
   this stronger claim; further backend storage/admission work is required.

Prepared accepted/candidate objects, cloned inputs, queued products and simultaneous
copy source/target owners remain aggregate retained ownership, even while a worker
is preparing them. Temporary backend output builders and numerical scratch have
separate finite execution policies; they are not an aggregate scratch-byte/RSS
guarantee. Enforce final retained capacities before publishing an owner. Never
use this distinction to admit an oversized final result and reject it afterward.

Public root metadata and bounded-owner implementation may proceed. Actual host
admission and its complete compatibility/lifecycle tests remain acceptance gates.
Keep existing scientific limits and tests; establish explicit finite construction
limits where the owning backend does not already enforce them.

## Approved presentation and channel boundaries

On 2026-11-24 the user selected both following clarifications for the remaining
SESSION presentation/transport owners:

- **Persistent QML:** use explicit logical UTF-16 byte limits, array/object entry
  limits and open/retained-view population limits. Rust retains its actual-capacity
  accounting. Hidden/closed views that keep data must continue covering that data
  until it is cleared or disposed; visibility alone does not release ownership.
  Logical presentation quantities do not claim Qt JavaScript allocation capacity,
  engine object headers, total heap or RSS.
- **Standard channels:** charge message data and explicit channel/slot counts;
  exclude private standard-library synchronization bookkeeping. Reuse existing
  bounded transport, with no private queue-layout guesses or replacement transport
  required solely to measure that bookkeeping. Payload guards and channel/slot
  charges must survive queued messages, worker/sender aliases and disconnection
  until their corresponding owners are disposed.

These clarify [REQ-UIU-07](../../../requirements.md#req-uiu-07), the
[session architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle)
and [UIU-SESSION-01](../../checklist.md#task-uiu-session-01). They preserve the
approved Rust payload/container/shared-root quantities, temporary-execution
boundary, scientific limits and all existing acceptance gates. Implement concrete
logical presentation and channel populations with existing checked admission
primitives; do not introduce another accounting framework or infer one domain's
limit from an unrelated equal number.

Acceptance must test each added population/quantity at and beyond its bound,
replacement and source/target overlap, hidden-view retention, cancelled/stale/
disconnected delivery and final alias cleanup in actual consumers. Earlier scoped
receipts that list these questions as pending remain historical evidence; this
approval resolves the policy questions without retroactively executing tests or
accepting their remaining implementation. Complete SESSION stays unchecked.
