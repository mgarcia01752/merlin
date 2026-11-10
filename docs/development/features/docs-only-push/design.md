# DOC-PUSH-01: documentation-only branch pushes

Ordinary branch pushes automatically skip the local build/test gate only when
all outgoing commits change documentation. The push helper owns remote ancestry,
clean-tree and unchanged-HEAD checks; `tools/git/docs_only.py` owns classification.

Compare the fetched remote commit with the captured local HEAD. Inspect each
outgoing commit separately, including both sides of renames and merge parents,
so a code change followed by a revert cannot masquerade as documentation-only.
Allow root README.md, Markdown under docs/, and raster/SVG images under docs/images/.
Changes to skills, AGENTS.md, fixtures, code, manifests, workflows or tools require
the full gate. Missing remote branches and empty ranges also use the full gate.
Git inspection failures stop the push. Tag/release modes always use the full gate.
No manual unrestricted skip flag is provided. GitHub CI behavior is unchanged.

Validation uses isolated Git repositories to cover documentation-only ranges,
mixed changes, renames, reverted code, merge history, initial pushes and Git
inspection failures. Existing guarded-push tests retain clean-tree and failure
coverage. No real remote is written during tests.

Completed validation: all 37 Git helper/policy/release tests pass, including
real-history classification and guarded docs-only/mixed push paths. Bash syntax,
ShellCheck, skill validation through all discovery paths and whitespace checks
pass. Rust tests were not rerun for this tooling-only change.
