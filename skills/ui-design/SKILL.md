---
name: ui-design
description: Apply MERLIN desktop UI conventions when creating or reviewing Qt dialogs, editor actions and workbench layouts.
---

# MERLIN UI design

Keep it simple (KIS): use short, familiar action labels and remove redundant
words when the window title already supplies context.

For editors that apply a draft, use `ApplyCancelFooter`: exactly **Apply** then
**Cancel**, left to right, in one aligned row at the lower right. Give both
buttons equal width and height, consistent spacing and matching edge margins.
Keep secondary actions to the left; never put Apply inside the form while Cancel
is in a separate footer. Do not rely on platform button-role ordering.

Apply preserves validation and accepts the draft; Cancel and the title X dismiss
without applying, preserving each editor's cleanup. Read-only windows retain
Close. Keep keyboard access and disabled states intact.

Follow the broader UI rules in [coding-style](../coding-style/SKILL.md), including
per-window QML files, symmetry, native file pickers and current dark user-guide
screenshots. Review existing Apply buttons when changing this shared pattern.

## Required action-button style

Use `apps/qt/qml/components/WorkbenchActionButton.qml` for ordinary action
buttons in every frame: inputs, workbenches, instruments, settings and About.
Reuse its rounded corners, theme colors, sizing, padding and hover/pressed/focus/
disabled states; do not introduce plain `Button` controls or copied decorations
for actions. Use the regular size by default, with equal dimensions for peer
actions and consistent spacing. Use the shared compact variant only for dense
symbol controls or the fixed-size menu update chip.

`WorkbenchDialog` supplies styled standard Close/Yes/Cancel buttons. Draft
editors use `ApplyCancelFooter` with the owning theme and preserve Apply then
Cancel. Keep existing signal handlers, button roles, shortcuts, focus behavior,
validation and disabled guards. Never make appearance changes alter accepted
settings. Native OS file pickers retain their native buttons.

Diagram blocks, TP circles, stage symbols, tree disclosures, tabs and selection
controls retain their distinct functional shapes. Profile/carrier toggle buttons
reuse the action base while retaining checked state and legend colors. The
Update chip retains its green identity and menu size. Ordinary instrument action
chips use the shared style rather than the stage decoration. Verify dark/light,
keyboard and disabled states, narrow layouts and native screenshots after a
migration; update the relevant user guides.

## Technical terms and definitions

When adding or changing technical UI/status wording, update the existing
[terminology guide](../../docs/user/terminology.md) with a short plain-language
definition and link to the relevant user guide. Reuse About → Terminology; do
not create competing glossaries. Explain what the user observes and distinguish
simulated duration, symbol counts and wall elapsed time. For example, document
interleaver drain and how depth changes its tail without promising proportional
wall runtime. Trace implementation-specific numbers to code; do not label them
as specification requirements without specification evidence.

## Contextual help and optional chart details

Known warning/error states must provide actionable hover and keyboard-focus
help, linked by stable diagnostic code to the shared
[help catalog](../../resources/help/diagnostics.json) and bundled
[FAQ](../../docs/user/faq.md). Keep the original message visible; unknown
errors must not receive guessed advice. Support click and Enter/Space/F1
access to offline details through About → FAQ. Compute context and valid
recommendations in Rust from the actual accepted settings, including boundary
and UI-limit checks. Help must never alter validation or apply settings.

Keep optional acquisition metadata outside the plotting area, below its axes.
Provide a simple Details checkbox to hide it without changing data, warnings,
power readings or navigation. Test placement, hiding and keyboard access.
