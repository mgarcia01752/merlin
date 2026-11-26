# Prepare a saved downstream full loop

`merlin::downstream::inputs::Workbench::prepare_full_loop` admits a self-contained
schema 1/2 workbench and returns an idle public sweep plan. Enable the facade's
`native-input` feature. Python mirrors the operation as
`merlin.downstream.Workbench.prepare_full_loop(data)` in the same native package.

```mermaid
flowchart LR
    Saved[Saved original project bytes] --> Admission[Native Rust admission]
    Admission --> Plan[Idle downstream sweep plan]
    Plan --> Request[Explicit selected profile request]
    Request --> Run[Explicit public full-loop run]
```

Admission checks embedded originals, names, lengths, hashes, profile/capture
selection, receiver settings and existing resource limits. It reconstructs saved
Designer recipes through their shared Rust owner. Original capture and decoded
intermediate hashes remain distinct. No capture sidecars or Qt process are needed.

The prepared source retains continuous PRBS15 or exact file bytes with optional
independently seeded finite fill. Playback retains single, average, hold or
interpolate mode, stored selection order and the synthetic OFDM-symbol interval.
Single uses the first selected stored original, as in the legacy loader. Temporal
selection must fit the saved observation duration. Dormant files and unselected
captures are still checked. Missing, unknown or corrupt settings fail explicitly.

A legacy file source may have an empty display label. Admission retains that
empty label and exact source bytes/hash through the explicit unnamed-file source
contract. Profile, RxMER and channel-estimate captures still require valid
provenance identities. The file label never selects encoding or changes fill.

Rust loads without executing:

```rust,ignore
use merlin::downstream::{inputs::Workbench, FullLoopControl};
let plan = Workbench::prepare_full_loop(&project_bytes)?;
let request = plan.request(0)?;
let result = request.run(&FullLoopControl::new())?;
```

Python uses the same scientific admission and execution:

```python
from pathlib import Path
from merlin import downstream
plan = downstream.Workbench.prepare_full_loop(Path("saved.merlin.json").read_bytes())
request = plan.request(0)
result = request.run()
```

Discover the selected profiles through the plan's `info` before dispatching more
requests. Each request has its own public control and independently owned result;
the plan creates no worker and retains no collection of results. Rejected loading
does not modify an existing plan. Returned requests remain usable after input
bytes or the plan are dropped. Cancellation preserves actual completed trials.

Saved H/profile timestamp tolerance is independent of RxMER/profile tolerance.
The request's `response_tolerance` exposes the accepted H policy. For manually
assembled inputs, Rust's `Simulation::prepare_full_loop_with_response_tolerance`
and its Python static mirror accept that separate checked value; ordinary
`prepare_full_loop` retains its existing association behavior. Sweep preparation
preserves whichever policy the original request accepted. Neither tolerance
changes original capture epochs or grids.

Existing `Workbench.from_file/from_bytes` and their legacy batch runs keep their
documented PRBS/static-playback behavior. [Direction-tagged project admission](projects.md) now returns these same owned
plans through the common Project API. Qt Open/Save adoption remains a later task. See the
[serial sweep contract](full-loop-sweep.md),
[legacy loader guide](../python/workbench.md) and
[implementation design](../../development/features/unified-qt-workbench/design.md#uiu-api-05-direction-aware-public-project-admission).
