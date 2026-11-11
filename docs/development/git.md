# Git and versioning

The helpers resolve paths from their own locations, not the calling directory.
They do not publish GitHub Releases or configure an origin remote.

VERSION is the source of truth. `python3 tools/release/versions.py` checks the
three local Cargo manifests and Cargo.lock. Versions are MAJOR.MINOR.PATCH;
release tags are vMAJOR.MINOR.PATCH. The initial version is0.1.0.

## Save

Review tracked, staged and untracked content first. The helper stages all
non-ignored changes, including new files. Private development evidence, PDFs,
local toolchains and target/ remain ignored.

```bash
cd /home/dev01/Projects/merlin &&
tools/git/git-save.sh --commit-msg "Feature: Describe the complete change
- Describe the implementation
- Tests: Record actual validation"
```

Save creates a local commit only. It does not push or bump a version. Initial
commits work in a new repository once Git author identity is configured.

## Push

```bash
cd /home/dev01/Projects/merlin &&
tools/git/git-push.sh
```

Plain push requires clean main/master, a configured origin and a local branch
that is not behind or diverged. It classifies every outgoing commit against the
fetched remote HEAD, rechecks that branch/commit/tree are unchanged and pushes
only the captured branch object. Documentation-only ranges skip the local
build/test gate automatically; all other ranges run `scripts/check-before-push.sh`.
Feature branches must be reviewed and merged locally before using this helper.

The validation gate includes version consistency, Python helper tests, Bash
syntax/ShellCheck, rustfmt, Clippy, all workspace tests, analysis-only tests,
a release CLI build and an actual seeded simulation smoke run. It installs
nothing. Python3, ShellCheck, and the pinned Rust toolchain are prerequisites.
Local Rust test phases use `scripts/test-local.sh all`, with four nextest workers
by default; `MERLIN_TEST_THREADS` sets the process budget. Doctests, preparation
example and analysis-only coverage remain. First use fetches a checksum-pinned
runner into ignored local storage; no system installation is performed.
Hosted workflows retain their commands. See [parallel local validation](features/parallel-local-tests/design.md).
For automatic documentation-only pushes, the allowed paths are root `README.md`,
Markdown files under `docs/`, and PNG/JPG/JPEG/SVG/WebP/GIF images under
`docs/images/`. Changes to skills, AGENTS.md, fixtures, source, manifests, tools
or workflows default to the full gate. Every outgoing commit is inspected,
including merge-parent differences and both paths of renames; reverting code
later does not make a range documentation-only. Initial pushes and release/tag
modes default to the full gate. Git inspection errors stop the push.

Use the same ordinary push command for documentation changes. Explicit
`--skip-test` is available for branch, bump and existing-tag pushes when the
user chooses to omit local Rust test suites and simulation smoke runs. It retains
version consistency, Python tooling tests, shell/style checks, Clippy and the
release CLI build; all clean-tree, ancestry, immutable-tag and atomic-push guards
remain. Output explicitly reports omitted checks. GitHub CI, CodeQL, audits and
installed-package release gates remain unchanged. Default behavior runs everything.

```bash
cd /home/dev01/Projects/merlin &&
tools/git/git-push.sh --tag-patch --skip-test
```

This option does not claim skipped tests passed. Clean-tree, ancestry, unchanged-HEAD and non-force guards
still apply. This shortcut affects local validation only; GitHub CI is unchanged.
For full-gate pushes, missing checks remain failures.

## Local bump preview

```bash
cd /home/dev01/Projects/merlin &&
tools/bump-version.sh --bump patch --dry-run
```

Without --dry-run, only VERSION and local Cargo version metadata change. No
Git commit, tag or push occurs. Major resets minor/patch; minor resets patch.

## Combined release push

After saving and merging reviewed work to master:

```bash
cd /home/dev01/Projects/merlin &&
tools/git/git-push.sh --tag-patch
```

Use --tag-minor or --tag-major for those components. Release modes require clean
master and consistent committed metadata. They inspect origin, reject collisions,
bump, validate the candidate with the selected local checks, commit only version files, annotate the tag and
atomically push master plus that tag. They never force-push or overwrite a tag.
Add --dry-run to preview without a bump/build/commit/tag/push; it still reads origin.

A clean HEAD already published under its current tag is a no-op. Explicit --force
with a bump mode permits another release for a rebuild/test, while retaining all
checks. It is not Git force-push and does not bypass unpublished release recovery.

If a release upload fails, local state is retained. Inspect it and retry the same
tag with `tools/git/git-push.sh --tag vMAJOR.MINOR.PATCH`; do not bump again blindly.
Version edits or commits made before a failed check are never reset automatically.

Prepared tag workflows validate Ubuntu 22.04, 24.04 and 26.04 before publishing
the tested Ubuntu 22.04 CLI archive, DEB and checksums. Remote execution remains
unverified until the first push. See [native delivery](native-delivery.md).

The local checkout is `/home/dev01/Projects/merlin`, with `master` as the
integration/release branch. The original implementation branch is retained.
Configure origin only after the actual repository URL is supplied; the GitHub
owner profile alone is not a repository URL.
