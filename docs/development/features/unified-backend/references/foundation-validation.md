> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/foundation/validation.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Foundation checkpoint validation

Historical checkpoint record. The 2026-11-20
[all-family audit and codec acceptance](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/validation.md)
supersedes the source-table gap and paused-implementation status below.

Date: 2026-11-19. Scope: preserve existing code, adopt skills/instructions and
prepare a local save command. Scientific implementation remains paused.

The checkpoint has 23 passing Rust unit tests: 21 standalone codec cases and
two facade/report cases. `scripts/cargo-local.sh fmt --all -- --check`,
`scripts/cargo-local.sh test --workspace --locked` and
`scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings`
passed locally with Rust 1.85.1 on Linux x86_64. Warm test execution completed in
under one second; these results do not verify source table transcription.

`scripts/validate.sh` passed: 19 canonical skills and their 38 relative discovery
links, 210 local documentation links, workspace version identity, five isolated
save-helper tests, shell syntax, formatting, all 23 Rust unit tests and Clippy.
The isolated Git tests took 0.585 s and Rust codec cases took 0.39 s on the warm
local run. The skill-creator `quick_validate.py` also passed for all 19 skills.
These counts are coverage summaries, not scientific compatibility evidence.
No user repository commit or push was part of validation. Git tests committed
only inside temporary repositories without remotes. Actual candidate staging and
the commit are left to the prepared command.

Known unresolved issue: long matrix fifth row, zero-based column 20, is -1 in the
current source but 12 in PDF p57. The Gaussian test encoder and production share
table constants. Their agreement is independent algorithm evidence, not an
independent source transcription check. The complete LDPC milestone is unfinished.

Pinned PHY3.1 pp56–57 were inspected visually. PHY4.0 FEC delegation was checked
in extracted text only. No externally produced reference vectors, full OFDMA
waveform, native Python, hosted CI, cross-platform execution, installed lifecycle
or published package validation is claimed. PDE instructional redistribution
terms and complete dependency/package notice inventories remain publication gaps.

MERLIN's source checkout remains clean. PDE contains pre-existing unrelated
working-tree edits; the selected Git-workflow source instructions are working-tree
material above the recorded base revision. File hashes in the adoption manifest
identify the exact inspected inputs; uncommitted authorship is not inferred.
No PDE source files or index entries were changed by this task.
