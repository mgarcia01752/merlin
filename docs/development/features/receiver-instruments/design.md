# Receiver instrument completion

Tasks UI-QAM-01/02, UI-CH-EST-RX-01, UI-SLICER-01 and UI-FEC-01 finish the
receiver instrumentation listed after the first closed-loop UI integration.
The runnable transport remains Profile A/ID 0 and known-boundary timing; adding
instruments does not promise arbitrary RF acquisition or new coding modes.

## UI-QAM-01/02: matched NCP and retained selection

Extend the existing independent coordinate inverse in the Rust paired collector.
NCP cells are classified only by the CRC-accepted received field's consumed-cell
count and reversed received-hypothesis positions. Pair the original raw FFT and
equalized samples through the same inverse transport; never regenerate received
points from decoded labels or transmitter reservation lengths. Collection is
opt-in, bounded and independent of payload capture completion. Retain actual
post-descrambling soft values for eligible data/NCP cells with their coordinates.

The shared Qt analyzer selects actual retained profile, compatible modulation,
physical-bin range and receive-symbol range. Empty selections clear the chart.
Pilot and PLC data/preamble streams remain separate with their actual scaling.
Controls query one immutable result and never rerun the simulation. Missing unretained
history stays unavailable; the bounded capture limits are visible.

## UI-CH-EST-RX-01: response and echo

Retain bounded receiver H-hat snapshots through the existing response test point;
show supplied reference H separately. Rust computes gain, phase, real/imaginary
response and a band-limited impulse response using the shared safe IFFT. Missing/
excluded carriers are masked, with the bandwidth/window effect labeled. Delay
and optional velocity-factor distance are diagnostic estimates, not cable reach
or guaranteed echo protection. Never substitute supplied H under an H-hat label.

## UI-SLICER-01 / UI-FEC-01

Use the existing Rust nearest-symbol slicer on selected retained equalized cells,
keeping this diagnostic separate from production soft decoding. Show bounded
cell decisions/residuals and actual data/NCP LLR values with explicit post-
descrambling reference. Retain the payload decoder-input LLR test point for a
separate coding-order view. PLC/pilot cells have physical decisions; a missing
LLR boundary remains explicitly unavailable rather than being synthesized.

Provide dedicated response/echo, slicer/LLR and FEC dialogs, all using the shared
X header plus footer Close. Make FEC Monitor easy to locate and display existing
actual payload, NCP and PLC counters, iteration distribution and word outcomes.

## Ownership, lifecycle and validation

Core collectors/numeric operations stay Apache Rust; GPL Qt adapters own bounded
queries and display state. No Python, copied viewer implementation or new DSP
in QML. Existing session invalidation clears every result; all windows share
bounded immutable data, including cancelled/failed partial results.

Test independent NCP coordinate attribution, invariance against disabled capture,
known delayed-tap impulse vectors, selection rejection/empty states, actual
slicer/LLR values and native Qt interactions. Run affected core checks once per
simulation change and scoped Qt checks thereafter. Update architecture, both
checklists and dark actual VM user guides. Preserve unrelated pending edits.

## Implementation and evidence

The matched collector adds optional `capture_ncp` and independent NCP retention
(up to8,192 pairs), preserving the existing default payload-only path. Data and
NCP records retain actual post-descrambling LLRs. The Qt run enables65,536 payload
pairs and8,192 per protected stream; queries return at most2,048 scatter points,
128 decision rows and32,768 likelihoods. Received profile controls expose actual
admitted IDs; they do not expand the current Profile A/ID0 transport.

Core `receiver::diagnostics` owns response/impulse calculations and LLR statistics.
Qt's `receiver_instruments/selection.rs` and `diagnostics.rs` query the same
immutable result. Four response snapshots and one16,200-value LDPC-input window
are retained through registered test points. Per-stage event caps keep already-
full sample streams from consuming the global1,048,576-element probe budget.
Dedicated response, slicer and FEC QML files replace the old placeholder dialog.

The existing NCP decoder's CRC acceptance and consumed-cell contract define role
admission. Placement provenance remains PHY I20 §7.5.14.4, PDF page163 (local text
extract reviewed; no new rendered-PDF verification). No TX reservation or label
is used as receiver classification. Impulse processing uses the existing centered
unitary IFFT with explicit1/sqrt(N) conversion to channel-tap coefficients and
zero filling of masked bins; its peaks include band-limit ringing.

Scoped validation: all12 targeted core tests passed across `channel_playback`,
`ncp_constellation`, `paired_constellation` and `receiver_diagnostics`. These cover
independent inverse-coordinate attribution, disabled-capture summary invariance,
noise/FEC preservation, exhausted/failed observers, stage budgets, direct known-
tap delay/amplitude vectors for both FFT sizes and actual LLR statistics.
Warnings-denied core Clippy passed. Qt checks passed13 Rust adapter tests and25
QML tests, including a real measured run, actual NCP selection, empty selection,
cell/decoder LLR queries, missing-H-hat rejection, FEC parity and invalidation.
The final display-only axis adjustment was rebuilt and passed Qt Clippy.

Actual Ubuntu22.04 VM evidence: a pilot-estimated1ms run compared1,223,952 bits
with zero errors. It retained2,544 NCP cells, accepted25 NCP fields and recovered
86 payloads. Response snapshots, impulse view, matched NCP plots, empty-range
clearing, actual decision/likelihood views and the16,200-value decoder window were
inspected. The final FEC/Slicer layouts also fit960×640; the new slicer and
likelihood display were inspected in light mode, then restored to dark. Native
logs remain free of QML warnings. FEC Statistics reports real branch counts and64 word rows with22
omitted. These results are a supported internal simulation, not RF acquisition or
external interoperability evidence. Other native targets remain unverified.

Dark actual screenshots are in the [receiver guide](../../../user/receiver-workbench.md).
Local logs and disposable WB files remain under ignored `docs/development/local/`.
The user's wording update replaces authored documentation prose by context with
simulation, signal processing or numerical terms, preserving source identifiers,
canonical skill paths and pinned specification text.
