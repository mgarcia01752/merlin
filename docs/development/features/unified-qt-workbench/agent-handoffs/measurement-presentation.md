# C5 measurement viewport and capture-tree source preparation

## Scope and contract

Branch `feature/qt-measurement-presentation` starts from accepted primary
`6b32dcfc401929f087c9871933c088a77ac94ec8` in the existing isolated worktree.
The helper branch stays at `15d80e59a00e6fd830d4e9ae7a07cdef291dd4ee`;
frozen upstream tips `6b9519d` and `7a0cd0c` are preserved.

This is bounded implementation preparation for existing
[REQ-UIU-07](../../../../requirements.md#req-uiu-07),
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01) and the frozen
[C5 condition](../session-closure.md#fixed-closure-list), under the
[approved logical presentation contract](../retention-contract-decision.md#approved-presentation-and-channel-boundaries).
Architecture, requirements and checklist were reviewed for correspondence;
the existing C5/SESSION acceptance state requires no change in this source unit.

Exactly three paths are owned: `MeasurementViewer.qml`,
`MeasurementCaptureTree.qml` under `apps/qt/qml/downstream/measurements/`, and
this unique handoff. No other measurement owner, shared component, bridge,
registration, scientific API, test, manifest or canonical record is changed.

Both consumers import `../../shared/presentation` and consume the lead-supplied
`PresentationOwner` contract: required `theme`, optional plain `initialValue`
consumed and cleared at completion, readonly `value`, `replace(candidate): bool`,
`clear()`, and readonly bounded static `rejected`/`message`. The owner atomically
admits a measured candidate while previous coverage remains held, then adopts it;
a rejected replacement leaves `value` unchanged. The lead owns its factory at
`theme.presentationBackendFactory`, fail-closed behavior, application limits,
open/retained-view slots and the actual reservation implementation. This patch
does not implement that seam or choose ceilings.

## Consumer behavior

`MeasurementViewer.viewport` is now readonly and binds the guarded bundle's
`viewport`. The same bundle retains accepted animation mode, selected index,
metric, symbol, selection count and the actual files/options references.
`showFrame`, both `refresh` branches and `reset` return the admission result.
No failed replacement changes the displayed viewport/context. Backend request
operations and scientific arguments remain the existing `preview_frame`/`view`
calls. No scientific validation, coercion or reconstruction was added.

`animationActive` is readonly accepted display state. `stopPreview()` stops
playback and the separate `previewRequested` mode; it preserves accepted display
state until a successful replacement. Consequently a hidden preview can retain
its actual animation context without continuing playback. The title and frame
count follow the accepted context. Playback/position changes suspend on rejected
or mismatched context; ordinary refresh remains available for recovery. Static
feedback binds directly to the supplied owner. Plot bindings still share the
accepted viewport rather than receiving independent copies.

`MeasurementCaptureTree` admits one bundle containing expansion, precomputed
plain rows and the exact files/index/selection context. Incoming `files` and
selection properties remain direct parent aliases. Context changes coalesce
through `Qt.callLater`; rows are reused for selection-only changes and rebuilt
for new files or expansion. `ListView.model` reads admitted rows, so measurement
occurs before Qt model conversion. Existing row strings and metadata formatting
remain unchanged. Existing zero-argument `rows()` remains callable.

Rejected input-context replacement preserves the old rows, expansion, highlight,
checkbox/radio interpretation and original file-count context. Actions disable
while that context differs from incoming bindings, so old indices/identities
cannot act on new files. Expansion-only rejection keeps the valid old context.
Checked bindings are restored after click signals, avoiding an unchecked local
control toggle becoming authoritative after parent admission rejection.

The bundle references preserve actual parent aliases without cloning scientific
data. Because these aliases can outlive parent replacement, their explicitly
retained occurrences are measured conservatively along with independent row
wrappers, strings, maps and context wrappers. This makes no physical-allocation
deduplication claim. No mutable map or candidate array is edited after adoption.

Both owners call `clear()` at destruction. The tree stops scheduling refresh
before clearing and checks readiness before a deferred adoption. Visibility and
close do not clear retained payloads. Common checked-open slots and the four
other measurement owners remain lead-owned.

## Executed source checks and pending validation

Executed only source review and Qt formatter parsing, using the existing
read-only SDK `/home/dev01/Projects/raven/local/qt/6.8.3/gcc_64/bin/qmlformat`.
The formatter exited zero for each final QML file. Its output was captured and
not written back, preserving the existing surrounding compact layout; this is
a syntax/formatter parse receipt, not a claim of whole-file format equivalence.
`git diff --check` passed. Python integration guide `--check` reported current;
the authorized save helper must leave the generated guide unchanged.

Formatter SHA-256:

```text
6a83f4367c098e1f0069ce88483f5dec76817054db8bef7efa63eb14241230fc
```

Final consumer source SHA-256 values before save:

```text
apps/qt/qml/downstream/measurements/MeasurementViewer.qml
25180b4d11eef3ea3b75a160483ea0b52ee625434739ae03cb327b3048a03029
apps/qt/qml/downstream/measurements/MeasurementCaptureTree.qml
832f4dd0cb7601ffb8fc439fec5ba3019a5f0578c0e9ef8e30159733f2305b51
```

`PresentationOwner.qml` and its import directory are absent from this branch's
base, so no runtime test or resolving lint was attempted against an invented
stand-in. All QML runtime, native, Cargo, Clippy, full regression and application
acceptance gates are **UNEXECUTED** here. No tests were added under this scope.
Historical helper tests are not evidence for these new consumer bindings.

Lead runtime validation must cover unavailable factory/cold initialization,
exact and aggregate old/candidate rejection, viewport/reset mode preservation,
new-files/selection rejection with old rows and disabled stale actions,
checkbox/radio rebinding after rejected parent updates, expansion recovery,
hidden retention, deferred refresh/destruction and final alias release. The
shared owner must preserve supplied reference identity through `value` for the
context comparisons; independent deep copies would violate this consumer seam.
**C5 and SESSION remain open.**
