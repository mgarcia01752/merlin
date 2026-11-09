# DOCSIS OFDM Channel Sim

Offline Rust libraries and a CLI for downstream profile analysis and experimental
LDPC/BCH simulation. **Under development.**

DOCSIS OFDM Channel Sim provides a standalone public Rust library and CLI.
All scientific processing, including FEC, is Rust. Focus is downstream OFDM;
OFDMA comes later.
A Qt presentation component and Python bindings are deferred until the scientific
functionality is established.

- Actual BCH and LDPC encoding/decoding, seeded AWGN and measured per-carrier MER.
- Named BER stages, codeword errors, iteration counts and confidence intervals.
- Conservative and percentile candidate generation, protected carrier roles,
  full-resolution temporal screening and gross useful-symbol capacity.
- Exact device/channel/grid matching and explicit timestamp tolerance.
- Versioned decoded-scenario and candidate JSON; no device control or deployment.

The first coding experiment uses short16200-bit LDPC, rate8/9, BCH14232/14400 and
uniform16-QAM. Profile analysis recognizes data constellations through16384-QAM;
full coding simulations for higher orders remain pending. Label-ordered mapping
and reference soft demapping are available through16K-QAM;
see [QAM mapping](docs/user/qam-mapping.md). A standalone
[data/NCP randomizer](docs/user/cell-randomizer.md) supports the same data orders.
A [noiseless256-QAM coding loop](docs/user/qam256-coding-loop.md) now connects
bit processing, randomization and FEC, reporting iterations for each codeword.
A [stateful symbol packer](docs/user/symbol-packing.md) carries those codewords
across symbols and reserves NCP/CRC space. The [NCP codec](docs/user/ncp.md)
provides control LDPC, CRC and received codeword-start discovery.
[Cell interleaving](docs/user/cell-interleaving.md) now provides persistent time
delays, frequency permutations and the inverse physical pilot reference.
Separate [pilot generators and bounded test points](docs/user/pilot-signals.md)
provide physical-bin BPSK signs and explicit amplitude scaling.
The [PLC channel path](docs/user/plc-channel.md) now includes fixed acquisition
messages, FEC, interleaving, randomization, preamble, QAM and physical-bin assembly,
with known-phase soft recovery and bounded stage observations.
[Receiver configuration](docs/user/receiver-configuration/README.md) now derives
channel/profile settings from received descriptors behind a readiness gate.
[Waveform TX and matched RX](docs/user/waveform/README.md) now carry actual PLC/pilot
signals through time samples, with bounded test points and a noiseless example.
[Actual payload/NCP transport](docs/user/payload-transport/README.md) now uses
received PLC configuration, continuous state and real LDPC/BCH recovery over samples.
The [duration-based channel stepper](docs/user/channel-runner/README.md) now
streams continuous PRBS through actual waveform TX/RX, with separate acquisition,
observation, completion and drain. The cancellable channel CLI, nonideal acquisition
and external interoperability validation remain pending. See
[coding scope and equations](docs/reference/coding-mode.md).

[Simulation design guides](docs/user/simulation/README.md) explain each simulation
component with its own data-flow diagram and current implementation scope.

The isolated codec loop is available before involving modulation or a channel:

See the [FEC codec user guide](docs/user/fec-codec.md) for the encode/decode flow,
public API example and mathematical explanation.

```bash
./scripts/cargo-local.sh run -p fec-codec --example codec_loop --locked
```

```bash
./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  simulate --input examples/awgn.json --output awgn-result.json
```

Build prerequisite: Rust1.85.1. The wrapper uses an existing
project-local toolchain when present and installs nothing.

[Build and test](docs/development/build.md) ·
[Git and releases](docs/development/git.md) ·
[Logging](docs/development/logging.md) ·
[CLI examples](docs/cli.md) ·
[Architecture](docs/architecture.md) ·
[Implementation checklist](docs/development/checklist.md) ·
[Public API](docs/reference/public-api.md) ·
[Transmission setup](docs/user/transmission.md) ·
[Modulation coverage](docs/reference/ofdm-support.md) ·
[JSON contracts](docs/reference/documents.md)

Apache-2.0 project code, with MIT/Apache dependencies and a Rust-adapted MIT BCH
implementation. See [LICENSE](LICENSE), [NOTICE](NOTICE) and the
[AFF3CT notice](crates/fec-codec/LICENSE-MIT).
