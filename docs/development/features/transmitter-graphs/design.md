# UI-TX-GRAPH-01 — Graphical transmitter test points

Preserve the seven upper processing blocks. Replace the lower chips with equal
clickable graph cards: source digital trace, LDPC bit raster, interleaved bits /
label strip, mapped constellation, ordered I/Q sequences, physical carrier map
and SA with a TX Output caption. Keep graph areas and captions aligned. Remove
the redundant separate Spectrum Analyzer chip; SA opens the shared instrument.

Rust derives bounded graph coordinates and readouts from actual retained TX
snapshots. QML renders and navigates; it must not synthesize representative bits
or random points when no capture exists. Preserve nulls, indices and provenance.
Before capture show an empty plot and a concise Start hint. SA may use the
existing transmitter-only preview; the other graphs require main Start.

The stage inspector defaults to the appropriate full graph with optional Details
for exact values and existing bounded pagination. Bits / Labels selects whichever
actual boundary is retained and keeps the separate boundary selector. Ordered
Cells compares captured before/after I/Q sequences, with separate source/channel
symbol identities: do not infer per-cell identities or a complete interleaver
permutation. Physical Bins classifies layout roles using the same core map and
actual snapshot symbol; a Data location may carry transported NCP. Show captured
bin power and role bands for one symbol, not an invented waterfall.

No information/parity split is shown without retained boundary evidence. The
LDPC raster displays actual coded bits and a 0/1 legend. Mapped I/Q uses existing
role-aware reduction and Data green / NCP red. Physical data locations, pilots,
PLC and exclusions have contrasting colors and legends; missing samples remain
gaps, and power is relative sample power, not calibrated dBmV or a PSD estimate.

Own new numerical presentation in `instruments/transmitter/graphs`; bridge only
exposes bounded queries. Reuse one QML graph renderer in preview cards and full
inspectors, the existing constellation renderer and shared SA window. Fetch
previews on open and changed result tokens, not every polling tick. Keep keyboard,
hover details, dark/light themes and minimum-size layout usable.

Validation: independent bit/raster and I/Q coordinate checks, missing values,
role boundaries, rare-role preview retention and payload bounds; QML card
navigation, SA naming, Details, empty-state and narrow-layout checks. Run Qt
format/Clippy/tests/build and capture actual native dark graphs for the guide.
No simulation, FEC, WB JSON or retained-capture contract changes are required.

## Implemented and validated — 2026-11-11

The seven aligned cards now show bounded actual-data previews. Clicking a card
opens its full graph; SA reuses the existing shared analyzer and retains the
TX Output caption. The stage inspector keeps its boundary selector, paging and
optional exact Details. Canvas lifecycle repaint handles first-open previews.
The user guide includes fresh native screenshots of the cards, source trace,
LDPC raster, ordered cells, physical bins and SA.

Qt formatting, Clippy with warnings denied, 54 Rust adapter tests, 53 QML checks
and the native build pass. Tests cover coordinate/index preservation, missing
values, physical-role boundaries, bounded payloads, real captured observations,
card navigation, empty states, keyboard activation, both themes and narrow
layout. Native Ubuntu verification opened a saved workbench, ran profile 0 and
checked the actual graphs and shared SA, including first-open previews in dark
and light themes. Installed-package and other-platform behavior were not tested.

Native verification also exposed a background-loader readiness bug: channel
validation read GUI thread-local inputs on the worker. The pure
`channel_model::preview_inputs` now takes the supplied frozen input model, and
the real 132-capture loader regression asserts complete channel readiness.
The parallel loader test uses a bounded 60-second polling deadline with a
1-ms sleep rather than busy yielding against a 10-second deadline. No tests
were skipped. See the responsive Open design for ownership details.

Local logs, the owned-process native helper and intermediate captures live in
ignored `docs/development/local/transmitter-graphs/`; only the reviewed guide
screenshots are tracked.

### Spectrum Analyzer label follow-up — 2026-11-11

Spell out Spectrum Analyzer on its preview card, retaining TX Output below it.
The title stays on one line and scales within the card width instead of
truncating or wrapping, including the supported minimum window width.

Remove the duplicate QAM Analyzer action from Diagnostic Instruments. Mapped I/Q
opens its constellation; the stage selector still provides other captured I/Q
boundaries. The reusable analyzer component remains available internally.

Validation: 53 QML checks pass, including current labels, compact table values,
bounded widths and instrument keyboard routing. The native build passes. Actual
Ubuntu dark screenshots confirm the updated layout; the transmitter cards also
retain light-theme verification. Earlier in this same change, Qt formatting,
Clippy and all 54 Rust adapter tests passed. No simulation or persisted settings
were changed; installed packages and other platforms were not exercised.
