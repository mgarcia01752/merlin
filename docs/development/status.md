# Development status

Reviewed against the local source and checklist on **2026-11-09**.
Version source: root `VERSION`, currently **0.1.0**. This describes the current
checkout, not a published release, platform certification or interoperability claim.

MERLIN — DOCSIS OFDM Simulation Toolkit — has grown from an isolated FEC
experiment into a standalone Rust
**downstream OFDM waveform and FEC simulation engine**. Its current milestone is
measurement-driven impairment experiments with actual recovered-payload outcomes.
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
placement or waveform support. The current waveform path uses uniform 256-QAM
and short 16,200-bit rate-8/9 LDPC with BCH 14,232/14,400.

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
| Receiver realism | Full blind carrier/numerology/PLC-phase acquisition and broader estimator uncertainty models; current pilot/CP subset is implemented | WAVE-RX-05 |
| Measurement evolution | Future explicit correlated/burst models and dynamic profile activation; piecewise independent Gaussian epochs and frozen mixed profiles are implemented | NCP-03, future model extensions |
| Coding and profiles | Shortened waveform integration, unverified uniform higher-order coding and external mixed-profile vectors; dynamic profile activation | QAM-03/04, NCP-03 |
| Diagnostic and layout tools | Standalone hard slicer; specification-based additional continuous-pilot placement | QAM-02, LAYOUT-03 |
| Input compatibility | Formal decoded-JSON compatibility/version contract; current JSON import is already implemented | INPUT-02 |
| Independent validation | External full-channel interoperability vectors and Windows/macOS build/runtime CI | QA-05, QA-06 |
| Product interfaces | Native packaging/release automation, optional Python bindings and Qt presentation | QA-07, API-01, UI-01 |
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
remain deferred. Qt source controls remain planned under UI-02. See the
[file-source implementation](features/file-payload-source/implementation.md).

## Shortened-codec milestone

FEC-03 adds variable-length shortened coding around the existing downstream
short16200-bit rate8/9 mother code, retaining all BCH/LDPC parity. Independent
coordinate/parity tests, actual iterative correction, failure decisions and full
mode compatibility are covered. See the [design](features/fec-shortening/design.md)
and [component guides](../user/fec-shortening/README.md). Shortened interleaving,
QAM/NCP placement and waveform admission remain QAM-03; no64800-bit mode or
change to the SRC file-padding policy is introduced.
