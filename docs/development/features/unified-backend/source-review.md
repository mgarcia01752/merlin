# Source documentation and evidence review

MERLIN baseline: `fa26de619b5d28a442c42e46ad1ed19262a9d08a`; source baseline: `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
Reviewed 2026-11-20. Source checkout remains read-only. All 46 tracked source
Markdown documents and all 30 upstream feature design/validation/coverage records
are inventoried below, alongside 103 source checklist IDs and 59 requirement
mappings in the [machine-readable inventory](source-inventory.json).
The inventory also fingerprints 148 existing MERLIN architecture/requirements/
checklist/feature records and 52 source test/vector/fixture files.

## Interpretation and preservation

**Merge** carries behavior/decisions into an existing canonical MERLIN owner.
**Adapt** changes ownership, identifiers, paths or scope for the unified design.
**Retain** preserves a selected historical technical reference locally, clearly
labeled source-only. **Exclude** rejects redundant/obsolete repository-specific
instructions from active MERLIN guidance; it does not delete the original.

Original source documentation, vectors and captures are not deleted or modified.
All original source documents have exact commit URLs and SHA-256 fingerprints.
The complete 23 detailed stage definitions plus PRBS/Pre-Eq/RxMER are retained in
[scientific stage contracts](references/upstream-stage-contracts.md), keeping their
equations, state, budgets, source tables and limitations. The canonical architecture
selects local owners and explicit adaptations; this historical appendix supplies
technical detail and is not another current architecture or acceptance checklist.

The 11 selected retained records preserve independent LDPC audit outcomes,
validation chronology, the complete 108-row configuration coverage matrix and
four deferred extension designs. Links in retained source text point to pinned
source records; absent private/generated targets are plain provenance paths,
not broken local links or a promise that a fresh clone contains them.
Source product identifiers, commands and artifact names appear only as historical
technical/legal provenance in these development references. No user guide or
runtime product identity is renamed or populated with obsolete branding.

## Per-document decisions

| Source document at exact revision | Decision | MERLIN destination / reason |
| --- | --- | --- |
| [docs/architecture.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/architecture.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/README.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/README.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/build.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/build.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/development/checklist.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/checklist.md) | **Adapt** | [Local owner](../../checklist.md#unified-backend-integration) — Every source task maps to a local gate/deferred decision; source checkmarks do not transfer. |
| [docs/development/features/backend-delivery/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/backend-delivery/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/backend-stages/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/backend-stages/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/backend-stages/validation.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/backend-stages/validation.md) | **Retain** | [Local owner](references/backend-stages-validation.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/capture-formats/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/capture-formats/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/capture-formats/validation.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/capture-formats/validation.md) | **Retain** | [Local owner](references/capture-formats-validation.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/channel-simulator/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/channel-simulator/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/cm-us-tx/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/cm-us-tx/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/cmts-us-rx/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/cmts-us-rx/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/code-layout/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/code-layout/design.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/development/features/development-setup/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/development-setup/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/extension-designs/channel.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/channel.md) | **Retain** | [Local owner](references/extension-designs-channel.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/extension-designs/ranging.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/ranging.md) | **Retain** | [Local owner](references/extension-designs-ranging.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/extension-designs/receiver.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/receiver.md) | **Retain** | [Local owner](references/extension-designs-receiver.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/extension-designs/ui.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/ui.md) | **Retain** | [Local owner](references/extension-designs-ui.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/foundation/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/foundation/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/foundation/validation.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/foundation/validation.md) | **Retain** | [Local owner](references/foundation-validation.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/ldpc-source-audit/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/design.md) | **Merge** | [Local owner](../../../architecture.md#unified-shared-dsp-and-fec) — Preserve independent audit/parity/shortening decisions and existing full validation reference; acceptance must be rerun in fec-codec. |
| [docs/development/features/ldpc-source-audit/validation.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/validation.md) | **Retain** | [Local owner](references/ldpc-source-audit-validation.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/prbs-source/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/prbs-source/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/public-api/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/public-api/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-rust-python-and-cli-apis) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/remaining-delivery/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/remaining-delivery/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-rust-python-and-cli-apis) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/remaining-delivery/validation.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/remaining-delivery/validation.md) | **Retain** | [Local owner](references/remaining-delivery-validation.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/requirements/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/requirements/design.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/development/features/rxmer-capture/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/rxmer-capture/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/rxmer-capture/validation.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/rxmer-capture/validation.md) | **Retain** | [Local owner](references/rxmer-capture-validation.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/software-development-checklist/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/software-development-checklist/design.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/development/features/upstream-inputs/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/upstream-inputs/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/upstream-signal-path/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/upstream-signal-path/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/features/us-configuration-json/coverage.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/us-configuration-json/coverage.md) | **Retain** | [Local owner](references/us-configuration-json-coverage.md) — Preserve full historical source evidence, field coverage or deferred design, explicitly without MERLIN runtime acceptance. |
| [docs/development/features/us-configuration-json/design.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/us-configuration-json/design.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/development/foundation-adoption.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/foundation-adoption.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/development/setup.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/setup.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/git.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/git.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/reference/distribution.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/reference/distribution.md) | **Adapt** | [Local owner](../../../architecture.md#unified-backend-integration) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/reference/specifications.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/reference/specifications.md) | **Merge** | [Local owner](source-review.md#normative-provenance) — Preserve edition/hash/page claims with evidence level; no private corpus or unperformed PDF verification. |
| [docs/requirements.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/requirements.md) | **Merge** | [Local owner](../../../requirements.md) — All 59 behavior/acceptance contracts mapped uniquely; source aliases preserved; explicit local policy adaptations recorded. |
| [docs/sw-facts/README.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/sw-facts/README.md) | **Exclude** | [Local owner](../../../../skills/development-workflow/SKILL.md) — Redundant source repository workflow/paths/status; retain MERLIN-specific PDE, setup, Git, cache and delivery policy. Original remains pinned and is not deleted. |
| [docs/user/README.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/README.md) | **Adapt** | [Local owner](../../../architecture.md#unified-rust-python-and-cli-apis) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/user/capture-inspection.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/capture-inspection.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/user/measured-experiments.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/measured-experiments.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/user/rxmer-captures.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/rxmer-captures.md) | **Adapt** | [Local owner](../../../architecture.md#unified-pnm-and-capture-admission) — Adapt technical ownership and scope into canonical MERLIN integration records. |
| [docs/user/upstream-experiments.md](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/upstream-experiments.md) | **Adapt** | [Local owner](../../../architecture.md#unified-rust-python-and-cli-apis) — Adapt technical ownership and scope into canonical MERLIN integration records. |

## Normative provenance

These are **preserved source-review claims**, not new PDF inspections in this task.
Private PDFs/extractions/page images remain unshipped. The controlling edition and
endpoint must be checked before algorithm/format implementation; a hash identifies
an input, not independently verified scientific correctness.

| Authority | Preserved source scope | Original PDF SHA-256 |
| --- | --- | --- |
| CM-SP-PHYv3.1-I20-230419 | §7.4 and §9.4.8; Table8 p52, matrices/right shifts pp56–57, shortening p58; transform/prefix/window; Pre-Eq §7.4.17 pp106–108. | `304c26937f5beaf7ffddc88158abacead4ead2bc29e6acaae5226898c9f716bc` |
| CM-SP-PHYv4.0-I07-250627 | §7.2.4/§7.3.4 upstream delegation; original source reports extraction review, not a blanket rendered-PDF check. | Retain edition metadata; do not invent a missing original-PDF hash. |
| CM-SP-MULPIv3.1-I25-230419 | UCD Type51/version5 envelope and multipart, Tables28/29, represented MAP/P-MAP/IUC and 108-field coverage. | `fe9daf3d2bcee2a84b34cfcfed1094841efb04baa6136d1fe44d776a9a8afc1a` |
| CM-SP-CCAP-OSSIv3.1-I18-200610 | Upstream CCAP RxMER, §7.3.2.5, Tables397/410; source pp396/397/419 and context418–420/608. | `a74611960b377d8f23e9d9929af6c272c671164eba858a087432e7ce66c0d0aa` |
| CM-SP-CM-OSSIv3.1-I27-250219 | AnnexD.2.13 Tables77/78 pp187–192; applied-bank s2.13 versus last-update s1.14, with external operation/base/probe/status context. | `29f0ebf32c4c38610b1b9c10a418ffff4045c34def0ffbe4e85fc674cb5b91de` |

## Independent numerical evidence

Retain the [full LDPC audit](references/ldpc-source-audit-validation.md).
It records all490 independently transcribed matrix cells, 15 base-row and three
lifted-edge fingerprints, 20 frozen impulse parity supports, long/medium
transcription corrections and unchanged short definitions. Fingerprints detect
preservation/transcription drift; they are neither cryptographic proofs nor
external interoperability vectors. The Gaussian arithmetic oracle shares production
matrix data and cannot replace the independent page-derived fixtures.

The [inventory](source-inventory.json) retains exact source links/hashes for
`crates/ldpc/src/tests/{source_audit,source_fixtures,oracle}.rs`, the independent
Rust source-reference generator and the stage/API/native test and measurement
fixtures. Their originals stay in the untouched pinned source until validated
local replacements exist. No expected values may be regenerated from production
constants to conceal changes; record the cause, independent oracle and numerical
difference before accepting a deliberate fixture correction.

## Status chronology and exclusions

Source architecture/feature records mix historical design-time unchecked lists
with later synthetic, format-inspection and measured-delivery results. Preserve
that chronology rather than flattening it into one accepted-support claim. The
[remaining-delivery evidence](references/remaining-delivery-validation.md) is the
later source report; its 109 Rust/17 native tests, local packages and setup guests
remain source evidence only. They have not run in unified MERLIN.

Ordinary scheduled-data source support is not blind acquisition, live ranging,
full MAC/scheduling, external device interoperability or arbitrary vendor PNM
compatibility. Original Tx/Rx chain budgets, normalized noise assumptions,
explicit flag profiles, coverage limitations and external association requirements
remain in the retained contracts/coverage/evidence. Source schema/product labels
must be explicitly adapted without relabeling original bytes or confusing schemas.

The source rule to clean Cargo outputs is excluded: MERLIN preserves stable build
artifacts/caches. Its prohibition on depending on a MERLIN checkout becomes a
prohibition on external checkout/runtime dependencies, not a prohibition on
MERLIN's own local crates. Source setup/CI/package targets and completed design
extensions do not replace MERLIN delivery policy. Shared API conventions are
adopted without retaining a separate source facade, Python package or CLI.

## Completion boundary

INT-DOC-01 can accept this design/traceability consolidation after static checks.
INT-DOC-02 stays open until all ten user criteria hold against actual integrated
paths, API/PNM behavior, current user documentation and passing MERLIN evidence.
Do not delete original technical references merely because they are inventoried,
replaced by a summary, or passed tests in their source workspace.
