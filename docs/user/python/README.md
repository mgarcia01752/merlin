# Basic Python API Guide

The Basic Python API calls MERLIN's existing simulation engine through the
[Basic Rust API](../api/README.md). Scientific validation, averaging,
interpolation, simulation and FEC remain in Rust. Python never runs the CLI as a
subprocess and does not require Qt.

Distribution name: `merlin-ofdm`. Import name: `merlin`. The wheel described here
is a local development artifact from the checkout, not a published package or a
claim about an existing release asset. The Ubuntu target is 22.04+ x86_64 with CPython 3.10. Release CI builds on
22.04 and checks the same wheel on 24.04 and 26.04. New compatibility checks
remain unverified until hosted CI passes; other Python ABIs are not covered.

For Python-only setup, start with [download and install the Python API](integration.md#quick-start-download-and-install-only-the-python-api).
For your application, see [downstream installation and version
pinning](integration.md).

The [generated Basic Python API reference](../../reference/api-reference.md) provides
searchable native class, method, constructor-signature and property pages.

[Retained resource metadata](../api/retained-resources.md) covers immutable Rust
quantities for inputs, results and the existing downstream full-loop/preview and
upstream run controls. These methods preserve lifecycle state and share the Rust
validation; Python performs no storage calculations.

## Runnable examples

The [matching Rust/Python example inventory](../../../examples/api/README.md)
covers single capture, multi-RxMER with MCE or supplied-channel correction,
saved-workbench settings and a custom designed RxMER input. For the latter:

```bash
local/api-python-test/bin/python examples/api/python/run_designed_rxmer.py
```

It loads the binary under `examples/measurements/designed-rxmer/` and runs all
profiles from the compatible single-bundle profile. These are synthetic values,
not modem observations; its adjacent sidecar retains the recipe and provenance.
See the [collection notes](../../../examples/measurements/designed-rxmer/README.md)
for grid compatibility and the [Designer guide](../measurement-inputs/rxmer-designer.md)
for PNM/JSON saving and quantization.

## Load saved UI settings directly

Use `Workbench.from_file(path)` to import embedded captures and saved scientific
settings without duplicating constants. Only `run()` starts simulation. See the
[saved-workbench guide](workbench.md), supported settings and runnable example.

For release wheel filenames, platform compatibility, checksum verification and
required native tests, see the separate [Python wheel release guide](releases.md).

## Build and install locally

Build prerequisites are the repository's pinned Rust 1.85.1 toolchain and a
Python development environment. Create a project-local environment and install
the pinned wheel builder; this does not install system packages:

```bash
python3 -m venv local/api-python-build
local/api-python-build/bin/python -m pip install --only-binary=:all: maturin==1.8.3
local/api-python-build/bin/python tools/python/build_wheel.py
```

The helper collects complete locked dependency licenses/notices and builds the
native Linux wheel under ignored `local/python-wheels/`. It neither installs nor
publishes. PyO3 is pinned in the adapter's separate Cargo manifest and lockfile;
Rust/CLI consumers do not depend on it. Do not substitute an unpatched PyO3
version or omit dependency notices when rebuilding.

Install the matching CPython 3.10 Linux wheel into a separate local environment:

```bash
python3 -m venv local/api-python-test
local/api-python-test/bin/python -m pip install --no-index local/python-wheels/merlin_ofdm-0.2.9-cp310-cp310-linux_x86_64.whl
local/api-python-test/bin/python examples/api/python/run_single_capture.py
```

`run_single_capture.py` demonstrates the original single-capture workflow. For the
separate example using the modulation profile and 132 RxMER binaries from
`examples/workbench/multi-rxmer-2`, run:

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_mce.py
```

The collection example runs Profiles 0, 3 and 4 independently, each with the
Rust residual-power average, explicit unity physical response and MCE receiver
correction. The saved workbench uses CmChannelEstimated; enable MCE there to compare. It reports actual FEC
results and wall time per profile. The observation is 1 ms per profile; increase
`OBSERVATION_DURATION_MS` for a longer run. See the [example notes](../../../examples/api/README.md) for
scope and matched fixture settings.

To reproduce the bundled saved workbench's supplied-channel receiver instead:

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cm_chan_est.py
```

This companion shares all other settings and reporting with the MCE example;
it matches `examples/workbench/multi-rxmer-2/workbench.merlin.json` as saved.
UI edits made later are not automatically imported into either script.

For an already-loaded DPD, read its member directly:

```python
profile_ids = dpd.profile_ids
print([profile_id.value for profile_id in profile_ids])
```

The full catalog was retained during loading, independently of `dpd.profile_id`
(the selected profile). Reading it does not reopen or decode the file. Each
access returns an independent list; modifying that list does not alter the DPD.
Created DPDs list their one ID; edited DPDs retain their source catalog.

Static discovery is also available through the shared Rust loader:

```python
profile_ids = DPD.profile_ids_from_file(profile_path)

# For bytes: DPD.profile_ids_from_bytes("capture", capture_bytes)
for profile_id in profile_ids:
    profile = DPD.from_file(profile_path, profile_id)
```

The returned `ProfileId` objects are validated, owned and sorted by captured ID.
Binary PNM and decoded JSON use the same Rust validation. Invalid catalogs raise
structured `ApiError`; discovery never runs simulation. The collection example
uses this catalog to run every profile, without a fixed list of IDs.

Installing a compatible precompiled wheel needs neither Rust nor Qt. Building
from source needs the Rust toolchain. A native `linux_x86_64` tag does not promise
manylinux portability or support for another interpreter/OS.

## Simple workflow

This simple example loads one modulation-profile capture and one RxMER
capture, then simulates the first loaded profile and reads its FEC results.
It does not load a channel estimate: a flat (unity) channel response is used,
while RxMER supplies the measured signal-quality impairment. Loading only
configures the simulation; `run()` starts it.

`Simulation()` creates an empty simulation. `load_profiles(path)` loads the
profiles using DOCSIS 3.1 by default. Specify `protocol` only to change that
interpretation. Nothing runs until you call `run()`.

```python

# Import capture loaders, validated values, run settings and the controller.
from merlin import (
    Simulation,
    RxMer, EvaluationOptions, ChannelSelection,
    AssociationTolerance, ObservationDuration, RunLimits, RuntimeLimit,
)

# Keep fixture settings explicit, with units in each constant name.
OBSERVATION_DURATION_MS = 1  # Source observation, excluding acquisition/drain.
PROFILE_ASSOCIATION_TOLERANCE_SECONDS = 90_000  # Fixture capture-time allowance.
MAX_RUNTIME_MS = 300_000  # Limit each experiment to 300 seconds of host time.

# Load the capture once; this object owns its profiles and resolved channel settings.
sim = Simulation()
sim.load_profiles("examples/workbench/single/modulation-profile.bin")

# Inspect the retained IDs; the first loaded profile is selected for run().
print([profile_id.value for profile_id in sim.profile_ids])

# Add measured RxMER; retain the returned identifier for later selection.
first = sim.add_rxmer(RxMer.from_file("examples/workbench/single/rxmer.bin"))

# Configure the experiment before running it; loading does not start a run.
sim.set_options(EvaluationOptions(

    # Use a flat channel response instead of an imported channel estimate.
    channel_selection=ChannelSelection.unity(),

    # Set simulated source duration, independently of the host runtime limit.
    observation=ObservationDuration(OBSERVATION_DURATION_MS),

    # Allow this fixture's profile/RxMER timestamp difference without rewriting it.
    profile_tolerance=AssociationTolerance(PROFILE_ASSOCIATION_TOLERANCE_SECONDS),

    # Bound execution time; reaching the limit can leave a partial report.
    limits=RunLimits(runtime=RuntimeLimit(MAX_RUNTIME_MS)),
))

# Only run() starts simulation; iterate over the returned experiment entries.
for entry in sim.run():

    # Read the owned report, which may be absent for an unsuccessful entry.
    report = entry.report

    # Print FEC outcomes only when the report contains those measurements.
    if report is not None and report.fec is not None:

        # Show codewords that the decoder could not correct.
        print(report.fec.uncorrectable_codewords.value)

        # Show post-FEC bit error rate, preserving None when it is unavailable.
        print(report.profile.post_fec_ber.value
              if report.profile.post_fec_ber is not None else None)
```

The fixture's 187 MHz span uniquely identifies 192 MHz within supported channel
settings. Bandwidth is derived in Rust; the timestamp tolerance is explicit.
For narrower captures with multiple fitting bandwidths, supply an override:
`OcdOverrides(bandwidth=BandwidthHz(CHANNEL_BANDWIDTH_HZ))`, importing those types
and defining the constant from your channel configuration. Timing defaults and scientific assumptions are retained in reports.
Use the [complete example](../../../examples/api/python/run_single_capture.py) for
independent channel loading and explicit response reuse.

`from_file` accepts pathlib paths. `from_bytes(name, data)` uses bounded native
content detection, with an additional explicit ProfileId for DPD. Extensions are
ignored. Original bytes/hashes, decoded intermediate hashes and original epochs
stay distinct. Immutable `values` and `response` tuples preserve missing samples.
`DPD.from_ocd` creates a hypothesis; `with_modulation` preserves protected roles.
For a fully manual channel, `OcdConfiguration.from_json(key_json, layout_json,
overrides)` admits the existing Rust identity/layout transport contracts and
`OCD.create(protocol, configuration)` performs the same Rust validation. No
Python-side scientific interpretation is used.

## Selections and units

`RxMerSelection.each()` is the default. Choose
`individual(first)`, `average_all()` or `interpolate_at(CaptureTime(epoch))` for
one selected or derived observation. Average/interpolation operate in residual
power using the same Rust operations as Rust consumers; they are static
experiments, not timeline playback.

Choose `ChannelSelection.unity()`, `explicit(channel_id)` or
`nearest_within(AssociationTolerance(seconds))` deliberately. Channel estimates
are added independently with `add_channel_estimate`; no required pairs or equal
counts. Nearest matching rejects ties/missing matches and is invalid for a
multi-capture average. Profile tolerance is separate and defaults to zero.

Unit constructors validate in Rust; `.value` is the explicit primitive conversion.
For example `ObservationDuration(1)` means source milliseconds, whereas
`RuntimeLimit(30_000)` means host milliseconds. Descriptive aliases include
`CaptureTime.from_epoch_seconds`, `AssociationTolerance.from_seconds` and
`ObservationDuration.from_milliseconds`. IDs are opaque and controller-scoped;
they cannot be constructed from integers.

Options/overrides are writable requests. `set_options` accepts an independent
snapshot, including nested limits; later request edits do not change it. Results
and scientific quantities have read-only properties. Configuration, provenance,
sources, diagnostics and `to_dict()` are independent JSON transport snapshots;
editing a returned dictionary never changes accepted inputs or a native report.

## Cancellation, errors and results

`run()` releases the interpreter lock while Rust computes. Share a `RunControl`
and call `cancel()` from another Python thread. Ctrl-C requests cooperative
cancellation and returns an owned batch, preserving partial science. A request
cancelled during preflight may validly contain only `NotStarted` entries. The
same controller rejects concurrent access with `ApiError.code == ErrorCode.Busy`.

Pre-run failures raise `ApiError` with a stable `ErrorCode` and readable message.
Started failures/runtime limits remain report entries and do not discard prior
results. Each requested experiment has an entry; an unstarted entry has no
performance report. Unavailable windows, rates and BER denominators remain
`None`. Complete lifecycle status is distinct from successful FEC recovery.

Default FEC and profile summaries share observation-window scope. Whole-run FEC,
iteration statistics and per-carrier MER retain separately labeled scopes.
Reports remain valid after subsequent runs, controller mutation or deletion.
`to_json`/`to_dict` produce independent snapshots; `save_json(path)` refuses to
overwrite an existing destination. Payload information-bit rate is not application
goodput, and RxMER cannot uniquely reconstruct interference or predict a modem.

## Verification

The Rust reference example generates actual results before native Python parity
tests. Python tests read those results; the extension never invokes a subprocess:

```bash
scripts/cargo-local.sh run -p pnm-input --example run_rxmer_selection_parity --locked -- local/parity-example
MERLIN_API_PARITY="$PWD/local/parity-example" local/api-python-test/bin/python -m unittest discover -s bindings/python/tests -v
```

The output directory must be new. Counter/status comparisons are exact; floating
comparison uses absolute tolerances of 1e-12 for dimensionless ratios/power sums
and 1e-6 for dB/rates. Host elapsed time and controller ID scopes are excluded
from scientific parity. See the [validation record](../../development/features/high-level-api/validation.md)
for executed checks, wheel bytes/checksums and platform boundaries.

Return to the [documentation index](../../README.md).

### Save a loaded RxMER capture

```python
from pathlib import Path
import merlin

capture = merlin.RxMer.from_file(Path("capture.bin"))
capture.save_json(Path("designed.json"))  # full precision and nulls
capture.save_pnm(Path("designed.bin"))    # strict range, quarter-dB rounding
print(capture.pnm_export_summary())
```

Both saves create a companion provenance JSON and reject existing destinations.
`save_pnm(..., clamp=True)` explicitly opts into range clipping. `to_pnm_bytes()`
and `to_json_bytes()` return owned bytes without file I/O. All validation and
conversion stay in Rust; no CLI process is used. See the
[export API](../../reference/public-api.md#rxmer-capture-export).

### Select receiver correction independently of the channel

See the [plain-language receiver-mode comparison](../receiver-estimation/modes.md)
for channel correction, decoder noise information, defaults and saved settings.

```python
from merlin import EvaluationOptions, ChannelSelection, ReceiverMode

options = EvaluationOptions(
    channel_selection=ChannelSelection.unity(),
    receiver_mode=ReceiverMode.PILOT_ESTIMATED,
)
```

`ChannelSelection` chooses the applied physical response H. `ReceiverMode`
chooses how the receiver corrects it: `CM_CHANNEL_ESTIMATED` (the existing default),
`PILOT_ESTIMATED`, or `MERLIN_CHANNEL_ESTIMATED` (MCE with existing engine defaults).
Unity H can still be received through pilot-estimated correction. Selecting a
mode does not run simulation. Accepted options and reports retain independent
snapshots of the choice. All estimation/validation remains in Rust.

The collection example selects MCE and matches the curated WB's noise/PRBS
seeds, decoder limit, average policy, duration and OCD timing. The saved WB
selects CmChannelEstimated, so enable MCE there to match receiver behavior. It does
not import arbitrary WB JSON; changing that WB requires updating the example's
named constants. A fixture test guards drift. No change to the original
single-capture example's receiver default is implied.

The expanded MCE name is `MerlinChannelEstimated` in Rust and
`ReceiverMode.MERLIN_CHANNEL_ESTIMATED` in Python. Saved WB/CLI/report JSON
retains `merlin_estimated` for compatibility; input also accepts
`merlin_channel_estimated`. This is a naming change only.


## Run every loaded profile

Use `sim.run_all_profiles()` to evaluate every retained profile with the same
accepted RxMER, channel and run options. This preserves the selection used by
`sim.run()`. Every entry exposes `entry.profile_id`, including cancelled requests.
The controller preflights the complete collection before starting any simulation.
A collection holds at most 16 compatible profiles; a combined batch holds at most
256 experiments and preserves the existing 64 MiB capture budget. Runtime limits
apply per experiment, so running all profiles can take longer than one run.

```python

# Look up or select a loaded profile without reopening its capture.
loaded_profile = sim.profile_by_id(sim.profile_ids[0])
sim.select_profile(sim.profile_ids[0])

# Explicitly run every loaded profile; results remain independent of the controller.
for entry in sim.run_all_profiles():
    print(entry.profile_id.value, entry.status)
```

`sim.run_all_profiles(control)` also accepts a `RunControl` token. Cancellation
preserves completed/partial reports and the IDs of not-started profiles. Python
releases the interpreter lock during simulation. Existing `DPD.from_file`,
`OCD.from_profile`, `Merlin(ocd)` and `set_profile()` remain available for callers
who need explicit construction or replacement.

See matching [Python](../../../examples/api/python/run_all_profiles.py) and
[Rust](../../../examples/api/rust/run_all_profiles.rs) examples.

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

See [the explicit CM channel-estimate example](../../../examples/api/python/run_single_capture_cm_chan_est.py).
It loads one profile capture, one RxMER and one CM channel estimate. Explicit
channel selection applies that response to the signal; `ReceiverMode.CM_CHANNEL_ESTIMATED`
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

```python
LAUNCH_GAIN_DB = 12.0
CPA_ATTENUATION_DB = 6.0

# Configure the loaded session without starting simulation.
sim.set_launch_amplifier_gain_db(LAUNCH_GAIN_DB)
sim.set_cpa_attenuation_db(CPA_ATTENUATION_DB)

# Read back the accepted values in dB.
print(sim.get_launch_amplifier_gain_db())
print(sim.get_cpa_attenuation_db())
```

Saved workbench files and processing dictionaries retain the legacy `attenuation_db` key for compatibility; JSON input
also accepts `cpa_attenuation_db`. Editing a Python processing dictionary does
not update the session; use these setters. See [terminology](../terminology.md).

The [upstream native guide](upstream.md) describes the additive direction/common
namespaces, captured-plan admission and shared cancellation in the unified source
build. Existing root-level calls continue to use downstream OFDM.
