# UI-TOOLTIP-01: one shared tooltip delay

Use `Theme.tooltipDelayMs = 1000` as the single application-owned delay.
Every attached tooltip and the explicit contextual-help tooltip binds to it;
shared action buttons also supply this default for future consumers. Preserve
text, timeout, accessibility and click/F1 actions. Use visualFocus for keyboard
help on Controls, keyboard focus reasons on text inputs, and keyboard-only
focus on help labels/custom items. Mouse-click focus cannot retain a hover
trigger after pointer exit.
Qt owns the delay timer and cancels pending display when hover/focus ends.
Native operating-system dialogs are outside MERLIN's QML controls.

Validate all QML tooltip declarations for the shared binding, test actual delayed
appearance and early-hover cancellation, run Qt checks and rebuild the native UI.
This timing-only change does not change screenshots or scientific behavior.

## Validation

On Ubuntu with Qt 6.8.3, `scripts/qt-desktop.sh check` passed formatting,
Clippy, 83 Rust adapter tests (50.47 seconds), the native debug build and
82 QML checks (21.891 seconds). The tooltip regression checks cover delayed
appearance, mouse-click focus, cancellation on exit, a fresh wait on re-entry,
and hiding an already visible tooltip for both attached and explicit help.
All 42 attached triggers bind to the shared delay. UI skill validation and
discovery links passed. Windows runtime behavior remains unverified locally.
