---
name: git-workflow
description: Prepare commits and guarded pushes for DOCSIS OFDM Channel Sim using its local helpers, including change inspection and the required pre-push validation gate.
---

# Git workflow

Before a commit message, inspect git status --short, git diff --stat, git diff,
staged changes and every untracked file. git-save stages all non-ignored files.
Preserve private inputs under ignored docs/development/local/ and never stage
Cargo caches. Track reviewed feature plans and validation summaries under
docs/development/features/ and link them from its index.

At feature completion and whenever the user asks for commit-msg, review
`docs/architecture.md` and `docs/development/checklist.md` against the complete
pending change scope. Update affected ownership, contracts, status and next steps
before supplying the command. Check items only when implementation and supporting
validation are complete; record partial progress without checking combined items
early. If no architecture or checklist change is needed, leave them unchanged.
Verify the task-linked design record follows coding-style's per-task design
requirements and reflects the completed implementation/validation.
Inspect the final diff again so the message includes any documentation updates.

Use tools/git/git-save.sh --commit-msg with one double-quoted multiline argument.
Use Feature:, BugFix: or Documentation:, imperative summary and - detail lines,
including - Tests:. Keep every message line at most60characters.
Provide a command unless the user explicitly authorizes execution; start command
blocks with cd /home/dev01/Projects/merlin &&.

Use tools/git/git-push.sh for pushes. Plain push requires clean main/master,
origin and nondiverged ancestry. The helper automatically skips the local
build/test gate only when every outgoing commit changes root README.md, Markdown
under docs/, or approved documentation images under docs/images/. It inspects
per-commit merge/rename paths, so code reverted later still requires full checks.
Initial pushes, mixed changes and all tag/releases use the full gate. No manual
skip flag is supported; Git inspection failures stop the push. GitHub CI is
unchanged. The captured commit and clean tree must remain unchanged throughout.
Do not bypass the helper's classification or use Git force-push. No feature-branch push is authorized
by the helper. Use toolkit-release for explicit version/tag modes.

A new empty repository needs its initial reviewed commit and origin before a
push can succeed. Do not create either merely because tooling was requested.
