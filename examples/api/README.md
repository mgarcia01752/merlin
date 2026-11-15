# Rust and Python API examples

These examples call MERLIN's Basic Rust API. Python uses the native
extension over that same API. They load existing fixtures and run one channel;
they are source examples, not desktop workbench files or CLI JSON requests.
Loading does not run simulation. Only explicit `run()` or `run_all_profiles()` calls start it.
Both single-capture files include filename headers and numbered developer
walkthroughs. Adjust the named constants at the top for fixture paths,
observation duration, association tolerance and host runtime; units
and assumptions are explained alongside the workflow.

| Example | Purpose |
| --- | --- |
| [Saved WB in Python](python/run_multi_rxmer_cmce_workbench.py) | Load embedded captures and saved settings directly through Rust; run selected profiles without duplicate scientific constants. |
| [Saved WB in Rust](rust/run_multi_rxmer_cmce_workbench.rs) | Use the same native loader and print owned per-profile batch reports. |
| [Rust workflow](rust/run_single_capture.rs) | Import a profile, derive OCD settings, add RxMER and a channel estimate independently, select one capture and inspect simulated results. |
| [Python workflow](python/run_single_capture.py) | Perform the same fixture-backed workflow using the installed native Python extension and print FEC counts and BER. |
| [Python multi-RxMER collection](python/run_multi_rxmer_mce.py) | Discover all profiles in the channel-193 capture and load its 132 RxMER binaries; run independent profile experiments using a Rust residual-power average, unity physical response and MCE receiver. |
| [Python saved-UI comparison](python/run_multi_rxmer_cm_chan_est.py) | Run the same collection workflow with SuppliedChannel and all other settings matching the bundled multi-rxmer-2 workbench. |
| [Rust selection policies](rust/run_rxmer_selection_parity.rs) | Run `Each`, `Individual`, `AverageAll` and `InterpolateAt`; write actual reference results for the Python parity tests. |

The same workflows are available in both languages:

| Workflow | Rust | Python |
| --- | --- | --- |
| Single capture | [run_single_capture.rs](rust/run_single_capture.rs) | [run_single_capture.py](python/run_single_capture.py) |
| Multi-RxMER with MCE | [run_multi_rxmer_mce.rs](rust/run_multi_rxmer_mce.rs) | [run_multi_rxmer_mce.py](python/run_multi_rxmer_mce.py) |
| Multi-RxMER with supplied CM channel estimate | [run_multi_rxmer_cm_chan_est.rs](rust/run_multi_rxmer_cm_chan_est.rs) | [run_multi_rxmer_cm_chan_est.py](python/run_multi_rxmer_cm_chan_est.py) |
| Custom designed RxMER | [run_designed_rxmer.rs](rust/run_designed_rxmer.rs) | [run_designed_rxmer.py](python/run_designed_rxmer.py) |
| Saved multi-RxMER CMCE workbench | [run_multi_rxmer_cmce_workbench.rs](rust/run_multi_rxmer_cmce_workbench.rs) | [run_multi_rxmer_cmce_workbench.py](python/run_multi_rxmer_cmce_workbench.py) |

The two Rust collection entry points share the documented
[collection walkthrough](rust/support/multi_rxmer.rs), including named settings
and numbered loading/run/report steps. Both languages use the same captured
profile catalog, original binaries, AverageAll policy, unity physical response,
seeds, timing and runtime limits. The CM estimate example selects supplied-channel
correction; this collection has no CM response capture, so its reference is unity.

## Run Rust

From the repository root, with the pinned Rust toolchain configured:

```bash
scripts/cargo-local.sh run -p pnm-input --example run_single_capture --locked
```

Cargo registers these source files as examples of `pnm-input`. They use shared
captures under `examples/workbench/single` and
`examples/measurements/single-channel`; no captures are duplicated here.
The collection examples use `examples/workbench/multi-rxmer-2`:

```bash
scripts/cargo-local.sh run -p pnm-input --example run_multi_rxmer_mce --release --locked
scripts/cargo-local.sh run -p pnm-input --example run_multi_rxmer_cm_chan_est --release --locked
```

## Run Python

First build/install the native wheel using the
[Python setup guide](../../docs/user/python/README.md). Then, from the repository
root:

```bash
local/api-python-test/bin/python examples/api/python/run_single_capture.py
```

For the separate channel-193 collection example:

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_mce.py
```

This loads the companion modulation-profile binary and all 132 RxMER binaries
from `examples/workbench/multi-rxmer-2`, then runs an independent `AverageAll`
experiment for each captured profile: **0, 3 and 4**, with explicit unity H. Rust computes the average in residual power, preserving
missing readings. The two-hour profile tolerance covers the collection.
It explicitly selects `ReceiverMode.MERLIN_CHANNEL_ESTIMATED` (MCE) and matches
the saved WB seeds, decoder limit and OCD timing. The saved WB selects
SuppliedChannel; enable MCE in its receiver for comparable results. It loads the capture files directly; it is
not a general desktop WB JSON importer. `run_single_capture.py` retains the original
single-capture workflow with an explicit measured channel estimate.

To match the **unchanged saved UI workbench**, run the separate companion:

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cm_chan_est.py
```

It reuses the same workflow with `ReceiverMode.SUPPLIED_CHANNEL`; the MCE
walkthrough stays available separately. Both use the same profile discovery,
132 captures, residual-power average, unity H, seeds, limits and timing.
This matches the bundled saved WB; later UI edits require matching the shared
constants and receiver choice. Neither script imports arbitrary WB JSON.

`DPD.profile_ids_from_file()` discovers the catalog through Rust and returns
validated `ProfileId` objects. If a DPD is already loaded, use `dpd.profile_ids`
to read its retained catalog without accessing the file again. The fixture
currently contains 0, 3 and 4; no IDs
are hardcoded in the script. Each fresh controller resets the common configured PRBS/noise
seeds and prints its own actual FEC/BER and host execution time. The default
`OBSERVATION_DURATION_MS = 1` requests only 1 ms of source observation per
profile. Increase that named constant for a longer experiment; execution speed
is not simulated time. Averaging 132 captures produces one residual model,
not 132 experiments per profile.

The interpreter must have the compatible native `merlin-ofdm` wheel installed.
A precompiled wheel needs neither Rust nor Qt. Python does not invoke the CLI.

## Exercise all selection modes

Choose a new output directory; the generator refuses an existing directory:

```bash
mkdir -p local
scripts/cargo-local.sh run -p pnm-input --example run_rxmer_selection_parity --locked -- local/api-example-parity
MERLIN_API_PARITY="$PWD/local/api-example-parity" local/api-python-test/bin/python -m unittest discover -s bindings/python/tests -p test_parity.py -v
```

The generator creates two synthetic RxMER JSON fixtures in that output directory
without changing the original captures. The Python parity test independently
runs each policy and compares actual Rust-engine results.

The workflow examples derive 192 MHz from the fixture's unambiguous captured
span and explicitly supply capture-time tolerance. They request 1 ms of source observation with a separate
300 s host runtime bound. Those choices are examples, not inferred modem settings
or promises about wall-clock duration. Results retain actual status and scope;
complete execution does not imply FEC success.

See the [Rust API guide](../../docs/user/api/README.md),
[Python API guide](../../docs/user/python/README.md) and
[validation record](../../docs/development/features/high-level-api/validation.md).
Return to the [examples index](../README.md).

Python examples display BER in scientific notation, for example `1.234e-06`.
Change `BER_FORMAT` at the top of each script to adjust display precision.
Unavailable BER prints `unavailable`; numeric API results are unchanged.

The Rust parity generator accepts an optional receiver mode after its new output
directory: `supplied_channel` (default), `pilot_estimated`, or `merlin_estimated`.
Generate a separate directory for each; the Python parity test reads the recorded
mode and compares Each/Individual/AverageAll/InterpolateAt under that same mode.


For a saved WB with its own settings, use:

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cmce_workbench.py /path/to/workbench.merlin.json
```

See the [WB import guide](../../docs/user/python/workbench.md) for supported
settings, structured failures and the equivalent Rust example. The bundled WB
uses supplied CM channel estimation (CMCE); another input WB retains its saved
receiver mode regardless of the script filename.

## Custom designed RxMER

Use the supplied [designed binary](../measurements/designed-rxmer/merlin_rxmer_designed.bin) with its
[recipe and export provenance](../measurements/designed-rxmer/merlin_rxmer_designed.bin.merlin.json):

```bash
scripts/cargo-local.sh run -p pnm-input --example run_designed_rxmer --release --locked
local/api-python-test/bin/python examples/api/python/run_designed_rxmer.py
```

Both examples discover and independently run every profile in
`examples/workbench/single/modulation-profile.bin` (currently 0, 1, 2 and 3).
This profile's decoded hash matches the sidecar's `profile_sha256` and its grid
exactly matches the designed capture: channel 34, 7,480 bins, first bin 356,
25 kHz spacing and 631.1 MHz frequency at bin zero. The multi-rxmer-2 profile
is a different channel/grid and cannot be substituted.

The designer sidecar declares a 48 dB baseline, -10 dB tilt, 2 dB peak-to-peak
ripple with 20 MHz spacing, and a 6 dB notch centered at 766.4875 MHz. These are
**synthetic designed values, not modem measurements**. The binary already
contains the quantized result; the examples never reapply the recipe. Native
binary loading preserves the original bytes/provenance but does not read or
automatically attach the adjacent sidecar; keep that file with the example to
retain its synthetic origin and recipe.

The examples use explicit Individual selection, SuppliedChannel correction and
unity physical response; no separate complex channel capture is loaded. Unity
correction does not remove RxMER-driven residual noise. Named constants document
192 MHz bandwidth, CP 512, roll-off 128, interleaver depth 16, noise seed 41,
PRBS seed 0x7fff, 50 decoder iterations, 1 ms source observation per profile and
a 300-second host budget per experiment. The 90,000-second profile association
tolerance is specific to the different original fixture epochs; it changes
neither timestamp. Loading does not run simulation; each explicit run returns
actual status/FEC/BER, with unavailable results kept absent.

With this exact fixture and the documented settings, the validated Rust and
Python runs produced matching results: Profiles 0/1/2 had 82/102/112 corrected
codewords and zero uncorrectable words; Profile 3 had 17 corrected and 105
uncorrectable out of 122, with BER `1.282e-02`. All runs completed. These are
software simulation results for this designed input, not modem performance claims.


## Loaded profile collection

`run_all_profiles.py` and `run_all_profiles.rs` load one capture into a controller,
list its retained IDs, and run every profile with the same RxMER and options.
Each result carries `profile_id`, including cancelled/not-started entries.
`select_profile(id)` selects one loaded profile for `run()`; `profile_by_id(id)`
returns the retained profile. Existing static loaders and `set_profile()` remain
available. No loading or selection operation runs simulation.

```bash
local/api-python-test/bin/python examples/api/python/run_all_profiles.py
scripts/cargo-local.sh run -p pnm-input --example run_all_profiles --locked
```

The Rust and Python single-capture and all-profile examples use the default
DOCSIS 3.1 interpretation. Advanced examples keep explicit protocol choices
where they construct an OCD directly. File-format versions do not select a
protocol automatically.

## Simulation sessions and timed checks

The basic Rust/Python examples create a Simulation, load the profile collection,
then add measurements and run explicitly. Advanced examples can supply a
validated OCD to the Simulation constructor (Rust: `with_ocd`). Saved-workbench
examples continue to use Workbench and the same simulation engine.

`scripts/test-local.sh example` runs the existing example unit tests plus the
single-capture and all-profile Rust API examples. Each API check prints elapsed
wall time including incremental compilation and retains logs/JSON under
`local/api-example-tests/`. The all phase, guarded checks and shared CI include
these checks. Failure still emits elapsed time and fails the gate.

Native wheel release checks print Python example-only elapsed seconds separately
from total wheel validation time. Their JSON report retains
`example_elapsed_seconds` and `elapsed_seconds`; installation time is excluded
from example time. This applies to Windows and Ubuntu wheel validation.
