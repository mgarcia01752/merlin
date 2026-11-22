# Original profile capture inspection

`ProfileCapture` loads a complete original modulation-profile source without
running a simulation. Rust uses `merlin::downstream`; optional `native-input`
provides `inputs::CaptureLoader` for binary/JSON convenience. Installed Python
uses the same classes under `merlin.downstream`, the preserved root and legacy
namespace. Existing `ProfileId` and simulation behavior remain unchanged.

| Rust operation | Python operation | Result/contract |
| --- | --- | --- |
| `ProfileCapture::from_bytes(&NamedCapture)` | `ProfileCapture.from_bytes(name, data)` via native loader | Rust core admits decoded JSON; Python/native loader also admits pinned binary PNM. Original bytes and intermediate hashes stay distinct. |
| `ProfileCapture::from_decoded(original, decoded, contract)`; `inputs::CaptureLoader::profile_capture(&NamedCapture)` | `ProfileCapture.from_file(path)` or `from_bytes(name, data)` | Shared decoder/contract/source validation; no filename-based format guessing or output writes. |
| `profile_ids() -> &[u8]` | `.profile_ids -> tuple[int, ...]` | Every original encoded identity, sorted for discovery. |
| `metadata() -> ProfileCaptureInfo` | `.metadata -> dict` | Original header/key, sorted IDs, source profile order and original encoding. |
| `provenance() -> &CaptureProvenance` | `.provenance -> dict` | Original/intermediate names, hashes, epoch and decoder declaration; Rust also retains exact immutable bytes. |
| `summary(raw_id) -> ApiResult<ProfileSummary>` | `.summary(profile_id) -> dict` | Full-grid role counts, original origin/time and known data-carrier useful-symbol capacity. |
| `schemes(raw_id, start, limit)` | `.schemes(profile_id, start=0, limit=256)` | Bounded original compact scheme page, source codes/order and authoritative original offsets/frequencies/constellations. |
| `inspect(&ProfileViewRequest)` | `.inspect(ProfileViewRequest) -> dict` | Bounded raw metric extrema, original coordinates, selected-role support and actual provenance. |
| `Dpd::summary()` / `Dpd::view(&ProfileViewRequest)` | `DPD.summary()` / `DPD.view(request)` | Same numerical owner for the one accepted profile, retaining captured/created/edited origin and actual sources. |

The capture catalog preserves byte-sized encoded identities, including values
outside the legacy simulation `ProfileId` domain of 0–15. Source recognition and
read-only inspection preserve those identities; simulation admission keeps its
existing domain. The full-loop compatibility gate for source identities remains
[UIU-API-02](../../development/checklist.md#task-uiu-api-02) before Qt migration.
No source ID is silently dropped or treated as an array index.

## Metrics and bounds

Rust `ProfileViewRequest` owns optional `profiles`, `roles`, a typed `ProfileMetric`,
`MeasurementViewport` and `max_buckets`. Python constructs
`ProfileViewRequest(profiles=None, roles=None, metric="bits", start=0, count=0,
max_buckets=2048)`. None selects the complete source/role catalog; an explicit
empty list selects no traces/roles. Profile selections sort and deduplicate.
Rust metric variants `BitLoading` and `IdealShannonSnr` map to Python `bits` and
`shannon_db`. Role names use the existing backend enum:
`data`, `continuous_pilot`, `plc`, `exclusion`, `zero_bit_loaded`, `reserved`, `unknown`.

Original input/intermediate size, profile count, grid and expanded-carrier limits
reuse the existing parser. Requests permit at most 256 IDs, seven roles and
1–2048 total buckets. The total budget must cover the selected profile count.
Zero viewport count selects the complete original grid from zero; nonzero
windows reject overflow/out-of-range offsets. Scheme pages return at most 256
original rows and preserve unavailable/unknown values. Output role support is
bounded by seven spans per bucket; no source-frequency resampling occurs.

The main metric trace covers known data/pilot/PLC constellations. Zero-loaded,
excluded, reserved and unknown assignments retain separate role support, with
raw zero/absent loadings intact. Each bucket reports raw extrema, an actual
first-value coordinate and explicit gaps. Role spans preserve first/last original
coordinates, actual counts and loading extrema. Selected-role counts cover the
whole selected profiles, independently of the viewport/role filters. UI adapters
may transform these authoritative values into pixels/colors/labels.

The ideal Shannon value comes from the existing AWGN capacity-bound owner.
Receiver operating requirements, measured MER and implementation margins retain
their separate meanings. `uncoded_data_capacity_bps` reuses the useful-symbol
rate owner: sum known **data** loading times source spacing, before cyclic prefix,
FEC and MAC overhead. Pilots/PLC stay separate. `capacity_complete` identifies
unknown/reserved/missing-data limitations. Original PLC data uses its established
16-QAM description; preamble metadata separately identifies BPSK. These source
descriptions preserve existing interpretation and establish no new coding mode.

```python
from merlin.downstream import ProfileCapture, ProfileViewRequest

capture = ProfileCapture.from_file("modulation_profile.bin")
for source_id in capture.profile_ids:
    print(source_id, capture.summary(source_id)["uncoded_data_capacity_bps"])
view = capture.inspect(ProfileViewRequest(
    profiles=capture.profile_ids, metric="shannon_db", max_buckets=2048))
```

Rust results own their arrays and retain immutable source provenance; Python
returns independent dictionaries/tuples. Python releases the GIL for decoding,
inspection and result preparation. Failed requests leave accepted inputs and
previous results unchanged. There is no filesystem write, PHY run or RNG advance
in these operations. The original parser entry remains a compatibility wrapper
and omits inspection arrays for existing scientific consumers.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-09),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-01-profile-capture-inspection-unit)
and [task](../../development/checklist.md#task-uiu-api-01).
