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
Windows/macOS packaging helpers are retained but workflow execution is deferred. File presence does not establish
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
workflows or bypass any shared validation. Only Ubuntu 22.04 amd64 desktop
delivery is in scope now. Keep the existing CLI ownership separate. Build Qt via
`scripts/qt-desktop.sh`, then package with `tools/packaging/desktop.py` and verify
with `desktop_smoke.py`; include private Qt runtime, full notices, icon and all
curated examples. Retain caches and require exact VERSION/Cargo metadata.

A manual workflow uploads the DEB and SHA256SUMS as an artifact; notification
requires a newer stable GitHub release carrying
`merlin_VERSION_ubuntu22.04_amd64.deb` and its GitHub SHA-256 asset digest.
Do not publish from manual builds, mutate existing public releases or bypass
CodeQL/dependency checks. Tag publication waits for both CLI and desktop checks.
Run install/reinstall/remove tests only on disposable CI runners unless the user
explicitly requests local installation. Report package bytes, installed footprint,
external prerequisites and hosted/installed verification limits separately.
