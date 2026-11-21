# OFDMA received-sample receiver contracts

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-upstream-processing-order).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Support, inputs and outputs

The local upstream receiver remains planned under INT-US-RX-01. Its standalone LDPC
leaf has INT-FEC-01 evidence. Normal RX gets received indexed samples and legitimate
accepted receiver configuration; it receives no original bits/PRBS seed, injected
H/C, noise/RNG parameters, expected transmitted cells or hidden scheduler truth.
The independent verifier/observer owns those comparisons outside RX.

## Ordered stage boundaries

| Stage | Representation and local responsibility |
| --- | --- |
| RX-01 | Received blocks → configured burst intervals; retain exact coordinates, continuity and rejected coverage. |
| RX-02 | Framed intervals → complete useful-symbol windows using accepted prefix/window geometry. |
| RX-03 | Useful samples → raw centered physical FFT bins with independent unitary sign/scale checks. |
| RX-04 | Received pilots/bins → effective C×H estimate and equalized cells; supplied-truth reference mode stays labeled separately. |
| RX-05 | Physical cells → grant order by exact inverse roles/interleaving without compacting exclusions. |
| RX-06 | Equalized cells → finite positive-for-zero MSB-first LLRs with an explicit supported variance policy. |
| RX-07 | Soft bits → inverse PHY randomization by sign changes; erasures remain zero and transmitted pad clocks correctly. |
| RX-08 | Retained LLRs → once-restored shortening and actual LDPC decisions/iterations/flags. |
| RX-09 | Information decisions → meaningful recovered extents after receiver-known fill removal; retain partial/failed coverage. |

Configured timing is not blind acquisition. Wider acquisition/tracking/confidence
policies in the retained receiver extension design are deferred; they require their
own independent vectors, ambiguity/rejection, bounds and calibration evidence.

## State and honesty of results

Preserve absolute sample/bit coordinates across arbitrary input blocks. Missing,
duplicate or reordered input cannot become invented zeros or a silently repaired
PRBS phase. Invalid admission changes no accepted state. Reports distinguish no-lock,
unsupported, missing, partial and failed decoding from a complete successful result.
Convergence/parity/known-zero consistency do not prove original-source equality.

Pilot estimation observes the received effective channel, including applied C.
No injected noise variance or exact H enters an estimator disguised as measured
confidence. Keep raw FFT, equalized cells and receiver decisions distinct so taps
can identify their reference planes and actual availability without rerunning science.

## Planned acceptance and norms

Use independently calculated impulse/tone/pilot/resource/soft-bit vectors, noiseless
source recovery, controlled errors, invalid/ambiguous inputs, interrupted coverage,
failed decisions and bounded cancellation between completed calls. The minimum
integrated acceptance is MERLIN source → TX → channel → RX → original comparison.
External interoperability remains separate optional evidence, not a prerequisite.

PHY 3.1 I20 §7.4.3–10 and §7.4.16 provide code/transmitted geometry/pilot evidence;
receiver estimator algorithms are explicitly documented engineering choices, not
claimed mandated implementations. Ranging acquisition has its own §7.4.15 design.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-RX-01](../../../requirements.md#req-us-rx-01) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/receiver/framing.rs` (planned); Test: `crates/ofdma/tests/rx_framing.rs` (planned) |
| [REQ-US-RX-02](../../../requirements.md#req-us-rx-02) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/receiver/framing.rs` (planned); Test: `crates/ofdma/tests/rx_framing.rs` (planned) |
| [REQ-US-RX-03](../../../requirements.md#req-us-rx-03) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/signal_processing/transform.rs`; Test: `crates/ofdma/tests/waveform_transform.rs` (planned) |
| [REQ-US-RX-04](../../../requirements.md#req-us-rx-04) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/signal_processing/estimation.rs` (planned); Test: `crates/ofdma/tests/rx_estimation.rs` (planned) |
| [REQ-US-RX-05](../../../requirements.md#req-us-rx-05) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/receiver/mod.rs` (planned); Test: `crates/ofdma/tests/rx_resources.rs` (planned) |
| [REQ-US-RX-06](../../../requirements.md#req-us-rx-06) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/signal_processing/demapping.rs`; Test: `crates/ofdma/tests/soft_metrics.rs` (planned) |
| [REQ-US-RX-07](../../../requirements.md#req-us-rx-07) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/bit_processing/randomizer.rs`; Test: `crates/ofdma/tests/randomization.rs` |
| [REQ-US-RX-08](../../../requirements.md#req-us-rx-08) | [INT-FEC-01](../../checklist.md#int-fec-01) | Module: [crates/fec-codec/src/upstream_ldpc/codec.rs](../../../../crates/fec-codec/src/upstream_ldpc/codec.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests.rs) |
| [REQ-US-RX-09](../../../requirements.md#req-us-rx-09) | [INT-US-RX-01](../../checklist.md#int-us-rx-01) | Module: `crates/ofdma/src/receiver/framing.rs` (planned); Test: `crates/ofdma/tests/rx_framing.rs` (planned) |

Related delivery gates: [INT-US-RX-01](../../checklist.md#int-us-rx-01), [INT-FEC-01](../../checklist.md#int-fec-01), [INT-DEFER-01](../../checklist.md#int-defer-01).

## Supporting records and validation evidence

- [unified-backend/references/upstream-stage-contracts.md#rx-01-burst-framing-and-timing](../unified-backend/references/upstream-stage-contracts.md#rx-01-burst-framing-and-timing).
- [unified-backend/references/upstream-stage-contracts.md#rx-09-information-deframing](../unified-backend/references/upstream-stage-contracts.md#rx-09-information-deframing).
- [unified-backend/references/extension-designs-receiver.md](../unified-backend/references/extension-designs-receiver.md).
- [unified-backend/references/upstream-stage-contracts.md#rx-02-useful-symbol-extraction](../unified-backend/references/upstream-stage-contracts.md#rx-02-useful-symbol-extraction).
- [unified-backend/references/upstream-stage-contracts.md#rx-03-fft](../unified-backend/references/upstream-stage-contracts.md#rx-03-fft).
- [unified-backend/references/upstream-stage-contracts.md#rx-04-channel-estimation-and-equalization](../unified-backend/references/upstream-stage-contracts.md#rx-04-channel-estimation-and-equalization).
- [unified-backend/references/upstream-stage-contracts.md#rx-05-resource-extraction-and-inverse-interleaving](../unified-backend/references/upstream-stage-contracts.md#rx-05-resource-extraction-and-inverse-interleaving).
- [unified-backend/references/upstream-stage-contracts.md#rx-06-soft-qam-demapping](../unified-backend/references/upstream-stage-contracts.md#rx-06-soft-qam-demapping).
- [unified-backend/references/upstream-stage-contracts.md#rx-07-phy-soft-derandomization](../unified-backend/references/upstream-stage-contracts.md#rx-07-phy-soft-derandomization).
- [unified-backend/references/upstream-stage-contracts.md#rx-08-shortening-restoration-and-ldpc-decoding](../unified-backend/references/upstream-stage-contracts.md#rx-08-shortening-restoration-and-ldpc-decoding).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.
