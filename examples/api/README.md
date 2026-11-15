# Rust and Python API examples

These examples call MERLIN's high-level Rust API. Python uses the native
extension over that same API. They load existing fixtures and run one channel;
they are source examples, not desktop workbench files or CLI JSON requests.
Loading does not run simulation. Only the explicit `run()` call starts it.

| Example | Purpose |
| --- | --- |
| [Rust workflow](rust/high_level.rs) | Import a profile, derive OCD settings, add RxMER and a channel estimate independently, select one capture and inspect simulated results. |
| [Python workflow](python/high_level.py) | Perform the same fixture-backed workflow using the installed native Python extension and print FEC counts and BER. |
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
