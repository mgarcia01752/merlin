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
| `signal_processing::receiver::channel_estimation::search` | Bounded bootstrap hypothesis selection, integer/fractional CFO and two-symbol phase outputs |
| `WaveformReceiver::acquire_search` | Live pilot receiver using selected geometry and continuous total-CFO correction; initial samples consumed once |
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
| `input_format::{detect,InputFormat,json_bytes}` | Content-based binary/JSON/text classification without filesystem access |
| `scenario::pnm_json::import_profiles` | Standalone profile JSON validation shared with the CLI; available with `pnm-input` or `simulation` |
| `scenario::pnm_json::{import,JsonCapture}` | Current-decoder JSON import, exact grid/time policy, bounded expansion and hashed JSON provenance |
| `signal_processing::channel::{awgn,impairments}` | Seeded sample noise and frozen physical-grid H/residual/variance plans |
| `simulation::channel::measurement::{MeasurementSelection,captured_layout}` | Explicit profile/source admission and captured PLC/pilot layout |
| `profile::candidate` | Captured assignments and editable hypotheses |
| `profile::{evaluation, document}` | Temporal screening, candidate documents and role verification |
| `signal_processing::{modulation, noise}` | QAM, soft demapping and residual-noise models |
| `signal_processing::noise::lte_ingress::{LteDownlinkIngress, IngressResult}` | Fixed10MHz Band12 frequency-envelope approximation; combines existing residual and added interference power, preserves nulls and exposes per-carrier added ratios; no waveform or thermal floor |
| `signal_processing::modulation::{map_symbol, qam_llrs}` | Shared label-ordered mapping and reference soft demapping through16K-QAM |
| `signal_processing::modulation::randomization::CellRandomizer` | Stateful data/NCP cell randomization and soft-bit inverse through16K-QAM |
| `signal_processing::modulation::interleaving::BitInterleaver` | Short8/9 full uniform parity/twist/demux and soft inverse through16384-QAM |
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

## Paired receive constellation

`simulation::channel::constellation` exposes `ConstellationSelection`,
`ConstellationPair`, `ConstellationResult`, `ConstellationStatus` and count limits.
Set `ChannelRunSettings.constellation` before constructing a run. The stepper's
`constellation()` borrows committed observations; `take_constellation()` returns
confirmed partial/final pairs and disables further diagnostic retention only.
`ChannelJobResult.constellation` contains the same owned result on every job
termination status when requested. No Python/Qt types or file paths enter the
scientific API. See the [component guide](../user/impairments/paired-constellation.md)
for coordinate meaning, finite bounds and reference scaling.

## Receiver Channel Estimation (CH-EST-RX) and applied measurements

`signal_processing::receiver::channel_estimation` exposes receiver mode, pilot LS
observations, full bounded estimates and interpolation through `PilotChannelEstimator`.
Its `timing` module exposes the bounded CP search; `WaveformReceiver::from_pilots`
and `acquire_from_pilots` compose independent reception without supplied H.
The latter still needs known numerology and PLC phase.

`ChannelRunSettings.receiver_mode` selects supplied or pilot-estimated correction.
The stepper's `channel_estimate()` borrows the latest estimate. The job/CLI carries
the same optional owned `channel_estimate`. No observer is required to obtain it.
`protected_constellation` configuration/results and borrowed/take operations
provide four finite acquisition-safe receive-pair streams. See the
[analyzer guide](../user/receiver-estimation/analyzers.md) for metadata and limits.

`transmission::layout::bit_loading::source_bit_loading` implements Ci=P^-1(Bi).
`PayloadBitOrder` exposes the uniform/mixed coded-bit permutation before cell
grouping. Applied-profile TX/RX consume the same verified profile processing;
`SoftDataCell.llrs` is now variable-length, including empty zero-loaded cells.
The existing default constructors retain their uniform256 behavior.

`simulation::channel::timeline::ImpairmentEpoch` schedules explicit physical
measurement changes. Imported `MeasurementSelection.transitions` derives the
schedule from original capture epochs; `use_rxmer_noise=false` selects a CE-only
static experiment while preserving source association. Source/profile/cell state
is independent of these measurement updates. Library callers never need paths,
Qt objects or a Python runtime. See the [profile](../user/payload-transport/applied-profiles.md)
and [timeline](../user/impairments/measurement-timeline.md) guides.

## Shared simulation errors

`SimulationError` is the simulation core's shared Rust error type, re-exported
from the crate root. It contains a stable `code` and a readable `message`,
implements Rust's standard error/display traits, and serializes those same
fields for adapters. The type rename does not change existing JSON error codes;
`InvalidLabInput` remains the established invalid-input category.

## Shared spectrum analysis and TX preview

`signal_processing::waveform::spectrum::estimate` analyzes bounded contiguous
complex samples using the existing centered FFT, periodic Hann, 50% overlap and
linear-power averaging. `PowerSpectrum` retains full normalized density and
frequency metadata; `buckets` supplies at most 2048 contiguous min/max spans.
`density_db` keeps zero/unavailable density distinct from finite power.

`simulation::transmitter_preview::TransmitterPreviewRequest` freezes accepted
transmission settings, captured Profile A and immutable source configuration.
`generate` reuses production TX stages with cooperative cancellation and scalar
progress. Its result retains bounded actual samples, first-sample identity,
source generation/EOF metadata and full PSD. It runs no channel or receiver.
The estimator and its units are reusable by other time-sample tap adapters;
capture-frequency labels are metadata, not RF upconversion.

`signal_processing::waveform::timing::symbol_timing` consumes validated
`WaveformGeometry` and optional nominal cable velocity factor. It returns bounded
actual window coefficients, symbol/CP/roll-off times and ideal flat-guard path
budgets. Unknown VF yields no distance; invalid values return a structured error.
The explicitly assumed two-way reflector spacing is half the extra path length,
with zero reserved timing margin, not guaranteed receiver protection.

## Finite file sources

`simulation::source::file` exposes `FilePayload`, `PrbsFill`, `FileSource`,
`FileVerifier`, `FileSourceSummary` and `RecoveredFile`. `PayloadSourceConfig`
selects continuous PRBS or validated immutable file data. `FilePayload::new`
checks exact original bytes and optional target/seed; cloned configs share those
bytes without serializing them. `FilePayload::with_fill` revalidates target/seed
while sharing original byte storage, allowing interactive policy changes without
copying payloads. The core remains path-independent.

The optional `source-input` feature exposes PRBS state/validation, `FilePayload`,
`PrbsFill`, `SourceSelection` and `PayloadSourceConfig` without FEC, waveform or
runner dependencies. Payload-sized generation, file comparison and runner APIs
remain gated by `simulation`; their established imports are unchanged.

`ChannelRunSettings.source` and `ChannelDocument.settings_with_inputs` select
this path. Existing `settings`/`settings_with_scenario` methods continue to serve
PRBS requests; file requests need supplied bytes. `ChannelRun.recovered_file()`
copies the bounded recovered original prefix. Job results own that same optional
`recovered_file`; JSON serializes metadata only, with `bytes()` for binary output.
`ChannelSummary.file_source` contains segment/completion accounting, absent for
continuous PRBS. See [file source design](../user/simulation/file-source/README.md)
for cursor, padding, truth and partial-export contracts.

## Downstream FEC shortening

`fec_codec::shortening::{Shortening, ShortenedCodec}` provides checked original
lengths1..=14232 and reusable shortened encoding/decoding. Geometry methods expose
BCH prefix insertion/removal, LDPC tail insertion/removal, received likelihood
restoration and omitted-zero consistency independently. `EncodedShortenedBlock`
and `DecodedShortenedBlock` retain bounded stage outputs and original-length
payload decisions. `compare_payload` counts exactly K original bits independently
of actual decoder flags. `success()` additionally requires known-zero consistency.

Existing full-block APIs remain unchanged. Finite RX saturation is explicit in
`shortening::decoding::SHORTENED_ZERO_LLR_MAGNITUDE`. No filesystem, Qt or Python
type enters these interfaces. See the [component guides](../user/fec-shortening/README.md)
and [FEC-03 design](../development/features/fec-shortening/design.md) for source
applicability, equations and test-point shapes. QAM-03 integration is described below.

## QAM decisions and variable word transport

`signal_processing::modulation::slice_symbol` returns `HardDecision` containing
label, reference I/Q, received-minus-reference error and squared distance. Exact
computed ties choose the lowest label. It does not replace soft demapping.

`interleaving::BitInterleaver` accepts all ten downstream data orders. Its output
indices include appended-zero positions >=16200 where required; encode accepts
16200 mother bits, while reverse accepts the selected padded count and returns
16200 mother likelihoods. Direct twist/demux stages accept that padded size.
`interleaving::shortened::ShortenedBitOrder` takes checked `Shortening` geometry
and exposes K+1968-coordinate ordering/inverse, with unused matrix holes omitted.

`transmission::packing::shortening` exposes odd-byte transport validation and
received-span payload-length discovery. `SymbolPacker::enqueue_shortened_bits`
accepts a validated already-ordered shortened word in profile-aware mode.
`PayloadTransmitter::enqueue_shortened` provides actual FEC and ordered-bit test
points. `PayloadReceiver::with_shortened_words` enables received-boundary length
discovery from live acquired descriptors. `VariablePayloadCollector` is the
independently callable bounded receive stage; `RecoveredPayload::payload()`
borrows original-length decisions, with `shortening` and known-zero consistency
retained separately from the actual LDPC/BCH stage outcomes.

`simulation::payload::shortened::run_shortened_waveform` accepts explicit settings,
layout, physical loading and1..16 original words, plus bounded waveform probes.
It returns serializable per-word actual outcomes and independent error counts.
See [component guides](../user/qam-processing/README.md) for lifecycle, math and
bounds. No new general channel request schema or Python/Qt dependency is added.

## Planned Python convenience class

After the Qt UI milestone, API-01 proposes one `MerlinSimulation` class wrapping
the existing Rust operations. It holds settings, source, optional measurements,
limits and the latest result. No Python package is available yet. See the
[high-level class proposal](../development/features/python-adapter/design.md).

## Shared measurement inputs and playback

`scenario::pnm_json::{import_rxmer, import_channel}` admit standalone current
decoder JSON using the existing header/grid/unit contracts. Adapters retain
original binary/JSON bytes and their provenance separately from intermediate JSON.
`scenario::measurement_view` projects complex response values and supplies bounded
min/max views with gaps and Rust-derived units/frequencies.

`simulation::channel::measurement_playback::MeasurementPlayback` freezes exact
compatible RxMER frames and explicit static/average/hold/interpolated-power policy.
Its `at` operation returns residual powers with source indices and interpolation
weight without changing original epochs or RNG state. `span` and
`interval_milliseconds` use frozen OFDM cadence. `validate_response` checks imported
H or explicit unity against the captured profile. These operations are available
for adapters and are connected to core `ChannelRunSettings.playback`. The optional
`PlaybackConfiguration` retains original frames/profile and explicit mode/clock;
transitions update residual powers without resetting RNG or waveform history.
`channel::job::run_channel_controlled` adds a pause gate at OFDM-step boundaries,
excluding paused host time. Existing `run_channel` retains its unpaused behavior.
`TestPointProbe::set_phase_filter` filters before consuming observation budgets.
