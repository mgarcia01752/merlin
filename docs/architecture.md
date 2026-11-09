# DOCSIS OFDM Channel Sim architecture

DOCSIS OFDM Channel Sim is a standalone DOCSIS scientific toolkit with a Rust library and CLI.
Downstream OFDM is the current focus. OFDMA is deferred. All signal processing,
channel modeling, modulation, soft demapping and FEC are implemented in Rust.

```text
crates/
├── fec-codec/src/
│   ├── lib.rs                 Public exports and shared input validation
│   ├── parameters.rs          Supported coding dimensions
│   ├── bch.rs                 BCH encoding and decoding
│   ├── ldpc.rs                LDPC encoding and decoding
│   └── concatenated.rs        BCH→LDPC; reverse decoding and payload comparison
├── ofdm/src/
│   ├── lib.rs                 Module wiring and compatibility exports
│   ├── error.rs               Shared scientific diagnostics
│   ├── types/                 Domain aliases, units and carrier roles
│   ├── transmission/          Fixed channel numerology and one-second timing
│   │   ├── packing/           Bounded codeword queue and pre-interleaving symbol plans
│   │   └── layout/            Physical pilot/PLC roles and separate NCP eligibility
│   ├── signal_processing/
│   │   ├── modulation/        Embedded QAM through16K, bit ordering and soft demapping
│   │   └── noise/             Residual powers and controlled AWGN
│   ├── simulation/
│   │   ├── coding_loop/       Independent256-QAM cell trials and decoder metrics
│   │   ├── source/            Current seeded source; PRBS remains unimplemented
│   │   ├── config.rs          Receiver and work limits
│   │   ├── runner.rs          Connect stages; own cancellation and progress
│   │   └── results.rs         Stage comparisons, counters and confidence bounds
│   ├── scenario/              Grids, observations, association and input documents
│   └── profile/               Candidates, role protection and analytical evaluation
└── cli/src/
    ├── main.rs                Startup, signals and process exit
    ├── arguments.rs           CLI syntax
    ├── commands/              Focused simulate/evaluate adapters
    └── json_io.rs             Bounded file input and safe new-file output
```

The scientific crates forbid unsafe code and have no Qt, Python-runtime or native
codec dependency. Python remains useful for development/release tooling only.
The optional future Python and Qt adapters call the same public scientific API;
scientific functionality takes priority over either adapter.

Two simulation modes are planned:

- **Ideal/reference:** known symbols and channel, noiseless baseline or controlled
  AWGN; actual FEC outcomes establish reference behavior.
- **Measurement-driven:** exact-grid RxMER, downstream channel estimates and an
  applied modulation profile describe the supplied OFDM conditions. RxMER-based
  residual AWGN is implemented for the initial coding experiment. The complete
  channel-estimate-driven model remains pending. Establish measurement reference
  points before combining them; do not count equalized impairment twice.

Typed source inputs retain identity, units, timestamps, original grid positions,
missing readings, exclusions and provenance. Applications that decode binary
capture files retain original bytes and compute source hashes. A supplied JSON
hash is metadata, not authentication. No silent interpolation or resampling occurs.

Scientific results distinguish measured observations, analytical target screening
and simulated decoder outcomes. Individual frames and worst intervals remain
visible; an average curve must not hide transient impairments. MER-derived noise
averages linear error powers rather than arithmetic dB values.

`fec-codec` provides safe Rust encoder/decoder APIs. LDPC reuses the pinned
ldpc-toolbox matrix/decoder; BCH uses a bounded Rust implementation with documented
algorithm provenance and retained licensing. A coding round trip does not prove
DOCSIS interoperability. See [coding evidence](reference/coding-mode.md).

`docsis-ofdm-fec` owns shared scenario and candidate models, exact association,
analytical evaluation, simulation settings/results and document validation. Its
optional `simulation` feature allows analysis-only builds without codecs.
The CLI owns bounded file I/O, cancellation signals and safe new-file output.

Candidate edits preserve verified pilot, PLC and excluded roles. Saved result
claims are recalculated against supplied observations, and captured roles are
reverified. Internal candidate JSON does not deploy or activate a profile.

The downstream modulation target covers the pinned DOCSIS3.1/4.0 FDD definitions
through16384-QAM, while implementation status remains explicit per mode. Device
support requirements and implemented scientific capabilities are separate.
See [modulation coverage](reference/ofdm-support.md) and [public API](reference/public-api.md).

The isolated codec loop lives in `fec_codec::concatenated`. It can encode, reverse
decode and compare a payload without QAM, noise, profiles or a scenario. See
[the codec-loop example](../crates/fec-codec/examples/codec_loop.rs).
Source generation remains separate; no PRBS polynomial has been introduced.
The [FEC codec user guide](user/fec-codec.md) shows the current flow, block
dimensions and encoding/decoding equations.

A simulation runner coordinates stages but does not implement their algorithms.
Its source and AWGN operations share the same job RNG stream to preserve seeded
draw order. Result reporting owns truth comparisons and confidence calculations.

Existing `math::*`, `profile::model`, `profile::association`, `profile::simulation`
and `profile::document::ScenarioDocument` imports remain available through
re-exports. Each type and algorithm has one implementation at its canonical path.

`transmission` validates fixed single-channel setup and calculates integer
sample-clock timing independently of the simulation feature. The channel runner,
waveform and interleavers remain pending. See
[transmission settings and lifecycle](user/transmission.md).

`transmission::layout` owns immutable physical-bin geometry and128-symbol
scattered-pilot progression. NCP eligibility uses a separate pre-interleaving
domain; physical-bin indices cannot substitute for inverse interleaver mapping.
Additional-pilot generation and full placement compliance remain pending. See
[carrier-layout scope and equations](user/carrier-layout.md).

QAM resources under `crates/ofdm/resources/qam-table` retain PyPNM provenance.
The Rust build helper compiles label-ordered normalized references into static
arrays. Shared mapping and max-log demapping cover all downstream data orders
through16K-QAM; the full coding CLI remains16-QAM only. See
[mapping, scaling and evidence](user/qam-mapping.md).

`signal_processing::modulation::randomization` owns the data/NCP GF(2^12) LFSR
and cell-mask processing. It retains state across codewords, resets on explicit
128-symbol boundaries and reverses soft-bit randomization with LLR sign flips.
The packer must supply synchronized pre-interleaving cell order; no existing
simulation mode is silently changed. PLC has separate pending state/timing rules.
See [randomizer equations and lifecycle](user/cell-randomizer.md).

`modulation::interleaving` separates pinned coding parameters from reusable
permutation/label/soft-inverse operations.16and256-QAM are enabled; other orders
remain unsupported in bit processing even when their mapper exists.
`simulation::coding_loop` connects the256-QAM stages for one independent codeword
per call and exports actual LDPC iterations. It does not implement continuous
OFDM packing or channel timing. See [coding-loop flow](user/qam256-coding-loop.md).

`transmission::packing` owns a bounded queue of post-bit-processing256-QAM
codewords. It carries partial words across symbols, previews NCP/CRC reservations
and commits valid plans atomically. Callers supply the inverse-interleaved pilot
mask; the physical layout API alone cannot provide that mask. Typed NCP meanings
and diagnostic word IDs are available, but control-message encoding and receiver
boundary discovery remain pending. See [symbol packing](user/symbol-packing.md).
