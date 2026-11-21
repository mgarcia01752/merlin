# Shared FEC mechanisms and distinct code families

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-shared-dsp-and-fec).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Boundary and current support

The standalone `fec-codec` crate owns coding independently of modulation, channel,
captures and adapters. Shared sparse construction is an implementation mechanism;
it does not select a normative code or add an outer code. Preserve downstream BCH,
data LDPC, shortening, NCP and PLC definitions and all existing public imports.

| Decision/evidence plane | Current state |
| --- | --- |
| Architecture | Shared infrastructure with distinct direction/control-channel definitions is recorded in the canonical owner. |
| Shared implementation | The immutable sparse plan and separate GF2/byte encode operations exist. |
| Numerical validation | INT-FEC-01 records MERLIN source-audit, oracle, shortening/failure and downstream regression passes; not rerun for this documentation change. |
| Upstream PHY integration | Grant selection, waveform transport and public upstream Rust/Python jobs remain planned. |

## Code identity and operation contracts

| Ordinary upstream family | Mother N / K | Kept distinct from |
| --- | --- | --- |
| Long | 16200 / 14400 | Downstream data matrix with matching dimensions |
| Medium | 5940 / 5040 | Other upstream/downstream matrices |
| Short | 1120 / 840 | Ranging-specific coding/puncturing |

`LdpcSettings` admits retained information in 1..=K and a 1..100 iteration work
budget. Encode validates binary input before work, appends shortening zeros in
mother-information coordinates, computes parity and omits only those zero positions.
Decode consumes finite natural-log P(0)/P(1) LLRs, positive for zero; the existing
codec saturates to ±50 and restores omitted zeros once. Retain owned final decisions,
actual iterations, convergence, parity and shortened-zero consistency independently.
A valid different codeword can satisfy parity yet fail comparison with the original.
Ordinary upstream data uses neither downstream BCH nor ranging puncturing.

The sparse plan is immutable and reused. The existing downstream GF2 operation stays
separate from the byte-buffer operation. Decoder scratch belongs to one codec/job;
parallel jobs use independent codecs. Adapters contain no second decoder. Cancellation
belongs around completed codec calls in the future job owner, not inside this leaf.

## Validation and evidence boundaries

Preserve independent original-page transcription, every base-row/expanded-edge
fingerprint and frozen parity response. Do not derive replacement expectations
from production matrices. Dense/sparse comparison, syndrome, noiseless recovery,
controlled errors, erasures, shortening bounds, invalid LLRs and failed decisions
have distinct purposes. The Gaussian BPSK oracle is a codec experiment, not an
OFDMA RxMER measurement or external interoperability record.

Independent upstream source material is historical; INT-FEC-01 has its own MERLIN
acceptance record. Downstream acceptance remains with its original task owners.
No public Python FEC-only facade is introduced before the real upstream Rust loop.

## Pinned normative scope

DOCSIS 3.1 PHY I20 §7.4.3/Table 8 and §7.4.3.2 specify upstream families and
circulant construction. The retained source audit identifies its original rendered
pages and corrections. Existing downstream stage records retain their own PHY/ETSI
norms; the upstream table does not replace them. No PDF or matrix was re-audited here.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01) | [INT-REG-01](../../checklist.md#int-reg-01) | Module: [crates/fec-codec/src/lib.rs](../../../../crates/fec-codec/src/lib.rs); Test: [crates/fec-codec/tests/codec.rs](../../../../crates/fec-codec/tests/codec.rs); Test: [crates/fec-codec/tests/normative.rs](../../../../crates/fec-codec/tests/normative.rs) |
| [REQ-US-TX-02](../../../requirements.md#req-us-tx-02) | [INT-FEC-01](../../checklist.md#int-fec-01) | Module: [crates/fec-codec/src/upstream_ldpc/codec.rs](../../../../crates/fec-codec/src/upstream_ldpc/codec.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests.rs) |
| [REQ-US-RX-08](../../../requirements.md#req-us-rx-08) | [INT-FEC-01](../../checklist.md#int-fec-01) | Module: [crates/fec-codec/src/upstream_ldpc/codec.rs](../../../../crates/fec-codec/src/upstream_ldpc/codec.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests.rs) |
| [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-DSP-01](../../checklist.md#int-dsp-01) | Module: [crates/fec-codec/src/sparse_encoder.rs](../../../../crates/fec-codec/src/sparse_encoder.rs); Module: [crates/ofdm/src/signal_processing](../../../../crates/ofdm/src/signal_processing); Test: [crates/fec-codec/src/sparse_encoder/tests.rs](../../../../crates/fec-codec/src/sparse_encoder/tests.rs) |
| [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05) | [INT-FEC-01](../../checklist.md#int-fec-01) | Module: [crates/fec-codec/src/upstream_ldpc/definitions.rs](../../../../crates/fec-codec/src/upstream_ldpc/definitions.rs); Test: [crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs](../../../../crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs) |

Related delivery gates: [INT-FEC-01](../../checklist.md#int-fec-01), [INT-DSP-01](../../checklist.md#int-dsp-01).

## Supporting records and validation evidence

- [sparse-encoder/design.md](../sparse-encoder/design.md).
- [fec-shortening/design.md](../fec-shortening/design.md).
- [unified-backend/fec-validation.md](../unified-backend/fec-validation.md).
- [unified-backend/references/ldpc-source-audit-validation.md](../unified-backend/references/ldpc-source-audit-validation.md).
- [unified-backend/references/upstream-stage-contracts.md#tx-02-ldpc-encoding-and-shortening](../unified-backend/references/upstream-stage-contracts.md#tx-02-ldpc-encoding-and-shortening).
- [unified-backend/references/upstream-stage-contracts.md#rx-08-shortening-restoration-and-ldpc-decoding](../unified-backend/references/upstream-stage-contracts.md#rx-08-shortening-restoration-and-ldpc-decoding).
- [Actual MERLIN standalone codec acceptance](../unified-backend/fec-validation.md).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.
