> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/validation.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# LDPC source audit and codec acceptance

Date: 2026-11-20. LDPC-01 and LDPC-02 are complete in the
[official checklist](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/checklist.md). The [design](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/design.md) owns the reuse
map, independent expectations and requirement links. Baseline: `04875ac`.
Execution: Linux x86_64, kernel 6.8.0-149-generic, Rust/Cargo 1.85.1, existing warm
Cargo caches and unchanged dev/test profiles. No dependency or lockfile changed.

## Original-source review and corrections

Original [CableLabs PHY I20](https://account.cablelabs.com/server/alfresco/4eb470eb-aca3-4e64-af88-773b8c594fbd),
CM-SP-PHYv3.1-I20-230419: visually inspected Table 8 p52, systematic encoding and
right-shift definition p56, every matrix cell p57, and shortening p58. Enlarged
long/medium crops resolved small values and individual column boundaries.
Original PDF SHA-256:
`304c26937f5beaf7ffddc88158abacead4ead2bc29e6acaae5226898c9f716bc`.

All 490 cells were separately transcribed from the page before comparison with
production. Independent local transcription SHA-256:
`1da9a79e0780d99bcf2e6ec1c945ce22797b0d6328151437c4e888ea2cf15301`.
Source PDFs, page images, crops, full transcription and logs remain ignored under
`docs/development/local/ldpc-source-audit/`. Tests ship compact derived fixtures
and require none of these private inputs or a reference checkout.

| Family | Audited cells | Nonzero QC blocks | Lifted edges | Correction |
| --- | ---: | ---: | ---: | --- |
| Long | 225 | 169 | 60,840 | Row 5: insert shift 12 at zero-based column 20, restore subsequent column positions and remove a spurious dash. Nineteen cell coordinates change. |
| Medium | 165 | 131 | 23,580 | Row 4: zero-based columns 17/18 become dash/67, replacing 67/158. Row 5: restore shift 2 at column 16 and subsequent positions; remove a spurious dash. Fourteen cell coordinates change across both rows. |
| Short | 100 | 79 | 4,424 | No source discrepancy found; retain the original constants. |

Every family now matches the separate transcription exactly. Public mother-code
identities/dimensions, shortening domain and iteration limits remain unchanged.
Corrected long/medium words can differ from the checkpoint for identical inputs;
that is the intended scientific correction. Existing report/CLI schema 1 and
Rust imports remain compatible. Historical words made with incorrect matrices
are not asserted to satisfy the corrected code.

## Independent regressions and decoder behavior

`crates/ldpc/src/tests/source_fixtures.rs` contains fifteen base-row fingerprints,
three full lifted-edge fingerprints and twenty frozen impulse parity supports.
The standalone Rust source-reference example derives these from the independent
transcription using inverse column displacement and polynomial rotation/XOR,
without importing production constants, matrix construction or encoder helpers.
The fingerprint encoding is specified in [design](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/design.md). It guards accidental
changes; it is not a cryptographic proof or an external vector set.

Before correction, the long/medium source-fingerprint and frozen-impulse tests
failed, while short source/parity checks passed. That first run also exposed an
overstrict new erasure-test assumption: zero LLR ties can already form a valid
word at iteration zero. The assertion was corrected after inspecting the pinned
decoder's actual tie behavior; no decoder algorithm or iteration count was
changed. The complete pre-correction log is retained locally.

After correction, all source fingerprints and frozen words pass. Tests encode
and decode source-derived mother and shortened words directly, including I=1,
block boundaries, affected information columns and full K. Existing Gaussian
elimination remains an independent arithmetic oracle, but still reads production
table data; it supplements rather than replaces source fixtures.

All three families additionally pass:

- Exact systematic order, independent Gaussian parity, zero syndrome, individual
  corrupted information/parity detection and shortening I=1, 420, K-1 and K.
- Exact noiseless recovery, deterministic controlled soft errors, finite ±50
  saturation including f64 extremes, signed-zero erasures and actual iterations.
- One-iteration nonconvergence retaining failed decisions, subsequent clean
  recovery, repeatable failure after reuse and preserved owned prior results.
- Typed length/nonbinary/nonfinite rejection across information and parity
  positions; accepted decoder behavior survives invalid calls.
- A valid mother word violating a restored shortening zero: convergence and
  syndrome can pass while shortened-zero consistency fails. Reports count this
  codeword as failed even when retained payload BER is zero.
- A different valid codeword: decoder checks can pass while independent source
  comparison reports BER=1. Reports preserve those distinct outcomes.

Deterministic real BPSK/AWGN cases use seed `0x41474e4c`, unit amplitude,
real variance 0.16 and natural-log LLR=2y/variance. Long/medium/short showed
101/45/14 initial sign errors and recovered exactly in 2/1/2 iterations.
Controlled three-error cases recovered in one iteration for every family.
These are codec experiments, not measured OFDMA RxMER.

## Executed validation

`bash scripts/validate.sh` passed: 20 skills and local guidance/navigation,
5 isolated Git-helper tests, 19 setup tests, shell syntax, formatting,
35 LDPC plus 3 facade unit tests, doctests and all-targets Clippy with warnings
denied. The final measured gate took 2.74 seconds, checked 878 local links and
reported 0.39/0.07 seconds for codec/facade cases and 0.675/0.772 seconds for
Git/setup cases. Build caches were warm; these are local observations.

Additional executed checks:

```bash
scripts/cargo-local.sh build --workspace --examples --locked
scripts/cargo-local.sh build -p morgana-cli --locked
scripts/cargo-local.sh run -p morgana-ldpc --locked \
  --example ldpc-source-reference -- \
  docs/development/local/ldpc-source-audit/source-rows.txt
git diff --check
```

Regenerating from the private independent transcription and formatting with the
pinned rustfmt reproduced the checked-in fixture byte for byte. Twelve bounded
CLI smokes (long/medium/short × I=1,420,K-1,K) preserved schema-1 settings,
source metadata, complete mother decisions, zero payload errors and null RxMER.
The existing `ldpc_loop` public Rust example passed. Source-reference invocation
checks rejected missing/extra arguments and missing input without emitting data.
Normal offline Cargo tests use the frozen fixtures, never private source files.

## Boundaries

Only source-verified standalone PHY 3.1 codec acceptance is complete. No full
waveform, grant selector, PRBS source, high-level session, native Python, captured
PNM, external interoperability, DOCSIS 4.0 mode, hosted CI or cross-platform
acceptance is claimed. The first unfinished task is DEV-CORE-01. No user index
entries, commits, pushes, system packages or reference repositories were changed.
