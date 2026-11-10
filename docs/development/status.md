# Development status

Reviewed against the local source and checklist on **2026-11-10**.
Version source: root `VERSION`, currently **0.1.0**. This describes the current
checkout, not a published release, platform certification or interoperability claim.

MERLIN — DOCSIS OFDM Simulation Toolkit — has grown from an isolated FEC
experiment into a standalone Rust
**downstream OFDM waveform and FEC simulation engine**. Its minimum closed-loop milestone is complete for the documented supported
configurations: MERLIN TX → channel model → MERLIN RX → original-source comparison.
OFDMA and multi-channel simulation remain outside the implemented scope.
Continue work through the [development workspace](README.md), which links public
feature records and defines the ignored local evidence/output location.

## Completed milestones

| Milestone | What is available | Design / evidence |
| --- | --- | --- |
| Scientific foundation | Separate Rust full/shortened BCH/LDPC codecs, constellation mapping and soft demapping through 16K-QAM, bit processing and randomization | [FEC](../user/fec-codec.md), [QAM](../user/qam-mapping.md) |
| Channel construction | Fixed single-channel numerology, protected roles, continuous/scattered pilots, frequency/time cell interleaving | [Transmission](../user/transmission.md), [pilots](../user/pilot-signals.md), [interleaving](../user/cell-interleaving.md) |
| Signaling | Actual PLC/NCP coding and messages; receiver configuration derived from decoded descriptors for the fixed supported subset | [PLC](../user/plc-channel.md), [NCP](../user/ncp.md), [configuration](../user/receiver-configuration/README.md) |
| Waveform TX/RX | IFFT/FFT, cyclic prefix, windowing/overlap and actual uniform-256-QAM payload recovery over samples | [Waveform](../user/waveform/README.md), [payload transport](../user/payload-transport/README.md) |
| File traffic | Exact text/binary bytes, optional finite PRBS15 fill, independent file/fill BER and recovered original-byte export | [File source](../user/simulation/file-source/README.md) |
| Run infrastructure | Continuous PRBS15; configurable duration; acquisition, observation, completion and drain; source-window accounting, cancellation and partial results | [Channel runner](../user/channel-runner/README.md) |
| Observability | Typed stage boundaries, bounded probes/JSONL, standard Rust logging and real decoder statistics | [Probe export](../user/simulation/probe-export.md), [logging](logging.md) |
| Measurement experiments | Current decoded-JSON import, per-carrier Gaussian residuals, sample AWGN, supplied response H and known-reference equalization | [Measurements](../user/measurement-inputs/README.md), [impairments](../user/impairments/README.md) |
| Paired constellation | Same-cell raw/equalized I/Q, payload-only receiver attribution, bounded retention, partial results and shared Rust/CLI export | [Guide and actual plot](../user/impairments/paired-constellation.md) |
| Error-boundary demos | Protected-PLC RxMER stress sweep, observed payload bit errors and exact binary PNM representations of the tested inputs | [Experiments](../user/impairments/experiments.md), [binary demos](../../examples/measurements/rxmer-fec-stress/README.md) |

The isolated coding experiment and complete waveform runner have different
support boundaries. Mapping a QAM order does not establish its full FEC,
placement or waveform support. The channel path supports the documented uniform orders through16K-QAM and
frozen mixed loading, with short16,200-bit rate8/9 LDPC and BCH14,232/14,400;
explicit shortened transport has its separately tested entry point.

## Latest demonstrated experiment

A fixed 3 ms, seed-41 run degraded 1,072 of 7,480 captured grid positions in
700–727 MHz, leaving the full 648–654 MHz PLC neighborhood unchanged. Each case
compared 3,614,928 payload bits. The 12 dB case was error-free; the tested 11 dB
case had 483 post-FEC errors. At 9 dB all 254 payload words reached the
50-iteration LDPC limit. No PLC bit errors or NCP rejection occurred in these runs.

These are reproducible short-run observations, not a universal DOCSIS RxMER or
percentage-of-carriers failure threshold. The wider stress band is synthetic,
not a single LTE carrier. See the experiment guide for the complete results.

## Current observation milestone

**CONST-01..05: paired constellation capture before and after equalization.**
Supplied H and1/H compensation now expose8192 matched payload QAM observations
by default through a bounded Rust collector, direct API and opt-in channel CLI
JSON. The measured demonstration and11dB stress case each exported8192 pairs;
the stress case retained483 errors with the earlier scientific summary unchanged.
See the [implementation record](features/paired-constellation/implementation.md).

## Receiver and measurement milestone

MODEL-05/06/07 now provide Receiver Channel Estimation (CH-EST-RX), bounded
pilot/PLC analyzers, explicit measurement epochs and frozen applied mixed ProfileA
transport. The [implementation record](features/pilot-channel-estimation/implementation.md)
links tests and per-component guides. Pilot-mode RX receives no supplied H.
Bounded CP sample-offset acquisition is separately available; the CLI retains
known timing/PLC phase. No full blind modem-acquisition claim is made.

## Work still open

| Work area | Remaining scope | Checklist IDs |
| --- | --- | --- |
| Receiver extensions | Arbitrary RF acquisition, sample-rate error and severe-channel tracking remain outside the bounded minimum model | Future receiver extensions |
| Measurement evolution | Future explicit correlated/burst models; streaming dynamic DPD/NCP activation is implemented, duration CLI remains fixed-profile | Future model extensions |
| Cross-platform validation | Windows/macOS native builds/runtime after the later workflow push | QA-06 |
| Product interfaces | Native installer lifecycle/release execution; optional Python bindings and remaining Qt instrumentation | QA-07, API-01, UI-01 |
| Later PHY scope | OFDMA, channel bonding/multiple channels and FDX-specific behavior | LATER-01, LATER-02 |

This table groups outstanding work; it does not reorder the official checklist
or authorize implementation. Select the next task using its dependencies and
current priority. Keep broader partially complete coding/acquisition items unchecked.

## Validation and boundaries

The repository has a local pre-push gate for formatting, warnings-denied Clippy,
workspace/analysis-only tests, release CLI builds and smoke checks. Recent fixture
work additionally verified quarter-dB encoding, binary header matching,
non-overwriting output and exact sample equality after external `pnm-decode`.
The receiver/measurement completion run executes this full gate and actual
measured-CE CLI experiments; see its implementation record for results.

The measurement-driven receiver currently uses supplied channel response and
known timing/equalization. RxMER contributes an explicit Gaussian residual model;
it does not reveal real impulse timing, temporal correlation or modem behavior.
No extra thermal floor is silently added. Actual FEC results, analytical profile
screening and measured observations remain separate.

Use the [architecture](../architecture.md) for design and the
[implementation checklist](checklist.md) for individual completion states.

## File-source milestone

SRC-04..06 implement bounded exact-byte files, send-once EOF, optional explicit
PRBS15 append-to-fill, coding-padding exclusion and separate original/fill
verification. The Rust API stays path-independent; CLI file options use the same
contract. Size/alignment requirements inferred from channel/profile/duration
remain deferred. Qt source controls are implemented under UI-02. See the
[file-source implementation](features/file-payload-source/implementation.md).

## Shortened-codec milestone

FEC-03 adds variable-length shortened coding around the existing downstream
short16200-bit rate8/9 mother code, retaining all BCH/LDPC parity. Independent
coordinate/parity tests, actual iterative correction, failure decisions and full
mode compatibility are covered. See the [design](features/fec-shortening/design.md)
and [component guides](../user/fec-shortening/README.md). QAM-03 now adds shortened
interleaving, received-boundary length discovery and explicit waveform transport;
no64800-bit mode or change to the SRC file-padding policy is introduced.

## QAM decisions and coding milestone

QAM-02 adds the diagnostic hard slicer while preserving soft LDPC inputs. QAM-03
implements full uniform rules through16384-QAM, exact padding/soft inversion and
explicit shortened transport through live PLC/NCP and waveform samples. See the
[design](features/qam-completion/design.md), [validation](features/qam-completion/validation.md)
and [component guides](../user/qam-processing/README.md). QAM-04 closes with
complete source-table preservation and supported full/shortened/mixed recovery
within MERLIN. External interoperability is outside minimum scope.

## Downstream completion group

NCP-03, LAYOUT-03 and INPUT-02 now have opt-in dynamic full-word transport,
generated additional pilots and a pinned decoder import contract. WAVE-RX-05 adds
bounded received-only nonideal acquisition. Its later integer-CFO/numerology
completion is recorded below. Each component retains its stated limits;
these do not silently change the fixed-profile CLI channel runner.
QA-05 closes with the documented internal end-to-end comparisons. QA-06/07 tooling is prepared
locally, with Linux package tests; Windows/macOS execution and release publication
wait for the later workflow push. No remote action was performed. QAM-04 also closes for internal validation. See the [design/validation record](features/completion-group/design.md)
and [native delivery guide](native-delivery.md).

## Synthetic channel-width validation

The [width fixtures](../../examples/measurements/channel-widths/README.md) add
24 MHz minimum and 90 MHz mid-width cases at 25 kHz spacing. Both preserve the
existing four profile IDs/data orders (256/1024/2048/4096-QAM), with new valid
pilot geometry, aligned synthetic epochs, flat RxMER and unity channel correction.
All eight combinations are exercised through the noiseless waveform/FEC path.
The captured IDs remain in imported scenarios; fixed runner configurations carry
each selected loading as internal Profile A. This does not expand measured
selection beyond ID 0. See the [design](features/channel-widths/design.md).


## Minimum engine completion: receiver and reference checks

WAVE-RX-05 is complete for a finite bootstrap-hypothesis model: unknown25/50kHz
selection, signed integer/fractional CFO, two-symbol pilot phase and received-pilot
CH-EST-RX. Tests cover24/90/192MHz, actual reflected paths/sample perturbations,
block-split invariance and known coded-block recovery without TX H in RX.
See the [design](features/receiver-acquisition/design.md). The CLI reference path
is unchanged; this is not arbitrary RF channel discovery.

QAM-04 verifies all32720 source-table labels, scaling and supported coding/profile
round trips. QA-05 verifies the complete internal source/control/payload path,
impairments and honest partial results. On2026-11-09 the user clarified that another
implementation is not required; those two items therefore close for the documented
MERLIN-only acceptance scope. No external comparison has been claimed. See the
[closed-loop acceptance record](features/closed-loop-validation/design.md).

## Qt workbench and closed loop (UI-01 through UI-04 partial)

The native Qt workbench now has accepted OCD/DPD/source/measurement editors,
receiver configuration, independent source-comparison acceptance and derived
readiness. Start runs the Rust closed loop; Stop retains completed comparisons,
and Pause/Resume keeps the same engine state. Actual payload/PLC QAM, RX spectrum
and BER/FEC results share one bounded run. NCP matched constellation capture and
broader observation selectors remain pending.

Native File Open/Save/Save As use self-contained versioned workbench JSON with
exact embedded input bytes and checksums. The VM is the interactive review target;
Windows/macOS and other Ubuntu versions are not runtime-validated here. See the
[closed-loop design](features/qt-closed-loop/design.md),
[validation](features/qt-closed-loop/validation.md),
[user guide](../user/closed-loop-workbench.md) and
[UI checklist](ui-checklist.md). No entire UI milestone is marked complete.
