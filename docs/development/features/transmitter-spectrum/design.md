# Transmitter readiness and OFDM spectrum

UI-TX-03 now routes Transmitter through its staged
[workbench](../transmitter-workbench/design.md), with the SA attached to TX Output.
The direct-open behavior described below records the original preview integration.

## UI-TX-01: Accepted-input readiness

Transmitter becomes ready when accepted OCD, DPD and PRBS-TX form a supported
Rust request. Missing prerequisites are yellow; unsupported/inconsistent accepted
settings are red with a diagnostic. Accepted input changes invalidate the previous
spectrum. Rejected drafts preserve it. The first preview follows the existing
live descriptor contract: Profile A ID 0; other IDs are not silently relabeled.

Source/profile adapters publish immutable typed snapshots to one GUI-thread
workbench session. The transmitter freezes these with OCD settings; QML never
relays payload/profile arrays. One single-channel workbench is supported.

## UI-TX-02: Bounded TX sample spectrum

Clicking Transmitter opens a reusable Spectrum Analyzer (SA) dialog and generates a bounded
Rust-worker preview, independently of the full-loop Start gate. Reuse source,
BCH/LDPC, bit/cell interleaving, NCP, PLC, pilots and WaveformTransmitter's IFFT,
prefix, window and overlap. No receiver/channel impairment runs. Prime history
with idle/filler before consuming source bits. Do not repeat or auto-fill files.

Use the existing safe Rust FFT on contiguous actual TX output. Average 32
periodic-Hann windows with 50% overlap in linear power. FFT size is 8192/4096,
sample rate 204.8 Msamples/s. For unitary FFT Y, window energy U and sample rate
Fs, PSD is `N * |Y|² / (Fs * U)`. Its integral gives window-normalized sample
power. Units are normalized complex-baseband PSD, not calibrated dBm/dBmV or RF
compliance. Hann/averaging are application choices, separate from the unchanged
PHY I20 §7.5.8 transmitter window (text evidence, PDF pp135–136; no new rendered
PDF verification is claimed).

The signal is complex baseband I/Q; no RF upconversion occurs. The default
frequency axis is centered at DC, spanning -Fs/2 through Fs/2 minus one bin.
An optional capture-frequency mapping labels those same bins using the profile's
physical frequency origin, without changing any samples. The reusable
SpectrumAnalyzerDialog/SpectrumPlot and Rust spectrum estimator are independent
of TX orchestration. Each adapter supplies its tap identity and sample snapshot;
the first connected tap is `tx.overlap_add.samples`. Future channel/RX taps must
retain their own reference plane, units, sample identity and cancellation owner.

Retain bounded samples/full PSD in Rust. Viewports contain at most 2048 min/max
buckets preserving narrow peaks/notches, plus formatted axes/readouts. Qt draws
traces and transient zoom/selection only. At most one worker runs; Cancel/close
requests cooperative cancellation, progress is polled, and stale results discard.

```mermaid
flowchart LR
    INPUTS["Accepted OCD / DPD / source"] --> REQUEST["Frozen Rust TX request"]
    REQUEST --> TX["Existing coding, signaling and OFDM TX"]
    TX --> SAMPLES["Actual samples before channel"]
    SAMPLES --> FFT["Rust Hann / FFT / averaging"]
    FFT --> VIEW["Bounded min/max viewport"] --> QT["Qt analyzer"]
```

Validation: independent tone/Parseval checks at both spacings, demo profile/source
generation, cancellation and immutable/decimated result checks; scoped adapter/QML
checks and dark VM screenshots. UI-TX-01 and UI-TX-02 are implemented.
See [validation](validation.md) and the [SA user guide](../../../user/spectrum-analyzer.md).
Full-loop jobs, additional taps, continuous acquisition, calibrated RF levels and
other Ubuntu runtime targets remain separate work.

## UI-SA-03: Empty lower panel

Historical intermediate layout; UI-SA-04 below now occupies this space.

Remove visible selection readout, trace explanation, sample bookkeeping,
completion status and baseband disclaimer from below the plot. Reserve that
space as an empty panel for future work. Simulation results remain unchanged
in Rust. Selection still anchors zoom and supports the plot's accessibility
description; generation diagnostics are available on the generate button's
tooltip/accessibility description. No numerical or source lifecycle changes.
Validate the shared SA layout with Qt checks and updated actual VM screenshots.

## UI-SA-04: Symbol timing and cable echo budget

Original proposal: use the reserved lower panel for a selected-symbol timing overview,
a CP/roll-off zoom, actual sample trace and optional window overlay. Mark the
symbol and useful FFT starts. The first graphic illustrates timing/window shape
only, not an acquired waveform. Keep actual samples, window coefficients, symbol
identity and derived values in Rust; Qt draws bounded viewports.

For this windowed TX geometry, the ideal flat guard budget is
`Tguard = (Ncp - Nrp) / Fs`, before reserving receiver timing margin. This is an
engineering derivation from the non-overlapping flat segment, not a normative
guarantee of echo immunity. PHY I20 §7.5.8.1, PDF p135, delegates window insertion
to §7.4.10.1; §7.4.10.2.1, PDF p70, explains the shrinking flat interval. Evidence
was read from the pinned text extraction; no rendered PDF check is claimed.

For uniform cable, `Lextra = c * VF * Tguard`; a two-way reflection between
discontinuities separated by D has `D = Lextra / 2`. This distance is not plant
reach or a general distance-to-fault result. Mixed segments require summed
propagation delays. Unknown topology/VF must remain unknown. Echo amplitude,
equalization, timing error and precursors affect actual receiver performance.

Cable selection must identify manufacturer/model and editable nominal VF; do not
assign one universal VF to RG-6 or RG-11. Examples:
[Belden 1694WB](https://www.belden.com/products/cable/coax-triax-cable/75-ohm-coax-cable/1694wb)
uses nominal VF 0.82;
[CommScope P3 500](https://www.commscope.com/globalassets/digizuite/114447-p360-5341603-external.pdf)
uses 0.87. With CP512/RP128 at 204.8 MS/s, the ideal budget is 1.875 µs:
1512.2/1604.5 ft extra path, respectively, or 756.1/802.2 ft two-way spacing.
The compact window-timing panel, cable selector and shared Rust echo-budget API
are now implemented below. Actual I/Q waveform inspection remains pending; do not
label the coefficient plot as an acquired signal.

### UI-SA-04 implementation approach

Fit a compact two-column timing/window and cable-budget panel below the PSD.
Reuse TransmitWindow coefficients, not a decorative signal trace. The chart
shows CP-start zoom with a full-symbol timing summary; explicitly label window
coefficient rather than voltage. Reusable Rust waveform timing models own all
geometry, normalization, µs/feet conversion and input validation. The Qt adapter
uses the result's frozen settings, never current drafts, and returns a bounded
display document. Before generation/invalidation, hide stale timing. Cable model
and optional custom VF are display-only; neither reruns DSP nor alters samples.
Use nominal VF 0.82/0.87 source references above and permit custom values in (0,1].
Zero reserved timing margin is explicit. Test independent arithmetic, input bounds,
all allowed CP/RP pairs and Qt presentation/VM sizes.

UI-SA-04 completion: implemented SymbolTimingPanel.qml with the shared Rust
waveform timing API and a focused Qt display query. Completed results retain
their own TransmissionSettings; display never borrows a newer draft. Independent
core tests, Qt checks and actual VM normal/minimum-size review pass. See
[validation](validation.md#ui-sa-04-integrated-timingwindow-and-cable-panel) and
[user guide](../../../user/spectrum-analyzer.md#symbol-timing-and-cable-echo-budget).
