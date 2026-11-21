# Upstream configuration inputs

These generic synthetic inputs use `merlin.upstream.configuration`, schema 1.
They are separate from downstream workbench JSON and experiment reports.

| File | Purpose |
| --- | --- |
| [us-50khz.json](us-50khz.json) | 2K/50 kHz scheduled data with discoverable profiles |
| [us-25khz.json](us-25khz.json) | 4K/25 kHz scheduled data |
| [us-synthetic-playback.json](us-synthetic-playback.json) | Explicit synthetic correction/residual playback settings |
| [us-controls-inspection.json](us-controls-inspection.json) | Retained optional MAC controls for inspection; execution rejects |

The 25/50 kHz examples include mixed loading and zero-valued minislots. Their
vendor/template numbers are provenance, not modulation recommendations. Rust owns
inventory discovery; applications should query the selected document's inventory.

The Basic Rust API accepts these bytes through
`docsis_ofdma::api::Simulation::configure_json`; loading remains idle until `run`.
Run actual bounded acceptance with:

```bash
scripts/cargo-local.sh test -p docsis-ofdma --locked --test configuration_json
```

Use the same existing executable for idle preview and execution:

```bash
target/debug/ofdm-fec upstream preview --input examples/cli/upstream/us-50khz.json --output upstream-preview.json
target/debug/ofdm-fec upstream run --input examples/cli/upstream/us-50khz.json --output upstream-report.json
```

Choose new output paths; existing files are never replaced. The
[CLI guide](../../../docs/user/upstream/cli.md) explains captured plans, raw input
inspection and nonzero status with retained incomplete reports. The
[installed native Python example](../../api/python/run_upstream.py) uses the same
Rust engine. The
[upstream user guide](../../../docs/user/upstream/README.md) describes current support.

Adapted from Apache-2.0 source revision
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`. Only the format identifier was translated;
field values and independent expectations were retained. Original source hashes:
[adaptation inventory](../../../docs/development/features/ofdma/source-files.json).
