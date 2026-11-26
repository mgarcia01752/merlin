# MERLIN workbench examples

Use File → Open and choose a `workbench.merlin.json` file. Opening selects the
project's direction and loads its settings; press Start to run. Examples are
self-contained and require no device, network or Python process.

| Direction | Bundles |
| --- | --- |
| [Downstream](downstream/README.md) | Single/multiple RxMER captures, channel 193 collection and 192 MHz notch comparison. |
| [Upstream](upstream/README.md) | Runnable 25/50 kHz ideal and synthetic measured-RxMER configurations. |

[manifest.json](manifest.json) is the common runnable inventory: eight primary
projects plus twelve downstream notch companions, twenty saved workbenches in
total. The `companions` entries include every additional `.merlin.json` file.
Each bundle
includes provenance and applicable LICENSE/NOTICE files. Original downstream
files moved beneath `downstream/` without changing their saved input/capture
bytes. Historical validation commands may name their former locations.

The reserved downstream `multi-rxmer-dschan` folder remains deferred and is not
a runnable project. CLI request JSON lives separately under [../cli](../cli/README.md).

For a complete upstream scenario with UCD, RxMER and Pre-Eq inputs, start with
[synthetic-rxmer-25khz](upstream/synthetic-rxmer-25khz/README.md) or its
[50 kHz counterpart](upstream/synthetic-rxmer-50khz/README.md). Open the saved
workbench, press Start, then inspect PRBS-RX for actual source and FEC results.
Ideal upstream examples intentionally use no RxMER target. Their RxMER indicator
says **Not used**; PRBS-RX says **Awaiting results** until execution. Neither
state is a missing-configuration warning or a claim that verification passed.
