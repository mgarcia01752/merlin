# MERLIN branding assets

Use the wide logo pair for documentation and installer banners, and the square
icon pair as the source artwork for future application/installer icons.

| Purpose | Light theme | Dark theme |
| --- | --- | --- |
| Wide primary logo | `merlin-logo-light.png` | `merlin-logo-dark.png` |
| Rounded README logo | `merlin-logo-light-rounded.svg` | `merlin-logo-dark-rounded.svg` |
| Square application icon | `merlin-icon-light.png` | `merlin-icon-dark.png` |
| Detailed square emblem | `merlin-emblem-light.png` | `merlin-emblem-dark.png` |

`merlin-logo-dark-alternate.png` preserves the alternate supplied wide artwork.
All original PNG pixels are preserved. The README SVGs embed their matching PNG
and apply an 85-unit rounded clip at 1774×887; rounding works without GitHub CSS
support. The README displays them at 520 px and selects by system theme, with a
light fallback. Regenerate embedded SVG artwork when replacing a primary PNG.

The existing package staging copies `docs/images/`, so these assets accompany
CLI packages already. Application icon registration and native ICO/ICNS size
variants belong to later GUI/installer work; the current CLI has no desktop icon.
