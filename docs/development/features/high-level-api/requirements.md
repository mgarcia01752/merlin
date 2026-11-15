# High-level API acceptance contract

Source: user-supplied `docs/development/MERLIN-API-Codex-Handoff.zip`, prepared
2026-11-14. Its requirements govern API-RUST-01 and API-PY-01. The original archive
and extracted reference documents are preserved, not represented as implemented
code. This record maps the contract to the current implementation and evidence.

| Requirements | Implementation boundary | Evidence/status |
| --- | --- | --- |
| 001–007, 010–015 | `docsis_ofdm_fec::api`, optional native input adapter | Validated inputs, owned results and Qt-independent build checks |
| 020–027, 080 | `pnm-input` and immutable named bytes | All three binary/JSON fixture pairs passed fixture-backed imports |
| 030–038, 081, 090–092 | `Ocd`, `Dpd`, existing captured-layout validation | Bandwidth override required for current decoder profiles; defaults recorded |
| 040–048, 082 | Independent controller collections, opaque scoped IDs | Explicit/unity/nearest-time policies implemented |
| 100–113, 114–116 | `prepare`, existing `MeasurementPlayback` power operations | Static selections, exact grids, explicit epochs, null preservation |
| 050–059, 083–085 | Sequential existing `run_channel` jobs | Direct engine parity, runtime continuation and cancellation tests passed |
| 060–069 | Owned batch, window FEC/profile summaries, whole-run diagnostics | No analytical substitute counters; detailed scopes in reports |
| 070–074, 086–088 | Separate optional Python adapter | Native wheel, seven Python tests and all four policy parity checks passed |

Current interface refinements against the proposed signature inventory:

- Input unit constructors use `new` and explicit `value` getters. Whole-second
  CaptureTime and millisecond ObservationDuration also have descriptive aliases.
- `prepare()` is public for no-execution inspection and engine parity. It returns
  immutable snapshots, not a second configurable engine.
- The optional `CaptureLoader` handles named binary/JSON bytes; `read_capture`
  adds bounded file I/O. These are the recommended file conveniences. `FromFile` adds one-argument
  measurement loaders; `ProfileFromFile` adds the explicit two-argument profile
  loader, avoiding a dummy selection argument for measurements.
- Existing validated engine grid/layout models are reused at manual OCD
  construction; no replacement scientific model or descriptor serialization.
- JSON output is an owned string; file export is separate and create-new only.
- Full engine diagnostics retain their existing typed Rust representation.
  Serialized diagnostics omit private compatibility settings for static derived
  observations; public sources retain original provenance and true weights.

Executed evidence and verification boundaries: [validation record](validation.md).

No Octave, simultaneous channels, continuous playback, MAC services, publishing
or system installation is part of this milestone.

## Mandatory downstream installation-document maintenance

These additional repository requirements govern ongoing delivery:

- **REQ-PY-DOC-001:** Every `tools/git/git-push.sh` branch or release invocation
  MUST refresh or validate `docs/user/python/integration.md`, including
  documentation-only pushes and explicit test-omission modes.
- **REQ-PY-DOC-002:** Generation MUST live in a separate script,
  `tools/python/update_integration_doc.py`, invoked by Git helpers. The guide
  MUST derive version/tag/installation pins from validated VERSION and Python
  metadata plus a tracked template; generation logic MUST NOT live in shell.
- **REQ-PY-DOC-003:** Version-bump releases MUST refresh the guide before
  candidate validation and include it in the same validated release commit.
  Ordinary saves MUST include any refresh. A branch push that creates an
  uncommitted guide change MUST stop for review/commit rather than silently
  creating a commit or bypassing the clean-tree guard.
- **REQ-PY-DOC-004:** Generation MUST be deterministic and leave unchanged
  content untouched. A dry run and existing-tag retry MUST check without
  modifying the guide or immutable tagged commit. Generator errors and stale
  checks MUST block pushing.
- **REQ-PY-DOC-005:** Interrupted release recovery MUST recognize only the exact
  version metadata and generated guide reconstructed from committed inputs.
  Modified guides, staged changes and unrelated edits MUST remain rejected.
- **REQ-PY-DOC-006:** Generation MUST NOT invent wheel availability, checksums,
  published tags or platform verification. Release artifact hashes and supported
  targets require their own evidence; refreshing docs does not publish wheels.
- **REQ-REPO-ARCHIVE-001:** Generated/downloaded archives, including ZIPs,
  compressed tar files and Python wheels, MUST be ignored by Git. A required
  source fixture needs a narrowly scoped, explicitly reviewed exception.
