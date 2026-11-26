# C5 transmitter presentation consumers

Source preparation for UIU-SESSION-01 C5 / REQ-UIU-07 under the lead-owned
[design](../design.md#uiu-session-01-c5-existing-persistent-presentation-owners).
Base: `6b32dcfc401929f087c9871933c088a77ac94ec8`.
This preparation does not close C5 or SESSION.

## Exact scope and interface

Only `TransmitterStageDialog.qml`, `TransmitterQamDialog.qml`, `TapQamDialog.qml`,
`TransmitterMessageDialog.qml` and this handoff are owned here. The lead supplies
`theme.presentationBackendFactory`, `PresentationOwner` and shared
`WorkbenchDialog::openChecked()` plus common ReceiverCellControls/TlvTreeGrid
complements. Those dependencies are read-only and are not copied into this branch.

Each existing DTO replacement is pre-admitted while its old response stays held.
Response/context/locally derived selector/display strings form one coherent plain
candidate. Readonly aliases share that owner's witness, including plot bindings;
rejection preserves the accepted response/context and shows bounded feedback.
Initial defaults use `initialValue`, consumed and cleared by the shared owner.
Hidden/closed data retains its witness. No scientific API, algorithm, saved schema,
backend query arguments, validation limits or plot values change.

- Stage: original DTO, stage/page, selector labels and summary/page/omission text.
  Failed refresh restores the accepted stage/page.
- TX QAM: original DTO, signal selector/labels and original summary. Failed refresh
  restores the selector; legend context follows the accepted signal.
- Tap QAM: original DTO, tap/name/query and original title/summary/axis text. Failed
  refresh restores the accepted tap/filter context without intermediate dispatch.
  The writable `tapName` request interface remains: its current ChannelWorkbench
  caller supplies exactly three compiled labels (Before Amplifier, After Amplifier,
  Channel Output / RX Input). This finite alias assumption is not an arbitrary
  dynamic-string contract. Displayed title/name always comes from admitted context.
- Messages: original document plus selector/status/byte-row/pending strings. The
  packet/TLV bindings alias the original document. The local byte ListView now
  reads the admitted preformatted strings; their values are unchanged. Selection
  is clamped only after successful document admission.

Tap's two local SpinBoxes call `reserveTextFields` on their actual TextInput
`contentItem`s at completion and retry on open. `textAdmitted` gates editing,
controls and dynamic displayed text. Existing maximumLength/domain validators are
unchanged. This separate conservative text complement survives DTO clear/hide
until owner disposal. ReceiverCellControls' own fields/derived models remain the
lead's shared complement, and are not duplicated here.

## Lead integration hooks

Supply the shared presentation import and injected native factory. These four
owned files contain no dialog-open invocation; parent TransmitterWorkbench and
ChannelWorkbench callers must use `openChecked()` (lead-owned). Existing mutable
request selectors/tapName remain; observation/document are now readonly admitted
aliases. Tests that inject DTOs directly must instead use a backend response and
refresh. No direct caller property assignment is needed in production.

## Source checks and remaining gates

All four files parsed with exit 0 and empty stderr using the existing read-only
Qt 6.8.3 `qmlformat --ignore-settings FILE`; output went only to ignored receipts.
This is syntax parsing, not import/type checking or runtime acceptance. Source
review preserves the original backend calls, scientific values, null handling,
selector labels, numerical limits and actions. `git diff --check`, raw-NUL and
exact five-path/unstaged/untracked reviews are recorded in the receipt.

Receipt directory:
`docs/development/local/unified-qt-workbench/transmitter-presentation-6b32dcf/`.
It contains parser commands/results, source SHA-256 hashes, the source diff,
allowlist/static checks and final commit receipt. Canonical architecture/checklist
remain lead-owned and SESSION unchecked. Qt runtime, Cargo/native/build/Clippy,
wheel and full-suite checks are UNEXECUTED here; lead owns actual helper/native
integration, rejection/success/selector/text/open-slot tests and final C5 evidence.
