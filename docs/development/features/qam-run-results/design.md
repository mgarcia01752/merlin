# UI-QAM-RUNS-02 — Run identity and transmitted-reference MER

The channel 193 multi-rxmer-2 example exposes two different ambiguities: the
QAM profile selector displayed the independently simulated wire Profile A/ID0,
while each completed run replaced the previous detailed constellation. Dense
4096-QAM can also produce an optimistic nearest-reference estimate after real
RxMER impairments move samples across decision boundaries.

## Actual transmitted reference

Retain the actual physical TX I/Q at each captured physical bin and channel
symbol. Keep it as optional diagnostic metadata on data/NCP/protected pairs,
never an argument to an RX stage, equalizer, decoder or decision. Carry data/NCP
references through the same diagnostic inverse interleaver as the existing
raw/equalized coordinates. Protected references attach after receive capture.
The optional full TX grid is copied only when payload/NCP capture is enabled;
no capture means no new TX-grid allocation. Existing RNG and science are unchanged.

The public Rust streaming paired MER operation sums the actual reference power
and squared error over the complete retained selection. QAM shows this as
**MER (TX reference)** separately for Raw and Equalized. The PNM-derived nearest
ideal point estimate remains visible as **Nearest-point estimate**, with its
severe-noise limitation. Missing TX references stay unavailable. No received
samples are fitted, rescaled, clipped or rejected to improve the reading.

## Independent profile results

Run Profile means original captured profile ID; the optional Wire Profile
filter remains a separate concept and is shown only when multiple wire IDs
exist. A latest-run choice preserves full latest capture resolution. Each of
up to 32 selected runs also archives evenly spread actual cells: at most 8192
data, 1024 NCP and 1024 per protected stream. This bounds archives to 425984
cells plus 518400 decoder LLRs. There are no archived waveform buffers, source
bytes or whole job results. The displayed retained count identifies the actual
archive population; MER never uses only the 2048 rendered dots.

Selection resolves one exact archive, preserving cell time/bin/modulation,
reference and actual LLR identity. It never pools independent runs or falls back
to another run when one is missing. Imported but unrun IDs remain visible with
No Result and an actionable explanation. Start and accepted-input invalidation
clear archives with profile performance. QAM, channel-output and slicer/decoder
queries honor the same run choice. Spectrum and channel-response instruments remain
latest-run scoped; FEC/Performance keep their existing profile selectors. Workbench JSON, imported captures and simulation selection
policy do not change; a new run is needed to populate diagnostic results.

## Validation

Verify matched-reference inverse transport and capture-disabled science
invariance, wrong-decision MER vectors, every available completed run, archive
bounds, no fallback for unavailable IDs, filter/empty states and invalidation.
Run the actual 132-capture WB with its original RxMER and with an explicit
20 dB synthetic perturbation of that data to distinguish TX-reference MER from
nearest-point bias. Keep this test diagnostic and preserve source captures.
Use scoped core tests/Clippy, full Qt checks, latest native build and actual
Ubuntu dark/light captures. No complete simulation permutation sweep or
installed/other-platform validation is implied.

The no-result state uses stable code `QamRunUnavailable` in the shared offline
help catalog and FAQ. Contextual status provides hover/focus advice and click,
Enter/Space/F1 navigation from QAM and slicer dialogs. Unknown errors retain their
original message without inferred advice.

Native review of the unchanged channel-193 measurements identifies captured IDs
0, 3 and 4. In Average mode, the original data gives about 44.93 dB Raw and
43.95 dB Equalized TX-reference MER, agreeing with the prior nearest estimate in
this particular low-error sample. Thus the original RxMER was already applied;
45 dB is its finite modeled impairment level, not an ignored input. Averaging
uses linear error powers, not arithmetic dB. The new 20 dB perturbation test
separately demonstrates the estimator's severe-noise bias and checks the raw
measured power against the playback plan's predicted linear error power.

## Completed validation

Fourteen scoped core cases pass across constellation MER, paired constellation,
NCP capture and receiver diagnostics, including exact reference transport and
capture-disabled science invariance. The five MER cases also pass without default
features. Core all-target warnings-denied Clippy passes. Qt formatting, Clippy,
65 Rust adapter tests, 55 QML checks and latest native build pass. Regression
coverage includes original channel193 playback power prediction, synthetic 20 dB
noise, four independent run archives, bounds/empty reduction, no fallback,
invalidation, run selection, paired labels and F1-to-FAQ routing.

Owned Ubuntu native dark/light runs load the original 132-capture example in
Average mode with all three profiles. Only the development fixture's decoder
iteration limit is reduced to five to bound visual QA; original captures and
shipped WB settings are unchanged. Latest Profile4 shows 4096-QAM, and selecting
archived Profile3 shows its own 2048-QAM cells. Profile0 is also inspected.
The measured/nearest numbers agree for the original good-MER data; the injected
20 dB case verifies the difference under severe noise. Native logs are free of
QML warnings, and updated guides include the actual views. Scripts, fixtures and
intermediate captures remain under ignored `docs/development/local/qam-run-results/`.
No full permutation sweep, installed package or other platform was tested.
