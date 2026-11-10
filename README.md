<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/logo/merlin-logo-dark-rounded.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/images/logo/merlin-logo-light-rounded.svg">
    <img src="docs/images/logo/merlin-logo-light-rounded.svg" alt="MERLIN — DOCSIS OFDM Simulation Toolkit" width="520">
  </picture>
</p>

# MERLIN — DOCSIS OFDM Simulation Toolkit

*Modulation, Encoding, Reception, Link Impairments & Noise*

[![Status: Under development](https://img.shields.io/badge/Status-Under_development-yellow)](docs/development/status.md)
[![Version: 0.1.0](https://img.shields.io/badge/Version-0.1.0-blue)](VERSION)
[![Ubuntu CI](https://github.com/mgarcia01752/merlin/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mgarcia01752/merlin/actions/workflows/ci.yml)
[![CodeQL](https://github.com/mgarcia01752/merlin/actions/workflows/codeql.yml/badge.svg?branch=master)](https://github.com/mgarcia01752/merlin/actions/workflows/codeql.yml)
[![Dependency security](https://github.com/mgarcia01752/merlin/actions/workflows/dependency-security.yml/badge.svg?branch=master)](https://github.com/mgarcia01752/merlin/actions/workflows/dependency-security.yml)
[![Rust 1.85.1](https://img.shields.io/badge/Rust-1.85.1-000000?logo=rust&logoColor=white)](rust-toolchain.toml)
[![Ubuntu LTS targets: 22.04, 24.04, 26.04 x64](https://img.shields.io/badge/Ubuntu_LTS-22.04%20%7C%2024.04%20%7C%2026.04-E95420?logo=ubuntu&logoColor=white)](docs/development/native-delivery.md)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

Simulate a single downstream DOCSIS OFDM channel, recover its payload through
actual BCH/LDPC decoding, and measure how impairments affect bit errors and decoder
work. Run an ideal reference or use decoded RxMER, downstream channel estimates
and an applied modulation profile. All signal processing and FEC run in Rust.

MERLIN simulates the **downstream PHY transmit functions of a CMTS for a single
DOCSIS OFDM channel**. It encodes payloads with BCH/LDPC, applies profile-based
QAM modulation and interleaving, and generates PLC/NCP signaling, pilots and the
OFDM waveform. The channel model applies noise and channel-response impairments;
the receiver processes the resulting samples, corrects the channel response and
decodes the payload. Independent comparison with the original source measures
recovered-bit errors, while decoder statistics show failures, corrections and
iteration counts. This is a PHY simulation with actual waveform generation and
payload recovery; full CMTS MAC services and multi-channel operation are outside
its scope.

## Workbench overview

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/logo/merlin-toolkit-diagram-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="docs/images/logo/merlin-toolkit-diagram-light.png">
    <img src="docs/images/logo/merlin-toolkit-diagram-light.png" alt="MERLIN workbench: OCD and DPD configure the transmitter; RxMER and DS channel estimates feed the channel and receiver; PRBS source data is recovered at PRBS-RX." width="1100">
  </picture>
</p>

Illustrated system overview. See the [workbench guide](docs/user/workbench.md)
for native UI screenshots, configuration and test-point instruments.

## Signal analyzers

Actual native UI captures. Select an image to open its guide.

| Spectrum Analyzer | QAM Analyzer |
| --- | --- |
| [<img src="docs/images/workbench/sa-tx-baseband-dark.png" alt="OFDM spectrum with symbol timing and cable echo estimate" width="520">](docs/user/spectrum-analyzer.md) | [<img src="docs/images/workbench/transmitter-qam-dark.png" alt="Transmitter QAM constellation from captured mapped I/Q" width="520">](docs/user/transmitter-workbench.md) |
| **Channel Response / Echo** | **RxMER** |
| [<img src="docs/images/workbench/receiver-response-dark.png" alt="Receiver-estimated impulse response showing delayed channel energy" width="520">](docs/user/receiver-workbench.md#channel-response-and-echo) | [<img src="docs/images/workbench/rxmer-single-dark.png" alt="Measured RxMER versus subcarrier frequency" width="520">](docs/user/channel-measurements.md#rxmer-captures) |

## Measurements and demos

- [File payloads](docs/user/simulation/file-source/README.md): send exact text/binary bytes, optionally append finite PRBS15 fill, and export recovered bytes.
- [PNM input guide](docs/user/measurement-inputs/README.md): supported capture types, decoded JSON and file limits.
- [Impairment experiments](docs/user/impairments/experiments.md): reproduce the RxMER sweeps and inspect FEC outcomes.
- [Paired constellations](docs/user/impairments/paired-constellation.md): export matched payload I/Q before and after equalization; includes an actual measured-response plot.
- [Receiver Channel Estimation (CH-EST-RX)](docs/user/receiver-estimation/README.md): independent pilot estimation, protected analyzers and bounded timing search.
- [Temporal measurements](docs/user/impairments/measurement-timeline.md) and [mixed profiles](docs/user/payload-transport/applied-profiles.md): explicit playback and variable-width coding.
- [Binary RxMER demos](examples/measurements/rxmer-fec-stress/README.md): baseline plus 12, 11, 10 and 9 dB stress cases for external viewing.

## Documentation

- [UI Guide](docs/user/workbench.md): configure and run the Qt workbench.
- [Rust API Guide](docs/reference/public-api.md): use the simulation engine directly.
- [Python Guide](docs/user/python/README.md): planned bindings and current status.

See the [documentation index](docs/README.md) for CLI usage, simulation models,
reference material and development guides.

Apache-2.0 signal-processing libraries and tooling; the Qt application is
[GPL-3.0-only](apps/qt/LICENSE). With MIT/Apache dependencies and a Rust-adapted MIT BCH
implementation. See [LICENSE](LICENSE), [NOTICE](NOTICE) and the
[AFF3CT notice](crates/fec-codec/LICENSE-MIT).
