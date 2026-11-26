# Upstream native Python API

[Python guide](README.md) · [Upstream models](../upstream/README.md) ·
[Operation matrix](../../development/features/high-level-api/compatibility-matrix.md)

The unified source build provides real `merlin.downstream`, `merlin.upstream` and
`merlin.common` modules in one `merlin-ofdm` distribution. Every model, decoder,
channel, receiver and run executes in the same Rust owners used by the Basic Rust
API. Python converts inputs and returns owned results. The existing
`merlin.Simulation` and `merlin.Merlin` retain downstream behavior and identity.

```python
from merlin.downstream import Simulation as DownstreamSimulation
from merlin.upstream import Simulation as UpstreamSimulation
from merlin.common import ErrorCode, RunControl
```

Use an upstream Simulation with explicit typed configuration or the independently
versioned upstream configuration JSON. Loading validates and remains idle;
`preview()` discovers the actual Rust inventory and bounded plan. Only `run()` or
`run_with_control()` starts processing. Complete and partial reports retain actual
source comparison, FEC decisions, settings and provenance. Report status and zero
observed bit errors alone do not establish complete recovery.

The [paired Python example](../../../examples/api/python/run_upstream.py) uses the
same fixture as [the unified Rust example](../../../crates/merlin/examples/upstream_session.rs).
Run Python with the wheel installed in an isolated environment; Rust uses
`scripts/cargo-local.sh run -p merlin --example upstream_session --locked`.
A matching version/tag alone does not establish a published unified wheel.

## Captured input

`UcdCapture`, `RxmerCapture` and `PreEqCapture` inspect bounded original bytes.
Their exported metadata and exact original bytes/hashes remain observations.
`configure_measured_json(bytes)` admits a complete explicit association plan through
Rust's PNM adapter, then atomically accepts the prepared core input. It does not
infer CM/CCAP identity, resample grids or reinterpret corrective C as physical H.
See [measurement admission](../upstream/measurements.md) for supported composition,
source-time and corresponding-probe constraints.

For offline synthetic inputs, use
`SyntheticRxmerCapture.from_json(request_json)` and retain both
`original_bytes()` and `to_json()`. The native generator records synthetic
origin and never starts a simulation or contacts a device. The
[generator guide](../upstream/synthetic-rxmer.md) documents the request, limits,
paired Rust/Python examples and future parser follow-up.

## Control and errors

The common RunControl is the existing downstream stop token. An upstream
RunControl retains its directional pause/progress state and optionally accepts
`cancellation=shared_common_control`. Either handle can request cancellation;
source/FEC outcomes already completed remain in the owned report. A paused worker
also observes the shared stop request. Native long-running work releases the
interpreter lock; competing session mutation raises Busy.

ApiError.code is the shared ErrorCode enum. Upstream errors also retain optional
field/byte-offset and legacy diagnostic code metadata. Handle categories directly;
do not parse messages. A failed decoder remains a scientific report outcome.
Rust borrowed views and generic Serde helpers deliberately have language-specific
mappings recorded in the matrix. Python contains no simulation calculations or
required subprocess sidecar. Native platform acceptance is recorded separately
from source support and packaging configuration.
