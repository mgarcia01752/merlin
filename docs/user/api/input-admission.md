# Passive input admission

These operations validate owned input choices without running a simulation or
changing an existing session. Rust uses `merlin::downstream`; installed Python
uses `merlin.downstream` and preserves the same classes at the `merlin` root
and legacy `merlin.merlin` path. Existing Simulation behavior is unchanged.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `TransmissionConfiguration::new(TransmissionSettings)` | `TransmissionConfiguration()` for defaults; `from_json(data: bytes)` for explicit settings | Existing numerology validation; immutable accepted snapshot. |
| `TransmissionConfiguration::from_json(&[u8])` | `TransmissionConfiguration.from_json(data)` | UTF-8 settings, at most 4096 bytes; malformed/unknown fields reject. |
| `settings() -> &TransmissionSettings`; serde serialization | `.settings: dict`; `.to_json() -> str` | Rust borrows accepted settings; Python returns an independent copy. |
| `timing(ObservationDuration) -> ApiResult<DurationTiming>` | `.timing(ObservationDuration) -> dict` | Exact existing integer sample/symbol/tail accounting; no source advancement. |
| `minimum_observation_milliseconds_after(u64) -> ApiResult<u64>` | `.minimum_observation_milliseconds_after(last_symbol_index: int) -> int` | Least whole-millisecond source window completing the zero-based symbol; immutable query. |
| `check_profile(&Dpd) -> ApiResult<()>` | `.check_profile(DPD) -> None` | Original-grid spacing and occupied-envelope compatibility. Full transmitter readiness remains separate. |
| `FileFillPolicy::new(u64, PrbsSeed)` | `FileFillPolicy(target_bits, PrbsSeed)` | Finite file-plus-fill target; independent nonzero 15-bit fill seed. |
| `SourceInput::prbs15(PrbsSeed) -> SourceInput` | `SourceInput.prbs15(PrbsSeed)` | Passive continuous PRBS15 choice; no generator/verifier is retained. |
| `SourceInput::file(NamedCapture, Option<FileFillPolicy>)` | `SourceInput.from_bytes(name, data, fill=None)` | Own exact original bytes and SHA-256; no filename-based conversion. |
| `SourceInput::unnamed_file(&[u8], Option<FileFillPolicy>)` | `SourceInput.from_unnamed_bytes(data, fill=None)` | Explicitly absent file display label; retain empty name, exact bytes/hash and original file/fill limits. |
| `kind()`, `prbs_seed()`, `name()`, `original_bytes()`, `original_sha256()`, `target_bits()` | Read-only properties with the same names | File target includes fill, excludes FEC padding. Empty file is target zero. Continuous PRBS has no finite target or original bytes. |

File admission preserves BOM, zero bytes and line endings, up to the existing
16 MiB input bound. Fill targets cannot be shorter than the original file and
are bounded by the existing finite-source limit. File fill and continuous source
seeds have separate meanings; neither replaces PHY scrambling. Python file
ownership/hash admission releases the GIL and delegates validation to Rust.

Use the explicit unnamed-file operation when the original file label is absent,
including legacy saved file sources. Its name is the empty string (`Some("")`
in Rust), distinct from continuous PRBS's absent name (`None`). Empty file bytes
are valid and remain a finite zero-bit file. This does not relax the existing
nonempty identity requirement for named inputs or scientific profile/measurement
captures, and never invents a replacement label.

```python
from merlin.downstream import (
    FileFillPolicy, ObservationDuration, PrbsSeed, SourceInput,
    TransmissionConfiguration,
)

timing = TransmissionConfiguration().timing(ObservationDuration(1))
source = SourceInput.from_bytes("payload.bin", b"\x00\r\n",
                                FileFillPolicy(32, PrbsSeed(1)))
assert source.original_bytes == b"\x00\r\n"
assert source.target_bits == 32
```

The minimum-window query reports a theoretical source-observation threshold,
including values above the admitted observation duration or UI run budget. It
excludes acquisition, interleaver drain and window tails, and admits no run.
Both languages use the same checked integer clock calculation. For the default
25-kHz spacing and 512-sample prefix, index 399 completes exactly at 17 ms.

These source objects are also accepted by the implemented
[controlled full-loop API](full-loop.md). Measurement/channel/Designer admission
and idle transmitter previews have separate [API guides](README.md).
Saved workbenches use the [saved full-loop entry point](saved-full-loop.md).

## Original measurement previews

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `RxMer::preview(MeasurementViewport) -> ApiResult<MeasurementPreview>` | `RxMer.preview(start=0, count=0) -> dict` | Original measured RxMER in dB; no averaging, replay or simulation. |
| `ChannelEstimate::preview(MeasurementViewport, ChannelResponseMetric)` | `ChannelEstimate.preview(metric="gain_db", start=0, count=0) -> dict` | Original complex H; `gain_db`, `phase_degrees`, `real` or `imaginary`. |

`MeasurementViewport { start, count }` uses original sample offsets. Zero count
selects the complete grid from zero, matching the existing measurement viewer;
nonzero windows must fit and reject overflow. Rust's typed metric variants are
`GainDb`, `PhaseDegrees`, `Real` and `Imaginary`. Python rejects unknown names with
`ApiError(InvalidInput)` and releases the GIL during Rust view construction.

The owned result contains `view`, exact full `grid`, original `provenance` and
`projection`. Views contain at most 1024 min/max groups with explicit missing
samples, original frequency labels and units. Zero-magnitude H is unavailable
for dB gain; no unity substitution or equalizer inversion occurs. Rust previews
retain original bytes through immutable shared ownership; serialized/Python
metadata contains the original name, epoch and hashes. Caller edits to Python
results do not change the capture. These views introduce no collection-selection,
Designer, channel-composition or simulation execution behavior.

## Measured sequence admission and preview

`MeasurementSequence` freezes an explicitly ordered collection of original
RxMER inputs. It does not run a simulation, interpolate original timestamps or
replace individual capture provenance. Both languages use the existing physical
measurement planner with identical grid, epoch, required-coverage and mode checks.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `MeasurementSequence::new(&Dpd, &TransmissionConfiguration, Vec<RxMer>, MeasurementSequenceOptions)` | `MeasurementSequence(profile, transmission, captures, options)` | Unique originals in caller order; maximum 256 captures, 4,000,000 values and 96 MiB original bytes. Count rejects before traversal; aggregate limits reject before backend frame cloning. |
| `MeasurementSequence::from_capture(&ProfileCapture, raw_id, &TransmissionConfiguration, Vec<RxMer>, options)` | `MeasurementSequence.from_capture(capture, profile_id, transmission, captures, options)` | Same existing ordered-original/resource/grid/epoch/coverage admission for a raw source ID, without simulation-ID admission. |
| `TransmissionConfiguration::check_response_capture(&ProfileCapture, raw_id, response, tolerance)` | `.check_response_capture(capture, profile_id, response=None, tolerance=None)` | Same physical H/unity and epoch checks; success does not establish run readiness. |
| `MeasurementSequenceOptions { mode, interval_symbols, tolerance }` | `MeasurementSequenceOptions(mode="single", interval_symbols=1, tolerance=None)` | Typed original-epoch tolerance; Python None requires exact epoch matching. Temporal modes require a positive symbol interval. |
| `MeasurementSequenceOptions::capacity(&TransmissionConfiguration, ObservationDuration)` | `options.capacity(transmission, duration) -> int` | Idle source-clock reachability before a profile/capture/sequence exists; the same backend query used by admitted sequences. Static modes ignore dormant intervals; zero temporal intervals reject. |
| `info() -> MeasurementSequenceInfo` | `.info() -> dict` | Mode/count and synthetic symbol interval/span in milliseconds; original epochs stay in provenance. |
| `capture(usize) -> ApiResult<&RxMer>` | `.capture(index) -> RxMer` | Rust borrows the immutable original; Python returns an independently owned capture snapshot. Invalid positions reject. |
| `at(u64) -> MeasurementResidual` | `.at(symbol) -> dict` | Dimensionless normalized residual power on the original grid plus every actual contributor's index, linear-power weight and provenance. |
| `preview(u64, MeasurementViewport)` | `.preview(symbol, start=0, count=0) -> dict` | Bounded equivalent RxMER dB view, exact grid/symbol and all contributors; no fabricated average timestamp. |
| `capacity(ObservationDuration) -> ApiResult<usize>` | `.capacity(ObservationDuration) -> int` | Permitted selection capacity for the proposed observation. It is separate from the actual admitted source count and does not admit a simulation job. |
| `TransmissionConfiguration::check_response(&Dpd, Option<&ChannelEstimate>, AssociationTolerance)` | `.check_response(profile, response=None, tolerance=None)` | Explicit synthetic unity or original H, with physical-grid/epoch/coverage admission; no equalizer inversion. |

Rust modes are `MeasurementMode::{Single, Average, Hold, Interpolate}`; Python
uses `single`, `average`, `hold`, `interpolate`. Single requires one capture.
Average uses every capture with equal linear-power weight. Hold/Interpolate
require at least two captures, use the explicit synthetic OFDM-symbol clock and
hold the final observation after the last transition. Interpolation operates on
residual power, preserving missing-value semantics; it does no dB interpolation.
`capacity` returns one for Single, 256 for Average, or the bounded number of
possible temporal positions in the requested complete-symbol duration. It does
not report how many captures were selected. `info().source_count` reports that.
Options can query this capacity before sequence admission without dummy scientific
inputs. It preserves the existing checked observation-duration domain, independently
of the Qt CWB's narrower duration policy. The result describes positions before
the complete-symbol boundary; it does not validate selected capture count,
original epochs, profile compatibility or run readiness. No source or RNG advances.

```python
from merlin.downstream import AssociationTolerance, MeasurementSequence, MeasurementSequenceOptions, ObservationDuration

# profile, transmission and captures are previously admitted inputs.
options = MeasurementSequenceOptions("interpolate", 10, AssociationTolerance(90000))
capacity = options.capacity(transmission, ObservationDuration(1))  # One-ms source window; no sequence needed.
sequence = MeasurementSequence(profile, transmission, captures, options)
midpoint = sequence.at(5)  # original contributor epochs are retained
```

The immutable sequence supports independent concurrent queries; Python releases
the GIL for backend admission and bounded scientific queries. Returned dictionaries
are caller-owned copies; scalar clock/capacity metadata uses immutable native reads.
Construction and query errors do not change accepted captures or another sequence.
Original/capture previews and equivalent sequence previews remain distinct.
Legacy saved-WB loaders retain their documented single/average scope; the
independent [saved full-loop preparation](saved-full-loop.md) supports accepted
temporal projects. Designer admission below, [channel previews](channel-preview.md)
and [idle TX previews](transmitter-preview.md) have their accepted backend contracts.
Their remaining Qt adoption belongs to UIU-DS-01.

## Immutable RxMER Designer inputs

Rust's optional `merlin::downstream::inputs` exposes `RxMerDesign`,
`RxMerDesignPosition` and curated existing recipe configuration types such as
`RxmerRecipe`, `Tilt` and `CaptureScope`. Python exposes the same design/position
classes through `merlin.downstream` and preserves root/legacy class identity.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `RxMerDesignPosition::new(index, count)` | `RxMerDesignPosition(index=1, count=1)` | One-based position in 1..256 original captures; separate from playback symbols. |
| `RxMerDesign::from_bytes(&NamedCapture)` | `RxMerDesign.from_bytes(name, data)` | Strict saved version-1 Designer envelope; preserves exact original bytes/whitespace. |
| `RxMerDesign::flat(&Dpd, MerDb, CaptureTime, RxmerRecipe, RxMerDesignPosition)` | `RxMerDesign.flat(profile, baseline, epoch, recipe_json=None, position=None)` | Exact profile grid/identity and backend role mask; explicit synthetic anchor epoch. |
| `RxMerDesign::flat_capture(&ProfileCapture, raw_id, level, epoch, recipe, position)` | `RxMerDesign.flat_capture(capture, profile_id, baseline, epoch, recipe_json=None, position=None)` | Same authoritative role mask/grid/baseline validation using original source IDs. |
| `RxMerDesign::imported_capture(&ProfileCapture, raw_id, input, recipe, position)` | `RxMerDesign.imported_capture(capture, profile_id, name, data, recipe_json=None, position=None)` | Same exact ordinary baseline ownership, noncompounding edits and spatial association. |
| `RxMerDesign::check_capture(&ProfileCapture, raw_id)` | `.check_capture(capture, profile_id)` | Exact original device/channel/grid association; source epochs and tolerance remain separate. |
| `RxMerDesign::imported(&Dpd, &NamedCapture, RxmerRecipe, RxMerDesignPosition)` | `RxMerDesign.imported(profile, name, data, recipe_json=None, position=None)` | Exact ordinary baseline bytes embedded with integrity metadata; spatial identity checked. |
| `check_profile(&Dpd)` | `.check_profile(profile)` | Explicit exact device/channel/grid check for an independently loaded design. |
| `with_recipe(RxmerRecipe)` | `.with_recipe(recipe_json)` | Re-evaluate the retained original baseline with new effects; previous output is never the new baseline. |
| `materialize() -> RxMer` | `.materialize() -> RxMer` | Owned copy of the already evaluated observation; no PHY simulation starts. |
| `original_bytes()`, `recipe()`, `position()`, `is_synthetic()` | Read-only `.original_bytes`, `.recipe`, `.position`, `.is_synthetic` | Original envelope, owned Python configuration copy, original position and baseline origin. |
| `Dpd::key()` | `DPD.key` | Exact original identity/grid; Python returns an independent dictionary. |
| `RxMer::source_kind()`, `ChannelEstimate::source_kind()` | `.source_kind` | Existing backend frame labels, preserving synthetic/designed/ordinary distinctions. |

Python recipe arguments are UTF-8 JSON bytes with a 64 KiB application bound;
None selects an empty recipe for creation. Rust retains a typed recipe route and
`RxMerDesign::recipe_from_json` for the same transport. Recipe/count/grid
validation and evaluation use the existing Rust owner. Typed positions default
to 1 of 1 in Python. Frames-per-collection times source carriers remain bounded
at 4,000,000 values; the final original envelope must fit 16 MiB. Embedded base64
can make that envelope larger than its imported raw capture. Existing embedded
name/integrity limits remain in force. No source file is written by these calls.

Imported baselines reject nested Designer envelopes and flat-only random
variation. Replacing an imported -6 dB tilt with -3 dB applies -3 dB to the
original baseline; it preserves the previous design as a separate immutable
value. Seeded Designer variation uses its own deterministic recipe stream and
preserves simulation source/noise state. Python releases the GIL for Rust
validation/evaluation; bounded original-byte copying uses the existing native
admission helper before backend dispatch.

Generated frames retain `merlin_synthetic_rxmer` or `merlin_designed_rxmer` source
labels. Ordinary inputs retain their existing `decoded_json_*` frame labels,
while `provenance.original_kind` records their original binary/JSON encoding.
Qt consumers use validated design operations for evaluation; curated recipe
configuration exports do not authorize direct recipe-kernel calls from Qt.

```python
from merlin.downstream import CaptureTime, MerDb, RxMerDesign

# profile is a previously admitted DPD.
draft = RxMerDesign.flat(profile, MerDb(48), CaptureTime(1700000000))
edited = draft.with_recipe(b'{"tilt":{"scope":{"mode":"all"},"tilt_db":-3}}')
observation = edited.materialize()
assert observation.source_kind == "merlin_synthetic_rxmer"
```

The [profile inspector](profile-inspection.md) exposes original compact schemes
and scientific summaries through the same facade. [Configured channel previews](channel-preview.md)
provide typed loss/power/filter admission and authoritative transfer data. [Idle transmitter previews](transmitter-preview.md)
provide explicit private TX generation and authoritative timing/window data. These design operations preserve the existing
saved-recipe schema and its shared Rust/native-Python workbench behavior.

Optional Rust facade feature `native-input` exposes existing
`downstream::inputs::{CaptureLoader, Workbench, read_capture, save_json,
rxmer_export}` and the explicitly exported `upstream::inputs` capture/association
contracts. These retain their original errors, provenance and resource bounds.
Enabling native input includes both PHY adapters; independent facade direction
features and direct PHY crates remain usable without it. The current Workbench
loader supports its existing single/average RxMER policies; re-exporting it does
not add timeline policies or a direction-tagged project schema.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-09),
[task and validation](../../development/features/unified-qt-workbench/validation.md#uiu-api-01-input-admission-unit).


## Finite source limits and immutable fill editing

`SourceInput::file_limits()` returns the existing original-file byte ceiling and
finite meaningful-bit work ceiling as SourceFileLimits. These application
resource policies are separate from capture-decoder limits and PHY parameters.
Native Python `SourceInput.file_limits()` returns the matching owned dictionary.

`source.file_info()` returns SourceFileInfo in Rust and an owned dictionary in
Python, or None for continuous PRBS15. A finite empty file returns a present
zero-valued summary. The fields are original_bytes, original_bits, target_bits,
fill_bits and optional fill (target_bits and independent checked seed). An
explicit zero-bit append policy remains distinguishable from no append policy.
The query generates no source or fill bits and excludes FEC padding.

`source.with_fill(fill)` returns a new independently validated file source while
sharing the immutable original byte buffer and preserving name/hash. Passing
None removes append and restores the original bit length. The original object
stays unchanged; continuous PRBS inputs reject this operation. Rust owns all
admission, counting and memory sharing. Python `source.with_fill(fill=None)`
returns a new native handle; original_bytes still returns an owned Python copy.

```rust
// `source` is an already accepted finite SourceInput.
let info = source.file_info().expect("finite source");
let limits = SourceInput::file_limits();
assert!(info.target_bits <= limits.max_total_bits);
let original_only = source.with_fill(None)?;
assert_eq!(original_only.file_info().unwrap().fill_bits, 0);
```

```python
import merlin.downstream as m

# `source` is an already accepted finite m.SourceInput.
info = source.file_info()
limits = m.SourceInput.file_limits()
assert info['target_bits'] <= limits['max_total_bits']
original_only = source.with_fill()
assert original_only.file_info()['fill_bits'] == 0
```

See the bounded [Rust process reference](../../../crates/ofdm/examples/api_source_file_info.rs).

Raw source-catalog operations preserve every byte-sized encoded ID. They do not
widen `ProfileId` (0..15), establish coding support or start a simulation.
Missing source IDs return `InvalidIdentifier`; existing operation signatures,
errors, bounds and checked-profile behavior remain available unchanged.


### Original measurement frames

`ChannelEstimate::frame() -> &ChannelEstimateFrame` borrows the exact admitted
key, epoch, decoded-source metadata and I/Q/nulls without copying. Native Python
`channel.frame() -> dict` returns an owned bounded copy. Editing that dictionary
cannot change the native input or later previews. No inverse response or
simulation observation is inferred. The existing Rust `RxMer::frame` and
`RxMer::from_frame` use the nominal public `RxMerFrame` data re-export; their
signatures and behavior remain unchanged. These DTOs add no native classes.

Frame source metadata describes the scientific decoder intermediate. For binary
inputs its hash differs from the original file hash. Read exact original bytes,
original hash/epoch and decoder lineage through `provenance()` (Python
`.provenance`). Do not relabel decoded data as an original capture or silently
rewrite either provenance record. DTO copies and borrowed returns retain their
Rust ownership rules; native dictionaries own their values.
