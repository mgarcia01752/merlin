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
