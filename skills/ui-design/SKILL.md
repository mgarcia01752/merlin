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

## Technical terms and definitions

When adding or changing technical UI/status wording, update the existing
[terminology guide](../../docs/user/terminology.md) with a short plain-language
definition and link to the relevant user guide. Reuse About → Terminology; do
not create competing glossaries. Explain what the user observes and distinguish
simulated duration, symbol counts and wall elapsed time. For example, document
interleaver drain and how depth changes its tail without promising proportional
wall runtime. Trace implementation-specific numbers to code; do not label them
as specification requirements without specification evidence.
