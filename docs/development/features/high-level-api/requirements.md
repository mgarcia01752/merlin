# High-level API acceptance contract

Source: user-supplied `docs/development/MERLIN-API-Codex-Handoff.zip`, prepared
2026-11-14. Its requirements govern API-RUST-01 and API-PY-01. The original archive
and extracted reference documents are preserved, not represented as implemented
code. This record maps the contract to the current implementation and evidence.

| Requirements | Implementation boundary | Evidence/status |
| --- | --- | --- |
| 001–007, 010–015 | `docsis_ofdm_fec::api`, optional native input adapter | Validated inputs, owned results and Qt-independent build checks |
| 020–027, 080 | `pnm-input` and immutable named bytes | All three binary/JSON fixture pairs passed fixture-backed imports |
| 030–038, 081, 090–092 | `Ocd`, `Dpd`, existing captured-layout validation | Derive uniquely determined profile bandwidth; ambiguous captures require override; defaults recorded |
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

## Shared Rust/Python API evolution

- **REQ-API-PARITY-001:** Shared capabilities requested through Python MUST be
  implemented and validated in the public Rust API first, then exposed by the
  optional adapter with matching domain semantics and structured errors. Python
  must not become the only owner of discovery, validation or scientific logic.
- **REQ-API-PARITY-002:** Profile discovery MUST read the actual capture catalog
  through Rust decoding, return validated captured identities and reject invalid
  catalogs. Examples that run all profiles MUST use discovery rather than a
  hardcoded list or probing possible identifiers. Discovery MUST NOT simulate.
- **REQ-API-PARITY-003:** New discovery operations MUST cover binary/decoded JSON
  agreement, ordering, empty/duplicate/invalid IDs and independent owned results
  across Rust and Python. File convenience wrappers remain adapter responsibilities.

- **REQ-API-PARITY-004:** An already-loaded DPD MUST expose its retained profile
  catalog as a Rust member accessor and Python property, without file access,
  redecoding or simulation. Keep static discovery available. Distinguish the
  complete source catalog from the selected profile ID; preserve immutable
  ownership, created-profile singleton catalogs and edited-source catalogs.

## API-RECEIVER-01: receiver correction parity

- Expose the existing receiver correction policy through validated Rust options
  before mirroring it in Python; retain the supplied-channel default.
- Keep applied channel H and receiver correction independent. Loading/preparing
  or selecting a mode must not run simulation. Reports preserve chosen settings.
- Match the saved-UI collection companion's correction, seeds, iterations,
  duration, average policy and OCD timing to its curated WB, with a fixture
  drift check. Keep the separate MCE walkthrough explicitly labeled.
- Compare actual observation-window FEC and independent bit comparison per profile;
  matching BER alone is insufficient. Keep all static selection modes, owned
  results and cancellation intact. Do not force agreement by changing counters.

## API-RECEIVER-02: explicit MCE naming

Use `ReceiverMode::MerlinChannelEstimated` in Rust and
`ReceiverMode.MERLIN_CHANNEL_ESTIMATED` in Python. Preserve existing WB/CLI/report
JSON `merlin_estimated` and accept `merlin_channel_estimated` on input. Renaming
must not change correction algorithms, decoder reliability or receiver defaults.

## API-WB-01: saved workbench input

- Provide native Rust and Python WB file/byte loaders, independently of Qt/CLI.
  Read schema 1/2 accepted embedded captures and settings; verify lengths/hashes,
  exact grids, explicit selections and bounded work before returning.
- Preserve legacy/current profile selection precedence, static checked RxMER
  single/average behavior, saved OCD, PRBS15 seed, receiver mode/MCE settings,
  waveform channel filter, H/unity, association tolerance, loss/power and limits.
- Keep original WB bytes and capture provenance; loading/preparing does not run.
  Only run starts the existing engine. Return independent batches per profile,
  retain controller scopes and preserve cancellation/partial/not-started results.
- Reject unsupported sources/timeline modes, unknown fields and incomplete WBs
  explicitly. Do not approximate saved behavior. General timeline playback and
  file-source import are not implemented by this static PRBS15 loader.
- Add a Python WB-path example without duplicate scientific constants, and a
  compiling Rust example. Compare actual reports with the Rust engine and native
  UI, test integrity/selection/limits/cancellation, and document platform evidence.

## API-WB-02: bundled receiver defaults

All runnable WB examples must explicitly save CmChannelEstimated, matching the
new-configuration UI/API default. Preserve captures and all other settings.
The saved-UI Python companion and WB loader must reproduce that saved choice;
the separate MCE walkthrough remains MCE. Guard this policy in the packaged
bundle inventory test and maintain current FEC references and user guides.

## Generated API reference maintenance

Maintain searchable generated references for the public Rust high-level API,
PNM input/workbench adapter and native Python API. Source-owned documentation
must describe public parameters, units, defaults, ownership, errors, cancellation
and partial results; Python pages must not expose hidden interpreter arguments.
The development builder uses strict rustdoc and pinned Sphinx against the actual
installed native extension. Discover all exported classes, require public-member
coverage and reject installed-version mismatch. Rebuild/reinstall the native
wheel when its documentation changes even without a version bump. Keep generated
HTML ignored and local; publication remains a separate authorized action.
See the [build guide](../../../reference/api-reference.md).

## Native wheel release gate

Release tags must build and install CPython 3.10 x64 wheels on native Ubuntu
22.04 and Windows 2022/MSVC runners and execute the actual single-capture Python
API example in an isolated fresh environment. Require exact package/ABI/platform
identity, complete notices and real fixture FEC/BER outcomes. Both checksummed
wheel artifacts are mandatory before publication; retain all existing application
and security gates. Keep release filenames, compatibility and installation/hash
verification in a separate [Python release guide](../../../user/python/releases.md),
linked from Python installation and generated downstream integration documents.
Configuration is not evidence of native Windows execution or published assets.

API documentation and example changes cover both Rust and Python unless
explicitly scoped otherwise. Default profile-capture loading uses DOCSIS 3.1;
explicit protocol selection remains supported and file versions do not infer it.

Simulation is the preferred create/load/configure/run entry point in both APIs.
Keep legacy constructors compatible. Loading is idle and atomic; preserve scoped
measurement identity and owned results. Shared build/release example checks must
report elapsed wall time, including failures, with Python example-only time
separated from total wheel validation and Rust build-inclusive time labeled.

### API-CPA-01 — Channel conditioning controls

Both APIs expose setters/getters for launch-amplifier gain and Common Path
Attenuator (CPA) loss. Rust validates finite 0..60 dB values and rejects invalid
updates without changing accepted settings. Setters never run simulation.
Explicit gain clears a target level; the gain getter returns effective gain.
Preserve legacy saved attenuation keys and define CPA in the shared terminology.
