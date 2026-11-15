# pnm-input

Optional native Apache-2.0 capture and file adapter for the MERLIN Rust high-level API.
Use `CaptureLoader` with `NamedCapture` for content-based binary/JSON import;
`read_capture` adds bounded read-only file access. No Qt, Python, subprocess,
modem or network service is used.

The pinned `pnm` 0.3.6 core at revision
`5fbd5561f3e06b73486546b5ffa520279080a5eb` is Apache-2.0. Direct parser modules:
`cm_ds_ofdm_modulation_profile`, `cm_ds_ofdm_rx_mer` and
`cm_ds_ofdm_chan_estimate_coef`. Both original and decoded intermediate hashes
are retained. Each type has binary/JSON fixture tests in `tests/high_level_api.rs`.
GPL Qt input orchestration is not copied.

See [user guide](../../docs/user/api/README.md) and the runnable
[API examples](../../examples/api/README.md).

`rxmer_export` adds independent capture output: PNM 1.0 binary with explicit
quarter-dB rounding and reject/clamp policy, or existing-contract decoded JSON
with full-precision finite samples and nulls. The byte encoders borrow validated
`RxMerFrame`; `RxMer::frame()` exposes one from the high-level API. Bounded
preflight/save helpers support new single files or collections, cancellation,
progress and provenance manifests. They never overwrite inputs/destinations or
run simulation. See the [save design](../../docs/development/features/rxmer-designer/pnm-export-plan.md)
for format evidence, publication rules and platform boundaries.
