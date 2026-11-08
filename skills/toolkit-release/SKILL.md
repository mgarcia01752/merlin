---
name: toolkit-release
description: Prepare DOCSIS OFDM FEC Toolkit three-component version bumps, release tags and guarded push commands. Use only for this repository's version and release workflow.
---

# Toolkit release workflow

Read docs/development/git.md, VERSION and the local helpers before commands.
VERSION is canonical; tools/release/versions.py checks every local manifest and
Cargo.lock. Do not edit package versions independently or adopt four-part versions.

Use tools/bump-version.sh --bump major|minor|patch for local version edits.
--dry-run previews without writes. Major resets minor/patch; minor resets patch.
Tags are vMAJOR.MINOR.PATCH. Bumping does not authorize committing or pushing.

Combined tools/git/git-push.sh --tag-patch|--tag-minor|--tag-major requires clean
master and matching metadata. It bumps, runs all local checks, commits only the
version files, creates an annotated tag and atomically pushes master plus that
tag. No force-push, validation skip or blanket git push --tags is allowed.
--force only permits a fresh bump from an already published clean HEAD; it
retains validation, collision checks and non-force atomic pushes. Do not default
to --force or use it to bypass a pending local release.

--tag vMAJOR.MINOR.PATCH retries an existing matching tag at HEAD, without a new
bump or commit. Preserve local state after failures; inspect origin and the local
candidate/commit/tag before recovery. Never move or overwrite public tags.
--dry-run on release modes reads origin and checks preflight but does not bump,
build, commit, create tags or push.

Supply copy/paste commands beginning with
cd /home/dev01/Projects/DOCSIS-OFDM-FEC-Toolkit &&
unless explicitly asked to execute. Adding release tooling is not authorization
to release. This repository currently has no automated publication/installer
workflow; a successful tag push does not itself create a GitHub Release.
Future GUI installers and Python packaging need their own explicitly scoped work.
Report native platform and interoperability evidence separately from tag creation.
