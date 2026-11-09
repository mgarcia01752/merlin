# Implementation checklist

This is the official progress and TODO list for DOCSIS OFDM Channel Sim. Read it
with the [architecture](../architecture.md) and the concise
[development status](status.md). The [development workspace](README.md) links
ongoing plans and historical feature records. Update both when adding a stage,
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

## Terminology

**Cell = QAM symbol**: one complex I/Q point on one subcarrier during one OFDM
symbol period. An **OFDM symbol** contains the complete set of subcarrier points
for that period. A256-QAM cell carries eight coded bits; pilot/control cells may
instead use BPSK or QPSK. Read cell interleaving as modulation-symbol interleaving,
distinct from bit interleaving. Introduce the term as **QAM symbol (cell)**; see
the [architecture definition](../architecture.md#terminology-cell-and-symbol).

## Architecture and module rules

- [x] **ARCH-01** Maintain one canonical architecture and this checklist.
- [x] **ARCH-02** Separate codec, DSP, transmission, simulation and adapter ownership.
- [x] **ARCH-03** Record stage test-point IDs, metadata and bounded observation design.
- [x] **ARCH-04** Implement the common typed test-point model and opt-in observer.
- [x] **ARCH-05** Implement bounded stage/range selection and export through an adapter.
  - [x] Select registered stages, element ranges and source/channel symbol ranges
    with finite event/element budgets in `simulation/test_points/`.
  - [x] Deliver the bounded versioned JSONL writer and channel CLI adapter;
    preserve original selected metadata and reject byte/I/O failures explicitly.
- [x] **ARCH-06** Prove probes do not change RNG draws, state, outputs or decisions.
  - [x] Verify available pilot/PLC/waveform/payload producers, including retained
    state and disabled/enabled/failed/exhausted observer cases where supported.
  - [x] Cover actual source and preexisting coding-noise producers: enabled,
    exhausted and failed observers preserve source/next-noise draws and outcomes.
- [x] **ARCH-07** Expose parity, twist and demultiplexing as separately callable stages,
  including reverse boundaries, while preserving the existing bit-processing API.
  - [x] Provide the composed16/256-QAM permutation and soft inverse (`BITS-01`).
  - [x] Expose independently callable parity/twist/demultiplexing forward and
    soft-inverse modules; compose those same mapping owners in the legacy facade.

Every new algorithm gets a focused module, typed output and independent tests.
Stateful stages own their state; runners only connect stages. Direct stage outputs
remain usable independently of the observer. ARCH-04 currently registers the four
pilot stages, seven PLC stages, physical bins and thirteen waveform TX/channel/RX
boundaries plus ten payload/NCP TX/RX producers and source/legacy-noise outputs.
ARCH-05 now includes bounded JSONL/CLI export; ARCH-06 covers current stateful/RNG
producers; ARCH-07 supplies independent bit-processing boundaries. Future producers
must preserve the same limits and invariance requirements.

## Payload source

Guide: [Source design](../user/simulation/source.md).

- [x] **SRC-01** Deterministic seeded payload source, separate from codecs.
- [x] **SRC-02** PRBS source with explicit polynomial, seed, ordering and continuity.
- [x] **SRC-03** Independent PRBS recurrence vectors, full-period tests and
  recovered-stream comparison through actual waveform samples at both spacings.
  [PRBS design](../user/simulation/prbs.md) records seed/bit order, bounds and
  the distinction between mathematical and external interoperability evidence.

- [x] **SRC-04** Add a bounded exact-byte text/binary source beside PRBS15, with an
  explicit bit order and cursor; preserve PRBS/noise behavior.
- [x] **SRC-05** Define send-once EOF, final-codeword padding and meaningful-length
  accounting; support explicit total-bit targets with opt-in PRBS15 append-to-fill.
  Independently compare file/fill bits, exclude padding and preserve original bytes.
- [x] **SRC-06** Expose source selection through shared Rust/CLI configuration;
  test text/binary byte preservation, size checks, optional PRBS fill, empty/oversized
  inputs, EOF, truncation and cancellation; keep noise/source state independent.
  See the [file-source design](../user/simulation/file-source/README.md),
  [task design](features/file-payload-source/design.md),
  [implementation and tests](features/file-payload-source/implementation.md) and
  [source/workspace plan](features/visual-workspace/plan.md).

Automatic channel-derived minimum/alignment/target rules remain a deferred policy
decision. The implemented file source uses bounded arbitrary bytes and an optional
explicit target; completing SRC-04..06 does not implement the Qt controls.

## Forward error correction

Guide: [FEC codec](../user/fec-codec.md).

- [x] **FEC-01** Rust BCH14232/14400 and short16200 rate8/9 LDPC encode/decode.
- [x] **FEC-02** Concatenated codec results, payload truth comparison and actual
  LDPC iterations, with correction/failure and zero-error tests.
- [x] **FEC-03** Implement PHY-verified downstream shortened codeword lengths and
  effective rates using the short16200-bit rate8/9 mother code; retain all BCH/LDPC
  parity and verify both TX/RX shortening boundaries independently.
  - [x] Verify local PHY I20 Table23/Figures46–47 and4.0 FEC delegation; normal64800
    downstream frames and other mother rates are not introduced.
  - [x] Expose validated lengths, independent stage outputs, known-zero restoration,
    consistency checks, actual decoder statuses/iterations and payload comparison.
  - [x] Validate independent parity/coordinate oracles, varied lengths, noiseless
    recovery, correction/failure boundaries and full-mode compatibility.
  See the [task design](features/fec-shortening/design.md),
  [validation](features/fec-shortening/validation.md) and
  [component guides](../user/fec-shortening/README.md). QAM-03 now connects
  explicit shortened words through bit processing, received NCP boundaries and waveform.

## Codeword bit processing

Guide: [Bit-processing flow](../user/qam256-coding-loop.md).

- [x] **BITS-01** Uniform16/256-QAM parity/twist/demux and soft inverse.
- [x] **BITS-02** Non-shortened mixed24-column parity/twist ordering, bypassed
  demultiplexing and independently transcribed full-matrix permutation tests.

## QAM mapping and soft demapping

Guide: [QAM mapping](../user/qam-mapping.md).

- [x] **QAM-01** Embedded label-ordered mapping and max-log LLRs through16384-QAM.
- [x] **QAM-02** Standalone hard slicer with deterministic ties, nearest label,
  reference I/Q and error/distance output; keep LDPC soft inputs unchanged.
- [x] **QAM-03** Complete coding/placement support beyond uniform16/256-QAM,
  including verified per-order padding, shortening and non-square rules.
  - [x] Implement all full uniform orders through16384-QAM from pinned tables.
  - [x] Implement shortened24-column holes, soft inverse and final-cell grouping.
  - [x] Discover shortened lengths from received NCP boundaries with odd-byte admission.
  - [x] Validate uniform and explicit shortened words through live PLC/NCP and waveform
    at both spacings, retaining actual failed decisions and original-length test points.
  See the [task design](features/qam-completion/design.md) and
  [component guides](../user/qam-processing/README.md). General SRC/file jobs retain
  full-word EOF padding; shortened traffic is an explicit API/reference experiment.
- [ ] **QAM-04** Mixed per-carrier data profiles and independent high-order
  interoperability vectors; mapper availability alone does not satisfy this item.
  - [x] Implement frozen mixed ProfileA transport, variable-width cells, zero loading
    and final-cell padding through actual PLC/NCP/waveform/FEC at both spacings.
  - [ ] Validate independent external high-order interoperability vectors.
    Deferred at the user's request. [Required vector bundle and acceptance plan](features/qam-completion/external-vectors.md).
    Existing local specification-derived matrices/round trips do not close this item.

## Data and NCP randomization

Guide: [Cell randomization](../user/cell-randomizer.md).

- [x] **RAND-01** Stateful data/NCP cell randomization and soft inverse, including
  pilot skipping, zero-loaded clocking and128-symbol reset tests.

## Codeword packing

Guide: [Symbol packing](../user/symbol-packing.md).

- [x] **PACK-01** Bounded256-QAM queue, continuation, filler and NCP reservations.
- [x] **PACK-02** Bounded coded-bit packing against inverse-frequency profiles,
  variable-width mapping, zero loading and last-cell padding without TX IDs in RX.

## Next Codeword Pointer signaling

Guide: [NCP signaling](../user/ncp.md).

- [x] **NCP-01** Real NCP messages, CRC-24-D, separate control LDPC and QAM grouping.
- [x] **NCP-02** CRC-accepted receiver pointer discovery without transmitter IDs.
- [x] **NCP-03** Opt-in bounded data/NCP generation switching: received DPD bank,
  C-selected data profiles0..15, exact128-symbol U announcement and N-selected
  positive uniform NCP changes. Preserve codeword/randomizer/interleaver state;
  verify real PLC DPD recovery and crossing-word payload recovery at both spacings.
  Same-parity generation replacement requires reacquisition; OPT tagging and dynamic
  shortened words are excluded. The duration CLI remains fixed-profile.
  Guide: [dynamic profiles](../user/ncp-profiles/README.md).

## Fixed channel and cell transport

- [x] **CFG-01** Fixed25/50kHz numerology, up-to192MHz setup, prefix/roll-off validation
  and integer one-second timing calculations.
- [x] **LAYOUT-01** Explicit physical PLC/continuous/scattered/excluded roles.
- [x] **LAYOUT-02** Reduced-domain association and128-symbol inverse pilot reference.
- [x] **LAYOUT-03** Generate additional continuous pilots using PHY I20 density,
  merged spectrum, PLC/poor-bin avoidance, edge guards, isolated-region coverage
  and reproducible bounded disturbance. Validate both spacings and invalid inputs.
  Guide: [pilot placement](../user/carrier-placement.md).
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
The complete RUN group now includes source-window accounting, cancellation,
partial jobs, CLI delivery and probe export. The subsequent MODEL-04/INPUT-01
group now adds waveform impairments and fixed-profile measured operation;
MODEL-05/06/07 now add supported pilot estimation, temporal epochs and mixed
profiles below; the broader WAVE-RX-05 acquisition scope remains open.

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
remain directly accessible; additional RX common-observer producers remain
future work, while the shared JSONL writer/CLI export is now implemented. External interoperability remains open under QA-05;
the initial known-reference waveform validation is now covered by WAVE-QA-02.

Planned paths and test-point IDs are in the architecture. Resolve any missing
normative behavior using the [pilot/PLC development record](features/pilot-plc-signals/architecture.md)
and private evidence under `docs/development/local/pilot-plc-signals/` before implementing it.
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
these settings under WAVE-RX-06; opt-in dynamic activation is documented under NCP-03. See the
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
Bounded nonideal synchronization is now available under WAVE-RX-05. WAVE-RX-06 now
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

- [ ] **WAVE-RX-05** Complete nonideal synchronization/channel-estimation
  verification, including the broader acquisition scope retained by this checklist.
  - [x] Bounded received-only CP timing, fractional CFO correction and held-out
    scattered-pilot phase selection, followed by pilot CH-EST-RX. Tests cover both
    spacings, unknown offset/phase, nonzero CFO, additive sample perturbations and
    a smooth complex channel without TX H in RX.
  - [ ] Verify integer-carrier/unknown-numerology acquisition and broader channel
    conditions beyond the implemented smooth-channel, known-layout subset.
  Known geometry and an unaliased fractional offset remain assumptions. Do not
  close the parent from these synthetic bounded-subset tests.
  Guide: [WAVE-RX acquisition](../user/waveform/rx-acquisition.md).
- [x] **WAVE-RX-06** Connect physical data extraction, inverse cell interleaving,
  source-phase inverse randomization, NCP discovery and data decoding in production.
  Use received configuration; preserve the separate control/payload codec paths.
  The original fixed uniform256-QAM ProfileA and positive uniform QPSK/16-QAM/64-QAM NCP
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
actual encoded payload/NCP recovery; the separate RUN group now validates
the duration-based lifecycle rather than inferring it from either bounded example. [Evidence](../reference/waveform-evidence.md).

## Configurable-duration reference run

Design: [Channel runner](../user/channel-runner/README.md),
[timing](../user/channel-runner/timing.md), [session](../user/channel-runner/session.md)
and [results](../user/channel-runner/results.md). All RUN items are implemented
for fixed-profile noiseless operation. See [accounting](../user/channel-runner/accounting.md),
[job lifecycle](../user/channel-runner/job.md), [CLI](../user/channel-runner/cli.md)
and [the later impairment reference](impairment-reference.md).

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
- [x] **RUN-04** Use source-symbol/sample identities for comparison and report
  complete, incomplete and excluded warm-up/drain payloads separately.
  Whole-word source-window classification retains crossing/outside counts, late
  contained recoveries, received incomplete starts and generated-unrecovered words.
  Window BER/rate use only contained comparisons; final rate is null for partial jobs.
- [x] **RUN-05** Zero noiseless payload errors with actual NCP discovery; retain
  LDPC iterations, BCH outcomes, codeword errors and the number of compared bits.
  - [x] Recover four real payloads through waveform samples at both spacings;
    compare56928 PRBS bits per spacing with zero errors and retain decoder results.
  - [x] Retain per-step received comparisons, genuine iterations/BCH flags and
    bounded cumulative totals in duration runs. A1.01-second50kHz release run
    recovered8950 words/127376400 bits with zero payload or PLC errors.
    RUN-04 now adds independent observation-window incomplete/excluded accounting.
- [x] **RUN-06** Bound memory/runtime/probe output, support cancellation and retain
  completed scientific results, including an explicit partial status.
  - [x] Keep waveform/queue/history storage bounded independently of duration;
    expose completed step results and close failed sessions explicitly.
  - [x] Add cooperative token/runtime checks, progress, bounded retained details
    and Complete/Cancelled/RuntimeExceeded/Failed results preserving completed science.
- [x] **RUN-07** Expose the channel runner through one public Rust API and a thin
  CLI command with shared typed JSON results; preserve the earlier coding mode.
  - [x] Expose `ChannelRun`, settings, duration plans and typed step/summary JSON.
  - [x] Add `ofdm-fec channel` with versioned requests, shared typed JSON and
    bounded optional JSONL probes; preserve older coding commands and no-clobber output.
- [x] **RUN-08** Provide a reproducible PRBS-based example, math, assumptions and
  CLI/user documentation for the duration-based path and its initial one-second default.
  - [x] Provide bounded and duration-based PRBS waveform examples and
    per-component guides with Mermaid, timing, recurrence and comparison math.
  - [x] Provide the one-second default request, quick probe-export request and
    subsystem guides with Mermaid/math, bounds and partial-result meanings.

## Impairments and measurement-driven operation

- [x] **INPUT-01** Import current `pnm-decode` JSON for RxMER, downstream channel
  estimates and applied modulation profiles into the shared typed scenario model.
  Keep binary decoding external and direct Rust API inputs available; preserve
  source grids, nulls, timestamps and provenance. The CLI currently requires this
  repository's scenario JSON; `import-pnm` converts current decoder range-scheme
  documents using explicit timestamp tolerance and SHA-256 of the exact JSON inputs.
  - [x] Prepare three current decoded single-capture fixtures with provenance,
    original local binaries and reproducible Rust-generated RxMER alternatives.
    Document timestamp differences and retain the parent importer TODO.
  - [x] Add a Rust LTE-inspired10MHz downlink fixture with linear residual-power
    combination, exact identity/grid/coverage rejection and an alignment sidecar.
    Keep the rectangular notch and record no additional thermal floor/apply-once
    policy. Fixture preparation alone does not establish waveform/FEC behavior.
  - [x] Validate/import all three capture types, expand bounded profiles, preserve
    missing readings and provide standalone CLI plus direct typed Rust API tests.
- [x] **INPUT-02** Explicit decoder contract revision1 pins pnm-decode0.3.6;
  request schema2 requires the producer declaration. Preserve legacy schema1,
  exact-byte hashes/nulls/grids and shared Rust validation. Test actual exports,
  additive display fields and rejection of unsupported producer/revision drift.
  Guide: [import contract](../user/impairments/pnm-import.md#explicit-decoder-contract-input-02).
- [x] **MODEL-01** Controlled cell AWGN and RxMER-derived residual noise in the
  original coding experiment, distinct from measured observations and screening.
- [x] **MODEL-02** Scenario identity/grid/time association and protected-role profiles.
- [x] **MODEL-03** Analytical temporal screening and gross useful-symbol capacity.
- [x] **MODEL-04** Verified waveform channel/impairment models with test points and
  explicit reference planes; validate against the noiseless baseline first.
  - [x] Seeded sample AWGN with chunk-invariant draws and unitary variance tests.
  - [x] Gaussian per-carrier residual synthesis through waveform samples, with
    independently transported per-cell LLR variances and no added thermal floor.
  - [x] Verify actual MER, PRBS/FEC outcomes, partial failure and protected narrow
    LTE sweep. This is a synchronized diagonal model, not general FIR multipath.
  - [x] Expose LTE interference strength and a separate data-only RxMER stress
    fixture for CLI error-boundary sweeps, keeping the PLC neighborhood unchanged.
  - [x] Export baseline and12/11/10/9dB binary PNM demos, preserving original
    headers and verifying all readings through the external decoder.

## Paired constellation before and after equalization

**Completed group: CONST-01 through CONST-05.** Uses supplied H and
matching1/H first. MODEL-05 now adds independent pilot estimation; the bounded nonideal acquisition model is documented under WAVE-RX-05.
The [feature plan](features/paired-constellation/plan.md) defines confirmed reuse,
profile/coordinate selection, reference planes, bounded collection and tests.
The [user design](../user/impairments/paired-constellation.md) explains the result.

- [x] **CONST-01** Define typed selection and payload-only coordinate attribution
  through interleaving/NCP; default to8192 total QAM pairs for a supported profile.
- [x] **CONST-02** Collect matched raw FFT/equalized I/Q with finite storage,
  exact carrier/time metadata, shared scaling and honest partial/exhausted status.
- [x] **CONST-03** Expose one Rust result through the public API and opt-in channel
  CLI JSON; preserve existing input contracts and no-clobber/output limits.
- [x] **CONST-04** Verify H/inverse reference planes, payload eligibility including
  failed words, startup/drain handling and unchanged scientific/RNG outcomes.
- [x] **CONST-05** Provide a reproducible8192-pair demo, user instructions and
  actual validation evidence; update design/status after implementation.

Implementation and Linux validation are recorded in the
[component record](features/paired-constellation/implementation.md). The full gate
passed, including release-mode measured capture with8192 pairs. Independent
estimation and nonideal timing remain unfinished below.

## Remaining receiver and measurement work

Notation: **Channel Estimation (CH-EST)** is the function; **Receiver Channel
Estimation (CH-EST-RX)** and **Transmitter Channel Estimation (CH-EST-TX)**
include direction. Use these prefixes for their respective task IDs.

- [x] **MODEL-05** Applied downstream channel-estimate model and receiver estimation/
  equalization with no double counting of post-equalization RxMER noise.
  - [x] Apply supplied channel response H and independent known-reference1/H;
    verify complex gain and captured frequency response without duplicate residuals.
  - [x] Independently estimate H from pilots and validate noisy equalization;
    provide bounded CP timing acquisition with known numerology/PLC phase.
    Bounded WAVE-RX-05 acquisition is opt-in; the CLI retains known timing.

  - [x] **CH-EST-RX-01** Build receiver-owned known pilot references and per-pilot H-hat
    estimates, starting noiseless with explicit known timing/phase.
  - [x] **CH-EST-RX-02** Interpolate estimates across data/PLC carriers and apply1/H-hat
    independently of supplied H; test startup, coverage and near-zero policies.
  - [x] **CH-EST-RX-03** Add bounded separate continuous/scattered-pilot pre/post I/Q
    analyzers with sign/boost/phase metadata and in-fit/held-out interpretation.
  - [x] **CH-EST-RX-04** Add bounded separate PLC16-QAM and BPSK-preamble analyzers,
    observable during acquisition and retained on failed control/FEC outcomes.
  - [x] **CH-EST-RX-05** Validate reference-versus-estimated reception, noisy/noiseless
    behavior and independent errors/iterations, plus observation invariance.
  - [x] **CH-EST-RX-06** Export shared Rust/CLI results and actual pilot/PLC plots;
    update per-subsystem user guides and pass the full gate for supported scope.

The implemented MODEL-05 group is detailed in the
[pilot-estimation and analyzer plan](features/pilot-channel-estimation/plan.md).
WAVE-RX-05 now provides bounded nonideal acquisition. Bounded CP sample-offset search
is implemented independently; CLI operation retains known timing/PLC phase.
See the [implementation record](features/pilot-channel-estimation/implementation.md).

- [x] **MODEL-06** Measurement-driven channel runs using the applied modulation
  profile, missing-carrier policy, temporal frames and actual FEC outcomes.
  - [x] Select one captured frame/profile/response with explicit epoch tolerance,
    reject missing active samples and unsupported loading, and run real256-QAM FEC.
  - [x] Add explicit same-grid temporal epochs and frozen uniform16/256/mixed
    ProfileA modes through actual coding/waveform/FEC, including zero loading.
    This fixed subset rejects other selectors; NCP-03 provides opt-in dynamic full-word transport.
- [x] **MODEL-07** Validate any temporal/burst model independently; RxMER alone does
  not supply time correlation, impulse noise or modem implementation behavior.
  - [x] State the piecewise-constant, independent Gaussian model; verify power/lag
    statistics and unchanged RNG/FEC when an identical epoch is inserted.
  - [x] Preserve original capture times and reject invalid/out-of-window/oversized
    schedules before work; do not infer a burst process from RxMER.

## Tooling, validation and later interfaces

### Project-local tooling

- [x] **QA-01** Project-local Cargo wrapper, version source and Git/release helpers.

### Local validation and scientific regression

- [x] **QA-02** Formatting, warnings-denied Clippy, workspace and analysis-only tests,
  release CLI build and smoke validation in the existing pre-push gate.
- [x] **QA-03** Deterministic scientific tests with pinned source evidence and
  independent fixed vectors where available.

### Rust logging and diagnostics

- [x] **QA-08** Add standard Rust facade and CLI-owned stderr logging, explicit
  levels/error ownership and tests for filters, clean data output and scientific
  invariance; document the current instrumentation scope.

### CLI and test-point validation

- [x] **QA-04** Extend the gate as new channel commands/test-point exports are added.
  Release-mode channel/JSONL smoke validation and actual CLI no-clobber/schema tests
  now accompany the existing coding CLI checks.

### External interoperability validation

- [ ] **QA-05** Independent external interoperability vectors for full channel stages.
  - [x] Record the required dataset/provenance and add bounded manifest/hash admission.
  - [ ] Obtain external data, implement its Rust replay adapter and compare independent
    scientific truth. Admission reports explicitly say admitted_unexecuted.
  Requirements: [external channel vectors](features/completion-group/external-channel-vectors.md).

### Cross-platform builds and runtime

- [ ] **QA-06** Windows/macOS build/runtime validation and CI; Linux checks alone
  do not establish cross-platform runtime support.
  - [x] Prepare lean preflight, native science tests and CLI smoke jobs for Linux,
    Windows x64 and macOS ARM64/Intel with pinned Rust and scoped caches.
  - [ ] Push/run the workflows later and retain actual native results.
  Guide: [native validation and delivery](native-delivery.md).

### Native packaging and release automation

- [ ] **QA-07** Native packaging and release automation after core functionality.
  - [x] Prepare CLI archives, DEB/PKG/MSI builders, checksums, license/example staging,
    tag/version verification, tested-artifact publication and per-OS install guides.
  - [ ] Verify all native installers and their install/upgrade/uninstall lifecycles;
    run tag publication after the later workflow push.
  Guide: [installation](../user/install/README.md).

### Optional Python adapter

- [ ] **API-01** Python bindings as a separate optional adapter, preserving the same
  simulation core, buffer lifetimes and observation contracts.

### Qt system workspace

- [ ] **UI-01** Build the planned Qt system workspace after core APIs exist:
  clickable source/TX/channel/RX blocks with purpose, settings and stage details.
  Include the separate OFDM transmission settings panel from the
  [updated diagram guide](../architecture.md#updated-workspace-diagram-guide):
  cyclic prefix, roll-off/windowing, time-interleaver depth and pilot settings;
  validate through Rust and apply changes to the next run.
- [ ] **UI-02** Add a role-based file panel and OS file picker; validate inputs
  before applying them to a new run, with clickable/editable PRBS15/text/binary
  source selection, size feedback and an explicit append-PRBS15 fill option.
- [ ] **UI-03** Present separate LDPC, BER, payload pre/post-CE, continuous/scattered
  pilot and PLC-data/preamble analyzers with shared bounded Rust results.
- [ ] **UI-04** Validate responsive jobs, cancellation, observation invariance and
  dark/light readability; add user guides and actual workspace screenshots.
  See the [starting diagram and interaction plan](features/visual-workspace/plan.md).
  All scientific computation stays in Rust; the UI remains unimplemented.

### Future DOCSIS scope

- [ ] **LATER-01** OFDMA architecture and implementation after downstream scope.
- [ ] **LATER-02** Channel bonding/multi-channel simulation or FDX-specific behavior
  only under a separate explicit design and validation scope.

The previously described “three major stages remaining” refers to the first
noiseless waveform milestone: pilots/PLC, waveform TX/RX, then one-second
integration. It does not mean all the broader product items above are complete.

### Pilot/PLC supplied-H reference evidence

- [x] Provide a [noiseless measured-H demonstration](../user/impairments/pilot-plc-constellation.md)
  with separate continuous/scattered-pilot, PLC-data and preamble plots using
  existing probes; verify modulation geometry and Y=HZ in Rust. This does not
  itself complete CH-EST-RX-01..06. The later shared collector/estimator is now
  implemented and validated separately in the linked receiver implementation record.
