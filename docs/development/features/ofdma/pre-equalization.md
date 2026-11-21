# Upstream Pre-Eq banks and transmitter correction

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-channel-and-measurement-reference-planes), [canonical architecture](../../../architecture.md#unified-pnm-and-capture-admission).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Ownership and support

This feature owns the transmitter correction-bank/application contract and its
reference planes. Raw-file decoding belongs to the PNM adapter, accepted projections
to the upstream PHY. TX and native measured application now pass local
INT-US-TX-01 and INT-PNM-01/02 acceptance; native Python/CLI exposure remains
separate. See [measured acceptance](../pnm-measurement-inputs/upstream.md#measured-association-acceptance).

## Input and output contracts

Inputs identify original capture bytes/hash, CM/channel/RF grid, capture time,
coefficient scaling/unknowns, bank identity and the explicit operation/update context.
An input bank is C, not physical plant H. Preserve set/adjust or other represented
source semantics without inferring how or when coefficients were applied merely
because their values exist. Unknown/excluded coefficients retain status and cannot
silently become unity or zero.

TX-06 consumes accepted physical-bin cells and the correction bank scheduled for
that exact interval. It emits corrected bins before the unitary IFFT. Preserve
normalization, phase/delay convention, complete-grid admission and bank identity.
Normal receiver pilot estimation observes effective C×H; only independent observers
may compare against injected correction/response truth.

## Bank state and preflight

Parsing/inspection, association and application are distinct. Loading a bank does not
start a run or change channel mode. Whole-plan admission checks every future bank
and update against the selected waveform/measurement policy before source/RNG work.
Invalid grids, incompatible identity, unexplained missing coefficients or unsupported
updates leave the prior accepted generation unchanged.

Initially restricted measured-RxMER synthesis requires unity H and every scheduled
C bank with timing disabled. Do not admit a later nonunity update by validating only
the first bank. Wider inverse-C response inference is a deferred engineering model,
not plant measurement or an automatic decoder default.

## Acceptance and pinned evidence

Use authored format fixtures and independent raw scaling/byte-order/sentinel checks
at the adapter, then bank operation/grid/time tests in core, and independent TX-06
application/reference-plane/chunk tests. Retain rejected updates and partial coverage
in diagnostics; do not manufacture complete corrective coverage.

PHY 3.1 I20 §7.4.17 and its represented MULPI/OSSI update context govern applicability.
The retained source records identify the exact editions and source-field decisions.
The coefficient array alone is insufficient authority for update semantics. No
new private-PDF or modem capture audit was executed for this document.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-TX-06](../../../requirements.md#req-us-tx-06) | [INT-US-TX-01](../../checklist.md#int-us-tx-01) | Module: `crates/ofdma/src/signal_processing/pre_eq.rs`; Test: `crates/ofdma/tests/pre_eq.rs` |
| [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |
| [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |
| [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |

Related delivery gates: [INT-US-TX-01](../../checklist.md#int-us-tx-01), [INT-PNM-01](../../checklist.md#int-pnm-01), [INT-PNM-02](../../checklist.md#int-pnm-02).

## Supporting records and validation evidence

- [unified-backend/references/upstream-stage-contracts.md#tx-06-cm-pre-eq](../unified-backend/references/upstream-stage-contracts.md#tx-06-cm-pre-eq).
- [unified-backend/references/upstream-stage-contracts.md#us-pre-eq-input](../unified-backend/references/upstream-stage-contracts.md#us-pre-eq-input).
- [unified-backend/references/remaining-delivery-validation.md#preeq-format-01](../unified-backend/references/remaining-delivery-validation.md#preeq-format-01).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.
