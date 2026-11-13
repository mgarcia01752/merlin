# MERLIN examples

Choose the folder for the task you want to perform:

| Folder | What it contains | How to use it |
| --- | --- | --- |
| [cli](cli/README.md) | CLI simulation requests, candidate settings and sample payload | Pass a JSON file to the `ofdm-fec` command shown in its README. These are not saved UI settings. |
| [workbench](workbench/README.md) | Ready-to-open desktop workbenches and original example captures | In MERLIN, use File → Open and select `workbench.merlin.json`. |
| [measurements](measurements/README.md) | Shared measurement data, provenance and related import/analysis fixtures | Load compatible captures through the UI, or use the documented CLI import workflows. |

JSON is a file format, not a single kind of MERLIN setting. A `workbench.merlin.json`
file stores desktop state. CLI request JSON describes a command's simulation or
analysis inputs. Measurement JSON contains decoded capture values; manifests list
files, while provenance records describe their origins and hashes.

The CLI requests formerly at the top of `examples/` now live under `cli/`.
Desktop installers include the workbench/measurement collections and their
licenses/notices, but omit the `cli/` folder. CLI distributions include all public
examples. The examples are not automatically executed when the application starts.
