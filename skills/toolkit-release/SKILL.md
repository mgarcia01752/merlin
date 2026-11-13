---
name: toolkit-release
description: Prepare DOCSIS OFDM Channel Sim three-component version bumps, release tags and guarded push commands. Use only for this repository's version and release workflow.
---

# Toolkit release workflow

Read docs/development/git.md, VERSION and the local helpers before commands.
VERSION is canonical; tools/release/versions.py checks every local manifest and
Cargo.lock. Do not edit package versions independently or adopt four-part versions.

Use tools/bump-version.sh --bump major|minor|patch for local version edits.
--dry-run previews without writes. Major resets minor/patch; minor resets patch.
Tags are vMAJOR.MINOR.PATCH. Bumping does not authorize committing or pushing.

Combined tools/git/git-push.sh --tag-patch|--tag-minor|--tag-major requires clean
master and matching metadata. It bumps, runs the selected local checks, commits only the
version files, creates an annotated tag and atomically pushes master plus that
tag. Explicit `--skip-test` omits local Rust tests/smokes, retaining version,
tooling, style, Clippy, release build and Git guards. Report omitted tests;
hosted release gates remain mandatory. No force-push or blanket git push --tags.
The local full gate uses `scripts/test-local.sh all` with four process-isolated
workers by default (`MERLIN_TEST_THREADS` overrides). Preserve analysis-only,
example and doctest phases; hosted workflow commands remain independent.
--force only permits a fresh bump from an already published clean HEAD; it
retains validation, collision checks and non-force atomic pushes. Do not default
to --force or use it to bypass a pending local release.

--tag vMAJOR.MINOR.PATCH retries an existing matching tag at HEAD, without a new
bump or commit. Preserve local state after failures; inspect origin and the local
candidate/commit/tag before recovery. Never move or overwrite public tags.
--dry-run on release modes reads origin and checks preflight but does not bump,
build, commit, create tags or push.

Supply copy/paste commands beginning with
cd /home/dev01/Projects/merlin &&
unless explicitly asked to execute. Adding release tooling is not authorization
to release. Native CLI validation/release workflows are prepared under .github/workflows/.
Initial workflows test Ubuntu 22.04, 24.04 and 26.04 x64, then publish the
22.04-built archive, DEB and checksums after successful version-tag validation.
Windows x64 desktop UI MSI validation/packaging is configured alongside Ubuntu.
Windows CLI assets are excluded; macOS workflow execution remains deferred. File presence does not establish
native execution or authorize a push. Keep QA-06/07 pending until target evidence
exists. tools/packaging/ owns staging/builders/checksums/collection/notes; use those
helpers instead of duplicating packaging in workflow shell. Preserve full license
notices and public examples. Install nothing while testing package generation.
Use docs/development/native-delivery.md for exact stage ownership and the
per-OS docs/user/install/ guides for installer/upgrade/uninstall instructions.
The Ubuntu desktop path below has its own scoped task; Python packaging remains deferred.
Report native platform and interoperability evidence separately from tag creation.


## Ubuntu 22.04 desktop

Keep exactly two workflow entry points: Master (`ci.yml`) and Release
(`release.yml`). Release calls the shared Master workflow with its release
input enabled. Shared validation includes Qt, three Ubuntu native jobs, CodeQL
and both-lock RustSec; optimized compilation, packaging, installed lifecycle
and publication are release-only. Do not recreate standalone security/desktop
workflows or bypass any shared validation. Ubuntu 22.04 amd64 and Windows x64 desktop
delivery are in scope; macOS remains deferred. Keep the existing CLI ownership separate. Build Qt via
`scripts/qt-desktop.sh`, then package with `tools/packaging/desktop.py` and verify
with `desktop_smoke.py`; include private Qt runtime, full notices, icon and all
curated examples. Retain caches and require exact VERSION/Cargo metadata.

A manual workflow uploads the DEB and SHA256SUMS as an artifact; notification
requires a newer stable GitHub release carrying
`merlin-desktop_VERSION_ubuntu22.04_amd64.deb` and its GitHub SHA-256 asset digest
(the exact legacy desktop name remains a supported compatibility fallback).
Do not publish from manual builds, mutate existing public releases or bypass
CodeQL/dependency checks. Tag publication waits for both CLI and desktop checks.
Run install/reinstall/remove tests only on disposable CI runners unless the user
explicitly requests local installation. Report package bytes, installed footprint,
external prerequisites and hosted/installed verification limits separately.

Explicit `--resume` with a bump mode accepts only a complete, exact unstaged
version-only candidate reconstructed from HEAD. It reruns selected checks and
finishes the same version without resets or a second bump. Reject unrelated,
staged, partial, redirected or mismatched files. Use `--tag` after a release
commit/tag exists; `--force` never serves as dirty-tree recovery. Commit source
and helper changes separately before leaving only the pending version edits.

Explicit `--test-quick` selects the reviewed local subset via
`tools/testing/quick-tests.json`, skipping 21 named expensive sweep cases while
retaining representative recoveries and other tests. Keep doctests, preparation
example, analysis-only checks, release smokes, static/tooling checks and Git
guards. Full remains default; hosted CI is unchanged. Reject combining it with
`--skip-test`; preserve the selected mode through `--resume` and printed retry
commands. Report omissions and do not equate subset success with full coverage.

## Required release asset naming

Use `tools/packaging/asset_names.py` for all distributable filenames:
`merlin-COMPONENT_VERSION_TARGET_ARCH.EXT`. COMPONENT is desktop or cli; VERSION
is MAJOR.MINOR.PATCH; ARCH is amd64 or arm64 (normalize internal x64 to amd64).
Ubuntu DEBs use ubuntu22.04; portable Linux archives use linux; prepared macOS
and Windows CLI formats use macos/windows. Keep SHA256SUMS unchanged. Preserve
installed package IDs merlin/merlin-ofdm, executable paths and installer IDs.

Publish an identical `merlin_VERSION_ubuntu22.04_amd64.deb` compatibility alias
until older installed updater clients are explicitly retired. Checksum both.
Prefer canonical desktop metadata in new updaters, allow only the exact legacy
fallback, and never hide malformed canonical metadata by falling back. CI must
install/smoke only the canonical desktop DEB. Never rename published assets.

Installation guides use generic `sudo apt install ./PACKAGE.deb` instructions
and explain replacing the placeholder with the downloaded file. Render exact
release-version commands in release notes; retain the ./ local-file prefix.
Validate naming, package identity, alias byte/checksum equality, updater matching
and download URL validation together before changing this convention.

## Windows desktop UI MSI

Use `scripts/windows/qt-desktop.ps1` and
`tools/packaging/windows_desktop.py` on native Windows x64. The pinned build uses
MSVC2022, Rust1.85.1, Qt6.8.3 and WiX3.14.1. Reproduce installer artwork with
`tools/branding-windows.py`; preserve the existing logo and source provenance.
Keep the desktop UpgradeCode separate from the dormant Windows CLI helper.
Never publish a Windows CLI archive/MSI or macOS asset under this scope.

Release collection requires `merlin-assets-desktop-windows-amd64` containing the
canonical desktop MSI and SHA256SUMS. Retain all Ubuntu, CodeQL and RustSec gates.
Run installed lifecycle checks only on disposable CI; local builds authorize no
installation, signing or publication. Report native evidence and byte sizes
separately. See [Windows build guide](../../docs/development/windows-desktop.md).

## Example folder ownership

Standalone CLI request/settings JSON and the sample payload belong under
`examples/cli/` with purpose/use-case commands in its README. Desktop payloads
copy the explicit workbench/measurement allowlist through desktop_payload.py
and omit the CLI folder. CLI packages retain it. Keep workbench JSON, original
capture bytes, provenance and licenses together; update references/tests when
relocating examples. `examples/README.md` explains the folders.
