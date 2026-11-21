# Backend master prompt preflight and traceability

Delivery: [INT-PLAN-01](../../checklist.md#int-plan-01). Design authority remains
[architecture](../../../architecture.md#requirements-and-historical-evidence-ownership),
behavior remains in [requirements](../../../requirements.md#backend-master-prompt-additions),
and completion remains in the [official checklist](../../checklist.md).
This record accepts planning coverage only. It does not implement the new prompt.

## Instruction source and current baseline

The [new master prompt](../../backend-integration-master-guide.md) is preserved
byte-for-byte, SHA-256 `3ccc608d2de2f8dc1221276ba25209e6664fd1d53cc8d7e9fbca8ffb9a778af5` (24,216 bytes), from the user attachment received
2026-11-20. The [earlier guide](../../ofdm-ofdma-integration-guide.md) remains
unchanged, including its pinned source revision and scientific constraints.
The new guide refines API direction separation, matrix and evidence obligations;
it does not erase established acceptance or authorize Qt/backend code in this
preflight. Both guide texts are instruction sources; canonical docs own the
reconciled design, obligations and status.

Inspected branch: `merge-merlin-morgana`, initially clean worktree/index at
`402106f0b85605e38dbc1aa755b3799e6939b024`. No reset/rebase/branch recreation,
commit/push/release, dependency fetch or source-repository modification is part
of this review. Historical source revision remains
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.

Completed gates retain their evidence: INT-DOC-01, DS-REQ-01, DS-GAP-01..03,
INT-BASE-01, INT-LICENSE-01, INT-REUSE-01, INT-FEC-01, DOC-ORG-01 and engineering
skill migration. See [baseline](baseline.md), [reuse/rights review](reuse-review.md),
[FEC acceptance](fec-validation.md) and [documentation acceptance](validation.md).
Do not rerun their historical development process to fill a new evidence format.
The next backend task is INT-DSP-01 after this preflight; no backend task is
executed by the review. Upstream PHY, PNM and API gates remain unchecked.

## Reconciliation decisions

The 515 existing canonical IDs, 59 inherited contracts, 93 numbered API clauses,
521 original checkbox lines, old aliases and frozen organization/source evidence
are preserved. Five additional stable contracts cover genuine new obligations:
[REQ-COM-API-07](../../../requirements.md#req-com-api-07),
[REQ-COM-API-08](../../../requirements.md#req-com-api-08),
[REQ-COM-API-09](../../../requirements.md#req-com-api-09),
[REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01) and
[REQ-COM-TRACE-02](../../../requirements.md#req-com-trace-02).
INT-PLAN-01 is a new documentation gate, separate from historical tasks.
INT-DSP-01 adds it as a prerequisite. Existing runtime dependencies stay ordered;
INT-RUST-01/INT-PY-01 gain explicit namespace/matrix acceptance, INT-TOOLS-01 gains
automation coverage, and INT-DOC-02/INT-ACCEPT-01 require full final evidence.

All TX-01..08, CH-01..06 and RX-01..09 definitions, order, endpoint/reference
planes and pinned normative sources remain in the architecture and active feature
owners. No normative document or source matrix was reinterpreted by this review.
Existing API names (`Ocd`, `Dpd`, `EvaluationOptions`, `BatchReport`, etc.) take
precedence over conceptual names in the new prompt until approved additive aliases
are implemented. Namespace design approval in §4.4/§19 is a real future export gate;
it is not required to save/reconcile this planning material or inspect common DSP.

## Complete prompt clause map

Each heading and its text through the next heading has one stable `MASTER-*`
anchor below. The supporting [source inventory](source-inventory.json) records
its exact original line, section hash, full requirement set and official tasks.
The validator checks all 37 sections, not just the nineteen numbered titles.
Each task links its architecture owner and requirements in both directions;
there is no separate source acceptance state here. Parent title/role/final
clauses retain the full objective and backend-only boundary.

| Prompt clause | Canonical requirements | Official delivery gates / architecture owners |
| --- | --- | --- |
| <a id="master-title"></a>**MASTER-TITLE** — MERLIN — Unified DOCSIS OFDM/OFDMA Backend Integration (source line 1) | [REQ-COM-NFR-04](../../../requirements.md#req-com-nfr-04), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-UI-01](../../../requirements.md#req-ds-ui-01), [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08) | [INT-ACCEPT-01](../../checklist.md#int-accept-01), [INT-REG-01](../../checklist.md#int-reg-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-subtitle"></a>**MASTER-SUBTITLE** — Master Codex Implementation Prompt (source line 3) | [REQ-COM-NFR-04](../../../requirements.md#req-com-nfr-04), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-UI-01](../../../requirements.md#req-ds-ui-01), [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08) | [INT-ACCEPT-01](../../checklist.md#int-accept-01), [INT-REG-01](../../checklist.md#int-reg-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-role"></a>**MASTER-ROLE** — Role (source line 5) | [REQ-COM-NFR-04](../../../requirements.md#req-com-nfr-04), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-UI-01](../../../requirements.md#req-ds-ui-01), [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08) | [INT-ACCEPT-01](../../checklist.md#int-accept-01), [INT-REG-01](../../checklist.md#int-reg-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-1"></a>**MASTER-1** — 1. Repository and Git requirements (source line 26) | [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01), [REQ-COM-NFR-02](../../../requirements.md#req-com-nfr-02), [REQ-DS-DELIVERY-01](../../../requirements.md#req-ds-delivery-01) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-BASE-01](../../checklist.md#int-base-01), [INT-LICENSE-01](../../checklist.md#int-license-01), [INT-DOC-02](../../checklist.md#int-doc-02); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [crate ownership](../../../architecture.md#unified-crate-ownership) |
| <a id="master-1-completed"></a>**MASTER-1-COMPLETED** — Existing completed work (source line 58) | [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01), [REQ-COM-NFR-02](../../../requirements.md#req-com-nfr-02), [REQ-DS-DELIVERY-01](../../../requirements.md#req-ds-delivery-01) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-BASE-01](../../checklist.md#int-base-01), [INT-LICENSE-01](../../checklist.md#int-license-01), [INT-DOC-02](../../checklist.md#int-doc-02); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [crate ownership](../../../architecture.md#unified-crate-ownership) |
| <a id="master-2"></a>**MASTER-2** — 2. Mandatory additive-first policy (source line 75) | [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01), [REQ-DS-PHY-01](../../../requirements.md#req-ds-phy-01), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-RUN-01](../../../requirements.md#req-ds-run-01) | [INT-BASE-01](../../checklist.md#int-base-01), [INT-DSP-01](../../checklist.md#int-dsp-01), [INT-REG-01](../../checklist.md#int-reg-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec) |
| <a id="master-3"></a>**MASTER-3** — 3. Scientific crate architecture (source line 100) | [REQ-COM-SYS-03](../../../requirements.md#req-com-sys-03), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01), [REQ-COM-NFR-02](../../../requirements.md#req-com-nfr-02) | [INT-REUSE-01](../../checklist.md#int-reuse-01), [INT-DSP-01](../../checklist.md#int-dsp-01), [INT-LICENSE-01](../../checklist.md#int-license-01); [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec), [crate ownership](../../../architecture.md#unified-crate-ownership) |
| <a id="master-4"></a>**MASTER-4** — 4. Public Rust API — mandatory direction separation (source line 153) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-4-rule"></a>**MASTER-4-RULE** — Fundamental API rule (source line 155) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-4-1"></a>**MASTER-4-1** — 4.1 Downstream API (source line 198) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-4-2"></a>**MASTER-4-2** — 4.2 Upstream API (source line 237) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-4-3"></a>**MASTER-4-3** — 4.3 Common API contracts (source line 276) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-4-4"></a>**MASTER-4-4** — 4.4 Cargo namespace implementation (source line 298) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-06](../../../requirements.md#req-com-api-06) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-5"></a>**MASTER-5** — 5. Rust API lifecycle (source line 320) | [REQ-US-API-01](../../../requirements.md#req-us-api-01), [REQ-COM-LIFE-01](../../../requirements.md#req-com-life-01), [REQ-COM-LIFE-02](../../../requirements.md#req-com-life-02), [REQ-COM-LIFE-03](../../../requirements.md#req-com-life-03), [REQ-COM-API-03](../../../requirements.md#req-com-api-03) | [INT-US-LOOP-01](../../checklist.md#int-us-loop-01), [INT-RUST-01](../../checklist.md#int-rust-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-6"></a>**MASTER-6** — 6. Python API — mirror Rust direction separation (source line 362) | [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-US-API-02](../../../requirements.md#req-us-api-02), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-04](../../../requirements.md#req-com-api-04), [REQ-COM-API-05](../../../requirements.md#req-com-api-05), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01) | [INT-PY-01](../../checklist.md#int-py-01), [INT-RUST-01](../../checklist.md#int-rust-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-6-1"></a>**MASTER-6-1** — 6.1 Python package architecture (source line 387) | [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-US-API-02](../../../requirements.md#req-us-api-02), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-04](../../../requirements.md#req-com-api-04), [REQ-COM-API-05](../../../requirements.md#req-com-api-05), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01) | [INT-PY-01](../../checklist.md#int-py-01), [INT-RUST-01](../../checklist.md#int-rust-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-6-2"></a>**MASTER-6-2** — 6.2 Python responsibilities (source line 405) | [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-US-API-02](../../../requirements.md#req-us-api-02), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-04](../../../requirements.md#req-com-api-04), [REQ-COM-API-05](../../../requirements.md#req-com-api-05), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01) | [INT-PY-01](../../checklist.md#int-py-01), [INT-RUST-01](../../checklist.md#int-rust-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-6-3"></a>**MASTER-6-3** — 6.3 Rust/Python parity (source line 421) | [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-US-API-02](../../../requirements.md#req-us-api-02), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-04](../../../requirements.md#req-com-api-04), [REQ-COM-API-05](../../../requirements.md#req-com-api-05), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01) | [INT-PY-01](../../checklist.md#int-py-01), [INT-RUST-01](../../checklist.md#int-rust-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-7"></a>**MASTER-7** — 7. Unified API compatibility matrix (source line 442) | [REQ-COM-API-09](../../../requirements.md#req-com-api-09) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-8"></a>**MASTER-8** — 8. Mandatory implementation traceability (source line 474) | [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-DOC-02](../../checklist.md#int-doc-02), [INT-ACCEPT-01](../../checklist.md#int-accept-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-9"></a>**MASTER-9** — 9. Automated traceability validation (source line 523) | [REQ-COM-TRACE-02](../../../requirements.md#req-com-trace-02), [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-TOOLS-01](../../checklist.md#int-tools-01), [INT-ACCEPT-01](../../checklist.md#int-accept-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [normative evidence and exclusions](../../../architecture.md#unified-normative-evidence-and-exclusions), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-10"></a>**MASTER-10** — 10. Shared DSP integration (source line 554) | [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01), [REQ-COM-SYS-03](../../../requirements.md#req-com-sys-03) | [INT-DSP-01](../../checklist.md#int-dsp-01), [INT-REUSE-01](../../checklist.md#int-reuse-01); [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec) |
| <a id="master-11"></a>**MASTER-11** — 11. FEC integration (source line 592) | [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-FEC-01](../../checklist.md#int-fec-01), [INT-REG-01](../../checklist.md#int-reg-01); [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-11-downstream"></a>**MASTER-11-DOWNSTREAM** — Downstream (source line 600) | [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-FEC-01](../../checklist.md#int-fec-01), [INT-REG-01](../../checklist.md#int-reg-01); [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-11-upstream"></a>**MASTER-11-UPSTREAM** — Upstream (source line 607) | [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-FEC-01](../../checklist.md#int-fec-01), [INT-REG-01](../../checklist.md#int-reg-01); [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-12"></a>**MASTER-12** — 12. Upstream OFDMA backend (source line 622) | [REQ-US-CFG-01](../../../requirements.md#req-us-cfg-01), [REQ-US-CFG-02](../../../requirements.md#req-us-cfg-02), [REQ-US-CFG-03](../../../requirements.md#req-us-cfg-03), [REQ-US-CFG-04](../../../requirements.md#req-us-cfg-04), [REQ-US-CFG-05](../../../requirements.md#req-us-cfg-05), [REQ-US-CFG-06](../../../requirements.md#req-us-cfg-06), [REQ-US-PRBS-01](../../../requirements.md#req-us-prbs-01), [REQ-US-PRBS-02](../../../requirements.md#req-us-prbs-02), [REQ-US-PRBS-03](../../../requirements.md#req-us-prbs-03), [REQ-US-PRBS-04](../../../requirements.md#req-us-prbs-04), [REQ-US-TX-01](../../../requirements.md#req-us-tx-01), [REQ-US-TX-03](../../../requirements.md#req-us-tx-03), [REQ-US-TX-04](../../../requirements.md#req-us-tx-04), [REQ-US-TX-05](../../../requirements.md#req-us-tx-05), [REQ-US-TX-06](../../../requirements.md#req-us-tx-06), [REQ-US-TX-07](../../../requirements.md#req-us-tx-07), [REQ-US-TX-08](../../../requirements.md#req-us-tx-08), [REQ-US-CH-01](../../../requirements.md#req-us-ch-01), [REQ-US-CH-02](../../../requirements.md#req-us-ch-02), [REQ-US-CH-03](../../../requirements.md#req-us-ch-03), [REQ-US-CH-04](../../../requirements.md#req-us-ch-04), [REQ-US-CH-05](../../../requirements.md#req-us-ch-05), [REQ-US-CH-06](../../../requirements.md#req-us-ch-06), [REQ-US-RX-01](../../../requirements.md#req-us-rx-01), [REQ-US-RX-02](../../../requirements.md#req-us-rx-02), [REQ-US-RX-03](../../../requirements.md#req-us-rx-03), [REQ-US-RX-04](../../../requirements.md#req-us-rx-04), [REQ-US-RX-05](../../../requirements.md#req-us-rx-05), [REQ-US-RX-06](../../../requirements.md#req-us-rx-06), [REQ-US-RX-07](../../../requirements.md#req-us-rx-07), [REQ-US-RX-09](../../../requirements.md#req-us-rx-09), [REQ-US-SYS-01](../../../requirements.md#req-us-sys-01), [REQ-US-SYS-02](../../../requirements.md#req-us-sys-02), [REQ-US-SYS-04](../../../requirements.md#req-us-sys-04), [REQ-COM-OBS-01](../../../requirements.md#req-com-obs-01), [REQ-COM-OBS-02](../../../requirements.md#req-com-obs-02), [REQ-COM-LIFE-01](../../../requirements.md#req-com-life-01), [REQ-COM-LIFE-02](../../../requirements.md#req-com-life-02), [REQ-COM-LIFE-03](../../../requirements.md#req-com-life-03), [REQ-COM-NFR-01](../../../requirements.md#req-com-nfr-01), [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05), [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01), [REQ-US-OBS-03](../../../requirements.md#req-us-obs-03), [REQ-US-OBS-04](../../../requirements.md#req-us-obs-04) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01), [INT-US-TX-01](../../checklist.md#int-us-tx-01), [INT-US-CH-01](../../checklist.md#int-us-ch-01), [INT-US-RX-01](../../checklist.md#int-us-rx-01), [INT-US-LOOP-01](../../checklist.md#int-us-loop-01), [INT-FEC-01](../../checklist.md#int-fec-01), [INT-DEFER-01](../../checklist.md#int-defer-01); [upstream configuration and source](../../../architecture.md#unified-upstream-configuration-and-source), [upstream processing order](../../../architecture.md#unified-upstream-processing-order), [channel and measurement reference planes](../../../architecture.md#unified-channel-and-measurement-reference-planes), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis), [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec), [normative evidence and exclusions](../../../architecture.md#unified-normative-evidence-and-exclusions) |
| <a id="master-13"></a>**MASTER-13** — 13. PNM integration (source line 654) | [REQ-US-PNM-01](../../../requirements.md#req-us-pnm-01), [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02), [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03), [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04), [REQ-DS-INPUT-01](../../../requirements.md#req-ds-input-01), [REQ-COM-API-05](../../../requirements.md#req-com-api-05) | [INT-PNM-01](../../checklist.md#int-pnm-01), [INT-PNM-02](../../checklist.md#int-pnm-02), [INT-REG-01](../../checklist.md#int-reg-01), [INT-PY-01](../../checklist.md#int-py-01); [pnm and capture admission](../../../architecture.md#unified-pnm-and-capture-admission), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-14"></a>**MASTER-14** — 14. Unified CLI (source line 689) | [REQ-COM-IDENTITY-01](../../../requirements.md#req-com-identity-01), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01) | [INT-CLI-01](../../checklist.md#int-cli-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-15"></a>**MASTER-15** — 15. Backend validation requirements (source line 705) | [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-COM-NFR-04](../../../requirements.md#req-com-nfr-04), [REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01) | [INT-REG-01](../../checklist.md#int-reg-01), [INT-ACCEPT-01](../../checklist.md#int-accept-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-15-downstream"></a>**MASTER-15-DOWNSTREAM** — Downstream (source line 709) | [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-DS-PHY-01](../../../requirements.md#req-ds-phy-01), [REQ-DS-RUN-01](../../../requirements.md#req-ds-run-01), [REQ-DS-INPUT-01](../../../requirements.md#req-ds-input-01), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-UI-01](../../../requirements.md#req-ds-ui-01) | [INT-REG-01](../../checklist.md#int-reg-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-15-upstream"></a>**MASTER-15-UPSTREAM** — Upstream (source line 725) | [REQ-US-CFG-01](../../../requirements.md#req-us-cfg-01), [REQ-US-CFG-02](../../../requirements.md#req-us-cfg-02), [REQ-US-CFG-03](../../../requirements.md#req-us-cfg-03), [REQ-US-CFG-04](../../../requirements.md#req-us-cfg-04), [REQ-US-CFG-05](../../../requirements.md#req-us-cfg-05), [REQ-US-CFG-06](../../../requirements.md#req-us-cfg-06), [REQ-US-PRBS-01](../../../requirements.md#req-us-prbs-01), [REQ-US-PRBS-02](../../../requirements.md#req-us-prbs-02), [REQ-US-PRBS-03](../../../requirements.md#req-us-prbs-03), [REQ-US-PRBS-04](../../../requirements.md#req-us-prbs-04), [REQ-US-TX-01](../../../requirements.md#req-us-tx-01), [REQ-US-TX-03](../../../requirements.md#req-us-tx-03), [REQ-US-TX-04](../../../requirements.md#req-us-tx-04), [REQ-US-TX-05](../../../requirements.md#req-us-tx-05), [REQ-US-TX-06](../../../requirements.md#req-us-tx-06), [REQ-US-TX-07](../../../requirements.md#req-us-tx-07), [REQ-US-TX-08](../../../requirements.md#req-us-tx-08), [REQ-US-CH-01](../../../requirements.md#req-us-ch-01), [REQ-US-CH-02](../../../requirements.md#req-us-ch-02), [REQ-US-CH-03](../../../requirements.md#req-us-ch-03), [REQ-US-CH-04](../../../requirements.md#req-us-ch-04), [REQ-US-CH-05](../../../requirements.md#req-us-ch-05), [REQ-US-CH-06](../../../requirements.md#req-us-ch-06), [REQ-US-RX-01](../../../requirements.md#req-us-rx-01), [REQ-US-RX-02](../../../requirements.md#req-us-rx-02), [REQ-US-RX-03](../../../requirements.md#req-us-rx-03), [REQ-US-RX-04](../../../requirements.md#req-us-rx-04), [REQ-US-RX-05](../../../requirements.md#req-us-rx-05), [REQ-US-RX-06](../../../requirements.md#req-us-rx-06), [REQ-US-RX-07](../../../requirements.md#req-us-rx-07), [REQ-US-RX-09](../../../requirements.md#req-us-rx-09), [REQ-US-SYS-01](../../../requirements.md#req-us-sys-01), [REQ-US-SYS-02](../../../requirements.md#req-us-sys-02), [REQ-US-SYS-04](../../../requirements.md#req-us-sys-04), [REQ-COM-OBS-01](../../../requirements.md#req-com-obs-01), [REQ-COM-OBS-02](../../../requirements.md#req-com-obs-02), [REQ-COM-LIFE-01](../../../requirements.md#req-com-life-01), [REQ-COM-LIFE-02](../../../requirements.md#req-com-life-02), [REQ-COM-LIFE-03](../../../requirements.md#req-com-life-03), [REQ-COM-NFR-01](../../../requirements.md#req-com-nfr-01), [REQ-US-PNM-01](../../../requirements.md#req-us-pnm-01), [REQ-US-PNM-02](../../../requirements.md#req-us-pnm-02), [REQ-US-PNM-03](../../../requirements.md#req-us-pnm-03), [REQ-US-PNM-04](../../../requirements.md#req-us-pnm-04), [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05), [REQ-US-TX-02](../../../requirements.md#req-us-tx-02), [REQ-US-RX-08](../../../requirements.md#req-us-rx-08), [REQ-DS-FEC-01](../../../requirements.md#req-ds-fec-01), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-US-CONFIG-01](../../checklist.md#int-us-config-01), [INT-US-TX-01](../../checklist.md#int-us-tx-01), [INT-US-CH-01](../../checklist.md#int-us-ch-01), [INT-US-RX-01](../../checklist.md#int-us-rx-01), [INT-US-LOOP-01](../../checklist.md#int-us-loop-01), [INT-PNM-02](../../checklist.md#int-pnm-02), [INT-FEC-01](../../checklist.md#int-fec-01); [upstream configuration and source](../../../architecture.md#unified-upstream-configuration-and-source), [upstream processing order](../../../architecture.md#unified-upstream-processing-order), [channel and measurement reference planes](../../../architecture.md#unified-channel-and-measurement-reference-planes), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis), [pnm and capture admission](../../../architecture.md#unified-pnm-and-capture-admission), [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec) |
| <a id="master-15-api"></a>**MASTER-15-API** — API (source line 743) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-COM-API-03](../../../requirements.md#req-com-api-03), [REQ-COM-API-04](../../../requirements.md#req-com-api-04), [REQ-COM-API-05](../../../requirements.md#req-com-api-05), [REQ-COM-API-06](../../../requirements.md#req-com-api-06), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-DELIVERY-01](../../../requirements.md#req-ds-delivery-01) | [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01), [INT-REG-01](../../checklist.md#int-reg-01), [INT-TOOLS-01](../../checklist.md#int-tools-01); [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [normative evidence and exclusions](../../../architecture.md#unified-normative-evidence-and-exclusions) |
| <a id="master-16"></a>**MASTER-16** — 16. Backend-only completion boundary (source line 761) | [REQ-COM-NFR-04](../../../requirements.md#req-com-nfr-04), [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-DS-UI-01](../../../requirements.md#req-ds-ui-01), [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08) | [INT-ACCEPT-01](../../checklist.md#int-accept-01), [INT-REG-01](../../checklist.md#int-reg-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-17"></a>**MASTER-17** — 17. Documentation updates (source line 788) | [REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01), [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-COM-API-09](../../../requirements.md#req-com-api-09) | [INT-DOC-02](../../checklist.md#int-doc-02); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-18"></a>**MASTER-18** — 18. Required workflow (source line 816) | [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01), [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-DOC-01](../../../requirements.md#req-com-doc-01), [REQ-COM-REUSE-01](../../../requirements.md#req-com-reuse-01) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-DSP-01](../../checklist.md#int-dsp-01), [INT-US-CONFIG-01](../../checklist.md#int-us-config-01), [INT-US-LOOP-01](../../checklist.md#int-us-loop-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01), [INT-DOC-02](../../checklist.md#int-doc-02); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [shared dsp and fec](../../../architecture.md#unified-shared-dsp-and-fec), [upstream configuration and source](../../../architecture.md#unified-upstream-configuration-and-source), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis), [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate) |
| <a id="master-19"></a>**MASTER-19** — 19. Required initial Codex response (source line 845) | [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08), [REQ-COM-API-09](../../../requirements.md#req-com-api-09), [REQ-COM-TRACE-01](../../../requirements.md#req-com-trace-01) | [INT-PLAN-01](../../checklist.md#int-plan-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [requirements and historical evidence ownership](../../../architecture.md#requirements-and-historical-evidence-ownership), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |
| <a id="master-final"></a>**MASTER-FINAL** — Final objective (source line 871) | [REQ-COM-NFR-04](../../../requirements.md#req-com-nfr-04), [REQ-DS-API-01](../../../requirements.md#req-ds-api-01), [REQ-DS-UI-01](../../../requirements.md#req-ds-ui-01), [REQ-COM-API-07](../../../requirements.md#req-com-api-07), [REQ-COM-API-08](../../../requirements.md#req-com-api-08) | [INT-ACCEPT-01](../../checklist.md#int-accept-01), [INT-REG-01](../../checklist.md#int-reg-01), [INT-RUST-01](../../checklist.md#int-rust-01), [INT-PY-01](../../checklist.md#int-py-01); [acceptance and final documentation gate](../../../architecture.md#unified-acceptance-and-final-documentation-gate), [rust python and cli apis](../../../architecture.md#unified-rust-python-and-cli-apis) |

## Required initial API review package

Master §19's thirteen items are routed as follows; unresolved implementation
choices are visible rather than assigned invented callable signatures.

| Initial review item | Concrete owner / current finding |
| --- | --- |
| 1–2 Existing downstream Rust/Python inventories | [Existing API inventory](../high-level-api/compatibility-matrix.md#existing-inventory-and-ownership), verified source paths and 25 exact Rust session rows with native mappings. Remaining domain/export coverage is explicitly open in the same matrix. |
| 3 Implemented upstream inventory | [FEC acceptance](fec-validation.md): three upstream LDPC families and private sparse mechanism only. `crates/ofdma`, upstream sessions and native wrappers do not exist. |
| 4–6 Rust/Python trees and exact crate mapping | [Namespace proposal](../high-level-api/compatibility-matrix.md#namespace-proposal-for-review): preserve current crate/imports; proposed direction crate aliases; package/submodule alternatives and common nominal ownership remain approval decisions. No existing package is silently renamed. |
| 7 Backward compatibility | REQ-DS-API-01 and REQ-COM-API-07/08; same downstream types/engine, legacy native aliases and saved JSON/recipes; add aliases only after review. |
| 8 Lifecycle/errors/reports | [API contracts](../high-level-api/upstream.md#request-lifecycle-and-result-contracts): Configure → Validate → Prepare → Run → Report is conceptual; retain actual methods, idle load/prepare, immutable accepted generation, explicit unsupported controls and real decoder/partial outcomes. |
| 9 Operation/parity matrix | [Single matrix](../high-level-api/compatibility-matrix.md); current seed and named coverage gaps. Full export/native coverage is an acceptance requirement, not claimed complete today. |
| 10 Implementation traceability | [Fourteen-field record contract](#implementation-evidence-contract), existing feature records/indexes, no new status tracker. |
| 11 Requirement/task updates | Five new contracts above; INT-PLAN-01 documentation gate; focused additions to existing open API/tool/final gates. Original historical marks remain. |
| 12 Exact Add/Edit/Reuse/Remove map | [File dispositions](#file-dispositions); no deletions or source moves. API implementation files are planned, not part of this review. |
| 13 Regression and acceptance | [Validation gates](#regression-and-acceptance-plan), full backend acceptance after relevant native Rust/Python/CLI/PNM checks; no Qt upstream prerequisite. |

This is the preflight package, not namespace approval or a claim that all thirteen
future export decisions are finalized. Resolve the common Rust placement and
Python package choice and complete every operation row at INT-RUST-01/INT-PY-01
before final acceptance; obtain mapping approval before compatibility-sensitive
exports even if they arise earlier with upstream stages.

## Implementation evidence contract

Use existing feature design/validation records and the source/backfill indexes.
For each applicable backend requirement, record the following fourteen named
fields directly or by an exact unambiguous reference to those records:

| Field | Required content / planned versus accepted treatment |
| --- | --- |
| Stable ID | One canonical requirement ID; old aliases remain aliases. |
| Architecture owner | Exact canonical section anchor. |
| Feature design | Existing active owner and exact acceptance subsection. |
| Checklist task | Stable official task ID; its state remains authoritative. |
| Crate | Actual Cargo package, or explicitly proposed package before creation. |
| Source file | Exact repository-relative file; planned paths are labeled. |
| Symbol or stage | Exported Rust/Python symbol or canonical scientific stage; unresolved names are gaps. |
| Test file | Exact test location; planned locations are labeled. |
| Specific test case | Qualified independently checked case name; a whole test directory is insufficient. |
| Validation command | Exact reproducible command, feature/lock/target/options and environment as applicable. |
| Observed result | Actual result/count/failure/omission; `not run` for planned work. |
| Commit SHA | Actual tested revision, plus working diff/artifact identity if uncommitted; never substitute a later commit for the tested candidate. |
| Status | Planned; implemented-unvalidated; integrated-validated; failed/blocked; deferred; historical-source. An implementation record cannot overrule the task's acceptance. |
| Limitations | Scope, endpoints/modes/units, platform, decoder/source independence and missing evidence. |

A planned record must still name its proposed owner/acceptance location and the
missing exact symbols/cases as a prerequisite to code changes; it must say `not
run` and cannot carry an integrated pass. An accepted new implementation requires
all actual paths/symbols/cases and executed evidence. Aggregate historical
acceptance can retain a valid historical suite record without inventing per-case
logs or replaying the full process. Add a targeted gap if that record cannot
support a particular claim. Old source validation is historical-source only.

An existing accepted chain illustrates the evidence precision without inventing
a new execution:

| Field | REQ-COM-SYS-05 example |
| --- | --- |
| ID / architecture | [REQ-COM-SYS-05](../../../requirements.md#req-com-sys-05); [normative ownership](../../../architecture.md#unified-normative-evidence-and-exclusions). |
| Feature / task | [FEC design](../fec/design.md); [INT-FEC-01](../../checklist.md#int-fec-01), accepted standalone foundation. |
| Crate / source / symbols | `fec-codec`; `crates/fec-codec/src/upstream_ldpc/definitions.rs`; `upstream_ldpc` three directional normative definitions. |
| Test file / cases | `crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs`; `upstream_ldpc::tests::source_audit::{long,medium,short}::complete_source_matrix_and_expansion` and `exact_source_derived_impulses`, expanded by `source_cases!` for all three families. |
| Command / observed result | `scripts/cargo-local.sh test -p fec-codec --locked`: 74 passed, as recorded in [executed checks](fec-validation.md#executed-checks). |
| Candidate / commit precision | Tested working candidate based on `9007feaf231f8f1b331baf6f7559efce89516f54`, subsequently committed in `343b07d9209dc10f3c21d80ae2222a60432f684a`. The original record has no complete working-diff fingerprint; do not claim the later commit was itself rerun. |
| Status / limitations | Integrated-validated standalone codec boundary with the recorded candidate precision; original-page transcription limits retained. Upstream waveform/session acceptance and final integration regression remain open. |

The independent `every_source_cell_and_lifted_edge` and
`frozen_impulse_parity_and_recovery` helpers serve those macro-generated cases.
Historical precision is preserved; future implementation records identify the
tested working diff/artifact as well as its base SHA.

## Automated validation gaps

[REQ-COM-TRACE-02](../../../requirements.md#req-com-trace-02) is delivered by
[INT-TOOLS-01](../../checklist.md#int-tools-01), with final coverage at INT-ACCEPT-01.
This table describes current versus remaining checks, not a parallel task list.

| Required category (master §9) | Existing/preflight coverage | Remaining acceptance |
| --- | --- | --- |
| Missing implementation owners; invalid/stale paths | Functional record owner states, 127 backfill routes, specific FEC owner guard; planned paths distinguished | Exact file coverage for each newly implemented requirement/symbol, not only directory owners. |
| Missing public API symbols | Source inventory and explicit current/proposed API rows | Parse/resolve every mapped Rust export and inspect actual native Python exports, qualified signatures and module imports; reject removed/misnamed symbols. |
| Missing acceptance cases | Existing declared test files, exact seed matrix cases | Verify exact test case discovery for every new backend requirement and every exported operation; no path-only passes. |
| Missing executed evidence | Linked historical baseline/FEC records and truthful `not run` planning states | Machine-check required command/result/candidate/limitations fields against actual acceptance records for all newly accepted work. |
| Requirement/task/architecture links | Existing full bidirectional task guard, current 37-clause prompt map, five additive IDs | Keep checks current after each integration step. |
| Duplicate requirement/task IDs and invalid aliases | Existing canonical identity, explicit anchor, task label and historical alias guards | Extend identity inventory for each new requirement; verify no undeclared definitions. |
| Unmatched Rust/Python exports | Existing actual-native reference inventory tool and seed matrix with intentional Rust-only `prepare` | Full operation coverage/signature/status validation and explicit justified native-adapter-only or Rust-only distinctions. No blanket name equality. |
| Incorrect documentation completion/status | Official unchecked gates; preflight checks against canonical links and task state | Reject integrated-validated records lacking required executed cases or prematurely completed final/API tasks; preserve accepted legacy history. |

A successful structural audit does not claim that the remaining semantic/native
checks above are implemented. Extend the current tooling as stages land; keep
INT-TOOLS-01, INT-DOC-02 and INT-ACCEPT-01 open until the full contract passes.

## File dispositions

| Disposition | Exact scope |
| --- | --- |
| Add (this preflight) | `docs/development/backend-integration-master-guide.md` (verbatim), this review, `docs/development/features/high-level-api/compatibility-matrix.md`, `tools/validation/test_master_prompt_traceability.py`. |
| Edit (this preflight) | `docs/architecture.md`, `docs/requirements.md`, `docs/development/checklist.md`, `docs/development/README.md`, `docs/development/features/high-level-api/upstream.md`, `docs/development/features/unified-backend/source-inventory.json`, `docs/development/features/README.md`; existing validation tools. |
| Reuse | All original downstream/source requirements, stage definitions, historical acceptance, source fixtures, licenses, canonical skills and frozen DOC-ORG-01 artifacts. |
| Add/Edit (planned backend, outside this preflight) | Existing `crates/ofdm/src/lib.rs` for reviewed downstream alias; planned `crates/ofdma/{Cargo.toml,src/lib.rs,src/api/session.rs,src/api/error.rs,src/api/reports.rs,tests/api_parity.rs}`; existing `bindings/python/{Cargo.toml,pyproject.toml,src/lib.rs,src/controller.rs}`, proposed `bindings/python/src/upstream/{configuration,session,reports}.rs`, planned `bindings/python/tests/test_upstream_parity.py`. Python package files and common leaf locations await the namespace/package decision before coding. |
| Remove | None. No original technical records or fixtures are deleted or moved. |

Other numerical source/test owners remain in the active
[DSP](../signal-processing/design.md), [FEC](../fec/design.md),
[configuration](../ofdma/configuration.md), [TX](../ofdma/transmitter.md),
[channel](../ofdma/channel.md), [RX](../ofdma/receiver.md),
[Pre-Eq](../ofdma/pre-equalization.md) and [PNM](../pnm-measurement-inputs/upstream.md)
records. Do not infer authorization to change these files from this review table.

## Regression and acceptance plan

Before each numerical edit, identify actual symbols, consumers, requirement IDs,
independent vectors and affected regressions; preserve a reproducible before/after
baseline. INT-DSP-01 compares all ten kernel candidates, numerical conventions,
SHARE/REUSE/ADAPT/SEPARATE/DEFER decisions and hot-path measurements; a DSP crate
requires actual consumers/equivalence evidence. Keep the already accepted FEC
matrices and frozen oracles unchanged.

Then follow existing typed config → eight TX stages → channel → nine received-only
RX stages → independently compared source loop → verified PNM association → final
Rust API → native Python/CLI → tools → full regression → documentation → backend
acceptance dependencies. Initial Rust stage APIs accompany the stages; no early
FEC-only Python facade. Both numerologies, declared pilots/QAM/code families,
C/H/effective-estimate separation, controls/determinism and honest failure/partial
results retain their explicit stage acceptance. Only run the relevant new and
regression suites as required; never change golden vectors to obtain passes.

Final relevant validation includes the repository Rust fmt/workspace/Clippy/
analysis-only/doctest/example gates, actual native Python parity/import/lifetime/
Busy/GIL/control tests, wheel/installed checks within declared platforms, and
independent CLI/PNM regression. Report unexecuted installed/hosted scopes plainly.
No upstream Qt implementation or tab is required for backend acceptance; existing
Qt integration checks apply only when shared contracts/dependencies are affected.
No setup, system installation or publishing is authorized.

## Preflight verification

Executed in the documentation/validation-tool working candidate based on
`402106f0b85605e38dbc1aa755b3799e6939b024`, 2026-11-20. No commit was created.
The exact scope is the file-disposition table and current Git diff, including the
verbatim guide hash above. This is structural preflight evidence only.

| Command / check | Observed result |
| --- | --- |
| `python3 -B tools/validation/integration_traceability.py` | Pass: all 22 integration task/requirement/architecture round trips, 37 hashed prompt clauses and ten exact seed-matrix test declarations resolve. |
| `python3 -B tools/validation/feature_documentation.py` | Pass: 520 canonical IDs (515 retained + five new), 8,233 local links, 127 downstream routes, 430 original task mappings, 29 acyclic dependency nodes, 12 skills / 24 discovery links. |
| `python3 -B -m unittest discover -s tools/validation -p 'test_*.py'` | 35 passed, including ten new independent positive/negative prompt, delivery and exact-case discovery cases. These run no scientific or native extension tests. |
| `python3 -B tools/python/update_integration_doc.py --check` | Generated Python integration guide current. |
| `git diff --check` | Pass. |
| Frozen historical snapshot comparison | All 521 historical checkbox lines/states preserved; DOC-ORG-01 remains completed; all 163 Keep/Archive feature files match their original SHA-256. |
| Original inventory/prompt and scope comparison | All 59 inherited contract records, frozen DOC-ORG-01 metadata and earlier guide unchanged. New guide exactly matches all 24,216 attachment bytes. Only docs and validation tools changed; Git index untouched. |

INT-PLAN-01 accepts these planning/structural deliverables. This review runs no
new Rust scientific suite, Python extension test, wheel build or Qt test;
prior executed evidence retains its original scope. API namespace approval,
full matrix/export coverage, automated semantic/evidence coverage and all unchecked
runtime gates remain outstanding. No backend migration is started by this review.
