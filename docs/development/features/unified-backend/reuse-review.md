# INT-LICENSE-01 / INT-REUSE-01: Pre-import decisions

Reviewed source `69b38071cd3ebebc4eb96849f82b7871ab8da5ec` against the
[executed MERLIN baseline](baseline.md). The
[file and dependency inventory](scientific-source-review.json) records 133
scientific source/resource files with exact hashes, declared licenses, decisions
and concrete target paths. Paths for future phases are proposed, not installed.
Source remains read-only; do not copy its workspace, product wrappers or locks.

## Rights and dependency boundary

All 117 Rust files in the reviewed scientific scope retain Apache-2.0 banners.
The source root LICENSE/NOTICE explicitly covers owned scientific code/tooling;
its PDE instructional exception remains excluded from this import scope. Retain
original holders, source revision and actual modifications in MERLIN NOTICE.
Existing Rust BCH MIT obligations and GPL Qt separation remain unchanged.

Selected raw QAM tables and their full Apache LICENSE/NOTICE are byte-identical
to existing MERLIN resources; reuse that data and its preserved PyPNM provenance.
Source UCD/RxMER/Pre-Eq/measured fixture READMEs identify invented authored inputs,
not customer/modem captures or specification pages. Retain their authorship,
hashes, limits and independent expected bytes when their adapter phase migrates.
Do not import private PDFs/corpora, real captures or generated artifacts.

Locked host metadata contains 77 source packages and 97 target packages; every
source-only package is project-owned Apache-2.0. External source dependencies
already occur in MERLIN. Preserve target versions: source differs in ctrlc,
nix, clap, clap_derive, clap_lex, syn and itoa. Do not import source patch versions
or lockfiles. Existing legal-text collection and future artifact inspection remain
mandatory under INT-TOOLS-01; metadata review is not distribution acceptance.
The baseline records current three-lock audit results, including the informational
humantime notice. No suppression or unsupported blanket compliance claim is made.

## Function-level reuse decisions

| Inspected owner/symbol | Semantics and evidence | Decision / target |
| --- | --- | --- |
| Source `ldpc::SparseEncoder::{from_h,encode}`; MERLIN `fec-codec::sparse_encoder::SparseEncoder::{from_h,encode}` | Same immutable row-degree queue, pivot order and systematic substitution. Source encodes binary u8; existing owner encodes GF2 arrays. Existing independent dense/substitution and syndrome tests pass. | **REUSE** the existing plan; add a validated-caller byte-buffer encoding operation in `crates/fec-codec/src/sparse_encoder.rs`. Keep GF2 hot path/state unchanged. No second upstream sparse planner. Verify byte results against independent dense and frozen source parity at INT-FEC-01. |
| Source `ldpc::DocsisLdpcCode::{parameters,matrix}` and `LdpcCodec::{encode,decode}`; downstream `parameters`/`ldpc`/BCH | Matching long-code dimensions do not imply matching parity matrices. Source has all three audited upstream definitions, retained-zero/failed-decision checks and independent frozen row/edge/impulse expectations. | **SEPARATE** definitions/codec under `crates/fec-codec/src/upstream_ldpc/{mod,definitions,codec,error}.rs`; retain `tests.rs` and `tests/{oracle,source_audit,source_fixtures}.rs`. No ordinary upstream BCH or replacing downstream imports. |
| Source `signal_processing::Transform::{forward,inverse}`; downstream waveform `TransformPlan::apply` | Same centered unitary radix-2 math, different typed samples/admission and 2K/4K versus 4K/8K geometry. Existing independent transform/tone/energy tests pass. | **SHARE** numerical kernel at `crates/dsp/src/transform.rs`, with representation support in `complex.rs`; retain PHY wrappers/errors. INT-DSP-01 must prove 2K/4K/8K values, sign/normalization, bounds, allocations and timings before rewiring downstream. |
| Source `source::Prbs15::generate`; downstream `simulation::source::Prbs15::generate` | Same x^15+x^14+1 recurrence/LSB-first seed, but source identity/chunk admission and independent comparison differ. Mathematical full-period/recurrence tests exist. | **SHARE** recurrence/state primitive in `crates/dsp/src/prbs15.rs`; keep source ranges/metadata/admission/verifier in each PHY. Not a PHY scrambler. INT-DSP-01 requires rejected-call/chunk/reset/period equivalence. |
| Source QAM resources/build helper and direction-specific mapping/LLRs; current reference/indexed demapper | Common raw resources have exact matching hashes; source also supports direction-specific 8/32-QAM. Clipping, units, typed errors and supported orders differ. Downstream has optimized indexed search. | **REUSE** current data; **ADAPT** common table generation under the DSP owner where justified. Keep direction-specific loading/label policy and optimized downstream behavior; no generic DVB replacement or unproved demapper substitution. `crates/dsp/src/modulation.rs` is gated by INT-DSP-01. |
| Source `channel::Channel` / `RxmerChannel`; downstream channel/filter/noise/measurement owners | Output-referred noise, source C/H/unity restrictions, timing/drain and transactional schedule admission differ from downstream channel composition. Both already use the same pinned Rust Gaussian/RNG dependencies. | **SEPARATE** direction-specific stream/measurement state in `crates/ofdma/src/{channel,measurements}/`; reuse established Rust dependencies. No universal channel or relaxed C/H/RxMER restrictions. Defer further kernel sharing until exact state/draw/order equivalence is demonstrated. |
| Source capture parsers/projection and current pinned `pnm` adapter | Parsing, association and simulation are different contracts. Existing decoder reuse requires verified actual file-type/version support, not matching names. | **ADAPT/REUSE** under `crates/pnm-input/src/upstream/` at INT-PNM-01/02. Core PHY receives typed values, never depends back on the native adapter. Preserve originals, metadata, source fixtures and explicit whole-plan preflight. |
| Source facade/lib FEC-only product controller, API and examples; existing MERLIN adapters | A new obsolete product controller would duplicate composition and break the unified identity. Source raw input/report semantics remain relevant. | **REUSE/ADAPT** core operations and report semantics; exclude the obsolete controller/package identity. Initial upstream API lives in `crates/ofdma/src/api`, file admission in `pnm-input`; final Rust/Python signatures remain INT-RUST-01/INT-PY-01. No early FEC-only Python adapter. |

The file inventory settles current owners and future adaptation destinations;
collapsing the source facade is explicit, not a second runtime dependency. Public
names and schema compatibility remain reviewed at their own API gates. No source
algorithm, normative table or test assertion is discarded just because wrappers
are replaced. Follow the existing 23-stage architecture and dependency order.

## Measured baseline and first implementation gate

The existing release sparse microbenchmark passed independently of compilation:
at parity sizes 80/192/384/768/1536, sparse setup measured
11.194/25.221/76.223/103.064/99.732 microseconds and encoding
0.589/0.508/0.989/1.924/4.473 microseconds. These synthetic host measurements are
pre-change anchors, not DOCSIS throughput claims. The old GF2 encoder path must
stay unchanged; new byte encoding needs equivalence/allocations and independent
upstream vectors before INT-FEC-01 closes. DSP sharing waits for its separate
equivalence/performance acceptance; no speculative shared-crate stubs are added.

INT-REUSE-01 closes this inspected review/file plan, not numerical integration.
With INT-BASE-01 and INT-LICENSE-01 satisfied, the next implementation is
INT-FEC-01: add the three source-checked upstream families, reuse the sparse plan,
retain every independent test and run actual MERLIN/downstream regressions.

That first step now has [MERLIN acceptance](fec-validation.md). The remaining
DSP and PHY decisions retain the separate gates above.
