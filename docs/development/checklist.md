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

The [unchecked-item review](checklist-review.md) records source/test evidence for
the remaining items. Checked substeps show finished components inside an open
parent requirement; they do not imply that the full parent is complete.

Checklist IDs use the owning component or work area's prefix and a stable local
number: `PLC-01` for PLC work, `WAVE-TX-01` for waveform transmission,
`WAVE-RX-01` for waveform reception and `RX-CONFIG-01` for receiver
configuration. `WAVE-CH` owns the connecting channel; `WAVE-QA` owns shared
waveform observation and integration checks. Use this pattern for new groups instead of a temporary
`NEXT` prefix. Keep IDs stable when status or scheduling changes; numbering is
local to a component, not a global execution order. Select the next unfinished
item by the documented dependencies and current implementation priority.

## Architecture and module rules

- [x] **ARCH-01** Maintain one canonical architecture and this checklist.
- [x] **ARCH-02** Separate codec, DSP, transmission, simulation and adapter ownership.
- [x] **ARCH-03** Record stage test-point IDs, metadata and bounded observation design.
- [x] **ARCH-04** Implement the common typed test-point model and opt-in observer.
- [ ] **ARCH-05** Implement bounded stage/range selection and export through an adapter.
  - [x] Select registered stages, element ranges and source/channel symbol ranges
    with finite event/element budgets in `simulation/test_points/`.
  - [ ] Deliver a maintained export adapter; serialization in test sinks alone
    does not supply a user-facing export path.
- [ ] **ARCH-06** Prove probes do not change RNG draws, state, outputs or decisions.
  - [x] Verify available pilot/PLC/waveform/payload producers, including retained
    state and disabled/enabled/failed/exhausted observer cases where supported.
  - [ ] Cover source/noise RNG producers as they are integrated; the seeded
    source used to prepare a test payload does not establish RNG-probe coverage.
- [ ] **ARCH-07** Expose parity, twist and demultiplexing as separately callable stages,
  including reverse boundaries, while preserving the existing bit-processing API.
  - [x] Provide the composed16/256-QAM permutation and soft inverse (`BITS-01`).
  - [ ] Expose each intermediate forward/reverse boundary independently.

Every new algorithm gets a focused module, typed output and independent tests.
Stateful stages own their state; runners only connect stages. Direct stage outputs
remain usable independently of the observer. ARCH-04 currently registers the four
pilot stages, seven PLC stages, physical bins and thirteen waveform TX/channel/RX
boundaries plus ten payload/NCP TX/RX producers. ARCH-05 has bounded stage/element/symbol selection; adapter export
remains TODO. Pilot/PLC/waveform observation invariance is tested; ARCH-06 still needs
coverage of stateful/RNG-driven stages as their producers are added.

## Payload source

Guide: [Source design](../user/simulation/source.md).

- [x] **SRC-01** Deterministic seeded payload source, separate from codecs.
- [x] **SRC-02** PRBS source with explicit polynomial, seed, ordering and continuity.
- [x] **SRC-03** Independent PRBS recurrence vectors, full-period tests and
  recovered-stream comparison through actual waveform samples at both spacings.
  [PRBS design](../user/simulation/prbs.md) records seed/bit order, bounds and
  the distinction between mathematical and external interoperability evidence.

## Forward error correction

Guide: [FEC codec](../user/fec-codec.md).

- [x] **FEC-01** Rust BCH14232/14400 and short16200 rate8/9 LDPC encode/decode.
- [x] **FEC-02** Concatenated codec results, payload truth comparison and actual
  LDPC iterations, with correction/failure and zero-error tests.
- [ ] **FEC-03** Additional downstream codeword lengths, coding modes and
  shortening rules verified against the PHY; the current short8/9 mode does not
  establish every DOCSIS coding mode.

## Codeword bit processing

Guide: [Bit-processing flow](../user/qam256-coding-loop.md).

- [x] **BITS-01** Uniform16/256-QAM parity/twist/demux and soft inverse.

## QAM mapping and soft demapping

Guide: [QAM mapping](../user/qam-mapping.md).

- [x] **QAM-01** Embedded label-ordered mapping and max-log LLRs through16384-QAM.
- [ ] **QAM-02** Standalone hard slicer with deterministic ties, nearest label,
  reference I/Q and error/distance output; keep LDPC soft inputs unchanged.
- [ ] **QAM-03** Complete coding/placement support beyond uniform16/256-QAM,
  including verified per-order padding, shortening and non-square rules.
- [ ] **QAM-04** Mixed per-carrier data profiles and independent high-order
  interoperability vectors; mapper availability alone does not satisfy this item.

## Data and NCP randomization

Guide: [Cell randomization](../user/cell-randomizer.md).

- [x] **RAND-01** Stateful data/NCP cell randomization and soft inverse, including
  pilot skipping, zero-loaded clocking and128-symbol reset tests.

## Codeword packing

Guide: [Symbol packing](../user/symbol-packing.md).

- [x] **PACK-01** Bounded256-QAM queue, continuation, filler and NCP reservations.

## Next Codeword Pointer signaling

Guide: [NCP signaling](../user/ncp.md).

- [x] **NCP-01** Real NCP messages, CRC-24-D, separate control LDPC and QAM grouping.
- [x] **NCP-02** CRC-accepted receiver pointer discovery without transmitter IDs.
- [ ] **NCP-03** Dynamic data/NCP profile switching and session-level signaling
  validation; the current fixed-profile path does not implement this behavior.

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
- [x] **CELL-02** Promote test-only data/NCP/filler assembly and receive orchestration
  into reusable production modules with selected test points.

CELL-02 now supplies reusable assembly, TX/RX stream owners, received-configuration
checks, NCP hypotheses and bounded payload collection. Its
[actual payload waveform proof](../user/payload-transport/reference-experiment.md)
adds sample-domain recovery; the earlier CELL-01 fixture remains a cell-only test. Guides:
[transmission](../user/transmission.md), [layout](../user/carrier-layout.md),
[cell interleaving](../user/cell-interleaving.md).

## Pilot and PLC signal stages

Start here before moving to IFFT. Develop each component separately and keep its
input, output, phase and normalization available for tests. Typed pilot results
and bounded test points are now implemented in separate modules. The [pilot guide](../user/pilot-signals.md) records the verified sequence,
scaling and example. The [PLC message guide](../user/plc-messages.md) now covers
the timestamp/MC framing and FEC-input boundary. [PLC coding](../user/plc-coding.md)
now recovers those inputs. [PLC block interleaving](../user/plc-interleaving.md)
adds tested permutation/inverse stages and bounded label observations.
[PLC randomization](../user/plc-randomization.md) now adds separate TX/RX state,
frame resets and a sign-only soft inverse, with direct typed snapshots.
The [complete fixed PLC path](../user/plc-channel.md) now includes acquisition
packets, repetition/segmentation, preamble, mapping, physical-bin assembly and
bounded observations. **RX-CONFIG-01** now builds receiver-owned configuration
from those decoded messages. The first waveform TX/channel/RX milestone is now
implemented below. **CELL-02 / WAVE-RX-06** now add actual fixed256-QAM
payload/NCP waveform transport. **SRC-02 / SRC-03: PRBS** are now complete,
with independent recurrence vectors and actual recovered-stream comparison.
The RUN group now supplies duration stepping and zero-error recovery;
its next priority is **RUN-04** window-specific payload accounting.

- [x] **PLC-01** Confirm local PHY3.1 I20 section7.5.15 pilot requirements and
  section7.5.13 PLC requirements; record endpoint, sequence/bit order, scaling,
  phase and exact section/page evidence. Check PHY4.0 delegation where applicable.
- [x] **PLC-02** Define typed pilot sequence/value and PLC stage results, with
  shared test-point metadata and bounded optional observation.
- [x] **PLC-03** Implement shared pilot sequence logic in its own module and
  verify independent sequence/phase vectors.
- [x] **PLC-04** Implement continuous-pilot signal generation separately from
  additional-pilot placement; expose signs, I/Q, boost and locations.
- [x] **PLC-05** Implement scattered-pilot signal generation separately, using
  the existing physical pattern and correct PLC-aligned phase after interleaving.
- [x] **PLC-06** Implement shared pilot power/boost conversion with explicit
  reference energy; test signs, levels and collisions with protected roles.
- [x] **PLC-07A** Implement validated timestamp/MC message blocks, exact frame
  capacity and consecutive MSB-first36-byte/288-bit FEC inputs.
- [x] **PLC-07** Complete fixed-setting PLC acquisition messages: OCD, Profile A
  and NCP DPD MAC packets, packet CRCs, segmentation and repetition scheduling.
- [x] **PLC-08** Implement verified PLC coding as a separate stage; reuse a codec
  only where the PHY parameters match, otherwise add a focused Rust primitive.
- [x] **PLC-09** Implement PLC block interleaving in its own module with
  independent permutation vectors and a coded-to-interleaved test point.
- [x] **PLC-10** Implement PLC randomization with independent state and its own
  phase/reset tests; do not reuse data/NCP randomizer state.
- [x] **PLC-11** Implement the PLC preamble in its own module with phase/sign tests.
- [x] **PLC-12** Implement PLC QAM mapping separately with label/scale vectors.
- [x] **PLC-13** Implement physical-bin assembly that combines interleaved cells,
  pilot values and PLC without moving protected bins or replacing placeholders early.
- [x] **PLC-14** Add selectable outputs for pilot values, PLC bytes/coded bits/IQ
  and complete physical frequency bins; test with observation on and off.
- [x] **PLC-15** Add module tests, a bounded example, integration tests and a user
  guide with equations/diagram. Mark this stage complete only after these pass.

Pilot and PLC evidence for PLC-01 is recorded in [PLC evidence](../reference/plc-evidence.md)
and the stage guides. Tables48/49, Figure68 and the block-interleaving figures
were verified against the local PHY PDF. Acquisition management fields, assignments,
MC segmentation and repeat intervals use pinned MULPI I25 text. The supported
fixed subset has single-part OCD/ProfileA/NCP messages; arbitrary profiles,
list/vector TLVs, multipart MMMs and EM/TR modes are not claimed.

PLC-14/PLC-15 now include selectable message bytes, transmitted coded bits, labels,
PLC data/preamble I/Q and full physical grids, with independent module tests,
both-spacing multi-frame FEC/QAM/CRC integration and a bounded example. RX diagnostics
remain directly accessible; additional RX common-observer producers and CLI/file
export remain later work. External interoperability remains open under QA-05;
the initial known-reference waveform validation is now covered by WAVE-QA-02.

Planned paths and test-point IDs are in the architecture. Resolve any missing
normative behavior in `development/pilot-plc-signals/` before implementing it.
Do not create empty source files merely to make the planned tree appear complete.

## Receiver configuration from PLC

- [x] **RX-CONFIG-01** Add receiver-owned state that consumes CRC-validated,
  reassembled OCD/ProfileA/NCP DPD descriptors and applies a consistent channel
  configuration. Preserve channel/profile/change identities; validate required
  descriptors before data/NCP recovery. Derive layout, interleaving and modulation
  from received signaling rather than copying transmitter settings. Keep supplied
  PLC bootstrap knowledge explicit. Test missing/corrupt/inconsistent messages,
  configuration readiness and unchanged valid state after rejection.

Separate channel/profile/collection/configuration/state modules now validate
received settings and expose a complete immutable snapshot behind a readiness
gate. The acquisition composition receives actual PLC I/Q and messages; consumers
can build physical/pilot/interleaver settings from received data alone. Tests cover
all descriptor orders, independent counters, missing/corrupt/conflicting input,
profile precedence and multi-frame transport. Fixed payload recovery now consumes
these settings under WAVE-RX-06; dynamic activation remains NCP-03. See the
[receiver configuration design](../user/receiver-configuration/README.md).

## Waveform direction and completion scope

Waveform work covers the **transmitter, connecting channel and receiver**.
TX converts physical frequency-bin I/Q into time-domain samples. RX processes
received samples back into frequency-bin I/Q and recovers the carried signals.
Each direction has separate component IDs, modules, tests and design guides.
Shared numerical primitives keep one implementation; direction labels do not
require duplicate algorithms.

```mermaid
flowchart LR
    GRID["TX: assembled frequency bins"] --> TX["WAVE-TX: IFFT, prefix, window and overlap-add"]
    TX --> CH["WAVE-CH: time-domain channel"]
    CH --> RX["WAVE-RX: framing, prefix removal and FFT"]
    RX --> SIGNAL["WAVE-RX: pilots, PLC and data recovery"]
    SIGNAL --> QA["WAVE-QA: compare recovered signals with independent TX truth"]
```

The first noiseless milestone now includes TX stages, their matched RX
primitives, the unity channel, known-reference framing/extraction, bounded
observations and actual PLC waveform integration. Each completed item links
to its subsystem guide in the [waveform index](../user/waveform/README.md).
Nonideal synchronization (WAVE-RX-05) remains later work. WAVE-RX-06 now
implements production transport for the explicitly supported fixed256-QAM
subset; it does not imply dynamic or every-order payload support. The one-second runner has its own RUN group below.

## Waveform transmitter: IFFT

Design: [TX IFFT](../user/waveform/tx-ifft.md).

- [x] **WAVE-TX-01** Convert the complete physical frequency grid to time-domain
  I/Q in a separate IFFT module. Specify bin ordering, amplitude normalization,
  sample clock and sample indices. Verify independent impulse/tone and energy
  vectors at both FFT sizes, alongside the matched WAVE-RX-01 transform.

## Waveform transmitter: cyclic-prefix insertion

Design: [TX prefix insertion](../user/waveform/tx-prefix.md).

- [x] **WAVE-TX-02** Insert the selected cyclic prefix in a focused TX operation.
  Users choose a PHY-permitted length before the run; validate and retain that
  configuration for the run. Copy the useful symbol's trailing samples to its
  front without changing the useful samples. Test every permitted prefix,
  invalid settings, output lengths and symbol boundaries at both spacings.

A fixed prefix is a per-run setting, not a hard-coded user restriction.
Changing it requires a newly configured run/session. The transmitter advertises
it in OCD; the receiver uses decoded settings after acquisition, with initial
PLC bootstrap knowledge kept explicit. WAVE-RX-02 owns the inverse operation.

## Waveform transmitter: windowing and overlap-add

Design: [TX windowing](../user/waveform/tx-window.md) and [TX overlap-add](../user/waveform/tx-overlap.md).

- [x] **WAVE-TX-03** Keep TX windowing and stateful overlap-add in separate
  modules. Apply the selected roll-off and cyclic extension using pinned PHY
  rules. Verify taper samples, energy, continuity, symbol cadence, startup/drain
  tails and state across block boundaries. Roll-off must not add to symbol cadence.

## Waveform channel: reference connection

Design: [Unity channel](../user/waveform/channel.md).

- [x] **WAVE-CH-01** Connect TX time samples to RX time samples through a
  reference/noiseless channel module. Document input/output reference planes,
  sample clock, timing and gain. Verify unity transfer and block continuity;
  impairments remain separate MODEL work.

## Waveform receiver: FFT

Design: [RX FFT](../user/waveform/rx-fft.md).

- [x] **WAVE-RX-01** Convert a useful received time-domain symbol to physical
  frequency-bin I/Q in a separate FFT module. Match the documented TX ordering
  and normalization. Test independent vectors and IFFT/FFT recovery at both
  sizes; a round trip alone is insufficient validation.

## Waveform receiver: cyclic-prefix removal

Design: [RX prefix removal](../user/waveform/rx-prefix.md).

- [x] **WAVE-RX-02** Remove the configured prefix from a correctly framed RX
  symbol, preserving exactly the useful FFT input. Check input length, boundaries
  and every permitted prefix at both spacings. Keep prefix removal separate
  from timing acquisition and from TX prefix insertion.

## Waveform receiver: timing and symbol framing

Design: [RX known-reference framing](../user/waveform/rx-framing.md).

- [x] **WAVE-RX-03** Recover symbol windows from the received stream using
  explicit known-reference timing first. Compose framing, prefix removal and
  FFT recovery without borrowing transmitter buffers or payload truth. Verify
  offsets, arbitrary input block splits, partial symbols and windowed boundaries.
  Known timing does not establish blind synchronization.

## Waveform receiver: signal extraction and equalization

Design: [RX extraction](../user/waveform/rx-extraction.md) and [equalization](../user/waveform/rx-equalization.md).

- [x] **WAVE-RX-04** Extract pilots and PLC from recovered bins and apply
  known-reference equalization first. Preserve frequency/role associations and
  amplitude units; feed actual received PLC cells into the existing decoder and
  receiver-configuration owner. Keep bootstrap settings explicit.

## Waveform receiver: nonideal acquisition and payload recovery

Design: [Known-reference waveform](../user/waveform/README.md) and
[fixed payload transport](../user/payload-transport/README.md).

- [ ] **WAVE-RX-05** Independently verify synchronization/channel estimation for
  nonideal operation. This extends the known-reference receiver; it must not be
  marked complete from noiseless tests with supplied timing/channel knowledge.
- [x] **WAVE-RX-06** Connect physical data extraction, inverse cell interleaving,
  source-phase inverse randomization, NCP discovery and data decoding in production.
  Use received configuration; preserve the separate control/payload codec paths.
  Fixed uniform256-QAM ProfileA and positive uniform QPSK/16-QAM/64-QAM NCP
  now have live readiness checks, source/channel identities, explicit startup
  filler and M-1 drain, terminal rejection diagnostics and ten bounded test points.
  Tests include actual PLC acquisition then real sample-domain payload recovery
  at both spacings, all three NCP orders in cell transport, reset-crossing words,
  CRC/profile/continuation errors and observer invariance. Dynamic switching,
  mixed/zero DPD loading and blind acquisition remain outside this fixed subset.

## Waveform validation: observations

Design: [Waveform test points](../user/waveform/observations.md).

- [x] **WAVE-QA-01** Add direction-tagged test points at each TX, channel and RX
  boundary, with explicit sample/bin units, finite block/range limits and direct
  library access. Prove observation does not change outputs or stage state.

## Waveform validation: noiseless integration

Design: [Actual waveform reference experiment](../user/waveform/reference-experiment.md).

- [x] **WAVE-QA-02** Connect TX waveform generation, the noiseless channel and
  matched known-reference RX using real pilot/PLC signals. Verify recovered
  frequency bins numerically and actual decoded PLC information with zero bit
  errors at both spacings. Include prefix/window/overlap boundaries and retain
  decoder diagnostics. Document which payload/runner stages remain unfinished.

Completed waveform validation includes independent centered/unitary vectors,
all permitted CP/RP combinations at both spacings, shifted-window sums,
partial/offset stream framing, rejection invariance and actual two-frame PLC
recovery. Integration exercises a192MHz configured cap and wide active grid
with synthetic exclusions, complete FFT-bin comparisons, actual decoder
iterations, zero information-bit errors and observation invariance. This is
reference PLC/pilot verification. The separate CELL-02/WAVE-RX-06 proof adds
actual encoded payload/NCP recovery; configurable-duration lifecycle remains
RUN work, not an inference from either bounded example. [Evidence](../reference/waveform-evidence.md).

## Configurable-duration reference run

Design: [Channel runner](../user/channel-runner/README.md),
[timing](../user/channel-runner/timing.md), [session](../user/channel-runner/session.md)
and [results](../user/channel-runner/results.md). RUN-01/02/03/05 are implemented;
next is RUN-04 observation-window payload accounting, followed by RUN-06/07/08
job control and CLI delivery.

- [x] **RUN-01** Separate transmitter, receiver and runner modules; avoid moving
  numerical algorithms into a large orchestration function.
  - [x] Separate reusable payload/waveform TX and RX state owners; compose them
    in the bounded reference verifier without duplicating stage mathematics.
  - [x] Add the symbol-stepping duration-runner owner around those existing stages.
- [x] **RUN-02** Stream a user-configured duration of one fixed channel, initially
  defaulting to one simulated second. Use checked64-bit sample accounting and
  bounded buffers for longer runs; never reset stage state at second boundaries.
  Keep acquisition, observation, queue completion, cell drain and final tail
  separate. `duration_timing`/`sample_timing` now provide64-bit plans; the legacy
  one-second helper delegates to them without changing its result contract.
- [x] **RUN-03** Preserve partial codewords, interleaver history, PLC phase and
  sequence state across buffer boundaries; no per-codeword state resets.
  - [x] Verify these stateful foundations in payload/waveform/PLC tests and
    continuous PRBS chunk, codeword and period tests.
  - [x] Verify their joint lifecycle in the duration runner, including longer
    intervals and observation boundaries without reseeding or phase resets.
- [ ] **RUN-04** Use source-symbol/sample identities for comparison and report
  complete, incomplete and excluded warm-up/drain payloads separately.
- [x] **RUN-05** Zero noiseless payload errors with actual NCP discovery; retain
  LDPC iterations, BCH outcomes, codeword errors and the number of compared bits.
  - [x] Recover four real payloads through waveform samples at both spacings;
    compare56928 PRBS bits per spacing with zero errors and retain decoder results.
  - [x] Retain per-step received comparisons, genuine iterations/BCH flags and
    bounded cumulative totals in duration runs. A1.01-second50kHz release run
    recovered8950 words/127376400 bits with zero payload or PLC errors.
    Observation-window incomplete/excluded classification remains RUN-04.
- [ ] **RUN-06** Bound memory/runtime/probe output, support cancellation and retain
  completed scientific results, including an explicit partial status.
  - [x] Keep waveform/queue/history storage bounded independently of duration;
    expose completed step results and close failed sessions explicitly.
  - [ ] Add formal cancellation, wall-clock budgets and partial-job results.
- [ ] **RUN-07** Expose the channel runner through one public Rust API and a thin
  CLI command with shared typed JSON results; preserve the earlier coding mode.
  - [x] Expose `ChannelRun`, settings, duration plans and typed step/summary JSON.
  - [ ] Add the duration-channel CLI facade alongside the older coding commands.
- [ ] **RUN-08** Provide a reproducible PRBS-based example, math, assumptions and
  CLI/user documentation for the duration-based path and its initial one-second default.
  - [x] Provide bounded and duration-based PRBS waveform examples and
    per-component guides with Mermaid, timing, recurrence and comparison math.
  - [ ] Provide duration-based CLI examples and documentation once that path exists.

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
