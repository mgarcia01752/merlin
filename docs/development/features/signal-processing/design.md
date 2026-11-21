# Shared DSP conventions and reuse boundaries

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-shared-dsp-and-fec).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Architecture and support states

This feature owns numerical conventions and reuse decisions shared by the PHYs.
Existing downstream interfaces remain in place. INT-REUSE-01 accepted the inspected
placement/reuse plan. `crates/dsp` now implements the bounded FFT kernel and PRBS15
recurrence; downstream wrappers delegate to them after shared numerical acceptance.
INT-DSP-01 is accepted with [full consumer evidence](validation.md). No
direction-specific channel/QAM/measurement state is extracted by this slice.

| Mechanism | Boundary to prove before sharing | Direction-specific owner retained |
| --- | --- | --- |
| Complex IQ / FFT / IFFT | Centered physical bins, forward/inverse sign, unitary scale, valid finite shapes, energy and phase | Numerology, useful-window/CP selection, burst assembly and pilots |
| QAM / distance / LLR | Source labels, normalization, bit order, LLR sign/variance, clipping and unsupported loads | Resource visits, role-dependent loading and coding policy |
| AWGN / FIR / phase | Complex variance convention, output reference plane, seeded RNG draw order, causal state and exact drain | Channel admission, response/measurement policy and timing state |
| Power / RxMER arithmetic | Units, available/excluded bins, noise/signal reference plane and aggregation weights | Capture association and measured versus simulated meanings |
| PRBS primitives | Recurrence, seed/bit ordering, absolute phase and arbitrary chunk continuity | Source identity/verifier, grant randomizer and PHY scrambler |

## Inputs, outputs and invariants

Numerical leaves consume validated typed samples/bits/grids and explicit parameters;
return owned results or bounded updates without filesystem, Qt or Python state.
Validation precedes state/RNG/source consumption. Missing carriers/measurements stay
unknown or excluded; no generic zero-fill policy is inferred. Units and positive
LLR convention travel with the contract, not caller guesses.

A unitary centered transform has inverse scale 1/sqrt(N) and forward scale 1/sqrt(N)
with opposite signs. Physical-bin ordering and exact DC/edge treatment must agree
with independent vectors. Upstream 2K/4K and downstream 4K/8K geometries do not become
one PHY configuration. Float tolerances, operation order and serialization behavior
must be explicit before rewiring downstream consumers.

PRBS15 source continuity is distinct from grant-seeded 23-bit randomization and
other PHY scrambling/pilot generators. FIR delays, tails and phase/RNG state must
survive arbitrary input chunks; rejected calls and passive observation consume no
extra draws. Shared helpers do not own the full directional receiver/channel.

## Planned placement and acceptance

Current numerical owners are [complex.rs](../../../../crates/dsp/src/complex.rs),
[transform.rs](../../../../crates/dsp/src/transform.rs) and
[prbs15.rs](../../../../crates/dsp/src/prbs15.rs), with
[equivalence.rs](../../../../crates/dsp/tests/equivalence.rs). Generic modulation,
channel, noise and measurement modules remain uncreated; the candidate decisions
below keep their current directional owners. Preserve downstream public re-exports and independent
vectors. Benchmark changed hot kernels with setup/allocation scope stated; do not
regenerate golden results to make an extraction pass.

Use independent impulse/DC/tone/direct-sum checks, known QAM labels/soft-bit signs,
hand convolution, complex Gaussian variance and frozen PRBS phase vectors. Then
exercise actual consumers and unchanged downstream serialization/seeds. A forward/
inverse round trip or shared production oracle alone cannot prove the convention.

## Normative ownership

PHY-specific transforms/pilots/framing use their pinned PHY 3.1 I20 §7.4.4–10
(upstream) and existing downstream feature sources. Generic arithmetic is an
engineering mechanism, not a new normative receiver. The existing PHY 4.0 edition
skill controls any explicitly scoped delegated/FDD/FDX applicability.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-DSP-01](../../checklist.md#int-dsp-01) | Module: [crates/dsp/src/transform.rs](../../../../crates/dsp/src/transform.rs); Module: [crates/dsp/src/prbs15.rs](../../../../crates/dsp/src/prbs15.rs); Test: [crates/dsp/tests/equivalence.rs](../../../../crates/dsp/tests/equivalence.rs); Module: [crates/fec-codec/src/sparse_encoder.rs](../../../../crates/fec-codec/src/sparse_encoder.rs); Module: [crates/ofdm/src/signal_processing](../../../../crates/ofdm/src/signal_processing); Test: [crates/fec-codec/src/sparse_encoder/tests.rs](../../../../crates/fec-codec/src/sparse_encoder/tests.rs) |
| [REQ-COM-SYS-03](../../../requirements.md#req-com-sys-03) | [INT-REUSE-01](../../checklist.md#int-reuse-01) | Module: `crates/ofdma/src/lib.rs` (planned); registration: [Cargo.toml](../../../../Cargo.toml); acceptance check: Rust-only dependency/build inspection and no-unsafe guards |
| [REQ-DS-PHY-01](../../../requirements.md#req-ds-phy-01) | [INT-REG-01](../../checklist.md#int-reg-01) | Module: [crates/ofdm/src/transmission](../../../../crates/ofdm/src/transmission); Test: [crates/ofdm/tests/waveform_integration.rs](../../../../crates/ofdm/tests/waveform_integration.rs) |

Related delivery gates: [INT-REUSE-01](../../checklist.md#int-reuse-01), [INT-DSP-01](../../checklist.md#int-dsp-01).

## Supporting records and validation evidence

- [unified-backend/reuse-review.md](../unified-backend/reuse-review.md).
- [qam-mapping/architecture.md](../qam-mapping/architecture.md).
- [qam-demapper-performance/design.md](../qam-demapper-performance/design.md).
- [waveform/plan.md](../waveform/plan.md).
- [prbs-source/plan.md](../prbs-source/plan.md).
- [impairment-group/plan.md](../impairment-group/plan.md).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.

## INT-DSP-01 implementation slice

At base `37be29ec2d536a96a504211fd49b20152e5555cb`, the nine existing public PRBS
and waveform-transform cases pass before edits. The upstream source remains clean
at its pinned revision. The local Cargo/rustup discovery symlinks point to an
unavailable checkout; an existing MERLIN-local Rust 1.85.1 compiler is used through
an explicit PATH and ignored `local/backend-integration-cargo` cache. No compiler
installation, cache deletion or source-repository mutation is required.

Add Cargo `merlin-dsp` / library `merlin_dsp` under `crates/dsp`, safe Rust with no
runtime dependencies. The justified consumers are current OFDM transform/source
wrappers and the independently owned upstream transform/source owners selected
for the next migration. Sharing here removes their otherwise duplicated numerical
implementation without depending either PHY on the other. It does not accept the
upstream wrapper or transport before its later tests run inside MERLIN.

| Candidate / inspected source → downstream owner | Numerical contract / consumers | Current decision and acceptance |
| --- | --- | --- |
| IQ: source `types/iq.rs::Iq`; downstream `types::IqPair` | Both Cartesian f64; source struct serializes named re/im, downstream pair serializes an array. | **ADAPT** shared numerical input alias `[f64;2]` in `dsp::complex`; retain directional public types/JSON, no transmute or generic arithmetic replacement. |
| FFT: source `signal_processing/transform.rs::Transform`; downstream `waveform/transform.rs::TransformPlan` | Centered unitary, ascending physical bins, positive inverse roots; source 2K/4K, downstream 4K/8K. | **SHARE** one bounded `dsp::transform::TransformPlan`; preserve downstream wrappers/errors and exact operation order. Independent tones/impulses/direct sums, all three sizes, frozen former-kernel bit equality and benchmarks precede replacing caller internals. |
| QAM distance: source `signal_processing/qam.rs` / `qam_tables.rs`; downstream `modulation/qam.rs` / indexed demapper | Shared large raw resources and label permutations; source adds independent 8/32-QAM and upstream normalization/limits. Downstream uses cached indexed search. | **REUSE** unchanged existing raw resources when upstream migrates; **SEPARATE** directional tables/lookup/search for now. No speculative distance helper or downstream demapper rewrite. Preserve current normative/label tests. |
| LLR: source `signal_processing/demapping.rs`; downstream soft demapper | Positive-for-zero convention agrees; variance/clipping/available-order and search policies differ. | **SEPARATE** current direction owners; no unproved substitution. Their independent finite/erasure/error/decoder cases remain stage gates. |
| FIR: source `channel/stream.rs::response`; downstream `channel/filter.rs::StreamingChannelFilter` | Causal complex math agrees, but source configured taps versus downstream inferred/bounded filter plan, scale/drain/state differ. | **SEPARATE** state/admission/drain; no common stream wrapper. Preserve hand-convolution and downstream block/drain cases. |
| AWGN: source `channel/stream.rs::noise`; downstream `noise/awgn.rs` | Both use pinned pure-Rust rand/rand_chacha/rand_distr; reference power, sigma policy and draw ordering belong to different channel admissions. | **REUSE** dependencies; **SEPARATE** channel state/draw policy. No RNG stream change; future upstream cases must retain original draws. |
| Phase/CFO: source `channel/stream.rs::rotate`; downstream waveform/channel impairment owners | Source absolute ticks, integer delay and bounded phase/CFO versus downstream supported composition/acquisition. | **SEPARATE** phase origin/units/state; no new generic phase implementation. |
| Response: source `measurements/pre_eq.rs` / channel; downstream channel/filter and scenario owners | C, physical H and effective received C×H have distinct authority and association constraints. | **SEPARATE** semantics and measurement state. Generic complex arithmetic alone does not justify merging response models. |
| PRBS: source `source/prbs15.rs::Prbs15`; downstream `simulation/source/prbs.rs::Prbs15` | Same LSB-first x^15+x^14+1 recurrence; metadata, chunk/range bounds and verifier policies differ. | **SHARE** only pure one-step recurrence in `dsp::prbs15`; retain existing downstream seed/position/chunk/error/verifier bodies and private state. Frozen seed vectors, full period, chunk/overflow invariance and benchmark prove equivalence. This is not PHY randomization. |
| MER: source `instruments/metrics.rs::MatchedMer`; downstream `modulation/mer.rs` | Both accumulate signal/error power; nearest-point versus known-reference semantics, unavailable/zero-error status and retention differ. | **SEPARATE** existing accumulators; **DEFER** common reduction until accumulation order, overflow and unknown/zero error representations are shown equivalent. |

All source paths above are relative to pinned `crates/phy/src/`, not runtime
MERLIN dependencies. Normative geometry/reference indexing remains PHY I20
§7.4.9, PDF65–66 and §7.5.7.1, PDF133–134; the skill text was inspected, not a new
rendered-PDF verification. Unitary normalization and operation order are preserved
engineering conventions of the existing kernels; no unread extracted equation
is reconstructed here. Fixed clocks, CP/window/pilots/grants remain PHY-owned.

Added in this slice: `crates/dsp/{Cargo.toml,src/lib.rs,src/complex.rs,src/error.rs,src/transform.rs,src/prbs15.rs,tests/equivalence.rs,tests/benchmark.rs,tests/support/legacy_transform.rs}`.
Edit only the workspace/ofdm dependency declarations, version inventory, three
local locks, downstream private transform wrapper and its one-bit PRBS helper.
Preserve the existing FFT/IFFT public wrappers, every public import, PRBS state/
position/seed/budget/verifier, all historical test bodies and raw QAM/FEC tables.
The frozen numerical fixture is a test-only former implementation, with origin
and adaptations recorded; independent Fourier/recurrence checks remain separate.

Before downstream rewiring, run the shared kernel/fixture and independent tests
in the new crate and changed-hot-path release benchmark. After rewiring, run
relevant existing suites, full Rust/analysis/examples/Clippy and native Python
regression as applicable. Record results in [DSP integration validation](validation.md).
Do not mark INT-DSP-01 complete before the shared kernel and real downstream
consumers pass; upstream PHY acceptance stays with the later unchanged tasks.
