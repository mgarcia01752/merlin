# Upstream engineering skills migration

Task: [SKILL-US-01](../../checklist.md#upstream-skill-migration).
This migrates engineering guidance only. Upstream scientific code, source fixtures,
dependencies, APIs and runtime acceptance remain separate integration work under
the [master integration guide](../../ofdm-ofdma-integration-guide.md).

## Baseline and permission scope

MERLIN baseline: `c9a143465aa3302da824b404f3ae7679fd7e9eda` on
`merge-merlin-morgana`, reviewed 2026-11-20. The previously saved integration
guide and its development-index link were already uncommitted and are preserved.
No staged changes were present.

The user's follow-up explicitly requires a prepared `commit-msg` command after
each completed refactor. Record that local handoff rule in the Git workflow skill;
it does not authorize executing commits or adopt the source's broader automatic
command-after-every-section convention.

Source repository: `https://github.com/PyPNMApps/morgana`, exact commit
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`. The local source checkout was clean
at this commit. Reads use that revision; no later commits, source checkout writes,
fetch, build or source test execution are part of this migration.
The [source inventory](source-inventory.json) fingerprints all inspected tracked
skill files and applicable root instructions/license/notices.

The user explicitly authorized skill migration into MERLIN. Source LICENSE is
Apache-2.0, with NOTICE identifying exceptions for PDE-derived instructional
material and CableLabs specifications. This task does not assign new terms to that
material or authorize publication. Applicable source attribution and the PDE
caveat are retained in the root NOTICE. Source-private corpora, PDFs, captured
data and rendered pages are excluded. Source-backend completion and external
interoperability claims are not adopted as MERLIN acceptance evidence.

## Architecture and destination files

Extend the existing skill owners instead of importing parallel catalogs. Generic
PDE practices already have local owners from DEV-GUIDE-01. New detailed references
hold upstream science, test and API procedures; canonical entrypoints route there
only for applicable work. No new skill identity or discovery link is needed.

| Path | Disposition | Responsibility |
| --- | --- | --- |
| `skills/docsis-signal-processing/SKILL.md` | Edit | Route upstream guidance; correct stale adapter-status wording without changing downstream rules. |
| `skills/docsis-signal-processing/references/upstream.md` | Add | Upstream endpoints, processing contracts, receiver reference planes and capture ownership. |
| `skills/scientific-testing/SKILL.md` | Edit by addition | Route direction-specific upstream checks. |
| `skills/scientific-testing/references/upstream.md` | Add | Independent upstream codec/waveform/measurement acceptance. |
| `skills/public-api/SKILL.md` | Edit by addition | Route paired-contract procedure while preserving saved-workbench rules. |
| `skills/public-api/references/paired-api.md` | Add | Rust/Python mapping, planned declarations, native lifecycle and parity. |
| `skills/git-workflow/SKILL.md` | Edit by addition | User-requested automatic command preparation after each completed refactor. |
| `skills/README.md`, `AGENTS.md` | Edit by addition | Discover upstream guidance and state guidance-only scope. |
| `NOTICE` | Edit by addition | Exact source, local adaptation scope and applicable source notice. |
| `docs/architecture.md`, `docs/development/checklist.md` | Edit by addition | Guidance ownership and one authoritative task status. |
| `docs/development/README.md`, `docs/development/features/README.md` | Edit by addition | Navigation to this record. |
| `docs/development/features/upstream-skills/source-inventory.json` | Add | Source-file identities and all 20 skill dispositions. |
| `docs/development/features/upstream-skills/validation.md` | Add | Observed structural/semantic checks and limits. |
| Existing versioned PHY skills, `.agents/skills/`, `.claude/skills/` | Reuse unchanged | Existing editions/navigation and relative discovery. |

There is no new runtime state, numerical implementation, serializer or tool.
The API reference defines review criteria, not new callable signatures.

## Complete source-skill disposition map

All source paths below are relative to `skills/` at the pinned source revision.
Destinations are MERLIN's existing skill owners or new references above.

| Source skill | Decision | Local owner and reason |
| --- | --- | --- |
| api-contracts | REUSE | public-api already owns validation, persistence and Rust/Python compatibility. |
| build-validation | REUSE | scientific-testing and AGENTS retain MERLIN's scoped commands/caches; no source validation script or cleanup rule imported. |
| coding-style | REUSE | MERLIN coding-style already covers domain types, banners, constants, complexity and errors. |
| data-fixture-hygiene | REUSE | scientific-testing's fixture-transformation section covers provenance, sanitization and bounded inventory. |
| development-workflow | REUSE | Existing local workflow owns architecture/requirements/checklist traceability. |
| docsis-phy-3-1-i20 | REUSE | Same pinned metadata/topic/section indexes; source-only corpus paths are not portable to MERLIN. |
| docsis-phy-4-0-i07 | REUSE | Same pinned metadata/topic/section indexes; preserve existing delegation and PDF-verification limits. |
| docsis-signal-processing | ADAPT | Merge upstream rules into the existing owner through references/upstream.md; no second scientific skill. |
| external-change-review | REUSE | Local procedure already inspects staged/unstaged/untracked/committed boundaries and preserves index state. |
| generic-guidance | REUSE | Local development-workflow evidence reference owns self-contained selective adoption; no template dependency import. |
| git-workflow | REUSE | Keep MERLIN save/push/release behavior; automatic refactor handoffs follow the user's explicit local instruction. |
| knowledge-freshness | REUSE | Local evidence reference already defines official-source review without automatic upgrades. |
| knowledge-maintenance | REUSE | Local evidence reference retains project facts and cross-checkout authorization. |
| modular-bootstrap | REUSE | Existing setup guide and evidence reference retain MERLIN's setup contract; no source installer copied or run. |
| numerical-testing | REUSE | scientific-testing already separates independent oracles, convergence and payload truth. |
| public-api | ADAPT | Add the paired API reference and route it from the established public-api owner; no source product facade/import names. |
| reuse-first-changes | REUSE | Local development-workflow already defines path/symbol semantic dispositions and preserved public imports. |
| scientific-testing | ADAPT | Add direction-specific independent codec/waveform checks; retain all downstream acceptance and command gates. |
| software-facts | REUSE | Local evidence reference keeps facts with feature owners, revisions, evidence categories and refresh triggers. |
| third-party-reuse | REUSE | coding-style's reused-material audit owns notices, provenance and actual package inspection. |

REUSE means retain the local guidance owner, not claim that source backend code
is integrated. ADAPT means preserve technical rules while changing local paths,
status labels, examples and commands to fit MERLIN. No source skill is omitted
because its name differs from the local owner.

## Requirements

### SKILL-US-R1: Pinned and scoped migration

The migration MUST use the exact source revision, inspect all 20 source skills,
record a destination/disposition for each and preserve pre-existing local work.
Architecture: [baseline](#baseline-and-permission-scope) and
[disposition map](#complete-source-skill-disposition-map).
Acceptance: matching source hashes, unchanged source checkout/index and preserved
saved prompt; no algorithm, dependency or API implementation changes.
Delivery: [SKILL-US-01](../../checklist.md#upstream-skill-migration).

### SKILL-US-R2: Direction-correct scientific guidance

Guidance MUST retain distinct upstream matrices/shortening, processing order,
receiver input separation and measurement reference planes. It MUST preserve
downstream evidence and avoid treating guidance migration as runtime acceptance.
Architecture: [destination owners](#architecture-and-destination-files).
Acceptance: upstream DSP and test references cover these contracts without copying
private evidence, inventing a source-verification result or requiring interoperability.
Delivery: [SKILL-US-01](../../checklist.md#upstream-skill-migration).

### SKILL-US-R3: Paired APIs and compatibility

Guidance MUST define Rust-first domain semantics and paired native Python review
criteria while retaining MERLIN imports, saved-workbench compatibility and optional
adapter boundaries. Architecture: [destination owners](#architecture-and-destination-files).
Acceptance: explicit operation/signature/error/validation mapping, lifecycle,
buffer ownership, native parity gates and clearly non-executable planning examples.
Delivery: [SKILL-US-01](../../checklist.md#upstream-skill-migration).

### SKILL-US-R4: Local resources and attribution

Adapted guidance MUST resolve required local resources, retain applicable notices
and avoid source product names in canonical skill instructions. Architecture:
[baseline](#baseline-and-permission-scope) and [file map](#architecture-and-destination-files).
Acceptance: metadata, added-link/fragment, discovery, source-attribution and scoped
branding checks pass; remote/source-specific paths are not runtime dependencies.
Delivery: [SKILL-US-01](../../checklist.md#upstream-skill-migration).

## Dependency order and acceptance owner

The official checklist owns SKILL-US-01 status. Its ordered substeps are:

1. Verify baseline, license/notices and all source-skill dispositions (SKILL-US-R1).
2. Adapt the three scientific/API owners (SKILL-US-R2, SKILL-US-R3); depends on 1.
3. Wire local discovery and attribution (SKILL-US-R4); depends on 2.
4. Validate candidate preservation, local resources and semantics; reconcile records
   (SKILL-US-R1 through SKILL-US-R4); depends on 1–3.

The [validation record](validation.md) reports checks actually performed. Whole
backend migration architecture review remains required before source migration.
