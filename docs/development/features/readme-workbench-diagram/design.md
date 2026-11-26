# DOC-BRAND-01: Theme-aware README workbench diagram

Historical implementation, superseded by DOC-BRAND-05 below. The original
illustrated overview assets have been removed in favor of native captures.

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

## DOC-BRAND-05: Refresh the native screenshot gallery

Replace the illustrated overview with a current native workbench capture,
refresh the four analyzer images, and add RxMER Designer linked to its guide.
Center the gallery titles above their images. Capture the built Qt application
on the local Linux desktop using the shipped single-capture workbench; preserve
the captured pixels and keep staging scripts/output under ignored local/.
Replace shared analyzer assets in place so user-guide links stay valid;
replace the retired transmitter QAM thumbnail with the paired receiver view,
and remove obsolete illustrated overview files after checking references.
Validation covers visual review, image decoding, maintained relative links and
diff whitespace. No application code or full automated test suite is involved.

Completed with six unedited 1210×736 native Linux/X11 captures from the current
built Qt executable. The shipped single example completed all four profiles;
the retained latest profile compared 1,779,000 bits with zero errors. The
Designer screenshot previews a new flat 50-capture sequence with −4 dB tilt,
2 dB ripple and an 18 dB synthetic notch. Draft edits were cancelled; no
workbench or capture input files were saved. The two illustrated overview PNGs
and retired TX QAM thumbnail were removed, and shared user-guide assets were
replaced in place. Architecture ownership is unchanged. Image decoding,
README/affected guide links, centered gallery markup and diff whitespace pass.
The full automated suite was not run for this documentation-only refresh.

## DOC-BRAND-06: Compact workspaces and quick start

Keep README as the product overview. Move its downstream/upstream CLI command
sequence and paired Rust/Python namespace imports into
`docs/user/getting-started.md`, linked from README and the documentation index.
Preserve the existing command arguments, compatibility notes, explicit-run
semantics and non-overwriting output guidance. This changes documentation
ownership, not simulation, public APIs or desktop delivery support.

Reuse the native downstream/upstream PNGs from Qt integration commit
`d66c287870def960146cc8219f765b65dae2e00d`, adding them under
`docs/images/workbench/` without resampling or editing pixels. Display both in
one two-column table at 430 pixels each, retaining click-through access to the
original images. Identify them as integration-branch previews: that branch is
not merged by this change, and the released desktop remains downstream-only.
Keep the previous screenshot for its existing guide consumers.

Owners: edit README, `docs/README.md` and this record; add the getting-started
guide and two PNGs. Architecture:
[workspace guide](../../../architecture.md#updated-workspace-diagram-guide).
Requirement: [DOC-BRAND-06](../../../requirements.md#doc-brand-06).
Task: [DOC-BRAND-06](../../checklist.md#task-doc-brand-06).

Acceptance covers visual/image decoding, exact source hashes, preserved example
blocks, side-by-side image markup, Markdown links/fragments, traceability and
diff whitespace. No application code, dependency, scientific vector, QML or
wheel changes; Rust/Qt/Python suites are not required for this documentation edit.

Implemented and validated: all three original Bash/Rust/Python code blocks remain
verbatim in the new guide; README has no fenced copy-and-paste examples. Existing
overview/API fragment targets remain available. Both 1210×736 PNGs were decoded
and visually inspected; their hashes match the source commit exactly:

- Downstream: `a8497f83fa60f062c337aa3377c9d26184a909ed2003a7e112d17edc34cf0831`.
- Upstream: `5c3ccdb19a31fcf23406d193d2dfc1df220eba3a11da715a725ae85534f1a178`.

Both 430-pixel images occupy the same table row. The oversized Designer preview
uses 700 pixels; its original asset is unchanged. Documentation links/fragments,
integration traceability, generated Python-guide freshness and diff whitespace
pass. Full simulation, native Qt and wheel suites were not rerun because only
documentation and unchanged screenshot assets moved or were added. No branch
merge, application build, release or publication is part of this task.
