# INT-FEC-01: Standalone upstream LDPC acceptance

Executed 2026-11-20 in MERLIN's working candidate based on
`9007feaf231f8f1b331baf6f7559efce89516f54`, Linux x86_64, scoped Rust 1.85.1.
Source remains clean at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
The [pre-import baseline](baseline.md) and [reuse/rights review](reuse-review.md)
preceded this implementation. No source-repository result was transferred as a pass.

## Implemented boundary and traceability

[Architecture](../../../architecture.md#unified-shared-dsp-and-fec) and
[INT-FEC-01](../../checklist.md#int-fec-01) own this acceptance for
[REQ-US-TX-02](../../../requirements.md#req-us-tx-02),
[REQ-US-RX-08](../../../requirements.md#req-us-rx-08),
[REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05),
[REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01) and
[REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01).

The existing `fec-codec` crate now exports `upstream_ldpc` with distinct long
16200/14400, medium 5940/5040 and short 1120/840 definitions. The codec retains
systematic order, trailing-zero shortening, finite positive-for-zero LLRs,
1..100 iteration budgets and actual failed decoder decisions. Parity, convergence,
known-zero consistency and independent original-source comparison remain distinct.
Ordinary upstream data acquires no BCH or ranging puncturing.

The private sparse encoder plan is shared. Its existing GF2 encoder and plan
construction bodies are unchanged; a byte-buffer operation follows the same
immutable equations. Existing public codec imports remain intact. This accepts
standalone coding only. Grant policy, job controls around codec calls, upstream
waveforms/receivers and Rust/Python upstream operations await their own gates.
The master integration guide deliberately defers an upstream Python adapter until
the real Rust upstream loop; no separate FEC-only Python product was introduced.

## Executed checks

| Command/check | Observed result |
| --- | --- |
| `scripts/cargo-local.sh test -p fec-codec --locked` | 74 passed: all 35 imported upstream cases, a new byte/GF2/dense-solver equivalence case and 38 existing cases. The explicit microbenchmark remains ignored in ordinary runs. |
| `scripts/test-local.sh all` | Passed in 9m42s: 475 workspace tests, four preparation-example tests and 139 analysis-only tests. Doctest commands pass. Three actual Rust API examples pass in 116.51s including incremental build; no quick filter. |
| `scripts/cargo-local.sh fmt --all -- --check` | Passed. |
| `scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings` | Passed, including the final benchmark extension. |
| Existing locked wheel builder and isolated native Python discovery | Rebuilt current 0.4.0, installed without index/dependencies into a fresh private venv. All 25 Python tests passed in 25.470s against the pre-import Rust four-policy reference. This checks preserved downstream Python behavior, not upstream Python support. |
| Explicit release encoder microbenchmark | `scripts/cargo-local.sh test -p fec-codec --release --locked --lib benchmark_encoder_methods_only -- --ignored --nocapture`: passed; includes new byte operation and independent dense-result comparison. |
| Integration traceability tool and validation-tool discovery | All 21 integration task/requirement/architecture links pass; nine tool tests pass, including eight traceability cases. |
| Preservation/dependencies | All eight adapted source files differ only by declared ownership/import/shared-helper/provenance changes and formatting. Production matrices, frozen row/edge/parity expectations and oracle logic remain unchanged. All 123 root lock package identities are preserved; only three already pinned test dependency edges were added. Qt/Python locks are unchanged. |

The first targeted compile found a test still naming the removed private source
encoder module. Its call now reaches the shared MERLIN helper and maps the same
matrix-definition error; the original rejection assertion remains. That failed
compile is retained in the local logs and is not counted as a test pass.
After the full run, only the existing ignored benchmark and a provenance comment
changed; the explicit release benchmark, formatting and Clippy cover those edits.

Wheel SHA-256:
`0a9aedf6d1ebf6f0ba97ada59a78c1059b34534ec5ac619cac8839661f27c97b`.
It identifies the tested local artifact, not a publication.

## Numerical and allocation evidence

The source-audit cases independently check every declared base-row fingerprint,
expanded edge count/fingerprint and frozen sparse parity response for all three
families. The prior independent original-page transcription and limitations are
retained in [historical source evidence](references/ldpc-source-audit-validation.md).
No private PDF was re-audited here and no expected values were regenerated.
Controlled errors, erasures, shortening boundaries, invalid inputs, iteration
bounds, a wrong valid codeword and failed decisions run inside MERLIN. The BPSK
Gaussian oracle is a codec experiment, not OFDMA RxMER or external interoperability.

Byte encoding allocates one owned codeword buffer and traverses the same sparse
equations as the source operation; no per-edge allocation or GF2 conversion was
added. The surrounding padded-information, output and decoder allocations retain
the bounded source behavior. This is an implementation inspection, not allocator
profiling. Reuse, unchanged input and dense-solver equality are tested separately.

| Synthetic parity bits | Existing sparse GF2 encode (µs) | Byte encode (µs) |
| --- | --- | --- |
| 80 | 0.255 | 0.204 |
| 192 | 1.151 | 0.377 |
| 384 | 2.966 | 0.953 |
| 768 | 1.804 | 1.211 |
| 1536 | 4.737 | 3.351 |

These are 100-call host averages from the bounded release microbenchmark;
setup is excluded. Host scheduling varies, so they establish neither a universal
speedup nor end-to-end DOCSIS throughput. The pre-import record retains separate
setup/encoding measurements for comparison.

## Evidence location and remaining gates

[The file review](scientific-source-review.json) retains source and integrated
target hashes for this slice; future scientific imports remain reviewed candidates.
Raw logs, exits, wheel, structural/source comparisons and identities remain in
ignored `docs/development/local/unified-backend/baseline-f1l07499/`.
Use the scoped toolchain/cache selection described in [the baseline](baseline.md).

INT-FEC-01 is complete in MERLIN. INT-DSP-01 is next; upstream PHY, PNM, API,
CLI/tooling, final regression, documentation and backend acceptance remain open.
No downstream simulation owner, Qt implementation, original capture/vector,
external package version or completed historical task was replaced. No Qt,
installed-platform or hosted-CI result is claimed by these runs.
