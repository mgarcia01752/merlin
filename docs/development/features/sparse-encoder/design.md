# FEC-SPARSE-01: sparse systematic encoder setup

The O(n^3)-type operation is upstream Encoder::from_h's general dense Gaussian
reduction, not SparseMatrix storage. Payload already selects the upstream linear
staircase encoder. NCP and PLC parity-check matrices are block-lower-triangular
with permuted identity diagonal blocks; they do not need dense elimination.

Before changing codec execution, measure the existing upstream setup and encode
methods alone on deterministic synthetic matrices. No capture, Qt, channel or simulation runner is part of that benchmark.
Use the release compiler profile to measure the methods, excluding compilation.

Add a private sparse substitution encoder for peelable parity submatrices. Treat
information columns as known; queue checks with one unresolved parity coordinate,
record a pivot/equation and update neighboring check degrees via existing sparse
column adjacency. Store each equation once and evaluate it in solved order.
Construction and encoding are O(rows + columns + nonzero entries), with sparse
storage. Reject non-peelable/cyclic parity matrices explicitly; this helper is
not a general Gaussian solver. NCP/PLC fixed supported matrices must pass all
existing independent basis/oracle tests. Retain the current payload encoder and
all decoders, matrices, shortening/puncturing, iterations and public APIs.

Do not modify Cargo registry sources or introduce C/C++/Python calculations.
Use safe Rust, preserve bit ordering and invalid-input behavior. Test hand-solved
small/permuted systems, invalid shapes, cyclic parity rejection, deterministic
synthetic upstream parity and zero syndrome. Record before/after setup and encode
wall timings without timing assertions. Then integrate into NCP/PLC and run their
full codec tests and a focused actual engine check. Existing normative matrix
contracts remain unchanged; no new specification interpretations are needed.

## Isolated validation and timings

Measured locally on Linux x86_64 KVM (8 vCPUs, host-reported i7-13700HX), pinned
Rust/Cargo 1.85.1, release profile. Synthetic block-shift matrices use 80 through
1536 parity coordinates and information length 1.5 times parity length. These
are fake inputs, not capture contents or claims about arbitrary sparse systems.

First run the upstream-only test and benchmark before changing codec execution.
The original largest case measured 19.200 ms setup and 984.510 µs per encoding.
After the sparse helper passed its method-only tests, measure both implementations
in one process. Means below include allocation/destruction of each returned
object, five setup repeats and 100 encode repeats. Scheduling noise is visible;
there are no elapsed-time assertions or guaranteed performance thresholds.

| Parity bits | Upstream setup µs | Sparse setup µs | Upstream encode µs | Sparse encode µs |
| --- | ---: | ---: | ---: | ---: |
| 80 | 536.347 | 9.269 | 2.610 | 0.419 |
| 192 | 291.305 | 12.819 | 13.022 | 0.570 |
| 384 | 1886.680 | 69.519 | 86.361 | 3.122 |
| 768 | 7213.141 | 84.151 | 271.486 | 4.396 |
| 1536 | 18959.328 | 132.332 | 946.919 | 5.721 |

The largest same-process case is approximately 143× faster at setup and 166×
faster per encoding. This measures the specialized encoder only. It does not
measure decoder runtime, Qt responsiveness or total simulation speed.

Reproduce from the repository root:

```bash
scripts/cargo-local.sh test -p fec-codec --release --locked --lib sparse_encoder::tests -- --nocapture
scripts/cargo-local.sh test -p fec-codec --release --locked --lib sparse_encoder::tests::benchmark_encoder_methods_only -- --ignored --nocapture
scripts/cargo-local.sh test -p fec-codec --release --locked
```

Four isolated unit tests pass: upstream systematic/zero-syndrome baseline,
all hand-solved two-bit words with permuted rows, malformed/non-peelable systems,
and bit-exact upstream comparisons at all five synthetic sizes. Inputs and
matrices remain unchanged and plans can be reused. The benchmark is ignored by
default, explicitly selectable and has no simulation or file I/O.

After integration, all 38 codec tests pass, including the independent NCP and
PLC parity/basis oracles, error correction, punctures and actual failure cases.
A cached full codec run took 1.113 s including Cargo; the explicit benchmark
ran in 0.33 s (1.195 s including Cargo/cache contention). Compilation is separate;
the initial integration build took 7.42 s. Formatting and codec all-targets
Clippy with denied warnings pass. Full workspace and Windows/macOS runs are not
part of this focused validation. Private raw logs are in ignored
`docs/development/local/sparse-encoder/{before,after,codec-tests}.log`.

The rebuilt Rust `pnm-input` example `run_single_capture` also completed with
exact observation-window anchors: 82 clean codewords, zero corrected or
uncorrectable words and zero bit errors. Direct execution took 0.704 s, excluding
the 31.55 s rebuild. This is a correctness smoke, not a before/after simulation
benchmark; its summary is retained in ignored `api-check.json`.
