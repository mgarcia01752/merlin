# Upstream capture source JSON prerequisite

Owner: [UIU-US-03](../../checklist.md#task-uiu-us-03), under
[REQ-UIU-13](../../../requirements.md#req-uiu-13) and
[REQ-UIU-21](../../../requirements.md#req-uiu-21).
Status: implemented with [scoped public API acceptance](validation.md#uiu-us-03-lossless-capture-source-api-prerequisite); native Qt adoption remains pending. Earlier API/FEC
planning and acceptance remain complete; this is a demonstrated input-consumer gap.

## Consumer gap and reuse

US03 requires actual UCD/RxMER/Pre-Eq binary and JSON input while preserving
original bytes/hashes. Current `UcdCapture::to_json`, `RxmerCapture::to_json` and
`PreEqCapture::to_json` produce explicitly inspection-only documents without the
original byte envelope. There is no public JSON importer. Reconstructing binary
from displayed fields could lose opaque bytes and cannot establish the claimed
original hash. Qt must not implement that reconstruction or a second decoder.

| Existing owner | Decision |
| --- | --- |
| `crates/pnm-input/src/upstream/{ucd,rxmer,pre_eq}.rs` native binary admission and immutable capture types | Reuse the existing decoders and return the same typed capture. Add bounded source JSON adapters, preserving all old methods and inspection output. |
| `MeasuredExperiment::from_json/prepare/admit`, through `merlin::upstream::inputs` | Reuse complete measured-plan JSON for association and scientific admission. Capture-source JSON remains a transport for one capture, not an executable plan. |
| `bindings/python/src/upstream/capture.rs` | Add thin native method parity, releasing the interpreter and preserving the existing error mapper. No Python decoding/calculations. |
| `apps/qt/src/upstream/workbench` | Consumer follows only after API acceptance. Retain actual file delivery bytes separately from decoded original capture bytes; preserve explicit measured association and reject unsupported composition. |

## Selected operation and wire contract

Added on each existing Rust capture type:

```rust
pub fn from_source_json(bytes: &[u8]) -> ApiResult<Self>;
pub fn to_source_json(&self) -> ApiResult<String>;
```

Python mirrors these as `Capture.from_source_json(bytes: bytes) -> Capture` and
`capture.to_source_json() -> str` on the existing `merlin.upstream` classes.
The input is UTF-8 JSON bytes, independent of paths, Qt and interpreter objects.
The result owns the same immutable original binary snapshot as native admission.
It does not retain the outer JSON formatting; applications retain source-file
bytes and their delivery hash separately. Loading does not associate or execute.

Use one strictly versioned envelope with these required fields:

- `format`: exactly `merlin.upstream.capture-source`.
- `schema_version`: exactly 1, independent of inspection/configuration/project schemas.
- `capture_kind`: exactly `ucd`, `rxmer` or `pre_eq` for the selected typed importer.
- `original_byte_count`: exact source capture byte count.
- `original_sha256`: exact lowercase SHA-256 over those bytes.
- `original_bytes`: unchanged source octets as a JSON byte array.

Reject duplicate/unknown fields, wrong format/version/family, invalid byte values,
count/hash mismatches and resource excess. Then call the original binary decoder;
its unsupported/invalid/offset semantics remain authoritative. Envelope diagnostics
name the envelope boundary at offset zero; they do not pretend to locate a binary
byte in formatted JSON. No observation/configuration JSON is silently upgraded.

The independent text policy is 1 MiB before parsing/serialization. The source-byte
ceiling reuses the largest currently supported binary envelope: the UCD management
length origin plus its unsigned 16-bit declared extent. Share that existing bound
through a private constants module; do not change native parser limits/behavior.
JSON construction is finite temporary work within the text bound; final source
length is checked before binary decoding/copy. The existing inspection JSON format
and outputs remain unchanged.

## Owner files and acceptance

| Disposition | Path / role |
| --- | --- |
| Add | `crates/pnm-input/src/upstream/source_json.rs`: private strict envelope codec shared by the three real consumers. |
| Add | `crates/pnm-input/src/upstream/constants.rs`: shared source bound, distinct JSON policy/version/identity. |
| Edit | `upstream/mod.rs`, `ucd.rs`, `rxmer.rs`, `pre_eq.rs`: module registration and documented typed adapters; original numerical decoding remains unchanged. |
| Edit | `bindings/python/src/upstream/capture.rs`: six paired native methods. |
| Add | `crates/pnm-input/tests/upstream_source_json.rs`, `bindings/python/tests/test_upstream_source_json.py`: independent source/format/ownership/bounds/parity cases. |
| Reuse | Existing UCD/RxMER/Pre-Eq fixtures/tests, native wheel builder/environment, operation matrix and documentation validators. |

Acceptance requires exact bytes/hashes/header/null/grid/coefficient preservation,
all three family round trips, independent literal envelope checks, cross-family and
malformed/unknown/version/count/hash rejection, limits before accepted output,
legacy inspection output compatibility and original binary error preservation.
Run the complete relevant `pnm-input` feature/crate tests, Rust-only facade/import
checks, formatting/Clippy, installed-wheel method/import/parity/ownership tests
and current operation/traceability checks. Do not rebuild Qt for this API-only
unit. Existing scientific kernels, downstream behavior and project writers remain
unchanged; targeted affected compatibility is required, not an unrelated full-suite
repeat. Commit this independently validated API unit before Qt adoption; US03
remains unchecked until its actual native input/viewer acceptance.

## Executed scope and metadata correction

The complete `pnm-input` suite passes 113 cases, including twelve independent
source-envelope cases; facade acceptance passes 31 cases. Installed Linux
CPython 3.10 passes ten new source tests and twelve existing capture, measured
plan and namespace tests. Fresh separate Rust processes supply both UCD/RxMER
inspection references and 25/50 kHz complete measured-run references. Targeted
input/facade/native Clippy, formatting, CLI compilation and workspace rustdoc pass.
The unchanged Qt application needs no rebuild for this API-only unit.

Fresh rustdoc also exposed an existing inventory omission: an inline facade
re-export can omit Source links and spell an owner's `Self` as the concrete type.
The narrow resolver correction proves the exact public common-module import and
canonical compiler type, kind, full method signature and source. Both displayed
and canonical signatures remain evidence; unrelated same-named types and drift
must fail. Independent tooling regressions cover this observed failure. This
changes validation discovery, not facade exports or scientific implementation.

Qt source-file delivery retention, native viewer interaction, complete measured
admission and protection against losing captured context remain US03 acceptance.
