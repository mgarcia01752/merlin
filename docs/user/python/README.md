# Python Guide

The optional native Python API calls MERLIN's existing Rust engine through the
[high-level Rust API](../api/README.md). Scientific validation, averaging,
interpolation, simulation and FEC remain in Rust. Python never runs the CLI as a
subprocess and does not require Qt.

Distribution name: `merlin-ofdm`. Import name: `merlin`. The wheel described here
is a local development artifact from the checkout, not a published package or a
claim about an existing release asset. The tested development target is Ubuntu
22.04 x86_64 with CPython 3.10. Other Python versions and platforms are unverified.

For PyPNM or another consumer, see [downstream installation and version
pinning](integration.md).

## Load saved UI settings directly

Use `Workbench.from_file(path)` to import embedded captures and saved scientific
settings without duplicating constants. Only `run()` starts simulation. See the
[saved-workbench guide](workbench.md), supported settings and runnable example.

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
local/api-python-test/bin/python examples/api/python/high_level.py
```

`high_level.py` demonstrates the original single-capture workflow. For the
separate example using the modulation profile and 132 RxMER binaries from
`examples/workbench/multi-rxmer-2`, run:

```bash
local/api-python-test/bin/python examples/api/python/multi_rxmer.py
```

The collection example runs Profiles 0, 3 and 4 independently, each with the
Rust residual-power average, explicit unity physical response and MCE receiver
correction. The saved workbench uses SuppliedChannel; enable MCE there to compare. It reports actual FEC
results and wall time per profile. The observation is 1 ms per profile; increase
`OBSERVATION_DURATION_MS` for a longer run. See the [example notes](../../../examples/api/README.md) for
scope and matched fixture settings.

To reproduce the bundled saved workbench's supplied-channel receiver instead:

```bash
local/api-python-test/bin/python examples/api/python/multi_rxmer_ui.py
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

```python
from merlin import (
    DPD, OCD, Merlin, ProfileId, ProtocolScope, OcdOverrides,
    BandwidthHz, RxMer, EvaluationOptions, ChannelSelection,
    AssociationTolerance, ObservationDuration, RunLimits, RuntimeLimit,
)

profile = DPD.from_file(
    "examples/workbench/single/modulation-profile.bin", ProfileId.PROFILE_A)
ocd = OCD.from_profile(ProtocolScope.DOCSIS31, profile,
    OcdOverrides(bandwidth=BandwidthHz(192_000_000)))
sim = Merlin(ocd)
sim.set_profile(profile)
first = sim.add_rxmer(RxMer.from_file("examples/workbench/single/rxmer.bin"))
sim.set_options(EvaluationOptions(
    channel_selection=ChannelSelection.unity(),
    observation=ObservationDuration(1),
    profile_tolerance=AssociationTolerance(90_000),
    limits=RunLimits(runtime=RuntimeLimit(300_000)),
))

# Only this call starts simulation.
for entry in sim.run():
    report = entry.report
    if report is not None and report.fec is not None:
        print(report.fec.uncorrectable_codewords.value)
        print(report.profile.post_fec_ber.value
              if report.profile.post_fec_ber is not None else None)
```

The bundled fixture's occupied bandwidth and timestamp tolerance are explicitly
selected. Timing defaults and scientific assumptions are retained in reports.
Use the [complete example](../../../examples/api/python/high_level.py) for
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
scripts/cargo-local.sh run -p pnm-input --example policy_parity --locked -- local/parity-example
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
chooses how the receiver corrects it: `SUPPLIED_CHANNEL` (the existing default),
`PILOT_ESTIMATED`, or `MERLIN_CHANNEL_ESTIMATED` (MCE with existing engine defaults).
Unity H can still be received through pilot-estimated correction. Selecting a
mode does not run simulation. Accepted options and reports retain independent
snapshots of the choice. All estimation/validation remains in Rust.

The collection example selects MCE and matches the curated WB's noise/PRBS
seeds, decoder limit, average policy, duration and OCD timing. The saved WB
selects SuppliedChannel, so enable MCE there to match receiver behavior. It does
not import arbitrary WB JSON; changing that WB requires updating the example's
named constants. A fixture test guards drift. No change to the original
single-capture example's receiver default is implied.

The expanded MCE name is `MerlinChannelEstimated` in Rust and
`ReceiverMode.MERLIN_CHANNEL_ESTIMATED` in Python. Saved WB/CLI/report JSON
retains `merlin_estimated` for compatibility; input also accepts
`merlin_channel_estimated`. This is a naming change only.
