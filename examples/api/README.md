# Rust and Python API examples

These examples call MERLIN's high-level Rust API. Python uses the native
extension over that same API. They load existing fixtures and run one channel;
they are source examples, not desktop workbench files or CLI JSON requests.
Loading does not run simulation. Only the explicit `run()` call starts it.
Both `high_level` files include filename headers and numbered developer
walkthroughs. Adjust the named constants at the top for fixture paths,
bandwidth, observation duration, association tolerance and host runtime; units
and assumptions are explained alongside the workflow.

| Example | Purpose |
| --- | --- |
| [Saved WB in Python](python/workbench.py) | Load embedded captures and saved settings directly through Rust; run selected profiles without duplicate scientific constants. |
| [Saved WB in Rust](rust/workbench.rs) | Use the same native loader and print owned per-profile batch reports. |
| [Rust workflow](rust/high_level.rs) | Import a profile, derive OCD settings, add RxMER and a channel estimate independently, select one capture and inspect simulated results. |
| [Python workflow](python/high_level.py) | Perform the same fixture-backed workflow using the installed native Python extension and print FEC counts and BER. |
| [Python multi-RxMER collection](python/multi_rxmer.py) | Discover all profiles in the channel-193 capture and load its 132 RxMER binaries; run independent profile experiments using a Rust residual-power average, unity physical response and MCE receiver. |
| [Python saved-UI comparison](python/multi_rxmer_ui.py) | Run the same collection workflow with PilotEstimated and all other settings matching the bundled multi-rxmer-2 workbench. |
| [Rust selection policies](rust/policy_parity.rs) | Run `Each`, `Individual`, `AverageAll` and `InterpolateAt`; write actual reference results for the Python parity tests. |

## Run Rust

From the repository root, with the pinned Rust toolchain configured:

```bash
scripts/cargo-local.sh run -p pnm-input --example high_level --locked
```

Cargo registers these source files as examples of `pnm-input`. They use shared
captures under `examples/workbench/single` and
`examples/measurements/single-channel`; no captures are duplicated here.

## Run Python

First build/install the native wheel using the
[Python setup guide](../../docs/user/python/README.md). Then, from the repository
root:

```bash
local/api-python-test/bin/python examples/api/python/high_level.py
```

For the separate channel-193 collection example:

```bash
local/api-python-test/bin/python examples/api/python/multi_rxmer.py
```

This loads the companion modulation-profile binary and all 132 RxMER binaries
from `examples/workbench/multi-rxmer-2`, then runs an independent `AverageAll`
experiment for each captured profile: **0, 3 and 4**, with explicit unity H. Rust computes the average in residual power, preserving
missing readings. The two-hour profile tolerance covers the collection.
It explicitly selects `ReceiverMode.MERLIN_CHANNEL_ESTIMATED` (MCE) and matches
the saved WB seeds, decoder limit and OCD timing. The saved WB selects
PilotEstimated; enable MCE in its receiver for comparable results. It loads the capture files directly; it is
not a general desktop WB JSON importer. `high_level.py` retains the original
single-capture workflow with an explicit measured channel estimate.

To match the **unchanged saved UI workbench**, run the separate companion:

```bash
local/api-python-test/bin/python examples/api/python/multi_rxmer_ui.py
```

It reuses the same workflow with `ReceiverMode.PILOT_ESTIMATED`; the MCE
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
scripts/cargo-local.sh run -p pnm-input --example policy_parity --locked -- local/api-example-parity
MERLIN_API_PARITY="$PWD/local/api-example-parity" local/api-python-test/bin/python -m unittest discover -s bindings/python/tests -p test_parity.py -v
```

The generator creates two synthetic RxMER JSON fixtures in that output directory
without changing the original captures. The Python parity test independently
runs each policy and compares actual Rust-engine results.

The workflow examples deliberately supply the fixture's 192 MHz bandwidth and
capture-time tolerance. They request 1 ms of source observation with a separate
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
local/api-python-test/bin/python examples/api/python/workbench.py /path/to/workbench.merlin.json
```

See the [WB import guide](../../docs/user/python/workbench.md) for supported
settings, structured failures and the equivalent Rust example.
