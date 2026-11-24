# Measurement dialog presentation preparation

This is source-only preparation for the existing C5 condition in
[SESSION closure](../session-closure.md),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), the
[session architecture](../../../../architecture.md#unified-qt-session-and-file-lifecycle)
and [UIU-SESSION-01](../../../checklist.md#task-uiu-session-01).
C5, C6 and the official SESSION task remain open. No new accounting category,
scientific API, scientific limit or saved-workbench schema is introduced.
Canonical documents and task status are lead-owned and unchanged.

## Source and scope

Prepared on `feature/qt-measurement-dialog-presentation`, directly from
`f34190608d05af6bd2eb10deae18e5ecfbb65f94`. Exactly these five paths belong
to this preparation:

- `apps/qt/qml/downstream/measurements/MeasurementInputDialog.qml`
- `apps/qt/qml/downstream/measurements/MeasurementPlaybackControls.qml`
- `apps/qt/qml/downstream/measurements/RxmerDesignerDialog.qml`
- `apps/qt/qml/downstream/measurements/RxmerPnmExportDialog.qml`
- This handoff.

The existing Viewer and CaptureTree preparation at `f341906` is unchanged.
The helper at `15d80e59a00e6fd830d4e9ae7a07cdef291dd4ee` and upstream
branches `4f350a2`, `6b9519d` and `7a0cd0c` remain frozen. Existing Qt GPL
notices, including the PlaybackControls PNM Viewer attribution, are preserved.
No source is imported from another project in this unit.

## Lead-owned integration seam

The four consumers import `../../shared/presentation` and use the lead-owned
`PresentationOwner`: required `theme`, consumed `initialValue`, readonly `value`,
`rejected`, `message`, `textAdmitted`, atomic `replace(candidate):bool`,
`clear():bool`, and `reserveTextFields(actualFields):bool`. The application
supplies `theme.presentationBackendFactory`. Missing admission fails closed.
The source branch intentionally does not copy the helper, native counter,
factory, shared component or registration from primary.

The shared owner must reserve candidate UTF-16/entries while old coverage is
held, retain old value on rejection and block nested replace/clear. Editable
text uses the same native counter for a separate complement based on the
actual Qt `maximumLength` values. Its coverage survives DTO clear and hide
until disposal; validators and maximumLength values are unchanged here.
`WorkbenchDialog.openChecked()` is lead-owned and gates opening with the
separate retained/open-view counts. The two owned open call sites use it.

Lead-owned `RxmerDesignScope` supplies `reserveText():bool`, readonly
`textAdmitted` and `restore(value):bool`, preserving its existing aliases and
wire value. Designer preflights all four scope reservations before beginning
restore, checks their restore results, and requires all four admitted states
for preview/generation/export readiness. Successful preflight remains valid
through synchronous restore because coverage lasts until disposal and the
actual maximumLength values do not change. This relies on that shared seam;
no cross-owner transaction framework is introduced. After DTO adoption, source
and capture count are restored before scope.restore so its existing all-scope
last-capture fallback sees the restored count.

## Actual retained consumers

| Owner | Guarded state and adoption point | Lifetime and rejection |
| --- | --- | --- |
| Input `adoptState` | One metadata/options/selectionStatus/admission/error bundle; plain candidates are computed before replacement. Existing derived selection/filter arrays are inside the candidate. | Parental `runOptions`, files/options consumers and child plots remain aliases. Rejection preserves the previous bundle and viewer context. Index-dependent native edit actions and opening Designer are disabled after rejection; Open/Restore/Clear remain recovery paths. Actual successful Apply still emits and closes if optional error display admission fails. |
| Designer `replaceState` | Information, preview viewport, numeric-text edit rows, error and diagnostic move together. Restore prepares the canonical Rust Request preview before publishing this bundle, then restores existing controls. Add/remove creates a new measured edit array rather than mutating the retained one. | Preview and parentOptions bindings remain aliases. Working/completed state follows actual stage/poll responses even if optional details reject. Hidden/closed data stays retained. |
| Export `showSnapshot` / `replaceDetails` | Document snapshot string, capture/preview context, summary, message, diagnostic and saved path share one guard. Snapshot is admitted before checked opening; detail updates preserve admitted snapshot context. | Busy/completed/saved state is consumed before optional detail admission. Rejected saved details retain a bounded success message and suppress stale summary display. Save readiness requires admitted details. Existing native cancel cleanup stays on close. |
| Playback `refreshLabels` | Plain speed labels plus frame index/count and derived frame text are admitted before ComboBox model conversion. The speeds array is reused as a binding alias after adoption. | Rejection stops playback and blocks frame actions while preserving the old bundle. A rejected synchronous frame request cannot immediately toggle playback back on. No Qt sequence is measured after model conversion. |

Each owner clears on destruction. Closing/hiding keeps DTOs and editable text
covered; no early release is claimed. Native parented adapter destruction is
the final reservation owner. Old/candidate overlap is delegated to the shared
atomic replace operation, rather than inferred from synchronous JSON returns.

Designer `stagedAdopter(metadata)` is optional. Existing standalone consumers
with no callback still receive `staged(metadata)` and close as before. The Input
parent installs an adopter returning its actual boolean `setMetadata` result.
Rejection leaves Designer open with static feedback, emits no accepted staged
signal and does not discard or cancel completed backend science. Acceptance
emits the existing signal and closes. This is the explicit parent-adoption
acknowledgement missing from the previous void signal.

## Editable text and wire preservation

Text reservation covers exactly 21 existing fields: Input's interval, preview
symbol and tolerance; Designer's count, baseline, random deviation/seed, tilt,
ripple, spacing, distance, velocity factor, notch center/width/depth/edge and
three value-edit fields; Export's first/last. The three Input fields now have
IDs for reservation. Failed admission leaves these controls blank and disabled.
The eight scope fields are lead-owned; their four admitted states participate
in Designer readiness. There are no new maximumLength or validator bounds.

Derived edit records retain numeric text so an editable NaN/Infinity does not
make the logical plain-value counter silently reject or coerce scientific
input. `editText` stores the original Number/Math.round results as strings;
`editValue` converts only in the temporary existing recipe output. Finite
numbers round-trip through String/Number. Negative zero becomes positive zero,
but both serialize as `0` through the existing JSON.stringify wire path.
NaN and either Infinity reconstruct the original nonfinite Number and serialize
as `null`, as before, reaching the existing Rust diagnostic. Scope endpoints
have the same treatment; absent endpoints stay absent. No fallback, finite
filter, new rounding or scientific validation is added.

Read-only review of `bridge/rxmer_designer.rs` and the existing Designer Request
owner verifies that `designer_begin` returns its typed request and
`designer_preview` deserializes that same request schema. Restore now previews
that canonical request before control assignment; normal preview/stage/export
still use the existing document conversions. Whole JSON key order is not
claimed identical. Real bridge restore parity and numeric-error wire comparison
remain unexecuted integration gates, including nonfinite drafts and negative
zero. Existing scientific constants/defaults, validators, steps and conversion
operations are retained.

## Source evidence and pending acceptance

Executed: Qt 6.8.3 `qmlformat <file>` parsing of each of the four owned QML
files, without rewriting formatting; all four returned exit 0. `git diff
--check` passed. Source review covered every owned diff and the staged/untracked
scope. `tools/python/update_integration_doc.py --check` reported current. The
local save helper is authorized; its guide refresh must leave the guide hash
unchanged.

The formatter is the read-only SDK artifact
`/home/dev01/Projects/raven/local/qt/6.8.3/gcc_64/bin/qmlformat`, SHA-256
`6a83f4367c098e1f0069ce88483f5dec76817054db8bef7efa63eb14241230fc`.
No target/cache or rendered output was created. QML runtime tests, qmllint,
Qt/native builds, Rust/Cargo/Clippy, Python/wheel and full regressions are
**UNEXECUTED** for this unit. The required shared seam is absent from this
frozen-base branch; the lead owns combined adoption and runtime validation.

Required focused integration cases: unavailable factory and retries; exact and
one-beyond logical/text/view limits; old/candidate overlap; rejected candidate
preserving matching context; hidden data and final disposal; text rejection
before any field restore; all four scope restore results; parent staged
accept/reject/default-signal behavior; real Request restore parity and
NaN/Infinity/negative-zero wire parity; successful terminal generation/save
under detail pressure; and playback rejection without a new timer run. No
prepared case is reported as executed or accepted.

Source SHA-256 before save:

| File | SHA-256 |
| --- | --- |
| MeasurementInputDialog.qml | `4d03365e1e12196ff5b9e3f9b97d465a4fea60c453d4f2693ebe4acd977018d9` |
| MeasurementPlaybackControls.qml | `fbadc7759d80d5f27397061da5f8931c7f15d699c3c851ac831e31e1ad0b57f3` |
| RxmerDesignerDialog.qml | `c6e01bb10df716abe77b411ad1a4928aad818e184a854e408beb24031b77b0e4` |
| RxmerPnmExportDialog.qml | `baed29bfbc1b5e503eddd6e548cd9515533891a0eb4558d7bd07d4324082374a` |

The unchanged Python guide SHA-256 before the helper is
`ea767a75658ff4b8b8121aedf751a4e2cfc10ebbbc08a0384c874e4a55501591`.
