# Copied receiver performance admission preparation

Base: `5bc84c125b2c7c823d8ca07605695f6974b7153d`.
Branch: `feature/qt-receiver-performance-admission`.
Owner: UIU-SESSION-01 / REQ-UIU-03, REQ-UIU-06, REQ-UIU-07.
Source-only preparation; no SESSION/runtime acceptance is claimed.

## Interface and owner

`performance::{begin,retain,reject}` now return
`Result<(), crate::shell::RetentionError>`. `clear`, `view`, `fec_view`,
selection and original plotting logic are unchanged. Each stored Entry has a
last-field `Retained<()>` guard. Its reservation includes the actual fixed
`size_of::<Entry>()` once (including every Option, guard and padding), actual
normalized Point Vec/status/error/JSON String and Array capacities, and nested
JSON map key/value payloads and ordered-entry population. No scientific or
presentation Arc is introduced; shared payload roots are zero. The accounting
handle graph and outer registry capacity are separate unfinished owners under
the contract, not additional exclusions from it.

Finite `PreparedEntry` candidates are never installed or queued. After existing
builders finish, final Vec/String/JSON capacities are normalized through boxed
storage, measured with checked arithmetic and admitted before moving into the
registry. There is no collect/clone capacity assumption, growth multiplier,
scientific reimplementation or new public API. Previously accepted Entries and
the independently charged immutable scientific source survive target rejection.
Candidate admission overlaps existing retained ownership; its charge follows
replacement/clear/Context destruction until actual child disposal. No aggregate
temporary-builder/preparation-peak claim is made.

`begin` admits ALL selected initial Entries (at most existing MAX_PROFILES)
before clearing performance or QAM history. A later candidate failure drops all
earlier candidates and preserves both old registries. `retain` keeps the existing
QAM snapshot call/log order. A performance admission failure preserves the prior
performance Entry; separately successful QAM adoption remains authoritative.
Original public query errors still create the same failed Entry when that target
is admitted. `reject` preserves full original diagnostics without synthetic FEC.

## Finite construction audit

The unchanged public `FullLoopResult::profile_performance` query at
`crates/ofdm/src/api/full_loop/performance.rs:59` traverses original validated
profile carriers: FrequencyGrid limits sample_count by MAX_LAB_CARRIERS.
Each point contains fixed coordinates/role/CarrierMer/optional ideal MER.
ScenarioKey is fixed; the stored FecReport is audited at api/reports.rs as fixed
MetricScope plus fec-codec statistics' five-u64 Copy counters. Private fields are
not reconstructed through getters or serialization sizing.

The unchanged `receiver::fec::snapshot` consumes actual admitted retained_words
bounded by job.limits.max_retained_words <= MAX_RETAINED_WORDS, the existing
MAX_CHANNEL_DECODER_ITERATIONS + 1 histogram, and verification charts capped by
MAX_VERIFICATION_WORD_BARS. Other inputs are fixed-width counters/scopes/floats,
finite scalar formatting and static labels. Scientific diagnostics are the
existing admitted bounded-result error; query errors preserve public grid/unit
admission text. Setup rejection transfers the already produced owned String:
normalization is linear in its actual finite length, with no new parser or error
population. Caller/worker error transport remains a separately unfinished owner.
The shallow known JSON builders introduce no arbitrary imported JSON recursion.
Final full text, coordinates, nulls, values and map populations are preserved.

## Root integration hooks and evidence

Root owns closed_loop.rs Model start/poll: remove the pre-begin clear; handle
begin failure by finishing the acquired lease before token/state mutation.
Handle retain/reject host failures through the existing diagnostic path while
still adopting the completed scientific ResultData. No Context, Model, bridge,
shared primitive, FEC builder, QAM owner, backend or manifest is edited here.

Six focused cases are prepared: exact private spare-capacity/JSON counts and
overflow; real complete/pre-cancelled science and FEC JSON parity; exact and
one-unit limits in positive dimensions plus honest zero roots and transactional
host pressure; second-candidate begin rollback preserving QAM aliases; full setup
error preservation and failed replacement; admitted old/candidate overlap,
replacement/clear/Context disposal. The original PLC coordinate assertions stay
byte-identical. Independent backend/comparator fixtures are outside these Entry
ledger assertions; they do not establish aggregate closure.

Executed: pinned recursive rustfmt parse/format, whitespace, generated-guide and
scoped source/ownership review. **ALL compilation, Clippy, Cargo/runtime tests,
Qt/native checks and integration acceptance are UNEXECUTED here**, as delegated.
Source receipt/hashes:
`docs/development/local/session-shell/receiver-performance-admission/` (ignored).
Remaining owners include registry spare capacity/fixed-root reconciliation,
view/fec_view response copies, QAM Entry diagnostics/catalog, input/model/control/
channel/accounting/transport infrastructure and complete session admission.
