# UI-RXMER-SELECT-01 — Select captures and show playback capacity

Keep an imported RxMER collection intact while choosing which originals drive
the next run. Single Shot selects exactly one capture. Replay uses checked
captures in the displayed order, with the existing hold/interpolation policy.
Average remains a separate static error-power combination.

Persist optional selected original SHA-256 identities in measurement options;
missing selection means all originals for existing workbenches. Never infer
scientific selection from the row used to preview a capture. Reordering preserves
selected identities; deleting selected originals removes them from the draft
selection. Apply commits and Cancel preserves the accepted configuration.

Rust owns identity validation, selected playback construction and capacity.
The UI passes accepted CWB run options and OCD to a bounded query. For a temporal
replay with N complete observation symbols and interval I, capacity is
`1 + floor((N - 1) / I)` because the last transition must be strictly before N.
Use the core duration timing API, not duplicated timing math. Show loaded and
selected counts, CWB duration, interval and reachable capacity, including clear
guidance when the selection exceeds the observation window. Duration is per
profile; a selected profile count does not multiply the observation window.
Unconfigured workbenches request CWB setup rather than inventing a duration.

The 256-frame, 64 MiB and 4-million-value limits remain. Identity options have a
bounded 24 KiB IPC budget to accommodate 256 hashes. No whole captures cross QML.

Ownership: measurement Rust model owns selection and persistence; Qt adapter
owns the bounded capacity query; measurement dialog/tree owns checkbox drafts
and chart preview. Tests cover sparse selection, single shot, replay order,
unknown/duplicate identities, legacy defaults, save/reload, exact window
boundaries, Cancel and QML controls. Verify native dark/light layouts and update
the measurement user guide.

## UI-RXMER-ANIM-01 — Animated capture preview

Follow PNM Viewer's `RxMerPlaybackControls.qml` interaction: Previous, Play/Pause,
Next, 1/2/4/8 frames per second and stop at the final frame. Add a position slider
and selected-frame count. Reuse its GPL attribution; adapt to MERLIN's shared
action buttons. Do not change the reference repository.

The Rust measurement model resolves checked identities and returns one bounded
original-capture viewport with source index/time and frame position per request.
Animation steps through those originals in displayed order; it does not rebuild
an average or the whole scientific playback plan on each tick. Charts retain
the current zoom while advancing. Mark this as capture preview: display frames
per second is independent of the OFDM-symbol simulation schedule. Single Shot
has one frame; Average remains a static effective model, while its selected
originals can still be previewed. Effective RxMER returns to that configured
model. Editing selection/mode, inspecting a row, closing/hiding the dialog or
reaching the last frame pauses animation. Preview never applies inputs or starts
a simulation; frame counts/identities and decimation remain Rust-owned.

Test sparse frame order, viewport bounds, unknown/out-of-range selections,
Play/Pause/end/restart, slider/step controls and close cleanup. Capture native
dark/light preview layouts and document the distinction from simulation replay.

Reference inspected read-only: PNM Viewer Toolkit
`apps/qt/qml/RxMerPlaybackControls.qml` on 2026-11-11. SHA-256:
`ea52f62814cfa78f6671812c47dc41a4708a362ba43feccc1faca45779b87667`.
Local working-tree evidence, not a claim of exact upstream HEAD equivalence.
Original GPL copyright retained; theme/button types and first-frame requests are
adapted for MERLIN, with a separate position slider and Rust-selected frames.

## Implemented and validated — 2026-11-11

Optional selected identities persist in measurement options; legacy projects
still select all originals. Single Shot radio buttons, replay checkboxes and
Select All/Clear Selection/Use First N retain the original collection. Rust
resolves selection for preview, publication and core playback validation, and
reports CWB capacity using the strict observation boundary. The editor blocks
Apply when its checked selection exceeds that capacity; future CWB changes are
still revalidated by normal run admission.

The PNM Viewer transport adaptation and position slider animate originals via
one bounded Rust viewport query per frame. Source hashes/times/counts come from
Rust, zoom persists, and paused/closed editors do not advance. Pane widths stay
stable as the frame counter and timestamp change. This does not add capture-time
paced playback or smooth between-frame interpolation to the animation; the
existing effective model and simulation retain their configured hold/error-power
interpolation policy.

Qt format/Clippy/build and 51 Rust adapter tests pass. Selection tests cover
original retention, persisted single-shot selection, ordered sparse replay,
unknown/duplicate/empty identities and exact capacity boundaries. Original-frame
preview tests cover sparse order, hashes, count, viewport bounds and 1024-point
reduction. The final 51 QML checks include animation pause/end/restart/step/close,
legacy single-shot radio state, capacity actions and stable minimum-size layouts
in both themes.

Native Ubuntu 22.04 verification selected one capture from 132, then selected
23 for 1 ms/23-symbol CWB timing at a one-symbol interval. Apply and editor reopen
retained 23 checked captures and all 132 originals. Animated frames advanced
both the trace and capture UTC; 16,810 plot pixels changed between retained
frames. A saved-format derivative with those identities reopened in the native
light theme at 960×640. Dark screenshots replace the affected Single Shot and
interpolation images and add selection/animation examples to the
[measurement guide](../../../user/channel-measurements.md).

Local-only scripts, derivative WB and additional screenshots/logs are under
`docs/development/local/responsive-workbench-open/`; no installed release or
simulation execution was part of the native preview verification.
