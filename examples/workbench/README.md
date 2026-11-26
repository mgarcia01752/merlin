# MERLIN workbench examples

Use File → Open and choose a `workbench.merlin.json` file. Opening selects the
project's direction and loads its settings; press Start to run. Examples are
self-contained and require no device, network or Python process.

| Direction | Bundles |
| --- | --- |
| [Downstream](downstream/README.md) | Single/multiple RxMER captures, channel 193 collection and 192 MHz notch comparison. |
| [Upstream](upstream/README.md) | Runnable 25/50 kHz ideal and synthetic measured-RxMER configurations. |

[manifest.json](manifest.json) is the common runnable inventory. Each bundle
includes provenance and applicable LICENSE/NOTICE files. Original downstream
files moved beneath `downstream/` without changing their saved input/capture
bytes. Historical validation commands may name their former locations.

The reserved downstream `multi-rxmer-dschan` folder remains deferred and is not
a runnable project. CLI request JSON lives separately under [../cli](../cli/README.md).
