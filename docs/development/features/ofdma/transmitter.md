# OFDMA transmitter contracts

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-upstream-processing-order).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Support and composition boundary

The local typed upstream transmitter now composes all eight stages.
[The implementation record](design.md#int-us-tx-01-transmitter-implementation-unit)
records its tests; combined INT-US-TX-01 acceptance now passes complete
configuration admission and the local session cases. Standalone LDPC acceptance remains INT-FEC-01. Consume
only validated source/configuration/resource plans and explicit correction-bank
updates. Emit indexed complex baseband samples with configuration/run identity and
actual retained tap coverage.

## Ordered stage boundaries

| Stage | Representation and local responsibility |
| --- | --- |
| TX-01 | Meaningful source → grant plan; count transmitted coded positions and separate fill, shortening, parity and pad. |
| TX-02 | Plan → systematic retained information/parity with one selected upstream LDPC family; no BCH. |
| TX-03 | Encoded transmitted bits → grant-seeded 23-bit randomization with exact first-bit/clock/reset semantics. |
| TX-04 | Randomized bits → MSB-first labeled cells using supported upstream loading and normalization. |
| TX-05 | Cells/pilots → scaled physical bins through accepted role visits and joint interleaving; exclusions stay physical. |
| TX-06 | Physical bins → applied correction C at the transmitter reference plane; bank identity and timing remain explicit. |
| TX-07 | Centered corrected bins → unitary IFFT useful samples, independent sign/scale/bin conventions. |
| TX-08 | Useful samples → prefix/window/overlap, guards and indexed burst assembly with exact startup/drain. |

Detailed source equations, normative tables, dimensions, state and limitations are
preserved at the linked historical stage anchors below. The canonical architecture
and local requirement table control MERLIN module names and acceptance. No complete
source document is duplicated here.

## State, observability and failure

Grant boundaries, randomizer phase, pilot/resource visits, Pre-Eq updates and overlap
state have independent ownership; PRBS source continuity must not reset accidentally.
Validate future grants and correction/measurement schedules before source consumption.
Keep complete/partial/rejected outcomes and actual identities. A tap is passive and
bounded; enabling it cannot change random draws, packing or scientific results.

Each stage preserves its representation boundary even if an eventual numerical
implementation fuses operations. No generic scheduled-data preamble or ranging-only
puncturing is added. TX payload, applied C and injected channel parameters are test
observer truth, never hidden normal receiver inputs.

## Planned acceptance and norms

First prove independent grant capacity, source/fill/shortening conservation, randomizer
bits, QAM labels/LLRs, role/bin maps, pilot/interleaving coordinates, Pre-Eq gain and
transform/prefix/window samples. Then validate stream/chunk/rejection/drain behavior
and the actual unity loop. Existing source passes do not close those MERLIN gates.

PHY 3.1 I20 §7.4.1, §7.4.3–10 and §7.4.16–17 govern the applicable stage details.
Initial/fine ranging/probing (§7.4.15) retain separate applicability and remain deferred.
Use each retained stage's exact table/page references before implementation.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-TX-01](../../../requirements.md#req-us-tx-01) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/grant/plan.rs`; Test: `crates/ofdma/tests/grant_plan.rs` |
| [REQ-US-TX-02](../../../requirements.md#req-us-tx-02) | [INT-FEC-01](../../checklist.md#int-fec-01) | Module: [crates/fec-codec/src/upstream_ldpc/codec.rs](../../../../crates/fec-codec/src/upstream_ldpc/codec.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests.rs) |
| [REQ-US-TX-03](../../../requirements.md#req-us-tx-03) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/bit_processing/randomizer.rs`; Test: `crates/ofdma/tests/randomization.rs` |
| [REQ-US-TX-04](../../../requirements.md#req-us-tx-04) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/signal_processing/qam.rs`; Test: `crates/ofdma/tests/qam_mapping.rs` |
| [REQ-US-TX-05](../../../requirements.md#req-us-tx-05) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/transmitter/framing.rs`; Test: `crates/ofdma/tests/upstream_interleaving.rs` |
| [REQ-US-TX-06](../../../requirements.md#req-us-tx-06) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/signal_processing/pre_eq.rs`; Test: `crates/ofdma/tests/pre_eq.rs` |
| [REQ-US-TX-07](../../../requirements.md#req-us-tx-07) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/signal_processing/transform.rs`; Test: `crates/ofdma/tests/transforms.rs` |
| [REQ-US-TX-08](../../../requirements.md#req-us-tx-08) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/transmitter/assembly.rs`; Test: `crates/ofdma/tests/tx_framing.rs` |

Related delivery gates: [INT-US-TX-01](../../checklist.md#int-us-tx-01), [INT-FEC-01](../../checklist.md#int-fec-01).

## Supporting records and validation evidence

- [unified-backend/references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection](../unified-backend/references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection).
- [unified-backend/references/upstream-stage-contracts.md#tx-08-sample-assembly](../unified-backend/references/upstream-stage-contracts.md#tx-08-sample-assembly).
- [unified-backend/references/backend-stages-validation.md](../unified-backend/references/backend-stages-validation.md).
- [unified-backend/references/upstream-stage-contracts.md#tx-02-ldpc-encoding-and-shortening](../unified-backend/references/upstream-stage-contracts.md#tx-02-ldpc-encoding-and-shortening).
- [unified-backend/references/upstream-stage-contracts.md#tx-03-grant-seeded-phy-randomization](../unified-backend/references/upstream-stage-contracts.md#tx-03-grant-seeded-phy-randomization).
- [unified-backend/references/upstream-stage-contracts.md#tx-04-cell-word-construction-and-qam-mapping](../unified-backend/references/upstream-stage-contracts.md#tx-04-cell-word-construction-and-qam-mapping).
- [unified-backend/references/upstream-stage-contracts.md#tx-05-pilots-ofdma-framing-and-interleaving](../unified-backend/references/upstream-stage-contracts.md#tx-05-pilots-ofdma-framing-and-interleaving).
- [unified-backend/references/upstream-stage-contracts.md#tx-06-cm-pre-eq](../unified-backend/references/upstream-stage-contracts.md#tx-06-cm-pre-eq).
- [unified-backend/references/upstream-stage-contracts.md#tx-07-ifft](../unified-backend/references/upstream-stage-contracts.md#tx-07-ifft).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.
