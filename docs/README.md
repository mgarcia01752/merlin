# MERLIN documentation

## Guides

| Guide | Start here |
| --- | --- |
| [UI Guide](user/workbench.md) | Workbench configuration, instruments, runs and saved projects |
| [Rust API Guide](reference/public-api.md) | Simulation entry points, typed inputs, results and stage APIs |
| [Python Guide](user/python/README.md) | Native Rust bindings, wheel setup and [downstream installation](user/python/integration.md) |

[FAQ](user/faq.md): contextual run messages and suggested fixes, also available offline under About → FAQ.

[Terminology](user/terminology.md): definitions used by the workbench and analyzers.

## Command line

- [Channel CLI](user/channel-runner/cli.md)
- [Other CLI experiments](cli.md)
- [Windows desktop MSI](user/install/windows.md)
- [Native CLI installation](user/install/README.md)

## Simulation and reference

- [Simulation flow and duration-based channel](user/channel-runner/README.md)
- [Component design guides](user/simulation/README.md)
- [Transmission settings](user/transmission.md)
- [FEC codec and equations](user/fec-codec.md)
- [Codeword shortening](user/fec-shortening/README.md)
- [Receiver modes: supplied CM estimate, pilot estimation and MCE](user/receiver-estimation/modes.md)
- [QAM processing and decisions](user/qam-processing/README.md)
- [Modulation support](reference/ofdm-support.md)
- [PNM inputs](user/measurement-inputs/README.md)
- [RxMER Designer](user/measurement-inputs/rxmer-designer.md)
- [Impairment models](user/impairments/README.md)
- [JSON contracts](reference/documents.md)

## Development

- [Engineering skill catalog](../skills/README.md)
- [Planning, semantic reuse and traceable acceptance](../skills/development-workflow/SKILL.md)
- [Review changes from other sessions](../skills/external-change-review/SKILL.md)

- [Development workspace](development/README.md)
- [Current status and validation limits](development/status.md)
- [Architecture](architecture.md)
- [Implementation checklist](development/checklist.md)
- [Development setup](development/setup.md)
- [Build and test](development/build.md)
- [Qt setup and VM validation](development/qt.md)
- [Logging](development/logging.md)
- [Desktop debug controls](development/debugging.md)
- [Git and releases](development/git.md)

- [Example folder guide](../examples/README.md) · [CLI purposes and use cases](../examples/cli/README.md)

- [Python wheel releases and native validation](user/python/releases.md)
- [Generated Rust/Python API reference](reference/api-reference.md)
- [Basic Rust API and Basic Python API](user/api/README.md)
- [Matching Rust/Python examples](../examples/api/README.md), including the
  [custom designed RxMER collection](../examples/measurements/designed-rxmer/README.md)
