# Downstream observation/capture admission preparation

Source-only descendant of frozen `a92d99182acdd99ae0f455097158944280b53352`
on `feature/qt-downstream-observation-admission`. Ownership is UIU-SESSION-01,
REQ-UIU-03/06/07; no SESSION or aggregate acceptance follows this preparation.
Canonical ownership/design/checklist remain lead-owned and unchanged here.

## Interface and concrete proof

`Observations::from_result_admitted(&FullLoopResult, &RetentionBudget)` returns
`Result<Observations, ObservationError>`; errors distinguish public metadata,
retention and an unaudited future JSON representation. No scientific execution
occurs. Existing `from_result` is fixture-only, with independent test policy.

The source must already carry its backend reservation in the same canonical
budget. The existing query prerequisite is accepted primary `42d177e`:
individual exact-capacity `stage_inventory`/`stage` returns fit source payload
bytes with zero ordered entries/shared roots. Inventory owns its reservation.
Accumulator, page and replacement each have a separate source-envelope slot;
all four slots remain charged through planning and final target construction.
A concatenation is an exact-capacity prefix of the same source event, retaining
its original first context: fixed-size current data leaves and normalized
context text/roles cannot exceed that original event's charged children.
Snapshot is externally non_exhaustive: current other public fields are fixed;
future owned fields require revisiting this explicit current-field audit.

An allocation-free first pass plans actual selected typed vector lengths,
ordered map entries, warning lengths and the existing tagged JSON shape from
borrowed events. It never serializes or copies a target to discover its size.
Reserve that complete dynamic target before the second pass. Existing capture
serialization and observation selections retain original values/nulls/tags,
coordinates, phases, first/contiguous-event and PLC-frame rules. Normalize all
final Vec/String/JSON buffers through boxed storage. BTree/JSON entry payloads
include complete fixed key/value pairs and owned keys; private nodes are excluded
by contract. The existing serde_json 1.0.145 default Map/Number configuration
is required; feature/serialization shape changes require re-auditing this plan.

The private Retained<()> guard is declared after all charged buffers. Its fixed
layout is already included by the unchanged ResultData size_of calculation in
result_retention.rs; there is no second scientific fixed root or extra scientific
Arc in the projection charge. Queued/current/old ResultData aliases retain the
guard until final drop. Query wrapper deltas are separately charged. The shell's
private accounting-counter graph/Arc handle edges are outside this partial
payload proof; full host/transport metadata closure remains lead-owned.

Temporary concatenation/serialization/collection/normalization builders remain
separate finite execution storage: accepted max_events,
max_elements_per_event, max_total_elements, max_retained_bytes,
MAX_STAGE_QUERY_ELEMENTS, MAX_INSTRUMENT_TIME_SAMPLES,
MAX_TX_FRAME_SEQUENCE_SAMPLES and MAX_TRANSMITTER_MESSAGE_FRAMES bound their
inputs/populations. No RSS, allocator multiplier or preparation-peak claim.

## Exact lead integration hooks

Pass the canonical budget from worker execution into ResultData::from_retained,
then call the admitted constructor. This branch intentionally leaves that frozen
caller unchanged; production compilation requires that lead-owned wiring.
On Err, retain the same backend report and use Observations::default() plus a
static borrowed or separately admitted diagnostic. Never allocate an uncharged
warning Vec/String or reject completed science. The fixture-only constructor
must not become a production fallback. Preserve the existing refresh revision.

Only observations.rs and its children, transmitter/capture.rs and its children,
and this unique handoff are changed. No a92 worker/result/coordinator sources,
shared primitives, manifests, scientific APIs or primary files are changed.

## Validation and remaining consumers

Executed: pinned rustfmt parse/check of both owner roots and recursive children;
git diff --check; exact path/source/unchanged-scientific-body audit; guide check.
ALL compilation, Cargo, Clippy, Rust/runtime, Qt/QML, native/wheel and acceptance
checks are UNEXECUTED here, by assignment. Lead must compile/reexecute after
query prerequisite and constructor wiring; no executable delivery is claimed.

Ten prepared cases: eight-variant exact JSON/null/spare parity; JSON arithmetic
overflow; all phase spellings; actual result/source/coordinate parity plus
independent final-capacity walker; exact source/workspace/target overlap and
payload/entry +1 rejection with unchanged source roots; queued/current/old/final
alias lifetime; labelled synthetic presentation-selection edges; real pre-cancel
empty projection; separate real paged-query slot ownership; slot overflow/zero
policy rejection. Synthetic fixtures assert presentation rules, not science.

Remaining consumers include independently requested instrument graph/table/JSON
and receiver-query outputs, detached history/archive targets, bridge/QML/native
transport, host/control/accounting metadata and any independently escaping copy.
This unit covers the stored Observations/Capture graph only. The ignored receipt
under docs/development/local/session-shell/downstream-observation-admission/
contains exact source hashes, audit script, commands/status and save log.
