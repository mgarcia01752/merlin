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
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example pilot_signals --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example plc_messages --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example plc_codec_loop --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example plc_interleaving --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example plc_randomization --locked
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example plc_channel --locked
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

Standard logging dependencies are pinned in Cargo.lock and downloaded by Cargo;
no operating-system packages or separate installation step are required.
See [logging configuration](logging.md).

## Channel job and constrained-VM validation

The [channel CLI](../user/channel-runner/cli.md) and
[probe export](../user/simulation/probe-export.md) use the same Rust APIs as tests.
The pre-push gate now includes a release channel/JSONL smoke run and keeps the
older coding-mode smoke. No new operating-system packages are required.

Debug build outputs can become large on a small VM. To remove only generated
debug-profile outputs while retaining source files, Cargo downloads and release
artifacts, use:

```bash
./scripts/cargo-local.sh clean --profile dev
```

For lower disk usage during a local full gate, disable debug symbols and
incremental artifacts for that command only:

```bash
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 ./scripts/check-before-push.sh
```

This runs the same checks; it changes build metadata/cache size, not scientific
algorithms. Normal development can keep debug information enabled. The repository's
ignored `development/` stores private validation logs; it is not a runtime input.

## Generated files and Git

`.gitignore` excludes nested Cargo `target/` directories, profiling/coverage
outputs, native executables/libraries/debug symbols and installer packages.
Future Qt/CMake build outputs and Qt Creator per-user settings are excluded too;
this does not introduce a Qt build or installer implementation.

Keep `Cargo.lock`, shared build configuration, QAM resources and intentional PNM
`.bin` demos in Git. PNM binaries are measurement fixtures, not compiled programs.
`crates/ofdm/build/` contains maintained Rust build-helper source and must remain
tracked. Use root `build/`, an app's `build/`, or `cmake-build-*` for native build
trees; use root `dist/` or `artifacts/` for release staging.
