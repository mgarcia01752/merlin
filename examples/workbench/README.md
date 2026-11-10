# MERLIN workbench examples

Use File → Open and select a bundle's `workbench.merlin.json`. Each runnable
directory also contains the original PNM inputs for opening individual blocks.
WB files embed those exact bytes, so they remain self-contained after relocation.
The former `examples/workbench.merlin.json` is now under `single/`.

| Directory | Modulation profile | RxMER | DS channel estimate | Default |
| --- | --- | --- | --- | --- |
| [single](single/README.md) | One | One | One | Single capture |
| [multi-rxmer](multi-rxmer/README.md) | One | Five | One | Average residual power |
| [multi-rxmer-dschan](multi-rxmer-dschan/README.md) | Planned | Planned collection | Planned collection | Deferred |

The runnable sets share the same modem/channel, full frequency grid and DPD
coverage. Original capture times differ; the configured association tolerance
is explicit. Synthetic RxMER variants keep the baseline epoch and are alternate
impairments, not successive measured events. Profile A/ID 0 and existing Rust
transport limits still apply. A valid WB does not promise error-free recovery.

[manifest.json](manifest.json) inventories runnable bundles and the deferred
scenario. Each runnable bundle preserves provenance and upstream LICENSE/NOTICE.
See the [user guide](../../docs/user/workbench-examples.md) for installed paths.
