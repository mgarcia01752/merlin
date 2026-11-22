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
