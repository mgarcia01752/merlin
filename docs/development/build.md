# Development

Use Rust 1.85.1, Cargo, rustfmt and Clippy. Linux was validated
locally; macOS and Windows runtime/build validation remains outstanding.

```bash
./scripts/cargo-local.sh fmt --all -- --check
./scripts/cargo-local.sh test --workspace --locked
./scripts/cargo-local.sh run -p fec-codec --example codec_loop --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example qam256_codec_loop --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example symbol_packing --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example ncp_field --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example cell_interleaving --locked
./scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
./scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
./scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
```

A globally available Cargo works with rust-toolchain.toml. Alternatively, a
project-local Rust installation can live under ignored local/cargo and local/rustup.
No script in this checkout installs system packages or changes shell profiles.
Project-local toolchain/cache links can reuse an existing development installation;
build outputs remain in this repository's target/.

For an independent local toolchain, use rustup with CARGO_HOME and RUSTUP_HOME
pointing at this checkout's local directories, `--no-modify-path` and toolchain
1.85.1. Do not replace cache links while they are in use. Local Rust setup
does not change globally managed executables.

Cargo.lock pins the dependency graph. ldpc-toolbox0.10.1 was selected because
newer inspected releases use Rust APIs unavailable on the pinned toolchain.
BCH encoding and decoding are implemented in safe Rust. Upstream algorithm
provenance and the retained MIT notice are described in NOTICE. No native codec
build script, CXX bridge or C++ source is included in the build.

A Rust-only build helper under `crates/ofdm/build/` validates and embeds the
preserved QAM resources. It introduces no additional packages or native build
dependencies. See [QAM resources and runtime behavior](../user/qam-mapping.md).

Private normative PDFs, evidence and migration snapshots live in ignored
`development/`. They are not present in a fresh clone. The specification skills
include versioned lookup extracts; diagrams still require the original PDF.

The implementation branch is feature/ds-profile-fec-lab. Git operations use the
local helpers described in [Git and versioning](git.md).
