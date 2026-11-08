# DOCSIS OFDM FEC Toolkit

Offline Rust libraries and a CLI for downstream profile analysis and experimental
LDPC/BCH simulation. **Under development.**

PNM Viewer Toolkit will consume this library. PNM parsing and native Qt screens
remain the viewer's responsibility; this repository owns the scientific engine.

- Actual BCH and LDPC encoding/decoding, seeded AWGN and measured per-carrier MER.
- Named BER stages, codeword errors, iteration counts and confidence intervals.
- Conservative and percentile candidate generation, protected carrier roles,
  full-resolution temporal screening and gross useful-symbol capacity.
- Exact device/channel/grid matching and explicit timestamp tolerance.
- Versioned decoded-scenario and candidate JSON; no device control or deployment.

The first coding experiment uses short16200-bit LDPC, rate8/9, BCH14232/14400 and
uniform16-QAM. Full DOCSIS framing/placement and interoperability validation are
pending. See [coding scope and equations](docs/reference/coding-mode.md).

```bash
./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  simulate --input examples/awgn.json --output awgn-result.json
```

Build prerequisites: Rust1.85.1 and a C++17 compiler. The wrapper uses an existing
project-local toolchain when present and installs nothing.

[Build and test](docs/development/build.md) ·
[Git and releases](docs/development/git.md) ·
[CLI examples](docs/cli.md) ·
[Architecture](docs/architecture.md) ·
[PNM Viewer integration](docs/development/pnm-viewer-integration.md) ·
[JSON contracts](docs/reference/documents.md)

Apache-2.0 project code, with MIT/Apache dependencies and an adapted MIT BCH
kernel. See [LICENSE](LICENSE), [NOTICE](NOTICE) and the
[AFF3CT notice](crates/fec-codec/native/AFF3CT-LICENSE).
