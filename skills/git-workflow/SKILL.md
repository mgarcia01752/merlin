---
name: git-workflow
description: Prepare commits and guarded pushes for DOCSIS OFDM FEC Toolkit using its local helpers, including change inspection and the required pre-push validation gate.
---

# Git workflow

Ported from PNM Viewer Toolkit and OSCAR-UI helper conventions.
Before a commit message, inspect git status --short, git diff --stat, git diff,
staged changes and every untracked file. git-save stages all non-ignored files.
Preserve private inputs under ignored development/ and never stage Cargo caches.

Use tools/git/git-save.sh --commit-msg with one double-quoted multiline argument.
Use Feature:, BugFix: or Documentation:, imperative summary and - detail lines,
including - Tests:. Keep every message line at most60characters.
Provide a command unless the user explicitly authorizes execution; start command
blocks with cd /home/dev01/Projects/DOCSIS-OFDM-FEC-Toolkit &&.

Use tools/git/git-push.sh for pushes. Plain push requires clean main/master,
origin, nondiverged ancestry and successful scripts/check-before-push.sh.
The same tested commit and clean tree must remain unchanged throughout validation.
Do not bypass checks or use Git force-push. No feature-branch push is authorized
by the helper. Use toolkit-release for explicit version/tag modes.

A new empty repository needs its initial reviewed commit and origin before a
push can succeed. Do not create either merely because tooling was requested.
