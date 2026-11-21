# DS-REQ-01 documentation validation

Current reconciliation: [the pre-import baseline](../unified-backend/baseline.md)
records scoped execution for DS-GAP-01/02 and recovered historical DPD evidence
for DS-GAP-03. All three gap tasks are now complete. The original documentation
checkpoint and its then-open states remain below; final backend acceptance is separate.

Reviewed on 2026-11-20 against MERLIN commit
`c91e2c86845c192aa5a6550fb77bbd232c5780e7`, branch
`merge-merlin-morgana`. This records documentation checks executed during backfill,
not a fresh run of historical scientific tests or final backend acceptance.

## Executed structural checks

The one-off Python standard-library audit used the Git baseline, canonical
Markdown, [task/evidence index](traceability.json) and the existing pinned
[integration inventory](../unified-backend/source-inventory.json). It checked
these independent invariants rather than inferring support from checkbox counts:

| Check | Observed result |
| --- | --- |
| Historical checkbox preservation | All **371 official** and **146 subordinate UI** checkbox lines, IDs, descriptions and states preserved, including nested criteria. Only four new backfill/gap tasks were added. |
| Downstream task coverage | **430 original top-level task occurrences** mapped to **322 stable contract identities**, retaining paired aliases and ambiguous original UI-DOC-02 labels at distinct targets. Anonymous UI refinements retain their existing parent/status owner. |
| Feature contract coverage | CH-EST-RX-01..06 individually retained; GUIDE-R1..R5, SKILL-US-R1..R4 and DOC-R1..R5 retain their exact original identities and linked defining paragraphs. |
| Complete identity coverage | **515 canonical requirement identities** and **two retained paired aliases** have unique resolving targets, including historical/pending contracts and upstream requirements. This count does not mean 515 implemented or accepted capabilities. |
| Original API requirement fidelity | **93 numbered clauses** match the ignored original byte-for-byte at the clause-text level, with original IDs and no invented missing numbers. Original file hash matches the index. Existing named API/repository requirements and prior canonical contract text also survive. |
| Architecture/task/module/evidence routing | Each registered contract links its architecture owner, current module, original task occurrence and maintained test or validation record. Task links return to the corresponding requirement. Planned upstream owners retain their explicit proposed status. |
| Historical evidence preservation | **628 tracked feature/evidence/code/test files** match their baseline hashes. The feature index and unified planning design received additive navigation only. Scientific source, tests, vectors and historical validation summaries are unchanged. |
| Upstream preservation | All **59 source requirements**, **103 source task mappings**, **46 source documents** and **52 source evidence-file hashes** retained. Full original requirement bodies survive. Source checkout remains clean at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`. |
| Dependency integrity | **27 integration/backfill/guidance nodes** have known prerequisites and no cycles; checklist dependencies match the integration inventory. The 21 integration tasks retain their states, with only INT-DOC-01 checked. New DS-REQ-01 closes documentation; DS-GAP-01..03 stay open. Previously accepted guidance/skills tasks remain checked. |
| Navigation | All maintained local Markdown file/fragment targets resolve across canonical docs, feature records and skills, excluding ignored private material and extracted specification corpora. Explicit requirement/task targets are unique. |
| Repository hygiene | `git diff --check` passed; generated Python integration guide check passed; generated archive/wheel paths remain ignored. HEAD and index unchanged; no commit or push executed. |

The audit compared full pre-existing document lines and all checkbox lines, not
just task-ID sets. Changes are additive except three strengthened integration
dependency declarations. All old canonical requirement text remains present.
File/fragment checks include current task-specific anchors, the later successful
Windows MSI section and the full retained API handoff. External historical URLs
remain citations; their remote reachability/results were not rechecked.

Read-only guide check:

```bash
python3 -B tools/python/update_integration_doc.py --check
git diff --check
```

Ignore-rule checks use `git check-ignore --stdin` with synthetic ZIP, tar,
compressed-tar and wheel paths; they create no files. Preserved evidence hashes
and baseline task-block hashes allow future audits to distinguish an old record
from a newly executed result. The index's checkmarks are explicitly frozen at
the reviewed baseline; they do not replace either current checklist.

## Evidence limits and open acceptance

The [three targeted gaps](design.md#targeted-evidence-gaps) cover eight existing
requirements: ARCH-02, FEC-01/02, QAM-01, RAND-01, CFG-01 and UI-DPD-VIEW-01/02.
Their historical completion marks are unchanged. The early records do not supply
fully attributable execution evidence for each exact accepted scope; maintained
tests alone are not reported as newly passing. Recover an existing valid result
or execute only the focused acceptance in DS-GAP-01..03, recording scope/results.
The synthetic-width requirement has its own located passing width/profile record
and does not inherit the earlier standalone numerology-record gap.

Earlier failed/pending native snapshots are linked to their later task-specific
results where available. This preserves the recorded Windows Server CI result
without asserting interactive Windows 11/UAC or current release verification.
Existing pending provisioning, hosted-matrix, installed-update, wider RF and
field-FEC tasks keep their original marks and owners. Superseded Designer
controls/example filenames route to later refinements; the old Python-after-UI
scheduling sentence is explicitly identified as superseded by implemented APIs.

No Rust/DSP/FEC simulation suite, native Python suite, Qt build/test, installer
lifecycle or hosted workflow was run for this documentation-only change. No
scientific behavior changed. DS-REQ-01 closes the requirements/traceability
baseline only; DS-GAP-01..03, INT-REG-01, INT-DOC-02 and INT-ACCEPT-01 still
require their actual MERLIN-workspace acceptance. Source-repository validation
remains historical evidence and cannot close integrated upstream requirements.
