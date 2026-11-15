# Run a saved workbench through the API

Use a self-contained `workbench.merlin.json` saved by MERLIN's UI as the input to
Rust or Python. The Rust loader reads its embedded captures and settings, so you
do not need to restate timing, receiver mode, seeds, limits or profile IDs.

```python
from merlin import Workbench

wb = Workbench.from_file("workbench.merlin.json")  # Validate only; no simulation.
print([profile.value for profile in wb.profile_ids])
print(wb.options[0].receiver_mode)
for batch in wb.run():  # Explicit simulation; releases Python's interpreter lock.
    for entry in batch:
        print(entry.status)
        if entry.report is not None and entry.report.fec is not None:
            print(entry.report.fec.corrected_codewords.value)
```

Run the complete [Python example](../../../examples/api/python/run_multi_rxmer_cmce_workbench.py):

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cmce_workbench.py examples/workbench/multi-rxmer-2/workbench.merlin.json
```

Omitting the argument selects that same bundled WB. A native wheel containing
this API is required; rebuild/reinstall your local wheel after updating source.
No Qt installation, running UI, CLI subprocess or external capture files are
needed. Loading bytes is also supported: `Workbench.from_bytes(wb_bytes)`.

## Settings taken from the WB

The loader supports saved schema 1 and 2 with accepted PRBS15, DPD, RxMER and
receiver sections. It verifies embedded base64 originals against their saved
lengths and SHA-256 hashes, preserves exact grids and missing values, and retains
the complete original WB independently of its input file.

| WB setting | API behavior |
| --- | --- |
| OCD | Validate saved bandwidth, spacing, CP, roll-off and interleaver depth against embedded DPD. |
| Profiles | Use explicit schema-2 selection first, otherwise all-profiles or active-profile setting; preserve legacy all-profile behavior. |
| PRBS15 | Use saved source seed; other file-source editor fields are dormant in this mode. |
| RxMER | Support static single or residual-power average; honor checked capture hashes in saved order. |
| Channel response | Use saved H and response tolerance, or explicit unity when absent. |
| Receiver | Use saved SuppliedChannel, PilotEstimated or MCE, including MCE tuning and waveform channel-filter settings. |
| Physical channel | Use saved additional cable/tilt loss and power conditioning. |
| Work limits | Use saved observation duration, decoder iterations, runtime budget and noise seed. |
| Theme/comparison display | Presentation settings do not alter the API's simulated FEC reporting. |

The bundled multi-rxmer-2 WB selects SuppliedChannel, not MCE. Saving a new WB
with MCE selected changes the imported mode without editing Python constants.
The loader does not silently substitute modes. Unknown fields and unsupported
schema versions fail with structured `ApiError` diagnostics.

File payload sources, continuous RxMER hold/interpolation playback and incomplete
editor-only WBs are unsupported by this importer. They fail explicitly rather
than being approximated. Static API Each/Individual/AverageAll/InterpolateAt
workflows remain available separately; static interpolation is not timeline
playback. One channel per WB is supported.

## Ownership, cancellation and limits

Loading preflights the complete selected sweep but runs no simulation. `options`
returns independent snapshots; editing them does not override saved settings.
Its `processing` dictionary exposes MCE/filter/loss/power choices for inspection.

`run()` returns a list of independently owned BatchReports, one per profile in
`profile_ids` order. Experiment IDs remain scoped to each profile controller;
do not compare identifiers from different batches. Results survive dropping the
WB. Inspect status and report availability: cancellation or runtime expiry can
produce partial or not-started entries, not zero counters.

Use `run_with_control(control)` with a native `RunControl` to cancel from another
Python thread; the token is shared across profiles. Later profiles remain in the
returned list as not-started. Concurrent runs of the same Python WB raise Busy.

Bounds are 144 MiB of WB JSON, 16 MiB per embedded original, 96 MiB combined
originals and 256 RxMER captures. Existing engine memory, decoder and execution
bounds also apply. Inputs are read only; no save/export operation is implied.

## Rust

`pnm_input::Workbench::from_file(path)` and `from_bytes(bytes)` expose the same
loader without Qt or Python dependencies. `controllers()` permits read-only
inspection/preparation. `into_controllers()` transfers controllers for explicit
manual option overrides through the normal API. `run()` and `run_with_control()`
return `Vec<BatchReport>`, preserving controller-scoped identities and ownership.

```bash
scripts/cargo-local.sh run -p pnm-input --example workbench --release --locked -- examples/workbench/multi-rxmer-2/workbench.merlin.json
```

The importer independently implements the saved format contract in the Apache
adapter; it does not import GPL Qt orchestration or provide a second engine.
Native Linux CPython development validation does not establish Windows wheel
or other interpreter/platform support.
