# UI-SHELL-06: Popup close controls

Add an upper-right circular X to every MERLIN-owned popup dialog, including
nested instruments, configuration editors, About, licenses and file feedback.
Retain existing footer Close/Cancel/Apply controls. Native OS file choosers retain
the operating system's window controls; menus and dropdown lists are not dialogs.

Use one `WorkbenchDialog.qml` base with a shared title header and accessible,
keyboard-focusable close button. Each window keeps its own QML implementation,
content, footer and lifecycle handlers. X rejects/dismisses the current popup;
it never applies draft settings. Existing onClosed cancellation stays in force,
and closing an analyzer does not stop the main closed-loop job.

Validation: exercise actual X clicks across the block dialogs and nested
instruments, verify draft cancellation and parent-popup preservation, run the
Qt presentation checks, build the current executable and inspect dark/light
rendering on the VM. No Rust signal-processing code or calculations change.

## Completed implementation and validation

All 16 direct dialog implementations now inherit `WorkbenchDialog`; RxMER and
Channel Estimate also inherit it through `MeasurementInputDialog`. Their original
footers and lifecycle handlers are unchanged. The shared title keeps long labels
elided so they cannot overlap X. Hover, focus and text colors use the active theme.

All 24 QML tests passed. Existing interaction tests now click X on the nine block
dialogs, payload QAM/RX SA and nested About/license dialogs, verify the parent
remains open, and prove an OCD draft is discarded. The retained footer controls
remain in their owning dialogs. The native Qt build passed using retained Cargo
outputs; no simulation crate test or rebuild was requested separately.

Reviewed the actual Ubuntu 22.04 VM: dark receiver and nested QAM X controls,
parent preservation, light Settings, and a 960×640 OCD editor with accessible X
and original Cancel/Apply footer. The native log contains no QML warnings. The
latest executable is built and running. Other Ubuntu targets remain unverified.
The updated coding-style skill passed its validator; both discovery links still
resolve to the canonical skill. See the [workbench user guide](../../../user/workbench.md)
for the actual dark screenshot. Local logs are in ignored
`docs/development/local/popup-close-controls/`.
