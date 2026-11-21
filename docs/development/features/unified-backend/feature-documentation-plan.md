# Proposed functional documentation organization

**Phase 1 proposal; awaiting review. No feature files have moved, and no functional
reorganization or backend/UI implementation is authorized by this record.**

The attached request, §§16 and 18, explicitly requires review of the complete mapping
before reorganization. This proposal records that reviewable mapping; it does not
create a second architecture, requirements baseline or completion checklist.

## Baseline and scope

Reviewed branch `merge-merlin-morgana`, clean commit `343b07d9209dc10f3c21d80ae2222a60432f684a`.
The baseline has **170 feature files: 164 Markdown and six JSON**, including the
feature index. The full [machine-readable inventory](feature-documentation-inventory.json) records each exact existing/proposed path,
disposition, SHA-256, owner, requirement/task/architecture references, evidence,
inbound occurrences with source/line/column, adaptation outputs and migration risk.

The inventory captures the clean commit before these two planning records and the
small proposal link in the existing feature index. It deliberately separates
observed references, existing backfill routes, source aliases and proposed navigation
owners. Metadata stewardship under DOC-R3 / REQ-COM-DOC-01 is not scientific acceptance.

Canonical owners remain [architecture](../../../architecture.md), [requirements](../../../requirements.md) and
[the official checklist](../../checklist.md). The UI checklist remains subordinate.
No Rust, manifests, dependencies, bindings, Qt/QML, vectors, captures or API symbols change.

## Recommended organization

Keep established downstream directories where they already own the subject. Group
existing and new records by function in the current feature index; do not create an
`ofdm/` mirror merely for visual symmetry. Keep `unified-backend/` for migration
decisions, source mappings and acceptance history. Preserve its 12 historical
references byte-for-byte. Active MERLIN adaptations get functional owners below.

```text
features/
  README.md                         existing, grouped functional navigation
  fec/design.md                     new common infrastructure / distinct codes
  signal-processing/design.md       new conventions and reuse boundaries
  ofdma/configuration.md            new UCD/IUC, grants, resources and source
  ofdma/transmitter.md              new TX-01..08 contracts
  ofdma/receiver.md                 new RX-01..09 contracts
  ofdma/channel.md                  new CH-01..06 contracts
  ofdma/pre-equalization.md          new correction-bank/reference-plane contract
  pnm-measurement-inputs/upstream.md new bounded native-adapter contract
  high-level-api/upstream.md         new paired Rust/Python convergence design
  <all established feature folders> retained, indexed by function
  unified-backend/                   integration records and historical archive
```

Existing `high-level-api/` owns both language contracts; a second `public-api/` tree
would add duplication. Existing `visual-workspace/`, workbench/receiver/transmitter
folders own Qt design. Future upstream presentation is linked there as deferred
source guidance; this task begins no Qt development. No empty directories are added.

## Exact dispositions and content mapping

| Action on original files | Count | Proposed treatment |
| --- | --- | --- |
| Keep | 151 | Same path and bytes; functional index supplies navigation. |
| Adapt | 7 | Same path; focused links/current interpretation or named stale locator fix. |
| Archive in place | 12 | Existing historical references remain at the same paths, byte-identical. |
| Move / Merge / Delete | 0 / 0 / 0 | None proposed. |

Add nine functional records after approval. Together with the two planning records,
the projected feature-file total is **181**. All original 170 paths survive.

### Existing records to adapt

- [README.md](../README.md).
- [high-level-api/design.md](../high-level-api/design.md).
- [pnm-measurement-inputs/README.md](../pnm-measurement-inputs/README.md).
- [unified-backend/design.md](design.md).
- [unified-backend/source-inventory.json](source-inventory.json).
- [unified-backend/source-review.md](source-review.md).
- [unified-backend/validation.md](validation.md).

The feature index gains functional groups and complete navigation. The integration
design/source review gain links to functional owners. The validation record gains
an appended documentation-only result; original dated findings remain intact.
The API and PNM indexes gain clearly planned upstream links. The source inventory
changes only current routing/metadata needed for DOC-ORG-01 and the named FEC locator;
original source paths, clauses, hashes, aliases and historical snapshot fields remain frozen.

### New functional records and exact source sections

Each new record identifies typed inputs/outputs, units/reference planes, stage order,
state/reset/continuity, bounds, failure behavior, local module/test ownership and its
existing delivery gates. It adapts active contracts and links preserved equations,
full normative detail and independent evidence. It does not copy whole validation
reports, repeat the mandatory requirements baseline or make source passes current.

#### `fec/design.md`

Shared sparse infrastructure; distinct downstream BCH/data/NCP/PLC and upstream families; actual INT-FEC-01 evidence, separate source audits.

Canonical requirements: [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01), [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05).
Existing tasks: `INT-FEC-01`, `INT-DSP-01`.
Architecture: [unified-shared-dsp-and-fec](../../../architecture.md#unified-shared-dsp-and-fec).

Source material: [sparse-encoder/design.md](../sparse-encoder/design.md); [fec-shortening/design.md](../fec-shortening/design.md); [unified-backend/fec-validation.md](fec-validation.md); [unified-backend/references/ldpc-source-audit-validation.md](references/ldpc-source-audit-validation.md); [unified-backend/references/upstream-stage-contracts.md#tx-02-ldpc-encoding-and-shortening](references/upstream-stage-contracts.md#tx-02-ldpc-encoding-and-shortening); [unified-backend/references/upstream-stage-contracts.md#rx-08-shortening-restoration-and-ldpc-decoding](references/upstream-stage-contracts.md#rx-08-shortening-restoration-and-ldpc-decoding).

#### `signal-processing/design.md`

Conventions and sharing decisions for complex IQ, FFT, QAM/LLR, FIR/noise, power and PRBS; current owners stay intact; extraction remains INT-DSP-01.

Canonical requirements: [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01), [REQ-COM-SYS-03](../../../requirements.md#req-com-sys-03), [REQ-DS-PHY-01](../../../requirements.md#req-ds-phy-01).
Existing tasks: `INT-REUSE-01`, `INT-DSP-01`.
Architecture: [unified-shared-dsp-and-fec](../../../architecture.md#unified-shared-dsp-and-fec).

Source material: [unified-backend/reuse-review.md](reuse-review.md); [qam-mapping/architecture.md](../qam-mapping/architecture.md); [qam-demapper-performance/design.md](../qam-demapper-performance/design.md); [waveform/plan.md](../waveform/plan.md); [prbs-source/plan.md](../prbs-source/plan.md); [impairment-group/plan.md](../impairment-group/plan.md).

#### `ofdma/configuration.md`

Typed UCD/IUC and represented MAP/grant/minislot contracts, source framing and PRBS; source field coverage and unsupported modes remain explicit.

Canonical requirements: [REQ-US-CFG-01](../../../requirements.md#req-us-cfg-01), [REQ-US-CFG-02](../../../requirements.md#req-us-cfg-02), [REQ-US-CFG-03](../../../requirements.md#req-us-cfg-03), [REQ-US-CFG-04](../../../requirements.md#req-us-cfg-04), [REQ-US-CFG-05](../../../requirements.md#req-us-cfg-05), [REQ-US-CFG-06](../../../requirements.md#req-us-cfg-06), [REQ-US-PRBS-01](../../../requirements.md#req-us-prbs-01), [REQ-US-PRBS-02](../../../requirements.md#req-us-prbs-02), [REQ-US-PRBS-03](../../../requirements.md#req-us-prbs-03), [REQ-US-PRBS-04](../../../requirements.md#req-us-prbs-04).
Existing tasks: `INT-US-CONFIG-01`.
Architecture: [unified-upstream-configuration-and-source](../../../architecture.md#unified-upstream-configuration-and-source).

Source material: [unified-backend/references/us-configuration-json-coverage.md](references/us-configuration-json-coverage.md); [unified-backend/references/foundation-validation.md](references/foundation-validation.md); [unified-backend/references/extension-designs-ranging.md](references/extension-designs-ranging.md); [unified-backend/references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection](references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection); [unified-backend/references/upstream-stage-contracts.md#prbs-block-architecture](references/upstream-stage-contracts.md#prbs-block-architecture).

#### `ofdma/transmitter.md`

TX-01..08 in normative order; grant layout, randomizer, QAM/resource visits, pilots/interleaving, Pre-Eq handoff, transform and sample assembly; no upstream BCH.

Canonical requirements: [REQ-US-TX-01](../../../requirements.md#req-us-tx-01), [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-TX-03](../../../requirements.md#req-us-tx-03), [REQ-US-TX-04](../../../requirements.md#req-us-tx-04), [REQ-US-TX-05](../../../requirements.md#req-us-tx-05), [REQ-US-TX-06](../../../requirements.md#req-us-tx-06), [REQ-US-TX-07](../../../requirements.md#req-us-tx-07), [REQ-US-TX-08](../../../requirements.md#req-us-tx-08).
Existing tasks: `INT-US-TX-01`, `INT-FEC-01`.
Architecture: [unified-upstream-processing-order](../../../architecture.md#unified-upstream-processing-order).

Source material: [unified-backend/references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection](references/upstream-stage-contracts.md#tx-01-grant-framing-and-fec-selection); [unified-backend/references/upstream-stage-contracts.md#tx-08-sample-assembly](references/upstream-stage-contracts.md#tx-08-sample-assembly); [unified-backend/references/backend-stages-validation.md](references/backend-stages-validation.md).

#### `ofdma/receiver.md`

RX-01..09; received-only admission/framing/FFT/estimation, inverse resource processing, LLRs, shortening and independent source verification. Wider receiver remains deferred.

Canonical requirements: [REQ-US-RX-01](../../../requirements.md#req-us-rx-01), [REQ-US-RX-02](../../../requirements.md#req-us-rx-02), [REQ-US-RX-03](../../../requirements.md#req-us-rx-03), [REQ-US-RX-04](../../../requirements.md#req-us-rx-04), [REQ-US-RX-05](../../../requirements.md#req-us-rx-05), [REQ-US-RX-06](../../../requirements.md#req-us-rx-06), [REQ-US-RX-07](../../../requirements.md#req-us-rx-07), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-US-RX-09](../../../requirements.md#req-us-rx-09).
Existing tasks: `INT-US-RX-01`, `INT-FEC-01`, `INT-DEFER-01`.
Architecture: [unified-upstream-processing-order](../../../architecture.md#unified-upstream-processing-order).

Source material: [unified-backend/references/upstream-stage-contracts.md#rx-01-burst-framing-and-timing](references/upstream-stage-contracts.md#rx-01-burst-framing-and-timing); [unified-backend/references/upstream-stage-contracts.md#rx-09-information-deframing](references/upstream-stage-contracts.md#rx-09-information-deframing); [unified-backend/references/extension-designs-receiver.md](references/extension-designs-receiver.md).

#### `ofdma/channel.md`

CH-01..06 and Y=XCH+N; direction-specific state, residual measurement policy, timing/drain and observation limits. Wider composition remains deferred.

Canonical requirements: [REQ-US-CH-01](../../../requirements.md#req-us-ch-01), [REQ-US-CH-02](../../../requirements.md#req-us-ch-02), [REQ-US-CH-03](../../../requirements.md#req-us-ch-03), [REQ-US-CH-04](../../../requirements.md#req-us-ch-04), [REQ-US-CH-05](../../../requirements.md#req-us-ch-05), [REQ-US-CH-06](../../../requirements.md#req-us-ch-06).
Existing tasks: `INT-US-CH-01`, `INT-PNM-02`, `INT-DEFER-01`.
Architecture: [unified-channel-and-measurement-reference-planes](../../../architecture.md#unified-channel-and-measurement-reference-planes).

Source material: [unified-backend/references/upstream-stage-contracts.md#ch-01-input-admission](references/upstream-stage-contracts.md#ch-01-input-admission); [unified-backend/references/upstream-stage-contracts.md#ch-06-output-and-observations](references/upstream-stage-contracts.md#ch-06-output-and-observations); [unified-backend/references/extension-designs-channel.md](references/extension-designs-channel.md).

#### `ofdma/pre-equalization.md`

Applied correction C versus physical H; TX-06 filtering, banks, time/grid/provenance and update semantics; parsing belongs to PNM adapter.

Canonical requirements: [REQ-US-TX-06](../../../requirements.md#req-us-tx-06), [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02), [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03), [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04).
Existing tasks: `INT-US-TX-01`, `INT-PNM-01`, `INT-PNM-02`.
Architecture: [unified-channel-and-measurement-reference-planes](../../../architecture.md#unified-channel-and-measurement-reference-planes), [unified-pnm-and-capture-admission](../../../architecture.md#unified-pnm-and-capture-admission).

Source material: [unified-backend/references/upstream-stage-contracts.md#tx-06-cm-pre-eq](references/upstream-stage-contracts.md#tx-06-cm-pre-eq); [unified-backend/references/upstream-stage-contracts.md#us-pre-eq-input](references/upstream-stage-contracts.md#us-pre-eq-input); [unified-backend/references/remaining-delivery-validation.md#preeq-format-01](references/remaining-delivery-validation.md#preeq-format-01).

#### `pnm-measurement-inputs/upstream.md`

UCD/RxMER/Pre-Eq bounded decode, association, policies and whole-plan playback; preserve raw bytes/grids/unknowns; source format evidence is not integrated support.

Canonical requirements: [REQ-US-PNM-01](../../../requirements.md#req-us-pnm-01), [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02), [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03), [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04).
Existing tasks: `INT-PNM-01`, `INT-PNM-02`.
Architecture: [unified-pnm-and-capture-admission](../../../architecture.md#unified-pnm-and-capture-admission).

Source material: [pnm-measurement-inputs/README.md](../pnm-measurement-inputs/README.md); [unified-backend/references/capture-formats-validation.md](references/capture-formats-validation.md); [unified-backend/references/rxmer-capture-validation.md](references/rxmer-capture-validation.md); [unified-backend/references/remaining-delivery-validation.md](references/remaining-delivery-validation.md); [unified-backend/references/upstream-stage-contracts.md#rxmer-input](references/upstream-stage-contracts.md#rxmer-input).

#### `high-level-api/upstream.md`

Paired direction-aware composition, errors, lifecycle, bounds and reports through the same Rust engine; current downstream imports and persistence preserved; no early FEC-only Python facade.

Canonical requirements: [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-04](../../../requirements.md#req-com-api-04), [REQ-COM-API-05](../../../requirements.md#req-com-api-05), [REQ-COM-API-06](../../../requirements.md#req-com-api-06), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-COM-IDENTITY-01](../../../requirements.md#req-com-identity-01).
Existing tasks: `INT-US-LOOP-01`, `INT-RUST-01`, `INT-PY-01`, `INT-CLI-01`.
Architecture: [unified-rust-python-and-cli-apis](../../../architecture.md#unified-rust-python-and-cli-apis).

Source material: [high-level-api/design.md](../high-level-api/design.md); [high-level-api/requirements.md](../high-level-api/requirements.md); [unified-backend/references/remaining-delivery-validation.md#aggregate-api-and-scientific-acceptance](references/remaining-delivery-validation.md#aggregate-api-and-scientific-acceptance).

### Historical references: individual decisions

Each original remains byte-identical at its current location. These are archival
source records, not the active owner of unified MERLIN contracts.

| Preserved original | Functional adaptation / review |
| --- | --- |
| [unified-backend/references/backend-stages-validation.md](references/backend-stages-validation.md) | `ofdma/transmitter.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/capture-formats-validation.md](references/capture-formats-validation.md) | `pnm-measurement-inputs/upstream.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/extension-designs-channel.md](references/extension-designs-channel.md) | `ofdma/channel.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/extension-designs-ranging.md](references/extension-designs-ranging.md) | `ofdma/configuration.md` Retain deferred ranging/probe design; cross-link its PHY-specific differences from OFDMA configuration; do not create new runtime/UI work. |
| [unified-backend/references/extension-designs-receiver.md](references/extension-designs-receiver.md) | `ofdma/receiver.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/extension-designs-ui.md](references/extension-designs-ui.md) | Existing visual-workspace navigation; deferred INT-DEFER-01. Retain future presentation source design; group under existing visual-workspace navigation and INT-DEFER-01, without adopting obsolete Qt files or beginning Qt work. |
| [unified-backend/references/foundation-validation.md](references/foundation-validation.md) | `ofdma/configuration.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/ldpc-source-audit-validation.md](references/ldpc-source-audit-validation.md) | `fec/design.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/remaining-delivery-validation.md](references/remaining-delivery-validation.md) | `ofdma/pre-equalization.md`, `pnm-measurement-inputs/upstream.md`, `high-level-api/upstream.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/rxmer-capture-validation.md](references/rxmer-capture-validation.md) | `pnm-measurement-inputs/upstream.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/upstream-stage-contracts.md](references/upstream-stage-contracts.md) | `fec/design.md`, `ofdma/configuration.md`, `ofdma/transmitter.md`, `ofdma/receiver.md`, `ofdma/channel.md`, `ofdma/pre-equalization.md`, `pnm-measurement-inputs/upstream.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |
| [unified-backend/references/us-configuration-json-coverage.md](references/us-configuration-json-coverage.md) | `ofdma/configuration.md` Retain complete historical validation/limits as immutable provenance; active contracts live in the named functional output(s). |

The large stage appendix already has a frozen hash in the downstream evidence
index. Keeping it intact preserves its 23 stage definitions and PRBS/Pre-Eq/RxMER
source contracts without invalidating archival locators. Its old source-module
paths remain provenance. New records supply MERLIN paths and current support states.
Original normative section/table references, independent row/edge/parity expectations,
configuration coverage and documented limitations are retained. No private PDFs,
captures or generated runtime artifacts are added.

## Inbound-link impact

| Current source | Feature Markdown links | Distinct target files |
| --- | --- | --- |
| [docs/architecture.md](../../../architecture.md) | 87 | 66 |
| [docs/requirements.md](../../../requirements.md) | 573 | 107 |
| [docs/development/checklist.md](../../checklist.md) | 193 | 88 |
| [docs/development/ui-checklist.md](../../ui-checklist.md) | 63 | 35 |
| [docs/development/README.md](../../README.md) | 28 | 26 |
| [README.md](../README.md) | 143 | 141 |
| [AGENTS.md](../../../../AGENTS.md) | 2 | 3 |

The baseline scan found **1,808 local feature-reference occurrences across 110 tracked source files**,
from 1,313 tracked UTF-8 files inspected. It covers Markdown and exact root/absolute/
relative plain-text paths, including all six feature JSON records, tooling, source
comments, user/API documentation and skills. The inventory records every observed
occurrence. Bare same-folder filenames are conservative possible references, not
proof that a quoted example is an active locator. External URLs are excluded from
local rewriting; private/untracked/generated content and remote availability are
outside this tracked baseline.

With no moves, all current destinations/anchors remain valid. Phase 3 adds focused
links in the three canonical documents and indexes; it does not replace hundreds
of established links. No Markdown or JSON global text replacement is needed.
Any newly proposed path is code-formatted until it exists, then linked relative
to the containing document. Historical JSON path/hash fields and pinned external
source URLs remain untouched. Canonical skill references retain their locations.

## Requirements, tasks and architecture impact

Preserve **515 canonical requirement IDs, two retained compatibility aliases, 59
source contracts and 93 original numbered API clauses**. Add no behavior requirement
and renumber nothing. Preserve every original checkbox line: **375 official and
146 UI**. Feature records do not become a second completion owner.

After approval, add one distinct unchecked **DOC-ORG-01** task to the official
checklist, linking DOC-R3 and REQ-COM-DOC-01, the current architecture/evidence owner
and this mapping. Prerequisites: DS-REQ-01 and INT-DOC-01. Add DOC-ORG-01 as a
documentation prerequisite to INT-DOC-02; update the matching current dependency
metadata. Check DOC-ORG-01 only after the approved structural acceptance passes.
No existing runtime integration task changes state because of documentation work.

All 21 integration tasks currently pass the nonempty requirement/architecture
return-link guard, including INT-PNM-01 with REQ-US-PNM-01/02/03. Retain that guard.
Use the existing canonical contract register and backfill routes; distinguish direct
IDs, inherited same-feature routes, proposed navigation associations and purely
historical source aliases. Records without a direct functional ID retain an explicit
coverage note and documentation-stewardship owner; this is not invented old acceptance.

### Findings requiring explicit treatment

1. **Historical UI-DOC-02 alias collision:** the official checklist has two completed
   entries with that label. Existing unique task anchors are
   `task-req-ds-doc-terms-01` and `task-req-ds-doc-png-01`; the paired canonical
   requirement IDs are REQ-DS-DOC-TERMS-01 and REQ-DS-DOC-PNG-01. Preserve the
   original labels and checkmarks, and validate uniqueness by stable path/anchor.
   Require unique labels for every new task. Literal historical label uniqueness
   conflicts with the instruction to preserve historical IDs; it cannot be claimed.
2. **Stale current FEC test locator:** REQ-COM-SYS-05 and its current source-inventory
   owner still name `crates/fec-codec/tests/upstream_source_audit.rs` (absent).
   After approval, point them to the actual
   `crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs`. Preserve behavior,
   acceptance clauses, original source audit provenance and requirement ID.
3. **Historical pending wrapper:** the preserved stage appendix says all corresponding
   upstream runtime work is pending. Its source-specific framing stays archived.
   The new functional records and canonical links distinguish accepted standalone
   LDPC (INT-FEC-01) from the still-planned upstream transport.

## Support-state presentation

Every functional record separates architectural decision, implementation, numerical
validation and integration acceptance. The reviewed reuse plan does not itself
prove numerical equivalence. In particular:

| Scope | Current interpretation to preserve |
| --- | --- |
| Existing downstream features | Accepted within the official recorded supported scope; historical evidence retains its original platform/date. |
| Sparse encoder / upstream standalone LDPC | INT-FEC-01 has actual MERLIN evidence; matrices remain direction-specific and ordinary upstream has no BCH. |
| Shared FFT/QAM/PRBS/noise/channel kernels | Reuse decisions reviewed; INT-DSP-01 extraction/equivalence acceptance remains open. No new DSP crate exists. |
| Upstream configuration/TX/channel/RX/PNM | Planned local integration, with source records explicitly historical. |
| Rust/Python APIs | Current downstream APIs retained; upstream core/Rust-first and paired adapter gates remain open. |
| Future upstream Qt / ranging / wider receiver/channel | Explicitly deferred, not implementation authorized by this reorganization. |

## Automated validation and acceptance plan

Before each approved edit, compare the current branch/commit and all original file
hashes against this baseline. Preserve any new user changes; stop only a conflicting
move/edit and reconcile it without resetting the branch. Proposed validator owners:
`tools/validation/feature_documentation.py` and focused tests under
`tools/validation/test_feature_documentation.py`; neither is created at this phase.

After approval the structural checks must:

1. Resolve local Markdown destinations and GitHub-style fragments, explicit anchors,
   duplicate-heading suffixes, images and relative paths; ignore literal fenced examples.
2. Match all original 170 inventory entries and expected nine new functional records;
   prove all 151 Keep and 12 Archive originals are byte-identical and review each
   of the seven Adapt diffs. Retain all original heading/requirement/task anchors.
3. Compare the 515-ID canonical set, retained aliases, source clauses/API text and
   requirement→task→architecture/module/test relationships. Do not equate a test
   path with an executed pass. Validate one representative chain per functional group.
4. Compare all 521 historical checkbox lines and named task anchors to the baseline;
   allow only the new DOC-ORG-01 state. Validate new IDs, known legacy aliases,
   inventory/checklist dependencies and graph acyclicity.
5. Validate maintained implementation/test locators as existing or explicitly planned;
   tag pinned source/private/generated paths as provenance, not local runtime support.
   Correct the specifically identified current FEC owner and test the rejection case.
6. Validate all six original JSON inventories/fixtures in their proper roles. Preserve
   frozen source identities and evidence hashes; new current metadata must not rewrite
   historical locators. The plan inventory remains a snapshot, not a status tracker.
7. Traverse Markdown navigation from the canonical/index/skill roots; every feature
   must remain reachable and every new functional record indexed. Check both Markdown
   and plain-text feature paths without modifying quoted historical examples.
8. Validate the 12 canonical skills, all 24 discovery symlinks and their local references.
   Extend relevant references to new functional owners without parallel duplicate skills.
9. Run `python3 -B tools/validation/integration_traceability.py`, existing validation-tool
   unittest discovery (plus the proposed focused cases), the generated Python guide
   check and `git diff --check`. Verify all implementation/manifests/locks/UI/vector
   files remain unchanged. No Rust simulation or Qt rebuild is justified by these docs.
10. Append a completion report to the existing unified-backend validation record with
    original/final counts, exact dispositions, IDs/states, links, evidence retention,
    skill results and limitations. Reconcile DOC-ORG-01 only; final backend acceptance
    stays open. Supply the documentation commit message after the approved refactor.

Useful existing commands (documentation checks only):

```bash
python3 -B tools/validation/integration_traceability.py
python3 -B -m unittest discover -s tools/validation -p 'test_*.py'
python3 -B tools/python/update_integration_doc.py --check
git diff --check
```

### Checks actually executed during planning

At the clean baseline, 6,842 maintained local Markdown links/fragments resolve; no feature
document is unreachable from canonical/index roots. This is a tracked documentation
scan, not a remote URL or scientific validation. The existing integration traceability
guard passes all 21 tasks; validation-tool discovery passes nine tests. All 12 skill
metadata records and 24 discovery links pass. The saved inventory matches all 170
original feature paths/hashes. No new runtime tests or acceptance are claimed.

## Risks and approval decisions

- Frozen evidence and JSON locators: retain all original paths and archival bytes;
  change only explicitly current metadata. The one-to-one map makes that reviewable.
- Scientific drift: adapt conventions, bounds and reference planes explicitly;
  keep OFDM control/BCH, upstream grants/pilots/Pre-Eq and source evidence distinct.
- Redundant authorities: functional designs link canonical requirements and official
  task status; they add neither new project-wide baselines nor competing checklists.
- Navigation: 13 records currently need an inferred navigation association; the
  inventory labels that basis. New grouping must not present it as recovered history.
- Historical duplicate label: approve preserving the two UI-DOC-02 aliases with their
  existing distinct stable anchors; do not silently renumber or assert literal uniqueness.

**Approval requested:** the zero-move/zero-deletion mapping, nine new functional
records, seven focused adaptations, unchanged historical archive, proposed DOC-ORG-01
gate and the explicit legacy-label handling above. Functional directories and
canonical routing changes begin only after that review, as the attached request requires.

## Complete original-file inventory

For every row, **proposed path equals the exact existing path**. This table is the
complete 170-file map; full requirement/task/architecture/evidence and inbound occurrence
details are in the machine-readable companion. Archive means preserve the existing
historical archive, not relocate it. Keep files do not need navigation edits in their bodies.

| Existing = proposed path | Disposition | Functional owner | Inbound occurrences | Risk |
| --- | --- | --- | --- | --- |
| [README.md](../README.md) | Adapt | Integration / functional navigation | 10 | Medium |
| [about-menu/design.md](../about-menu/design.md) | Keep | Workbench / Qt presentation | 7 | Low |
| [automatic-updates/design.md](../automatic-updates/design.md) | Keep | Delivery / development tooling | 10 | Low |
| [build-cache/design.md](../build-cache/design.md) | Keep | Delivery / development tooling | 12 | Low |
| [carrier-layout/architecture.md](../carrier-layout/architecture.md) | Keep | OFDM transmission / signaling | 6 | Low |
| [cell-interleaving/architecture.md](../cell-interleaving/architecture.md) | Keep | OFDM transmission / signaling | 8 | Low |
| [cell-randomizer/architecture.md](../cell-randomizer/architecture.md) | Keep | OFDM transmission / signaling | 5 | Low |
| [channel-runner/plan.md](../channel-runner/plan.md) | Keep | Simulation source / lifecycle | 3 | Low |
| [channel-runner/validation.md](../channel-runner/validation.md) | Keep | Simulation source / lifecycle | 13 | Low |
| [channel-tilt/design.md](../channel-tilt/design.md) | Keep | OFDM channel / RF | 11 | Low |
| [channel-widths/design.md](../channel-widths/design.md) | Keep | DSP / downstream OFDM | 8 | Low |
| [channel-workbench/design.md](../channel-workbench/design.md) | Keep | Workbench / Qt presentation | 17 | Low |
| [checklist-audit/plan.md](../checklist-audit/plan.md) | Keep | Integration / documentation governance | 3 | Low |
| [checklist-audit/validation.md](../checklist-audit/validation.md) | Keep | Integration / documentation governance | 3 | Low |
| [checklist-naming/plan.md](../checklist-naming/plan.md) | Keep | Integration / documentation governance | 3 | Low |
| [cl-notch-workbench/comparison-sim-1.json](../cl-notch-workbench/comparison-sim-1.json) | Keep | Workbench / Qt presentation | 2 | Low |
| [cl-notch-workbench/comparison-sim-1.md](../cl-notch-workbench/comparison-sim-1.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [cl-notch-workbench/design.md](../cl-notch-workbench/design.md) | Keep | Workbench / Qt presentation | 13 | Low |
| [closed-loop-validation/design.md](../closed-loop-validation/design.md) | Keep | Simulation source / lifecycle | 12 | Low |
| [closed-loop-validation/validation.md](../closed-loop-validation/validation.md) | Keep | Simulation source / lifecycle | 15 | Low |
| [command-timing/design.md](../command-timing/design.md) | Keep | Delivery / development tooling | 8 | Low |
| [completion-group/design.md](../completion-group/design.md) | Keep | Simulation source / lifecycle | 6 | Low |
| [completion-group/external-channel-vectors.md](../completion-group/external-channel-vectors.md) | Keep | Simulation source / lifecycle | 3 | Low |
| [completion-group/validation.md](../completion-group/validation.md) | Keep | Simulation source / lifecycle | 15 | Low |
| [contextual-help/design.md](../contextual-help/design.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [development-setup/design.md](../development-setup/design.md) | Keep | Delivery / development tooling | 10 | Low |
| [development-workspace/plan.md](../development-workspace/plan.md) | Keep | Integration / documentation governance | 3 | Low |
| [docs-only-push/design.md](../docs-only-push/design.md) | Keep | Delivery / development tooling | 7 | Low |
| [downstream-requirements/design.md](../downstream-requirements/design.md) | Keep | Integration / documentation governance | 10 | Low |
| [downstream-requirements/traceability.json](../downstream-requirements/traceability.json) | Keep | Integration / documentation governance | 5 | Low |
| [downstream-requirements/validation.md](../downstream-requirements/validation.md) | Keep | Integration / documentation governance | 11 | Low |
| [engineering-guidance/design.md](../engineering-guidance/design.md) | Keep | Integration / documentation governance | 22 | Low |
| [engineering-guidance/source-inventory.json](../engineering-guidance/source-inventory.json) | Keep | Integration / documentation governance | 4 | Low |
| [engineering-guidance/validation.md](../engineering-guidance/validation.md) | Keep | Integration / documentation governance | 12 | Low |
| [example-layout/design.md](../example-layout/design.md) | Keep | Delivery / development tooling | 8 | Low |
| [example-layout/validation.md](../example-layout/validation.md) | Keep | Delivery / development tooling | 5 | Low |
| [fec-monitor/design.md](../fec-monitor/design.md) | Keep | Workbench / Qt presentation | 13 | Low |
| [fec-shortening/design.md](../fec-shortening/design.md) | Keep | FEC / direction-specific signaling | 9 | Low |
| [fec-shortening/validation.md](../fec-shortening/validation.md) | Keep | FEC / direction-specific signaling | 7 | Low |
| [file-payload-source/design.md](../file-payload-source/design.md) | Keep | Simulation source / lifecycle | 5 | Low |
| [file-payload-source/implementation.md](../file-payload-source/implementation.md) | Keep | Simulation source / lifecycle | 13 | Low |
| [high-level-api/design.md](../high-level-api/design.md) | Adapt | Public Rust / Python API | 76 | Medium |
| [high-level-api/requirements.md](../high-level-api/requirements.md) | Keep | Public Rust / Python API | 10 | Low |
| [high-level-api/validation.md](../high-level-api/validation.md) | Keep | Public Rust / Python API | 168 | Low |
| [impairment-group/plan.md](../impairment-group/plan.md) | Keep | OFDM channel / RF | 3 | Low |
| [impairment-group/validation.md](../impairment-group/validation.md) | Keep | OFDM channel / RF | 12 | Low |
| [independent-channel-estimation/design.md](../independent-channel-estimation/design.md) | Keep | OFDM receiver / channel estimation | 11 | Low |
| [independent-channel-estimation/implementation.md](../independent-channel-estimation/implementation.md) | Keep | OFDM receiver / channel estimation | 13 | Low |
| [lte-ingress/plan.md](../lte-ingress/plan.md) | Keep | OFDM channel / RF | 3 | Low |
| [lte-ingress/validation.md](../lte-ingress/validation.md) | Keep | OFDM channel / RF | 3 | Low |
| [mouse-zoom/design.md](../mouse-zoom/design.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [ncp-codec/architecture.md](../ncp-codec/architecture.md) | Keep | FEC / direction-specific signaling | 6 | Low |
| [paired-constellation/implementation.md](../paired-constellation/implementation.md) | Keep | OFDM receiver / channel estimation | 17 | Low |
| [paired-constellation/plan.md](../paired-constellation/plan.md) | Keep | OFDM receiver / channel estimation | 6 | Low |
| [parallel-local-tests/design.md](../parallel-local-tests/design.md) | Keep | Delivery / development tooling | 13 | Low |
| [payload-transport/plan.md](../payload-transport/plan.md) | Keep | OFDM transmission / signaling | 3 | Low |
| [payload-transport/validation.md](../payload-transport/validation.md) | Keep | OFDM transmission / signaling | 6 | Low |
| [pilot-channel-estimation/implementation.md](../pilot-channel-estimation/implementation.md) | Keep | OFDM receiver / channel estimation | 24 | Low |
| [pilot-channel-estimation/plan.md](../pilot-channel-estimation/plan.md) | Keep | OFDM receiver / channel estimation | 6 | Low |
| [pilot-plc-signals/architecture.md](../pilot-plc-signals/architecture.md) | Keep | OFDM transmission / signaling | 15 | Low |
| [plc-block-interleaving/architecture.md](../plc-block-interleaving/architecture.md) | Keep | OFDM transmission / signaling | 5 | Low |
| [plc-completion/plan.md](../plc-completion/plan.md) | Keep | OFDM transmission / signaling | 3 | Low |
| [plc-completion/validation.md](../plc-completion/validation.md) | Keep | OFDM transmission / signaling | 10 | Low |
| [plc-ldpc/architecture.md](../plc-ldpc/architecture.md) | Keep | FEC / direction-specific signaling | 5 | Low |
| [plc-message-blocks/architecture.md](../plc-message-blocks/architecture.md) | Keep | OFDM transmission / signaling | 5 | Low |
| [plc-randomization/architecture.md](../plc-randomization/architecture.md) | Keep | OFDM transmission / signaling | 3 | Low |
| [plc-randomization/validation.md](../plc-randomization/validation.md) | Keep | OFDM transmission / signaling | 5 | Low |
| [pnm-measurement-inputs/README.md](../pnm-measurement-inputs/README.md) | Adapt | PNM / measurement processing | 3 | Medium |
| [pnm-measurement-inputs/profile-view.md](../pnm-measurement-inputs/profile-view.md) | Keep | PNM / measurement processing | 14 | Low |
| [pnm-reference-skill/design.md](../pnm-reference-skill/design.md) | Keep | PNM / measurement processing | 6 | Low |
| [popup-close-controls/design.md](../popup-close-controls/design.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [prbs-source/plan.md](../prbs-source/plan.md) | Keep | Simulation source / lifecycle | 3 | Low |
| [prbs-source/validation.md](../prbs-source/validation.md) | Keep | Simulation source / lifecycle | 8 | Low |
| [profile-performance/design.md](../profile-performance/design.md) | Keep | Workbench / Qt presentation | 12 | Low |
| [project-architecture/plan.md](../project-architecture/plan.md) | Keep | Integration / documentation governance | 3 | Low |
| [project-rename/notes.md](../project-rename/notes.md) | Keep | Integration / documentation governance | 3 | Low |
| [python-adapter/design.md](../python-adapter/design.md) | Keep | Public Rust / Python API | 4 | Low |
| [qam-completion/design.md](../qam-completion/design.md) | Keep | DSP / downstream OFDM | 8 | Low |
| [qam-completion/external-vectors.md](../qam-completion/external-vectors.md) | Keep | DSP / downstream OFDM | 9 | Low |
| [qam-completion/validation.md](../qam-completion/validation.md) | Keep | DSP / downstream OFDM | 11 | Low |
| [qam-demapper-performance/design.md](../qam-demapper-performance/design.md) | Keep | DSP / downstream OFDM | 8 | Low |
| [qam-mapping/architecture.md](../qam-mapping/architecture.md) | Keep | DSP / downstream OFDM | 5 | Low |
| [qam-markers/design.md](../qam-markers/design.md) | Keep | Workbench / Qt presentation | 6 | Low |
| [qam-mer/design.md](../qam-mer/design.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [qam-receiver-mode/design.md](../qam-receiver-mode/design.md) | Keep | Workbench / Qt presentation | 4 | Low |
| [qam-run-results/design.md](../qam-run-results/design.md) | Keep | Workbench / Qt presentation | 9 | Low |
| [qam256-coding-loop/architecture.md](../qam256-coding-loop/architecture.md) | Keep | DSP / downstream OFDM | 3 | Low |
| [qt-closed-loop/design.md](../qt-closed-loop/design.md) | Keep | Workbench / Qt presentation | 14 | Low |
| [qt-closed-loop/validation.md](../qt-closed-loop/validation.md) | Keep | Workbench / Qt presentation | 16 | Low |
| [qt-source-organization/design.md](../qt-source-organization/design.md) | Keep | Workbench / Qt presentation | 14 | Low |
| [quick-local-tests/design.md](../quick-local-tests/design.md) | Keep | Delivery / development tooling | 6 | Low |
| [readme-workbench-diagram/design.md](../readme-workbench-diagram/design.md) | Keep | Workbench / Qt presentation | 12 | Low |
| [receiver-acquisition/design.md](../receiver-acquisition/design.md) | Keep | OFDM receiver / channel estimation | 9 | Low |
| [receiver-acquisition/validation.md](../receiver-acquisition/validation.md) | Keep | OFDM receiver / channel estimation | 6 | Low |
| [receiver-configuration/plan.md](../receiver-configuration/plan.md) | Keep | OFDM receiver / channel estimation | 3 | Low |
| [receiver-configuration/validation.md](../receiver-configuration/validation.md) | Keep | OFDM receiver / channel estimation | 5 | Low |
| [receiver-instruments/design.md](../receiver-instruments/design.md) | Keep | Workbench / Qt presentation | 28 | Low |
| [receiver-workbench/design.md](../receiver-workbench/design.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [receiver-workbench/validation.md](../receiver-workbench/validation.md) | Keep | Workbench / Qt presentation | 16 | Low |
| [release-asset-names/design.md](../release-asset-names/design.md) | Keep | Delivery / development tooling | 6 | Low |
| [release-resume/design.md](../release-resume/design.md) | Keep | Delivery / development tooling | 8 | Low |
| [responsive-workbench-open/design.md](../responsive-workbench-open/design.md) | Keep | Workbench / Qt presentation | 15 | Low |
| [restore-workbench-inputs/design.md](../restore-workbench-inputs/design.md) | Keep | Workbench / Qt presentation | 9 | Low |
| [rf-power-budget/design.md](../rf-power-budget/design.md) | Keep | OFDM channel / RF | 24 | Low |
| [run-elapsed-time/design.md](../run-elapsed-time/design.md) | Keep | Workbench / Qt presentation | 10 | Low |
| [run-infrastructure/plan.md](../run-infrastructure/plan.md) | Keep | Simulation source / lifecycle | 3 | Low |
| [run-infrastructure/validation.md](../run-infrastructure/validation.md) | Keep | Simulation source / lifecycle | 27 | Low |
| [rust-logging/architecture.md](../rust-logging/architecture.md) | Keep | Delivery / development tooling | 5 | Low |
| [rust-only-signal-processing/architecture.md](../rust-only-signal-processing/architecture.md) | Keep | Integration / documentation governance | 7 | Low |
| [rxmer-designer/design.md](../rxmer-designer/design.md) | Keep | PNM / measurement processing | 32 | Low |
| [rxmer-designer/pnm-export-plan.md](../rxmer-designer/pnm-export-plan.md) | Keep | PNM / measurement processing | 9 | Low |
| [rxmer-designer/validation.md](../rxmer-designer/validation.md) | Keep | PNM / measurement processing | 21 | Low |
| [rxmer-error-sweep/README.md](../rxmer-error-sweep/README.md) | Keep | PNM / measurement processing | 3 | Low |
| [rxmer-selection/design.md](../rxmer-selection/design.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [rxmer-workbench/design.md](../rxmer-workbench/design.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [rxmer-workbench/validation.md](../rxmer-workbench/validation.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [security-checks/design.md](../security-checks/design.md) | Keep | Delivery / development tooling | 9 | Low |
| [simulation-documentation/plan.md](../simulation-documentation/plan.md) | Keep | Integration / documentation governance | 3 | Low |
| [simulation-documentation/validation.md](../simulation-documentation/validation.md) | Keep | Integration / documentation governance | 3 | Low |
| [single-channel-transmission/architecture.md](../single-channel-transmission/architecture.md) | Keep | OFDM transmission / signaling | 5 | Low |
| [source-organization/architecture.md](../source-organization/architecture.md) | Keep | Integration / documentation governance | 5 | Low |
| [sparse-encoder/design.md](../sparse-encoder/design.md) | Keep | FEC / direction-specific signaling | 9 | Low |
| [symbol-packing/architecture.md](../symbol-packing/architecture.md) | Keep | OFDM transmission / signaling | 6 | Low |
| [symbol-packing/design.md](../symbol-packing/design.md) | Keep | OFDM transmission / signaling | 6 | Low |
| [symbol-timing-popup/design.md](../symbol-timing-popup/design.md) | Keep | Workbench / Qt presentation | 9 | Low |
| [terminology/design.md](../terminology/design.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [tooltip-delay/design.md](../tooltip-delay/design.md) | Keep | Workbench / Qt presentation | 6 | Low |
| [transmitter-graphs/design.md](../transmitter-graphs/design.md) | Keep | Workbench / Qt presentation | 12 | Low |
| [transmitter-spectrum/design.md](../transmitter-spectrum/design.md) | Keep | Workbench / Qt presentation | 16 | Low |
| [transmitter-spectrum/validation.md](../transmitter-spectrum/validation.md) | Keep | Workbench / Qt presentation | 10 | Low |
| [transmitter-workbench/design.md](../transmitter-workbench/design.md) | Keep | Workbench / Qt presentation | 22 | Low |
| [ubuntu-desktop/design.md](../ubuntu-desktop/design.md) | Keep | Delivery / development tooling | 29 | Low |
| [ubuntu-desktop/local-validation.md](../ubuntu-desktop/local-validation.md) | Keep | Delivery / development tooling | 5 | Low |
| [ui-copy/design.md](../ui-copy/design.md) | Keep | Workbench / Qt presentation | 7 | Low |
| [unified-backend/baseline.md](baseline.md) | Keep | Integration / documentation governance | 26 | Low |
| [unified-backend/design.md](design.md) | Adapt | Integration / documentation governance | 15 | Medium |
| [unified-backend/fec-validation.md](fec-validation.md) | Keep | Integration / documentation governance | 22 | Low |
| [unified-backend/references/backend-stages-validation.md](references/backend-stages-validation.md) | Archive | OFDMA / source integration historical evidence | 3 | High |
| [unified-backend/references/capture-formats-validation.md](references/capture-formats-validation.md) | Archive | PNM / API / delivery historical evidence | 3 | High |
| [unified-backend/references/extension-designs-channel.md](references/extension-designs-channel.md) | Archive | OFDMA channel (historical) | 3 | High |
| [unified-backend/references/extension-designs-ranging.md](references/extension-designs-ranging.md) | Archive | OFDMA ranging / probes (deferred historical) | 3 | High |
| [unified-backend/references/extension-designs-receiver.md](references/extension-designs-receiver.md) | Archive | OFDMA receiver (historical) | 3 | High |
| [unified-backend/references/extension-designs-ui.md](references/extension-designs-ui.md) | Archive | Workbench / future OFDMA presentation (historical) | 3 | High |
| [unified-backend/references/foundation-validation.md](references/foundation-validation.md) | Archive | OFDMA / source integration historical evidence | 3 | High |
| [unified-backend/references/ldpc-source-audit-validation.md](references/ldpc-source-audit-validation.md) | Archive | FEC / upstream LDPC historical evidence | 5 | High |
| [unified-backend/references/remaining-delivery-validation.md](references/remaining-delivery-validation.md) | Archive | PNM / API / delivery historical evidence | 4 | High |
| [unified-backend/references/rxmer-capture-validation.md](references/rxmer-capture-validation.md) | Archive | PNM / API / delivery historical evidence | 3 | High |
| [unified-backend/references/upstream-stage-contracts.md](references/upstream-stage-contracts.md) | Archive | OFDMA / source integration historical evidence | 3 | High |
| [unified-backend/references/us-configuration-json-coverage.md](references/us-configuration-json-coverage.md) | Archive | OFDMA configuration (historical) | 3 | High |
| [unified-backend/reuse-review.md](reuse-review.md) | Keep | Integration / documentation governance | 7 | Low |
| [unified-backend/scientific-source-review.json](scientific-source-review.json) | Keep | Integration / documentation governance | 5 | Low |
| [unified-backend/source-inventory.json](source-inventory.json) | Adapt | Integration / documentation governance | 7 | High |
| [unified-backend/source-review.md](source-review.md) | Adapt | Integration / documentation governance | 10 | Medium |
| [unified-backend/validation.md](validation.md) | Adapt | Integration / documentation governance | 13 | Medium |
| [upstream-skills/design.md](../upstream-skills/design.md) | Keep | Integration / documentation governance | 24 | Low |
| [upstream-skills/source-inventory.json](../upstream-skills/source-inventory.json) | Keep | Integration / documentation governance | 4 | Low |
| [upstream-skills/validation.md](../upstream-skills/validation.md) | Keep | Integration / documentation governance | 13 | Low |
| [user-guide-captures/design.md](../user-guide-captures/design.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [user-guide-captures/png-audit.md](../user-guide-captures/png-audit.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [verification-panel/design.md](../verification-panel/design.md) | Keep | Workbench / Qt presentation | 8 | Low |
| [visual-workspace/design.md](../visual-workspace/design.md) | Keep | Workbench / Qt presentation | 22 | Low |
| [visual-workspace/plan.md](../visual-workspace/plan.md) | Keep | Workbench / Qt presentation | 11 | Low |
| [visual-workspace/validation.md](../visual-workspace/validation.md) | Keep | Workbench / Qt presentation | 12 | Low |
| [waveform-direction/notes.md](../waveform-direction/notes.md) | Keep | OFDM transmission / signaling | 3 | Low |
| [waveform/plan.md](../waveform/plan.md) | Keep | OFDM transmission / signaling | 3 | Low |
| [waveform/validation.md](../waveform/validation.md) | Keep | OFDM transmission / signaling | 14 | Low |
| [windows-desktop/design.md](../windows-desktop/design.md) | Keep | Delivery / development tooling | 21 | Low |
| [windows-desktop/validation.md](../windows-desktop/validation.md) | Keep | Delivery / development tooling | 19 | Low |
| [workbench-configuration/design.md](../workbench-configuration/design.md) | Keep | Workbench / Qt presentation | 20 | Low |
| [workbench-examples/design.md](../workbench-examples/design.md) | Keep | Workbench / Qt presentation | 19 | Low |
