# OFDMA configuration, grants and continuous source

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-upstream-configuration-and-source).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Scope and support

This is the active local design for UCD/IUC, represented MAP/grant/minislot state
and continuous source contracts. Typed OFDMA configuration, grants/resources and source modules now exist in
`crates/ofdma`; [implementation evidence](design.md) records the tested subset.
Document admission and full typed session coverage now pass under INT-US-CONFIG-01. The retained 108-row source coverage matrix is
historical format/domain evidence, not current MERLIN execution or a raw MAC parser.

## Inputs, outputs and ownership

Inputs carry explicit CM/channel/grid identity, accepted numerology/CP/window,
UCD/IUC/profile versions, exclusions, per-cell loading/roles, ordered grant epoch/
minislot allocation, source configuration and finite work limits. Output is an
immutable accepted configuration with separate legitimate TX and RX projections.
A profile identifier alone selects neither one QAM order nor a codeword family.

Representation/domain validation, capture inspection and executable support are
separate results. Preserve inspectable optional controls and original schema/source
identity without claiming their execution. Ranging, probes and profile-testing
modes stay explicitly unsupported until their own independent implementation gates.
No real scheduler, modem control, SNMP/TFTP or device capture is introduced.

Implemented typed owners include `crates/ofdma/src/configuration/ucd/{mod,channel,controls,profiles}.rs`,
`configuration/{grant,profile,waveform}.rs`, `resources/{grid,permutation,pilot_patterns}.rs`
and `grant/{mod,plan,fec_selection}.rs`. These paths are relative to `crates/ofdma/src/`
where abbreviated. Resource planning supplies one consistent physical
cell visit order/capacity to grant framing, TX and RX; do not duplicate pilot counters.

## State, source truth and atomic admission

Load/preview remains idle. Validate the whole request and all scheduled configuration/
bank/measurement transitions before publishing a new generation or consuming source.
Failure leaves the prior configuration and source position intact. Preserve temporal
coverage and version tags rather than guessing defaults from a later capture.

The continuous PRBS15 source and its independent verifier are separate implemented owners
under `crates/ofdma/src/source/{prbs15,verifier}.rs`. Preserve recurrence, seed/bit
order, absolute position and chunk continuity from the source contract. Grant resets
apply to the 23-bit PHY randomizer, not this source. Account separately for meaningful
bits, information-fill ones, shortening zeros, parity and zero grant pad. Rejection
or backpressure consumes no extra meaningful bits. Only the external verifier gets
original-source truth; it reports missing/reordered coverage without phase repair.

## Acceptance before consumers

The configuration gate requires every represented source field to have an explicit
local meaning or rejection, atomic invalid admission, independent grant/resource
capacity, all three FEC selections and PRBS phase/period/range/overflow cases.
Native Rust foundation and legitimate RX settings precede waveform use. Source
coverage does not satisfy these planned MERLIN test owners.

## Pinned normative references

Configuration representation uses MULPI 3.1 I25 §6.4.3 Tables 28/29, §6.4.4–5 and
its explicitly retained dependent clauses; the coverage record identifies original
pages. PHY 3.1 I20 §7.4 owns waveform applicability and §7.4.15 owns distinct ranging/
probe behavior. PHY guidance alone is not authority for MULPI wire fields. No new
private-specification review or raw-wire encoder is claimed by this adaptation.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-CFG-01](../../../requirements.md#req-us-cfg-01) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/configuration/ucd/mod.rs`; Test: `crates/ofdma/tests/configuration_json.rs` |
| [REQ-US-CFG-02](../../../requirements.md#req-us-cfg-02) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/configuration/ucd/mod.rs`; Test: `crates/ofdma/tests/configuration_json.rs` |
| [REQ-US-CFG-03](../../../requirements.md#req-us-cfg-03) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/configuration/ucd/mod.rs`; Test: `crates/ofdma/tests/configuration_json.rs` |
| [REQ-US-CFG-04](../../../requirements.md#req-us-cfg-04) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/configuration/ucd/mod.rs`; Test: `crates/ofdma/tests/configuration_json.rs` |
| [REQ-US-CFG-05](../../../requirements.md#req-us-cfg-05) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/configuration/ucd/mod.rs`; Test: `crates/ofdma/tests/configuration_json.rs` |
| [REQ-US-CFG-06](../../../requirements.md#req-us-cfg-06) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/configuration/ucd/mod.rs`; Test: `crates/ofdma/tests/configuration_json.rs` |
| [REQ-US-PRBS-01](../../../requirements.md#req-us-prbs-01) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/source/prbs15.rs`; Test: `crates/ofdma/tests/prbs.rs` |
| [REQ-US-PRBS-02](../../../requirements.md#req-us-prbs-02) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/source/prbs15.rs`; Test: `crates/ofdma/tests/prbs.rs` |
| [REQ-US-PRBS-03](../../../requirements.md#req-us-prbs-03) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/source/prbs15.rs`; Test: `crates/ofdma/tests/prbs.rs` |
| [REQ-US-PRBS-04](../../../requirements.md#req-us-prbs-04) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01) | Module: `crates/ofdma/src/source/prbs15.rs`; Test: `crates/ofdma/tests/prbs.rs` |

Related delivery gates: [INT-US-CONFIG-01](../../checklist.md#int-us-config-01).

## Supporting records and validation evidence

- [unified-backend/references/us-configuration-json-coverage.md](../unified-backend/references/us-configuration-json-coverage.md).
- [unified-backend/references/foundation-validation.md](../unified-backend/references/foundation-validation.md).
- [unified-backend/references/extension-designs-ranging.md](../unified-backend/references/extension-designs-ranging.md).
- [unified-backend/references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection](../unified-backend/references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection).
- [unified-backend/references/upstream-stage-contracts.md#prbs-block-architecture](../unified-backend/references/upstream-stage-contracts.md#prbs-block-architecture).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.

## Current document and field coverage

The strict [MERLIN schema](../../../../schemas/upstream-configuration-v1.schema.json)
and [synthetic inputs](../../../../examples/cli/upstream/README.md) use explicit
MERLIN identities. Document admission, typed preparation and actual session runs
share core Rust validation. The [108-row current owner mapping](configuration-field-mapping.json)
links the retained source matrix to local owners and executed case groups, with
explicit limits on per-value boundary coverage. This does not claim a new PDF review.
