# Development

Use Rust1.85.1, Cargo, rustfmt, Clippy and a C++17 compiler. Linux was validated
locally; macOS and Windows runtime/build validation remains outstanding.

```bash
./scripts/cargo-local.sh fmt --all -- --check
./scripts/cargo-local.sh test --workspace --locked
./scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
./scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
./scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
```

A globally available Cargo works with rust-toolchain.toml. Alternatively, a
project-local Rust installation can live under ignored local/cargo and local/rustup.
No script in this checkout installs system packages or changes shell profiles.
On the current VM those two paths reference the existing PNM Viewer development
toolchain/cache; build outputs remain in this repository's target/.

For an independent local toolchain, use rustup with CARGO_HOME and RUSTUP_HOME
pointing at this checkout's local directories, `--no-modify-path` and toolchain
1.85.1. Do not replace the current cache links while they are in use. Downloading
Rust is distinct from installing host C++ compiler prerequisites.

Cargo.lock pins the dependency graph. ldpc-toolbox0.10.1 was selected because
newer inspected releases use Rust APIs unavailable on the pinned toolchain.
The CXX toolchain is pinned to1.0.158. Native BCH source is vendored with its
upstream MIT notice and revision; no build-time upstream checkout occurs.

Private normative PDFs, evidence and migration snapshots live in ignored
`development/`. They are not present in a fresh clone. The specification skills
include versioned lookup extracts; diagrams still require the original PDF.

The initial branch is feature/ds-profile-fec-lab in the new repository, which had
no commits when migration began. PNM Viewer source baseline was
5fbd5561f3e06b73486546b5ffa520279080a5eb. No commit or push was performed.
