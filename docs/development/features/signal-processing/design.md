# Shared DSP conventions and reuse boundaries

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-shared-dsp-and-fec).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Architecture and support states

This feature owns numerical conventions and reuse decisions shared by the PHYs.
Existing downstream modules retain their implementations. INT-REUSE-01 accepted an
inspected placement/reuse plan; INT-DSP-01 still gates extraction and consumer
numerical/performance evidence. There is no `crates/dsp` implementation yet.
Documentation ownership does not establish that a candidate kernel is equivalent.

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

Conditional targets are `crates/dsp/src/{complex,transform,modulation,channel,noise,measurements}.rs`
and `crates/dsp/tests/equivalence.rs` (all planned). A documented direct-reuse/no-new-crate
choice is allowed by INT-DSP-01. Preserve downstream public re-exports and independent
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
| [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-DSP-01](../../checklist.md#int-dsp-01) | Module: [crates/fec-codec/src/sparse_encoder.rs](../../../../crates/fec-codec/src/sparse_encoder.rs); Module: [crates/ofdm/src/signal_processing](../../../../crates/ofdm/src/signal_processing); Test: [crates/fec-codec/src/sparse_encoder/tests.rs](../../../../crates/fec-codec/src/sparse_encoder/tests.rs) |
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
