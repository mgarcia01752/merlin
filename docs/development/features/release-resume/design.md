# RELEASE-RESUME-01: Explicit interrupted-bump recovery

Add `--resume` to bump release modes only. Reconstruct the requested candidate
from committed version metadata in an isolated temporary directory. Require the
working files to equal that candidate byte for byte, no other worktree changes,
and an unchanged index. Never restore, stash or discard files. Reject drift,
partial candidates, different bump modes and existing tags. Keep `--tag` as the
recovery path after a release commit/tag exists.

A resume reruns the selected checks against the same version and retains normal
ancestry, identity, candidate-content, immutable-tag and atomic-push guards.
Dry run performs all preflight checks without changes or validation execution.
The shell wrapper owns option forwarding; push-release.py owns candidate
recognition and release lifecycle. Test against disposable local Git remotes,
including failed checks, repeated failure, dry run and unrelated edits.

Validation: all 46 Git-helper tests pass against disposable local remotes
(including 30 release cases). Cover resume success, same-version publication,
previously published base, dry run, repeated failed checks, skip-test forwarding,
unrelated/staged/partial edits, wrong modes and tag collisions. Bash syntax,
ShellCheck, skill validation and diff whitespace checks pass. No real commit,
tag or push was executed; pre-existing version edits remain untouched.

## RELEASE-RESUME-02: Actionable failure output

After the active release has finished, improve checkout-change diagnostics.
Keep the original error and nonzero exit. Reuse exact candidate recognition
before printing a copyable `git-push.sh --tag-<mode> --resume` command; preserve
explicit skip-test/dry-run choices without forwarding force. For unrelated,
staged or altered work, show read-only status/diff commands instead of guessing
a recovery action. Error reporting performs no edits, resets, commits or pushes.
Test plain/force interrupted retries, argument propagation and unsafe candidates
against disposable remotes; existing release guards remain authoritative.

Validation of RELEASE-RESUME-02: disposable-remote tests verify plain/force
retry hints at patch/minor/major, quoted checkout paths, selection propagation,
failed-gate advice and no guessed resume for unrelated/staged/altered work.
The original error and nonzero exit remain; no real release action was executed.

RELEASE-RESUME-02 completion: all 52 current Git-helper tests pass, plus the
focused plain retry regression at all three bump modes. Recovery commands
include `--test-quick` when selected and never discard files or bypass checks.
