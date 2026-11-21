# INT-BASE-01: Pre-import MERLIN baseline

Executed 2026-11-20 on Linux x86_64, MERLIN
`9007feaf231f8f1b331baf6f7559efce89516f54`, source
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`. Both checkouts started clean.
Only traceability/tooling documents changed before these runs; downstream Rust,
fixtures, manifests and locks were unchanged. Original checked history is retained.

## Commands and results

| Check | Executed result |
| --- | --- |
| `scripts/test-local.sh all` | Passed in 11m11s: 439 workspace tests, four preparation-example tests, 139 analysis-only tests; doctest commands pass. Three actual Rust API examples pass in 110.14s including incremental build. No quick filter. |
| Existing ignored sparse microbenchmark | Explicitly executed separately with `cargo test -p fec-codec --release --locked --lib benchmark_encoder_methods_only -- --ignored --nocapture`: passed. This is the one ignored case in the workspace runner. |
| `scripts/cargo-local.sh fmt --all -- --check` | Passed. |
| `scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings` | Passed in 32.72s including compilation. |
| Native CPython 3.10 wheel | Built current 0.4.0 with existing locked wheel builder; installed without index/dependencies into a new private venv. All 25 Python tests passed in 27.373s against a fresh Rust four-policy reference. |
| Legacy CLI compatibility | Two actual `simulate` runs using unchanged `examples/cli/awgn.json` completed ten words each. Every JSON field matches except `elapsed_ms`; inputs, status, decoder counters, schema and seeded scientific outcomes agree. |
| Dependency audits | Existing audit command passed for core, Qt and Python locks: zero vulnerabilities/unsoundness findings, no suppressions. Each graph reports existing informational RUSTSEC-2026-0542, `humantime 2.4.0` unmaintained. No dependency changed to hide it. |
| Integration traceability | All 21 tasks have nonempty matching requirement lists, resolving definition/owner links and requirement/architecture return links. Eight regression cases pass; full validation-tool discovery passes nine cases. |

Wheel SHA-256:
`a6a83077064d35b26f3e7dd9b80de10e500639e5cde163a02610ba6ab5b6e41c`.
This identifies an ignored local baseline artifact, not a published wheel.

Raw commands, manifest/lock hashes, compiler identity, logs and JSON live under
ignored `docs/development/local/unified-backend/baseline-f1l07499/`. The first
runner attempt failed before executing tests because old local Rust links were
stale and the user toolchain lacked its compiler. Cached 1.85.1 components were
checked against the checksum-verified official 1.85.1 release manifest and unpacked
into a fresh ignored `local/toolchains/` directory. Scoped PATH/CARGO_HOME select
that toolchain and MERLIN's cache. No global installation, source pin/profile
change or artifact cleanup occurred. The failed attempt is not a test pass.

## Targeted downstream evidence reconciliation

- **DS-GAP-01:** Existing `public_paths` and standalone concatenated tests passed
  in the full workspace runner; the two fixed-seed CLI runs above add current
  compatibility evidence. This is a new scoped check, not a recreated old result.
- **DS-GAP-02:** Existing codec/normative/concatenated, mapping/source-table,
  cell-randomizer and transmission tests passed in MERLIN. They retain independent
  polynomial/matrix/label/recurrence/timing assertions and actual failure cases.
- **DS-GAP-03:** Recovered [the dedicated DPD viewer record](../pnm-measurement-inputs/profile-view.md).
  UI-DPD-VIEW-01 records binary/JSON/frequency/role/bounds checks, 27 adapter and
  32 QML passes plus native review. UI-DPD-VIEW-02 records full overlays/rates/
  filters and unchanged acceptance, 28 adapter and 32 QML passes plus native
  review. These are attributable historical Ubuntu results, not today's Qt run.

No UI code changed, so no Qt rebuild or historical UI process replay was needed.
Other installed/hosted/native tasks keep their existing states. This baseline
permits the next dependency-ordered integration work; it accepts no new waveform,
upstream capture decoder, Rust/Python upstream controller or final backend.
