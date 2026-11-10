# DOC-BRAND-01: Theme-aware README workbench diagram

Add the supplied `docs/images/logo/merlin-toolkit-diagram-dark.png` to README
with a matching light PNG selected by a standard picture element. Preserve the
dark original bytes and both images' aspect ratios; use a responsive1100px
display width rather than resampling the original. Link the workbench user guide
and identify the illustration separately from actual native UI screenshots.

Create the light raster as an edit of the supplied image using the built-in
image generation tool: same text, nodes, icons, arrows and layout, light surfaces,
navy text and teal accents. User authorization for this light variant overrides
the default dark-only documentation-graphic convention for this pair.

This is documentation/branding only. Validate picture paths, exact dark SHA-256,
image integrity and visual text/layout; no Qt build or DSP tests are required.

Implemented with the built-in imagegen edit tool and saved the light result beside
the original. The README uses theme media queries with a light fallback and1100px
display width. Source dimensions remain1672x941 (dark) and1671x941 (light), without
cropping or resampling. Both layouts and all block/PNM labels were visually checked.
The dark SHA-256 remains
`4b1de0c066f38239bf4032f14110ee70dc6d727bcf372ff46c45261e97008a1d`.
