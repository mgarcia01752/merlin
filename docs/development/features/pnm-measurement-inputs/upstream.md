# Upstream PNM admission, association and measured input

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-pnm-and-capture-admission).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Boundary and support states

The existing native `pnm-input` adapter remains the I/O/raw-byte owner. Current
downstream support and the PNM setup/source pin remain unchanged. Upstream UCD,
RxMER and Pre-Eq admission is planned under INT-PNM-01; whole-plan association and
actual measured playback require INT-PNM-02. A source decoder name or historical
source pass does not prove support in MERLIN's exact pinned decoder.

## Three separate operations

1. **Decode/inspect:** bounded bytes → versioned typed observation and retained
   provenance. Validate file type/version, length, byte order, scaling, sentinels,
   ranges and resource limits. Applications retain the exact original bytes/hash.
2. **Associate/project:** decoded observations + explicit CM/channel/grid/profile/
   time/bank context → accepted PHY values with available/missing/excluded coverage.
   Reject incompatible cohorts and guessed defaults; keep raw and accepted values.
3. **Apply/play back:** an admitted complete configuration/schedule → modeled inputs
   at the declared plane. Loading an inventory is idle; it neither starts a job nor
   selects a policy. Analytical screening is distinct from simulated FEC outcomes.

Capture inventories are discovered through Rust, not hardcoded profile catalogs.
Single, Average, Hold and Interpolate retain their explicit selection/order/time/
coverage rules. Duplicate capture selection is not extra statistical weight.
Preserve exact physical bins, null/excluded status, source timestamps/hashes and
unsupported file versions. A complete future schedule is checked before source work.

## Direction-specific observations

| Input | Interpretation boundary |
| --- | --- |
| UCD/IUC and represented grant context | Inspection/domain representation is separate from executable burst support; source format identity is preserved. |
| Upstream RxMER | Measurement at its declared plane/cohort, not injected SNR, a decoded FEC result or a fabricated full grid. |
| Pre-Eq bank/update | Corrective C with scaling, identity and update context; not physical H. |
| Existing downstream channel estimate/RxMER/profile | Retain established decoder/API/grid/provenance behavior and its separate downstream owners. |

The adapter depends on core PHY types/APIs; core PHY never depends back on file I/O.
Proposed local files are `crates/pnm-input/src/upstream/{ucd,rxmer,pre_eq,association}.rs`.
Verify actual exact-version decoder support per format before reuse; missing support
requires its own reviewed bounded Rust owner and independent format evidence.
No vendored PNM Viewer, network capture/control or Python calculations are added.

## Acceptance and normative evidence

Preserve authored independent bytes and expected raw scaling, bad versions/lengths,
unknowns/sentinels, identity/time/grid mismatches, input immutability and no-clobber
behavior. Test decoding separately from association and actual playback, including
rejected future updates and partial coverage. Native Python mirrors admitted Rust
contracts after their own API gate; it is not a second parser/scientific engine.

PHY 3.1 I20 §9.4.8 and §7.4.17 supply upstream measurement/correction context.
MULPI I25 and the retained exact CM-OSSI format references supply their own wire/
update details. PNM V05 RxMER/FEC guidance is informative interpretation, not a
substitute binary-format or normative PHY source. Historical source reports identify
original page/fixture reviews; those reviews were not re-executed by this reorganization.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-PNM-01](../../../requirements.md#req-us-pnm-01) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs` (planned); Test: `crates/pnm-input/tests/upstream_association.rs` (planned) |
| [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs` (planned); Test: `crates/pnm-input/tests/upstream_association.rs` (planned) |
| [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs` (planned); Test: `crates/pnm-input/tests/upstream_association.rs` (planned) |
| [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs` (planned); Test: `crates/pnm-input/tests/upstream_association.rs` (planned) |

Related delivery gates: [INT-PNM-01](../../checklist.md#int-pnm-01), [INT-PNM-02](../../checklist.md#int-pnm-02).

## Supporting records and validation evidence

- [pnm-measurement-inputs/README.md](README.md).
- [unified-backend/references/capture-formats-validation.md](../unified-backend/references/capture-formats-validation.md).
- [unified-backend/references/rxmer-capture-validation.md](../unified-backend/references/rxmer-capture-validation.md).
- [unified-backend/references/remaining-delivery-validation.md](../unified-backend/references/remaining-delivery-validation.md).
- [unified-backend/references/upstream-stage-contracts.md#rxmer-input](../unified-backend/references/upstream-stage-contracts.md#rxmer-input).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.
