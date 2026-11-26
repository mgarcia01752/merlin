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

On 2026-11-22 the user supplied and approved the
[upstream startup layout](upstream-startup-layout.md) for REQ-UIU-10 / UIU-US-01.
This satisfies the visual-input gate and explicitly authorizes implementation
when the existing dependencies pass, without another approval pause. The user
will fine-tune spacing, styling and drill-ins after a functional implementation.
Approval is not evidence of rendering, scientific behavior or task acceptance.
All original dependency, validation and local-only commit restrictions remain.
