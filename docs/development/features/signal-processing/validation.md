# INT-DSP-01 numerical integration evidence

[Architecture](../../../architecture.md#unified-shared-dsp-and-fec),
[requirement REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01),
[INT-DSP-01](../../checklist.md#int-dsp-01) and the
[ten-candidate design](design.md#int-dsp-01-implementation-slice) own this record.

Pre-edit base: `37be29ec2d536a96a504211fd49b20152e5555cb`.
Pinned source: `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`, read-only and clean.

## Executed pre-change baseline

`scripts/cargo-local.sh test -p docsis-ofdm-fec --locked --test waveform_transforms --test prbs_source`
with existing Rust 1.85.1 and explicit PATH/CARGO_HOME (see design) passed all
nine cases: four independent transform/shape/nonfinite/overflow tests and five
PRBS recurrence/period/chunk/verifier/transactional-rejection tests.
The initial helper attempt could not discover Rust through dangling local cache
symlinks. This was a discovery failure, not a failing scientific test. Reusing the
existing compiler and an isolated ignored cache resolved it without setup changes.

## Shared-kernel and consumer acceptance

Accepted for the shared kernels and existing downstream consumers after the
commands below passed in the unified MERLIN workspace. INT-US-CONFIG-01 is next;
upstream stages, public namespace exports and final backend acceptance remain open.

## Exact implementation and acceptance routing

| Evidence field | Current scope |
| --- | --- |
| Stable requirement / architecture | [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01); [shared DSP/FEC](../../../architecture.md#unified-shared-dsp-and-fec). Preservation also serves REQ-DS-PHY-01; safe ownership serves REQ-COM-SYS-03. |
| Design / checklist | [Ten-candidate audit](design.md#int-dsp-01-implementation-slice); [INT-DSP-01](../../checklist.md#int-dsp-01). Official checklist owns acceptance. |
| Crate / source | `merlin-dsp`; [transform.rs](../../../../crates/dsp/src/transform.rs), [prbs15.rs](../../../../crates/dsp/src/prbs15.rs), [complex.rs](../../../../crates/dsp/src/complex.rs), [error.rs](../../../../crates/dsp/src/error.rs); downstream adapters in `crates/ofdm/src/signal_processing/waveform/transform.rs` and `crates/ofdm/src/simulation/source/prbs.rs`. |
| Symbols / state | `TransformSize::samples`, `TransformPlan::{new,forward,inverse}`, `prbs15::step`; unchanged public FFT/IFFT wrappers and PRBS source/verifier/state/admission. |
| Test files | [equivalence.rs](../../../../crates/dsp/tests/equivalence.rs), [benchmark.rs](../../../../crates/dsp/tests/benchmark.rs), existing [waveform_transforms.rs](../../../../crates/ofdm/tests/waveform_transforms.rs) and [prbs_source.rs](../../../../crates/ofdm/tests/prbs_source.rs). |
| Exact new cases | `crates/dsp/tests/equivalence.rs::all_geometries_match_frozen_former_kernel_bit_for_bit`; `crates/dsp/tests/equivalence.rs::independent_centered_tones_and_impulses_cover_both_signs_and_scales`; `crates/dsp/tests/equivalence.rs::dense_grid_matches_independent_direct_sums_and_parseval`; `crates/dsp/tests/equivalence.rs::invalid_shape_nonfinite_overflow_and_rejection_leave_plan_reusable`; `crates/dsp/tests/equivalence.rs::prbs_matches_independent_literal_history_for_multiple_seeds_and_period_edges`; `crates/dsp/tests/equivalence.rs::prbs_full_period_and_arbitrary_chunking_match_former_recurrence`. |
| Command / observed result | See command table below; six shared-kernel cases pass in debug and release; all nine existing consumer cases pass before and after rewiring. Full Rust and installed native Python regression results are recorded below. |
| Commit / working candidate | Base `37be29ec2d536a96a504211fd49b20152e5555cb` plus code/build manifest SHA-256 `64b362dd1dea45252365adfffaae5fb3610cf4078086bd5a2833e9bbcdf27932`; individual file hashes are in [source inventory](../unified-backend/source-inventory.json). No later commit is substituted for this tested working candidate. |
| Status | Integrated-validated shared numerical kernel and downstream consumer slice; INT-DSP-01 is complete. No source-repository pass is transferred as a MERLIN stage pass. |
| Limitations | 2K is shared-kernel numerical support, not accepted upstream waveform/session support. No new facade/common-contract/Python namespace or Qt implementation; no external hardware/hosted/native Windows acceptance. |

## Reproducible build environment

The ordinary repository helper is used with this existing compiler/cache selection:

```bash
export PATH="/home/dev01/Projects/merlin/local/toolchains/rust-1.85.1-ylt882zx/bin:$PATH"
export CARGO_HOME="/home/dev01/Projects/merlin/local/backend-integration-cargo"
```

Rust 1.85.1 / Cargo 1.85.1, Linux x86_64; the compiler was already present.
Read-only source registry archives were copied into the ignored target cache;
Cargo fetched missing exact locked sources there. The original source checkout,
dangling discovery symlinks and existing target artifacts remain untouched.
The explicit environment fixes local discovery only; no setup/system install ran.

| Exact command | Observed result |
| --- | --- |
| `scripts/cargo-local.sh test -p merlin-dsp --locked` | Six independent/frozen-equivalence cases pass; explicit timing benchmark remains ignored in normal tests. |
| `scripts/cargo-local.sh test -p merlin-dsp --release --locked --test equivalence` | All six pass before downstream rewiring, including exact former-kernel bits, signed zero, invalid/overflow and concurrent plan reuse. |
| `scripts/cargo-local.sh test -p docsis-ofdm-fec --locked --test waveform_transforms --test prbs_source` | Nine pass before and after; existing test bodies unchanged. Private overflow tests also remain in the full suite. |
| `scripts/cargo-local.sh test -p merlin-dsp --release --locked --test benchmark benchmark_fft_and_prbs_kernels -- --ignored --nocapture` | Pass; correct values compared before timing. Detailed host measurements below. |
| `scripts/cargo-local.sh fmt --all -- --check` | Pass. |
| `scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings` | Pass. |
| `scripts/test-local.sh all` | Pass in 8m46s including compilation: 481 workspace tests (two explicit benchmarks skipped), four preparation-example tests, 139 analysis-only tests, doctests and three actual API examples. Example total 102.09s including build; no quick filter. |
| `local/api-python-build/bin/python tools/python/build_wheel.py --out docs/development/local/unified-backend/dsp-integration/wheels` | Existing maturin 1.8.3 built the locked CPython 3.10 Linux x86_64 wheel; only a private build environment was used. Cargo build concurrency was bounded to two. |
| `scripts/cargo-local.sh run -p pnm-input --example run_rxmer_selection_parity --locked -- docs/development/local/unified-backend/dsp-integration/rust-parity` | Fresh direct Rust results and original fixture inputs produced for all four static measurement policies. |
| Fresh private venv; wheel installed with `pip install --no-index --no-deps`; `python -m unittest discover -s /home/dev01/Projects/merlin/bindings/python/tests -p 'test_*.py' -v` from `/tmp` | All 25 native tests pass in 29.084s with `MERLIN_API_PARITY` set to the fresh Rust directory. Confirmed actual installed `.so`, version 0.4.0 and `Merlin is Simulation`. Tests execute real science, ownership, GIL/Busy/cancel, workbench/Designer and parity cases. |
| Version and documentation tooling | Canonical version check and generated Python guide check pass; 35 validation-tool cases and five release/version-tool cases pass. |

## Hot-path and allocation evidence

The first separate-crate timing run observed inverse slowdowns of 1.715x (2K),
1.327x (4K) and 1.088x (8K). Callers were not rewired at that point. Explicit inline
boundaries preserve the former same-crate direction specialization; release
correctness was rerun before replacement. The final benchmark alternates old/new
execution order across fifteen paired batches after warmup, retaining median
microseconds per operation. Plan setup includes allocations/root construction;
inverse includes result allocation/validation but excludes plan setup. PRBS includes
a one-Mibit output allocation. It compares identical inputs and former arithmetic;
this is host evidence, not a DOCSIS throughput or universal no-overhead guarantee.

| Operation | Former median µs | Shared median µs | Shared/former |
| --- | --- | --- | --- |
| 2K plan setup | 12.402 | 12.896 | 1.040 |
| 2K inverse | 24.084 | 24.110 | 1.001 |
| 4K plan setup | 25.241 | 25.269 | 1.001 |
| 4K inverse | 64.654 | 70.367 | 1.088 |
| 8K plan setup | 67.517 | 68.670 | 1.017 |
| 8K inverse | 146.735 | 135.497 | 0.923 |
| PRBS 1,048,576 bits | 1,146.939 | 1,162.840 | 1.014 |

The 4K sample remains about nine percent slower in this host run; other timing
samples vary. No guaranteed speedup or exact wall-time parity is claimed, and no
numerical assertion/tolerance or golden vector was weakened. Static allocation
review retains two plan vectors and one result vector per FFT; the scalar PRBS
step allocates nothing. Allocation counts were reviewed from code, not measured
with an unsafe global allocator. PHY metadata, typed errors and source positions
remain outside the shared kernel.

## Preservation and final boundaries

Wheel SHA-256: `4b5f1c39f05dbe9da2dd759237f4a42c289a49054f8f7c422dd1feda682958ae`.
This identifies the newly tested private artifact, not a published release.
The fresh venv is under ignored `docs/development/local/unified-backend/dsp-integration/python-acceptance`;
no global package installation occurred. No optional external JSON report was set
for WB/Designer tests: their actual native scientific assertions ran, alongside
the explicitly refreshed four-policy Rust/Python comparison. Namespace-specific
installed imports remain future INT-PY-01 acceptance, not this wheel's capability.

All original package versions/sources/checksums in all three locks remain; only
local `merlin-dsp` 0.4.0 and its downstream edge are added. Canonical VERSION and
package metadata remain 0.4.0. The version helper now includes the new local crate.
Existing Rust/native API source exports and Python implementation files are unchanged.
The only existing scientific source changes are the private downstream transform
adapter and the minimal PRBS recurrence delegation. All original downstream
acceptance-test bodies, QAM resources, FEC definitions/matrices and source vectors
remain byte-identical. The original source checkout remains clean and pinned.

The full-regression code/build fingerprint above remained unchanged through the
Rust/native checks. After those checks, the frozen test fixture constructor banner
was corrected from the former `spacing` parameter to its actual bounded `length`
parameter. The numerical code is byte-identical after stripping that exact comment
change; all six kernel cases and formatting were checked again. Current working
manifest SHA-256 is `fbc568a61f5d729925d75dbd61cd972f45d900f3f9a8424f26a338cc0140d90a`; both before/after fixture identities and the
full-regression digest are retained in the source inventory. No scientific
implementation or original acceptance assertion changed after full validation. Completed
planning/FEC/history stays accepted; only the new namespace mapping task and the
now-validated DSP task gain completion. No upstream runtime, facade/common API,
new Python namespace, Qt implementation, hosted CI or installed desktop/Windows
behavior is claimed. Backend final acceptance remains open.
