# Public API

DOCSIS OFDM Channel Sim's scientific entry points are public Rust APIs. The standalone CLI calls
those same functions. Future Python bindings and an optional Qt presentation
component will adapt inputs and results without duplicating scientific logic.

| Public module | Responsibility |
| --- | --- |
| `transmission::{TransmissionSettings, SubcarrierSpacing}` | Fixed single-channel numerology and PHY validation |
| `transmission::{TransmissionTiming,DurationTiming}` | Legacy one-second schedule and checked64-bit duration/sample-clock planning |
| `simulation::channel::{ChannelRun,ChannelRunSettings,ChannelSummary,ChannelStep}` | Frozen duration session, one-symbol stepping, independent PRBS comparison and bounded typed outcomes; shared bounded job/CLI lifecycle available |
| `simulation::channel::{run_channel,ChannelJobLimits,ChannelJobResult,ChannelJobStatus}` | Cooperative token/runtime limits, progress, finite word retention and preserved complete/partial science |
| `simulation::channel::accounting::{ObservationWindow,WordPlacement,WindowTotals}` | Source-symbol/sample window classification and whole-word BER/information-bit rate |
| `simulation::channel::document::ChannelDocument` | Versioned validated CLI/future-adapter request, with millisecond duration default |
| `simulation::test_points::JsonlProbeExporter` | Generic bounded versioned JSONL writer; no filesystem/DSP ownership |
| `signal_processing::modulation::interleaving::{parity,twist,demultiplexing}` | Independent permutations, binary forward and finite soft-inverse boundaries |
| `simulation::runner::simulate_with_probes` | Existing coding experiment with source/noise observation and unchanged RNG draws |
| `transmission::layout::{CarrierLayout, CarrierLayoutSettings}` | Immutable physical bin roles and per-symbol scattered pilots |
| `transmission::layout::ncp::select_ncp_cells` | Eligible NCP cells in the pre-interleaving domain |
| `transmission::packing::{SymbolPacker, PackingSettings, PackedSymbol}` | Stateful256-QAM queue, symbol plans and NCP capacity reservations |
| `transmission::ncp::{NcpMessage, NcpFieldCodec}` | Control serialization, encoded fields and CRC-accepted receiver pointers |
| `transmission::plc::{PlcFrameGeometry, ExtendedTimestamp, MessageChannelBlock, PlcInformationBlock}` | Fixed dimensions, validated timestamp/MC bytes and MSB-first FEC inputs; analysis-only compatible |
| `transmission::plc::{TimestampMessageBlock, PlcMessageFrame}` | CRC-backed TS+MC frame construction/parsing with the simulation feature; independent PLC parity generation through the adapter below |
| `transmission::plc::{PlcCodewordNibbles, InterleavedPlcBlock, block_interleaving}` | Validated nibble grouping, block permutation and hard/soft inverse; analysis-only compatible |
| `transmission::plc::{PlcRandomizer, RandomizedPlcBlock, DescrambledPlcBlock}` | Independent known-phase PLC state,96-label masks,384-LLR soft inverse and direct bounded snapshots; analysis-only compatible |
| `transmission::plc::{descriptors,mac_packets,scheduling,reassembly}` | Fixed acquisition packets, CRC validation, repeats and MC segmentation/reassembly; analysis-only compatible |
| `transmission::plc::{preamble,mapping}` | Verified BPSK columns, normalized16-QAM and soft demapping; analysis-only compatible |
| `transmission::frame::{assemble,PhysicalFrequencySymbol}` | Physical protected bins, explicit RMS and preserved missing history |
| `simulation::plc::{generate_frame,PlcFrameSignals,PlcFrameReceiver}` | Bounded known-phase PLC frame TX/RX with direct stage results |
| `transmission::plc::receiver_configuration::{ReceiverConfigurationState,ReceiverConfigurationStatus,ReceiverConfiguration}` | Fixed-session descriptor admission, readiness gate and immutable received channel/profile settings; analysis-only compatible |
| `simulation::plc::PlcAcquisitionReceiver` | Composed PLC IQ→FEC→MC packet reassembly→configuration, with retained decode/admission diagnostics |
| `signal_processing::waveform` | Frozen geometry, centered TX/RX transforms, separate prefix operations, window, overlap history and bounded indexed sample blocks; analysis-only compatible |
| `signal_processing::receiver::{timing,equalization,extraction}` | Known-reference RX framing, response compensation and protected-role extraction; analysis-only compatible |
| `signal_processing::channel::reference::NoiselessChannel` | Unity sample transfer with preserved indices and explicit TX/RX planes |
| `simulation::waveform::{WaveformTransmitter,WaveformReceiver}` | Separate state owners and directly inspectable bounded TX/RX results |
| `simulation::waveform::{observation,reference}` | Thirteen bounded stage producers and shared short actual-PLC waveform verifier; no one-second payload-run claim |
| `transmission::symbols::{assemble_packed,TransportCell}` | Actual randomized data/NCP/BPSK assembly; diagnostic TX identities excluded from transported values |
| `signal_processing::receiver::payload` | Received-profile checks, independent control hypotheses, bounded collector and source decoder; simulation feature required |
| `simulation::payload::{PayloadTransmitter,PayloadReceiver}` | Continuous fixed-profile state owners; live acquired readiness, source/channel indices and explicit startup/drain |
| `simulation::payload::{observation,reference}` | Ten bounded TX/RX producers and actual encoded-payload waveform verifier with post-decode truth comparison |
| `simulation::plc::observation` | Bounded PLC and physical frequency-grid observations |
| `simulation::plc::PlcInterleavingStages` | Immutable coded/interleaved stage outputs and bounded optional label observations |
| `fec_codec::plc::{PlcFecCodec, PlcBlockResult, puncturing}` | Separate PLC mother/punctured LDPC, restored LLRs and actual decoder iterations |
| `transmission::plc::{PlcBlockCodec, EncodedPlcBlock, DecodedPlcBlock}` | Thin typed/serializable PLC coding adapter; failed decisions remain diagnostic |
| `fec_codec::{ncp::NcpFecCodec, crc::crc24d}` | Separate NCP control LDPC and CRC-24-D |
| `signal_processing::interleaving::{InterleaverSettings, SymbolInterleaver}` | Bounded cell time/frequency TX/RX streams and explicit startup/drain |
| `signal_processing::interleaving::{FrequencyInterleaver, TimeInterleaver}` | Independent permutation and generic convolutional delay operations |
| `signal_processing::pilots::{PilotSequence, PilotPower, generate_continuous, generate_scattered}` | Independent pilot sequence, normalization and physical-bin signals; analysis-only compatible |
| `simulation::pilots::{PilotGenerator, PilotStageResult}` | Thin pilot composition and optional observation of completed results |
| `simulation::test_points` | Typed borrowed pilot events, observer, finite selection and budgets |
| `transmission::layout::PilotReference` | Physical-bin association and pre-interleaving pilot masks |
| `scenario::{grid, observation}` | Exact identity, source provenance and full-resolution observations |
| `scenario::association` | Explicit-tolerance timestamp association |
| `scenario::document` | Versioned decoded scenario inputs |
| `scenario::pnm_json::{import,JsonCapture}` | Current-decoder JSON import, exact grid/time policy, bounded expansion and hashed JSON provenance |
| `signal_processing::channel::{awgn,impairments}` | Seeded sample noise and frozen physical-grid H/residual/variance plans |
| `simulation::channel::measurement::{MeasurementSelection,captured_layout}` | Explicit profile/source admission and captured PLC/pilot layout |
| `profile::candidate` | Captured assignments and editable hypotheses |
| `profile::{evaluation, document}` | Temporal screening, candidate documents and role verification |
| `signal_processing::{modulation, noise}` | QAM, soft demapping and residual-noise models |
| `signal_processing::noise::lte_ingress::{LteDownlinkIngress, IngressResult}` | Fixed10MHz Band12 frequency-envelope approximation; combines existing residual and added interference power, preserves nulls and exposes per-carrier added ratios; no waveform or thermal floor |
| `signal_processing::modulation::{map_symbol, qam_llrs}` | Shared label-ordered mapping and reference soft demapping through16K-QAM |
| `signal_processing::modulation::randomization::CellRandomizer` | Stateful data/NCP cell randomization and soft-bit inverse through16K-QAM |
| `signal_processing::modulation::interleaving::BitInterleaver` | Short8/9 parity/twist/demux and soft inverse for16and256-QAM |
| `simulation::coding_loop::NoiselessQam256Codec` | Independent noiseless codeword trials with per-word decoder iterations |
| `simulation::source::{Prbs15,PrbsVerifier,PrbsComparison}` | Explicit PRBS15 state, bounded chunks, absolute bit identity and independent recovered-stream comparison |
| `simulation::payload::reference::{PayloadSource,run_payload_waveform_with_source}` | Explicit seeded/PRBS source selection and shared waveform verifier with post-decode source comparison |
| `simulation::{config, runner, results, source}` | Bounded trial orchestration and reproducible source state |
| `types::ofdm_modulation` | Normative constellation registry and implementation status |
| `fec_codec::{parameters, bch, ldpc}` | Mode dimensions and individual codecs |
| `fec_codec::concatenated` | Encode, reverse-decode and compare without a channel model |

Canonical modules are shown above. Existing imports under `math`, `profile::model`,
`profile::association`, `profile::simulation` and
`profile::document::ScenarioDocument` remain supported by re-exports. Existing
root `fec_codec::{encode_bch, decode_bch, LdpcCodec, ...}` imports also remain valid.
No duplicate scientific implementations are kept for compatibility.

New APIs should use documented units, typed input/results, structured recoverable
errors and explicit ownership/lifetime contracts. Library functions must not exit
the process, depend on GUI objects, require Python objects or emit results through
console text. Consumers preserve missing values and original scientific results.

Progress callbacks and cancellation are Rust job interfaces. A future binding
will manage Python interpreter locking, callback delivery and buffer lifetimes.
No Python dependency, binding package or stable C ABI is introduced now.

See the [public-api skill](../../skills/public-api/SKILL.md) for API development
requirements and [JSON contracts](documents.md) for the existing serialization.

Existing typed operation outputs remain usable directly. The common observer
covers pilot/PLC, physical/waveform, payload/NCP and source/legacy-noise boundaries
with the simulation feature. The bounded [export adapter](../user/simulation/probe-export.md)
and [channel CLI](../user/channel-runner/cli.md) serialize selected original views.
Independent [bit-processing stages](../user/bit-processing/README.md) expose direct
intermediate arrays without requiring observers. Underlying analysis-only DSP
remains available without codec-backed simulation. See the
[checklist](../development/checklist.md) for still-proposed producers and modes.
