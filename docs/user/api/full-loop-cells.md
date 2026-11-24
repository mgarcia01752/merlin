# Downstream receiver cell observations

Use an owned [full-loop result](full-loop.md) through
`merlin::downstream` or Python `merlin.downstream`. Its actual payload/NCP and
protected-stream captures are independent of raw test-point retention. The default
full-loop options enable those finite captures. Disabling them produces explicit
empty cell views; querying never starts a run or reconstructs missing history.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `result.qam(FullLoopCellQuery::default())` | `result.qam()` | Up to 2048 actual matched cells and scientific MER over every selected retained cell. |
| `result.cell_decisions(FullLoopCellQuery::default())` | `result.cell_decisions()` | Up to 128 physical diagnostic decisions and actual selected-cell soft statistics. |
| `FullLoopCellQuery::from_json(bytes)` | `FullLoopCellQuery.from_json(data)` | Strict query admission, at most 4096 bytes; result queries revalidate typed edits. |

Rust accepts an editable typed query by value. Python owns an immutable
`FullLoopCellQuery(stream="payload")`; its `settings` property returns a copy.
Use `from_json` for optional filters and pagination. No Python numerical work is
performed, and numerical queries release the interpreter lock.

```rust
use merlin::downstream::{FullLoopCellQuery, FullLoopCellStream};

// result is an already owned FullLoopResult.
let qam = result.qam(FullLoopCellQuery {
    stream: FullLoopCellStream::Payload,
    count: 128,
    ..Default::default()
})?;
let pilots = result.cell_decisions(FullLoopCellQuery {
    stream: FullLoopCellStream::ContinuousPilots,
    count: 16,
    ..Default::default()
})?;
```

```python
import merlin.downstream as m

qam = result.qam(m.FullLoopCellQuery.from_json(b'{"count":128}'))
pilots = result.cell_decisions(m.FullLoopCellQuery.from_json(
    b'{"stream":"continuous_pilots","count":16}'))
```

The six streams are `payload`, `ncp`, `plc_data`, `plc_preamble`,
`continuous_pilots` and `scattered_pilots`. Optional `profile_id` and
`modulation_order` filter actual cells. None/null order selects the first actually
available order after profile filtering. `first_bin`/`last_bin` and
`first_symbol`/`last_symbol` are optional inclusive physical-bin/channel-symbol
bounds. Reversed ranges, bins above 8191 and unsupported reference orders reject.
Coordinates describe the received physical grid; source-symbol identities remain
separate and are absent where the original protected owner supplied none.

`start` is an offset inside the filtered population. `count` selects up to the
operation's cap, with zero selecting up to that cap. A request may return fewer
cells when the retained population ends; it never invents trailing samples.
An offset beyond the selected population rejects. An offset at its end returns
an empty page. Original retained counts, profiles, available orders and coordinate
bounds remain separate from filtered and displayed counts. Payload profile IDs
describe the actual wire profile; the applied source-profile identity remains in
`result.inputs().summary()` / `result.input_summary()`.

QAM views preserve paired before/after correction I/Q, physical frequency/bin,
channel/source symbol, modulation, optional actual diagnostic TX reference and
post-descrambling receiver LLRs. Boosted pilots have known reference amplitude two;
other accepted streams use one. Protected streams carry no fabricated LLRs.

Nearest-point and matched-TX MER remain separate, for raw and corrected reference
planes. Existing scientific owners process **all** filtered retained cells before
display pagination. Nearest-point MER can be optimistic under severe noise; no
gain or phase is fitted. TX-reference MER is absent unless every selected cell has
its actual matched reference. Neither metric is the imported field RxMER reading.
Empty/perfect/invalid states retain explicit statuses and unavailable numeric
values rather than infinity or a finite substitute.

Decision rows include original cells, normalized I/Q and the existing nearest
symbol decision in unit-energy and original physical coordinates. These are
diagnostics and never replace production soft FEC input. Labels are physical
decisions before descrambling; attached cell LLRs retain their actual producer's
post-descrambling semantics. Soft statistics summarize at most 2048 selected cells
starting at the display offset, with explicit contributing-cell accounting.
They remain separate from [coding-order LDPC input](full-loop-observations.md).

[Reference/decision-region and detailed soft-value views](slicer-views.md) supply
the remaining slicer data through public Rust operations. API-02 backend acceptance
is recorded separately; Qt consumer migration remains UIU-DS-01.
Traceability:
[architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-16),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-02-selected-receiver-cell-unit)
and [parent task](../../development/checklist.md#task-uiu-api-02).

## Detached independent histories

Use `result.receiver_archive(policy)` when retaining several independent runs.
Cloning an entire result keeps its job, original inputs and unrelated stage data
alive. A receiver archive copies only finite actual cells and one decoder-event
prefix; its clones share those copies. It remains usable after all requests,
results and capture handles are dropped.

```rust
use merlin::downstream::{FullLoopCellQuery, FullLoopReceiverArchiveInput,
    FullLoopReceiverArchivePolicy};

let policy = FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput::default())?;
let history = result.receiver_archive(policy)?;
drop(result);
let view = history.qam(FullLoopCellQuery::default())?;
let accounting = history.info();
```

```python
import merlin.downstream as m

policy = m.FullLoopReceiverArchivePolicy()
history = result.receiver_archive(policy)
del result
view = history.qam()
accounting = history.info
```

The default policy retains at most8192 payload cells,1024 NCP cells,1024 cells
per protected stream and16200 decoder LLRs, with an independent eight-MiB copy
budget. Each count may be reduced to zero; Rust admits strict JSON through
`FullLoopReceiverArchivePolicy::from_json` / `.from_json`. A smaller byte budget
can reject the complete copy before any observation allocation. `retained_bytes`
counts the archive's fixed storage and actual copied nested cell/likelihood/context
payload, excluding allocator/control-block overhead and engine buffers. Applications
still need independent aggregate and run-count budgets.

Cell reduction uses original index floor(k×N/min(N,cap)). It keeps actual physical
coordinates and may leave gaps. MER uses every filtered **archived** cell before
pagination; it cannot describe omitted original cells. QAM, decisions and
cell-soft queries use the same typed selectors and numerical owners as live
results. Original raw/equalized, nearest-point/matched-TX and physical-cell/coding-
order planes stay distinct. Missing streams remain empty.

The decoder view retains the first actual nonempty retained payload LDPC-input
event's finite prefix. `info` reports its original snapshot index, coding context,
element offset and original total. `history.decoder_soft_view()` takes no event
selector because an archive contains at most that one event. Missing or deliberately
omitted decoder input raises MissingCoverage; it is never replaced by cell LLRs.
These views are diagnostics and do not rerun or modify decoding.

Traceability: [archive design](../../development/features/unified-qt-workbench/design.md#uiu-ds-01-bounded-receiver-archive-prerequisite),
[public boundary](../../architecture.md#unified-qt-public-api-boundary) and
[downstream migration](../../development/checklist.md#task-uiu-ds-01).


For aggregate retention admission, use the additive preflight/copy pair:

```rust
let required = result.receiver_archive_retained_bound(&policy)?;
// Reserve required against the application's aggregate budget before copying.
let history = result.receiver_archive_bounded(policy, required)?;
```

```python
required = result.receiver_archive_retained_bound(policy)
# Reserve these quantities against the application's budget before copying.
history = result.receiver_archive_bounded(required, policy)
```

The bound includes fixed archive storage and exact final nested buffer capacities,
with zero ordered entries and one detached shared root. Each insufficient caller
limit rejects with ResourceLimit before observation copying. The original result,
old histories and simultaneous source/target copies remain separately charged;
passing the returned value does not itself make a reservation. Temporary copy
builders have separate existing finite role/event/byte limits, without an aggregate
scratch-byte or process-memory guarantee. Scientific queries, selection, missing
coverage, coordinates and the old info.retained_bytes statistic keep their meaning.

Native Python can construct immutable `merlin.common.RetainedFootprint` limits
from nonnegative target-usize quantities, mirroring Rust's public struct literal.
Omitted ordered-entry/shared-root counts are zero. Quantities are caller policy;
Python does not calculate scientific storage. See
[retained resources](retained-resources.md) and the
[pre-production archive design](../../development/features/unified-qt-workbench/design.md#pre-production-detached-archive-bounds).
