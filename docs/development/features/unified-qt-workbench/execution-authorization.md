# Unified Qt execution authorization

On 2026-11-21 the user authorized continuous implementation of all 21 UIU
checklist tasks in dependency order, with applicable tests, traceability and
acceptance evidence for each task. This accepts the planning baseline committed
as `63844f8`; it does not accept unimplemented functionality.

For each accepted task, inspect modified/staged/untracked files, generate a
focused message under the Git workflow, execute
`tools/git/git-save.sh --commit-msg`, verify its SHA and immediately continue
to the next eligible task. Do not combine unrelated tasks. Preserve unrelated
or unfinished work because the helper stages all non-ignored files.

No routine approval pauses are required. Continue until all authorized tasks
pass or a genuine technical blocker prevents further progress. No remote push,
master merge, release publication or system-wide installation is authorized.
The direction-isolation, initial serial dispatch, active-project Save, public
API boundary and scientific compatibility contracts remain binding.

The user's upstream diagram has not been supplied. Keep this explicit final
visual-layout input separate from eligible API, boundary and downstream work;
do not invent evidence that a user diagram was reviewed.
