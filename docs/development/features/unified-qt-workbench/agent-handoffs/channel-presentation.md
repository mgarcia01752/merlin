# Channel presentation source preparation

This bounded source unit prepares the existing C5 condition in
[SESSION closure](../session-closure.md),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), the
[session architecture](../../../../architecture.md#unified-qt-session-and-file-lifecycle)
and [UIU-SESSION-01](../../../checklist.md#task-uiu-session-01).
C5/C6 and the official SESSION task remain open. The lead owns canonical
records, shared admission/global registration and runtime acceptance.

## Scope and base

Branch `feature/qt-channel-presentation` starts directly at accepted
`6b32dcfc401929f087c9871933c088a77ac94ec8`. Exactly seven existing files
under `apps/qt/qml/downstream/channel/` and this unique handoff are changed:

- `ChannelWorkbenchDialog.qml`
- `ChannelConfigurationDialog.qml`
- `ChannelResponsePreviewDialog.qml`
- `ChannelFilterDialog.qml`
- `ChannelLossDialog.qml`
- `ChannelAmplifierDialog.qml`
- `ChannelAttenuatorDialog.qml`
- `docs/development/features/unified-qt-workbench/agent-handoffs/channel-presentation.md`

Measurement `44e1da8305fd137dc42fd73bf6e7ebbb75ce3a26` and `f341906`,
helper `15d80e5` and upstream `4f350a2`/`6b9519d`/`7a0cd0c` branches are
preserved unchanged. There are no test, bridge, shared component, scientific,
Cargo, registration, architecture, requirements or checklist edits. Existing
GPL Qt notices are preserved; no external implementation is imported.

## Existing shared seam

Six concrete DTO owners use lead-owned `PresentationOwner` with required
`theme`, consumed `initialValue`, readonly `value`, `rejected`, `message` and
`textAdmitted`, `replace(candidate):bool`, `clear():bool` and
`reserveTextFields(actualFields):bool`. The native factory comes from
`theme.presentationBackendFactory`. Missing admission fails closed. Candidate
admission overlaps the old owner before assignment; rejected replacement keeps
old DTOs/context. Clear/hide does not release the separate editable-text
complement; final disposal does. Application ceilings remain lead-owned.

Every new owner is an explicit typed property with a distinct property name
and id. The three new Binding objects use the same explicit-property pattern.
No new nonvisual implicit Dialog contentData child can eagerly create an orphan
contentItem. This follows the concrete parent integration finding; runtime
window attachment remains a lead check.

All owned open invocations use the lead's `WorkbenchDialog.openChecked()` seam,
including its mechanical replacements in Workbench and Configuration. Separate
retained/open-view populations remain with that shared seam. No duplicate view
framework is added.

## Actual owners and replacement points

| Consumer | Guarded state and source hook | Context/lifetime |
| --- | --- | --- |
| Workbench `stateFor`, `refreshPower`, `refreshSpectrum`, `openTap` | Actual native power and tap_state DTOs, result-token string, admitted tap index/name/options, instrument title and amplifier/attenuator labels form one bundle. The three existing tap labels and tap indices are unchanged; indices now have named presentation constants. | `tapIndex` is readonly from the admitted bundle; tap requests enter through existing `openTap`. Rejected requests preserve the old tap/labels/token. On open, power and current tap state are refreshed; QAM/Spectrum opens require admitted current context. Actual settingsApplied still emits modelApplied if optional power display rejects. |
| Configuration `stateFor`, `refresh` | Native preview, existing serialized filter-settings string and independently derived loss/filter captions move together. The existing filter comparison is computed temporarily on runState notification before guarded replacement. | Parent configuration/runState remain aliases. Rejection preserves old preview/captions and shows static feedback. Enlarged Response and its child plot bind the same guarded preview; no second DTO charge is introduced. |
| Response Preview | Existing viewport plus rejection/message are direct parent bindings. | Its own common dialog lifetime is separate from the parent's shared DTO witness. Hiding either window keeps the parent data retained. |
| Filter `restore`, `applyDraft` | Accepted channel_filter snapshot, error and diagnostic share a guard. Existing paths/gain arrays stay within the snapshot; the existing independent path clone is temporary output for Apply. | Text capacity is admitted before snapshot or field restoration. Actual successful native Apply emits configurationApplied and closes even if optional detail replacement rejects. |
| Loss `replacePreview`, `restore`, `refresh`, `applyDraft` | Actual preview/error and derived band/gain/tilt strings share a guard. Restore computes its existing control values and exact existing document before querying/admitting the candidate preview, then restores controls. | Rejection leaves prior preview and controls on restore. Later editable text is covered separately; rejected live preview shows static feedback and blocks Apply. Successful native Apply still emits modelApplied and closes if optional details reject. |
| Amplifier/Attenuator `restore`, `applyDraft` | Actual accepted power snapshot and error are guarded together. Draft text remains in the existing Qt controls with finite text coverage. Existing Object.assign/Number/JSON.stringify wire output is temporary. | Text preflight and DTO admission precede field restoration. Rejection leaves old controls/snapshot. Successful native Apply emits modelApplied and closes independently of optional error display admission. |

DTO owners clear on destruction; hidden/closed DTOs and text remain covered
until actual clearing/disposal. Parent-bound data and plots use their existing
witness. Derived wrappers/labels included in candidate bundles are counted
conservatively as logical occurrences, without a physical-dedup/RSS claim.

Workbench's existing spectrum proxy queries only its admitted tap and requires
the admitted token to match the actual runState token. If the context is stale,
it returns a bounded static unavailable error through the existing JSON response
shape. It does not gate solely on rejected, because a child can query during a
successful retry before the parent's rejected flag clears. Shared Spectrum and
QAM sources are unchanged. Synchronous child state/tap label propagation, nested
binding notifications and successful retry are explicit root runtime checks.

## Text and scientific preservation

Text preflight covers nine existing TextFields: Filter real/imaginary gain;
Loss slope/length; Amplifier gain/reference/target; Attenuator attenuation/noise
density. It also covers the actual contentItem of Filter's three editable
SpinBoxes (echo delay, memory and shift). Calls occur after field creation and
retry before restore/open; controls are blank/disabled until admitted.

The actual Qt 6.8.3 Fusion SpinBox source was inspected read-only: its contentItem
is TextInput, text binds displayText, and its validator is the control validator.
Explicit Binding properties gate that existing contentItem text on admission
while preserving displayText behavior; no replacement styled input or guessed
maximumLength is supplied. Actual maximumLength values are passed to the shared
counter. No maximumLength, validator, SpinBox from/to/value default, scientific
option, step, error policy or native operation is changed.

Loss's documentFor only parameterizes the original mode/text/unit conversion so
restore can prepare the same request before persistent assignment. The original
trim/NaN/Number behavior, manual/cable/off keys, cable identifier and units remain.
Filter's document body, default echo/direct paths and reconstruction tolerances
are unchanged. Power drafts preserve all accepted fields and the same nullable
calibration/target/noise choices. Invalid/nonfinite text still takes the existing
JSON.stringify-to-Rust diagnostic path; it is neither normalized nor validated
scientifically in QML. Accepted configuration and Cancel/close cleanup remain
with the original native owners. Wire/scientific parity is source-reviewed,
not a runtime claim.

## Executed source receipt and deferred checks

Executed: each of the seven files parsed with read-only Qt 6.8.3
`qmlformat <file>` (all exit 0), without formatting rewrites; git diff --check;
complete owned source/diff/untracked review; Python integration guide --check
reported current. The save helper refresh must leave the guide unchanged.
No target/cache, rendered output or generated scientific artifact is created.

Qt SDK `/home/dev01/Projects/raven/local/qt/6.8.3/gcc_64` artifacts:

- qmlformat SHA-256: `6a83f4367c098e1f0069ce88483f5dec76817054db8bef7efa63eb14241230fc`.
- Fusion/SpinBox.qml SHA-256: `5033e95702774781d7c633a911bc426aa5e6dee60e771bae1d9be73d58e574ab`.

QML runtime tests, qmllint, Qt/native builds, Rust/Cargo/Clippy, Python/wheel and
full regression suites are **UNEXECUTED**. The shared owner/open/text seam is
intentionally not copied into this isolated base; root owns combined adoption
and all execution. No runtime case is added or represented as passed here.

Required focused root cases: exact/one-beyond bytes/entries/text/view limits;
old/candidate overlap; unavailable factory and retry; rejected restore with
unchanged fields/accepted state; all three actual SpinBox text edits/validation;
rejected tap/token replacement preserving old plots and labels; synchronous
Spectrum context notifications/retry; enlarged preview parent alias and hidden
retention; native successful Apply under optional detail pressure; unchanged
manual/cable/off, echo/FIR and calibrated/noise wire behavior including invalid
text; checked popup window attachment; close/reopen and final disposal. These
are remaining validation risks, not added acceptance categories.

Source SHA-256 before save:

| File | SHA-256 |
| --- | --- |
| ChannelWorkbenchDialog.qml | `40391f2d2fbb66a05bf530225cd6e793e961374def5f87501370b84589073897` |
| ChannelConfigurationDialog.qml | `81e6438c2d36db213fb1fdeb224cee99689b0a08826826ff63533d60c105c34f` |
| ChannelResponsePreviewDialog.qml | `996590d7e4e27ed4493e5b75b066b1a488ba256e5376d446091bf2ff9cdceb26` |
| ChannelFilterDialog.qml | `00bfe0e117dc3fdbb0d8ed04c8b985de742522a8a0fb71f4949dd8a02d3ac50b` |
| ChannelLossDialog.qml | `a167ecf945efccbc07d2e64c392bcc2c57328b05f1eed944ac32c05375e94757` |
| ChannelAmplifierDialog.qml | `bb9b5c0acfec7dda19480cf50fad5d84db35e292d7e7377b0f120e234612b102` |
| ChannelAttenuatorDialog.qml | `43ecd38a7085a64c48338bbf04929db3f04c32bc0e271223c2c2a0e6eeea1d5f` |

Python guide SHA-256 before the authorized helper:
`ea767a75658ff4b8b8121aedf751a4e2cfc10ebbbc08a0384c874e4a55501591`.
