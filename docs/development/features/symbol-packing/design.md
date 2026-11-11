# PACK-03: Analysis-only packing build hygiene

The coded-bit length is used exclusively by simulation-feature constructors,
profile activation and binary enqueue validation. Gate its private field and
fixed-mode initializer with the same feature. Analysis-only label packing keeps
its existing state and behavior; simulation retains its existing length checks.
No public API, algorithm, test point or persistence contract changes. Do not
suppress dead-code diagnostics. Own the change in
`crates/ofdm/src/transmission/packing/packer.rs`.

Validate formatting, warnings-denied Clippy without default features, and the
existing symbol-packing integration tests with simulation enabled.

Validation: workspace rustfmt passes; no-default-features Clippy across all
available targets passes with warnings denied. All five existing symbol-packing
integration tests pass with default simulation features. Full suites and release
checks were not rerun. The interrupted 0.1.2 version edits remain untouched.
