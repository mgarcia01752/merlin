# MERLIN branding assets

Use the wide logo pair for documentation and installer banners, and the rounded-square
icon pair for application/installer icons.

| Purpose | Light theme | Dark theme |
| --- | --- | --- |
| Wide primary logo | `merlin-logo-light.png` | `merlin-logo-dark.png` |
| Rounded README logo | `merlin-logo-light-rounded.svg` | `merlin-logo-dark-rounded.svg` |
| Rounded application icon | `merlin-icon-light.png` | `merlin-icon-dark.png` |
| Detailed square emblem | `merlin-emblem-light.png` | `merlin-emblem-dark.png` |

`merlin-logo-dark-alternate.png` preserves the alternate supplied wide artwork.
The approved light application icon was replaced with a light-theme derivative
of the dark icon on 2026-11-09. On 2026-11-10 both application icons received
the same transparent rounded silhouette following their outer border. The original
interior artwork is preserved in the new `merlin-icon-*-rounded.svg` source wrappers;
the PNGs are rasterizations of those wrappers. The two PNG alpha masks match exactly. Other supplied
PNG assets remain unchanged. The README SVGs embed their matching PNG
and apply an 85-unit rounded clip at 1774×887; rounding works without GitHub CSS
support. The README displays them at 520 px and selects by system theme, with a
light fallback. Regenerate embedded SVG artwork when replacing a primary PNG.

The existing package staging copies `docs/images/`, so these assets accompany
CLI packages already. The Qt shell embeds the rounded pair for window and side branding. A checkout-local
desktop launcher uses the dark icon; system registration and native ICO/ICNS
variants remain later packaging work. The CLI has no desktop icon.

The shared icon clip is x32/y42, 1192×1154 with 300-unit corner radii on the
1254×1254 canvas. The source SVGs embed the original unrounded artwork, preserving
independent light/dark interiors. Re-render them with an SVG-capable native renderer
when updating PNGs; do not repeatedly mask an already masked raster.
