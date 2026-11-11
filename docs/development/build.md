# Development

Use Rust 1.85.1, Cargo, rustfmt and Clippy. Linux was validated
locally; macOS and Windows runtime/build validation remains outstanding.

```bash
./scripts/cargo-local.sh fmt --all -- --check
./scripts/test-local.sh all
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
./scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
```

A globally available Cargo works with rust-toolchain.toml. Alternatively, a
project-local Rust installation can live under ignored local/cargo and local/rustup.
[Development setup](setup.md) provides `install.sh --development` to install
missing Ubuntu packages and local tools. Check/dry-run do not install; no mode
changes shell profiles.
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
build script, CXX bridge or C++ source is included in the simulation build.
The separate [Qt shell](qt.md) uses CXX-Qt only for presentation.

A Rust-only build helper under `crates/ofdm/build/` validates and embeds the
preserved QAM resources. It introduces no additional packages or native build
dependencies. See [QAM resources and runtime behavior](../user/qam-mapping.md).

Private normative PDFs, evidence and migration snapshots live in ignored
`docs/development/local/`. They are not present in a fresh clone. Public plans
and validation summaries live in the [feature index](features/README.md). The specification skills
include versioned lookup extracts; diagrams still require the original PDF.

The integration/release branch is `master`; the original
`feature/ds-profile-fec-lab` branch is retained. Git operations use the
local helpers described in [Git and versioning](git.md).

Standard logging dependencies are pinned in Cargo.lock and downloaded by Cargo;
no operating-system packages or separate installation step are required.
See [logging configuration](logging.md).

## Channel job and constrained-VM validation

### Parallel local tests

`scripts/test-local.sh all` runs workspace tests, doctests, the preparation
example tests and the no-default-features analysis suite. It uses cargo-nextest
0.9.151 with four test processes by default, independently scheduling tests
across binaries. The 16 uniform QAM/spacing cases and eight CLI width/profile
cases are separate tests with unchanged assertions. Each CLI case owns its
temporary output directory. A pause/timing-sensitive test takes the full worker
budget; Qt/QML checks remain in their separate native validation path.

```bash
scripts/test-local.sh all
MERLIN_TEST_THREADS=8 scripts/test-local.sh all
scripts/test-local.sh workspace -E 'binary(uniform_profile_orders)'
scripts/test-local.sh list
```

The default full pre-push/release gate calls this runner automatically. Existing
`--skip-test` behavior is unchanged. First use downloads the exact SHA-256-pinned
Linux x64 musl runner into `local/test-tools/`; it does not install globally or
change Rust/Cargo versions. Cached archives are reverified on every launch.
Tests still execute every time; successful results are not cached. Cargo retains
and reuses build artifacts. Worker overrides accept 1..32; start at four on this
eight-CPU VM and increase only after measuring CPU/memory pressure.

Nextest prints individual results and slow-case times. The latest completed
phase's JUnit report is `target/nextest/local/junit.xml`; retain the full console
log when measuring all phases. See the [design and measured validation](features/parallel-local-tests/design.md).
The measured four-worker run on this PC completed all 460 tests in 5m19s,
including example/analysis compilation after the initial workspace inventory
build. This is the Rust test command's time, not the entire pre-push gate.

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

This runs the same checks; it changes build metadata/cache size, not simulation
algorithms. Normal development can keep debug information enabled. The repository's
ignored `docs/development/local/` stores private validation logs and generated outputs.
The [workspace guide](README.md) explains where ongoing development belongs.

## Generated files and Git

`.gitignore` excludes nested Cargo `target/` directories, profiling/coverage
outputs, native executables/libraries/debug symbols and installer packages.
Qt/CMake build outputs and Qt Creator per-user settings are excluded too.
The Qt shell has a separate build; installer implementation remains pending.

Keep `Cargo.lock`, shared build configuration, QAM resources and intentional PNM
`.bin` demos in Git. PNM binaries are measurement fixtures, not compiled programs.
`crates/ofdm/build/` contains maintained Rust build-helper source and must remain
tracked. Use root `build/`, an app's `build/`, or `cmake-build-*` for native build
trees; use root `dist/` or `artifacts/` for release staging.

## Optional constellation documentation plot

The simulator and paired collector require only the Rust dependencies already
in Cargo.lock. The optional presentation-only plot helper uses Matplotlib3.10.8
in ignored `local/doc-plots/`; no system installation is required. See the
[paired constellation guide](../user/impairments/paired-constellation.md#reproduce-the-documentation-plot)
for isolated setup and rendering commands. All DSP, impairment generation and
simulation results still come from Rust. The release gate now also checks8192
paired samples in its existing measured channel smoke run.

## Native CLI delivery checks

The local gate also runs the packaging and external-vector admission helper tests.
For prepared native workflows and package builders, see
[native validation/delivery](native-delivery.md). These workflows stay unpushed
until requested; passing this Linux gate does not establish Windows/macOS support.
