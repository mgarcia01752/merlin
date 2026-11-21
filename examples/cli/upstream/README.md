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

The unified CLI dispatch and Basic Python API are being integrated; command and
installed-wheel examples follow those acceptance gates. The
[upstream user guide](../../../docs/user/upstream/README.md) describes current support.

Adapted from Apache-2.0 source revision
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`. Only the format identifier was translated;
field values and independent expectations were retained. Original source hashes:
[adaptation inventory](../../../docs/development/features/ofdma/source-files.json).
