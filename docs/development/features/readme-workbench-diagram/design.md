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

## DOC-BRAND-02: Concise repository introduction

Keep the product purpose, theme-aware diagram and documentation entry points in
README. Remove the redundant development/platform paragraphs, implementation
status table, local trial commands and simulation-flow section. The existing
channel guide already documents that flow; link it and development status rather
than duplicating them. Replace CMTS-like with a direct description of the CMTS
downstream PHY functions actually simulated, including waveform generation,
receiver decoding and independent comparison. Preserve single-channel/supported
PHY boundaries and the exclusion of full CMTS MAC services. Align architecture
scope wording without changing implementation or claiming interoperability.

Implemented as documentation-only edits. Preserve both theme diagrams and the
requested opening paragraph. Validate relative links, removed headings and diff
whitespace; no compilation or simulation tests are needed.

## DOC-BRAND-03: Documentation entry points

Replace README's flat Documentation link list with UI Guide, Rust API Guide and
Python Guide. Add docs/README.md as the grouped index for CLI, simulation/reference
and contributor links. Keep existing guide paths stable; clarify the Rust API
introduction and add a short Python landing page stating that bindings are
unimplemented and deferred until after UI-01 through UI-04. Link the proposed
MerlinSimulation design without presenting its sketch as a working import.
The Measurements and demos section remains unchanged.

Implemented as documentation-only navigation changes. Verify relative links,
README entry points and diff whitespace; no compilation or runtime tests needed.

## DOC-BRAND-04: Analyzer gallery

Add a compact two-column README gallery linking current native dark screenshots
to the SA, transmitter QAM, receiver response/echo and RxMER guides. Reuse the
reviewed user-guide assets without cropping, recoloring or making new mockups.
Each thumbnail opens its instrument guide. Keep captions short and identify the
images as actual native UI captures. No UI source or processing behavior changes.
Validate image integrity, local paths, heading anchors and diff whitespace; no
compilation or runtime tests are needed for reusing unchanged screenshots.
