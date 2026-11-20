# Selective engineering guidance adoption

Task: **DEV-GUIDE-01** in the [official checklist](../../checklist.md#engineering-guidance).
This is a documentation and agent-guidance change. It does not implement OFDMA,
change downstream behavior, or start Qt work.

## Source baseline

Reviewed on 2026-11-20 against MERLIN baseline
`096d5a476194d4f2e016863d723b225257bb6037`, on `merge-merlin-morgana`.
The source is the local Personal Development Environment (PDE) checkout at HEAD
`6f7197d17b0efc7a4be0d44a517d886b2f03009d`, including reviewed uncommitted and
untracked guidance. This is a **working-tree candidate**, not a published PDE
revision or proof of remote freshness. No fetch or modification of PDE is part
of this adoption. The [source inventory](source-inventory.json) records the
selected files' SHA-256 hashes and Git state so the candidate is identifiable.
Historical archives are excluded.

The user authorized additive incorporation, preserving MERLIN-specific rules.
Procedures are expressed locally for MERLIN; no PDE tools, templates, source
code, assets or archive trees are imported. Future actual material imports must
follow the [reuse audit](../../../../skills/coding-style/SKILL.md#reused-material-audit).

## Architecture and ownership

Use **Architecture → Requirements → Developer checklist → Implementation →
Validation**, starting from the user's goal and constraints. Existing MERLIN
owners remain authoritative. Two missing skill owners cover planning/reuse and
external-change review; existing skills receive only focused clarifications.
The local skill catalog and relative discovery links make all required guidance
available without a PDE checkout. There is no runtime component or new state.

| Path | Disposition | Responsibility |
| --- | --- | --- |
| `AGENTS.md` | Edit by addition | Route planning and external review without replacing domain rules. |
| `skills/README.md` | Add | Searchable catalog of canonical MERLIN skills. |
| `skills/development-workflow/SKILL.md` | Add | Planning, reuse decisions and linked acceptance. |
| `skills/development-workflow/references/evidence.md` | Add | Facts, freshness, selective adoption and setup review boundaries. |
| `skills/external-change-review/SKILL.md` | Add | Baseline, staged/unstaged/untracked inspection and semantic review. |
| `skills/coding-style/SKILL.md` | Edit by addition | Material provenance, permission and package-notice audit. |
| `skills/scientific-testing/SKILL.md` | Edit by addition | Boundary oracles, fixture transformation and coverage mapping. |
| `skills/ui-design/SKILL.md` | Edit by addition | Measurement-display and native-consumer acceptance. |
| `.agents/skills/`, `.claude/skills/` | Add two relative links in each | Discover new canonical skills. |
| `docs/development/README.md`, `docs/development/features/README.md`, `docs/README.md` | Edit by addition | Navigation to guidance and this adoption record. |
| `docs/architecture.md`, `docs/development/checklist.md` | Edit by addition | Guidance ownership and the single completion-status owner. |
| `docs/development/features/engineering-guidance/validation.md` | Add | Observed checks and limits. |
| `docs/development/features/engineering-guidance/source-inventory.json` | Add | Source candidate identity; no source payload. |

Validation uses whitespace, Markdown file/fragment, skill metadata, discovery-link
and preservation checks. No new executable or automated test suite is necessary
for this documentation-only change. Rust/Qt builds and simulation tests are outside
this validation scope because their inputs and implementation remain unchanged.

## Gap and adoption decisions

| PDE area | Existing MERLIN owner / decision |
| --- | --- |
| Development workflow, architecture navigation and checklist traceability | Add the development-workflow skill; retain existing architecture/checklist paths and IDs. Apply detailed traceability to new or materially revised sections; no historical rewrite. |
| Reuse-first changes | Add a concrete path/symbol and semantic disposition table to that skill. An existing feature is always the existing-code workflow. |
| External-change review | Add the missing skill, preserving index/unrelated changes and reporting unknown authorship. |
| Software facts, knowledge freshness and maintenance | Add one evidence reference; keep topic records with their current feature owner rather than creating a competing facts tree. |
| Generic guidance and selective adoption | Add self-contained resource/override checks to the evidence reference and catalog; no dependency on PDE paths or helpers. |
| Third-party reuse | Extend coding-style's existing SPDX/licensing owner with import and distribution audit fields; retain NOTICE and all existing licenses. |
| Numerical testing and fixture hygiene | Extend scientific-testing with parser boundary examples, explicit sanitization and coverage mapping. Preserve existing closed-loop acceptance and normative tests. |
| Qt native behavior, measurement visualization and desktop icon acceptance | Extend ui-design with data-grid/units/interaction and actual-consumer checks; no UI implementation or new theme promise. |
| Build validation | Retain AGENTS/build/scientific-testing commands and caches; clarify evidence scope in the new workflow. No new runner. |
| API contracts and saved-data/bindings | Retain public-api, including Rust/Python parity and saved-workbench recipes; already more specific. |
| Coding conventions, logging and diagnostics | Retain coding-style and existing help/logging owners. No banner/style reset or new diagnostic subsystem. |
| Git save/push and release versioning | Retain MERLIN helpers and authorization/gates. Do not import PDE scripts or their differing file-staging semantics. |
| Complete desktop packaging, CI/CD and updates | Retain toolkit-release and local delivery designs. Do not add CLI tools to Windows MSI, add macOS, change direct Update behavior, partial-release policy or the two workflow entrypoints. |
| Modular development setup | Add review criteria for genuinely read-only inspection and lazy toolchain writes; no installer changes, invocation or new platform claim. |
| Repository baseline and branding | Add skill navigation; retain the existing product README, badges and artwork. New foundation defaults do not authorize restyling. |
| AI development/integration and bounded reports | Retain explicit scope, Rust ownership and resource bounds. Do not add providers, reports, remote analysis or Python sidecars. |

## Requirements

### GUIDE-R1: Preserve local contracts

The adoption MUST add clarification without removing existing instructions,
changing source/tooling/dependencies or importing incompatible defaults.
Architecture: [ownership and file plan](#architecture-and-ownership).
Acceptance: existing-file lines are preserved; changed paths are documentation
and new discovery links only.
Delivery: [DEV-GUIDE-01](../../checklist.md#engineering-guidance).

### GUIDE-R2: Traceable planning and reuse

New guidance MUST connect architecture, observable requirements, prerequisite
task IDs and validation; it MUST distinguish planning from implemented support.
It MUST define semantic reuse decisions with concrete owner paths.
Architecture: [adoption decisions](#gap-and-adoption-decisions).
Acceptance: a worked planning chain, deferred-case policy, stage/file navigation
and preservation of existing task IDs are locally documented.
Delivery: [DEV-GUIDE-01](../../checklist.md#engineering-guidance).

### GUIDE-R3: Review and evidence

Guidance MUST cover staged, unstaged, untracked and committed review boundaries,
index preservation, evidence categories, freshness and actual material reuse.
Architecture: [adoption decisions](#gap-and-adoption-decisions).
Acceptance: no reliance on chat authorship, unavailable helpers, a blanket
license guarantee, automatic upgrades or unperformed native/hosted checks.
Delivery: [DEV-GUIDE-01](../../checklist.md#engineering-guidance).

### GUIDE-R4: Self-contained discovery

New skills MUST have valid metadata, complete local resources and relative
discovery links under both agent directories. New links MUST resolve to the
intended files and sections. Architecture: [file plan](#architecture-and-ownership).
Acceptance: catalog, metadata and file/fragment/link checks pass without PDE.
Delivery: [DEV-GUIDE-01](../../checklist.md#engineering-guidance).

### GUIDE-R5: Candidate provenance and truthful validation

The adoption MUST identify the dirty source candidate accurately and record
checks actually run. Architecture: [source baseline](#source-baseline).
Acceptance: selected source hashes/state match at handoff, no PDE edits occur,
and runtime/native/hosted validation is explicitly outside the claim.
Delivery: [DEV-GUIDE-01](../../checklist.md#engineering-guidance).

## Delivery order and status ownership

The [official checklist](../../checklist.md#engineering-guidance) owns
**DEV-GUIDE-01** status; this record provides its requirements and acceptance.

1. Inspect both baselines and map gaps/overrides (GUIDE-R1, GUIDE-R5); no prerequisite.
2. Add the local skill owners and narrow existing-owner clarifications
   (GUIDE-R2, GUIDE-R3); depends on step 1.
3. Add catalog/discovery/navigation and record source identity (GUIDE-R4,
   GUIDE-R5); depends on step 2.
4. Validate additions and preservation; reconcile this record and official status
   only after acceptance (GUIDE-R1 through GUIDE-R5); depends on steps 1–3.

Downstream/OFDMA integration architecture, PDE helper changes and installer/UI
implementation remain separate tasks. This adoption does not mark them complete.
