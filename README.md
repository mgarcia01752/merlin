<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/logo/merlin-logo-dark-rounded.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/images/logo/merlin-logo-light-rounded.svg">
    <img src="docs/images/logo/merlin-logo-light-rounded.svg" alt="MERLIN — DOCSIS OFDM/OFDMA Simulation Toolkit" width="520">
  </picture>
</p>

# MERLIN — DOCSIS OFDM/OFDMA Simulation Toolkit

*Modulation, Encoding, Reception, Link Impairments & Noise*

[![Status: Under development](https://img.shields.io/badge/Status-Under_development-yellow)](docs/development/status.md)
[![Latest tag](https://img.shields.io/github/v/tag/mgarcia01752/merlin?sort=semver&label=Latest%20tag)](https://github.com/mgarcia01752/merlin/tags)
[![Master](https://github.com/mgarcia01752/merlin/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mgarcia01752/merlin/actions/workflows/ci.yml)
[![Release](https://github.com/mgarcia01752/merlin/actions/workflows/release.yml/badge.svg)](https://github.com/mgarcia01752/merlin/actions/workflows/release.yml)
[![Rust 1.85.1](https://img.shields.io/badge/Rust-1.85.1-000000?logo=rust&logoColor=white)](rust-toolchain.toml)
[![Ubuntu 22.04+ targets: 22.04, 24.04, 26.04 x64](https://img.shields.io/badge/Ubuntu_LTS-22.04%20%7C%2024.04%20%7C%2026.04-E95420?logo=ubuntu&logoColor=white)](docs/development/native-delivery.md)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

MERLIN is a Rust toolkit for **DOCSIS downstream OFDM and upstream OFDMA PHY
simulation**. Two independent engines share DSP and FEC infrastructure, with one
public Rust facade, one native Python distribution and one command-line executable.
The native Qt workbench provides downstream configuration and instruments.

Application source, including the licensed PNM decoder and its tests, lives in
this checkout. Builds do not require another project repository or private source
credentials; normal locked registry dependencies and toolchain downloads remain.

Run a synthetic reference or an admitted measurement-driven experiment, then
compare recovered data with the original source. Inspect actual bit errors,
decoder decisions, corrections and iteration counts. All signal processing,
FEC and simulation orchestration run in Rust.

## Supported scope

| Direction | Signal path | Configuration and observations |
| --- | --- | --- |
| [Downstream OFDM](docs/user/channel-runner/README.md) | CMTS transmitter → channel → CM receiver | BCH/LDPC, profile-based QAM, interleaving, PLC/NCP and pilots; ideal references or captured RxMER/channel estimates with an applied modulation profile. |
| [Upstream OFDMA](docs/user/upstream/README.md) | CM transmitter → channel → CMTS receiver | Typed channel, burst-profile and grant configuration; synthetic runs or explicitly associated UCD, RxMER and CM Pre-Eq captures. Normal receiver processing uses received observations. |

The unified backend has passed its
[local integration acceptance](docs/development/features/unified-backend/validation.md#int-accept-01-final-backend-acceptance).
Direction-specific configurations, limits and reports remain distinct. See the
[downstream support matrix](docs/reference/ofdm-support.md) and
[upstream measurement contracts](docs/user/upstream/measurements.md) before
selecting operating modes or combining captures.

Full CMTS MAC services, channel bonding, multi-channel simulation and wider
upstream acquisition/ranging are outside the accepted scope. Closed-loop results
describe MERLIN's modeled experiment; they do not establish external modem
interoperability. The Qt workbench on `master` presents downstream OFDM only;
the unified upstream UI remains separate development work.

<a id="rust-and-python-apis"></a>

## Get started

**Desktop:** follow the [Ubuntu DEB](docs/user/install/ubuntu-desktop.md) or
[Windows MSI](docs/user/install/windows.md) guide and choose the matching
[release assets](https://github.com/mgarcia01752/merlin/releases).

**CLI, Rust and Python:** the [getting-started guide](docs/user/getting-started.md)
contains runnable commands, namespace imports and links to the detailed API guides.
Browse the [CLI examples](examples/cli/README.md) and
[paired Rust/Python examples](examples/api/README.md) for complete workflows.

<a id="workbench-overview"></a>
<a id="downstream-workbench"></a>

## Workspaces

| Downstream | Upstream |
| :---: | :---: |
| [<img src="docs/images/workbench/unified-downstream-dark.png" alt="Downstream workspace in the unified Qt development branch" width="430">](docs/images/workbench/unified-downstream-dark.png) | [<img src="docs/images/workbench/unified-upstream-dark.png" alt="Upstream workspace in the unified Qt development branch" width="430">](docs/images/workbench/unified-upstream-dark.png) |

Native previews from the
[`feature/unified-qt-workbench` branch](https://github.com/mgarcia01752/merlin/tree/feature/unified-qt-workbench).
Select either image to view it full size. See the [workbench guide](docs/user/workbench.md)
and [saved examples](examples/workbench/README.md) for the released downstream UI.

## Signal analyzers and RxMER Designer

Native downstream UI captures. Select an image to open its guide.

| Spectrum Analyzer | QAM Analyzer |
| :---: | :---: |
| [<img src="docs/images/workbench/sa-tx-baseband-dark.png" alt="OFDM spectrum with PLC shading, symbol timing and cable echo estimate" width="520">](docs/user/spectrum-analyzer.md) | [<img src="docs/images/workbench/receiver-qam-mer-dark.png" alt="Paired raw and equalized receiver constellations with calculated MER" width="520">](docs/user/receiver-workbench.md#qam-analyzer-and-retained-selectors) |
| **Channel Response / Echo** | **RxMER** |
| [<img src="docs/images/workbench/receiver-response-dark.png" alt="Pilot-estimated channel impulse response" width="520">](docs/user/receiver-workbench.md#channel-response-and-echo) | [<img src="docs/images/workbench/rxmer-single-dark.png" alt="Loaded RxMER capture with PLC region and playback controls" width="520">](docs/user/channel-measurements.md#rxmer-captures) |

<h3 align="center">RxMER Designer</h3>

<p align="center">
  <a href="docs/user/measurement-inputs/rxmer-designer.md"><img src="docs/images/workbench/rxmer-designer-dark.png" alt="RxMER Designer with seeded random variation, frequency effects and PLC shading" width="700"></a>
</p>

## Measurements and demos

- [File payloads](docs/user/simulation/file-source/README.md): send exact text/binary bytes, optionally append finite PRBS15 fill, and export recovered bytes.
- [PNM input guide](docs/user/measurement-inputs/README.md): supported capture types, decoded JSON and file limits.
- [Impairment experiments](docs/user/impairments/experiments.md): reproduce the RxMER sweeps and inspect FEC outcomes.
- [Paired constellations](docs/user/impairments/paired-constellation.md): export matched payload I/Q before and after equalization; includes an actual measured-response plot.
- [Receiver Channel Estimation](docs/user/receiver-estimation/README.md): independent pilot estimation, protected analyzers and bounded timing search.
- [Temporal measurements](docs/user/impairments/measurement-timeline.md) and [mixed profiles](docs/user/payload-transport/applied-profiles.md): explicit playback and variable-width coding.
- [Binary RxMER demos](examples/measurements/rxmer-fec-stress/README.md): baseline plus 12, 11, 10 and 9 dB stress cases for external viewing.

## Documentation

- [Getting Started](docs/user/getting-started.md): installation, CLI commands and Rust/Python imports.
- [UI Guide](docs/user/workbench.md): configure and run the Qt workbench.
- [Upstream OFDMA Guide](docs/user/upstream/README.md): configuration, stages, captures, sessions and CLI.
- [Rust API Guide](docs/reference/public-api.md): use both simulation engines directly.
- [Python Guide](docs/user/python/README.md): native bindings, local wheel setup and examples.
- [Architecture](docs/architecture.md), [requirements](docs/requirements.md) and [checklist](docs/development/checklist.md): canonical ownership, contracts and acceptance evidence.

See the [documentation index](docs/README.md) for CLI usage, simulation models,
reference material and development guides. The [examples index](examples/README.md)
separates CLI requests, saved workbenches and measurement fixtures. The checkout's
canonical version is [VERSION](VERSION).

Apache-2.0 signal-processing libraries and tooling; the Qt application is
[GPL-3.0-only](apps/qt/LICENSE). With MIT/Apache dependencies and a Rust-adapted MIT BCH
implementation. See [LICENSE](LICENSE), [NOTICE](NOTICE) and the
[AFF3CT notice](crates/fec-codec/LICENSE-MIT).
