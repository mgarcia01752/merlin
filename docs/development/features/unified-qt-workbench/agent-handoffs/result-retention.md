# Downstream result reservation preparation

Source-only preparation for UIU-SESSION-01 / REQ-UIU-03/07.
The lead owns the approved primary design section
`design.md#pre-production-downstream-result-bounds`, adoption, paired native
operations and acceptance. No official task is accepted by this handoff.

## Interface and ownership

Adds exactly `FullLoopRequest::result_retained_bound(&self)` and
`run_bounded(&self, &FullLoopControl, RetainedFootprint)`, both returning the
existing `ApiResult` contracts. Every dimension is checked before `control.begin`
and scientific progress. Insufficient reservations return ResourceLimit with the
control idle. Original construction errors and Busy are preserved; actual step
failure/cancellation keeps the existing partial report and completed trials.
A final debug assertion checks the audited invariant; it is not admission.

The API freezes exact final input/settings/policy destinations before admission,
reuses immutable Arc pointees and measures those graphs conservatively on every
edge. All moved output Vec/String capacities are normalized through boxed
slices/strings. This makes separate query/run calls independent of unspecified
Clone spare capacity. A caller-created later clone remains a separately measured
root. Legacy run, engine, observation and scientific bodies are unchanged.

The reservation includes the fixed result/job/stage roots, settings and original
inputs/provenance, retained detailed words, recovered original-file prefix,
physical-bin MER, latest channel-estimate pilots/response, payload/NCP pairs and
nested LLRs, all four protected streams and private pilot cache, stage map entries,
snapshots, context strings/roles, selected data and failure text. Fixed values are
covered once by their enclosing layout; immutable Arc payloads repeat per edge.

The stage proof reuses the existing `max_retained_bytes`: legacy accounting
already includes data lengths, snapshot fixed size, roles and identity lengths.
Exact final capacities cannot exceed that charge. Map key/value slots and ordered
entries are added separately. No historical statistics or byte policy changes.

Finite retained diagnostics have only three sources on this closed execution
path: audited static owner text (at most the existing 135-byte composition
message), `fec_codec::binary_bits` fixed text plus target-usize decimal width,
and the pinned ldpc-toolbox 0.10.1 encoder's sole 82-byte static Display variant.
MCE remapping preserves text. Filter-fit float text is construction-only and
returns before a job report. This fixed observer path admits no caller diagnostic,
I/O, export or JSON parser message. No truncation or new error policy is added.
The ignored receipt includes the 608-entry static diagnostic source inventory;
future diagnostic-path changes require a new audit.

This reserves final payload bytes, ordered entries and shared roots, excluding
allocator/control-block/tree-node overhead and temporary storage. Existing
capture/file/playback, duration/acquisition/completion/interleaver, decoder,
constellation/protected and probe/event/element/byte limits still govern temporary
builders and engine execution. No growth multiplier or scratch-byte/aggregate
preparation-peak promise is introduced. The host must charge simultaneous source,
old results, candidates, queues and copied roots separately.

## Files and integration hooks

New `api/full_loop/result_retention/{mod.rs,bound.rs,normalize.rs,output.rs,tests.rs}`
is registered only in `api/full_loop.rs`. Private snapshot helpers are added in
`api/{admission.rs,inputs.rs,channel.rs}` and
`api/full_loop/observations/{policy.rs,store.rs}`. Scientific-private capacity moves
are in `simulation/channel/protected_constellation/mod.rs` and
`simulation/source/file/verification.rs`; these two hooks are gated for PNM/API
or tests, keeping simulation-only builds independent of API helpers.

API-internal reuse: `SourceInput`, `Dpd`, `CaptureProvenance`,
`ChannelConfiguration` expose `normalize_result_storage()` only inside `crate::api`.
`full_loop::result_retention::normalize` exposes exact `vector`, `text`, `layout`,
`settings`, `source`, `profile`, `rxmer`, `response`, `filter` helpers at that same
visibility. `ScenarioKey` is fixed-size. No engine owner imports the API layer.
Receiver archives and the lead's TX owners are untouched.

## Prepared checks and receipt

All compilation, Clippy and runtime tests are **UNEXECUTED**, as instructed.
Twelve prepared cases cover:

- Exact/+1 admission, each deficient dimension including zero shared roots.
- Checked cell/histogram overflow and finite diagnostic width.
- Vec/String values with spare capacity and exact final storage.
- Private protected pilot-cache preservation.
- Complete/partial recovered-prefix storage.
- Stage text/roles/LLRs/events and unchanged historical statistics.
- Real complete unobserved result, bound stability and retained alias lifetime.
- Real complete all-selected-stage result and legacy report/query parity.
- Rejection before control/source progress, followed by unchanged file recovery.
- Actual finite-file result with one extra unit in all reservation dimensions.
- Observed/unobserved pre-cancelled report parity and used-control Busy.
- Observed/unobserved cancellation after genuine completed scientific trials.

Private fixture children accompany the stage, protected and file owners;
`crates/ofdm/tests/api_result_retained_bound.rs` owns the six real API cases.
Executed: pinned Rust 1.85.1 direct rustfmt/check, diff/allowlist/body-preservation
source review and Python integration-guide freshness check. No Cargo, targets,
caches, wheels or Qt runtime were built or changed.

Ignored artifact directory:
`docs/development/local/session-shell/result-retention/`.
`receipt.json` records exact commands, source hashes, preparation base and test
names/status; `source-sha256.txt` enumerates every delivered file and
`error-literals.json` records the source diagnostic inventory. The lead must
compile/reexecute the exact adopted source, review retained metadata against
actual results and run affected Rust/native parity and application ownership
checks before backend or UIU acceptance.
