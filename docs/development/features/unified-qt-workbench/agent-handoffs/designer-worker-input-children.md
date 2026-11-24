# Designer worker-input child visitors

Source-only base: `d4c60d6daf5fa8353390f01550adf009c183ca22`.
Branch: `feature/qt-designer-worker-input-children`. This preparation supports
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07). It adds no admission owner
and establishes no official acceptance.
Primary runtime adoption now links the
[worker-input validation](../validation.md#uiu-session-01-retained-designer-worker-inputs);
source-only execution statements below describe the frozen preparation.

Only the existing `measurement_input/designer.rs`, `designer/retention.rs`,
`model.rs`, `model/retention.rs`, new `designer/worker_input_tests.rs` and this
handoff change. Existing retention test files remain byte-identical and read-only.

The two application-private forwarding hooks are:

```rust
designer::request_children(&Request) -> Result<RetainedResources, String>
model::options_children(&Options) -> Result<RetainedResources, String>
```

They delegate to the same-named `pub(super)` functions in each owner's retention
module. Those functions contain the exact existing Request/Options traversal
bodies extracted from Session/Model tails, starting from zero children. The full
Session and Model visitors append their returned quantities; their containing
fixed values remain counted once. No Request/Options fixed layout is added by
these hooks, and all original scientific bodies/assertions stay unchanged.

Request children are actual source String capacity plus existing recipe children
(edit Vec capacity and exhaustive fixed component audit). Capture count and
baseline pass the existing fixed Copy audit. Options children are owned Cow
interval capacity, selected Vec slot capacity and every selected String capacity;
borrowed static text owns zero payload. Mode/tolerance pass the existing Copy
audit. Current child graphs add payload bytes only, with zero ordered entries and
shared roots. Existing checked append/capacity arithmetic and errors are reused.

The lead owns concrete parsed-request/selection and Work metadata owners. Count
their fixed layouts once, then append these child quantities before admission
through existing Retained/admit_model. Measure actual copies rather than assuming
Clone capacity equality; keep input guards through actual worker completion,
queue/disposal and any transferred child ownership. This unit changes no bridge,
thread, queue, control, terminal error, export product or saved-reply lifecycle.
Those remain distinct concrete retained consumers, not contract exclusions.

Two focused tests are prepared, both **UNEXECUTED**:

- A valid fixture-derived Request with a full-grid edit measures source/edit spare
  capacity deltas, excludes fixed bytes and preserves values/buffer identity under
  repeated queries. No edit is materialized or scientific capture rebuilt.
- Real Options measure Owned Cow, selected Vec and String spare capacity deltas;
  borrowed default text costs zero and restoring borrowed text removes only the
  owned buffer. Values remain identical. No artificial positive entry/root claim
  or unrelated aggregate admission test is introduced.

Only pinned rustfmt and source/diff/allowlist/link checks run here. Compilation,
Clippy, Rust/Qt/QML/native runtime checks and bridge acceptance remain
**UNEXECUTED**, lead-owned. No shared cache, build, manifest, public scientific API
or canonical document is changed.

Focused save:

```bash
cd /home/dev01/Projects/merlin-worktrees/qt-session-shell &&
tools/git/git-save.sh --commit-msg "Feature: Share Designer worker input children
- Reuse Request and Options ownership visitors
- Preserve containing roots and existing values
- Tests: source checks; runtime unexecuted"
```
