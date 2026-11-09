# DOCSIS OFDM Channel Sim architecture

This is the maintained architecture for this repository. The
[implementation checklist](development/checklist.md) records completed work,
remaining requirements and the next implementation step. Detailed equations and
examples live in the linked stage guides; private evidence and investigation
notes live under ignored `development/`.

Status describes the **current source checkout**, including work awaiting commit.
It does not imply a published release or external DOCSIS interoperability.
Review this document and the checklist at feature completion and each commit-message
request. Update affected design/status before supplying the command; partial work
remains unchecked until its implementation and validation are complete.

## Purpose and scope

Build a standalone, DOCSIS-focused Rust scientific engine with two modes:

- **Ideal/reference:** known payload, fixed single downstream OFDM channel up to
  192 MHz, initially uniform 256-QAM, noiseless recovery first, then controlled
  impairments. The target observation is one second, with separate warm-up/drain.
- **Measurement-driven:** condition a simulation on supplied RxMER, downstream
  channel estimates and an applied modulation profile. Preserve exact grids,
  missing readings, timestamps and provenance. Establish reference planes before
  combining impairments so measured residual noise is not counted twice.

The full one-second waveform runner and measurement-driven waveform model are
not implemented. The existing CLI runs the earlier 16-QAM coding experiment;
256-QAM packing/interleaving integration is currently exercised by library APIs,
examples and tests. The seeded source is implemented; PRBS is still TODO.

Downstream mapping and reference soft demapping cover constellations through
16384-QAM. Complete coding support must be tracked separately for each order;
current short8/9 bit processing supports uniform 16/256-QAM. See
[modulation coverage](reference/ofdm-support.md). DOCSIS4.0 FDD delegation does
not establish FDX behavior. OFDMA is a later scope.

All FEC, DSP, channel models and scientific calculations are Rust, including
algorithm dependencies. Scientific crates forbid unsafe code. A future Qt
presentation component and Python bindings consume the public Rust APIs;
neither owns scientific calculations. There is no UI or binding implementation
in this stage, and no device capture/control or network-service dependency.

## Modules and stage boundaries

**Each distinct algorithm stage has a focused module and a directly testable typed
input/output boundary.** Stateful stages own explicit structs; stateless stages
can use functions. Do not create empty object wrappers or files solely to mimic
classes. Orchestrators connect components without hiding their intermediate data
or implementing a second copy of their mathematics.

Existing composite stages that do not expose all intermediate results are tracked
as gaps in the checklist. In particular, parity interleaving, column twisting and
bit demultiplexing currently share a composed permutation. They need explicit
intermediate boundaries before claiming complete stage instrumentation.

Current source ownership:

```text
crates/
├── fec-codec/src/
│   ├── bch.rs                 Payload BCH encoder/decoder
│   ├── ldpc.rs                Payload LDPC encoder/soft decoder
│   ├── concatenated.rs        Composition and independent payload comparison
│   ├── crc.rs                 CRC-24-D
│   ├── ncp/                   NCP mother matrix, shortening/puncturing and decoder
│   └── parameters.rs          Verified coding dimensions
├── ofdm/src/
│   ├── types/                 Shared identities, units and modulation registry
│   ├── transmission/
│   │   ├── settings.rs        Fixed numerology
│   │   ├── timing.rs          Integer sample-clock schedules
│   │   ├── layout/            Physical roles and inverse pilot reference
│   │   ├── packing/           Bounded codeword queue and symbol plans
│   │   ├── ncp/               Messages, control mapping and field CRC acceptance
│   │   └── plc/               Geometry, timestamp/MC blocks, frames and FEC inputs
│   ├── signal_processing/
│   │   ├── modulation/        QAM, LLRs, bit processing and cell randomization
│   │   ├── interleaving/      Separate time, frequency and composition modules
│   │   ├── pilots/            Shared sequence, power and separate CP/SP signals
│   │   ├── noise/             AWGN and measured residual-noise processing
│   │   └── ...                Analytical bit-loading/capacity helpers
│   ├── simulation/
│   │   ├── source/            Seeded source; PRBS remains TODO
│   │   ├── pilots/            Typed stage composition and optional observation
│   │   ├── test_points/       Borrowed events, selection and finite dispatch budgets
│   │   ├── coding_loop/       Independent 256-QAM codeword trials
│   │   ├── runner.rs          Existing 16-QAM trial orchestration
│   │   ├── config.rs          Work limits and receiver policy
│   │   └── results.rs         Counters, truth comparisons and confidence bounds
│   ├── scenario/              Decoded observations, grids and association
│   └── profile/               Hypotheses, role protection and screening
└── cli/src/
    ├── main.rs                Startup and exit
    ├── arguments.rs           CLI syntax
    ├── commands/              Thin operation adapters
    └── json_io.rs             Bounded input and safe new-file exports
```

Dependencies flow from CLI/future adapters to `docsis-ofdm-fec`, then to the
optional `fec-codec`. Analysis-only builds exclude codec-backed simulation.
Preserve established imports through re-exports when reorganizing modules;
each calculation retains one canonical implementation.

## Transmission and receiver flow

The diagram describes the target channel path. Dashed-border boxes are planned
components. A solid box means the algorithm/API exists, not that a complete
waveform runner already connects every box.

```mermaid
flowchart TB
    SOURCE["Source payload · seeded now, PRBS TODO"] --> BCH["BCH encoding"]
    BCH --> LDPC["LDPC encoding"] --> BITS["Parity → twist → demultiplexing"]
    BITS --> PACK["Codeword packing"]
    PACK --> NCP["NCP messages → CRC → control FEC"]
    PACK --> CELLS["Typed data/NCP/filler assembly"]
    NCP --> CELLS
    CELLS --> RAND["Cell randomization"] --> QAM["QAM mapping"]
    QAM --> TIME["Time interleaving"] --> FREQ["Frequency interleaving"]
    PILOTS["Continuous/scattered pilot signals"] --> FRAME["Physical-bin assembly"]
    PLC["PLC preamble, messages and signal"] --> FRAME
    FREQ --> FRAME --> IFFT["IFFT"] --> CP["Cyclic prefix"]
    CP --> WINDOW["Windowing and overlap-add"] --> CHANNEL["Reference or impaired channel"]
    CHANNEL --> TIMING["RX timing and block framing"] --> FFT["CP removal and FFT"]
    FFT --> EQ["Pilot/PLC extraction and equalized data bins"]
    EQ --> INVF["Frequency de-interleaving"] --> INVT["Time de-interleaving"]
    INVT --> DEMAP["Soft demapping and inverse randomization"]
    DEMAP --> NCPRX["NCP FEC/CRC → received boundaries"]
    NCPRX --> BITRX["Data collection and inverse bit processing"]
    BITRX --> LDPCDEC["LDPC decoding · iterations/status"]
    LDPCDEC --> BCHDEC["BCH decoding"] --> TRUTH["Compare original and recovered payload"]
    SOURCE -->|"Independent truth"| TRUTH
    NCPRX -.-> SLICER["Optional hard slicer · profile-selected data I/Q"]
    classDef planned stroke-dasharray: 5 5;
    class CELLS,PLC,FRAME,IFFT,CP,WINDOW,CHANNEL,TIMING,FFT,EQ,SLICER planned;
```

The NCP and payload receiver paths use the appropriate separate demapping
hypotheses. Inverse randomization uses the recovered source-symbol phase, not the
later channel-output phase. Actual pilot signals are inserted after interleaving;
PLC, continuous pilots and exclusions never enter the reduced interleaver domain.
Initially timing/equalization can use known reference settings; blind acquisition
and estimated-channel operation are separate milestones, not implied by that mode.

Current integration tests already connect real payload/NCP coding, randomization,
QAM, cell interleaving, inverse pilot references and received pointer discovery.
The assembly/receive orchestration in those tests must become reusable production
modules before it is advertised as a standalone channel-run API.

## Test points and data access

Existing APIs expose useful intermediate arrays and results. The common typed,
bounded observer is implemented for **four pilot stages**. Other stage IDs below
remain proposed; their direct outputs are not yet wired to that observer. CLI/file
export and whole-run cancellation policy remain TODO. “Direct” means a caller can
obtain data through an existing stage API.

| Test point | Owner / data | Access now |
| --- | --- | --- |
| `source.payload` | `simulation::source`; source bits and seed/source identity | Direct seeded output; PRBS TODO |
| `tx.bch.word` | `fec_codec::bch`; BCH data plus parity | Direct |
| `tx.ldpc.word` | `fec_codec::ldpc`; systematic word plus parity | Direct |
| `tx.bits.parity`, `tx.bits.twist`, `tx.bits.demux` | Separate bit-processing stages; bit positions and cell labels | Final labels/permutation direct; named intermediate arrays TODO |
| `tx.packing.symbol` | `transmission::packing`; starts, roles, filler and offsets | Direct plan; IDs are transmitter truth only |
| `tx.ncp.messages`, `tx.ncp.crc`, `tx.ncp.encoded` | Message bytes, CRC, control-code bits/labels | Direct field/codec outputs |
| `tx.randomizer.cells` | Randomized labels and diagnostic LFSR state | Direct per-cell output/state |
| `tx.qam.iq` | Normalized I/Q with modulation and reference plane | Direct mapper output |
| `tx.time.cells`, `tx.frequency.cells` | Each cell-interleaver output with startup markers | Direct separate APIs |
| `tx.pilots.sequence`, `tx.pilots.continuous`, `tx.pilots.scattered`, `tx.pilots.power` | Separate sequence, signals, boost and physical locations | Direct values and bounded observer |
| `tx.plc.message` | `transmission::plc`; TS+MC information bytes and288-bit inputs | Direct bounded APIs; observer and OCD/DPD packet construction TODO |
| `tx.plc.coded`, `tx.plc.interleaved`, `tx.plc.randomized`, `tx.plc.preamble`, `tx.plc.iq` | Separate PLC coding, interleaving, randomization, preamble and mapping outputs | TODO |
| `tx.frame.bins` | Full physical frequency grid, roles and zero/excluded bins | TODO production assembler |
| `tx.ifft.samples`, `tx.cp.samples`, `tx.window.samples`, `tx.overlap_add.samples` | Separate waveform stages and sample indices | TODO |
| `channel.input`, `channel.output` | Channel-plane I/Q and applied impairment configuration | TODO waveform path; cell AWGN exists |
| `rx.timing`, `rx.cp.samples`, `rx.fft.bins`, `rx.channel.estimate`, `rx.equalized.iq` | Timing result, recovered grid, channel estimate and equalized data | TODO waveform receiver |
| `rx.frequency.cells`, `rx.time.cells` | Separate inverse cell-interleaver outputs and source phase | Direct |
| `rx.ncp.llrs`, `rx.ncp.accepted` | NCP soft inputs, accepted pointers and block iterations | Direct primitives; orchestration currently in tests |
| `rx.data.llrs`, `rx.descrambler.llrs` | Data likelihoods before/after sign inversion | Direct primitives |
| `rx.slicer.decisions` | Nearest label/reference point and distance/error vector | TODO hard slicer |
| `rx.bits.inverse` | LLRs restored to codeword order | Direct final inverse; individual reverse stages TODO |
| `rx.ldpc.bits` | Decisions, parity success and actual iteration count | Direct |
| `rx.bch.payload` | Recovered bits, status and corrected-bit count | Direct |
| `result.truth` | Actual bit/codeword comparisons and trial counters | Direct codec/trial results; channel-run aggregate TODO |

The implemented pilot contract provides borrowed bit arrays, sparse pilot records
and a power record, with explicit metadata and finite selection budgets. See the
[pilot guide](user/pilot-signals.md) for the executable example and exact behavior.
The broader contract, including future producers, must provide:

- Stage ID, direction and run/configuration identity; codeword/profile identity
  where known. Receiver identities must come from decoded signaling/session state,
  never transmitter diagnostic IDs supplied as hidden receiver input.
- Source-symbol and channel-symbol indices separately; sample-clock position when
  applicable, PLC phase, physical/reduced grid mapping, units and normalization.
- Explicit value type and shape: bits, labels, I/Q, roles, LLRs or typed metadata.
  Preserve missing/startup/drain markers and success/failure states.
- Immutable borrowed data during a callback or explicitly owned snapshots with
  documented lifetime. Do not expose mutable internal buffers or RNG state.
- Opt-in stage selection, symbol/codeword/sample ranges and explicit data limits.
  Stream large sample blocks; do not retain an entire second of waveform by default.
- A disabled-observer path and tests proving observation does not alter RNG draws,
  stage order, scientific outputs or decoder decisions. Export/sampling belongs
  to adapters; scientific calculations remain in Rust stage modules.

Native typed stage calls remain usable without observers. JSON is an export
adapter, not the only way to get data. Probe failures and cancellation need an
explicit runner policy that preserves already completed scientific results.

The PLC message boundary now exposes a typed extended timestamp, timestamp block,
message-channel block, complete TS+MC information frame and36-byte FEC inputs.
An idle frame is valid at that boundary but does not carry channel-acquisition
configuration. MAC OCD/DPD packets and repetition scheduling remain separate work;
PLC FEC and signal stages are not implemented. See the
[PLC message guide](user/plc-messages.md) and [evidence](reference/plc-evidence.md).

## Planned module ownership

These paths are the accepted direction for upcoming work, not existing files or
promises that the algorithms have been implemented. Refine a path in this document
before implementation if source analysis shows a better existing owner.

| Planned path under `crates/ofdm/src/` | Responsibility |
| --- | --- |
| `signal_processing/modulation/interleaving/{parity,twist,demultiplexing}.rs` | Separate intermediate and inverse boundaries behind the existing facade |
| `transmission/layout/additional_pilots.rs` | Specification-based additional continuous-pilot placement |
| `transmission/plc/{descriptors,mac_packets,scheduling}.rs` | Fixed-setting OCD/DPD packets, packet CRCs, segmentation and repetition; current message layer transports opaque segments |
| `transmission/plc/{coding,block_interleaving,randomization,preamble,mapping}.rs` | Remaining PLC signal stages and distinct state; invoke verified codec primitives |
| `transmission/symbols/assembly.rs` | Typed data/NCP/filler composition before randomization/mapping |
| `transmission/frame/assembly.rs` | Reduced-to-physical grid assembly and protected-role insertion |
| `signal_processing/waveform/{ifft,fft,cyclic_prefix,windowing,overlap_add}.rs` | Separate waveform transforms and inverse operations |
| `signal_processing/channel/` | Explicit reference-plane channel and impairment models |
| `signal_processing/receiver/{timing,estimation,equalization}.rs` | Known-reference first, then independently verified estimation/acquisition |
| `signal_processing/modulation/slicing.rs` | Diagnostic hard decisions; keep soft demapping for LDPC |
| `simulation/channel/{transmitter,receiver,runner,results}.rs` | Thin stateful orchestration and completed-result accounting |
| `simulation/source/prbs.rs` | Explicit PRBS polynomial, seed, continuity and comparison |

If PLC needs a new coding algorithm, its reusable primitive belongs in
`fec-codec`, selected only after local PHY verification. Do not assume payload or
NCP coding is interchangeable with PLC coding. Shared sequence/math definitions
have one owner; do not copy an LFSR into every consumer.

The hard slicer is diagnostic: nearest constellation point with an explicit tie
policy, symbol-error counts and data for EVM inspection. It must not silently
replace soft LLR inputs to LDPC. Its implementation remains on the checklist.

## Lifecycle, validation and future interfaces

Freeze numerology, layout and profile before each run. Carry partial codewords,
interleaver buffers and sequence phase across calls. Keep PLC and data/NCP
randomizer states separate. The cell-interleaver pair adds M-1 symbols of delay;
None history is not transmitted payload. Warm-up, one-second observation and
drain need separate accounting, including complete versus unfinished words.

Bound files, buffers, observations, work and decoder iterations. Preserve
completed results on cancellation. Keep decoder success, truth errors, measured
observations and analytical screening distinct. No completed comparisons means
no fabricated zero BER. The seeded baseline is reproducible on the tested build;
external interoperability and untested platforms remain separate validation work.

Future Python/Qt adapters call the same scientific APIs and test-point contracts.
They do not implement DSP, codec math or scientific JSON post-processing. Native
bindings, packaging and presentation are later milestones. Git/release helpers
exist, but native installers and multi-OS release automation do not yet exist.

## Stage references

- [FEC codec](user/fec-codec.md), [coding evidence](reference/coding-mode.md)
- [QAM mapping](user/qam-mapping.md), [256-QAM bit/coding loop](user/qam256-coding-loop.md)
- [Randomizer](user/cell-randomizer.md), [symbol packing](user/symbol-packing.md), [NCP](user/ncp.md)
- [PLC message blocks](user/plc-messages.md), [PLC evidence](reference/plc-evidence.md)
- [Pilot signals and test points](user/pilot-signals.md)
- [Physical layout](user/carrier-layout.md), [cell interleaving](user/cell-interleaving.md)
- [Transmission setup and timing](user/transmission.md), [public APIs](reference/public-api.md)
- [JSON contracts](reference/documents.md), [build/test](development/build.md), [Git/releases](development/git.md)

Pinned scientific sources and section/page evidence are recorded in those guides.
Use local versioned PHY sources before adding pilot/PLC details. Update this
architecture and the [checklist](development/checklist.md) together as stages change.
