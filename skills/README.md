# MERLIN skill catalog

[Repository instructions](../AGENTS.md) · [Development workspace](../docs/development/README.md)

Choose skills by the work being done. Existing MERLIN contracts and the user's
scope govern selective foundation guidance; a skill does not authorize new scope.
Canonical folders live here, with relative discovery links under `.agents/skills/`
and `.claude/skills/`. No separate PDE checkout is required.

| Task | Canonical skill |
| --- | --- |
| Architecture, requirements, dependency checklist, semantic reuse | [development-workflow](development-workflow/SKILL.md) |
| Review another session's staged, unstaged, untracked or committed work | [external-change-review](external-change-review/SKILL.md) |
| Source ownership, conventions, notices and material reuse | [coding-style](coding-style/SKILL.md) |
| Numerical/parser tests, fixtures and acceptance evidence | [scientific-testing](scientific-testing/SKILL.md) |
| Rust/Python public APIs and saved-workbench compatibility | [public-api](public-api/SKILL.md) |
| Signal processing and supported implementation states | [docsis-signal-processing](docsis-signal-processing/SKILL.md) |
| DOCSIS 3.1 PHY I20 evidence | [docsis-phy-3-1-i20](docsis-phy-3-1-i20/SKILL.md) |
| DOCSIS 4.0 PHY I07 evidence and delegation | [docsis-phy-4-0-i07](docsis-phy-4-0-i07/SKILL.md) |
| Field RxMER/profile margins and FEC interpretation | [docsis-pnm-rxmer-fec](docsis-pnm-rxmer-fec/SKILL.md) |
| Qt actions, layout, instruments and native acceptance | [ui-design](ui-design/SKILL.md) |
| Commit commands, save/push helpers and validation gates | [git-workflow](git-workflow/SKILL.md) |
| Canonical versions, release/package contracts and retries | [toolkit-release](toolkit-release/SKILL.md) |

For software facts, official-source freshness, setup review and selective adoption,
read the workflow's [evidence reference](development-workflow/references/evidence.md).
Private specification PDFs and captures remain under ignored local evidence paths;
linked reference guidance does not promise those private files in a fresh clone.

## Upstream integration guidance

The existing skill owners now include
[upstream scientific contracts](docsis-signal-processing/references/upstream.md),
[upstream validation](scientific-testing/references/upstream.md) and
[paired Rust/Python review](public-api/references/paired-api.md).
The [migration record](../docs/development/features/upstream-skills/design.md)
maps every source skill to a local owner and preserves source attribution.
Skill availability does not establish upstream runtime support.
