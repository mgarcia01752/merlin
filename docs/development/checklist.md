# Implementation checklist

This is the official progress and TODO list for DOCSIS OFDM Channel Sim. Read it
with the [architecture](../architecture.md). Update both when adding a stage,
changing ownership, exposing a test point or extending scientific support.
Review both at feature completion and before supplying each commit message;
update affected sections and retain unchecked status for partial work.

Checked means present in the current checkout with supporting validation; it does
not mean committed, released or externally interoperable. Unchecked means work
remains. Detailed limitations stay in the stage guides rather than being hidden
by a broad “implemented” claim.

## Architecture and module rules

- [x] **ARCH-01** Maintain one canonical architecture and this checklist.
- [x] **ARCH-02** Separate codec, DSP, transmission, simulation and adapter ownership.
- [x] **ARCH-03** Record stage test-point IDs, metadata and bounded observation design.
- [x] **ARCH-04** Implement the common typed test-point model and opt-in observer.
- [ ] **ARCH-05** Implement bounded stage/range selection and export through an adapter.
- [ ] **ARCH-06** Prove probes do not change RNG draws, state, outputs or decisions.
- [ ] **ARCH-07** Expose parity, twist and demultiplexing as separately callable stages,
  including reverse boundaries, while preserving the existing bit-processing API.

Every new algorithm gets a focused module, typed output and independent tests.
Stateful stages own their state; runners only connect stages. Direct stage outputs
remain usable independently of the observer. ARCH-04 currently registers the four
pilot stages and two PLC label stages. ARCH-05 has bounded stage/element/symbol
selection; adapter export remains TODO. Pilot/PLC observation invariance is tested; ARCH-06 still needs
coverage of stateful/RNG-driven stages as their producers are added.

## Existing scientific foundation

- [x] **SRC-01** Deterministic seeded payload source, separate from codecs.
- [ ] **SRC-02** PRBS source with explicit polynomial, seed, ordering and continuity.
- [ ] **SRC-03** Independent PRBS vectors and recovered-stream comparison.
- [x] **FEC-01** Rust BCH14232/14400 and short16200 rate8/9 LDPC encode/decode.
- [x] **FEC-02** Concatenated codec results, payload truth comparison and actual
  LDPC iterations, with correction/failure and zero-error tests.
- [ ] **FEC-03** Additional downstream codeword lengths, coding modes and
  shortening rules verified against the PHY; the current short8/9 mode does not
  establish every DOCSIS coding mode.
- [x] **BITS-01** Uniform16/256-QAM parity/twist/demux and soft inverse.
- [x] **QAM-01** Embedded label-ordered mapping and max-log LLRs through16384-QAM.
- [ ] **QAM-02** Standalone hard slicer with deterministic ties, nearest label,
  reference I/Q and error/distance output; keep LDPC soft inputs unchanged.
- [ ] **QAM-03** Complete coding/placement support beyond uniform16/256-QAM,
  including verified per-order padding, shortening and non-square rules.
- [ ] **QAM-04** Mixed per-carrier data profiles and independent high-order
  interoperability vectors; mapper availability alone does not satisfy this item.
- [x] **RAND-01** Stateful data/NCP cell randomization and soft inverse, including
  pilot skipping, zero-loaded clocking and128-symbol reset tests.
- [x] **PACK-01** Bounded256-QAM queue, continuation, filler and NCP reservations.
- [x] **NCP-01** Real NCP messages, CRC-24-D, separate control LDPC and QAM grouping.
- [x] **NCP-02** CRC-accepted receiver pointer discovery without transmitter IDs.
- [ ] **NCP-03** Dynamic data/NCP profile switching and session-level signaling
  validation; the current fixed-profile path does not implement this behavior.

Guides: [codec](../user/fec-codec.md), [bit processing](../user/qam256-coding-loop.md),
[QAM](../user/qam-mapping.md), [randomizer](../user/cell-randomizer.md),
[packing](../user/symbol-packing.md), [NCP](../user/ncp.md).

## Fixed channel and cell transport

- [x] **CFG-01** Fixed25/50kHz numerology, up-to192MHz setup, prefix/roll-off validation
  and integer one-second timing calculations.
- [x] **LAYOUT-01** Explicit physical PLC/continuous/scattered/excluded roles.
- [x] **LAYOUT-02** Reduced-domain association and128-symbol inverse pilot reference.
- [ ] **LAYOUT-03** Generate additional continuous-pilot positions from the PHY rules;
  current tests supply explicit synthetic positions.
- [x] **INT-01** Frequency permutation and inverse, full/partial columns and both NI limits.
- [x] **INT-02** Stateful time interleaving and inverse at all supported depths,
  including dummy padding, M-1 pair delay, startup/drain and rejected-call atomicity.
- [x] **INT-03** Physical pilot-reference alignment over the complete128-symbol cycle.
- [x] **CELL-01** Integration tests recovering real NCP and payloads at both spacings,
  maximum time depth and the randomizer reset, with zero payload bit errors.
- [ ] **CELL-02** Promote test-only data/NCP/filler assembly and receive orchestration
  into reusable production modules with selected test points.

These checked items establish cell transport, not an RF waveform. Guides:
[transmission](../user/transmission.md), [layout](../user/carrier-layout.md),
[cell interleaving](../user/cell-interleaving.md).

## Next implementation: pilot and PLC signals

Start here before moving to IFFT. Develop each component separately and keep its
input, output, phase and normalization available for tests. Typed pilot results
and bounded test points are now implemented in separate modules. The [pilot guide](../user/pilot-signals.md) records the verified sequence,
scaling and example. The [PLC message guide](../user/plc-messages.md) now covers
the timestamp/MC framing and FEC-input boundary. [PLC coding](../user/plc-coding.md)
now recovers those inputs. [PLC block interleaving](../user/plc-interleaving.md)
adds tested permutation/inverse stages and bounded label observations.
[PLC randomization](../user/plc-randomization.md) now adds separate TX/RX state,
frame resets and a sign-only soft inverse, with direct typed snapshots.
Next is PLC preamble generation (NEXT-11);
fixed-setting acquisition descriptor packets remain part of NEXT-07.

- [ ] **NEXT-01** Confirm local PHY3.1 I20 section7.5.15 pilot requirements and
  section7.5.13 PLC requirements; record endpoint, sequence/bit order, scaling,
  phase and exact section/page evidence. Check PHY4.0 delegation where applicable.
- [x] **NEXT-02** Define typed pilot sequence/value and PLC stage results, with
  shared test-point metadata and bounded optional observation.
- [x] **NEXT-03** Implement shared pilot sequence logic in its own module and
  verify independent sequence/phase vectors.
- [x] **NEXT-04** Implement continuous-pilot signal generation separately from
  additional-pilot placement; expose signs, I/Q, boost and locations.
- [x] **NEXT-05** Implement scattered-pilot signal generation separately, using
  the existing physical pattern and correct PLC-aligned phase after interleaving.
- [x] **NEXT-06** Implement shared pilot power/boost conversion with explicit
  reference energy; test signs, levels and collisions with protected roles.
- [x] **NEXT-07A** Implement validated timestamp/MC message blocks, exact frame
  capacity and consecutive MSB-first36-byte/288-bit FEC inputs.
- [ ] **NEXT-07** Complete fixed-setting PLC acquisition messages: OCD, Profile A
  and NCP DPD MAC packets, packet CRCs, segmentation and repetition scheduling.
- [x] **NEXT-08** Implement verified PLC coding as a separate stage; reuse a codec
  only where the PHY parameters match, otherwise add a focused Rust primitive.
- [x] **NEXT-09** Implement PLC block interleaving in its own module with
  independent permutation vectors and a coded-to-interleaved test point.
- [x] **NEXT-10** Implement PLC randomization with independent state and its own
  phase/reset tests; do not reuse data/NCP randomizer state.
- [ ] **NEXT-11** Implement the PLC preamble in its own module with phase/sign tests.
- [ ] **NEXT-12** Implement PLC QAM mapping separately with label/scale vectors.
- [ ] **NEXT-13** Implement physical-bin assembly that combines interleaved cells,
  pilot values and PLC without moving protected bins or replacing placeholders early.
- [ ] **NEXT-14** Add selectable outputs for pilot values, PLC bytes/coded bits/IQ
  and complete physical frequency bins; test with observation on and off.
- [ ] **NEXT-15** Add module tests, a bounded example, integration tests and a user
  guide with equations/diagram. Mark this stage complete only after these pass.

Pilot evidence for NEXT-01 is verified against PHY3.1 I20 §7.5.15.3–4, PDF171,
and PHY4.0 FDD delegation. PLC frame/message/bit-order requirements are verified
and recorded in [PLC evidence](../reference/plc-evidence.md); exact remaining
preamble vectors for NEXT-01 are still pending. Randomization is checked against
rendered Figure68/PDF157 and independent polynomial/cadence tests at both spacings.
Interleaving is
verified against rendered Figures65–67 and full independent coordinate vectors.
PLC coding is verified against the rendered Table21 and explicit puncturing
ranges; the source numbering/figure discrepancies are recorded in the evidence.
NEXT-02 now has pilot results, shared metadata and typed PLC message/FEC-input
results; later PLC algorithms will add their own result types. NEXT-14/15 have
pilot and PLC label probes plus pilot/message/codec/interleaver tests, examples
and guides; remaining PLC observer producers,
remaining signal stages and full-frame integration keep those items unchecked.

Planned paths and test-point IDs are in the architecture. Resolve any missing
normative behavior in `development/pilot-plc-signals/` before implementing it.
Do not create empty source files merely to make the planned tree appear complete.

## Waveform transmitter and receiver

- [ ] **WAVE-01** Separate IFFT and FFT modules with explicit bin ordering,
  normalization, sample-clock units and independent impulse/tone vectors.
- [ ] **WAVE-02** Separate cyclic-prefix insertion/removal with boundary tests.
- [ ] **WAVE-03** Separate windowing and stateful overlap-add, checking symbol
  cadence, taper lengths, energy and continuity at block boundaries.
- [ ] **WAVE-04** Reference/noiseless channel module with explicit input/output plane.
- [ ] **RX-01** Reference timing/framing, CP removal and FFT recovery.
- [ ] **RX-02** Pilot/PLC extraction and known-reference equalization first.
- [ ] **RX-03** Independently verified synchronization/channel estimation for
  nonideal operation; no claim of blind acquisition from a known-reference test.
- [ ] **RX-04** Connect physical data extraction, inverse cell interleaving,
  source-phase inverse randomization, NCP discovery and data decoding in production.
- [ ] **WAVE-05** Test points at each waveform and receive boundary, with finite
  block/range limits and direct library access.
- [ ] **WAVE-06** Zero-error noiseless waveform integration with real pilot/PLC signals.

## One-second reference run

- [ ] **RUN-01** Separate transmitter, receiver and runner modules; avoid moving
  numerical algorithms into a large orchestration function.
- [ ] **RUN-02** Stream one second of a single fixed channel using integer sample
  timing, with separate warm-up, observation and drain intervals.
- [ ] **RUN-03** Preserve partial codewords, interleaver history, PLC phase and
  sequence state across buffer boundaries; no per-codeword state resets.
- [ ] **RUN-04** Use source-symbol/sample identities for comparison and report
  complete, incomplete and excluded warm-up/drain payloads separately.
- [ ] **RUN-05** Zero noiseless payload errors with actual NCP discovery; retain
  LDPC iterations, BCH outcomes, codeword errors and the number of compared bits.
- [ ] **RUN-06** Bound memory/runtime/probe output, support cancellation and retain
  completed scientific results, including an explicit partial status.
- [ ] **RUN-07** Expose the channel runner through one public Rust API and a thin
  CLI command with shared typed JSON results; preserve the earlier coding mode.
- [ ] **RUN-08** Provide a reproducible PRBS-based example, math, assumptions and
  CLI/user documentation for the actual one-second path.

## Impairments and measurement-driven operation

- [x] **MODEL-01** Controlled cell AWGN and RxMER-derived residual noise in the
  original coding experiment, distinct from measured observations and screening.
- [x] **MODEL-02** Scenario identity/grid/time association and protected-role profiles.
- [x] **MODEL-03** Analytical temporal screening and gross useful-symbol capacity.
- [ ] **MODEL-04** Verified waveform channel/impairment models with test points and
  explicit reference planes; validate against the noiseless baseline first.
- [ ] **MODEL-05** Applied downstream channel-estimate model and receiver estimation/
  equalization with no double counting of post-equalization RxMER noise.
- [ ] **MODEL-06** Measurement-driven channel runs using the applied modulation
  profile, missing-carrier policy, temporal frames and actual FEC outcomes.
- [ ] **MODEL-07** Validate any temporal/burst model independently; RxMER alone does
  not supply time correlation, impulse noise or modem implementation behavior.

## Tooling, validation and later interfaces

- [x] **QA-01** Project-local Cargo wrapper, version source and Git/release helpers.
- [x] **QA-02** Formatting, warnings-denied Clippy, workspace and analysis-only tests,
  release CLI build and smoke validation in the existing pre-push gate.
- [x] **QA-03** Deterministic scientific tests with pinned source evidence and
  independent fixed vectors where available.
- [x] **QA-08** Add standard Rust facade and CLI-owned stderr logging, explicit
  levels/error ownership and tests for filters, clean data output and scientific
  invariance; document the current instrumentation scope.
- [ ] **QA-04** Extend the gate as new channel commands/test-point exports are added.
- [ ] **QA-05** Independent external interoperability vectors for full channel stages.
- [ ] **QA-06** Windows/macOS build/runtime validation and CI; Linux checks alone
  do not establish cross-platform runtime support.
- [ ] **QA-07** Native packaging and release automation after core functionality.
- [ ] **API-01** Python bindings as a separate optional adapter, preserving the same
  scientific engine, buffer lifetimes and observation contracts.
- [ ] **UI-01** Qt presentation after the core API is established; no duplicated DSP.
- [ ] **LATER-01** OFDMA architecture and implementation after downstream scope.
- [ ] **LATER-02** Channel bonding/multi-channel simulation or FDX-specific behavior
  only under a separate explicit design and validation scope.

The previously described “three major stages remaining” refers to the first
noiseless waveform milestone: pilots/PLC, waveform TX/RX, then one-second
integration. It does not mean all the broader product items above are complete.
