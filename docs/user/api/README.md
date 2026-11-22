# MERLIN Basic Rust API

The Basic Rust API configures MERLIN's existing single-channel source → transmitter
→ channel → receiver → independent source comparison. It does not implement a
second simulator. Loading and preparation never run the experiment; only explicit run methods
start simulation. Existing low-level APIs, CLI and Qt remain available.

Use the `merlin` facade; enable optional `native-input` for directional binary/JSON
and file conveniences through the existing pinned native adapter. Independent
core consumers may still enable `high-level-api` and use `pnm-input` directly.
Neither route needs Qt or Python. See [passive input admission](input-admission.md)
for timing/source contracts and Rust/Python parity.
[Original profile inspection](profile-inspection.md) covers complete source
catalogs, compact schemes and bounded scientific views.
[Configured channel preview](channel-preview.md) documents immutable loss/power/
filter admission and authoritative transfer observations.

For searchable class/method documentation generated from the actual APIs, see
the [generated Rust/Python reference](../../reference/api-reference.md).

## Direction namespaces

The workspace's thin `merlin` crate exposes the independent engines through
`merlin::downstream::Simulation` and `merlin::upstream::Simulation`. Both direction
features are enabled by default; either can be selected independently. Existing
`docsis_ofdm_fec::api::{Simulation, Merlin}` imports remain valid and refer to the
same downstream types. No new published Cargo artifact is implied.

`merlin::common` contains shared error categories and cooperative cancellation.
The downstream RunControl is the common type. The upstream RunControl retains
its own pause/progress state and can share a common token through
`RunControl::with_cancellation`. Configuration, limits and reports remain
direction-specific. The [upstream guide](../upstream/README.md) describes supported
synthetic and capture-backed models.

The existing Python `merlin.Simulation` retains downstream behavior. Installed
wheels expose genuine `merlin.downstream`, `merlin.upstream` and `merlin.common`
modules through one native extension/distribution; compatibility imports retain
their nominal type identity.

## Rust workflow

This simple example loads one modulation-profile capture and one RxMER
capture, then simulates the first loaded profile and reads its FEC results.
It does not load a channel estimate: a flat (unity) channel response is used,
while RxMER supplies the measured signal-quality impairment. Loading only
configures the simulation; `run()` starts it.

Create `Simulation::new()`, decode with `CaptureLoader::profiles`, then call
`sim.load_profiles(profiles)`. Loading defaults to DOCSIS 3.1 and derives OCD
settings. For explicit interpretation or settings use
`sim.load_profiles_with_options(protocol, profiles, overrides)`.
`Simulation::with_ocd(ocd)` supports manually configured channels. Legacy
`Merlin` and capture-controller factories remain supported.

The expanded fixture-backed example, which also loads a channel estimate, lives in
[`run_single_capture.rs`](../../../examples/api/rust/run_single_capture.rs). Run it
from the repository root:

```bash
scripts/cargo-local.sh run -p pnm-input --example run_single_capture --locked
```

```rust,ignore

// Import the simulation types and native capture-loading helpers.
use docsis_ofdm_fec::api::*;
use pnm_input::{CaptureLoader, read_capture};

// Allow this fixture's profile and RxMER timestamps to differ by up to 25 hours.
const PROFILE_ASSOCIATION_TOLERANCE_SECONDS: u32 = 90_000;

// Create an empty simulation; no engine work starts yet.
let mut sim = Simulation::new();

// Decode and retain all profiles; the first profile is selected automatically.
sim.load_profiles(CaptureLoader::profiles(
    &read_capture("examples/workbench/single/modulation-profile.bin")?,
)?)?;

// Add one RxMER capture and retain its ID for selection.
let first = sim.add_rxmer(CaptureLoader::rxmer(
    &read_capture("examples/workbench/single/rxmer.bin")?,
)?)?;

// Configure the simulation before starting it.
sim.set_options(EvaluationOptions::default()

    // Use only the RxMER capture added above.
    .with_rxmer_selection(RxMerSelection::Individual(first))

    // Use a flat channel response without loading a channel estimate.
    .with_channel_selection(ChannelSelection::Unity)

    // Accept the fixture's timestamp difference without changing either capture.
    .with_profile_tolerance(AssociationTolerance::new(
        PROFILE_ASSOCIATION_TOLERANCE_SECONDS,
    )?))?;

// Start simulation explicitly and retain the independently owned results.
let batch = sim.run()?;

// Inspect each experiment and print available FEC results.
for entry in batch.iter() {

    // A report may be absent if the experiment did not start.
    if let Some(report) = entry.report() {

        // FEC counters may be unavailable when a run ends early.
        if let Some(fec) = report.fec() {

            // Print the number of codewords the decoder could not correct.
            println!("UFEC: {}", fec.uncorrectable_codewords().value());
        }
    }
}
```

The sample derives 192 MHz because its 187 MHz captured span fits only that
supported bandwidth. Rust records the value as `Derived`. If multiple supported
bandwidths fit a narrower capture, supply `OcdOverrides.bandwidth`; the API does
not guess the smallest fit. Explicit overrides remain validated. CP 512, roll-off 128 and depth 16 are reported
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
The optional [Basic Python API](../python/README.md) calls these same Rust operations
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
the engine's typed `CmChannelEstimated`, `PilotEstimated` and `MerlinChannelEstimated`
choices; the existing default remains `CmChannelEstimated`. Channel response
selection is independent: explicit unity configures a flat applied H, while the
receiver can still estimate its correction from received pilots. MCE uses the
engine's existing default reliability settings in the Basic Rust API.

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
Saved RxMER Designer recipes (flat or imported baselines) are reconstructed in
Rust automatically, preserving gaps, seeded variation and original bytes.
See the [supported WB contract and Rust example](../python/workbench.md).

### Matching Rust and Python collection examples

The [example inventory](../../../examples/api/README.md) provides the same
single-capture, multi-RxMER MCE, multi-RxMER supplied CM channel estimate,
saved-WB and custom designed-RxMER workflows in both languages.
The collection walkthroughs discover all
captured profiles through Rust, preserve original captures and explicitly run
each profile with matching policies, seeds, timing and bounds. Their FEC/BER
values come from the shared simulation engine; no analytical replacements are used.


## Controller-owned profiles

`Simulation::load_profiles` accepts decoded profiles from
`CaptureLoader::profiles`, retaining them in one idle session. For explicit
protocol/settings use `load_profiles_with_options`. Legacy
`CaptureLoader::controller_from_profiles` and `Merlin::from_profiles` remain available. The first supplied profile
is selected automatically. Use `profile_ids()`, `profile_by_id(id)` and
`select_profile(id)` without reloading files. Existing construction and
`set_profile()` remain supported.

`run_all_profiles()` preflights every retained profile, then runs the same engine
sequentially with unchanged options and fresh per-experiment engine state.
`run_all_profiles_with_control(&control)` preserves cancellation and partial
results. Each `ExperimentEntry::profile_id()` remains available even when no
report was produced. Current selection is restored before execution. At most
16 profiles, 256 combined experiments and 64 MiB retained capture storage are
accepted. Runtime limits apply per experiment, not to the complete collection.
See [the runnable example](../../../examples/api/rust/run_all_profiles.rs).

### Simulation lifecycle

Create the session, load profiles, add measurements, configure selection, then
call `run()` or `run_all_profiles()`. Empty sessions expose an empty profile list;
operations requiring channel geometry return a structured error until profiles
or an explicit OCD have been supplied. No construction or loading runs the engine.

Reloading replaces the entire profile collection and selects its first profile.
Compatible retained RxMER/channel estimates, their IDs, accepted run options and
run numbering remain intact. Invalid loads leave the previous state unchanged.
Changing to a different measurement grid requires a new simulation. Reports
remain independently owned. Run preflight still checks timestamps and coverage.

Multiple RxMER captures support Each (one experiment per capture), Individual,
AverageAll and InterpolateAt. Averaging/interpolation uses residual power and
preserves missing values on exact compatible grids. All-profile execution applies
that policy separately to every loaded profile. Channel selection remains an
independent explicit, nearest-time or unity policy.

### Apply a captured channel response and correct it at the receiver

See [the explicit CM channel-estimate example](../../../examples/api/rust/run_single_capture_cm_chan_est.rs).
It loads one profile capture, one RxMER and one CM channel estimate. Explicit
channel selection applies that response to the signal; `ReceiverMode::CmChannelEstimated`
uses the same response for receiver equalization. This is known-reference
correction, not pilot/MCE estimation. RxMER residual noise remains present.

The preferred receiver name is `CmChannelEstimated` in Rust and
`CM_CHANNEL_ESTIMATED` in Python. The previous `SuppliedChannel` name remains a
compatibility alias (Python also retains `SUPPLIED_CHANNEL`). Saved JSON still
writes `supplied_channel` and accepts both `supplied_channel` and
`cm_channel_estimated`. This naming change does not enable blind estimation:
the receiver corrects using the supplied CM response selected for the channel.

## Launch amplifier and Common Path Attenuator (CPA)

After loading profiles, configure the shared channel's gain and attenuation.
Both accept finite values from 0 through 60 dB. Invalid updates leave the previous
settings intact. An explicit launch gain clears any amplifier target level; its
getter returns the effective gain. CPA changes preserve that target and all other
settings. These controls apply to subsequent runs, not existing reports.

```rust
const LAUNCH_GAIN_DB: f64 = 12.0;
const CPA_ATTENUATION_DB: f64 = 6.0;

// Configure the loaded session without starting simulation.
sim.set_launch_amplifier_gain_db(LAUNCH_GAIN_DB)?;
sim.set_cpa_attenuation_db(CPA_ATTENUATION_DB)?;

// Read back the accepted values in dB.
let gain_db = sim.get_launch_amplifier_gain_db()?;
let cpa_db = sim.get_cpa_attenuation_db()?;
```

The core Rust power field is `cpa_attenuation_db`. Saved workbench and processing
dictionaries retain the legacy `attenuation_db` key for compatibility; JSON input
also accepts `cpa_attenuation_db`. See [terminology](../terminology.md).
