# MERLIN examples

Choose the folder for the task you want to perform:

| Folder | What it contains | How to use it |
| --- | --- | --- |
| [api](api/README.md) | Runnable Rust and native Python API examples | Load captures, configure a simulation and inspect actual FEC/profile results. |
| [cli](cli/README.md) | CLI simulation requests, candidate settings and sample payload | Pass a JSON file to the `ofdm-fec` command shown in its README. These are not saved UI settings. |
| [workbench](workbench/README.md) | Ready-to-open desktop workbenches and original example captures | In MERLIN, use File → Open and select `workbench.merlin.json`. |
| [measurements](measurements/README.md) | Shared measurement data, provenance and related import/analysis fixtures | Load compatible captures through the UI, or use the documented CLI import workflows. |

JSON is a file format, not a single kind of MERLIN setting. A `workbench.merlin.json`
file stores desktop state. CLI request JSON describes a command's simulation or
analysis inputs. Measurement JSON contains decoded capture values; manifests list
files, while provenance records describe their origins and hashes.

The CLI requests formerly at the top of `examples/` now live under `cli/`.
Desktop installers include the workbench/measurement collections and their
licenses/notices, but omit the `cli/` and `api/` source folders. CLI distributions include all public
examples. The examples are not automatically executed when the application starts.

The custom [merlin_rxmer_designed.bin](measurements/designed-rxmer/merlin_rxmer_designed.bin) and its
[export sidecar](measurements/designed-rxmer/merlin_rxmer_designed.bin.merlin.json) are synthetic RxMER
Designer output, not a CM measurement. Matching Rust/Python walkthroughs and
compatible profile selection are documented in the
[API examples](api/README.md#custom-designed-rxmer).

[Upstream configuration fixtures](cli/upstream/README.md) exercise typed Rust admission and bounded runs.

The unified API pair uses [Rust](../crates/merlin/examples/upstream_session.rs) and
[Python](api/python/run_upstream.py) over the same bounded upstream configuration.
Both discover inventory through Rust and preserve existing downstream examples.

The [synthetic CMTS RxMER generator](../docs/user/upstream/synthetic-rxmer.md)
has paired Rust/native Python examples and standard binary/provenance outputs
for both upstream carrier spacings. It performs no simulation or device capture.
