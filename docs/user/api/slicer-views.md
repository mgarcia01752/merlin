# Reference and soft-value views

These passive downstream operations provide scientific reference geometry and
actual likelihood summaries through `merlin::downstream` and Python
`merlin.downstream`. They return owned values and never start a PHY/source or
change a session. Native work releases the interpreter lock.

| Rust | Python | Result |
| --- | --- | --- |
| `Simulation::constellation_reference(order, amplitude, start, count)` | `Simulation.constellation_reference(order, amplitude=1.0, start=0, count=0)` | Up to 4096 original labelled reference points. |
| `Simulation::decision_region(order, label, amplitude)` | `Simulation.decision_region(order, label, amplitude=1.0)` | Finite center-relative nearest-reference region and nearby labels. |
| `result.cell_soft_view(query)` | `result.cell_soft_view(query=None)` | Actual post-descrambling soft values from the explicit selected-cell window. |
| `result.decoder_soft_view(snapshot)` | `result.decoder_soft_view(snapshot=0)` | Actual retained coding-order LDPC input with original context/extent. |

Reference orders are 2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192 and 16384.
The 2/4-point tables are control references. Unsupported orders reject. Reference points do
not establish a supported complete coding chain or modem capability. `amplitude`
must be finite and positive; coordinates that cannot preserve their finite
nonzero scale reject. Returned I/Q already includes that known multiplier.
`normalization_scale_factor` describes the original unit-energy table and is not
another multiplier for the returned coordinates.

Reference pages preserve absolute binary labels. `start` must identify an existing
label; count zero copies up to 4096 remaining points. A positive count is capped
at 4096 and may return fewer points when the table ends. No points are repeated or
interpolated. Caller edits cannot change the original static tables.

Decision regions use every original reference as a Euclidean half-plane
constraint. The result is intersected with an explicit square viewport whose
half extent is 2.5 times the nearest-neighbor spacing. `center_iq` is absolute
reference I/Q; `vertices_relative` and `nearby_references[].relative_iq` are relative
to that center, with the same known amplitude. At most 49 nearby references and
64 polygon vertices are returned. This finite view does not claim a bounded
outer Voronoi cell. Geometric boundaries are closed; exact sample ties retain
the existing slicer's `lowest_label` rule. Production soft decoding is unchanged.

```rust
use merlin::downstream::{FullLoopCellQuery, Simulation};
let references = Simulation::constellation_reference(128, 1.0, 0, 128)?;
let region = Simulation::decision_region(128, 0, 1.0)?;
// result is an already owned FullLoopResult with explicit stage retention enabled.
let cells = result.cell_soft_view(FullLoopCellQuery::default())?;
let decoder = result.decoder_soft_view(0)?;
```

```python
import merlin.downstream as m
references = m.Simulation.constellation_reference(128, count=128)
region = m.Simulation.decision_region(128, 0)
cells = result.cell_soft_view()
decoder = result.decoder_soft_view()
```

Cell soft views use [actual receiver selections](full-loop-cells.md): at most
2048 displayed cells contribute their original post-descrambling LLRs. Sequence
indices describe this explicit selected sequence, independently of a global
codeword. Protected streams keep empty LLRs. Decoder soft views require an actual
retained `rx.bits.inverse` event and preserve its original codeword context,
element offset and total. A truncated event remains shorter than its original
total; absent coverage rejects. No missing values or decoder inputs are generated.

Both views keep the original `log P(0)/P(1)` convention and existing likelihood
statistics. Fixed display histograms have 40 equal-width bins over (-100,100)
and (-10,10), with open outer endpoints. Interior lower bounds are inclusive;
upper bounds are exclusive. Direct comparisons against bin edges preserve tiny
signed values and adjacent floating-point values. Full-range endpoint counts
describe values at or beyond ±100, independently of histogram visibility.
At most 1024 contiguous original-index groups retain their exact extrema.
`sequence_abs_limit` is a display bound of at least one, not a fabricated measured
LLR. These are descriptive summaries; no probability calibration, empirical BER
or new decoder input is inferred.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-19),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-02-slicer-reference-and-soft-view-unit)
and [task](../../development/checklist.md#task-uiu-api-02).
