# DEV-04: Explicit local test omission

The user requested `--skip-test` after a full local release validation took
32 minutes, to avoid repeating simulation tests for unrelated tooling fixes.
This explicit choice supersedes the previous blanket local no-skip policy.

`tools/git/git-push.sh` accepts the option for branch, version-bump and
existing-tag modes. Release orchestration forwards it to
`scripts/check-before-push.sh`, and failure recovery commands preserve it.
Defaults are unchanged; `--dry-run` still performs no validation or mutation.

The selected gate retains version checks, Python tooling tests, Bash syntax,
ShellCheck, rustfmt, Clippy and a release CLI build. It omits the local Rust
test suites, example test and simulation/CLI smoke runs. A clear console notice
identifies omitted tests; successful remaining checks never imply those tests
ran. It does not grant commit/push authorization or alter GitHub CI, security,
native installed-package checks or publication dependencies. Clean-tree,
ancestry, metadata, unchanged-candidate, immutable-tag and atomic-push checks
remain mandatory. No automatic assertion that a change is test-independent is
made; this is the user's explicit validation choice.

Verification uses disposable local bare remotes for option forwarding, matching
tag/commit publication and rejection after retained-check failures. A separate
test executes the actual gate with command doubles to verify retained commands,
omitted Rust tests/smokes and unknown-option rejection without rebuilding Rust.
No real remote, system installation or simulation run is needed for these tests.

Validation: all 41 Git/helper tests pass, including the four added option/gate
checks. ShellCheck, Bash syntax, Python compilation and whitespace checks pass.
No Rust compilation or simulation tests were repeated for this tooling change.

Usage after committing reviewed changes:

```bash
cd /home/dev01/Projects/merlin &&
tools/git/git-push.sh --tag-patch --skip-test
```
