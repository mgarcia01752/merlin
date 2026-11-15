# MERLIN high-level API

The Rust high-level API configures MERLIN's existing single-channel source → transmitter
→ channel → receiver → independent source comparison. It does not implement a
second simulator. Loading and preparation never run the experiment; only `run()`
starts simulation. Existing low-level APIs, CLI and Qt remain available.

Enable the core's `high-level-api` feature. Add optional `pnm-input` for native
binary/JSON and file conveniences; it directly uses pinned Apache-2.0 `pnm`
0.3.6. Neither crate needs Qt or Python.

## Rust workflow

The complete fixture-backed example lives in
[`run_single_capture.rs`](../../../examples/api/rust/run_single_capture.rs). Run it
from the repository root:

```bash
scripts/cargo-local.sh run -p pnm-input --example run_single_capture --locked
```

```rust,ignore
use docsis_ofdm_fec::api::*;
use pnm_input::{CaptureLoader, read_capture};

let dpd = CaptureLoader::profile(
    &read_capture("examples/workbench/single/modulation-profile.bin")?,
    ProfileId::PROFILE_A,
)?;
let ocd = Ocd::from_profile(ProtocolScope::Docsis31, &dpd, OcdOverrides {
    bandwidth: Some(BandwidthHz::new(192_000_000)?),
    ..Default::default()
})?;
let mut sim = Merlin::new(ocd);
sim.set_profile(dpd)?;
let first = sim.add_rxmer(CaptureLoader::rxmer(
    &read_capture("examples/workbench/single/rxmer.bin")?,
)?)?;
sim.set_options(EvaluationOptions::default()
    .with_rxmer_selection(RxMerSelection::Individual(first))
    .with_channel_selection(ChannelSelection::Unity)
    .with_profile_tolerance(AssociationTolerance::new(90_000)?))?;
let batch = sim.run()?;
for entry in batch.iter() {
    if let Some(report) = entry.report() {
        if let Some(fec) = report.fec() {
            println!("UFEC: {}", fec.uncorrectable_codewords().value());
        }
    }
}
```

The sample explicitly selects 192 MHz because its source record supplies that
configuration; the high-level API never assumes that bandwidth. The imported profile
establishes grid, protected PLC/pilot roles and loading, but decoder JSON does
not establish occupied bandwidth. CP 512, roll-off 128 and depth 16 are reported
application defaults when not explicitly supplied. Manual OCD plus
`Dpd::from_ocd` creates a hypothesis; `with_modulation` edits data carriers while
preserving original captured bytes and protected roles.

For an already-loaded DPD, `dpd.profile_ids()` borrows the sorted validated
catalog retained during import, independently of `dpd.profile_id()` (the
selected profile). It performs no I/O or decoding. Created profiles list one ID;
edited profiles retain their source catalog. Serialized DPDs include
`profile_ids`. This metadata does not change the selected simulation profile.

Static discovery remains available before selecting one:

```rust
let input = read_capture("examples/workbench/single/modulation-profile.bin")?;
let ids = CaptureLoader::profile_ids(&input)?;
for id in ids {
    let profile = CaptureLoader::profile(&input, id)?;
    // Configure a fresh controller and run this profile explicitly.
}
```

For decoded JSON without the native binary adapter, use
`Dpd::profile_ids_from_bytes(&input)`. Both return owned validated `ProfileId`
values in ascending captured-ID order, reject invalid catalogs and leave inputs
unchanged. Discovery and loading do not run simulation. IDs describe what is
present; profile selection and simulation admission remain separate operations.

## Selection and ownership

Add RxMER captures and channel estimates independently. Counts and epochs need
not match. IDs belong to the controller that returned them. Failed additions
leave accepted configuration unchanged. Inputs and old reports stay immutable.

- `Each` (default) runs each RxMER independently, in insertion order.
- `Individual(id)` runs one selected original.
- `AverageAll` runs one equal-capture residual-power average.
- `InterpolateAt(epoch)` runs one static observation, using immediately bracketing
  epochs and residual-power interpolation. No extrapolation or duplicate epochs.

Average/interpolation preserve exact identity/grids and propagate missing
contributors. Equal 20/30 dB observations produce about 22.596373 dB at equal
weights, not 25 dB. The shared engine's active-carrier coverage checks still apply.
These independent experiments are not continuous playback.

Choose `Unity`, `Explicit(channel_id)` or `NearestWithin(tolerance)` explicitly.
Explicit H may be reused across epochs with original provenance retained. Nearest
matching rejects ties/missing matches and cannot accompany `AverageAll`, which
has no unique epoch. Profile-time tolerance is independent and defaults to zero;
widen it deliberately for captures acquired at different times.

## Execution and reports

`prepare()` preflights every experiment without executing simulation. `run()`
uses fresh PRBS15/noise state per experiment. Defaults are 100 ms observation,
noise seed 41, PRBS seed 0x7fff, 50 iterations, 30 s host runtime and 64 retained
word details. Source observation time excludes acquisition/completion/drain and
is separate from host runtime. A short host budget can return partial results.

`RunControl` is cloneable and may be cancelled by another thread. Cancellation
preserves completed/partial science and includes every remaining request as
`NotStarted`. Preflight errors reject the entire batch. Runtime/started failures
retain their report and dispatch continues unless cancelled. Complete lifecycle
status does not mean successful FEC recovery.

Default FEC and profile summaries share observation-window scope. Missing windows,
BER denominators and unfinished rates remain absent. All-completed FEC counts and
iteration diagnostics are separately labeled; MER is observation-cell diagnostic
power. Payload rate is information-bit rate, not application goodput. RxMER
conditions Gaussian residual error; it cannot uniquely reconstruct interference
or predict a particular modem.

`to_json()` creates an owned schema-versioned snapshot. `pnm_input::save_json`
creates a new file and refuses to overwrite an existing destination. File reads
are bounded at 16 MiB; controller capture storage is bounded at 64 MiB and 256
captures. Decoder expansion and engine limits remain active.

Implementation/evidence: [feature design](../../development/features/high-level-api/design.md),
[acceptance mapping](../../development/features/high-level-api/requirements.md),
and [validation](../../development/features/high-level-api/validation.md).
The optional [Python API](../python/README.md) calls these same Rust operations
through native bindings; see its local wheel setup and parity evidence.

Capture export is independent of simulation: use
`pnm_input::rxmer_export::encode(rxmer.frame(), RangePolicy::Reject)` for PNM
bytes, or `encode_json(rxmer.frame())` for lossless decoded JSON. The shared
save helpers add no-overwrite publication and provenance. See the
[export API](../../reference/public-api.md#rxmer-capture-export).

## Receiver correction

The [receiver-mode comparison](../receiver-estimation/modes.md) explains the
three choices and their channel/noise sources using the UI names.

`EvaluationOptions::with_receiver_mode(ReceiverMode::PilotEstimated)` selects
received-pilot correction through the existing engine. `ReceiverMode` re-exports
the engine's typed `SuppliedChannel`, `PilotEstimated` and `MerlinChannelEstimated`
choices; the existing default remains `SuppliedChannel`. Channel response
selection is independent: explicit unity configures a flat applied H, while the
receiver can still estimate its correction from received pilots. MCE uses the
engine's existing default reliability settings in this high-level API.

Preparation forwards the choice without running; owned report options preserve
it after later controller edits and assumptions describe the selected receiver.
Python mirrors this as `EvaluationOptions(receiver_mode=...)`. The collection
walkthrough selects MCE; enable MCE in the saved workbench to compare results. See the
[Python receiver example](../python/README.md#select-receiver-correction-independently-of-the-channel).

The expanded MCE name is `MerlinChannelEstimated` in Rust and
`ReceiverMode.MERLIN_CHANNEL_ESTIMATED` in Python. Saved WB/CLI/report JSON
retains `merlin_estimated` for compatibility; input also accepts
`merlin_channel_estimated`. This is a naming change only.

## Load a saved workbench

Use `pnm_input::Workbench::from_file(path)` to load embedded captures and saved
settings directly, without restating scientific constants. Loading preflights
but does not run; `workbench.run()` returns one owned batch per saved selected
profile through the same engine. Python mirrors this as `Workbench.from_file`.
See the [supported WB contract and Rust example](../python/workbench.md).

### Matching Rust and Python collection examples

The [example inventory](../../../examples/api/README.md) provides the same
single-capture, multi-RxMER MCE, multi-RxMER supplied CM channel estimate and
saved-WB workflows in both languages. The collection walkthroughs discover all
captured profiles through Rust, preserve original captures and explicitly run
each profile with matching policies, seeds, timing and bounds. Their FEC/BER
values come from the shared simulation engine; no analytical replacements are used.
