# DS-REQ-01: Downstream requirements and traceability backfill

This is an additive documentation change against MERLIN commit
`c91e2c86845c192aa5a6550fb77bbd232c5780e7`. The
[canonical requirements](../../../requirements.md) own behavior;
the [official checklist](../../checklist.md) owns overall completion and the
[UI checklist](../../ui-checklist.md) retains its subordinate detail states.
No scientific implementation, API signature, fixture or test behavior changes.

## Scope and ownership

Inspect existing architecture, both current checklists, feature requirements,
designs, implementation/validation summaries, Rust/Python contracts and their
maintained test owners. Backfill named obligations into the root baseline;
link every original task occurrence in both directions. Keep full nested task
acceptance and design constraints binding rather than copying only their titles.
The machine index is a frozen provenance/traceability record, not another status
baseline. Current completion always comes from the owning checklist.

| File | Disposition / responsibility |
| --- | --- |
| [Architecture](../../../architecture.md#requirements-and-historical-evidence-ownership) | Edit: explain requirement, evidence and status ownership. Preserve all stage sections. |
| [Requirements](../../../requirements.md#detailed-downstream-contract-register) | Edit: retain broad downstream/upstream contracts; add detailed downstream and exact original numbered API clauses. |
| [Official checklist](../../checklist.md#downstream-requirements-and-acceptance-gaps) | Edit: add links/anchors and separate documentation/gap tasks; preserve every original task ID, checkbox and acceptance body. |
| [UI checklist](../../ui-checklist.md) | Edit: add links/anchors; preserve all detail marks and identify obsolete scheduling prose. |
| [Task/evidence index](traceability.json) | Add: current file owners, record/test paths, original labels/marks, aliases and task-body hashes at the reviewed baseline. |
| [Validation](validation.md) | Add: checks actually run, preservation/coverage and limits. |
| Existing feature records, code and tests | Reuse unchanged; keep original normative details, independent oracles and historical results. |
| Ignored API handoff requirements | Read only: preserve the original; publish its 93 numbered requirement clauses verbatim with source hash. Exclude illustrative draft code and private artifacts. |

## Identity and lifecycle rules

Retain existing downstream identifiers such as FEC-03, QAM-04, CH-EST-RX-01..06,
API-WB-01 and REQ-001. New REQ-DS identifiers are needed only for previously
unnamed obligations and the two different UI-DOC-02 contracts. Both original
UI-DOC-02 labels remain unchanged; distinct targets prevent ambiguous links.
Paired composition/receiver IDs remain aliases of one obligation. Unnumbered
UI detail tasks refine existing UI-01..04; a checked detail never closes its
unchecked parent. Feature requirements retain their original design paragraphs.

Historical pass statements are evidence within their declared scope. A planned
validation paragraph, an implementation description or test presence alone does
not establish execution. Link a specific later successful section when the same
record starts with a failed/pending snapshot, especially Windows MSI validation.
Do not replay the full development history. Retain missing-record gaps below and
accept narrowly scoped tests or a recovered valid record against its exact scope.

Earlier UI controls and example filenames can be superseded without invalidating
their historical acceptance. The register names later refinements explicitly.
The API handoff's initial single-profile construction is extended by retained
profiles and idle Simulation sessions; it does not forbid those later APIs.
Historical restrictions in early waveform/measurement records are not current
global support limits. Current supported behavior follows the architecture and
later named refinements, with each old result's original limits preserved.

## Targeted evidence gaps

These findings describe the backfill checkpoint. All three now have scoped
[current or recovered evidence](../unified-backend/baseline.md); their original
findings and acceptance criteria remain below for historical traceability.

| Task | Missing or incomplete evidence | Acceptance / affected requirements |
| --- | --- | --- |
| [DS-GAP-01](../../checklist.md#ds-gap-01) | Source-organization record describes pre/post validation to perform but records no execution result. This does not establish a failed refactor. | Recover an attributable result or run focused public-path/standalone-codec and deterministic CLI compatibility checks. ARCH-02; preserve imports, JSON and seeded draw order. |
| [DS-GAP-02](../../checklist.md#ds-gap-02) | Early FEC/QAM/randomizer/numerology descriptions preserve algorithms/vectors, but do not give a complete attributable execution record for their original standalone acceptance. Later broad gates do not identify every original assertion. | Recover scoped records or execute existing independent codec/constellation/randomizer/settings tests with exact commands/results and limits. FEC-01/02, QAM-01, RAND-01, CFG-01. No new scientific capability is required. |
| [DS-GAP-03](../../checklist.md#ds-gap-03) | Earlier spectrum/measurement-viewer records are insufficiently specific for all accepted DPD preview and later full-view behavior. Maintained adapter/QML tests exist; their presence is not a recorded pass for that exact scope. | Identify the corresponding existing pass record or run focused DPD binary/JSON/grid/role and plot checks in MERLIN. UI-DPD-VIEW-01/02. Native package lifecycle stays with its existing open tasks. |

These tasks preserve original completion marks and expose uncertainty; they do
not assert that previously accepted code is defective. Explicitly pending native
upgrades, provisioning, hosted matrices, wider RF behavior and field-FEC work keep
their existing task owners instead of receiving duplicate historical tasks.

## Delivery order and acceptance

1. DS-REQ-01 depends on INT-DOC-01. Reconcile original identities and current
   refinements, map modules/tests/records, preserve history and validate links.
2. DS-GAP-01..03 depend on DS-REQ-01. Recover evidence or run only the relevant
   existing acceptance cases; record actual results before closing each gap.
3. INT-BASE-01 depends on the documentation baseline. INT-REG-01 additionally
   depends on all three gap tasks. Existing upstream gates remain pending and
   require execution in unified MERLIN, regardless of source passes.
4. INT-DOC-02 and INT-ACCEPT-01 retain the final documentation/backend gates.
   A completed traceability baseline is not final runtime acceptance.

Acceptance for DS-REQ-01: all original official/UI task occurrences and nested
marks survive; every applicable downstream requirement has an owner, task and
test/record; absent execution evidence has a linked open task; all 93 original
API clauses and 59 upstream mappings survive; internal links resolve; no source
code, test vectors, private evidence or original source documents are overwritten.
