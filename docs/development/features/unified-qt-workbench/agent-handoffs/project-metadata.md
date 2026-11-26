# C4 project loader path and metadata preparation

This source preparation addresses the existing UIU-SESSION-01 C4 owner under
REQ-UIU-06/07. The lead owns runtime acceptance and the canonical
[design](../design.md#uiu-session-01-c4-project-paths-and-metadata) and
[closure plan](../session-closure.md#c4-minimum-project-ownership-implementation).
No SESSION acceptance follows from this source commit.

## Concrete owners and reuse

- `project_load.rs::PreparedProject` moves the shared `ProjectPath` Box, admits
  the actual JSON via `inputs::retention::json_children` plus
  `size_of::<Retained<Value>>()`, and retains a final `Retained<()>` witness for
  exactly `size_of::<PreparedProject>()`. Separately guarded accepted/editor/
  scientific/snapshot children and C3 closures/queues remain charged once.
- The raw `prepare` signature remains for fixtures (`cfg(test)` after primary
  all-target Clippy found no production callers). Its private guarded builder transfers an
  already admitted startup/picker path. `blank` finalizes New's existing flags
  before metadata admission. No serialized Project or scientific body changes.
- `adopt` checks path, metadata and fixed witnesses against the target before
  the existing accepted-state transaction. It returns `AdoptedProject` with
  guarded `path` and `metadata`; no admission occurs after the transaction.
- Loader startup accepts the lead's `startup::AdmittedSource`; recovery admits
  its raw path before capture, and the picker admits its path on the worker.

## Scope and pending hooks

Only `project_load.rs`, new `project_load/project_retention_tests.rs`, and this
handoff are owned here. The lead supplies `shared::project_path::{ProjectPath,
admit}` and `startup::AdmittedSource`, adapts existing callers/fixtures, moves
adopted path/metadata in the bridge, and owns startup/Save guards and acceptance.
No root helper source is copied into this worktree or commit.

## Validation boundary

Source base: `6f005c8e776cd0bb0485c38b10ad9472bcb34ff7`. The six new source
cases cover independently calculated path/JSON spare capacities and passivity;
byte/entry and raw-recovery-path rejection; each foreign product witness plus
stale identity; pointer-preserving same-host moves under pressure and final New
flags; received product lifetime after loader cancellation; and the original
admitted file path moving through actual loader preparation/adoption. The fixed
witness is checked against actual `size_of::<PreparedProject>()`. No clone
capacity equality or guessed scientific allowance is used.

Data precedes its last guard in `PreparedProject` and in the existing C3 delivery.
`AdoptedProject` is a transient move result with guarded path/metadata Boxes.
Those pointees remain independently charged after the project fixed guard drops.
Finite project read/decode/editor-state builders retain the existing
`MAX_PROJECT_BYTES` and original project/capture validators; this adds no
preparation-peak claim. Existing science, `read_bounded`, C3 enqueue/poll/drop and
all original inline test bodies are preserved, except the common Busy message
now uses one equal-purpose constant.

Pinned rustfmt and focused source/allowlist/body checks are the only executed
checks. Compilation, Cargo, runtime tests, Clippy, native Qt/build/wheel validation
are UNEXECUTED here. Source receipts live in the ignored directory
`docs/development/local/unified-qt-workbench/project-metadata-6f005c8/` in this
isolated worktree. The lead must adapt old `adopt()` callers to use
`adopted.metadata.value()` and move `adopted.path`, remove post-admission New JSON
mutation, and update old raw `start_requested` fixtures to admitted same-host
sources. No shared helper or old fixture source is included in this preparation.
