# UI-FEC-02: Readable FEC Monitor

Implement the approved dark design: profile selector, run state, TotalCW/CCW/UFEC
cards, codeword outcome stack, actual iteration bars and whole-run payload
recovery. Separate Control Channels and Codewords tabs; retain Close and X.

Rust owns scope, counters, ratios, bounded per-profile snapshots and histogram
normalization. Observation counters remain distinct from whole-run decoding,
verification and iteration statistics. Show real histogram bins with occupied
range; never copy the mockup's illustrative distribution. Empty and partial runs
remain explicit. Record retention stays capped at64 words per profile and32
profiles; original large sample snapshots remain in their existing owner.

Recovered payload graphics must account for rejected words and independent
source mismatches, rather than coloring all completed attempts as successful.
Codeword details use aligned columns and an Errors Only filter; disclose omitted
records. Selection never starts a run or changes accepted configuration.

Owners: Qt receiver FEC provider, existing profile retention, receiver-instruments
bridge and focused QML FEC components. Core FEC algorithms/counters stay unchanged.
Validate bounded graph values, profile switching/reset, empty/partial/error states,
QML interactions, dark native screenshots and the latest compiled Qt executable.

## Implementation and validation

Implemented in a focused receiver/fec provider plus three dedicated chart/table
QML components. Profile retention keeps the small FEC snapshot with the existing
profile result and clears it on new input/Start. Historical latest-result fec()
queries remain available. No decoder/core algorithms or saved WB contract changed.

The adapter suite passes37 tests, including all four independently run profiles,
matching retained FEC counters, bounded histogram/word retention and recovery
partition totals. QML passes38 checks, including profile switching, exact counters,
actual-bin delivery, tab navigation, Errors Only and zero/empty/partial state.
Final scoped FEC tests additionally cover rejected and independently mismatched
words, internal histogram gaps, zero iterations and empty denominators.

A real native VM run with captured profile0 completed and displayed82 observed
codewords (81 clean,1 corrected,0 UFEC),86 whole-run recovered payloads and zero
compared bit errors. Its actual histogram includes85 zero-iteration words and
one one-iteration word; zero iterations are genuine decoder acceptance. Dark
Overview, Control Channels and Codewords screenshots are in the user guide.
Native logs contain no Qt/QML warnings. The compiled executable includes the
current layout. Hosted CI and installed packaging are outside this local check.

Final warnings-denied Qt Clippy and all three focused FEC provider tests pass.
Rust formatting and changed documentation links pass. No non-Qt workspace
rebuild or full core test run was needed.
