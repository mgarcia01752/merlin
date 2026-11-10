> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/project-rename/`
> Logs, captures and specification images are not published.

# Project rename

Requested name and checkout directory: DOCSIS-OFDM-Channel-Sim.
Display name: DOCSIS OFDM Channel Sim.

Updated project documentation, local path examples, skill scope descriptions and
release helper labels. Relative discovery links and local toolchain links remain
valid after moving the checkout. Preserve existing contributor copyright notices;
new-file coding-style templates use the new project name.

Package/API identifiers docsis-ofdm-fec, fec-codec and ofdm-fec-cli, plus the
ofdm-fec CLI and existing scientific JSON discriminators, remain stable. No Git
remote, commit, push or publication changed. Previous uncommitted transmission
and carrier-layout work is retained.

## MERLIN checkout and initial GitHub preparation

On 2026-11-09 the checkout moved to `/home/dev01/Projects/merlin`.
The product remains MERLIN; package/API/CLI identifiers and copyright notices
are unchanged. Local `master` starts at the existing implementation HEAD and
`feature/ds-profile-fec-lab` is retained. No history is rewritten.

Initial delivery scope is Ubuntu 22.04, 24.04 and 26.04 x64. CI performs full
workspace, analysis-only and CLI smoke checks on each after shared preflight.
Tag publication waits for every Ubuntu job and publishes only the 22.04-built
archive, DEB and checksums. Existing Windows/macOS helpers are deferred.

The supplied GitHub owner is `mgarcia01752`; no repository URL is confirmed.
The lookup for `mgarcia01752/merlin` did not resolve, so origin remains unset.
No commit, tag, push or repository creation is part of local preparation.

Validation: path/skill links, Cargo metadata, version consistency, helper tests,
formatting and workflow structure checks. Full Rust tests are preserved from
the preceding validated commit; they must run again through the pre-push gate.

README badges follow the compact Shields.io style used by PyPNM/OSCAR-UI:
development status, local version, CI state, pinned Rust, Ubuntu targets and
license. CI is explicitly not run on GitHub. Replace its static badge with the
actual repository workflow status once origin is confirmed and CI has run.
