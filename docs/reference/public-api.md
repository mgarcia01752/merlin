# Rust API Guide

For generated signatures, types and method documentation, see the
[Rust and Basic Python API reference](api-reference.md). This page remains the
architecture-oriented map of shared public operations.

MERLIN exposes its simulation engine through public Rust APIs. The CLI and native
Qt workbench reuse those APIs; optional native Python bindings adapt the same inputs
and results. Signal processing remains in Rust.

Use the [channel design guide](../user/channel-runner/README.md) for the run
lifecycle and the [component guides](../user/simulation/README.md) for subsystem
behavior. The table below maps public modules to their responsibilities.
Return to the [documentation index](../README.md) for other guides.

| Public module | Responsibility |
| --- | --- |
| `merlin::upstream::inputs::{UcdCapture,RxmerCapture,PreEqCapture}` | Existing native binary inspection plus bounded `from_source_json`/`to_source_json` lossless source transport; immutable originals, unchanged inspection JSON and explicit association remain separate ([measurement contract](../user/upstream/measurements.md)) |
| `merlin::common::RetainedFootprint` | [Passive retained-resource metadata](../user/api/retained-resources.md) from downstream inputs/Designer/controls/previews/requests/sweeps/results/archives and upstream controls/sessions/reports; payload bytes plus independent ordered-entry/shared-root counts, excluding private allocator/tree/refcount bookkeeping |
| `merlin::downstream::{FullLoopReceiverArchivePolicy,FullLoopReceiverArchive}` | [Detached finite receiver histories](../user/api/full-loop-cells.md#detached-independent-histories), original cell/decoder context, compatible scientific queries and exact preflight/bounded-copy operations; no full job/input retention |
| `signal_processing::receiver::channel_estimation::{MceSettings,ReliabilityEstimator,ReceiverReliability}` | Received-only MCE reliability policy, bounded continuous-pilot differences and training/tracking summary; no supplied H or exact simulator noise inputs |
| `signal_processing::channel::composition::{prepare_channel,PreparedChannel}` | Shared admission/execution/preview planning, total gain and separate pre-filter synthesis response |
| `signal_processing::channel::filter::{ChannelFilterSettings,ChannelFilterPlan,StreamingChannelFilter}` | Bounded static echo/CM reconstruction plans, coefficient/response/fit getters and persistent sample filtering |
| `transmission::{TransmissionSettings, SubcarrierSpacing}` | Fixed single-channel numerology and PHY validation |
| `transmission::{TransmissionTiming,DurationTiming}` | Legacy one-second schedule and checked64-bit duration/sample-clock planning |
| `simulation::channel::{ChannelRun,ChannelRunSettings,ChannelSummary,ChannelStep}` | Frozen duration session, one-symbol stepping, independent PRBS comparison and bounded typed outcomes; shared bounded job/CLI lifecycle available |
| `simulation::channel::{run_channel,ChannelJobLimits,ChannelJobResult,ChannelJobStatus}` | Cooperative token/runtime limits, progress, finite word retention and preserved complete/partial science |
| `simulation::channel::accounting::{ObservationWindow,WordPlacement,WindowTotals}` | Source-symbol/sample window classification and whole-word BER/information-bit rate |
| `simulation::channel::document::ChannelDocument` | Versioned validated CLI/future-adapter request, with millisecond duration default |
| `simulation::test_points::{ProbeContext,TransmitCellRole}` | Optional bounded TX source-grid role metadata for data/NCP/filler/pilot diagnostics; no receiver inputs |
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
No duplicate simulation implementations are kept for compatibility.

New APIs should use documented units, typed input/results, structured recoverable
errors and explicit ownership/lifetime contracts. Library functions must not exit
the process, depend on GUI objects, require Python objects or emit results through
console text. Consumers preserve missing values and original simulation results.

Progress and cancellation remain Rust job interfaces. The separate optional
Python adapter releases the interpreter lock during simulation and owns result
snapshots; progress callbacks are deferred. No Python dependency enters the
reusable core or CLI, and no stable C ABI is promised.

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
simulation API. See the [component guide](../user/impairments/paired-constellation.md)
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

The high-level `TransmissionConfiguration::spectrum_average_capacity(max_samples)`
plans complete analysis windows within a sample budget through that same owner.
Its native Python method is `spectrum_average_capacity(max_samples)`. Both retain
the existing work cap and reject budgets smaller than the accepted FFT without
reading samples or starting a run. Use the count with existing spectrum query
fields; retained availability remains separate. See the paired
[sample-budget examples](../user/api/full-loop.md#spectrum-sample-budgets).

`FullLoopResult::calibrated_spectrum_sequence(query)` provides paired normalized
ordered-frame metadata and calibrated ChannelSpectrum through the existing
sequence/calibration owners. Native Python exposes the matching method with
stage/snapshot/average/viewport arguments. Original gaps, overlaps and explicit
contiguity remain in the returned `sequence`; `power` carries electrical or
relative units and zoom-independent occupied-band power. See
[ordered-frame diagnostics](../user/api/full-loop.md#calibrated-ordered-frame-diagnostics).

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

`SourceInput::file_limits`, `file_info` and `with_fill` expose authoritative
finite-source budgets, constant-size original/fill metadata and immutable append
replacement. SourceFileLimits and SourceFileInfo retain explicit octet/bit units
and optional independently seeded policy. Replacement shares the validated
original byte buffer; native Python mirrors these operations with owned results.
See [source admission semantics](../user/api/input-admission.md#finite-source-limits-and-immutable-fill-editing).

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

## Basic Rust API and native Python bindings

Enable `high-level-api` on `docsis-ofdm-fec` to use `api::{Simulation,Merlin,Dpd,Ocd,RxMer,
ChannelEstimate,EvaluationOptions}`. Checked quantities and opaque IDs prevent
unit/identity confusion. Inputs are independently loaded and preserved; static
selection/preflight precedes explicit bounded `run()` calls. Owned FEC and
profile summaries share observation-window scope; whole-run diagnostics are
separately labeled. `pnm-input` adds direct native binary/JSON and bounded
file conveniences without Qt or Python. `Dpd::profile_ids_from_bytes` validates
decoded JSON catalogs; `CaptureLoader::profile_ids` adds binary/JSON dispatch.
Both return sorted owned `ProfileId` values without simulation. Python
`DPD.profile_ids_from_file` and `profile_ids_from_bytes` delegate to this same
Rust discovery and validation. Loaded `Dpd::profile_ids()` and Python
`dpd.profile_ids` expose the catalog retained during import without further
file access or decoding. The selected identity remains `profile_id`.

The optional `bindings/python` workspace converts inputs and results for the
same Rust API. See [working Rust usage](../user/api/README.md),
[Python setup and examples](../user/python/README.md) and
[acceptance/evidence](../development/features/high-level-api/requirements.md).

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
`TestPointProbe::set_stage_phase_filter` overrides that filter for one selected
producer before dispatch starts; `None` admits all phases for that producer.
This permits acquisition PLC messages beside observation-only waveform/payload
captures without spending the latter stages' budgets during acquisition.

## Receiver instrument observations and diagnostics

`ConstellationSelection.capture_ncp` optionally retains CRC-classified matched
NCP pairs through the shared inverse transport, independently of payload capture
completion. `ConstellationResult.ncp_pairs` carries actual receive/source/bin
coordinates and LLRs; payload pairs also retain actual post-descrambling cell LLRs.
No profile is inferred for NCP and no hypothesis beyond the received field's
consumed-cell count is classified as control.

`signal_processing::receiver::diagnostics` provides bounded response/gain/phase,
zero-filled band-limited impulse analysis and exact retained-LLR statistics.
`TestPointProbe::limit_stage_events` caps a selected producer before dispatch,
without spending the global budget on already-full snapshots. It must be set
before any callback; later reconfiguration is rejected. Qt uses these APIs and
the existing Rust slicer rather than implementing DSP in QML.

`signal_processing::channel::additional_loss` exposes typed Off/manual/cable-distance
models, explicit units and bounded gain evaluation. `ChannelImpairments.additional_loss`
composes the magnitude with measured H while preserving originals and the existing
post-equalization RxMER reference plane. See the [model guide](../user/impairments/additional-loss.md).

`signal_processing::channel::power::PowerConditioning` exposes typed scalar gain,
attenuation, optional nominal DATA electrical reference/output target and optional
independent receiver noise. `ChannelImpairments.power` composes these with H and
additional loss; playback preserves them. Independent noise is added after H and
attenuation and included in LLR variance. `channel_power` integrates full-resolution
linear PSD over an explicit inclusive bin band independently of any GUI viewport,
returning normalized power and optional rms dBmV/band-average6 MHz readings.
`TransmitterPreview.layout` retains frozen occupied-band geometry for integration.
See the [channel guide](../user/channel-measurements.md#launch-amplifier-attenuator-and-tappoints).

## Reusable FEC counters and carrier MER

`fec_codec::statistics::{FecCounters, FecOutcome}` (also re-exported at the codec
root and `simulation::channel` in the OFDM crate) is the shared codeword accounting
API. FecCounters owns private state; its public operations are `new`, `record`,
`snapshot`, `total_codewords`, `corrected_codewords`, `uncorrectable_codewords`,
`clean_codewords` and `undetected_errors`. FecOutcome classifies actual stage
acceptance and LDPC/BCH corrections without using iteration counts or source
truth as decoder status. Snapshots are copied values and serialize for adapters.
No Qt, Python, filesystem or JSON parsing API is required to use the type.

`LdpcResult::hard_decision_changes` exposes actual changed non-erased received
sign decisions. Shortened decoding excludes restored synthetic coordinates.
`ChannelSummary::fec` counts all completed words; `WindowTotals::fec` counts only
whole-contained observation words, including their completion during drain.
Both use the same type and getters. UFEC is a union of failed decoder stages;
undetected source mismatches remain separate.

`FullLoopResult::fec_recovery() -> FullLoopFecRecovery` exposes the existing
whole-run source-verified recovery projection through the accepted API. Native
Python `FullLoopResult.fec_recovery()` returns the matching owned dictionary.
The five generated/recovered/rejected/mismatched/unrecovered counts preserve
actual complete/partial scope and remain independent of retained word details.
See the [recovery semantics](../user/api/full-loop.md#source-verified-whole-run-recovery).

```rust
// `result` is a completed or partial ChannelJobResult.
if let Some(window) = &result.summary.window {
    let counters = window.fec.snapshot();
    let total = counters.total_codewords();
    let corrected = counters.corrected_codewords();
    let uncorrectable = counters.uncorrectable_codewords();
    assert_eq!(total, counters.clean_codewords() + corrected + uncorrectable);
}
```

`simulation::channel::performance::CarrierMer` exposes `observe`, `samples`,
`signal_power`, `error_power` and `mer_db`. `ChannelRun::carrier_mer` and
`ChannelJobResult::carrier_mer` return physical-bin-indexed observation sums.
Missing and zero-error observations do not become arbitrary finite MER values.
These typed operations support ordinary Rust consumers and the optional native
Python API. The reusable libraries retain no Python runtime dependency.

Channel admission reports `PlaybackObservationTooShort` when a nonzero final
playback transition is at or beyond the observation's complete-symbol count.
The validation condition is unchanged; consumers can use this stable error
code rather than matching the English message.

## RxMER capture export

The independent Apache `pnm-input::rxmer_export` module exposes `ExportFormat`
(Pnm/Json), `RangePolicy` (Reject/Clamp), `ExportSummary` and structured
`ExportError` (code, optional capture/subcarrier, message). `encode` and
`encode_json` return owned bytes and summary from a borrowed validated
`RxMerFrame`; `RxMer::frame()` exposes that immutable frame. `preflight` and
`preflight_format` validate bounded collections. `save` defaults to binary;
`save_with_format` adds an explicit format. Both accept new single/collection
destinations, bounded provenance JSON, cancellation flag and capture progress.
No simulation, Qt or Python is required. See rustdoc for exact signatures and
[publication rules](../development/features/rxmer-designer/pnm-export-plan.md).

Python `RxMer` mirrors these operations as `to_pnm_bytes(*, clamp=False)`,
`pnm_export_summary(*, clamp=False)`, `save_pnm(path, *, clamp=False)`,
`to_json_bytes()` and `save_json(path)`. The native adapter releases the interpreter
lock. `ApiError` includes `export_code`, `capture` and `subcarrier`. JSON never
clamps/rounds; byte conversion alone does not write a recipe manifest.

## High-level receiver policy

`api::ReceiverMode` re-exports the existing engine enum. Add it through
`EvaluationOptions.receiver_mode` or `.with_receiver_mode(...)`;
`CmChannelEstimated` remains default, with `PilotEstimated` and `MerlinChannelEstimated`
also supported. Preparation copies it into `ChannelRunSettings.receiver_mode`.
`ChannelSelection` still chooses applied H independently. MCE uses existing
engine-default policies here; custom MCE tuning is not added in this milestone.
Report options serialize the selected mode and assumptions distinguish supplied,
pilot and MCE correction. Python exposes the same choice through `ReceiverMode`
uppercase aliases and a typed mutable request property/constructor keyword.

The expanded MCE name is `MerlinChannelEstimated` in Rust and
`ReceiverMode.MERLIN_CHANNEL_ESTIMATED` in Python. Saved WB/CLI/report JSON
retains `merlin_estimated` for compatibility; input also accepts
`merlin_channel_estimated`. This is a naming change only.

## Saved workbench loader

`pnm_input::Workbench::{from_file, from_bytes}` verifies supported self-contained
WB schema 1/2, retains originals and preflights saved profile controllers without
running them. `controllers()` borrows idle configured controllers;
`into_controllers()` transfers them for manual API overrides. `original_document()`
shares exact WB JSON. `run()` / `run_with_control()` return one owned BatchReport
per selected profile, preserving independent controller scopes.

`EvaluationOptions.processing: ProcessingOptions` copies existing MCE,
channel-filter, additional-loss and power policies into engine jobs; defaults
preserve prior behavior. Python option snapshots retain the same settings, with
`processing` returning a copied inspection dictionary.

Python `Workbench.from_file(path)` / `from_bytes(data)` call this Rust loader.
`profile_ids` and `options` expose copied settings. `run()` / `run_with_control()`
return a list of batches, release the GIL during simulation, and reject concurrent
runs of the same WB with Busy. See the [WB contract](../user/python/workbench.md)
for supported PRBS15/static modes, unsupported capabilities and finite bounds.


### Retained profile collections

`Simulation::load_profiles` accepts validated profiles decoded by
`pnm_input::CaptureLoader::profiles`. Legacy `Merlin::from_profiles` and
`CaptureLoader::controller_from_profiles` remain available.
`profile_ids`, `profile_by_id` and `select_profile` operate on retained objects.
`run_all_profiles` and its cancellation-aware variant use the same core engine
as `run`, preserving selection and returning `profile_id` on every entry.
Python mirrors the instance workflow with `Simulation()` and `load_profiles(path)`.
Bandwidth is derived only when the captured span fits one supported value;
ambiguous captures and manual OCD construction retain explicit overrides.

`ProtocolScope::default()` is `Docsis31`. The Rust capture convenience loader
`CaptureLoader::controller_from_capture` uses that default and empty OCD overrides.
Python `Merlin.from_profile_file(path, protocol=None, overrides=None)` uses the
same Rust default when protocol is omitted. Explicit loading remains available.

### Create a simulation session

`Simulation::new()` starts empty; `load_profiles(Vec<Dpd>)` derives its OCD
without running the engine. Decode via `pnm_input::CaptureLoader::profiles`.
Use `load_profiles_with_options` for protocol/OCD overrides, or `with_ocd` for
manual construction. Python mirrors this with `Simulation()`,
`load_profiles(path, protocol=None, overrides=None)`, `add_rxmer_file(path)` and
`add_channel_estimate_file(path)`. Existing typed measurement methods remain.
Python `Merlin` is a compatibility alias; Rust `Merlin` remains the existing
validated controller used by the session. Workbench runs that same engine.

The preferred receiver name is `CmChannelEstimated` in Rust and
`CM_CHANNEL_ESTIMATED` in Python. The previous `SuppliedChannel` name remains a
compatibility alias (Python also retains `SUPPLIED_CHANNEL`). Saved JSON still
writes `supplied_channel` and accepts both `supplied_channel` and
`cm_channel_estimated`. This naming change does not enable blind estimation:
the receiver corrects using the supplied CM response selected for the channel.

### Common Path Attenuator (CPA) and launch gain

Rust `Simulation` and `Merlin`, and Python `Simulation` expose matching
`set_cpa_attenuation_db`, `get_cpa_attenuation_db`,
`set_launch_amplifier_gain_db` and `get_launch_amplifier_gain_db` operations.
Setters validate finite 0..60 dB values in Rust and update atomically without
running simulation. Simulation requires loaded profiles or an explicit OCD.
Launch gain overrides a target level; its getter returns effective gain.
`PowerConditioning::cpa_attenuation_db` names the core scalar loss. The persisted
key remains `attenuation_db`; the new name is accepted as a JSON alias. Rust
callers using the old public field name must update to `cpa_attenuation_db`.
