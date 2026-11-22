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
the remaining slicer data through public Rust operations. Physical-spectrum/role
views and complete catalog/sweep parity remain API-02 gates before Qt adoption.
Traceability:
[architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-16),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-02-selected-receiver-cell-unit)
and [parent task](../../development/checklist.md#task-uiu-api-02).
