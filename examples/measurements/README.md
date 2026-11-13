# Shared measurement examples

[Examples index](../README.md) · [Desktop workbenches](../workbench/README.md)

These collections contain measurement inputs used by capture import, simulations,
UI viewers and development tests. They include decoded JSON, original PNM binary
files where available, provenance, licenses/notices and related CLI import/analysis
recipes. They are not saved desktop settings. Open `workbench.merlin.json` from
the separate workbench folder for a ready-configured desktop example.

| Collection | Use case |
| --- | --- |
| [single-channel](single-channel/README.md) | Inspect/import RxMER, channel estimates and modulation-profile data, with documented impairment variants. |
| [channel-widths](channel-widths/README.md) | Exercise compatible 24 MHz and 90 MHz channel grids and their profiles. |
| [rxmer-fec-stress](rxmer-fec-stress/README.md) | Inspect the baseline and synthetic 12/11/10/9 dB RxMER stress variants. |

Keep provenance and licenses with the data. See each collection's README for its
origin, interpretation and regeneration commands; a decoded JSON file and a CLI
request can both use `.json` but serve different purposes.

Standalone CLI requests live in `examples/cli/` in source/CLI distributions;
that folder is excluded from desktop installers.
