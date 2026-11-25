# UIU-FILE-01 dirty-project prompt presentation

Status: isolated source preparation; native integration remains root-owned.
This bounded unit implements only the compiled Save / Discard / Cancel prompt.

## Traceability and ownership

[REQ-UIU-22](../../../requirements.md#req-uiu-22) owns active-direction saving,
target-tab Open guards and failure/cancel preservation.
[REQ-UIU-24](../../../requirements.md#req-uiu-24) preserves Rust/Python contracts.
[UIU-FILE-01](../../checklist.md#task-uiu-file-01) depends on UIU-API-05,
UIU-SHELL-01 and UIU-US-03. See the canonical
[session/file architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle)
and [project design](design.md#uiu-api-05-direction-aware-public-project-admission).
This prompt neither serializes projects nor changes any public API.

| Disposition | Exact file / existing owner | Responsibility |
| --- | --- | --- |
| Add | `apps/qt/qml/shell/ProjectDirtyDialog.qml` | Compiled prompt and once-only outcomes. |
| Add | `apps/qt/tests/shell/tst_project_dirty_dialog.qml` | Focused presentation/lifecycle fixtures. |
| Reuse | `shared/components/WorkbenchDialog.qml` | Existing visibility admission and close cleanup. |
| Reuse | `shared/components/WorkbenchActionButton.qml` | Existing action styling/accessibility. |
| Reuse | `shared/presentation/PresentationOwner.qml` | Admit compiled display strings before publication. |
| Defer | Root native file/revision/intent owners | Canonical correlation, chooser, save/open/new/close. |

## Component contract and lifecycle

The caller supplies theme, direction (`downstream` / `upstream`), action
(`open` / `new` / `close`) and `saveAllowed`. Call `openChecked()`; invalid tags,
existing prompts or rejected presentation/visibility admission return false.
The dialog captures no document, path, native revision or arbitrary context.
Only compiled labels and admitted direction/action tags are retained.

Save/Discard require the still-matching displayed tags; Save also requires the
current `saveAllowed`. Cancel is the safe initial focus/default action.
Escape, header X and direct closing choose cancellation when unresolved.
A fixed private outcome prevents duplicate or reentrant dispatch.
After actual close and inherited visibility cleanup, one deferred signal emits:
`saveRequested()`, `discardRequested()` or `cancelled()`.
Root retains and validates its own exact native revision/intent independently.

The explicit typed presentation owner avoids implicit Dialog content ownership.
Closed strings clear after bindings observe the cleared owner; destruction also
clears it. No editable text, custom limits or retention framework is introduced.
Actions use equal shared buttons and stack in narrow viewports. Geometry names
are display constants; colours/palette follow the supplied dark/light theme.

## Validation boundary

Focused fixtures cover all compiled tags, Save-disabled behavior, Cancel default,
keyboard traversal, Escape/X/direct close, once-only/reentrant outcomes, tag drift,
reopen, presentation/open pressure, actual release, dark/light and narrow layouts.
Every `_data` provider has a matching test; no standalone test ends in `_data`.
Checks ran only after the root released its validation slot.
Native chooser, writes, two-tab dirty guards, scientific replay and public project
compatibility remain UIU-FILE-01 integration gates, never fixture acceptance.

Executed with read-only Qt 6.8.3 GCC SDK: `qmlformat`, diagnostic-free `qmllint`,
and this file's offscreen/software/Fusion QtTest run: 35 functional cases plus
setup/cleanup, 37 passed records, no failures, skips or warnings. Test discovery
was checked against source names. Dark/light wide/narrow images were inspected.
Ignored receipts: `docs/development/local/unified-qt-workbench/file-dialog/`.
No Cargo/native/full-suite build or test, commit or registration was performed.
