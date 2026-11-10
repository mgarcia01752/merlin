# UI-VERIFY-02: Recovered-data summary and graphs

Restyle PRBS-RX into a clear comparison status panel, four numeric summary cards
and two bounded charts: recovered/unrecovered generated payload counts and bit
errors by retained received payload. Use real Rust results only; do not invent
BER history or classify unreturned payloads as compared bit errors. Zero compared
bits keep BER unavailable. Complete, cancelled, runtime-exceeded and failed runs
remain distinct; running/paused status must not relabel an old result current.

`VerificationDialog.qml` owns layout and existing one-way comparison acceptance.
`VerificationBarChart.qml` renders normalized Rust-supplied coordinates; a focused
Qt Rust formatter adds chart data to the existing bounded FEC query. Keep at most
64 word bars and explicit omitted-detail count; scalar totals remain complete.
Opening/changing the panel does not rerun or mutate the engine. Preserve X,
footer Close, accepted configuration and result invalidation.

Also remove the left icon/WB gutter (UI-SHELL-11), keeping the top logo and native
menus. Use equal 24-pixel side margins for the main workbench content.

Validate Qt formatting/Clippy, affected adapter tests and QML lifecycle/empty/
partial-result cases. Build with retained artifacts; review actual dark VM
results and minimum-size layout. Refresh the workbench and run/save guides.
No core signal-processing change or non-Qt workspace checks are needed.

## Implementation and validation

The focused GPL Qt formatter under `receiver_instruments/verification.rs` supplies
exact count labels and bounded normalized coordinates through the existing FEC
query. Uncompared retained records are excluded from the word-error chart and
reported separately; zero-error outcomes stay on the zero baseline. Display
changes preserve one-way comparison acceptance and existing engine behavior.

The Qt check passed formatting, warnings-denied Clippy, 14 Rust adapter tests and
26 QML checks. Coverage includes zero/mixed count formatting, the actual measured
closed loop, chart count parity/bounds, cancelled no-comparison results,
running/paused state transitions, stale-data clearing and popup dismissal.
The latest QML-only text adjustment was rebuilt and passed the 26 QML checks.

Actual Ubuntu 22.04 VM evidence: the saved 1 ms pilot-estimated example run
completed with 1,223,952 compared bits, zero bit errors, 86 recovered payloads
and zero unrecovered generated payloads. The chart retained sequences 0–63,
reported 22 omitted details and no retained uncompared records. Reviewed the
empty view, actual dark/light result panels and the 960×640 scrollable layout;
X and footer Close remain accessible. Restored dark mode and normal window size.
The latest executable is running and the native log has no QML warnings.
Updated actual dark screenshots appear in the workbench and run/save guides.
Local logs are under ignored `docs/development/local/visual-workspace/verification-*`.
No non-Qt workspace checks were run; other target runtimes remain unverified.
