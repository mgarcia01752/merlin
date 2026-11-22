# Retained downstream stage observations

The public downstream API can retain bounded copies of actual core test-point
values. Capture is opt-in: prepare an ordinary FullLoopRequest, then create a fresh
request with `with_observations`. Accepted scientific settings and the original
request remain unchanged. Capture never starts until explicit `run`.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `FullLoopObservationPolicy::new(FullLoopObservationInput)` | `FullLoopObservationPolicy()` for defaults | Validated immutable supported-stage/event/element/byte policy; construction runs no science. |
| `FullLoopObservationPolicy::from_json(bytes)` | `FullLoopObservationPolicy.from_json(data)` | Same strict policy admission, at most 64 KiB JSON. |
| `policy.settings()`; serde | `.settings`; `.to_json()` | Exact accepted selections and limits; native dictionaries are independent copies. |
| `request.with_observations(policy)` | `request.with_observations(policy)` | Fresh request with validated passive capture; original request stays capture-disabled unless already configured. |
| `request.observation_policy()`; `result.observation_policy()` | `.observation_policy` on request/result | Frozen original retention policy, or None; results retain it after request disposal. |
| `result.stage_inventory()` | `result.stage_inventory()` | Registered retained IDs with actual event/element counts. |
| `result.observation_stats()` | `result.observation_stats()` | Owner-accounted bytes/events/elements and byte-budget dropped-copy counts, distinct from core dispatcher statistics. |
| `result.stage(FullLoopStage, snapshot, FullLoopStageViewport)` | `result.stage(stage, snapshot=0, start=0, count=0)` | Owned actual contiguous data/context window with original offsets/totals; at most 4096 copied values. |

Rust exposes the existing registered IDs through `FullLoopStage`; Python uses
their original serde strings. Discover configured IDs from policy settings and
available events from the result inventory. The default policy selects 22 existing
TX source/coded/interleaved/label/mapped/physical/PLC/pilot/waveform and channel/RX/
LDPC-input stages. Some are mode-dependent; an unproduced stage stays unavailable.
No ideal values or reconstructed placeholder data fill missing observations.

Default selections preserve existing workbench caps: one event per ordinary stage,
64 PLC messages, 12 channel-output blocks, and four TX overlap/RX framed/H events.
Most retain observation phase only; PLC message/IQ/preamble retain their original
warmup/observation/drain tags. Each selected stage permits 1..64 events, with at
most 32 unique supported IDs. Global limits are 1..10000 events, 1..16384 selected
elements per event, 1..1048576 total selected elements and 1..96 MiB owner-accounted
payload/context retention. These are application bounds, separate from PHY limits,
original input storage and engine/allocator overhead.

Original dispatcher selection/phase/global/per-stage budgets remain authoritative.
A copy that exceeds remaining owner bytes is dropped, increments dropped-event
statistics and sets `byte_budget_exhausted`; smaller later copies can still fit.
The flag records at least one rejected copy, rather than an exactly full allocation.
Scientific processing and completed FEC trials continue. A dispatcher cap can also stop events before the collector is called.
The reported dropped-copy count covers the byte owner only. Retention is finite
and does not promise a complete run history.

Each stage snapshot retains the original direction/domain/units, phase, source/
channel/sample/codeword/profile coordinates, reference, full grid metadata and
available transmit roles. `element_start` is the original offset, and
`total_elements` is the original producer extent. Query zero count selects up to
4096 remaining retained elements; out-of-range/overflow/count violations reject.
Unavailable stages return MissingCoverage; absent retained event indices return
InvalidIdentifier. The query never reruns a transmitter, receiver or source.

`FullLoopStageData` is a typed Rust enum; native output has `kind` and `values`.
Kinds are bits, bytes, IQ, optional IQ, QAM labels, transport cells, pilot records
and LLRs. Output models are non-exhaustive Rust contracts. Optional history and
known zeros remain distinct: frame assembly defines excluded FFT bins as explicit
zero I/Q, while None denotes unavailable history. No conversion merges them.
Pilot records come from the exact vectors computed by frame assembly and inserted
into that physical symbol; the capture hook does not regenerate pilots. Actual
pilot bins/IQ agree with the retained physical symbol and keep the original RMS
reference. A scheduled PLC message observed at construction does not by itself
prove that its entire waveform was transmitted.

Results and query windows own their data and remain valid after inputs, requests
and controls are disposed. Native queries release the interpreter lock for copying/
serialization and return independent dictionaries/lists. Original science/FEC/
source outcomes are unchanged with capture enabled or disabled within host budgets.
Original public low-level frame assembly retains its signature and no-op path.

```python
from merlin.downstream import FullLoopObservationPolicy

# base is an already prepared immutable FullLoopRequest.
observed = base.with_observations(FullLoopObservationPolicy())
result = observed.run()
print(result.stage_inventory())
window = result.stage('tx.overlap_add.samples', start=7, count=256)
print(window['element_start'], window['context']['sample_start_index'])
```

This ownership/query unit supplies actual raw observations. Numerical spectrum/
response/soft-decision preparation, decoded-message queries and full source-catalog/
sweep compatibility remain separate UIU-API-02 acceptance gates before Qt adoption.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-16),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-02-retained-test-point-ownership-unit)
and [task](../../development/checklist.md#task-uiu-api-02).
