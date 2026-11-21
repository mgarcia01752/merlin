# Upstream PNM admission, association and measured input

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-pnm-and-capture-admission).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Boundary and support states

The existing native `pnm-input` adapter remains the I/O/raw-byte owner. Current
downstream support and the PNM setup/source pin remain unchanged. Upstream UCD,
RxMER and Pre-Eq admission is implemented under INT-PNM-01; whole-plan association and
actual measured playback are now accepted locally under INT-PNM-02. A source decoder name or historical
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
Raw owners are `crates/pnm-input/src/upstream/{ucd,ucd_projection,rxmer,pre_eq}.rs`;
`association.rs` owns complete-plan association and the typed core handoff.
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
| [REQ-US-PNM-01](../../../requirements.md#req-us-pnm-01) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |
| [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |
| [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |
| [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04) | [INT-PNM-02](../../checklist.md#int-pnm-02) | Module: `crates/pnm-input/src/upstream/association.rs`; Test: `crates/pnm-input/tests/upstream_association.rs` |

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

## INT-PNM-01 implementation unit

Synthetic core checkpoint: `a3533ae`. Add the planned native upstream module and
three bounded source-derived format owners/tests. Reuse existing pnm-input I/O,
serde/SHA dependencies and exact `pnm` 0.3.6 revision
`5fbd5561f3e06b73486546b5ffa520279080a5eb`; add an independent docsis-ofdma edge.
Neither PHY depends on this adapter. Source fixture bytes stay identical under
`examples/measurements/upstream/`, with source hashes and legal origin recorded.

| Inspected path / symbol | Contract / evidence | Gap | Decision / destination |
| --- | --- | --- | --- |
| Pinned `pnm/parser/cm_us_ofdma_pre_eq.rs::CmUsOfdmaPreEq::parse` | Typed PNN6/7, BE signed pairs, applied s2.13 / update s1.14; literal tests | Pin only accepts exact version 1.0; approved source contract retains opaque minor 0..255 | ADAPT in `crates/pnm-input/src/upstream/pre_eq.rs`: delegate minor0 to full decoder; retain stricter envelope/grid/profile checks and original bytes |
| Pinned `pnm/lib/fixed_point_decoder.rs::decode_iq` | Same BE decoded i16 pair and power-of-two scaling, no excluded-pair marker | Nonzero opaque minor cannot use full pinned header parser | REUSE this kernel after bounded source-derived envelope admission, without changing originals or passing a falsified header |
| Source `crates/morgana/src/inputs/{ucd,rxmer}.rs` | UCD management envelope/TLVs and CCAP I18 Table410 major2 RxMER; independent authored hex fixtures | No typed counterpart in exact pinned decoder | ADAPT native owners; structural inspection is separate from projection/association |
| Pinned `common/json.rs::to_json` | Bounded serialization, exact errors, owned output | Native inspection export | REUSE; pretty formatting is an adapter representation change, separate from retained originals |

Input byte/array bounds precede allocation. Reject unsupported identity/major/spacing,
truncation, bad length/grid and flag/overflow combinations with typed category,
field and offset. Opaque metadata/unknown TLVs remain inspectable. Normalization,
association and simulated noise are not decoder responsibilities. Capture inspection
exports use explicit `merlin.upstream.*` identities; raw inputs are never relabeled.
No new authoritative private-PDF review is claimed; pinned source evidence and
independent authored fixtures retain their exact origins. INT-PNM-02 remains the
measured-association/playback gate.

### Decoder acceptance

All 15 upstream decoder/projection cases pass inside MERLIN, including literal
fixed-point values for every opaque minor byte, all raw RxMER octets, nulls,
original hashes, malformed extents/resource limits and multipart identity/order.
The complete 59-case pnm-input regression suite also passes, including downstream binary,
API, Designer and saved-workbench cases. Clippy denies warnings; formatting passes.
The final upstream-only run pins the exact source candidate independently of the
broader component run. No original field input or normative PDF was newly acquired.

- [Tested decoder candidate](decoder-candidate.json).
- [Executed decoder cases](decoder-test-results.json).
- [Exact requirement slices](implementation-evidence.json).
- [User guide](../../../user/upstream/measurements.md).

INT-PNM-01 accepts bounded inspection/projection only. Whole REQ-US-PNM-01..03
acceptance is recorded below under INT-PNM-02; decoder tests alone cannot accept
policy selection, update lineage or actual capture-driven runs.

## INT-PNM-02 measured association implementation unit

Build on decoder checkpoint `41d86d7`. Adapt the source measured association owner
into the already planned `crates/pnm-input/src/upstream/association.rs`, with its
full source test matrix in `tests/upstream_association.rs`. Reuse the accepted
UCD projection, capture observations, core PreEqPlayback/RxmerPlayback and preview;
do not repeat numerical models or add a PHY-to-adapter dependency.

| Inspected source path / symbol | Contract and gap | Decision / destination |
| --- | --- | --- |
| `morgana/crates/morgana/src/inputs/measured.rs::MeasuredExperiment::prepare` | Explicit identity/grid/source-time/probe lineage, whole-plan preflight; decoder APIs now exist locally. | ADAPT into `crates/pnm-input/src/upstream/association.rs`; retain every bound and association check. |
| `morgana/crates/morgana/src/api/session.rs::configure_measured` | Source facade directly owns parsers; copying it into PHY would invert the approved dependency. | ADAPT handoff: core-owned `MeasuredConfiguration` in `crates/ofdma/src/api/measured.rs`, constructed by adapter admission and consumed atomically by the real session. |
| `crates/ofdma/src/api/session.rs::configure_document` | Idle atomic replacement already validates before mutation. | REUSE ordering for `configure_measured`; preserve original generic document separately from projected execution input. |
| `morgana/crates/morgana/tests/measured_experiment.rs` | Independent authored capture cohorts, literal power/timestamp expectations and actual both-geometry loops. | ADAPT imports and prepared handoff; preserve all seven association cases and assertions. The eighth source multipart case remains in decoder tests. |

`MeasuredExperiment::admit` returns an owned `MeasuredConfiguration` containing
resolved experiment/options, the original generic document and bounded versioned
provenance. Its core constructor rechecks domain readiness and provenance format
before session mutation; it does not decode captures or certify caller-supplied
provenance claims. `Simulation::configure_measured` replaces all accepted settings
and generation only after validation succeeds. Native adapters parse measured JSON
through pnm-input, then call this same typed core method. No core JSON capture
loader or extension trait is needed.

Preserve original plan/file bytes, hashes and epochs in completed and partial
reports; source epochs never become synthetic activation offsets. Entire future
schedules are checked before source/RNG consumption. Accept power-domain selection,
average/hold/interpolation and applied/snapshot/initialize/adjust context only in
the supported unity-residual composition, with no measured-C-as-H inference.

### Measured association acceptance

All 23 upstream input cases pass: the 15 decoder cases plus seven preserved source
association cases and one new typed-handoff case. Both captured numerologies run
the actual source→TX→channel→received-only RX loop and recover source bits.
Literal power-domain average/interpolation, original plan/file hashes, older
corresponding bases, excluded zero records, future incompatible updates and
failed-admission state are checked. Pre-cancelled reports keep measured provenance;
later generic input clears future provenance without changing old owned reports.
The 83 upstream engine regressions, Clippy with warnings denied and formatting pass.

- [Tested association candidate](association-candidate.json).
- [Executed input cases](association-test-results.json).
- [Seven requirement slices and regression records](implementation-evidence.json).

This accepts Rust capture admission and playback, not native Python/CLI exposure,
field interoperability, arbitrary Pre-Eq inverse-H inference or Qt development.
