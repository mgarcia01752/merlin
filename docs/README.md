# MERLIN documentation

## Guides

| Guide | Start here |
| --- | --- |
| [UI Guide](user/workbench.md) | Workbench configuration, instruments, runs and saved projects |
| [Rust API Guide](reference/public-api.md) | Simulation entry points, typed inputs, results and stage APIs |
| [Python Guide](user/python/README.md) | Binding roadmap; implementation is deferred until after the UI |

[FAQ](user/faq.md): contextual run messages and suggested fixes, also available offline under About → FAQ.

[Terminology](user/terminology.md): definitions used by the workbench and analyzers.

## Command line

- [Channel CLI](user/channel-runner/cli.md)
- [Other CLI experiments](cli.md)
- [Native CLI installation](user/install/README.md)

## Simulation and reference

- [Simulation flow and duration-based channel](user/channel-runner/README.md)
- [Component design guides](user/simulation/README.md)
- [Transmission settings](user/transmission.md)
- [FEC codec and equations](user/fec-codec.md)
- [Codeword shortening](user/fec-shortening/README.md)
- [QAM processing and decisions](user/qam-processing/README.md)
- [Modulation support](reference/ofdm-support.md)
- [PNM inputs](user/measurement-inputs/README.md)
- [Impairment models](user/impairments/README.md)
- [JSON contracts](reference/documents.md)

## Development

- [Development workspace](development/README.md)
- [Current status and validation limits](development/status.md)
- [Architecture](architecture.md)
- [Implementation checklist](development/checklist.md)
- [Development setup](development/setup.md)
- [Build and test](development/build.md)
- [Qt setup and VM validation](development/qt.md)
- [Logging](development/logging.md)
- [Git and releases](development/git.md)
