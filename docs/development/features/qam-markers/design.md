# Received QAM scatter markers

## UI-QAM-DOTS-01: Received I/Q marker shape

Use circular dots for channel-output/raw RX and receiver raw/equalized I/Q.
The shared TX scatter keeps its existing role shapes for ideal mapped symbols;
a received-sample flag selects dots only at the impaired output boundary. Marker
shape is presentation only: preserve raw/equalized coordinates, normalization,
role colors, filtering and bounded Rust selection. This is a scatter plot, not
an intensity/density map. Show the existing retained/displayed limit at the
channel-output tap so a bounded subset is not mistaken for every retained cell.
Validate existing Qt tests/build and inspect actual dark channel-output/RX
screenshots. No DSP, file formats or WB persistence changes are required.

Validation: `scripts/qt-desktop.sh check` passes formatting, warnings-denied
Clippy, 40 adapter tests, native build and 45 QML checks. Existing tap-navigation
checks now verify received-dot routing only at output and unchanged TX markers
before/after the amplifier. Actual native captures show circular raw output
samples and matched raw/equalized NCP dots from the curated Profile3 run; both
user-guide images are refreshed. The native log is empty. The compiled executable
is ready. Test inputs/logs remain local under `docs/development/local/qam-markers/`;
no simulation changes or other-platform runtime validation are claimed.
