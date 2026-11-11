# UI-HELP-01: Contextual diagnostics and offline FAQ

Known workbench admission issues carry stable codes, contextual help and a
bundled FAQ target. Keep original messages, unknown errors and validation
behavior intact. Start with RxMER playback exceeding the source observation
window; do not claim every historical warning has a catalog entry.

The core emits `PlaybackObservationTooShort` at the existing validation guard.
The Qt Rust adapter preserves that code through profile admission, computes a
minimum whole-millisecond duration from the core's symbol/sample timing and
formats catalog templates. Require a complete symbol after the last transition;
recommend a duration only if it fits the existing 1–1000 ms UI range. Otherwise
recommend reducing interval/capture count or using a static mode. The host
runtime budget is separate. Do not re-parse English error messages or compute
physical timing in QML. A catalog/help failure must not change run admission.

`resources/help/diagnostics.json` owns static help copy and FAQ links.
`docs/user/faq.md` owns longer explanations and is embedded in the executable.
Rust whitelists topic keys and extracts only the matching bundled section.
`ContextualStatus` supports hover/focus tooltips and click/Enter/Space/F1 details;
FAQ dialogs retain X and Close. About → FAQ opens the offline index. Known help
is bounded transient presentation state, never WB JSON. Original diagnostics
remain visible if help is unknown or unavailable.

Owners: core channel validation, Qt help/admission adapters, shared status label,
FAQ dialog and menu wiring. Verify core error code and unchanged guard, exact
boundary recommendations, multiple capture counts/numerologies, over-limit and
unknown errors, catalog/FAQ consistency, invalidation, keyboard/hover and offline
navigation. Run affected core checks plus the full Qt check/native build. Capture
actual dark UI and update guides, architecture, checklist and UI skill.

## UI-SA-DETAILS-01: Spectrum acquisition readout

Move sample-rate/FFT/window/average/bin/ENBW text out of the plotting rectangle
to a wrapping strip below the frequency axis. A Details checkbox in the analyzer
header controls this optional readout. Keep it on initially and remember the
choice for that instrument window; this is transient display state, not WB JSON.
Power and warning labels remain independent. Preserve spectrum values, axes,
selection and navigation. Verify placement, toggling and native dark captures.

## Validation

Ubuntu VM, Qt 6.8.3: Rust formatting and affected-core Clippy pass;
three channel-playback tests pass. Full Qt check passes formatting, Clippy,
44 adapter tests, native build and 46 QML checks. Native XCB screenshots
verify metadata visible/hidden below axes, baseband/capture mapping, wheel
zoom and a real two-capture admission failure opening the bundled FAQ.
No waveform, FEC or channel impairment algorithms changed.
