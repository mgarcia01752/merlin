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
| `check_profile(&Dpd) -> ApiResult<()>` | `.check_profile(DPD) -> None` | Original-grid spacing and occupied-envelope compatibility. Full transmitter readiness remains separate. |
| `FileFillPolicy::new(u64, PrbsSeed)` | `FileFillPolicy(target_bits, PrbsSeed)` | Finite file-plus-fill target; independent nonzero 15-bit fill seed. |
| `SourceInput::prbs15(PrbsSeed) -> SourceInput` | `SourceInput.prbs15(PrbsSeed)` | Passive continuous PRBS15 choice; no generator/verifier is retained. |
| `SourceInput::file(NamedCapture, Option<FileFillPolicy>)` | `SourceInput.from_bytes(name, data, fill=None)` | Own exact original bytes and SHA-256; no filename-based conversion. |
| `kind()`, `prbs_seed()`, `name()`, `original_bytes()`, `original_sha256()`, `target_bits()` | Read-only properties with the same names | File target includes fill, excludes FEC padding. Empty file is target zero. Continuous PRBS has no finite target or original bytes. |

File admission preserves BOM, zero bytes and line endings, up to the existing
16 MiB input bound. Fill targets cannot be shorter than the original file and
are bounded by the existing finite-source limit. File fill and continuous source
seeds have separate meanings; neither replaces PHY scrambling. Python file
ownership/hash admission releases the GIL and delegates validation to Rust.

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

These source objects currently describe admission only. They are not yet inputs
to a new public controlled full-loop job; that extension remains
[UIU-API-02](../../development/checklist.md#task-uiu-api-02).
Measurement/channel/Designer and idle transmitter previews remain in
[UIU-API-01](../../development/checklist.md#task-uiu-api-01).

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
| `MeasurementSequenceOptions { mode, interval_symbols, tolerance }` | `MeasurementSequenceOptions(mode="single", interval_symbols=1, tolerance=None)` | Typed original-epoch tolerance; Python None requires exact epoch matching. Temporal modes require a positive symbol interval. |
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

```python
from merlin.downstream import AssociationTolerance, MeasurementSequence, MeasurementSequenceOptions

# profile, transmission and captures are previously admitted inputs.
options = MeasurementSequenceOptions("interpolate", 10, AssociationTolerance(90000))
sequence = MeasurementSequence(profile, transmission, captures, options)
midpoint = sequence.at(5)  # original contributor epochs are retained
```

The immutable sequence supports independent concurrent queries; Python releases
the GIL for backend admission and bounded scientific queries. Returned dictionaries
are caller-owned copies. Construction and query errors do not change accepted
captures or another sequence. Original/capture previews and equivalent sequence
previews remain distinct. This operation does not extend the legacy saved-WB
loader's single/average policy; temporal saved-project integration remains tracked
under UIU-API-05. Designer, channel composition and idle TX previews still await
separate API acceptance.

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
